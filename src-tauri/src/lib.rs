mod commands;
mod manager;
mod wallet_app;
mod watcher;
mod keycrypt;
mod thread_utils;
mod wallet_thread;

use std::sync::mpsc;
use tokio::sync::Mutex;

use tauri::{Emitter, Manager, AppHandle};

use tauri_plugin_updater::UpdaterExt;

use crate::watcher::Watcher;
use crate::wallet_app::WalletApp;
use crate::keycrypt::Keycrypt;
use crate::commands::*;

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};
use crate::commands::terms::TermsState;
use crate::thread_utils::spawn_restartable_thread;
use crate::wallet_thread::wallet_thread_worker;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    nockvm::check_endian();
    let shutdown_signal = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // update checker
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(15));
                loop {
                    interval.tick().await;
                    check_update(&app_handle).await;
                }
            });

            // data directory for all app data
            let data_dir: std::path::PathBuf = app.path().app_data_dir().unwrap();

            // nockchain node pma location
            let _nockchain_dir = data_dir.join("nockchain"); // unused for now
            // directory that holds all the wallet pmas
            let wallet_dir = data_dir.join("wallet");
            // where we'll write the watcher binary and execute from
            let watcher_dir = data_dir.join("watcher");
            // the file we'll store the seedphrases in
            let keycrypt_dir = data_dir.join("vault");

            // setup wallet thread using the restartable thread function
            let (wallet_tx, wallet_rx) = mpsc::channel::<manager::WalletCommand>();
            let wallet_dir_for_watcher = wallet_dir.clone();
            let wallet_dir_for_cmd = wallet_dir.clone();
            let shutdown_signal_wallet = shutdown_signal.clone(); // Use the main app shutdown signal

            // Spawn the wallet thread using the restartable thread utility (see thread_utils.rs)
            spawn_restartable_thread(
                "Wallet".to_string(),
                wallet_thread_worker(wallet_rx, wallet_dir_for_cmd, shutdown_signal_wallet),
                shutdown_signal.clone(),
            );

            // Initialize TermsState
            let terms_state = TermsState::new(&app.handle());

            // Initialize wallet manager
            let wallet_manager = manager::Wallet::new(wallet_tx, wallet_dir);

            // Initialize keycrypt
            let keycrypt = Keycrypt::new(keycrypt_dir);
            
            // load tauri state
            app.manage(Mutex::new(terms_state));
            app.manage(Mutex::new(wallet_manager));
            app.manage(Mutex::new(keycrypt));

            // start watcher
            tauri::async_runtime::spawn(async move {
                let watcher = Watcher::new(watcher_dir, wallet_dir_for_watcher);
                let res = watcher.start().await;
                if let Err(e) = res {
                    error!("Error starting watcher: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            //
            // terms
            //
            terms::privacy_policy_is_accepted,
            terms::terms_of_use_is_accepted,
            terms::accept_terms_of_use,
            terms::accept_privacy_policy,
            //
            // updater
            //
            updater::download_and_install_update,
            //
            // app
            //
            aeroe_status,
            //
            // wallet
            //
            wallet::vault_create,
            wallet::vault_load,
            wallet::wallet_create,
            wallet::keygen,
            wallet::wallet_load,
            wallet::master_pubkey,
            wallet::balance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Refactored: update checker logic as its own function
async fn check_update(app_handle: &AppHandle) {
    let window = match app_handle.get_webview_window("main") {
        Some(w) => w,
        None => {
            warn!("Update checker: could not get main window");
            return;
        }
    };
    let updater = match app_handle.updater() {
        Ok(u) => u,
        Err(e) => {
            error!("Update checker: error getting updater: {:?}", e);
            return;
        }
    };
    let check_result = updater.check().await;
    match check_result {
        Ok(Some(update)) => {
            let _ = window.emit("update", updater::UpdateInfo::new(
                true,
                update.version,
            ));
        },
        Ok(None) => {
            let _ = window.emit("update", updater::UpdateInfo::new(
                false,
                "".to_string(),
            ));
        },
        Err(e) => {
            warn!("Update checker: error during update check: {:?}", e);
        }
    }
}