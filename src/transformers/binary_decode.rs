use crate::{Transform, TransformError};

/// Binary Decode transformer
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BinaryDecode;

/// Default test input for Binary Decode
pub const DEFAULT_TEST_INPUT: &str = "01001000 01100101 01101100 01101100 01101111 00101100 00100000 01010111 01101111 01110010 01101100 01100100 00100001"; // "Hello, World!"

impl Transform for BinaryDecode {
    fn name(&self) -> &'static str {
        "Binary Decode"
    }

    fn id(&self) -> &'static str {
        "binarydecode"
    }

    fn description(&self) -> &'static str {
        "Decode space-separated binary representation back to text."
    }

    fn category(&self) -> crate::TransformerCategory {
        crate::TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let bytes: Result<Vec<u8>, _> = input
            .split_whitespace()
            .map(|s| {
                if s.len() != 8 || !s.chars().all(|c| c == '0' || c == '1') {
                    Err(TransformError::InvalidArgument(
                        format!("Invalid 8-bit binary chunk: '{}'", s).into(),
                    ))
                } else {
                    u8::from_str_radix(s, 2).map_err(|e| {
                        TransformError::InvalidArgument(
                            format!("Failed to parse binary chunk '{}': {}", s, e).into(),
                        )
                    })
                }
            })
            .collect();

        let bytes = bytes?;

        String::from_utf8(bytes).map_err(|e| {
            TransformError::InvalidArgument(format!("Invalid UTF-8 sequence: {}", e).into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_decode_empty() {
        let transformer = BinaryDecode;
        let result = transformer.transform("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_binary_decode_simple() {
        let transformer = BinaryDecode;
        let result = transformer.transform("01001000 01101001").unwrap(); // "Hi"
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_binary_decode_with_punctuation() {
        let transformer = BinaryDecode;
        let result = transformer.transform(DEFAULT_TEST_INPUT).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_binary_decode_unicode() {
        let transformer = BinaryDecode;
        let input = "11100010 10011100 10010011"; // '✓'
        let result = transformer.transform(input).unwrap();
        assert_eq!(result, "✓");
    }

    #[test]
    fn test_binary_decode_invalid_length() {
        let transformer = BinaryDecode;
        let result = transformer.transform("01001000 1101001"); // Second chunk is too short
        assert!(result.is_err());
        match result {
            Err(TransformError::InvalidArgument(msg)) => {
                assert!(msg.contains("Invalid 8-bit binary chunk: '1101001'"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_binary_decode_invalid_chars() {
        let transformer = BinaryDecode;
        let result = transformer.transform("01001000 0110100a"); // Contains 'a'
        assert!(result.is_err());
        match result {
            Err(TransformError::InvalidArgument(msg)) => {
                assert!(msg.contains("Invalid 8-bit binary chunk: '0110100a'"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_binary_decode_invalid_utf8() {
        let transformer = BinaryDecode;
        let result = transformer.transform("11110000 10011111 10011111"); // Invalid UTF-8 start byte
        assert!(result.is_err());
        match result {
            Err(TransformError::InvalidArgument(msg)) => {
                assert!(msg.contains("Invalid UTF-8 sequence"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_binary_decode_trailing_space() {
        let transformer = BinaryDecode;
        let result = transformer.transform("01001000 ").unwrap(); // "H"
        assert_eq!(result, "H");
    }

    #[test]
    fn test_binary_decode_leading_space() {
        let transformer = BinaryDecode;
        let result = transformer.transform(" 01001000").unwrap(); // "H"
        assert_eq!(result, "H");
    }
}
