use tauri::State;
use tokio::sync::Mutex;

use crate::manager::NockchainNode;

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