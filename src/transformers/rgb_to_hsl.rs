use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// RGB to HSL color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbToHsl;

impl Transform for RgbToHsl {
    fn name(&self) -> &'static str {
        "RGB to HSL"
    }

    fn id(&self) -> &'static str {
        "rgb_to_hsl"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts RGB color to HSL format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with("rgb(") {
            return Err(TransformError::InvalidArgument(
                "Invalid RGB format. Must start with rgb(".into(),
            ));
        }

        let color = Color::from_rgb(input)?;
        Ok(color.to_hsl())
    }

    fn default_test_input(&self) -> &'static str {
        "rgb(255, 0, 0)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsl() {
        let transformer = RgbToHsl;
        assert_eq!(
            transformer.transform("rgb(255, 0, 0)").unwrap(),
            "hsl(0deg,100%,50%)"
        );
        assert_eq!(
            transformer.transform("rgb(0, 255, 0)").unwrap(),
            "hsl(120deg,100%,50%)"
        );
        assert_eq!(
            transformer.transform("rgb(0, 0, 255)").unwrap(),
            "hsl(240deg,100%,50%)"
        );
        assert_eq!(
            transformer.transform("rgb(0, 0, 0)").unwrap(),
            "hsl(0deg,0%,0%)"
        );
        assert_eq!(
            transformer.transform("rgb(255, 255, 255)").unwrap(),
            "hsl(0deg,0%,100%)"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = RgbToHsl;
        assert_eq!(
            transformer.transform("rgb(255, 0, 0, 255)").unwrap(),
            "hsl(0deg,100%,50%,1.00)"
        );
        assert_eq!(
            transformer.transform("rgb(0, 255, 0, 128)").unwrap(),
            "hsl(120deg,100%,50%,0.50)"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = RgbToHsl;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("255, 0, 0").is_err()); // Missing rgb(
        assert!(transformer.transform("rgb(300, 0, 0)").is_err()); // Invalid value
    }
}
