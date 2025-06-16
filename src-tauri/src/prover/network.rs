use crate::prover::config;
use nockapp::nockapp::driver::IODriverFn;
use std::path::PathBuf;
use tokio::sync::oneshot;

use libp2p::connection_limits;
use libp2p::identity::Keypair;
use libp2p::multiaddr::Multiaddr;

use tracing::info;

pub fn libp2p_driver(
    libp2p_init_tx: Option<oneshot::Sender<()>>,
    nockchain_dir: PathBuf,
    name: String,
) -> Result<IODriverFn, String> {
    let keypair = load_keypair(nockchain_dir, name)?;
    let bind_multiaddrs: Vec<Multiaddr> = vec!["/ip4/0.0.0.0/udp/0/quic-v1"
        .parse::<Multiaddr>()
        .map_err(|e| e.to_string())?];
    let allowed = None;
    let limits = make_limits()?;
    let memory_limits = None;
    let initial_peer_multiaddrs = list_nodes();
    let force_peers = vec![];
    let equix_builder = equix::EquiXBuilder::new();

    let driver = nockchain_libp2p_io::nc::make_libp2p_driver(
        keypair,
        bind_multiaddrs,
        allowed,
        limits,
        memory_limits,
        &initial_peer_multiaddrs,
        &force_peers,
        equix_builder,
        config::CHAIN_INTERVAL,
        libp2p_init_tx,
    );

    Ok(Box::new(driver))
}

fn load_keypair(nockchain_dir: PathBuf, name: String) -> Result<Keypair, String> {
    let keypair_path = nockchain_dir.join(format!("identities/.{}", name));

    if keypair_path.try_exists().map_err(|e| e.to_string())? {
        let keypair_bytes = std::fs::read(&keypair_path).map_err(|e| e.to_string())?;

        let keypair = libp2p::identity::Keypair::from_protobuf_encoding(&keypair_bytes[..])
            .map_err(|e| e.to_string())?;

        let peer_id = keypair.public().to_peer_id();

        let pubkey_path = keypair_path.with_extension(config::PEER_ID_FILE_EXTENSION);

        if !pubkey_path.exists() {
            info!("Writing pubkey to {pubkey_path:?}");
            std::fs::write(pubkey_path, peer_id.to_base58()).map_err(|e| e.to_string())?;
        }

        info!("Loaded identity as peer {peer_id}");

        Ok(keypair)
    } else {
        if let Some(parent) = keypair_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let new_keypair = libp2p::identity::Keypair::generate_ed25519();

        let new_keypair_bytes = new_keypair
            .to_protobuf_encoding()
            .map_err(|e| e.to_string())?;

        std::fs::write(&keypair_path, new_keypair_bytes).map_err(|e| e.to_string())?;

        let peer_id = new_keypair.public().to_peer_id();

        // write the peer_id encoded as base58 to a file
        std::fs::write(
            keypair_path.with_extension(config::PEER_ID_FILE_EXTENSION),
            peer_id.to_base58(),
        )
        .map_err(|e| e.to_string())?;

        info!("Generated new identity as peer {peer_id}");

        Ok(new_keypair)
    }
}

fn make_limits() -> Result<connection_limits::ConnectionLimits, String> {
    let libp2p_config =
        nockchain_libp2p_io::config::LibP2PConfig::from_env().map_err(|e| e.to_string())?;

    let limits = connection_limits::ConnectionLimits::default()
        .with_max_established_incoming(Some(libp2p_config.max_established_incoming_connections))
        .with_max_established_outgoing(Some(libp2p_config.max_established_outgoing_connections))
        .with_max_pending_incoming(Some(libp2p_config.max_pending_incoming_connections))
        .with_max_pending_outgoing(Some(libp2p_config.max_pending_outgoing_connections))
        .with_max_established(Some(libp2p_config.max_established_connections))
        .with_max_established_per_peer(Some(libp2p_config.max_established_connections_per_peer));

    Ok(limits)
}

fn list_nodes() -> Vec<Multiaddr> {
    let backbone_peers = config::REALNET_BACKBONE_NODES
        .iter()
        .map(|multiaddr_str| {
            multiaddr_str
                .parse()
                .expect("could not parse multiaddr from built-in string")
        })
        .collect();
    backbone_peers
}
