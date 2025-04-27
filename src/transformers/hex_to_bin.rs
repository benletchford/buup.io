use crate::{Transform, TransformError, TransformerCategory};
use std::fmt;

#[derive(Debug)]
pub enum HexToBinError {
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for HexToBinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexToBinError::ParseError(e) => write!(f, "Failed to parse hexadecimal: {}", e),
        }
    }
}

impl std::error::Error for HexToBinError {}

impl From<HexToBinError> for TransformError {
    fn from(err: HexToBinError) -> Self {
        TransformError::HexDecodeError(err.to_string())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct HexToBinTransformer;

impl Transform for HexToBinTransformer {
    fn id(&self) -> &'static str {
        "hex_to_bin"
    }

    fn name(&self) -> &'static str {
        "Hex to Binary"
    }

    fn description(&self) -> &'static str {
        "Convert hexadecimal numbers to binary."
    }

    fn category(&self) -> TransformerCategory {
        // Categorizing as Encoder as it primarily changes representation
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok("".to_string());
        }
        let hex_value = input.trim().trim_start_matches("0x");
        let decimal_value =
            u64::from_str_radix(hex_value, 16).map_err(HexToBinError::ParseError)?;
        let binary_string = format!("{:b}", decimal_value);
        Ok(binary_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        let transformer = HexToBinTransformer;
        assert_eq!(transformer.transform("FF").unwrap(), "11111111".to_string());
        assert_eq!(transformer.transform("0").unwrap(), "0".to_string());
        assert_eq!(transformer.transform("2A").unwrap(), "101010".to_string());
        assert_eq!(
            transformer.transform("0xFF").unwrap(),
            "11111111".to_string()
        );
        assert_eq!(transformer.transform("10").unwrap(), "10000".to_string());
    }

    #[test]
    fn test_hex_to_bin_invalid_input() {
        let transformer = HexToBinTransformer;
        assert!(transformer.transform("FG").is_err());
    }

    #[test]
    fn test_empty_input() {
        let transformer = HexToBinTransformer;
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
