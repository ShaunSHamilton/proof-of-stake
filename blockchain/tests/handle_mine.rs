// use crate::handle_mine;
// use crate::NodeState;
extern crate blockchain;
use blockchain::{handle_mine, NodeState};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn handle_mine_works() {
    let fix_node_state = "{chain: {chain: [{id: \"0\",hash:\"00110101\",previous_hash:\"\"\"\", timestamp: \"123456789\", data:[{name: \"Camper\", tokens: 10, staked: 0, racks: 1, reputation: 1}], nonce: 123, next_miner: \"Camper\", next_validators: [\"Camper\"]}, network: [\"Camper\"
    ]}, network: [\"Camper\"], transactions: [{name: \"Camper\", event: \"stake\"}], task_valid: true}";
    let node_state = JsValue::from(fix_node_state);
    let res = handle_mine(node_state);
    let ns: NodeState = match res {
        Ok(v) => v.into_serde().unwrap(),
        Err(_e) => panic!("Oh no"),
    };
    println!("{:?}", ns);
    assert!(true);
}
