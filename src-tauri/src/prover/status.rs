use tokio::time::{sleep, Duration};

use nockapp::driver::IODriverFn;
use nockapp::noun::slab::NounSlab;
use nockapp::utils::make_tas;

use nockvm::noun::{D, T, Noun};
// use nockvm_macros::tas;

use crate::manager::NockchainStatus;

pub fn status_driver(status_tx: tokio::sync::mpsc::Sender<NockchainStatus>) -> IODriverFn {
    Box::new(move |handle| {
        Box::pin(async move {
            loop {
                let mut slab = NounSlab::new();
                let Ok(peek) = handle.peek(do_peek(&mut slab)).await else {
                    tracing::error!("peek failed");
                    continue;
                };
                let Ok(noun) = clean_peek_noun(peek) else {
                    tracing::error!("invalid peek noun");
                    continue;
                };
                let Ok(atom) = noun.as_atom() else {
                    tracing::error!("invalid noun, not an atom");
                    continue;
                };
                let Ok(height) = format!("{:?}", atom).parse::<u32>() else {
                    tracing::error!("invalid noun, not a valid u32");
                    continue;
                };
                let Ok(_) = status_tx.send(height).await else {
                    tracing::error!("failed to send status");
                    continue;
                };
                // sleep for 1 second
                sleep(Duration::from_secs(5)).await;
            }
            #[allow(unreachable_code)]
            Ok(())

        })
    })
}

fn do_peek(slab: &mut NounSlab) -> NounSlab {
    let command: &str = "height";
    let head = make_tas(slab, command).as_noun();
    let peek_noun = T(slab, &[head, D(0)]);
    slab.set_root(peek_noun);
    slab.clone()
}

fn clean_peek_noun(result: Option<NounSlab>) -> Result<Noun, String> {
    let Some(result) = result else {
        tracing::error!("peek is empty");
        return Err("peek is empty".to_string());
    };
    let effect = unsafe { result.root() };
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