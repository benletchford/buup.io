use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// RGB to Hex color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbToHex;

impl Transform for RgbToHex {
    fn name(&self) -> &'static str {
        "RGB to Hex"
    }

    fn id(&self) -> &'static str {
        "rgb_to_hex"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts RGB color to hex format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with("rgb(") {
            return Err(TransformError::InvalidArgument(
                "Invalid RGB format. Must start with rgb(".into(),
            ));
        }

        let color = Color::from_rgb(input)?;
        Ok(color.to_hex())
    }

    fn default_test_input(&self) -> &'static str {
        "rgb(255, 0, 0)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hex() {
        let transformer = RgbToHex;
        assert_eq!(transformer.transform("rgb(255, 0, 0)").unwrap(), "#ff0000");
        assert_eq!(transformer.transform("rgb(0, 255, 0)").unwrap(), "#00ff00");
        assert_eq!(transformer.transform("rgb(0, 0, 255)").unwrap(), "#0000ff");
        assert_eq!(transformer.transform("rgb(0, 0, 0)").unwrap(), "#000000");
        assert_eq!(
            transformer.transform("rgb(255, 255, 255)").unwrap(),
            "#ffffff"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = RgbToHex;
        assert_eq!(
            transformer.transform("rgb(255, 0, 0, 255)").unwrap(),
            "#ff0000ff"
        );
        assert_eq!(
            transformer.transform("rgb(0, 255, 0, 128)").unwrap(),
            "#00ff0080"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = RgbToHex;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("255, 0, 0").is_err()); // Missing rgb(
        assert!(transformer.transform("rgb(300, 0, 0)").is_err()); // Invalid value
    }
}
