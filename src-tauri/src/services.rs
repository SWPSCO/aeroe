use crate::wallet_app::WalletApp;
use crate::prover::Prover;
use std::panic::AssertUnwindSafe;
use futures::FutureExt;
use crate::manager::{WalletCommand, NockchainResponse, NockchainRequest, NockchainCommand, NockchainStatus};
use tracing::{ info, warn, error };

#[derive(Clone, Debug)]
enum ProverCommand {
    Shutdown,
}

pub fn spawn_wallet_service(mut wallet_rx: tokio::sync::mpsc::Receiver<WalletCommand>, wallet_dir: std::path::PathBuf) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async move {
            info!("[Wallet Service] Started on a dedicated OS thread");
            while let Some(cmd) = wallet_rx.recv().await {
                let command_name = format!("{:?}", cmd.command);
                info!("[Wallet Service] Received command: {}", command_name);

                let future = AssertUnwindSafe(WalletApp::run(cmd.command, wallet_dir.clone()));
                
                match future.catch_unwind().await {
                    Ok(run_result) => {
                        // Task completed successfully (or with a normal error)
                        if cmd.response.send(run_result).is_err() {
                            warn!("[Wallet Service] Command {}: Receiver dropped.", command_name);
                        }
                    },
                    Err(panic_payload) => {
                        // Task panicked
                        let panic_message = if let Some(s) = panic_payload.downcast_ref::<&'static str>() {
                            s.to_string()
                        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "Unknown panic reason".to_string()
                        };

                        error!("[Wallet Service] Command {} panicked: {}", command_name, &panic_message);
                        
                        // Inform the caller about the panic
                        let _ = cmd.response.send(Err(format!("Command panicked: {}", panic_message)));
                    }
                }
            }
            info!("[Wallet Service] Channel closed. Shutting down.");
        });
    });
}

pub fn spawn_nockchain_service(
    mut nockchain_rx: tokio::sync::mpsc::Receiver<NockchainCommand>,
    status_tx: tokio::sync::mpsc::Sender<NockchainStatus>,
    nockchain_dir: std::path::PathBuf,
) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async move {
            info!("[Nockchain Service] Supervisor started on a dedicated OS thread");

            let mut provers: std::collections::HashMap<String, tokio::sync::mpsc::Sender<ProverCommand>> = std::collections::HashMap::new();
            let mut running_intent: std::collections::HashSet<String> = std::collections::HashSet::new();
            let (death_tx, mut death_rx) = tokio::sync::mpsc::channel::<String>(32);
            let mut next_worker_id: u64 = 0;

            loop {
                tokio::select! {
                    Some(cmd) = nockchain_rx.recv() => {
                        let command_name = format!("{:?}", cmd.command);
                        info!("[Nockchain Service] Received command: {}", command_name);

                        match cmd.command {
                            NockchainRequest::StartMaster => {
                                if !provers.contains_key("master") {
                                    running_intent.insert("master".to_string());
                                    start_prover("master".to_string(), nockchain_dir.clone(), &mut provers, status_tx.clone(), death_tx.clone()).await;
                                }
                                let _ = cmd.response.send(Ok(NockchainResponse::Success));
                            },
                            NockchainRequest::StopMaster => {
                                if provers.contains_key("master") {
                                    running_intent.remove("master");
                                    stop_prover("master".to_string(), &mut provers).await;
                                }
                                let _ = cmd.response.send(Ok(NockchainResponse::Success));
                            },
                            NockchainRequest::SetWorkers(num_workers) => {
                                let current_workers = provers.keys().filter(|k| k.starts_with("worker")).count() as u64;

                                if num_workers > current_workers {
                                    for _ in 0..(num_workers - current_workers) {
                                        let worker_id = format!("worker{}", next_worker_id);
                                        next_worker_id += 1;
                                        running_intent.insert(worker_id.clone());
                                        start_prover(worker_id, nockchain_dir.clone(), &mut provers, status_tx.clone(), death_tx.clone()).await;
                                    }
                                } else if num_workers < current_workers {
                                    let workers_to_stop = provers.keys()
                                        .filter(|k| k.starts_with("worker"))
                                        .take((current_workers - num_workers) as usize)
                                        .cloned()
                                        .collect::<Vec<String>>();
                                    for worker_id in workers_to_stop {
                                        running_intent.remove(&worker_id);
                                        stop_prover(worker_id, &mut provers).await;
                                    }
                                }
                                let _ = cmd.response.send(Ok(NockchainResponse::Success));
                            },
                            NockchainRequest::GetStatus => {
                                let master_running = provers.contains_key("master");
                                let num_workers = provers.keys().filter(|k| k.starts_with("worker")).count() as u64;
                                let _ = cmd.response.send(Ok(NockchainResponse::Status { master_running, num_workers }));
                            }
                        }
                    },
                    Some(dead_id) = death_rx.recv() => {
                        info!("[Nockchain Service] Prover '{}' terminated.", dead_id);
                        provers.remove(&dead_id);

                        if running_intent.contains(&dead_id) {
                            warn!("[Nockchain Service] Prover '{}' crashed. Restarting...", dead_id);
                            start_prover(dead_id, nockchain_dir.clone(), &mut provers, status_tx.clone(), death_tx.clone()).await;
                        } else {
                            info!("[Nockchain Service] Prover '{}' was stopped intentionally.", dead_id);
                        }
                    },
                    else => {
                        break;
                    }
                }
            }
            info!("[Nockchain Service] Channel closed. Shutting down.");
        });
    });
}

async fn start_prover(
    id: String,
    nockchain_dir: std::path::PathBuf,
    provers: &mut std::collections::HashMap<String, tokio::sync::mpsc::Sender<ProverCommand>>,
    status_tx: tokio::sync::mpsc::Sender<NockchainStatus>,
    death_tx: tokio::sync::mpsc::Sender<String>,
) {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    provers.insert(id.clone(), tx);

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        
        runtime.block_on(async move {
            let prover_id = id.clone();
            info!("[Prover {}] Starting...", prover_id);

            let mut nc = Prover::new(prover_id.clone(), nockchain_dir, status_tx);
            
            tokio::select! {
                res = nc.start() => {
                    if let Err(e) = res {
                        error!("[Prover {}] Failed: {:?}", prover_id, e);
                    }
                },
                _ = rx.recv() => {
                    info!("[Prover {}] Shutdown signal received.", prover_id);
                }
            }

            info!("[Prover {}] Terminated.", prover_id);
            let _ = death_tx.send(id).await;
        });
    });
}

async fn stop_prover(
    id: String,
    provers: &mut std::collections::HashMap<String, tokio::sync::mpsc::Sender<ProverCommand>>,
) {
    if let Some(tx) = provers.get(&id) {
        info!("[Nockchain Service] Sending shutdown signal to '{}'", id);
        let _ = tx.send(ProverCommand::Shutdown).await;
    }
}
