use serde::{Deserialize, Serialize};

use crate::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chain {
    pub chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self { chain: vec![] }
    }
    pub fn get_last_block(&self) -> Block {
        self.chain.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_chain_returns_empty_vec() {
        let chain: Chain = Chain::new();
        assert_eq!(chain.chain.len(), 0);
    }
}
