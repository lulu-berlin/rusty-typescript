use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::parse_pseudo_big_int;
    use num_bigint::BigUint;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn strip_base10_strings(
            test_number in "[1-9][0-9]*",
            prefix in "0*"
        ) {

            let big_int = prefix + &test_number + "n";
            let result = parse_pseudo_big_int(&big_int);
            assert_eq!(result, test_number);
        }
    }

    #[test]
    fn parse_simple_binary() {
        assert_eq!(parse_pseudo_big_int("0b1n"), "1");
    }

    proptest! {
        #[test]
        fn parse_binary_literals(
            test_number in "1[0-1]*",
            leading_zeros in "0*",
            prefix in "0(b|B)"
        ) {
            let big_int = prefix + &leading_zeros + &test_number + "n";
            let result = parse_pseudo_big_int(&big_int);
            let expected = BigUint::parse_bytes(test_number.as_bytes(), 2).unwrap().to_string();
            assert_eq!(result, expected);
        }
    }

    proptest! {
        #[test]
        fn parse_octal_literals(
            test_number in "[1-7][0-7]*",
            leading_zeros in "0*",
            prefix in "0(o|O)"
        ) {
            let big_int = prefix + &leading_zeros + &test_number + "n";
            let result = parse_pseudo_big_int(&big_int);
            let expected = BigUint::parse_bytes(test_number.as_bytes(), 8).unwrap().to_string();
            assert_eq!(result, expected);
        }
    }

    proptest! {
        #[test]
        fn parse_hex_literals(
            test_number in "[1-9ABCDEF][0-9ABCDEF]*",
            leading_zeros in "0*",
            prefix in "0(x|X)"
        ) {
            let big_int = prefix + &leading_zeros + &test_number + "n";
            let result = parse_pseudo_big_int(&big_int);
            let expected = BigUint::parse_bytes(test_number.as_bytes(), 16).unwrap().to_string();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_large_literals() {
        assert_eq!(
            parse_pseudo_big_int("123456789012345678901234567890n"),
            "123456789012345678901234567890"
        );
        assert_eq!(
            parse_pseudo_big_int("0b1100011101110100100001111111101101100001101110011111000001110111001001110001111110000101011010010n"),
            "123456789012345678901234567890"
        );
        assert_eq!(
            parse_pseudo_big_int("0o143564417755415637016711617605322n"),
            "123456789012345678901234567890"
        );
        assert_eq!(
            parse_pseudo_big_int("0x18ee90ff6c373e0ee4e3f0ad2n"),
            "123456789012345678901234567890"
        );
    }
}
