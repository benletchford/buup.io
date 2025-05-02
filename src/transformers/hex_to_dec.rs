use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum HexToDecError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for HexToDecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexToDecError::ParseError(e) => write!(f, "Failed to parse hexadecimal: {}", e),
        }
    }
}

impl std::error::Error for HexToDecError {}

impl From<HexToDecError> for TransformError {
    fn from(err: HexToDecError) -> Self {
        TransformError::HexDecodeError(err.to_string())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct HexToDecTransformer;

impl Transform for HexToDecTransformer {
    fn id(&self) -> &'static str {
        "hex_to_dec"
    }

    fn name(&self) -> &'static str {
        "Hex to Decimal"
    }

    fn description(&self) -> &'static str {
        "Converts hexadecimal numbers to their decimal representation."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn default_test_input(&self) -> &'static str {
        "FF"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok("".to_string());
        }
        let hex_value = input.trim().trim_start_matches("0x");
        let decimal_value =
            u64::from_str_radix(hex_value, 16).map_err(HexToDecError::ParseError)?;
        Ok(decimal_value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_dec() {
        let transformer = HexToDecTransformer;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "255"
        );
        assert_eq!(transformer.transform("0").unwrap(), "0");
        assert_eq!(transformer.transform("a").unwrap(), "10");
        assert_eq!(transformer.transform("1a").unwrap(), "26");
        assert_eq!(transformer.transform("100").unwrap(), "256");
    }

    #[test]
    fn test_hex_to_dec_invalid_input() {
        let transformer = HexToDecTransformer;
        assert!(transformer.transform("FG").is_err());
        assert!(transformer.transform("10.5").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = HexToDecTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
