// use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: Vec<Node>,
    pub nonce: u64,
    pub next_miner: String,
    pub next_validators: Vec<String>,
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn new_block_returns_block() {
        // let block = Block::new(1, "".to_string(), vec![]);
        // assert_eq!(block.id, 1);
    }
}
