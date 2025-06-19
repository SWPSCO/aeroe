use nockapp::nockapp::driver::IODriverFn;
use nockapp::AtomExt;
use nockapp::wire::Wire;

use tokio::sync::oneshot;
use nockchain::mining::MiningWire;

use nockvm_macros::tas;
use nockapp::noun::slab::NounSlab;
use nockvm::noun::{D, T, Atom, YES, NO};

pub fn mining_driver(mining_init_tx: Option<oneshot::Sender<()>>) -> IODriverFn {
    Box::new(move |handle| {
        Box::pin(async move {
            if let Some(tx) = mining_init_tx {
                tx.send(()).map_err(|_| {
                    tracing::warn!("Could not send driver initialization for mining driver.");
                    nockapp::nockapp::NockAppError::OtherError
                })?;
            }

            let enable = false;
            let mut enable_mining_slab = NounSlab::new();
            let enable_mining = Atom::from_value(&mut enable_mining_slab, "enable-mining")
                .expect("Failed to create enable-mining atom");
            let enable_mining_poke = T(
                &mut enable_mining_slab,
                &[D(tas!(b"command")), enable_mining.as_noun(), if enable { YES } else { NO }],
            );
            enable_mining_slab.set_root(enable_mining_poke);
            let _ = handle
                .poke(MiningWire::Enable.to_wire(), enable_mining_slab)
                .await;
            Ok(())
        })
    })
}
