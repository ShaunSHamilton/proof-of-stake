use chrono::Utc;
use rand::Rng;
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

    pub fn get_last_block(&self) -> Block {
        self.chain.last().unwrap().clone()
    }

    pub fn get_next_miner(&self) -> String {
        let previous_miner = self.get_last_block().next_miner;
        // Get all nodes except the previous miner
        let mut nodes: Vec<&Node> = self
            .get_nodes()
            .into_iter()
            .filter(|node| node.name != previous_miner)
            .collect();

        nodes.sort_by(|a, b| a.weight_as_miner().cmp(&b.weight_as_miner()));
        let cumulative_weight = nodes
            .iter()
            .fold(0, |acc, node| acc + node.weight_as_miner());
        let cumulative_weights = nodes
            .iter()
            .map(|node| node.weight_as_miner() as f64 / cumulative_weight as f64)
            .collect::<Vec<f64>>();

        let rand_num = rand::thread_rng().gen::<f64>();
        let mut ind = 0;
        for (i, c_m) in cumulative_weights.iter().enumerate() {
            ind = i;
            if c_m > &rand_num {
                break;
            }
        }

        let next_miner = nodes[ind].name.clone();

        assert_ne!(previous_miner, next_miner);
        next_miner
    }
    pub fn get_next_validators(&self, next_miner: &String) -> Vec<String> {
        let next_miner_reputation = self
            .get_node_by_name(next_miner)
            .expect("can only find existing nodes")
            .reputation;

        let mut nodes = self
            .get_nodes()
            .into_iter()
            .filter(|node| &node.name != next_miner)
            .collect::<Vec<&Node>>();
        let mut max_reputation = 0;
        for node in nodes.iter() {
            if node.reputation > max_reputation {
                max_reputation = node.reputation;
            }
        }
        // TODO: calculate num_needed_validators from rep;
        // let num_available_validators = nodes.len();

        let num_needed_validators = match next_miner_reputation {
            0..=5 => 3,
            6..=10 => 2,
            _ => 1,
        };

        nodes.sort_by(|a, b| a.weight_as_validator().cmp(&b.weight_as_validator()));

        let cumulative_weight = nodes
            .iter()
            .fold(0, |acc, node| acc + node.weight_as_validator());
        let cumulative_weights = nodes
            .iter()
            .map(|node| node.weight_as_validator() as f64 / cumulative_weight as f64)
            .collect::<Vec<f64>>();

        let mut next_validators = vec![];
        for _ in 0..num_needed_validators {
            let rand_num = rand::thread_rng().gen::<f64>();
            let mut ind = 0;
            for (i, c_m) in cumulative_weights.iter().enumerate() {
                if c_m > &rand_num {
                    break;
                }
                ind = i;
            }
            next_validators.push(nodes[ind].name.clone());
        }
        next_validators
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

    pub fn get_nodes(&self) -> Vec<&Node> {
        let mut nodes = vec![];
        for block in self.chain.iter().rev() {
            for node in block.data.iter() {
                // If node.name is not in nodes, add it
                if !nodes.iter().any(|n: &&Node| n.name == node.name) {
                    nodes.push(node);
                }
            }
        }
        nodes
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
            let next_validators = self.get_next_validators(&next_miner);
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
