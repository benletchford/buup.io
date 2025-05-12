use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// HSL to RGB color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HslToRgb;

impl Transform for HslToRgb {
    fn name(&self) -> &'static str {
        "HSL to RGB"
    }

    fn id(&self) -> &'static str {
        "hsl_to_rgb"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts HSL color to RGB format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with("hsl(") {
            return Err(TransformError::InvalidArgument(
                "Invalid HSL format. Must start with hsl(".into(),
            ));
        }

        let color = Color::from_hsl(input)?;
        Ok(color.to_rgb())
    }

    fn default_test_input(&self) -> &'static str {
        "hsl(0deg, 100%, 50%)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_to_rgb() {
        let transformer = HslToRgb;
        assert_eq!(
            transformer.transform("hsl(0deg, 100%, 50%)").unwrap(),
            "rgb(255,0,0)"
        );
        assert_eq!(
            transformer.transform("hsl(120deg, 100%, 50%)").unwrap(),
            "rgb(0,255,0)"
        );
        assert_eq!(
            transformer.transform("hsl(240deg, 100%, 50%)").unwrap(),
            "rgb(0,0,255)"
        );
        assert_eq!(
            transformer.transform("hsl(0deg, 0%, 0%)").unwrap(),
            "rgb(0,0,0)"
        );
        assert_eq!(
            transformer.transform("hsl(0deg, 0%, 100%)").unwrap(),
            "rgb(255,255,255)"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = HslToRgb;
        assert_eq!(
            transformer.transform("hsl(0deg, 100%, 50%, 1.0)").unwrap(),
            "rgb(255,0,0,255)"
        );
        assert_eq!(
            transformer
                .transform("hsl(120deg, 100%, 50%, 0.5)")
                .unwrap(),
            "rgb(0,255,0,127)"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = HslToRgb;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("0deg, 100%, 50%").is_err()); // Missing hsl(
                                                                    // Note: HSL implementation accepts values outside the normal range
        assert!(transformer.transform("hsl(400, 100%, 50%)").is_ok()); // This is actually valid in the color implementation
    }
}
