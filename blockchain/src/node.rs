use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub name: String,
    pub staked: u64,
    pub tokens: u64,
    pub reputation: u64,
}

pub fn get_next_miner() -> String {
    // Search chain for:
    //   - previous block miner (previous miner cannot be next miner)
    unimplemented!();
}

pub fn get_next_validators() -> Vec<String> {
    unimplemented!();
}
