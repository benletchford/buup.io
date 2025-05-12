use crate::utils::Color;
use crate::{Transform, TransformError, TransformerCategory};

/// HSL to Hex color transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HslToHex;

impl Transform for HslToHex {
    fn name(&self) -> &'static str {
        "HSL to Hex"
    }

    fn id(&self) -> &'static str {
        "hsl_to_hex"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Color
    }

    fn description(&self) -> &'static str {
        "Converts HSL color to hex format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if !input.starts_with("hsl(") {
            return Err(TransformError::InvalidArgument(
                "Invalid HSL format. Must start with hsl(".into(),
            ));
        }

        let color = Color::from_hsl(input)?;
        Ok(color.to_hex())
    }

    fn default_test_input(&self) -> &'static str {
        "hsl(0deg, 100%, 50%)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_to_hex() {
        let transformer = HslToHex;
        assert_eq!(
            transformer.transform("hsl(0deg, 100%, 50%)").unwrap(),
            "#ff0000"
        );
        assert_eq!(
            transformer.transform("hsl(120deg, 100%, 50%)").unwrap(),
            "#00ff00"
        );
        assert_eq!(
            transformer.transform("hsl(240deg, 100%, 50%)").unwrap(),
            "#0000ff"
        );
        assert_eq!(
            transformer.transform("hsl(0deg, 0%, 0%)").unwrap(),
            "#000000"
        );
        assert_eq!(
            transformer.transform("hsl(0deg, 0%, 100%)").unwrap(),
            "#ffffff"
        );
    }

    #[test]
    fn test_with_alpha() {
        let transformer = HslToHex;
        assert_eq!(
            transformer.transform("hsl(0deg, 100%, 50%, 1.0)").unwrap(),
            "#ff0000ff"
        );
        assert_eq!(
            transformer
                .transform("hsl(120deg, 100%, 50%, 0.5)")
                .unwrap(),
            "#00ff007f"
        );
    }

    #[test]
    fn test_invalid_input() {
        let transformer = HslToHex;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("0deg, 100%, 50%").is_err()); // Missing hsl(
                                                                    // Note: HSL implementation accepts values outside the normal range
        assert!(transformer.transform("hsl(400, 100%, 50%)").is_ok()); // This is actually valid in the color implementation
    }
}
