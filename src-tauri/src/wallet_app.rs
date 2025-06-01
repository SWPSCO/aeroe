use clap::Parser;
use getrandom::getrandom;
use tokio::net::UnixStream;
use tracing::{error, info};

use nockchain_wallet_lib::{Commands, KeyType, Wallet};

use kernels::wallet::KERNEL;

use zkvm_jetpack::hot::produce_prover_hot_state;

use nockapp::kernel::boot::{self, Cli};
use nockapp::nockapp::NockApp;
use nockapp::noun::slab::NounSlab;
use nockapp::{exit_driver, file_driver, markdown_driver, one_punch_driver};

pub struct WalletApp {}

impl WalletApp {
    pub async fn run(
        command: Commands,
        data_dir: std::path::PathBuf,
    ) -> Result<Vec<NounSlab>, String> {
        let requires_sync = match command {
            // Commands that DON'T need sync
            Commands::PeekBalance
            | Commands::PeekSeedphrase
            | Commands::PeekMasterPubkey
            | Commands::PeekState
            | Commands::PeekReceiveAddress
            | Commands::Keygen
            | Commands::DeriveChild { .. }
            | Commands::ImportKeys { .. }
            | Commands::SignTx { .. }
            | Commands::MakeTx { .. }
            | Commands::ExportKeys
            | Commands::GenMasterPrivkey { .. }
            | Commands::GenMasterPubkey { .. }
            | Commands::ImportMasterPubkey { .. }
            | Commands::ExportMasterPubkey
            | Commands::ListPubkeys
            | Commands::ShowSeedphrase
            | Commands::ShowMasterPubkey
            | Commands::ShowMasterPrivkey
            | Commands::SimpleSpend { .. } => false,

            // All other commands DO need sync
            _ => true,
        };

        let poke = match command {
            // Peeks temporary location
            Commands::PeekSeedphrase => {
                let (noun, _op) = Wallet::peek_seedphrase().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }
            Commands::PeekBalance => {
                let (noun, _op) = Wallet::peek_balance().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }
            Commands::PeekMasterPubkey => {
                let (noun, _op) = Wallet::peek_master_pubkey().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }
            Commands::PeekState => {
                let (noun, _op) = Wallet::peek_state().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }
            Commands::PeekReceiveAddress => {
                let (noun, _op) = Wallet::peek_receive_address().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }
            Commands::PeekPubkeys => {
                let (noun, _op) = Wallet::peek_pubkeys().map_err(|e| e.to_string())?;
                return Ok(Self::do_peek(noun, data_dir).await?);
            }

            // Pokes start here
            Commands::Keygen => {
                let mut entropy = [0u8; 32];
                let mut salt = [0u8; 16];
                getrandom(&mut entropy)
                    .map_err(|e| format!("Failed to generate entropy: {}", e))?;
                getrandom(&mut salt).map_err(|e| format!("Failed to generate salt: {}", e))?;
                Wallet::keygen(&entropy, &salt).map_err(|e| e.to_string())?
            }
            Commands::DeriveChild {
                key_type,
                index,
                label,
            } => {
                let key_type = match key_type.as_str() {
                    "pub" => KeyType::Pub,
                    "priv" => KeyType::Prv,
                    _ => {
                        return Err(format!("Key type must be either 'pub' or 'priv'"));
                    }
                };
                Wallet::derive_child(key_type, index, label).map_err(|e| e.to_string())?
            }
            Commands::ExportKeys => Wallet::export_keys().map_err(|e| e.to_string())?,
            Commands::ImportKeys { input } => {
                Wallet::import_keys(&input).map_err(|e| e.to_string())?
            }
            Commands::SignTx { draft, index } => {
                Wallet::sign_tx(&draft, index).map_err(|e| e.to_string())?
            }
            Commands::MakeTx { draft } => Wallet::make_tx(&draft).map_err(|e| e.to_string())?,
            Commands::GenMasterPrivkey { seedphrase } => {
                Wallet::gen_master_privkey(&seedphrase).map_err(|e| e.to_string())?
            }
            Commands::GenMasterPubkey { master_privkey } => {
                Wallet::gen_master_pubkey(&master_privkey).map_err(|e| e.to_string())?
            }
            Commands::ImportMasterPubkey { key_path } => {
                Wallet::import_master_pubkey(&key_path).map_err(|e| e.to_string())?
            }
            Commands::ExportMasterPubkey => {
                Wallet::export_master_pubkey().map_err(|e| e.to_string())?
            }
            Commands::ListPubkeys => Wallet::list_pubkeys().map_err(|e| e.to_string())?,
            Commands::ShowSeedphrase => Wallet::show_seedphrase().map_err(|e| e.to_string())?,
            Commands::ShowMasterPubkey => {
                Wallet::show_master_pubkey().map_err(|e| e.to_string())?
            }
            Commands::ShowMasterPrivkey => {
                Wallet::show_master_privkey().map_err(|e| e.to_string())?
            }
            Commands::SimpleSpend {
                names,
                recipients,
                gifts,
                fee,
            } => Wallet::simple_spend(names.clone(), recipients.clone(), gifts.clone(), fee)
                .map_err(|e| e.to_string())?,
            // Sync
            Commands::Scan {
                master_pubkey,
                search_depth,
                include_timelocks,
                include_multisig,
            } => Wallet::scan(
                &master_pubkey,
                search_depth,
                include_timelocks,
                include_multisig,
            )
            .map_err(|e| e.to_string())?,
            Commands::ListNotes => Wallet::list_notes().map_err(|e| e.to_string())?,
            Commands::ListNotesByPubkey { pubkey } => {
                if let Some(pk) = pubkey {
                    Wallet::list_notes_by_pubkey(&pk).map_err(|e| e.to_string())?
                } else {
                    return Err(format!("Public key is required"));
                }
            }
            Commands::UpdateBalance => Wallet::update_balance().map_err(|e| e.to_string())?,
        };

        // If this command requires sync and we have a socket, wrap it with sync-run
        let final_poke = if requires_sync {
            Wallet::wrap_with_sync_run(poke.0, poke.1).map_err(|e| e.to_string())?
        } else {
            poke
        };

        let kernel = Self::make_kernel(data_dir).await?;
        let mut wallet = Wallet::new(kernel);

        wallet
            .app
            .add_io_driver(one_punch_driver(final_poke.0, final_poke.1))
            .await;

        {
            if requires_sync {
                // let socket_path = nockchain_dir.join("nockchain.sock");
                let socket_path = std::path::PathBuf::from(
                    "/Users/chuah/SWPS/nockchain_zorp/miner-node/nockchain.sock",
                );
                match UnixStream::connect(&socket_path).await {
                    Ok(stream) => {
                        info!("Connected to nockchain NPC socket at {:?}", socket_path);
                        wallet
                            .app
                            .add_io_driver(nockapp::npc_client_driver(stream))
                            .await;
                    }
                    Err(e) => {
                        error!(
                            "Failed to connect to nockchain NPC socket at {:?}: {}\n\
                                This could mean:\n\
                                1. Nockchain is not running\n\
                                2. The socket path is incorrect\n\
                                3. The socket file exists but is stale (try removing it)\n\
                                4. Insufficient permissions to access the socket",
                            socket_path, e
                        );
                    }
                }
            }

            wallet.app.add_io_driver(file_driver()).await;
            wallet.app.add_io_driver(markdown_driver()).await;
            wallet.app.add_io_driver(exit_driver()).await;
            wallet
                .app
                .run()
                .await
                .map_err(|e| format!("wallet run failed: {}", e))?;
            Ok(vec![NounSlab::new()])
        }
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

    async fn do_peek(
        noun: NounSlab,
        data_dir: std::path::PathBuf,
    ) -> Result<Vec<NounSlab>, String> {
        let kernel = Self::make_kernel(data_dir).await?;
        let mut wallet = Wallet::new(kernel);
        let res = wallet
            .app
            .peek(noun)
            .await
            .map_err(|e| format!("peek failed: {}", e))?;
        Ok(vec![res])
    }
}
