use crate::types::PseudoBigInt;
use crate::types::PseudoBigIntJs;
use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = isPinnedComment)]
pub fn is_pinned_comment(text: &str, start: usize) -> bool {
    text.chars().nth(start + 1) == Some('*') && text.chars().nth(start + 2) == Some('!')
}
