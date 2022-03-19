#[path = "../src/node.rs"]
mod node;
use node::Node;
#[test]
fn get_next_miner_returns_string() {
    let result = Node::get_next_miner();
    assert_eq!(result, "Camper");
}
