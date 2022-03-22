use serde::{Deserialize, Serialize};

use crate::{block::Block, calculate_hash, hash_to_binary, DIFFICULTY_PREFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub name: String,
    pub staked: u64,
    pub tokens: u64,
    pub reputation: u64,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            staked: 0,
            tokens: 10,
            reputation: 0,
        }
    }
    pub fn get_next_miner() -> String {
        // Search chain for:
        //   - previous block miner (previous miner cannot be next miner)
        unimplemented!();
    }

    pub fn get_next_validators() -> Vec<String> {
        unimplemented!();
    }
    pub fn validate_block(block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            println!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary(&hex::decode(&block.hash).expect("can decode"))
            .starts_with(DIFFICULTY_PREFIX)
        {
            println!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            println!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
            &block.next_miner,
            &block.next_validators,
        )) != block.hash
        {
            println!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }
    pub fn mine_block(
        id: u64,
        timestamp: u64,
        previous_hash: &str,
        data: &Vec<Node>,
    ) -> (u64, String, String, Vec<String>) {
        println!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100_000 == 0 {
                println!("nonce: {}", nonce);
            }

            let next_miner = Node::get_next_miner();
            let next_validators = Node::get_next_validators();

            let hash = calculate_hash(
                id,
                timestamp,
                previous_hash,
                data,
                nonce,
                &next_miner,
                &next_validators,
            );
            let bin_hash = hash_to_binary(&hash);
            if bin_hash.starts_with(DIFFICULTY_PREFIX) {
                println!(
                    "mined! nonce: {}, hash: {}, bin hash: {}",
                    nonce,
                    hex::encode(&hash),
                    bin_hash
                );
                return (nonce, hex::encode(hash), next_miner, next_validators);
            }
            nonce += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_next_miner_returns_string() {
        let result = Node::get_next_miner();
        assert_eq!(result, "Camper");
    }
    #[test]
    fn mine_block_returns_tuple() {
        let (nonce, hash, next_miner, next_validators) = Node::mine_block(1, 1, "", &vec![]);
        assert_eq!(nonce, 0);
        assert_eq!(hash.len(), 64);
        assert_eq!(next_miner.len(), 64);
        assert_eq!(next_validators.len(), 0);
    }
}
