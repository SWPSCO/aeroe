use nockapp::noun::slab::NounSlab;
use nockchain_wallet_lib::Commands;
use nockvm::noun::{Noun, D};

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use std::path::PathBuf;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct WalletCommand {
    pub command: Commands,
    pub response: oneshot::Sender<Result<Vec<NounSlab>, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionEntry {
    pub recipient: String,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NockchainTxStatus {
    Draft,
    Signed,
    Pending,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NockchainTxMeta {
    pub draft_id: String,
    pub transactions: Vec<TransactionEntry>,
    pub fee: u64,
    pub created_at: String,
    pub signed_at: Option<String>,
    pub broadcasted_at: Option<String>,
    pub status: NockchainTxStatus,
}

#[derive(Debug)]
pub struct NockchainTx {
    metadata: NockchainTxMeta,
    data: Noun,
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
    drafts: HashMap<String, NockchainTx>,
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
            drafts: HashMap::new(),
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
    pub async fn update(&mut self, new_height: u32) -> Result<(), String> {
        match self.block_height {
            Some(latest_block) => {
                if latest_block == new_height {
                    return Ok(());
                }
            },
            None => {}
        }

        tracing::info!("current block id: {:?}, new block id: {:?}", self.block_height, new_height);
        self.block_height = Some(new_height);

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
    pub async fn create_tx(&mut self, transactions: Vec<TransactionEntry>, fee: u64) -> Result<NockchainTxMeta, String> {
        // if amount of transactions is 0, return error
        if transactions.is_empty() {
            return Err("no transactions".to_string());
        }
        // if fee is 0, return error
        if fee == 0 {
            return Err("fee is 0".to_string());
        }
        // if fee is greater than balance, return error
        let balance = self.get_balance().await?;
        if fee > balance {
            return Err("fee is greater than balance".to_string());
        }
        // if combined amount of transactions is greater than balance, return error
        let total_amount = transactions.iter().map(|t| t.amount).sum::<u64>();
        if total_amount > balance {
            return Err("total amount of transactions is greater than balance".to_string());
        }
        // list notes
        // select appropriate notes
        // construct simple-spend
        // let draft = self.send_command(Commands::SimpleSpend { /* simple spend */ }).await?;
        // let (draft_name, draft_jam) = self.process_draft(draft).await?;
        // save draft to notes

        // temporary - generate random 32 char string
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&now, &mut hasher);
        std::hash::Hash::hash(&std::thread::current().id(), &mut hasher);
        let hash1 = std::hash::Hasher::finish(&hasher);
        
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&(hash1 ^ 0xdeadbeef), &mut hasher2);
        let hash2 = std::hash::Hasher::finish(&hasher2);
        
        let draft_name = format!("{:016x}{:016x}", hash1, hash2);
        let draft_jam = D(0);
        // end temporary

        let draft_id = draft_name.clone();

        let draft_meta = NockchainTxMeta {
            draft_id,
            transactions,
            fee,
            created_at: now.to_string(),
            signed_at: None,
            broadcasted_at: None,
            status: NockchainTxStatus::Draft,
        };
        self.drafts.insert(draft_name.clone(), NockchainTx {
            metadata: draft_meta.clone(),
            data: draft_jam,
        });
        Ok(draft_meta)
    }
    pub async fn sign_tx(&mut self, draft_id: String) -> Result<NockchainTxMeta, String> {
        let Some(draft) = self.drafts.get_mut(&draft_id) else {
            return Err("draft not found".to_string());
        };
        // send command to sign the draft
        /*
        let signed = self.send_command(Commands::SignTx {
            &draft.data // send in the raw data
        }).await?;
        */
        // let data = process_draft(signed)
        // temporary
        let signed = D(1);

        // update the draft
        draft.metadata.status = NockchainTxStatus::Signed;
        draft.data = signed;
        draft.metadata.signed_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string());
        Ok(draft.metadata.clone())
    }
    pub async fn send_tx(&mut self, draft_id: String) -> Result<NockchainTxMeta, String> {
        let Some(draft) = self.drafts.get_mut(&draft_id) else {
            return Err("draft not found".to_string());
        };
        // send command to broadcast the transaction
        /*
        let _ = self.send_command(Commands::MakeTx {
            &draft.data // send in the raw data
        }).await?;
        */

        // update the draft
        draft.metadata.status = NockchainTxStatus::Pending;
        draft.metadata.broadcasted_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string());
        Ok(draft.metadata.clone())
    }
    pub async fn list_unsent_txs(&self) -> Result<HashMap<String, NockchainTxMeta>, String> {
        // self.drafts but only the key and metadata
        let unsent_txs = self.drafts.iter()
            .map(|(k, v)| (k.clone(), v.metadata.clone()))
            .collect::<HashMap<String, NockchainTxMeta>>();
        Ok(unsent_txs)
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