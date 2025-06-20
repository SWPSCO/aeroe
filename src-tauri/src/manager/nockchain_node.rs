use tokio::sync::oneshot;

#[derive(Debug)]
pub enum NockchainRequest {
    StartMaster,
    StopMaster,
    ConnectExternal(std::path::PathBuf),
    DisconnectExternal,
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

#[derive(Debug)]
pub enum NodeMode {
    Local,
    External(std::path::PathBuf),
    Disconnected,
}


#[derive(Debug)]
pub struct NockchainNode {
    tx: tokio::sync::mpsc::Sender<NockchainCommand>,
    mode: NodeMode,
}

#[derive(Debug)]
pub struct NockchainCommand {
    pub command: NockchainRequest,
    pub response: oneshot::Sender<Result<NockchainResponse, String>>,
}

impl NockchainNode {
    pub fn new(tx: tokio::sync::mpsc::Sender<NockchainCommand>) -> Self {
        Self {
            tx,
            mode: NodeMode::Disconnected,
        }
    }

    pub async fn start_master(&mut self) -> Result<(), String> {
        match &self.mode {
            NodeMode::External(_) => {
                Err("Cannot start local node when connected to external socket".to_string())
            }
            _ => {
                let (response_tx, response_rx) = oneshot::channel();
                let cmd = NockchainCommand {
                    command: NockchainRequest::StartMaster,
                    response: response_tx,
                };

                self.tx
                    .send(cmd)
                    .await
                    .map_err(|e| format!("Failed to send start command: {}", e))?;

                match response_rx.await {
                    Ok(Ok(_)) => {
                        self.mode = NodeMode::Local;
                        Ok(())
                    }
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(format!("Command response error: {}", e)),
                }
            }
        }
    }

    pub async fn stop_master(&mut self) -> Result<(), String> {
        match &self.mode {
            NodeMode::Local => {
                let (response_tx, response_rx) = oneshot::channel();
                let cmd = NockchainCommand {
                    command: NockchainRequest::StopMaster,
                    response: response_tx,
                };

                self.tx
                    .send(cmd)
                    .await
                    .map_err(|e| format!("Failed to send stop command: {}", e))?;

                match response_rx.await {
                    Ok(Ok(_)) => {
                        self.mode = NodeMode::Disconnected;
                        Ok(())
                    }
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(format!("Command response error: {}", e)),
                }
            }
            NodeMode::External(_) => Err("Cannot stop external node".to_string()),
            NodeMode::Disconnected => Err("No node running to stop".to_string()),
        }
    }

    pub async fn connect_external(&mut self, socket_path: &str) -> Result<(), String> {
      let path = std::path::PathBuf::from(socket_path);
      
      tracing::info!("[NockchainNode] Attempting to connect to external socket: {:?}", path);
      tracing::info!("[NockchainNode] Current mode before connect: {:?}", self.mode);
      
      if !path.exists() {
          tracing::error!("[NockchainNode] Socket path does not exist: {}", socket_path);
          return Err(format!("Socket path does not exist: {}", socket_path));
      }
  
      let (response_tx, response_rx) = oneshot::channel();
      let cmd = NockchainCommand {
          command: NockchainRequest::ConnectExternal(path.clone()),
          response: response_tx,
      };
  
      tracing::info!("[NockchainNode] Sending ConnectExternal command to service");
      
      self.tx
          .send(cmd)
          .await
          .map_err(|e| format!("Failed to send connect command: {}", e))?;
  
      tracing::info!("[NockchainNode] Waiting for response from service");
      
      match response_rx.await {
          Ok(Ok(_)) => {
              tracing::info!("[NockchainNode] Service returned success, updating mode to External");
              self.mode = NodeMode::External(path.clone());
              tracing::info!("[NockchainNode] Mode updated to: {:?}", self.mode);
              tracing::info!("[NockchainNode] is_connected() now returns: {}", self.is_connected());
              Ok(())
          }
          Ok(Err(e)) => {
              tracing::error!("[NockchainNode] Service returned error: {}", e);
              Err(e)
          }
          Err(e) => {
              tracing::error!("[NockchainNode] Response channel error: {}", e);
              Err(format!("Command response error: {}", e))
          }
      }
    }

    pub async fn disconnect_external(&mut self) -> Result<(), String> {
        match &self.mode {
            NodeMode::External(_) => {
                let (response_tx, response_rx) = oneshot::channel();
                let cmd = NockchainCommand {
                    command: NockchainRequest::DisconnectExternal,
                    response: response_tx,
                };

                tracing::info!("[NockchainNode] Sending DisconnectExternal command");

                self.tx
                    .send(cmd)
                    .await
                    .map_err(|e| format!("Failed to send disconnect command: {}", e))?;

                match response_rx.await {
                    Ok(Ok(_)) => {
                        tracing::info!("[NockchainNode] External node disconnected successfully");
                        self.mode = NodeMode::Disconnected;
                        Ok(())
                    }
                    Ok(Err(e)) => {
                        tracing::error!("[NockchainNode] Failed to disconnect external node: {}", e);
                        Err(e)
                    }
                    Err(e) => Err(format!("Command response error: {}", e)),
                }
            }
            NodeMode::Local => Err("Cannot disconnect external node when using local node".to_string()),
            NodeMode::Disconnected => Err("No external node connected".to_string()),
        }
    }

    pub async fn get_status(&mut self) -> Result<(bool, u64), String> {
        let (tx, rx) = oneshot::channel();
        self.tx
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

    pub fn is_connected(&self) -> bool {
        !matches!(self.mode, NodeMode::Disconnected)
    }

    pub fn get_mode(&self) -> &NodeMode {
        &self.mode
    }

    pub fn socket_path(&self) -> Option<&std::path::Path> {
        match &self.mode {
            NodeMode::External(path) => {
                tracing::info!("[NockchainNode] Returning external socket path: {:?}", path);
                Some(path)
            },
            NodeMode::Local => {
                tracing::info!("[NockchainNode] In Local mode, returning None to use default socket");
                None
            },
            NodeMode::Disconnected => {
                tracing::info!("[NockchainNode] In Disconnected mode, returning None to use default socket");
                None
            },
        }
    }
}
