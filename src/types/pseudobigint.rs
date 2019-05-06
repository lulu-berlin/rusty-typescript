
#[wasm_bindgen]
extern "C" {
    pub type PseudoBigIntJs;
    #[wasm_bindgen(method, getter = negative)]
    pub fn get_negative(this: &PseudoBigIntJs) -> bool;
    #[wasm_bindgen(method, getter = base10Value)]
    pub fn get_base10_value(this: &PseudoBigIntJs) -> String;
}

#[derive(Clone)]
pub(crate) struct PseudoBigInt {
    negative: bool,
    base10_value: String,
}

impl From<&PseudoBigIntJs> for PseudoBigInt {
    fn from(pseudo_big_int_js: &PseudoBigIntJs) -> PseudoBigInt {
        PseudoBigInt {
            negative: pseudo_big_int_js.get_negative(),
            base10_value: pseudo_big_int_js.get_base10_value(),
        }
    }
}

impl std::fmt::Display for PseudoBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let prefix = if self.negative && self.base10_value != "0" {
            "-"
        } else {
            ""
        };
        write!(f, "{}", prefix)?;
        write!(f, "{}", self.base10_value)
    }
}

#[wasm_bindgen(js_name = pseudoBigIntToString)]
pub fn pseudo_big_int_to_string(pseudo_big_int_js: &PseudoBigIntJs) -> String {
    let pseudo_big_int: PseudoBigInt = pseudo_big_int_js.into();
    pseudo_big_int.to_string()
}

/// Converts a bigint literal string, e.g. `0x1234n`, to its decimal string
/// representation, e.g. `4660`.
#[wasm_bindgen(js_name = parsePseudoBigInt)]
pub fn parse_pseudo_big_int(string_value: &str) -> String {
    let (start, radix) = match string_value
        .chars()
        .nth(1)
        .map(|ch| ch.to_ascii_lowercase())
    {
        Some('b') => (2, 2),
        Some('o') => (2, 8),
        Some('x') => (2, 16),
        _ => (0, 10),
    };
    let length = string_value.len();
    let digits: &[u8] = string_value.get(start..(length - 1)).unwrap().as_bytes();
    BigUint::parse_bytes(digits, radix).unwrap().to_string()
}
