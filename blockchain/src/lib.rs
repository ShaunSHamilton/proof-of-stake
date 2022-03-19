// Setup code should go here. Maybe import to export?
mod block;
mod mine;
mod node;

use block::Block;

use wasm_bindgen::prelude::*;

pub static DIFFICULTY_PREFIX: &str = "00";

// Mine a Block, return Chain
#[wasm_bindgen]
pub fn handle_mine() -> JsValue {
    // Get chain
    let chain: Vec<Block> = vec![];
    JsValue::from_serde(&chain).unwrap()
}

// Validate a Block, return `bool`
#[wasm_bindgen]
pub fn handle_validate() -> bool {
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
