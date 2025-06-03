mod commands;
mod manager;
mod wallet_app;
mod watcher;

use std::sync::mpsc;
use tokio::sync::Mutex;

use tauri::{Emitter, Manager};

use tauri_plugin_updater::UpdaterExt;

use crate::watcher::Watcher;
use crate::wallet_app::WalletApp;
use crate::commands::{
    wallet,
    terms,
    updater,
};

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};
use crate::commands::terms::TermsState;

// Define a trait for the worker function passed to spawn_restartable_thread
// The worker function returns a Result:
// - Ok(()): Indicates the current unit of work completed, and the supervisor can re-invoke if needed.
// - Err(String): Indicates a desire to stop the thread gracefully, not due to a panic.
trait ThreadFn: FnMut() -> Result<(), String> + Send + 'static {}
impl<F: FnMut() -> Result<(), String> + Send + 'static> ThreadFn for F {}

/// Spawns a thread that will attempt to restart its core logic if it panics
/// or if its worker function completes and is designed to be restarted.
/// Includes a graceful shutdown mechanism via an AtomicBool.
pub(crate) fn spawn_restartable_thread<F>(
    thread_name: String,
    mut work_fn: F,
    shutdown_signal: Arc<AtomicBool>,
) where
    F: ThreadFn,
{
    thread::spawn(move || {
        info!("[{}] Starting thread", thread_name);
        while !shutdown_signal.load(Ordering::Relaxed) {
            // Ensure any captured state by work_fn is either cloneable for each iteration
            // or that work_fn is designed to re-initialize itself.
            let result = catch_unwind(AssertUnwindSafe(|| {
                work_fn() // Execute the provided worker function
            }));

            match result {
                Ok(Ok(_)) => {
                    // work_fn completed its current task successfully without signaling a stop.
                    // If work_fn is a long-running task that exited, this implies it should be restarted.
                    info!("[{}] Worker function completed a cycle. Checking for shutdown before potential restart.", thread_name);
                }
                Ok(Err(stop_reason)) => {
                    // work_fn completed and explicitly signaled to stop the restart loop.
                    warn!(
                        "[{}] Worker function signaled stop: {}. Thread exiting.",
                        thread_name, stop_reason
                    );
                    break; // Exit the while loop, terminating the thread.
                }
                Err(panic_payload) => {
                    if shutdown_signal.load(Ordering::Relaxed) {
                        info!("[{}] Thread panicked during shutdown. Suppressing restart. Panic: {:?}", thread_name, panic_payload);
                        break;
                    }
                    error!(
                        "[{}] Thread panicked: {:?}. Restarting after a delay...",
                        thread_name, panic_payload
                    );
                    // Brief delay before restarting. Check shutdown signal again before sleeping.
                    for _ in 0..50 {
                        // Sleep for 5 seconds, but check shutdown every 100ms
                        if shutdown_signal.load(Ordering::Relaxed) {
                            info!("[{}] Shutdown signaled during panic recovery delay. Thread exiting.", thread_name);
                            // Ensure the outer while loop condition will also catch this
                            // by not continuing if break is not immediately possible here.
                            // Storing true to signal is done by the main app.
                            return; // Exit thread
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }

            // Brief pause if not shutting down, to prevent tight spinning if work_fn exits very quickly
            // and is intended to be restarted.
            if !shutdown_signal.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(200));
            }
        }
        info!("[{}] Thread shutting down.", thread_name);
    });
}

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
                    let window = match app_handle.get_webview_window("main") {
                        Some(w) => w,
                        None => {
                            warn!("Update checker: could not get main window");
                            continue;
                        }
                    };
                    let updater = match app_handle.updater() {
                        Ok(u) => u,
                        Err(e) => {
                            error!("Update checker: error getting updater: {:?}", e);
                            continue;
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
                            continue;
                        }
                    }
                }
            });

            // data directory for all app data
            let data_dir: std::path::PathBuf = app.path().app_data_dir().unwrap();
            let _nockchain_dir = data_dir.join("nockchain"); // unused for now
            let wallet_dir = data_dir.join("wallet");
            let watcher_dir = data_dir.join("watcher");

            // setup wallet thread using the restartable thread function
            let (wallet_tx, wallet_rx) = mpsc::channel::<manager::WalletCommand>();
            let wallet_dir_for_watcher = wallet_dir.clone();
            let wallet_dir_for_cmd = wallet_dir.clone();
            let shutdown_signal_wallet = shutdown_signal.clone(); // Use the main app shutdown signal

            spawn_restartable_thread(
                "Wallet".to_string(),
                move || { // This is the worker_fn for the wallet
                    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
                    info!("Wallet thread worker started. Listening for commands.");

                    // Iterate over commands from the channel.
                    // If wallet_tx is dropped, this loop will end, work_fn returns Ok(()),
                    // and the supervisor will restart this worker, effectively re-listening.
                    for cmd in &wallet_rx { // Iterating over a reference to wallet_rx
                        if shutdown_signal_wallet.load(Ordering::Relaxed) {
                            info!("Wallet thread: shutdown signaled during command processing. Stopping.");
                            return Err("Shutdown signaled".to_string()); // Signal supervisor to stop this thread
                        }

                        // Clone necessary data for each command processing, if WalletApp::run consumes or modifies them
                        // and a panic could leave them in an inconsistent state for subsequent commands in *this* run.
                        // However, a panic in WalletApp::run will be caught by the supervisor, restarting the whole worker.
                        let command_to_run = cmd.command; // If WalletCommand is not Clone, this moves.

                        info!("Wallet thread: processing command...");

                        let result = runtime.block_on(WalletApp::run(
                            command_to_run, 
                            wallet_dir_for_cmd.clone(),
                        ));

                        if cmd.response.send(result).is_err() {
                            warn!("Wallet thread: receiver for command response dropped. This might be normal if client disconnected.");
                            // This doesn't necessarily mean the wallet thread should stop.
                        } else {
                            info!("Wallet thread: command processed and response sent.");
                        }
                    }
                    
                    // If the loop finishes (e.g., wallet_tx sender dropped), this worker function iteration is done.
                    info!("Wallet command channel loop finished (sender might have been dropped).");
                    // Return Ok(()) to allow the supervisor to restart (i.e., re-establish listening).
                    Ok(())
                },
                shutdown_signal.clone(),
            );

            // Initialize TermsState
            let terms_state = TermsState::new(&app.handle());

            // Initialize wallet manager
            let wallet_manager = manager::Wallet::new(wallet_tx);
            
            // load tauri state
            app.manage(Mutex::new(terms_state));
            app.manage(Mutex::new(wallet_manager));

            let app_handle_wallet = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let wallet_state = app_handle_wallet.state::<Mutex<manager::Wallet>>();
                let mut wallet_lock = wallet_state.lock().await;
                let res = wallet_lock.initialize().await;
                if let Err(e) = res {
                    eprintln!("Error initializing wallet: {}", e);
                }
            });

            tauri::async_runtime::spawn(async move {
                let watcher = Watcher::new(watcher_dir, wallet_dir_for_watcher);
                let res = watcher.start().await;
                if let Err(e) = res {
                    eprintln!("Error starting watcher: {}", e);
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // updater
            updater::download_and_install_update,
            //
            // terms
            //
            terms::privacy_policy_is_accepted,
            terms::terms_of_use_is_accepted,
            terms::accept_terms_of_use,
            terms::accept_privacy_policy,
            //
            // wallet
            //
            // status
            wallet::is_ready,
            wallet::is_setup_complete,
            // get
            wallet::get_master_pubkey,
            wallet::get_balance,
            // set
            wallet::initialize,
            // pokes
            wallet::gen_master_privkey,
            wallet::gen_master_pubkey,
            wallet::keygen,
            // peeks
            wallet::peek_seedphrase,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
