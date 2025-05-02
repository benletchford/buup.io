use crate::{Transform, TransformError, TransformerCategory};

/// Hex encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexEncode;

impl Transform for HexEncode {
    fn name(&self) -> &'static str {
        "Hex Encode"
    }

    fn id(&self) -> &'static str {
        "hexencode"
    }

    fn description(&self) -> &'static str {
        "Encode text to hexadecimal representation"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(hex_encode(input.as_bytes()))
    }

    fn default_test_input(&self) -> &'static str {
        "Hello, World!"
    }
}

/// Encodes bytes to hexadecimal without external dependencies
fn hex_encode(input: &[u8]) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut output = Vec::with_capacity(input.len() * 2);

    for &byte in input {
        output.push(HEX_CHARS[(byte >> 4) as usize]);
        output.push(HEX_CHARS[(byte & 0xf) as usize]);
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode() {
        let transformer = HexEncode;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "48656c6c6f2c20576f726c6421"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("a").unwrap(), "61");
        assert_eq!(transformer.transform("AB").unwrap(), "4142");
    }
}
