use tauri::State;
use tokio::sync::Mutex;

use crate::manager::{NockchainNode, NockchainPeek, NodeMode};

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
pub async fn node_connect_external(
    node: State<'_, Mutex<NockchainNode>>,
    socket_path: String,
) -> Result<(), String> {
    tracing::info!("[node_connect_external] Called with path: {}", socket_path);
    
    let mut node = node.lock().await;
    tracing::info!("[node_connect_external] Current mode before connect: {:?}", node.get_mode());
    
    let result = node.connect_external(&socket_path).await;
    
    tracing::info!("[node_connect_external] Result: {:?}", result);
    tracing::info!("[node_connect_external] Mode after connect: {:?}", node.get_mode());
    tracing::info!("[node_connect_external] Is connected: {}", node.is_connected());
    
    result
}

#[tauri::command]
pub async fn node_disconnect_external(node: State<'_, Mutex<NockchainNode>>) -> Result<(), String> {
    tracing::info!("[node_disconnect_external] Called");
    
    let mut node = node.lock().await;
    tracing::info!("[node_disconnect_external] Current mode before disconnect: {:?}", node.get_mode());
    
    let result = node.disconnect_external().await;
    
    tracing::info!("[node_disconnect_external] Result: {:?}", result);
    tracing::info!("[node_disconnect_external] Mode after disconnect: {:?}", node.get_mode());
    tracing::info!("[node_disconnect_external] Is connected: {}", node.is_connected());
    
    result
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

#[tauri::command]
pub async fn node_get_status(
    node: State<'_, Mutex<NockchainNode>>,
) -> Result<serde_json::Value, String> {
    let node = node.lock().await;
    
    // Debug logging
    tracing::info!("[node_get_status] Current mode: {:?}", node.get_mode());
    tracing::info!("[node_get_status] Is connected: {}", node.is_connected());
    
    let mode_str = match node.get_mode() {
        NodeMode::Local => "local",
        NodeMode::External(_) => "external", 
        NodeMode::Disconnected => "disconnected",
    };
    
    let result = serde_json::json!({
        "connected": node.is_connected(),
        "mode": mode_str
    });
    
    tracing::info!("[node_get_status] Returning: {}", result);
    
    Ok(result)
}