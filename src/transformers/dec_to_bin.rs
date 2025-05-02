use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum DecToBinError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for DecToBinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecToBinError::ParseError(e) => write!(f, "Failed to parse decimal: {}", e),
        }
    }
}

impl std::error::Error for DecToBinError {}

impl From<DecToBinError> for TransformError {
    fn from(err: DecToBinError) -> Self {
        // Using a generic error type for now
        TransformError::HexDecodeError(err.to_string()) // Reusing HexDecodeError temporarily
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct DecToBinTransformer;

impl Transform for DecToBinTransformer {
    fn id(&self) -> &'static str {
        "dec_to_bin"
    }

    fn name(&self) -> &'static str {
        "Decimal to Binary"
    }

    fn description(&self) -> &'static str {
        "Convert decimal numbers to binary."
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
            .map_err(DecToBinError::ParseError)?;
        let binary_string = format!("{:b}", decimal_value);
        Ok(binary_string)
    }

    fn default_test_input(&self) -> &'static str {
        "42" // Represents 101010 in binary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_to_bin() {
        let transformer = DecToBinTransformer;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "101010".to_string()
        );
        assert_eq!(transformer.transform("10").unwrap(), "1010".to_string());
        assert_eq!(transformer.transform("0").unwrap(), "0".to_string());
        assert_eq!(transformer.transform("7").unwrap(), "111".to_string());
        assert_eq!(
            transformer.transform("255").unwrap(),
            "11111111".to_string()
        );
    }

    #[test]
    fn test_dec_to_bin_invalid_input() {
        let transformer = DecToBinTransformer;
        assert!(transformer.transform("abc").is_err());
        assert!(transformer.transform("10.5").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = DecToBinTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
