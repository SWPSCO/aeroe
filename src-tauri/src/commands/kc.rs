use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use crate::keycrypt::Keycrypt;

#[tauri::command]
pub async fn exists(state: State<'_, Mutex<Keycrypt>>) -> Result<bool, String> {
    let keycrypt = state.lock().await;
    Ok(keycrypt.exists())
}