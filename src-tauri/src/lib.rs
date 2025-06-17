mod commands;
mod keycrypt;
mod manager;
mod prover;
mod services;
mod update_checker;
mod wallet_app;
mod watcher;

use tokio::sync::Mutex;

use tauri::Manager;

use crate::commands::*;
use crate::keycrypt::Keycrypt;
use crate::watcher::Watcher;

use crate::commands::terms::TermsState;
use std::time::Duration;
use tracing::error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    nockvm::check_endian();
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // --- Update Checker Service ---
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(15));
                loop {
                    interval.tick().await;
                    update_checker::check_update(&app_handle).await;
                }
            });

            // --- Application Directories ---
            let data_dir: std::path::PathBuf = app.path().app_data_dir().unwrap();
            let wallet_dir = data_dir.join("wallet");
            let draft_dir = data_dir.join("draft");
            let nockchain_dir = data_dir.join("nockchain");
            let keycrypt_dir = data_dir.join("vault");

            // --- Nockchain Status Receiver ---
            let (status_receiver_tx, status_receiver_rx) =
                tokio::sync::mpsc::channel::<manager::NockchainStatus>(128);

            // --- Nockchain Status Caller ---
            let (status_caller_tx, status_caller_rx) =
                tokio::sync::broadcast::channel::<manager::NockchainPeek>(128);

            // --- Wallet Service ---
            let (wallet_tx, wallet_rx) = tokio::sync::mpsc::channel::<manager::WalletCommand>(128);
            services::spawn_wallet_service(
                wallet_rx,
                wallet_dir.clone(),
                nockchain_dir.clone().join("npc/master.sock"),
            );

            // --- Nockchain Service ---
            let (nockchain_tx, nockchain_rx) =
                tokio::sync::mpsc::channel::<manager::NockchainCommand>(128);
            services::spawn_nockchain_service(
                nockchain_rx,
                status_receiver_tx,
                status_caller_rx,
                nockchain_dir.clone(),
            );

            // --- Application State Management ---
            app.manage(Mutex::new(TermsState::new(&app.handle())));
            app.manage(Mutex::new(manager::Wallet::new(
                wallet_tx,
                wallet_dir.clone(),
                draft_dir.clone(),
            )));
            app.manage(Mutex::new(manager::NockchainNode::new(nockchain_tx)));
            app.manage(Mutex::new(Keycrypt::new(keycrypt_dir)));
            app.manage(Mutex::new(status_receiver_rx));
            app.manage(Mutex::new(status_caller_tx));

            // --- Watcher Service ---
            tauri::async_runtime::spawn(async move {
                let watcher = Watcher::new(wallet_dir, data_dir);
                if let Err(e) = watcher.start().await {
                    error!("Watcher service failed to start: {}", e);
                }
            });

            // --- Start Master Node ---
            let master_node_app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let nockchain_node =
                    master_node_app_handle.state::<Mutex<manager::NockchainNode>>();
                let mut nockchain_node = nockchain_node.lock().await;
                if let Err(e) = nockchain_node.start_master().await {
                    error!("Failed to start master node: {}", e);
                }
            });

            // --- Wallet State Updater ---
            // receives status from nockchain node
            let status_app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let status_receiver_rx = status_app_handle
                    .state::<Mutex<tokio::sync::mpsc::Receiver<manager::NockchainStatus>>>();
                let wallet_app = status_app_handle.state::<Mutex<manager::Wallet>>();
                let mut status_receiver_rx = status_receiver_rx.lock().await;
                loop {
                    let status = match status_receiver_rx.recv().await {
                        Some(status) => status,
                        None => {
                            error!("Failed to receive status");
                            continue;
                        }
                    };
                    let height = match status.height() {
                        Ok(height) => height,
                        Err(e) => {
                            error!("Failed to get height: {}", e);
                            continue;
                        }
                    };
                    let mut wallet_app = wallet_app.lock().await;
                    let res = wallet_app.update(height).await;
                    if let Err(e) = res {
                        error!("Failed to update wallet state: {}", e);
                    }
                }
            });

            // --- Nockchain Status Caller ---
            // calls nockchain node to get status
            let status_app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let status_caller_tx = status_app_handle
                    .state::<Mutex<tokio::sync::broadcast::Sender<manager::NockchainPeek>>>();
                loop {
                    {
                        let status_caller_tx = status_caller_tx.lock().await;
                        let status = status_caller_tx.send(manager::NockchainPeek::Height);
                        if let Err(e) = status {
                            error!("Failed to send status: {}", e);
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(30)).await;
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
            wallet::create_tx,
            wallet::sign_tx,
            wallet::send_tx,
            wallet::list_unsent_txs,
            // nockchain node
            nockchain_node::node_start_master,
            nockchain_node::node_stop_master,
            nockchain_node::node_peek,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
