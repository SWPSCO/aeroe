use nockapp::noun::slab::NounSlab;
use nockchain_wallet_lib::Commands;
use nockvm::noun::Noun;
use std::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub struct WalletCommand {
    pub command: Commands,
    pub response: oneshot::Sender<Result<Vec<NounSlab>, String>>,
}

#[derive(Debug)]
pub struct Wallet {
    initialized: bool,
    setup_complete: bool,
    command_tx: Sender<WalletCommand>,
    //nockchain_rx: Receiver<NockchainEvent>,
    master_pubkey: String,
    balance: Option<u64>,
}

impl Wallet {
    // creates new wallet manager
    pub fn new(command_tx: Sender<WalletCommand>) -> Self {
        Self {
            initialized: false,
            setup_complete: false,
            command_tx,
            master_pubkey: String::new(),
            balance: None,
        }
    }
    // checks nockapp state and populates manager struct
    pub async fn initialize(&mut self) -> Result<(), String> {
        let Ok(pubkey) = self.peek_master_pubkey().await else {
            self.initialized = true;
            self.setup_complete = false;
            return Ok(());
        };
        let Ok(balance) = self.peek_balance().await else {
            self.initialized = true;
            self.setup_complete = false;
            return Ok(());
        };

        self.master_pubkey = pubkey;
        self.balance = Some(balance);
        self.setup_complete = true;
        self.initialized = true;
        Ok(())
    }
    // initialization status
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    // wallet keypair exists
    pub fn is_setup_complete(&self) -> bool {
        self.setup_complete
    }
    //
    // fetches from memory
    //
    pub fn get_master_pubkey(&self) -> String {
        self.master_pubkey.clone()
    }
    pub fn get_balance(&self) -> Result<u64, String> {
        let Some(balance) = self.balance else {
            return Err("balance not initialized".to_string());
        };
        Ok(balance)
    }
    //
    // peeks
    //
    pub async fn peek_master_pubkey(&self) -> Result<String, String> {
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
    pub async fn peek_balance(&self) -> Result<u64, String> {
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
    pub async fn gen_master_pubkey(&self, master_privkey: String) -> Result<(), String> {
        let _ = self.send_command(Commands::GenMasterPubkey { master_privkey }).await?;
        Ok(())
    }
    /*
    //
    // listens for new blocks and updates data accordingly
    //
    pub async fn update_info_loop(&self) -> Result<(), String> {
        loop {
            let Ok(pubkey) = self.peek_master_pubkey().await else {
                return Err("failed to get master pubkey".to_string());
            };
        }
    }
    */
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
