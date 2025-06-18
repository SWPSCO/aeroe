use clap::Parser;
use getrandom::getrandom;
use std::path::PathBuf;
use tokio::net::UnixStream;
use tracing::{error, info};

use nockchain_wallet_lib::{Commands, Wallet};

use zkvm_jetpack::hot::produce_prover_hot_state;

use nockapp::kernel::boot::{self, Cli};
use nockapp::nockapp::NockApp;
use nockapp::noun::slab::NounSlab;
use nockapp::driver::Operation;
use nockapp::{exit_driver, file_driver, markdown_driver, one_punch_driver};
static KERNEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wal.jam"));

pub struct WalletApp {}

impl WalletApp {
    pub async fn run(
        command: Commands,
        data_dir: std::path::PathBuf,
        master_socket: std::path::PathBuf,
    ) -> Result<Vec<NounSlab>, String> {

        let res = match command {
            // Peeks
            Commands::PeekSeedphrase => {
                let (noun, _p) = Wallet::peek_seedphrase().map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekBalance { pubkey } => {
                let (noun, _op) = Wallet::peek_balance(&pubkey).map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekMasterPubkey => {
                let (noun, _op) = Wallet::peek_master_pubkey().map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekState => {
                let (noun, _op) = Wallet::peek_state().map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekReceiveAddress => {
                let (noun, _op) = Wallet::peek_receive_address().map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekPubkeys => {
                let (noun, _op) = Wallet::peek_pubkeys().map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            Commands::PeekNotes { pubkey } => {
                let (noun, _op) = Wallet::peek_notes(&pubkey).map_err(|e| e.to_string())?;
                Self::do_peek(noun, data_dir).await?
            }
            // Pokes
            Commands::UpdateState => {
                let (noun, op) = Wallet::update_state().map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            Commands::AeroeSpend { names, recipients, gifts, fee, file_path } => {
                let (noun, op) = Wallet::aeroe_spend(
                    names.clone(),
                    recipients.clone(),
                    gifts.clone(),
                    fee,
                    file_path.clone(),
                )
                .map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            Commands::SignAeroeTx { draft, index, file_path } => {
                let (noun, op) = Wallet::sign_aeroe_tx(&draft, index, file_path.clone()).map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            Commands::MakeTx { draft } => {
                let (noun, op) = Wallet::make_tx(&draft).map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            Commands::Keygen => {
                let mut entropy = [0u8; 32];
                let mut salt = [0u8; 16];
                getrandom(&mut entropy).map_err(|e| format!("Failed to generate entropy: {}", e))?;
                getrandom(&mut salt).map_err(|e| format!("Failed to generate salt: {}", e))?;
                let (noun, op) = Wallet::keygen(&entropy, &salt).map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            Commands::GenMasterPrivkey { seedphrase } => {
                let (noun, op) = Wallet::gen_master_privkey(&seedphrase).map_err(|e| e.to_string())?;
                Self::do_poke(noun, op, data_dir, master_socket).await?
            }
            _ => return Err(format!("command not allowed: {:?}", command)),
        };
        Ok(res)
    }

    async fn make_kernel(data_dir: std::path::PathBuf) -> Result<NockApp, String> {
        let cli = Cli::parse_from(vec!["wallet"]);

        let prover_hot_state = produce_prover_hot_state();

        let kernel = boot::setup(
            KERNEL,
            Some(cli),
            prover_hot_state.as_slice(),
            "wallet",
            Some(data_dir),
        )
        .await
        .map_err(|e| format!("Kernel setup failed: {}", e))?;
        Ok(kernel)
    }

    async fn do_peek(noun: NounSlab, data_dir: PathBuf) -> Result<Vec<NounSlab>, String> {
        let kernel = Self::make_kernel(data_dir).await?;
        let mut wallet = Wallet::new(kernel);
        let res = wallet.app.peek(noun).await.map_err(|e| format!("peek failed: {}", e))?;
        Ok(vec![res])
    }
    async fn do_poke(noun: NounSlab, op: Operation, data_dir: PathBuf, master_socket: PathBuf) -> Result<Vec<NounSlab>, String> {
        let kernel = Self::make_kernel(data_dir).await?;
        let mut wallet = Wallet::new(kernel);

        let one_punch = one_punch_driver(noun, op);
        wallet.app.add_io_driver(one_punch).await;

        {
            match UnixStream::connect(&master_socket).await {
                Ok(stream) => {
                    info!("Connected to nockchain NPC socket at {:?}", master_socket);
                    wallet
                        .app
                        .add_io_driver(nockapp::npc_client_driver(stream))
                        .await;
                }
                Err(e) => error!("failed to connect to nockchain NPC socket at {:?}: {}", master_socket, e),
            }

            wallet.app.add_io_driver(file_driver()).await;
            wallet.app.add_io_driver(markdown_driver()).await;
            wallet.app.add_io_driver(exit_driver()).await;
            wallet.app.run().await.map_err(|e| format!("wallet run failed: {}", e))?;
            Ok(vec![NounSlab::new()])
        }
    }
}
