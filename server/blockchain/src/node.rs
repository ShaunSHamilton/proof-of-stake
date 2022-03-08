#![allow(dead_code)]
// use libp2p::PeerId;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

// use crate::p2p::PEER_ID;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    // pub peer_id: PeerId,
    pub reputation: usize,
    pub staked: usize,
    pub tokens: usize,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            // peer_id: PEER_ID.clone(),
            reputation: 1,
            staked: 0,
            tokens: 1, // Start every Node with 1 token,
        }
    }
}

// Implement a weighted random number generator
pub fn generate_random_number(max: usize) -> usize {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..max);
    num
}

// Get index of element in Vec based on weight and random number
pub fn get_index(weights: Vec<usize>, random_number: usize) -> usize {
    let mut cumulative_weight = 0usize;
    for (index, ele) in weights.iter().enumerate() {
        cumulative_weight += ele;
        if random_number < cumulative_weight {
            return index;
        }
    }
    return 0;
}

pub fn get_sorted_weights(weights: Vec<usize>) -> Vec<usize> {
    let mut sorted_weights = weights.clone();
    sorted_weights.sort();
    sorted_weights.reverse();
    sorted_weights
}
