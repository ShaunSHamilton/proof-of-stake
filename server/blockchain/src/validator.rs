use crate::node::Node;

/// A peer in the network
/// Lives for as long as the node is alive (connected to the network)
/// **Validator Responsibilities**
/// - Validate mined block
/// - Distribute valid block to peers
/// - Distribute valid block to clients
#[derive(Debug, Clone)]
pub struct Validator(Node);

impl Validator {
    pub fn get_validator_weight(&self) -> usize {
        calculate_validator_weight(&self.0)
    }
}

fn calculate_validator_weight(node: &Node) -> usize {
    let mut weight = 0;
    weight += node.staked;
    weight += node.tokens;
    weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_validator_weight() {
        let node = Node::new("Camper".to_string());
        let weight = calculate_validator_weight(&node);
        assert_eq!(weight, 0);
    }
}
