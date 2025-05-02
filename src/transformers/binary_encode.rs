use crate::{Transform, TransformError};

/// Binary Encode transformer
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BinaryEncode;

/// Default test input for Binary Encode
pub const DEFAULT_TEST_INPUT: &str = "Hello, World!";

impl Transform for BinaryEncode {
    fn name(&self) -> &'static str {
        "Binary Encode"
    }

    fn id(&self) -> &'static str {
        "binaryencode"
    }

    fn description(&self) -> &'static str {
        "Encode text into its binary representation (space-separated bytes)."
    }

    fn category(&self) -> crate::TransformerCategory {
        crate::TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let binary_chunks: Vec<String> =
            input.bytes().map(|byte| format!("{:08b}", byte)).collect();

        Ok(binary_chunks.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_encode_empty() {
        let transformer = BinaryEncode;
        let result = transformer.transform("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_binary_encode_simple() {
        let transformer = BinaryEncode;
        let result = transformer.transform("Hi").unwrap();
        // H = 72 = 01001000, i = 105 = 01101001
        assert_eq!(result, "01001000 01101001");
    }

    #[test]
    fn test_binary_encode_with_punctuation() {
        let transformer = BinaryEncode;
        let result = transformer.transform(DEFAULT_TEST_INPUT).unwrap();
        // H=72, e=101, l=108, l=108, o=111, ,=44,  =32, W=87, o=111, r=114, l=108, d=100, !=33
        let expected = "01001000 01100101 01101100 01101100 01101111 00101100 00100000 01010111 01101111 01110010 01101100 01100100 00100001";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_binary_encode_unicode() {
        let transformer = BinaryEncode;
        // Example: Unicode character '✓' (U+2713)
        // UTF-8 representation: E2 9C 93
        // Binary: 11100010 10011100 10010011
        let result = transformer.transform("✓").unwrap();
        assert_eq!(result, "11100010 10011100 10010011");
    }
}
