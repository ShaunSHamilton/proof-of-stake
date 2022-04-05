extern crate blockchain;

use blockchain::{chain::Chain, handle_mine, node::Node, Events, NodeState};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::ErrorEvent;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn handle_mine_stake() {
    let data = vec![Node::new("Camper")];
    let fix_node_state = fix(data);

    // console::log_1(&format!("{:?}", ns).into());
    let chain = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].staked, 1);
}

#[wasm_bindgen_test]
fn handle_mine_buy_rack() {
    let data = vec![Node::new("Camper")];
    let mut fix_node_state = fix(data);
    fix_node_state.transactions[0].event = Events::BuyRack;
    // console::log_1(&format!("{:?}", ns).into());
    let chain = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].racks, 1);
    assert_eq!(chain.get_last_block().unwrap().data[0].tokens, 10);
}

#[wasm_bindgen_test]
fn handle_mine_block_invalidated() {
    let data = vec![Node::new("Camper")];
    let mut fix_node_state = fix(data);
    fix_node_state.transactions[0].event = Events::BlockInvalidated;
    // console::log_1(&format!("{:?}", ns).into());
    let chain = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].racks, 0);
    assert_eq!(chain.get_last_block().unwrap().data[0].tokens, 19);
    assert_eq!(chain.get_last_block().unwrap().data[0].reputation, 0);
}

#[wasm_bindgen_test]
fn handle_mine_unstake() {
    let mut camper = Node::new("Camper");
    camper.staked = 1;
    let data = vec![camper];
    let mut fix_node_state = fix(data);
    fix_node_state.transactions[0].event = Events::Unstake;
    // console::log_1(&format!("{:?}", ns).into());
    let chain = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].staked, 0);
}

#[wasm_bindgen_test]
fn handle_mine_update_chain() {
    let mut camper = Node::new("Camper");
    camper.staked = 1;
    let data = vec![camper];
    let mut fix_node_state = fix(data);
    fix_node_state.transactions[0].event = Events::UpdateChain;
    fix_node_state.transactions[0].name = "Tom".to_string();
    // console::log_1(&format!("{:?}", ns).into());
    let chain = mine(fix_node_state).expect("result to be chain");
    assert!(chain.get_node_by_name("Tom").is_some());
    assert_eq!(chain.get_nodes().len(), 2);
}

#[wasm_bindgen_test]
fn handle_mine_cannot_buy_rack() {
    let mut camper = Node::new("Camper");
    camper.staked = 11;
    let data = vec![camper];
    let mut fix_node_state = fix(data);
    fix_node_state.transactions[0].event = Events::BuyRack;
    let chain_res = mine(fix_node_state);
    // console::log_1(&format!("{:?}", chain_res).into());
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(error_message.message(), "Node cannot buy rack");
    }
}

fn fix(data: Vec<Node>) -> NodeState {
    let node_vec_str = serde_json::to_string(&data).unwrap();
    let fix_node_state = format!(
        r#"{{
      "chain": {{
        "chain": [
          {{
            "id": 0,
            "hash": "00110101",
            "previous_hash": "",
            "timestamp": 123456789,
            "data": {},
            "nonce": 123,
            "next_miner": "Camper",
            "next_validators": ["Camper"]
          }}
        ],
        "network": ["Camper"]
      }},
      "network": ["Camper"],
      "transactions": [
        {{
          "event": "Stake",
          "name": "Camper"
        }}
      ],
      "task_valid": true
    }}"#,
        node_vec_str
    );
    serde_json::from_str(&fix_node_state).unwrap()
}

fn mine(fix_node_state: NodeState) -> Result<Chain, JsValue> {
    let node_state = JsValue::from_serde(&fix_node_state).unwrap();
    let res = handle_mine(node_state);
    let chain: Chain = match res {
        Ok(v) => {
            // console::log_1(&format!("{:?}", v).into());

            match v.into_serde() {
                Ok(v) => v,
                Err(_e) => {
                    // console::log_1(&format!("{:?}", e).into());
                    panic!("could not serde response");
                }
            }
        }
        Err(e) => {
            // console::log_1(&e.into());
            // Error is converted into a JsValue to make use of Debug trait
            return Err(JsValue::from(e));
        }
    };
    Ok(chain)
}
