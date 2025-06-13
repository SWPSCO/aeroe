mod miner;
mod network;
mod config;
mod status;

use clap::Parser;

use std::path::PathBuf;
use tokio::net::UnixListener;

use nockapp::kernel::boot;
use nockapp::npc_listener_driver;
use nockapp::noun::slab::NounSlab;
use nockapp::wire::{SystemWire, Wire};

use nockchain::driver_init;

use crate::manager::{NockchainPeek, NockchainStatus};

use nockvm::noun::{D, T};
use nockvm_macros::tas;

// use kernels::dumb::KERNEL;
static KERNEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dumb.jam"));

use zkvm_jetpack::hot::produce_prover_hot_state;

pub struct Prover {
    name: String,
    nockchain_dir: PathBuf,
    status_receiver_tx: tokio::sync::mpsc::Sender<NockchainStatus>,
    status_caller_rx: tokio::sync::broadcast::Receiver<NockchainPeek>,
}

impl Prover {
    pub fn new(
        name: String,
        nockchain_dir: PathBuf,
        status_receiver_tx: tokio::sync::mpsc::Sender<NockchainStatus>,
        status_caller_rx: tokio::sync::broadcast::Receiver<NockchainPeek>,
    ) -> Self {
        Self {
            name,
            nockchain_dir,
            status_receiver_tx,
            status_caller_rx,
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        // setup nockapp
        let mut nockapp = self.setup().await.map_err(|e| e.to_string())?;

        // setup driver signals
        let mut driver_signals = driver_init::DriverInitSignals::new();

        let mining_init_tx = Some(driver_signals.register_driver("mining"));
        let libp2p_init_tx = Some(driver_signals.register_driver("libp2p"));

        let _born_task = driver_signals.create_born_task();

        // realnet no BTC poke
        nockapp.poke(SystemWire.to_wire(), self.realnet_no_btc()).await.map_err(|e| e.to_string())?;

        // mining_driver
        nockapp.add_io_driver(miner::mining_driver(mining_init_tx)).await;

        // libp2p_driver
        let libp2p_driver = network::libp2p_driver(
            libp2p_init_tx,
            self.nockchain_dir.clone(),
            self.name.clone(),
        )?;
        nockapp.add_io_driver(libp2p_driver).await;

        // born_driver
        nockapp.add_io_driver(driver_signals.create_born_driver()).await;

        // npc_driver
        let socket_path = self.nockchain_dir.clone().join(format!("npc/{}.sock", self.name));
        nockapp.npc_socket_path = Some(socket_path.clone());
        nockapp.add_io_driver(npc_listener_driver(self.npc_socket(socket_path)?)).await;

        // status_driver
        nockapp.add_io_driver(
            status::status_driver(
                self.status_receiver_tx.clone(),
                self.status_caller_rx.resubscribe(),
            )
        ).await;

        // exit_driver
        nockapp.add_io_driver(nockapp::exit_driver()).await;

        // run
        nockapp.run().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn setup(&self) -> Result<nockapp::NockApp, String> {
        let hot_state = produce_prover_hot_state();
        let nockapp = boot::setup(
            KERNEL,
            Some(boot::Cli::parse_from(&["nockchain"])),
            &hot_state,
            &self.name,
            Some(self.nockchain_dir.clone()),
        )
        .await.map_err(|e| e.to_string())?;

        Ok(nockapp)
    }

    fn realnet_no_btc(&self) -> NounSlab {
        // Realnet with no BTC node
        let mut poke_slab = NounSlab::new();
        let poke_noun = T(
            &mut poke_slab,
            &[D(tas!(b"command")), D(tas!(b"btc-data")), D(0)],
        );
        poke_slab.set_root(poke_noun);
        poke_slab
    }
    fn npc_socket(&self, socket_path: PathBuf) -> Result<UnixListener, String> {
        // delete existing socket
        if socket_path.exists() {
            std::fs::remove_file(&socket_path).map_err(|e| e.to_string())?;
        }
        // create socket directory
        if let Some(parent) = socket_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        // bind socket
        let listener = UnixListener::bind(socket_path).map_err(|e| e.to_string())?;
        Ok(listener)
    }
}