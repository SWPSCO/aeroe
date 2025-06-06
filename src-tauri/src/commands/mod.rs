use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use crate::keycrypt::Keycrypt;
use crate::manager::Wallet;

pub mod wallet;
pub mod terms;
pub mod updater;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AeroeStatus {
    pub vault_exists: bool,
    pub vault_loaded: bool,
    pub num_nodes: u32,  // total number of nodes
    pub num_miners: u32, // total number of nodes mining
    pub wallets: Vec<String>, // list of wallet names
    pub active_wallet: Option<String>,
}

#[tauri::command]
pub async fn aeroe_status(vault: State<'_, Mutex<Keycrypt>>, wallet: State<'_, Mutex<Wallet>>) -> Result<AeroeStatus, String> {
    let vault = vault.lock().await;
    let wallet = wallet.lock().await;

    let status = AeroeStatus {
        vault_exists: vault.vault_exists(),
        vault_loaded: vault.is_loaded(),
        num_nodes: 0,
        num_miners: 0,
        wallets: vault.get_wallets(),
        active_wallet: wallet.get_active_wallet(),
    };
    Ok(status)
}