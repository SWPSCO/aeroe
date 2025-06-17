use tokio::time::{sleep, Duration};

use nockapp::driver::IODriverFn;
use nockapp::noun::slab::NounSlab;
use nockapp::utils::make_tas;

use nockvm::noun::{Noun, D, T};

use crate::manager::{NockchainPeek, NockchainStatus};

pub fn status_driver(
    status_receiver_tx: tokio::sync::mpsc::Sender<NockchainStatus>,
    mut status_caller_rx: tokio::sync::broadcast::Receiver<NockchainPeek>,
) -> IODriverFn {
    Box::new(move |handle| {
        Box::pin(async move {
            loop {
                let status = status_caller_rx.recv().await;
                tracing::info!("status: {:?}", status);
                let Ok(peek_command) = status else {
                    tracing::error!("failed to receive status");
                    continue;
                };
                let command = match peek_command {
                    NockchainPeek::Height => "height",
                    NockchainPeek::HeavySummary => "heavy-summary",
                    NockchainPeek::Transactions => "transactions",
                };
                let mut slab = NounSlab::new();
                let Ok(peek) = handle.peek(make_slab(&mut slab, command)).await else {
                    tracing::error!("peek failed");
                    continue;
                };
                let Ok(noun) = clean_peek_noun(peek) else {
                    tracing::error!("invalid peek noun");
                    continue;
                };
                let response = NockchainStatus::new(peek_command, noun);
                let Ok(_) = status_receiver_tx.send(response).await else {
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

fn make_slab(slab: &mut NounSlab, command: &str) -> NounSlab {
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
