use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum NockchainRequest {
    StartMaster,
    StopMaster,
    // StartWorker,
    // StopWorker,
    // GetMasterStatus,
    // GetNumWorkers,
}

#[derive(Debug)]
pub enum NockchainResponse {
    Success,
    // NoChange,
    Error(String),
    // MasterStatus(bool),
    // NumWorkers(u64),
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
        self.command_tx.send(NockchainCommand {
            command: NockchainRequest::StartMaster,
            response: tx,
        }).await.map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Success => Ok(()),
                // NockchainResponse::NoChange => Ok(()),
                NockchainResponse::Error(e) => Err(e),
                // NockchainResponse::MasterStatus(status) => Err("invalid response".to_string()),
                // NockchainResponse::NumWorkers(num_workers) => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn stop_master(&mut self) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(NockchainCommand {
            command: NockchainRequest::StopMaster,
            response: tx,
        }).await.map_err(|e| e.to_string())?;

        let res = rx.await.map_err(|e| e.to_string())?;
        match res {
            Ok(res) => match res {
                NockchainResponse::Success => Ok(()),
                // NockchainResponse::NoChange => Ok(()),
                NockchainResponse::Error(e) => Err(e),
                // NockchainResponse::MasterStatus(status) => Err("invalid response".to_string()),
                // NockchainResponse::NumWorkers(num_workers) => Err("invalid response".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}