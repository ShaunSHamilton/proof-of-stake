// Setup code should go here. Maybe import to export?
mod block;
mod chain;
mod node;

use block::Block;
use chain::Chain;
use node::Node;

use rand::{self, Rng};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

pub static DIFFICULTY_PREFIX: &str = "00";

#[wasm_bindgen]
pub fn handle_buy_rack(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
    let mut data = vec![];
    if let Some(node) = chain.get_node_by_name(&name) {
        if node.can_buy_rack() {
            let mut node = node.clone();
            node.tokens -= 10;
            data.push(node);
        } else {
            return Err(JsError::new(
                "Node does not have enough tokens to buy a rack",
            ));
        }
    } else {
        return Err(JsError::new("Node not found"));
    }
    chain.mine_block(&data);
    return Ok(JsValue::from_serde(&chain)?);
}

#[wasm_bindgen]
pub fn handle_connection(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
    let data = vec![Node::new(&name)];
    chain.mine_block(&data);
    return Ok(JsValue::from_serde(&chain)?);
}

#[wasm_bindgen]
pub fn handle_get_node_by_name(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let chain: Chain = chain.into_serde()?;
    if let Some(node) = chain.get_node_by_name(&name) {
        Ok(JsValue::from_serde(&node)?)
    } else {
        Err(JsError::new("Node not found in chain"))
    }
}

#[wasm_bindgen]
pub fn handle_get_nodes(chain: JsValue) -> Result<JsValue, JsError> {
    let chain: Chain = chain.into_serde()?;
    let nodes = chain.get_nodes();
    Ok(JsValue::from_serde(&nodes)?)
}

#[wasm_bindgen]
pub fn handle_punish(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
    let mut data: Vec<Node> = vec![];
    if let Some(node) = chain.get_node_by_name(&name) {
        let mut node = node.clone();
        if node.tokens > 0 {
            if node.staked == node.tokens {
                node.staked -= 1;
            }
            node.tokens -= 1;
            // Always reduce reputation
        } else {
            return Err(JsError::new("Node out of tokens"));
        }
        if node.reputation == 0 {
            return Err(JsError::new("Node out of reputation"));
        }
        node.reputation -= 1;
        data.push(node);
    } else {
        return Err(JsError::new("Node not found in chain"));
    }
    chain.mine_block(&data);
    Ok(JsValue::from_serde(&chain)?)
}

#[wasm_bindgen]
pub fn handle_reward(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
    let mut data: Vec<Node> = vec![];
    if let Some(node) = chain.get_node_by_name(&name) {
        let mut node = node.clone();
        node.tokens += 1;
        // Randomly increase reputation
        if rand::thread_rng().gen::<f32>() > 0.80 {
            node.reputation += 1;
        }
        data.push(node);
    } else {
        return Err(JsError::new("Node not found in chain"));
    }
    chain.mine_block(&data);
    Ok(JsValue::from_serde(&chain)?)
}

#[wasm_bindgen]
pub fn handle_stake(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
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
    Ok(JsValue::from_serde(&chain)?)
}

#[wasm_bindgen]
pub fn handle_unstake(chain: JsValue, name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = chain.into_serde()?;
    let mut data: Vec<Node> = vec![];
    if let Some(node) = chain.get_node_by_name(&name) {
        if node.can_unstake() {
            let mut node = node.clone();
            node.staked -= 1;
            data.push(node);
        } else {
            return Err(JsError::new("Node cannot unstake"));
        }
    } else {
        return Err(JsError::new("Node not found in chain"));
    }
    chain.mine_block(&data);
    Ok(JsValue::from_serde(&chain)?)
}

#[wasm_bindgen]
pub fn handle_validate(chain: JsValue) -> Result<bool, JsError> {
    let chain: Chain = chain.into_serde()?;
    if let Some(previous_block) = chain.chain.get(chain.chain.len() - 2) {
        let last_block: Block = chain.get_last_block();
        Ok(Node::validate_block(&last_block, previous_block))
    } else {
        Err(JsError::new("Chain is too short"))
    }
}

// Initialise a new blockchain, return Chain
#[wasm_bindgen]
pub fn initialise(name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = Chain::new();
    // Create genesis block
    let data = vec![Node::new(&name)];

    chain.mine_block(&data);

    Ok(JsValue::from_serde(&chain)?)
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
    use super::*;
    #[test]
    fn difficulty_is_not_too_high() {
        assert!(DIFFICULTY_PREFIX.len() <= 3);
    }
    #[test]
    fn calculate_hash_works() {
        let data = vec![Node::new("test")];
        let hash = calculate_hash(
            &data,
            1,
            &"test".to_string(),
            &vec!["test".to_string()],
            1,
            &"test".to_string(),
            1,
        );
        assert_eq!(hash.len(), 64);
    }
    #[test]
    fn hash_to_binary_works() {
        let hash = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let hash_str = hash_to_binary(&hash);
        assert_eq!(hash_str.len(), 256);
    }
}
