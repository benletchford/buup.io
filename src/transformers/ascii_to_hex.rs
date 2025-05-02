use crate::{Transform, TransformError, TransformerCategory};

/// Transformer to convert ASCII characters to their hexadecimal representation.
///
/// # Example
/// ```rust
/// use buup::{Transform, transformers::AsciiToHex};
/// let transformer = AsciiToHex;
/// assert_eq!(transformer.transform("Hello").unwrap(), "48656c6c6f");
/// assert_eq!(transformer.transform("").unwrap(), "");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AsciiToHex;

impl Transform for AsciiToHex {
    fn name(&self) -> &'static str {
        "ASCII to Hex"
    }

    fn id(&self) -> &'static str {
        "ascii_to_hex"
    }

    fn description(&self) -> &'static str {
        "Convert ASCII characters to their hexadecimal representation."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        use std::fmt::Write;

        input
            .chars()
            .try_fold(String::with_capacity(input.len() * 2), |mut output, c| {
                write!(output, "{:02x}", c as u8)
                    .map_err(|e| TransformError::InvalidArgument(e.to_string().into()))?;
                Ok(output)
            })
    }

    fn default_test_input(&self) -> &'static str {
        "Hello"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_hex() {
        let transformer = AsciiToHex;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "48656c6c6f"
        );
        assert_eq!(transformer.transform("World").unwrap(), "576f726c64");
        assert_eq!(transformer.transform("123").unwrap(), "313233");
        assert_eq!(transformer.transform(" ").unwrap(), "20"); // Space character
        assert_eq!(transformer.transform("").unwrap(), ""); // Empty string
        assert_eq!(transformer.transform("!@#").unwrap(), "214023");
    }

    #[test]
    fn test_properties() {
        let transformer = AsciiToHex;
        assert_eq!(transformer.name(), "ASCII to Hex");
        assert_eq!(transformer.id(), "ascii_to_hex");
        assert_eq!(
            transformer.description(),
            "Convert ASCII characters to their hexadecimal representation."
        );
        assert_eq!(transformer.category(), TransformerCategory::Encoder);
        assert_eq!(transformer.default_test_input(), "Hello"); // Test the new method
    }

    // Test with non-ASCII characters (behavior depends on how char -> u8 conversion works)
    // Rust truncates unicode chars to u8, effectively taking the low byte.
    // This might be unexpected for users, but it's consistent.
    #[test]
    fn test_non_ascii() {
        let transformer = AsciiToHex;
        // 'é' (U+00E9) -> byte 0xE9 -> hex "e9"
        assert_eq!(transformer.transform("é").unwrap(), "e9");
        // '€' (U+20AC) -> byte 0xAC -> hex "ac"
        assert_eq!(transformer.transform("€").unwrap(), "ac");
        // Mixed ASCII and non-ASCII
        assert_eq!(transformer.transform("Héllo").unwrap(), "48e96c6c6f");
    }
}
