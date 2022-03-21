extern crate blockchain;
use blockchain::handle_validate;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn handle_validate_returns_bool() {
    assert!(handle_validate());
}
