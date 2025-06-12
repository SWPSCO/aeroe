use nockapp::noun::slab::NounSlab;
use nockchain_wallet_lib::Commands;
use nockvm::noun::Noun;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use std::path::PathBuf;

use crate::manager::NockchainStatus;

pub struct WalletCommand {
    pub command: Commands,
    pub response: oneshot::Sender<Result<Vec<NounSlab>, String>>,
}

#[derive(Debug)]
pub struct Wallet {
    wallet_dir: PathBuf,
    command_tx: Sender<WalletCommand>,
    wallet_name: Option<String>,
    master_pubkey: Option<String>,
    balance: Option<u64>,
    block_height: Option<u32>,
    last_sync: Option<std::time::Instant>,
}

impl Wallet {
    // creates new wallet manager
    pub fn new(command_tx: Sender<WalletCommand>, wallet_dir: PathBuf) -> Self {
        Self {
            wallet_dir,
            command_tx,
            wallet_name: None,
            master_pubkey: None,
            balance: None,
            block_height: None,
            last_sync: None,
        }
    }
    pub async fn clear_state(&self) -> Result<(), String> {
        std::fs::remove_dir_all(self.wallet_dir.clone()).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn get_active_wallet(&self) -> Option<String> {
        self.wallet_name.clone()
    }
    pub fn get_block_height(&self) -> Option<u32> {
        self.block_height
    }
    pub async fn load(&mut self, wallet_name: String) -> Result<(), String> {
        self.wallet_name = Some(wallet_name);
        let pubkey = self.peek_master_pubkey().await?;
        self.master_pubkey = Some(pubkey);
        self.update_state().await?;
        let balance = self.peek_balance().await?;
        self.balance = Some(balance);
        Ok(())
    }
    pub async fn update(&mut self, option_status: Option<NockchainStatus>) -> Result<(), String> {
        let Some(status) = option_status else {
            return Err("cannot update wallet state, status is none".to_string());
        };

        match self.block_height {
            Some(latest_block) => {
                if latest_block == status {
                    return Ok(());
                }
            },
            None => {}
        }

        tracing::info!("current block id: {:?}, new block id: {:?}", self.block_height, status);
        self.block_height = Some(status);

        if let Some(last_sync) = self.last_sync {
            // only sync if last sync was more than 20 seconds ago
            if last_sync.elapsed() < std::time::Duration::from_secs(20) {
                tracing::info!("last sync was less than 20 seconds ago, skipping sync");
                return Ok(());
            }
        }
        self.update_state().await?;
        self.balance = Some(self.peek_balance().await?);
        // update history
        Ok(())
    }
    pub async fn get_balance(&self) -> Result<u64, String> {
        let Some(balance) = self.balance else {
            return Err("balance is not set".to_string());
        };
        Ok(balance)
    }
    pub async fn get_master_pubkey(&self) -> Result<String, String> {
        let Some(pubkey) = self.master_pubkey.clone() else {
            return Err("master pubkey is not set".to_string());
        };
        Ok(pubkey)
    }
    //
    // peeks
    //
    pub async fn peek_seedphrase(&self) -> Result<Vec<String>, String> {
        let result = self.send_command(Commands::PeekSeedphrase).await?;
        let phrase = Self::clean_peek_noun(result)?;
        let phrase_atom = phrase
            .as_atom()
            .map_err(|_| "seedphrase: phrase is not an atom".to_string())?;
        let phrase_bytes = phrase_atom.as_ne_bytes();
        let actual_phrase_str = std::str::from_utf8(phrase_bytes)
            .map_err(|e| format!("keygen: phrase atom bytes are not valid UTF-8: {}", e))?
            .to_string();

        // split the phrase at the spaces and return a vector of strings
        let words = actual_phrase_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let cleaned_words = words
            .iter()
            .map(|s| s.replace("\u{0000}", ""))
            .collect::<Vec<String>>();
        Ok(cleaned_words)
    }
    async fn peek_master_pubkey(&self) -> Result<String, String> {
        if self.wallet_name.is_none() {
            return Err("wallet is not loaded".to_string());
        }
        let result = self.send_command(Commands::PeekMasterPubkey).await?;
        let pubkey = Self::clean_peek_noun(result)?;
        let pubkey_atom = pubkey
            .as_atom()
            .map_err(|_| "pubkey: pubkey is not an atom".to_string())?;
        let pubkey_bytes = pubkey_atom.as_ne_bytes();
        let actual_pubkey_str = std::str::from_utf8(pubkey_bytes)
            .map_err(|e| format!("pubkey: pubkey atom bytes are not valid UTF-8: {}", e))?
            .to_string();

        Ok(actual_pubkey_str.replace("\u{0000}", ""))
    }
    async fn peek_balance(&self) -> Result<u64, String> {
        if self.wallet_name.is_none() {
            return Err("wallet is not loaded".to_string());
        }
        let result = self.send_command(Commands::PeekBalance).await?;
        let noun = Self::clean_peek_noun(result)?;
        let atom = noun
            .as_atom()
            .map_err(|_| "balance: noun is not an atom".to_string())?;
        let balance = format!("{:?}", atom)
            .parse::<u64>()
            .map_err(|e| format!("balance: atom is not a valid i64: {}", e))?;

        Ok(balance)
    }
    //
    // pokes
    //
    pub async fn keygen(&self) -> Result<(), String> {
        let _ = self.send_command(Commands::Keygen).await?;
        Ok(())
    }
    pub async fn gen_master_privkey(&self, seedphrase: String) -> Result<(), String> {
        let _ = self.send_command(Commands::GenMasterPrivkey { seedphrase }).await?;
        Ok(())
    }
    pub async fn update_state(&mut self) -> Result<(), String> {
        if self.wallet_name.is_none() {
            return Err("wallet is not loaded".to_string());
        }
        let _ = self.send_command(Commands::UpdateState).await?;
        self.last_sync = Some(std::time::Instant::now());
        Ok(())
    }
    //
    // Helpers
    //
    async fn send_command(&self, command: Commands) -> Result<Vec<NounSlab>, String> {
        let (resp_tx, resp_rx) = oneshot::channel();
        tracing::info!("sending command: {:?}", command);
        self.command_tx
            .send(WalletCommand {
                command,
                response: resp_tx,
            })
            .await
            .map_err(|_| "wallet thread gone".to_string())?;
        resp_rx.await.map_err(|_| "no reply".to_string())?
    }
    fn clean_peek_noun(result: Vec<NounSlab>) -> Result<Noun, String> {
        let effect = unsafe { result[0].root() };
        let unit_noun = effect
            .as_cell()
            .map_err(|_| "invalid noun".to_string())?
            .tail();
        let noun = unit_noun
            .as_cell()
            .map_err(|_| "invalid noun".to_string())?
            .tail();
        Ok(noun)
    }
}