use crate::{Transform, TransformError, TransformerCategory};

/// URL encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlEncode;

/// Default test input for URL Encode
pub const DEFAULT_TEST_INPUT: &str = "Hello, World! This is a test + example?";

impl Transform for UrlEncode {
    fn name(&self) -> &'static str {
        "URL Encode"
    }

    fn id(&self) -> &'static str {
        "urlencode"
    }

    fn description(&self) -> &'static str {
        "Encode text for use in URLs"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut encoded = String::new();
        for byte in input.bytes() {
            match byte {
                // Alphanumeric characters are not encoded
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => encoded.push(byte as char),
                // Specific characters are not encoded (- _ . ~)
                b'-' | b'_' | b'.' | b'~' => encoded.push(byte as char),
                // Space is encoded as '+' (common practice, though %20 is also valid)
                b' ' => encoded.push('+'),
                // All other characters are percent-encoded
                _ => {
                    encoded.push('%');
                    encoded.push_str(&format!("{:02X}", byte));
                }
            }
        }
        Ok(encoded)
    }

    fn default_test_input(&self) -> &'static str {
        DEFAULT_TEST_INPUT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        let transformer = UrlEncode;
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "Hello%2C+World%21+This+is+a+test+%2B+example%3F"
        );
        assert_eq!(transformer.transform("a b").unwrap(), "a+b");
        assert_eq!(transformer.transform("100%").unwrap(), "100%25");
    }
}
