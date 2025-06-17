use tauri::State;
use tokio::sync::Mutex;

use crate::manager::{NockchainNode, NockchainPeek};

#[tauri::command]
pub async fn node_start_master(node: State<'_, Mutex<NockchainNode>>) -> Result<(), String> {
    let mut node = node.lock().await;
    node.start_master().await
}

#[tauri::command]
pub async fn node_stop_master(node: State<'_, Mutex<NockchainNode>>) -> Result<(), String> {
    let mut node = node.lock().await;
    node.stop_master().await
}

#[tauri::command]
pub async fn node_peek(
    status_caller_tx: State<'_, Mutex<tokio::sync::broadcast::Sender<NockchainPeek>>>,
    command: String,
) -> Result<(), String> {
    let peek_command = match command.as_str() {
        "height" => NockchainPeek::Height,
        "heavy-summary" => NockchainPeek::HeavySummary,
        "transactions" => NockchainPeek::Transactions,
        _ => return Err(format!("Invalid peek command: {}", command)),
    };
    let status_caller_tx = status_caller_tx.lock().await;
    if let Err(e) = status_caller_tx.send(peek_command) {
        return Err(format!("Failed to send peek command: {}", e));
    }
    Ok(())
}
