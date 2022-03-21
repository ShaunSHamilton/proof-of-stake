// Setup code should go here. Maybe import to export?
mod block;
mod chain;
mod node;

use block::Block;
use chain::Chain;
use node::Node;

use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

pub static DIFFICULTY_PREFIX: &str = "00";

// Mine a Block, return Chain
#[wasm_bindgen]
pub fn handle_mine(chain: JsValue) -> JsValue {
    // Get chain
    let chain: Chain = chain.into_serde().unwrap();
    JsValue::from_serde(&chain).unwrap()
}

// Validate a Block, return `bool`
#[wasm_bindgen]
pub fn handle_validate(chain: JsValue, block: JsValue) -> bool {
    let chain: Chain = chain.into_serde().unwrap();
    let block: Block = block.into_serde().unwrap();
    let last_block: Block = chain.get_last_block();
    Node::validate_block(&last_block, &block)
    // true
}

#[wasm_bindgen]
pub fn initialise_chain() -> JsValue {
    let chain: Chain = Chain::new();
    JsValue::from_serde(&chain).unwrap()
}

pub fn hash_to_binary(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_hash(
    id: u64,
    timestamp: u64,
    previous_hash: &str,
    data: &Vec<Node>,
    nonce: u64,
    next_miner: &String,
    next_validators: &Vec<String>,
) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce,
        "next_miner": next_miner,
        "next_validators": next_validators,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::DIFFICULTY_PREFIX;
    #[test]
    fn difficulty_is_not_too_high() {
        assert!(DIFFICULTY_PREFIX.len() <= 3);
    }
}
