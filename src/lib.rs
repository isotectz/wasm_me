use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_me(i_val: i32) -> i32 {
    i_val + i_val
}
