use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum NockchainRequest {
    StartMaster,
    StopMaster,
    // SetWorkers(u64),
    GetStatus,
}

#[derive(Debug)]
pub enum NockchainResponse {
    Success,
    Status {
        master_running: bool,
        num_workers: u64,
    },
}

pub struct NockchainCommand {
    pub command: NockchainRequest,
    pub response: oneshot::Sender<Result<NockchainResponse, String>>,
}

#[derive(Debug)]
pub struct NockchainNode {
    pub command_tx: Sender<NockchainCommand>,
}

impl NockchainNode {
    pub fn new(command_tx: Sender<NockchainCommand>) -> Self {
        Self { command_tx }
    }
    pub async fn start_master(&mut self) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(NockchainCommand {
                command: NockchainRequest::StartMaster,
                response: tx,
            })
            .await
            .map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Success => Ok(()),
                _ => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn stop_master(&mut self) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(NockchainCommand {
                command: NockchainRequest::StopMaster,
                response: tx,
            })
            .await
            .map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Success => Ok(()),
                _ => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    /*
    pub async fn set_workers(&mut self, num_workers: u64) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(NockchainCommand {
            command: NockchainRequest::SetWorkers(num_workers),
            response: tx,
        }).await.map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Success => Ok(()),
                _ => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    */

    pub async fn get_status(&mut self) -> Result<(bool, u64), String> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(NockchainCommand {
                command: NockchainRequest::GetStatus,
                response: tx,
            })
            .await
            .map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Status {
                    master_running,
                    num_workers,
                } => Ok((master_running, num_workers)),
                _ => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
