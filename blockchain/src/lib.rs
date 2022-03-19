// Setup code should go here. Maybe import to export?
mod block;
mod chain;
mod mine;
mod node;

use chain::Chain;

use wasm_bindgen::prelude::*;

pub static DIFFICULTY_PREFIX: &str = "00";

// Mine a Block, return Chain
#[wasm_bindgen]
pub fn handle_mine() -> JsValue {
    // Get chain
    let chain: Chain = Chain::new();
    JsValue::from_serde(&chain).unwrap()
}

// Validate a Block, return `bool`
#[wasm_bindgen]
pub fn handle_validate() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::DIFFICULTY_PREFIX;
    #[test]
    fn difficulty_is_not_too_high() {
        assert!(DIFFICULTY_PREFIX.len() <= 3);
    }
}
