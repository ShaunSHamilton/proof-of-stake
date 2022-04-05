mod block;
pub mod chain;
pub mod node;

use block::Block;
use chain::Chain;
use node::Node;

// use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
// use web_sys::console;

pub static DIFFICULTY_PREFIX: &str = "0";

#[derive(Serialize, Deserialize, Debug)]
pub enum Events {
    BuyRack,
    BlockInvalidated,
    Stake,
    Unstake,
    UpdateChain,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub event: Events,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeState {
    pub chain: Chain,
    pub network: Vec<String>,
    pub transactions: Vec<Transaction>,
    pub task_valid: bool,
}

#[wasm_bindgen]
pub fn handle_mine(node_state: JsValue) -> Result<JsValue, JsError> {
    let node_state: NodeState = node_state.into_serde()?;
    let mut chain = node_state.chain;
    chain.network = node_state.network;
    let mut data: Vec<Node> = vec![];
    for transaction in node_state.transactions {
        if let Some(mut node) = chain.get_node_by_name(&transaction.name) {
            match transaction.event {
                Events::Unstake => {
                    if node.can_unstake() {
                        // let mut node = node.clone();
                        node.staked -= 1;
                        data.push(node);
                    } else {
                        return Err(JsError::new("Node cannot unstake"));
                    }
                }
                Events::Stake => {
                    if node.can_stake() {
                        node.staked += 1;
                        data.push(node);
                    } else {
                        return Err(JsError::new("Node cannot stake"));
                    }
                }
                Events::BuyRack => {
                    if node.can_buy_rack() {
                        node.racks += 1;
                        node.tokens -= 10;
                        data.push(node);
                    } else {
                        return Err(JsError::new("Node cannot buy rack"));
                    }
                }
                Events::BlockInvalidated => {
                    if node.can_punish() {
                        if node.tokens == node.staked {
                            node.staked -= 1;
                        }
                        node.tokens -= 1;
                        node.reputation -= 1;
                        data.push(node);
                    } else {
                        return Err(JsError::new("Node cannot be punished"));
                    }
                }
                _ => {
                    return Err(JsError::new("Invalid event"));
                }
            };
        } else {
            match transaction.event {
                Events::UpdateChain => {
                    // Add node to chain
                    data.push(Node::new(&transaction.name));
                }
                _ => {
                    return Err(JsError::new("Node not found in chain"));
                }
            };
        }
    }

    chain.mine_block(&data);
    Ok(JsValue::from_serde(&chain)?)
}

#[wasm_bindgen]
pub fn handle_validate(chain: JsValue) -> Result<bool, JsError> {
    let chain: Chain = chain.into_serde()?;
    if let Some(previous_block) = chain.chain.get(chain.chain.len() - 2) {
        let last_block: Block = match chain.get_last_block() {
            Some(block) => block,
            None => return Err(JsError::new("Chain is empty")),
        };
        Ok(Node::validate_block(&last_block, previous_block))
    } else {
        Err(JsError::new("Chain is too short"))
    }
}

// Initialise a new blockchain, return Chain
#[wasm_bindgen]
pub fn initialise(name: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = Chain::new();
    println!("{:?}", chain);
    // Create genesis block
    let data = vec![Node::new(&name)];
    println!("{:?}", data);

    chain.mine_block(&data);
    println!("{:?}", chain);

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
        assert_eq!(hash.len(), 32);
    }
    #[test]
    fn hash_to_binary_works() {
        let hash = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let hash_str = hash_to_binary(&hash);
        assert_eq!(hash_str.len(), 50);
    }
}
