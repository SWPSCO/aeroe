use std::path::PathBuf;
use clap::Parser;
use nockapp::kernel::boot;
use kernels::dumb::KERNEL;
use zkvm_jetpack::hot::produce_prover_hot_state;

pub struct Prover {
    pub name: String,
    pub nockchain_dir: PathBuf,
}

impl Prover {
    pub fn new(name: String, nockchain_dir: PathBuf) -> Self {
        Self { name, nockchain_dir }
    }

    pub async fn start(&self) -> Result<(), String> {
        let hot_state = produce_prover_hot_state();
        let mut nockapp = boot::setup(
            KERNEL,
            Some(boot::Cli::parse_from(&["nockchain"])),
            &hot_state,
            &self.name,
            Some(self.nockchain_dir.clone()),
        )
        .await.map_err(|e| e.to_string())?;
        nockapp.run().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}