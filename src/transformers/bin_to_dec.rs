use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum BinToDecError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for BinToDecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinToDecError::ParseError(e) => write!(f, "Failed to parse binary: {}", e),
        }
    }
}

impl std::error::Error for BinToDecError {}

impl From<BinToDecError> for TransformError {
    fn from(err: BinToDecError) -> Self {
        TransformError::HexDecodeError(err.to_string()) // Reusing HexDecodeError temporarily
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct BinToDecTransformer;

impl Transform for BinToDecTransformer {
    fn id(&self) -> &'static str {
        "bin_to_dec"
    }

    fn name(&self) -> &'static str {
        "Binary to Decimal"
    }

    fn description(&self) -> &'static str {
        "Convert binary numbers to decimal."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok("".to_string());
        }
        let binary_value = input.trim();
        let decimal_value =
            u64::from_str_radix(binary_value, 2).map_err(BinToDecError::ParseError)?;
        Ok(decimal_value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_to_dec() {
        let transformer = BinToDecTransformer;
        assert_eq!(transformer.transform("1010").unwrap(), "10".to_string());
        assert_eq!(transformer.transform("0").unwrap(), "0".to_string());
        assert_eq!(transformer.transform("111").unwrap(), "7".to_string());
        assert_eq!(
            transformer.transform("11111111").unwrap(),
            "255".to_string()
        );
    }

    #[test]
    fn test_bin_to_dec_invalid_input() {
        let transformer = BinToDecTransformer;
        assert!(transformer.transform("102").is_err());
        assert!(transformer.transform("abc").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = BinToDecTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
