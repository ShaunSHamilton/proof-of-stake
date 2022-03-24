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
        block.previous_hash = "invalid".to_string();
        assert!(!Node::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_block_hash_not_start_with_difficulty() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.hash = "invalid".to_string();
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
        let data = vec![_fixture_nodes().0];
        let id = 0;
        let previous_hash = "".to_string();
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let next_miner = "Camper".to_string();
        let next_validators = vec!["Tom".to_string(), "Quincy".to_string()];
        let nonce = 0;
        // let gen_hash = calculate_hash(
        //     &data,
        //     id,
        //     &next_miner,
        //     &next_validators,
        //     nonce,
        //     &previous_hash,
        //     timestamp,
        // );
        let genesis = Block {
            id,
            hash: "00101000111000001101001110010010111100111010001010101011111110001100100011101101111001001101000010101101101001111101001001011010011111011110000111101100101100011100000110010001101110010111000101110111110001010101111".to_string(),
            previous_hash,
            timestamp,
            data,
            nonce,
            next_miner,
            next_validators,
        };
        let block = Block {
            id: 1,
            hash: "00210011".to_string(),
            previous_hash: genesis.hash.clone(),
            timestamp: timestamp + 1,
            data: vec![_fixture_nodes().1],
            nonce: 1,
            next_miner: "Tom".to_string(),
            next_validators: vec!["Mrugesh".to_string()],
        };
        (genesis, block)
    }
}
