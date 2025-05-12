use crate::{Transform, TransformError, TransformerCategory};

/// Color Code Converter Inverse transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCodeConvertInverse;

impl Transform for ColorCodeConvertInverse {
    fn name(&self) -> &'static str {
        "Color Code Converter Inverse"
    }

    fn id(&self) -> &'static str {
        "color_code_convert_inverse"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn description(&self) -> &'static str {
        "Extracts a specific color format from the output of color_code_convert"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        let lines: Vec<&str> = input.lines().collect();
        for line in &lines {
            if line.starts_with("HEX:") {
                return Ok(line.trim_start_matches("HEX: ").to_string());
            } else if line.starts_with("RGB:") {
                return Ok(line.trim_start_matches("RGB: ").to_string());
            } else if line.starts_with("HSL:") {
                return Ok(line.trim_start_matches("HSL: ").to_string());
            } else if line.starts_with("CMYK:") {
                return Ok(line.trim_start_matches("CMYK: ").to_string());
            }
        }
        Err(TransformError::InvalidArgument(
            "Invalid color format output".into(),
        ))
    }

    fn default_test_input(&self) -> &'static str {
        "HEX: #ff0000\nRGB: rgb(255,0,0)\nHSL: 0deg,100%,50%\nCMYK: 0%,100%,100%,0%"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformers::color_code_convert::ColorCodeConvert;

    #[test]
    fn test_extract_hex() {
        let transformer = ColorCodeConvertInverse;
        let input = ColorCodeConvert.transform("#ff0000").unwrap();
        let result = transformer.transform(&input).unwrap();
        assert_eq!(result, "#ff0000");
    }

    #[test]
    fn test_extract_rgb() {
        let transformer = ColorCodeConvertInverse;
        let input = ColorCodeConvert.transform("#ff0000").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let rgb_line = lines.iter().find(|l| l.starts_with("RGB:")).unwrap();
        let result = transformer.transform(rgb_line).unwrap();
        assert_eq!(result, "rgb(255,0,0)");
    }

    #[test]
    fn test_extract_hsl() {
        let transformer = ColorCodeConvertInverse;
        let input = ColorCodeConvert.transform("#ff0000").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let hsl_line = lines.iter().find(|l| l.starts_with("HSL:")).unwrap();
        let result = transformer.transform(hsl_line).unwrap();
        assert_eq!(result, "hsl(0deg,100%,50%)");
    }

    #[test]
    fn test_extract_cmyk() {
        let transformer = ColorCodeConvertInverse;
        let input = ColorCodeConvert.transform("#ff0000").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let cmyk_line = lines.iter().find(|l| l.starts_with("CMYK:")).unwrap();
        let result = transformer.transform(cmyk_line).unwrap();
        assert_eq!(result, "cmyk(0%,100%,100%,0%)");
    }

    #[test]
    fn test_invalid_input() {
        let transformer = ColorCodeConvertInverse;
        assert!(transformer.transform("invalid").is_err());
    }
}
