use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// Hex to RGB color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexToRgb;

impl Transform for HexToRgb {
    fn name(&self) -> &'static str {
        "Hex to RGB"
    }

    fn id(&self) -> &'static str {
        "hex_to_rgb"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts hex color code to RGB format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with('#') {
            return Err(TransformError::InvalidArgument(
                "Invalid hex color format. Must start with #".into(),
            ));
        }

        let color = Color::from_hex(input)?;
        Ok(color.to_rgb())
    }

    fn default_test_input(&self) -> &'static str {
        "#FF0000"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        let transformer = HexToRgb;
        assert_eq!(transformer.transform("#FF0000").unwrap(), "rgb(255,0,0)");
        assert_eq!(transformer.transform("#00FF00").unwrap(), "rgb(0,255,0)");
        assert_eq!(transformer.transform("#0000FF").unwrap(), "rgb(0,0,255)");
        assert_eq!(transformer.transform("#000000").unwrap(), "rgb(0,0,0)");
        assert_eq!(
            transformer.transform("#FFFFFF").unwrap(),
            "rgb(255,255,255)"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = HexToRgb;
        assert_eq!(
            transformer.transform("#FF0000FF").unwrap(),
            "rgb(255,0,0,255)"
        );
        assert_eq!(
            transformer.transform("#00FF0080").unwrap(),
            "rgb(0,255,0,128)"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = HexToRgb;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("FF0000").is_err()); // Missing #
        assert!(transformer.transform("#GG0000").is_err()); // Invalid hex
    }
}
