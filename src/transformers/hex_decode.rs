use crate::{Transform, TransformError, TransformerCategory};

/// Hex decode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexDecode;

impl Transform for HexDecode {
    fn name(&self) -> &'static str {
        "Hex Decode"
    }

    fn id(&self) -> &'static str {
        "hexdecode"
    }

    fn description(&self) -> &'static str {
        "Decodes a hexadecimal string into its original bytes, then interprets as UTF-8."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn default_test_input(&self) -> &'static str {
        "48656c6c6f2c20576f726c6421"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Ignore whitespace in the input
        let input = input.replace(' ', "");

        // Validate input is valid hex
        if input.is_empty() {
            return Ok(String::new());
        }

        if input.len() % 2 != 0 {
            return Err(TransformError::HexDecodeError(
                "Hex string must have an even length".to_string(),
            ));
        }

        let bytes = hex_decode(&input)?;

        String::from_utf8(bytes).map_err(|_| TransformError::Utf8Error)
    }
}

/// Decodes a hexadecimal string without external dependencies
fn hex_decode(input: &str) -> Result<Vec<u8>, TransformError> {
    let input = input.as_bytes();
    let mut output = Vec::with_capacity(input.len() / 2);

    for chunk in input.chunks(2) {
        if chunk.len() != 2 {
            return Err(TransformError::HexDecodeError(
                "Incomplete hex byte".to_string(),
            ));
        }

        let high = decode_hex_digit(chunk[0])?;
        let low = decode_hex_digit(chunk[1])?;

        output.push((high << 4) | low);
    }

    Ok(output)
}

/// Decodes a single hex digit
fn decode_hex_digit(digit: u8) -> Result<u8, TransformError> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(TransformError::HexDecodeError(format!(
            "Invalid hex digit: {}",
            char::from(digit)
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_decode() {
        let transformer = HexDecode;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "Hello, World!"
        );
        assert_eq!(transformer.transform("68656c6c6f").unwrap(), "hello");
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("E29C93").unwrap(), "✓");
    }

    #[test]
    fn test_invalid_hex() {
        let transformer = HexDecode;
        assert!(transformer.transform("4").is_err()); // Odd length
        assert!(transformer.transform("xy").is_err()); // Invalid characters
    }
}
