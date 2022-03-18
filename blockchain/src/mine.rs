mod lib;
mod node;
use lib::DIFFICULTY_PREFIX;
use node::{get_next_miner, get_next_validators, Node};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mine_block(
    id: u64,
    timestamp: u64,
    previous_hash: &str,
    data: &Vec<Node>,
) -> (u64, String, String, Vec<String>) {
    println!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100_000 == 0 {
            println!("nonce: {}", nonce);
        }
        let hash = calc_hash(id, timestamp, previous_hash, data, nonce);
        let bin_hash = hash_to_bin_rep(&hash);
        if bin_hash.starts_with(DIFFICULTY_PREFIX) {
            println!(
                "mined! nonce: {}, hash: {}, bin hash: {}",
                nonce,
                hex::encode(&hash),
                bin_hash
            );
            // TODO: Not hardcode
            let next_miner = get_next_miner();
            let next_validators = get_next_validators();
            return (nonce, hex::encode(hash), next_miner, next_validators);
        }
        nonce += 1;
    }
}

pub fn calc_hash(
    id: u64,
    timestamp: u64,
    previous_hash: &str,
    data: &Vec<Node>,
    nonce: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
      "id": id,
      "previous_hash": previous_hash,
      "data": data,
      "timestamp": timestamp,
      "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn hash_to_bin_rep(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
