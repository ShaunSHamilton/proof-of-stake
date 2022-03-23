use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{block::Block, calculate_hash, hash_to_binary, node::Node, DIFFICULTY_PREFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chain {
    pub chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self { chain: vec![] }
    }
    pub fn get_node_by_name(&self, name: &str) -> Option<&Node> {
        // Search Chain data in reverse
        for block in self.chain.iter().rev() {
            for node in block.data.iter() {
                if node.name == name {
                    return Some(node);
                }
            }
        }
        None
    }
    pub fn get_next_miner(&self) -> String {
        // Search chain for:
        //   - previous block miner (previous miner cannot be next miner)
        let previous_miner = self.get_last_block().next_miner;
        let next_miner = "";

        assert_ne!(previous_miner, next_miner);
        unimplemented!();
    }
    pub fn get_next_validators(&self) -> Vec<String> {
        unimplemented!();
    }
    pub fn mine_block(&mut self, data: &Vec<Node>) {
        println!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100_000 == 0 {
                println!("nonce: {}", nonce);
            }

            let id = self.chain.len() as u64;
            let next_miner = self.get_next_miner();
            let next_validators = self.get_next_validators();
            let previous_hash = self.get_last_block().hash.clone();
            let timestamp = chrono::Utc::now().timestamp() as u64;
            let hash = calculate_hash(
                data,
                id,
                &next_miner,
                &next_validators,
                nonce,
                &previous_hash,
                timestamp,
            );
            let bin_hash = hash_to_binary(&hash);
            if bin_hash.starts_with(DIFFICULTY_PREFIX) {
                println!(
                    "mined! nonce: {}, hash: {}, bin hash: {}",
                    nonce,
                    hex::encode(&hash),
                    bin_hash
                );
                let new_block = Block {
                    id: self.chain.len() as u64,
                    hash: hex::encode(hash),
                    previous_hash: self.get_last_block().hash,
                    timestamp: Utc::now().timestamp() as u64,
                    data: data.clone(),
                    nonce: nonce,
                    next_miner: next_miner,
                    next_validators: next_validators,
                };
                self.chain.push(new_block);
                break;
            }
            nonce += 1;
        }
    }
    pub fn get_last_block(&self) -> Block {
        self.chain.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_chain_returns_empty_vec() {
        let chain: Chain = Chain::new();
        assert_eq!(chain.chain.len(), 0);
    }
}
