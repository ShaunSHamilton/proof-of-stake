use chrono::prelude::*;
use serde::{Deserialize, Serialize};

mod mine;
mod node;
use mine::mine_block;
use node::Node;

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

impl Block {
    pub fn new(id: u64, previous_hash: String, data: Vec<Node>) -> Self {
        let now = Utc::now();
        let (nonce, hash, next_miner, next_validators) =
            mine_block(id, now.timestamp() as u64, &previous_hash, &data);
        Self {
            id,
            hash,
            // Should this be re-computed after mining?
            timestamp: now.timestamp() as u64,
            previous_hash,
            data,
            nonce,
            next_miner,
            next_validators,
        }
    }
}
