use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum DecToHexError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for DecToHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecToHexError::ParseError(e) => write!(f, "Failed to parse decimal: {}", e),
        }
    }
}

impl std::error::Error for DecToHexError {}

impl From<DecToHexError> for TransformError {
    fn from(err: DecToHexError) -> Self {
        // For simplicity, mapping specific parse error to a generic HexDecodeError variant
        // We might want a more specific error variant in TransformError later.
        TransformError::HexDecodeError(err.to_string())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct DecToHexTransformer;

impl Transform for DecToHexTransformer {
    fn id(&self) -> &'static str {
        "dec_to_hex"
    }

    fn name(&self) -> &'static str {
        "Decimal to Hex"
    }

    fn description(&self) -> &'static str {
        "Convert decimal numbers to hexadecimal."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok("".to_string());
        }
        let decimal_value = input
            .trim()
            .parse::<u64>()
            .map_err(DecToHexError::ParseError)?;
        let hex_string = format!("{:X}", decimal_value);
        Ok(hex_string)
    }

    fn default_test_input(&self) -> &'static str {
        "255" // Represents FF in hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_to_hex() {
        let transformer = DecToHexTransformer;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "FF".to_string()
        );
        assert_eq!(transformer.transform("0").unwrap(), "0".to_string());
        assert_eq!(transformer.transform("42").unwrap(), "2A".to_string());
        assert_eq!(transformer.transform("65535").unwrap(), "FFFF".to_string());
    }

    #[test]
    fn test_dec_to_hex_invalid_input() {
        let transformer = DecToHexTransformer;
        assert!(transformer.transform("abc").is_err());
        assert!(transformer.transform("10.5").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = DecToHexTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
