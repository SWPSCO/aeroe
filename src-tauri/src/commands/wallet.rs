use tokio::sync::Mutex;
use tauri::State;

use crate::keycrypt::Keycrypt;
use crate::manager;

#[tauri::command]
pub async fn vault_create(state: State<'_, Mutex<Keycrypt>>, password: String) -> Result<(), String> {
    let mut keycrypt = state.lock().await;
    keycrypt.create(password)
}

#[tauri::command]
pub async fn vault_load(state: State<'_, Mutex<Keycrypt>>, password: String) -> Result<(), String> {
    let mut keycrypt = state.lock().await;
    keycrypt.load(password)
}

#[tauri::command]
pub async fn wallet_create(state: State<'_, Mutex<Keycrypt>>, wallet_name: String, seedphrase: Vec<String>) -> Result<(), String> {
    let mut keycrypt = state.lock().await;
    keycrypt.add_wallet(wallet_name, seedphrase.join(" "))
}
#[tauri::command]
pub async fn keygen(
    wallet: tauri::State<'_, Mutex<manager::Wallet>>,
) -> Result<Vec<String>, String> {
    let wallet_lock = wallet.lock().await;
    wallet_lock.keygen().await?;
    let seedphrase = wallet_lock.peek_seedphrase().await?;
    wallet_lock.clear_state().await?;
    Ok(seedphrase)
}

#[tauri::command]
pub async fn wallet_load(
    wallet: tauri::State<'_, Mutex<manager::Wallet>>,
    vault: tauri::State<'_, Mutex<Keycrypt>>,
    wallet_name: String,
) -> Result<(), String> {
    let vault_lock = vault.lock().await;
    let seedphrase = vault_lock.get_seedphrase(wallet_name.clone())?;
    tracing::debug!("seedphrase: {:?}", seedphrase);
    let mut wallet_lock = wallet.lock().await;
    wallet_lock.gen_master_privkey(seedphrase).await?;
    wallet_lock.load(wallet_name).await?;
    Ok(())
}

#[tauri::command]
pub async fn master_pubkey(
    wallet: tauri::State<'_, Mutex<manager::Wallet>>,
    wallet_name: String,
) -> Result<String, String> {
    let wallet_lock = wallet.lock().await;
    let loaded_wallet_name = wallet_lock.get_active_wallet();
    if loaded_wallet_name != Some(wallet_name) {
        return Err("wallet name mismatch".to_string());
    }
    wallet_lock.get_master_pubkey().await
}

#[tauri::command]
pub async fn balance(
    wallet: tauri::State<'_, Mutex<manager::Wallet>>,
    wallet_name: String,
) -> Result<u64, String> {
    let wallet_lock = wallet.lock().await;
    let loaded_wallet_name = wallet_lock.get_active_wallet();
    if loaded_wallet_name != Some(wallet_name) {
        return Err("wallet name mismatch".to_string());
    }
    wallet_lock.get_balance().await
}