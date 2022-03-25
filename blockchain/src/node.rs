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
        } else if !&block.hash.starts_with(DIFFICULTY_PREFIX) {
            println!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            println!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hash_to_binary(&calculate_hash(
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_always_creates_same_node() {
        let node1 = Node::new("node1");
        let node2 = Node::new("node1");
        assert_eq!(node1.name, node2.name);
        assert_eq!(node1.staked, node2.staked);
        assert_eq!(node1.tokens, node2.tokens);
        assert_eq!(node1.reputation, node2.reputation);
    }
    #[test]
    fn cannot_buy_rack_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(!node.can_buy_rack());
    }
    #[test]
    fn can_buy_rack_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(node.can_buy_rack());
    }
    #[test]
    fn can_buy_rack_when_no_rep() {
        let node = _fixture_nodes().2;
        assert!(node.can_buy_rack());
    }
    #[test]
    fn cannot_buy_rack_when_no_tokens() {
        let node = _fixture_nodes().3;
        assert!(!node.can_buy_rack());
    }
    #[test]
    fn cannot_stake_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(!node.can_stake());
    }
    #[test]
    fn can_stake_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(node.can_stake());
    }
    #[test]
    fn can_stake_when_no_rep() {
        let node = _fixture_nodes().2;
        assert!(node.can_stake());
    }
    #[test]
    fn cannot_stake_when_no_tokens() {
        let node = _fixture_nodes().3;
        assert!(!node.can_stake());
    }
    #[test]
    fn can_unstake_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(node.can_unstake());
    }
    #[test]
    fn cannot_unstake_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(!node.can_unstake());
    }
    #[test]
    fn can_unstake_when_no_rep() {
        let node = _fixture_nodes().2;
        assert!(node.can_unstake());
    }
    #[test]
    fn cannot_unstake_when_no_tokens() {
        let node = _fixture_nodes().3;
        assert!(!node.can_unstake());
    }
    #[test]
    fn all_staked_miner_weight() {
        let node = _fixture_nodes().0;
        assert_eq!(node.weight_as_miner(), 800);
    }
    #[test]
    fn all_unstaked_miner_weight() {
        let node = _fixture_nodes().1;
        assert_eq!(node.weight_as_miner(), 0);
    }
    #[test]
    fn no_rep_miner_weight() {
        let node = _fixture_nodes().2;
        assert_eq!(node.weight_as_miner(), 0);
    }
    #[test]
    fn no_tokens_miner_weight() {
        let node = _fixture_nodes().3;
        assert_eq!(node.weight_as_miner(), 0);
    }
    #[test]
    fn all_staked_validator_weight() {
        let node = _fixture_nodes().0;
        assert_eq!(node.weight_as_validator(), 8);
    }
    #[test]
    fn all_unstaked_validator_weight() {
        let node = _fixture_nodes().1;
        assert_eq!(node.weight_as_validator(), 8);
    }
    #[test]
    fn no_rep_validator_weight() {
        let node = _fixture_nodes().2;
        assert_eq!(node.weight_as_validator(), 0);
    }
    #[test]
    fn no_tokens_validator_weight() {
        let node = _fixture_nodes().3;
        assert_eq!(node.weight_as_validator(), 1);
    }
    #[test]
    fn invalidate_block_unequal_previous_hash() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.previous_hash = block.previous_hash.replace("1", "0");
        assert!(!Node::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_block_hash_not_start_with_difficulty() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.hash = block.previous_hash.replace("0", "1"); //"011111101111000110011110001110011001111011011010011011000101110010100101001001111001110001011010111000010011000010000100101000011010111110001110010011110101011000011101011110011001110010111001011011011111111010110100000".to_string();
        assert!(!Node::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_block_id_not_incremented() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.id = previous_block.id;
        assert!(!Node::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_encoded_hash_not_correct() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.hash = "001111101111000110011110001110011001111011011010011011000101110010100101001001111001110001011010111000010011000010000100101000011010111110001110010011110101011000011101011110011001110010111001011011011111111010110100000".to_string();
        assert!(!Node::validate_block(&block, &previous_block));
    }
    #[test]
    fn validate_block_correct() {
        let previous_block = _fixture_blocks().0;
        let block = _fixture_blocks().1;
        assert!(Node::validate_block(&block, &previous_block));
    }

    fn _fixture_nodes() -> (Node, Node, Node, Node) {
        let all_staked = Node {
            name: "Shaun".to_string(),
            staked: 100,
            tokens: 100,
            reputation: 8,
        };
        let all_unstaked = Node {
            name: "Tom".to_string(),
            staked: 0,
            tokens: 100,
            reputation: 8,
        };
        let no_rep = Node {
            name: "Ahmad".to_string(),
            staked: 80,
            tokens: 100,
            reputation: 0,
        };
        let no_tokens = Node {
            name: "Quincy".to_string(),
            staked: 0,
            tokens: 0,
            reputation: 1,
        };
        (all_staked, all_unstaked, no_rep, no_tokens)
    }
    fn _fixture_blocks() -> (Block, Block) {
        let genesis = Block { id: 0, hash: "001110000110011011000111111110001101110110100101100111101000011010011101010011010101101001111101110111101000011011000101100110100100011001000011010001101101011000011010100001110111011010111110000111010100000100100111101".to_string(), previous_hash: String::new(), timestamp: 1648206521, data: vec![Node { name: "Camper".to_string(), staked: 0, tokens: 10, reputation: 1 }, Node { name: "Tom".to_string(), staked: 0, tokens: 10, reputation: 0 }, Node { name: "Mrugesh".to_string(), staked: 0, tokens: 10, reputation: 0 }], nonce: 71904, next_miner: "Camper".to_string(), next_validators: vec!["Camper".to_string(), "Camper".to_string(), "Camper".to_string()] };

        let block = Block { id: 1, hash: "001110101010100110001011011111001111001111001001111000110000011101110100010010111001100100010001001100101101110111110001010010010000001111011011110110110001001011111001100110110110010001001011010010000".to_string(), previous_hash: "001110000110011011000111111110001101110110100101100111101000011010011101010011010101101001111101110111101000011011000101100110100100011001000011010001101101011000011010100001110111011010111110000111010100000100100111101".to_string(), timestamp: 1648206532, data: vec![Node { name: "Ahmad".to_string(), staked: 0, tokens: 10, reputation: 0 }], nonce: 89248, next_miner: "Mrugesh".to_string(), next_validators: vec!["Tom".to_string(), "Tom".to_string(), "Tom".to_string()] };

        (genesis, block)
    }
}
