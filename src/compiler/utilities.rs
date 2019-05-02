use crate::types::PseudoBigInt;
use crate::types::PseudoBigIntJs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "pseudoBigIntToString")]
pub fn pseudo_big_int_to_string(pseudo_big_int_js: &PseudoBigIntJs) -> String {
    let pseudo_big_int: PseudoBigInt = pseudo_big_int_js.into();
    pseudo_big_int.to_string()
}
