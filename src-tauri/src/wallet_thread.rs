use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::path::PathBuf;
use tracing::{info, warn};
use crate::manager::WalletCommand;
use crate::wallet_app::WalletApp;

pub fn wallet_thread_worker(
    wallet_rx: Receiver<WalletCommand>,
    wallet_dir_for_cmd: PathBuf,
    shutdown_signal_wallet: Arc<AtomicBool>,
) -> impl FnMut() -> Result<(), String> + Send + 'static {
    move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        info!("Wallet thread worker started. Listening for commands.");

        for cmd in &wallet_rx {
            if shutdown_signal_wallet.load(std::sync::atomic::Ordering::Relaxed) {
                info!("Wallet thread: shutdown signaled during command processing. Stopping.");
                return Err("Shutdown signaled".to_string());
            }

            let command_to_run = cmd.command;
            info!("Wallet thread: processing command...");

            let result = runtime.block_on(WalletApp::run(
                command_to_run,
                wallet_dir_for_cmd.clone(),
            ));

            if cmd.response.send(result).is_err() {
                warn!("Wallet thread: receiver for command response dropped. This might be normal if client disconnected.");
            } else {
                info!("Wallet thread: command processed and response sent.");
            }
        }

        info!("Wallet command channel loop finished (sender might have been dropped).");
        Ok(())
    }
} 