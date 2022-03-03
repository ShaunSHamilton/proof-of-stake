use libp2p::PeerId;
use rand::{thread_rng, Rng};

use crate::p2p::PEER_ID;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub peer_id: PeerId,
    pub staked: usize,
    pub tokens: usize,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            peer_id: PEER_ID.clone(),
            staked: 0,
            tokens: 1,
        }
    }
}

// Implement a weighted random number generator
fn generate_random_number(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let num = rng.gen_range(0.0..1.0);
    num
}

// Get index of element in Vec based on weight and random number
fn get_index(weights: Vec<usize>, random_number: f64) -> usize {
    let mut cumulative_weight = 0.0;
    for (index, ele) in weights.iter().enumerate() {
        cumulative_weight += *ele as f64;
        if random_number < cumulative_weight {
            return index;
        }
    }
    return 0;
}

fn get_sorted_weights(weights: Vec<usize>) -> Vec<usize> {
    let mut sorted_weights = weights.clone();
    sorted_weights.sort();
    sorted_weights.reverse();
    sorted_weights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_number() {
        let min = 0.0;
        let max = 1.0;
        let random_number = generate_random_number(min, max);
        assert!(random_number >= min && random_number < max);
    }

    #[test]
    fn test_get_index() {
        let weights = vec![3, 2, 1];
        let random_number = generate_random_number(0.0, 6.0);
        let index = get_index(weights, random_number);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_sort_by_weight() {
        let weights = vec![1, 3, 2];
        let expected_weights = vec![3, 2, 1];

        let sorted_weights = get_sorted_weights(weights);
        assert_eq!(sorted_weights, expected_weights);
    }
}
