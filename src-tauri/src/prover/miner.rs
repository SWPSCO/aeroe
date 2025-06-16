use nockapp::nockapp::driver::IODriverFn;
use tokio::sync::oneshot;

pub fn mining_driver(mining_init_tx: Option<oneshot::Sender<()>>) -> IODriverFn {
    Box::new(move |mut _handle| {
        Box::pin(async move {
            if let Some(tx) = mining_init_tx {
                tx.send(()).map_err(|_| {
                    tracing::warn!("Could not send driver initialization for mining driver.");
                    nockapp::nockapp::NockAppError::OtherError
                })?;
            }
            Ok(())
        })
    })
}
