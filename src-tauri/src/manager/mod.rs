pub mod nockchain_node;
pub mod wallet;

pub use nockchain_node::*;
pub use wallet::*;

use nockvm::noun::Noun;

#[derive(Debug, Clone, PartialEq)]
pub enum NockchainPeek {
    Height,
    HeavySummary,
    Transactions,
}

pub struct NockchainStatus {
    pub command: NockchainPeek,
    pub noun: Noun,
}
impl NockchainStatus {
    pub fn new(command: NockchainPeek, noun: Noun) -> Self {
        Self { command, noun }
    }
    pub fn command(&self) -> &NockchainPeek {
        &self.command
    }
    pub fn height(&self) -> Result<u32, String> {
        if self.command != NockchainPeek::Height {
            return Err(format!("not a height command: {:?}", self.command()));
        }
        let Ok(atom) = self.noun.as_atom() else {
            return Err(format!("invalid noun, not an atom: {:?}", self.noun));
        };
        let Ok(height) = format!("{:?}", atom).parse::<u32>() else {
            return Err("invalid noun, not a valid u32".to_string());
        };
        Ok(height)
    }
}
