use crate::manager;
use tokio::sync::Mutex;

//
// status commands
//
#[tauri::command]
pub async fn is_ready(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<bool, String> {
    let wallet_lock = command_tx.lock().await;
    Ok(wallet_lock.is_initialized())
}

#[tauri::command]
pub async fn is_setup_complete(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<bool, String> {
    let wallet_lock = command_tx.lock().await;
    Ok(wallet_lock.is_setup_complete())
}

//
// get from memory
//
#[tauri::command]
pub async fn get_master_pubkey(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<String, String> {
    let wallet_lock = command_tx.lock().await;
    Ok(wallet_lock.get_master_pubkey())
}

#[tauri::command]
pub async fn get_balance(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<u64, String> {
    let wallet_lock = command_tx.lock().await;
    let balance = wallet_lock.get_balance()?;
    Ok(balance)
}

//
// sets memory
//
#[tauri::command]
pub async fn initialize(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<(), String> {
    let mut wallet_lock = command_tx.lock().await;
    wallet_lock.initialize().await
}

//
// pokes
//
#[tauri::command]
pub async fn keygen(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<(), String> {
    let wallet_lock = command_tx.lock().await;
    wallet_lock.keygen().await
}

#[tauri::command]
pub async fn gen_master_privkey(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
    seedphrase: Vec<String>,
) -> Result<(), String> {
    let wallet_lock = command_tx.lock().await;
    let seedphrase = seedphrase.join(" ");
    wallet_lock.gen_master_privkey(seedphrase).await
}

#[tauri::command]
pub async fn gen_master_pubkey(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
    master_privkey: String,
) -> Result<(), String> {
    let wallet_lock = command_tx.lock().await;
    wallet_lock.gen_master_pubkey(master_privkey).await
}

//
// peeks
//
#[tauri::command]
pub async fn peek_seedphrase(
    command_tx: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<Vec<String>, String> {
    let wallet_lock = command_tx.lock().await;
    wallet_lock.peek_seedphrase().await
}