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

#[tauri::command]
pub async fn node_start_mining(node: State<'_, Mutex<NockchainNode>>, mining_profile: String) -> Result<(), String> {
    if mining_profile != "max" {
        return Err("Invalid mining profile".to_string());
    }
    let mut node = node.lock().await;
    let num_cores = num_cpus::get();
    node.set_workers(num_cores.saturating_sub(1) as u64).await
}

#[tauri::command]
pub async fn node_stop_mining(node: State<'_, Mutex<NockchainNode>>) -> Result<(), String> {
    let mut node = node.lock().await;
    node.set_workers(0).await
}