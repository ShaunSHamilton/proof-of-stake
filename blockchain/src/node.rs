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

    /// Check if Node can afford a server rack
    pub fn can_buy_rack(&self) -> bool {
        self.tokens - self.staked >= 10
    }
    /// Check if a Node can stake, by checking if it has any unstaked tokens
    pub fn can_stake(&self) -> bool {
        self.tokens > self.staked
    }
    /// Check if a Node can unstake, by checking if it has any staked tokens
    pub fn can_unstake(&self) -> bool {
        self.staked > 0
    }
    /// Calculates the miner weight of Node
    pub fn weight_as_miner(&self) -> u64 {
        self.reputation * self.staked
    }
    /// Calculates the validator weight of Node
    pub fn weight_as_validator(&self) -> u64 {
        self.reputation
    }

    /// Validates if two adjacent blocks have been correctly mined
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
            &block.data,
            block.id,
            &block.next_miner,
            &block.next_validators,
            block.nonce,
            &block.previous_hash,
            block.timestamp,
        )) != block.hash
        {
            println!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
