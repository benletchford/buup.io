use crate::{Transform, TransformError, TransformerCategory};

/// Transformer to convert hexadecimal representation back to ASCII characters.
///
/// # Example
/// ```rust
/// use buup::{Transform, transformers::HexToAscii};
/// let transformer = HexToAscii;
/// assert_eq!(transformer.transform("48656c6c6f").unwrap(), "Hello");
/// assert_eq!(transformer.transform("").unwrap(), "");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HexToAscii;

impl Transform for HexToAscii {
    fn name(&self) -> &'static str {
        "Hex to ASCII"
    }

    fn id(&self) -> &'static str {
        "hex_to_ascii"
    }

    fn description(&self) -> &'static str {
        "Convert hexadecimal representation back to ASCII characters."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Ensure input has an even number of characters
        if input.len() % 2 != 0 {
            return Err(TransformError::InvalidArgument(
                "Input hex string must have an even number of characters".into(),
            ));
        }

        // Remove common prefixes like 0x or spaces
        let cleaned_input = input.trim().trim_start_matches("0x");

        let mut bytes = Vec::with_capacity(cleaned_input.len() / 2);
        let mut chars = cleaned_input.chars();

        while let (Some(h), Some(l)) = (chars.next(), chars.next()) {
            let hex_pair = format!("{}{}", h, l);
            match u8::from_str_radix(&hex_pair, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => {
                    return Err(TransformError::InvalidArgument(
                        format!("Invalid hex character sequence found: '{}'", hex_pair).into(),
                    ))
                }
            }
        }

        String::from_utf8(bytes).map_err(|_| TransformError::Utf8Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_ascii() {
        let transformer = HexToAscii;
        assert_eq!(transformer.transform("48656c6c6f").unwrap(), "Hello");
        assert_eq!(transformer.transform("576f726c64").unwrap(), "World");
        assert_eq!(transformer.transform("313233").unwrap(), "123");
        assert_eq!(transformer.transform("20").unwrap(), " "); // Space character
        assert_eq!(transformer.transform("").unwrap(), ""); // Empty string
        assert_eq!(transformer.transform("214023").unwrap(), "!@#");
    }

    #[test]
    fn test_invalid_hex() {
        let transformer = HexToAscii;
        // Invalid hex character 'G'
        assert!(matches!(
            transformer.transform("48656c6c6G"),
            Err(TransformError::InvalidArgument(_))
        ));
        // Odd number of characters
        assert!(matches!(
            transformer.transform("48656c6c6"),
            Err(TransformError::InvalidArgument(_))
        ));
    }

    #[test]
    fn test_non_utf8_output() {
        let transformer = HexToAscii;
        // Represents invalid UTF-8 sequence (e.g., lone continuation byte)
        assert!(matches!(
            transformer.transform("80"),
            Err(TransformError::Utf8Error)
        ));
        assert!(matches!(
            transformer.transform("c0"),
            Err(TransformError::Utf8Error)
        )); // Overlong encoding start
    }

    #[test]
    fn test_properties() {
        let transformer = HexToAscii;
        assert_eq!(transformer.name(), "Hex to ASCII");
        assert_eq!(transformer.id(), "hex_to_ascii");
        assert_eq!(
            transformer.description(),
            "Convert hexadecimal representation back to ASCII characters."
        );
        assert_eq!(transformer.category(), TransformerCategory::Decoder);
    }
}
