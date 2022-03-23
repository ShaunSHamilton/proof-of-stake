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
// #[wasm_bindgen]
// pub fn handle_mine(chain: JsValue, data: JsValue) -> JsValue {
//     let mut chain: Chain = chain.into_serde().unwrap();
//     let data: Vec<Node> = data.into_serde().unwrap();
//     chain.mine_block(&data);
//     JsValue::from_serde(&chain).unwrap()
// }

// Do not mine from node. Use specific functions like `handle_stake`
#[wasm_bindgen]
pub fn handle_stake(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde().unwrap();
    let mut data: Vec<Node> = vec![];
    if let Some(node) = chain.get_node_by_name(&name) {
        if node.can_stake() {
            let mut node = node.clone();
            node.staked += 1;
            data.push(node);
        } else {
            return Err(JsError::new("Node cannot stake"));
        }
    } else {
        return Err(JsError::new("Node not found in chain"));
    }
    chain.mine_block(&data);
    Ok(JsValue::from_serde(&chain).unwrap())
}

// Validate a Block, return `bool`
#[wasm_bindgen]
pub fn handle_validate(chain: JsValue) -> bool {
    let chain: Chain = chain.into_serde().unwrap();
    let block: &Block = chain
        .chain
        .get(chain.chain.len() - 2)
        .expect("No previous block");
    let last_block: Block = chain.get_last_block();
    Node::validate_block(&last_block, block)
}

// Initialise a new blockchain, return Chain
#[wasm_bindgen]
pub fn initialise(peers: JsValue, name: String) -> JsValue {
    let mut chain: Chain = Chain::new();
    let mut peers: Vec<String> = peers.into_serde().unwrap();
    // Create genesis block
    let mut data = peers
        .iter_mut()
        .map(|peer| Node::new(peer))
        .collect::<Vec<Node>>();

    // Create a self
    let self_node = Node::new(&name);
    // Add self to data

    data.push(self_node);

    chain.mine_block(&data);

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
    data: &Vec<Node>,
    id: u64,
    next_miner: &String,
    next_validators: &Vec<String>,
    nonce: u64,
    previous_hash: &str,
    timestamp: u64,
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
