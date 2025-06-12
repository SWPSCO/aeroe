pub mod nockchain_node;
pub mod wallet;

pub use nockchain_node::*;
pub use wallet::*;

pub type NockchainStatus = u32;

#[derive(Debug, Clone)]
pub enum NockchainPeek {
    Height,
}