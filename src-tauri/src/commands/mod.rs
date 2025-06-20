use crate::keycrypt::Keycrypt;
use crate::manager::{NockchainNode, Wallet};
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

pub mod nockchain_node;
pub mod terms;
pub mod updater;
pub mod wallet;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AeroeStatus {
    pub vault_exists: bool,
    pub vault_loaded: bool,
    pub master_node_running: bool, // nockchain node (doesn't mine)
    pub block_height: Option<u32>,
    pub num_miners: u64,      // total number of nodes mining
    pub wallets: Vec<String>, // list of wallet names
    pub active_wallet: Option<String>,
}

#[tauri::command]
pub async fn aeroe_status(
    vault: State<'_, Mutex<Keycrypt>>,
    wallet: State<'_, Mutex<Wallet>>,
    nockchain_node: State<'_, Mutex<NockchainNode>>,
) -> Result<AeroeStatus, String> {
    let vault = vault.lock().await;

    let wallet = wallet.lock().await;

    let mut nockchain_node = nockchain_node.lock().await;

    let (master_running, num_workers) = nockchain_node
        .get_status()
        .await
        .map_err(|e| e.to_string())?;

    let status = AeroeStatus {
        vault_exists: vault.vault_exists(),
        vault_loaded: vault.is_loaded(),
        master_node_running: master_running,
        block_height: wallet.get_block_height(),
        num_miners: num_workers,
        wallets: vault.get_wallets(),
        active_wallet: wallet.get_active_wallet(),
    };
    Ok(status)
}
