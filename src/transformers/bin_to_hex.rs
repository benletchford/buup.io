use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum BinToHexError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for BinToHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinToHexError::ParseError(e) => write!(f, "Failed to parse binary: {}", e),
        }
    }
}

impl std::error::Error for BinToHexError {}

impl From<BinToHexError> for TransformError {
    fn from(err: BinToHexError) -> Self {
        TransformError::HexDecodeError(err.to_string()) // Reusing HexDecodeError temporarily
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct BinToHexTransformer;

impl Transform for BinToHexTransformer {
    fn id(&self) -> &'static str {
        "bin_to_hex"
    }

    fn name(&self) -> &'static str {
        "Binary to Hex"
    }

    fn description(&self) -> &'static str {
        "Convert binary numbers to hexadecimal."
    }

    fn category(&self) -> TransformerCategory {
        // Categorizing as Encoder as it primarily changes representation
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok("".to_string());
        }
        let binary_value = input.trim();
        let decimal_value =
            u64::from_str_radix(binary_value, 2).map_err(BinToHexError::ParseError)?;
        let hex_string = format!("{:X}", decimal_value);
        Ok(hex_string)
    }

    fn default_test_input(&self) -> &'static str {
        "11111111" // Represents 255 (FF in hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_to_hex() {
        let transformer = BinToHexTransformer;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "FF".to_string()
        );
        assert_eq!(transformer.transform("0").unwrap(), "0".to_string());
        assert_eq!(transformer.transform("101010").unwrap(), "2A".to_string());
        assert_eq!(transformer.transform("10000").unwrap(), "10".to_string());
    }

    #[test]
    fn test_bin_to_hex_invalid_input() {
        let transformer = BinToHexTransformer;
        assert!(transformer.transform("102").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = BinToHexTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
