#[path = "../src/node.rs"]
mod node;
use node::get_next_miner;
#[test]
fn get_next_miner_returns_string() {
    let result = get_next_miner();
    assert_eq!(result, "Camper");
}
