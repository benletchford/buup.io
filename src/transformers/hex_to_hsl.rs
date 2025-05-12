use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// Hex to HSL color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToHsl;

impl Transform for HexToHsl {
    fn name(&self) -> &'static str {
        "Hex to HSL"
    }

    fn id(&self) -> &'static str {
        "hex_to_hsl"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts hex color code to HSL format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with('#') {
            return Err(TransformError::InvalidArgument(
                "Invalid hex color format. Must start with #".into(),
            ));
        }

        let color = Color::from_hex(input)?;
        Ok(color.to_hsl())
    }

    fn default_test_input(&self) -> &'static str {
        "#FF0000"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_hsl() {
        let transformer = HexToHsl;
        assert_eq!(
            transformer.transform("#FF0000").unwrap(),
            "hsl(0deg,100%,50%)"
        );
        assert_eq!(
            transformer.transform("#00FF00").unwrap(),
            "hsl(120deg,100%,50%)"
        );
        assert_eq!(
            transformer.transform("#0000FF").unwrap(),
            "hsl(240deg,100%,50%)"
        );
        assert_eq!(transformer.transform("#000000").unwrap(), "hsl(0deg,0%,0%)");
        assert_eq!(
            transformer.transform("#FFFFFF").unwrap(),
            "hsl(0deg,0%,100%)"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = HexToHsl;
        assert_eq!(
            transformer.transform("#FF0000FF").unwrap(),
            "hsl(0deg,100%,50%,1.00)"
        );
        assert_eq!(
            transformer.transform("#00FF0080").unwrap(),
            "hsl(120deg,100%,50%,0.50)"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = HexToHsl;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("FF0000").is_err()); // Missing #
        assert!(transformer.transform("#GG0000").is_err()); // Invalid hex
    }
}
