mod commands;
mod manager;
mod wallet_app;
mod watcher;
mod keycrypt;
// services.rs is no longer used

use std::panic::AssertUnwindSafe;
use tokio::sync::Mutex;

use tauri::{Emitter, Manager, AppHandle};

use tauri_plugin_updater::UpdaterExt;

use crate::watcher::Watcher;
use crate::keycrypt::Keycrypt;
use crate::commands::*;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use futures::FutureExt;
use tracing::{error, warn, info};
use crate::commands::terms::TermsState;
use crate::wallet_app::WalletApp;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    nockvm::check_endian();
    // This shutdown signal might be used by other parts of the app, so we keep it for now.
    let _shutdown_signal = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // --- Update Checker Service ---
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(15));
                loop {
                    interval.tick().await;
                    check_update(&app_handle).await;
                }
            });

            // --- Application Directories ---
            let data_dir: std::path::PathBuf = app.path().app_data_dir().unwrap();
            let wallet_dir = data_dir.join("wallet");
            let watcher_dir = data_dir.join("watcher");
            let keycrypt_dir = data_dir.join("vault");

            // --- Wallet Service ---
            let (wallet_tx, mut wallet_rx) = tokio::sync::mpsc::channel::<manager::WalletCommand>(128);
            let wallet_dir_for_service = wallet_dir.clone();

            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                runtime.block_on(async move {
                    info!("[Wallet Service] Started on a dedicated OS thread");
                    while let Some(cmd) = wallet_rx.recv().await {
                        let command_name = format!("{:?}", cmd.command);
                        info!("[Wallet Service] Received command: {}", command_name);

                        let wallet_dir_clone = wallet_dir_for_service.clone();
                        
                        let future = AssertUnwindSafe(WalletApp::run(cmd.command, wallet_dir_clone));
                        
                        match future.catch_unwind().await {
                            Ok(run_result) => {
                                // Task completed successfully (or with a normal error)
                                if cmd.response.send(run_result).is_err() {
                                    warn!("[Wallet Service] Command {}: Receiver dropped.", command_name);
                                }
                            },
                            Err(panic_payload) => {
                                // Task panicked
                                let panic_message = if let Some(s) = panic_payload.downcast_ref::<&'static str>() {
                                    s.to_string()
                                } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                                    s.clone()
                                } else {
                                    "Unknown panic reason".to_string()
                                };

                                error!("[Wallet Service] Command {} panicked: {}", command_name, &panic_message);
                                
                                // Inform the caller about the panic
                                let _ = cmd.response.send(Err(format!("Command panicked: {}", panic_message)));
                            }
                        }
                    }
                    info!("[Wallet Service] Channel closed. Shutting down.");
                });
            });
            
            // --- Application State Management ---
            app.manage(Mutex::new(TermsState::new(&app.handle())));
            app.manage(Mutex::new(manager::Wallet::new(wallet_tx, wallet_dir.clone())));
            app.manage(Mutex::new(Keycrypt::new(keycrypt_dir)));

            // --- Watcher Service ---
            tauri::async_runtime::spawn(async move {
                let watcher = Watcher::new(watcher_dir, wallet_dir);
                if let Err(e) = watcher.start().await {
                    error!("Watcher service failed to start: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // terms
            terms::privacy_policy_is_accepted,
            terms::terms_of_use_is_accepted,
            terms::accept_terms_of_use,
            terms::accept_privacy_policy,
            // updater
            updater::download_and_install_update,
            // app
            aeroe_status,
            // wallet
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