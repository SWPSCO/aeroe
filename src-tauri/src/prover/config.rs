use std::time::Duration;

/** How often we should affirmatively ask other nodes for their heaviest chain */
pub const CHAIN_INTERVAL: Duration = Duration::from_secs(20);

/** Extension for peer ID files */
pub const PEER_ID_FILE_EXTENSION: &str = "peerid";

// we probably should add swps nodes here too
/** Backbone nodes for our realnet */
pub const REALNET_BACKBONE_NODES: &[&str] = &["/dnsaddr/nockchain-backbone.zorp.io"];