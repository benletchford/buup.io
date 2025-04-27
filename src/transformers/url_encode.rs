use crate::{Transform, TransformError, TransformerCategory};

/// URL encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlEncode;

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
        Ok(url_encode(input))
    }
}

/// URL encodes a string without external dependencies
fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);

    for byte in input.bytes() {
        match byte {
            // RFC 3986 Unreserved Characters (always safe to use)
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            // Space is encoded as '+' (application/x-www-form-urlencoded)
            b' ' => result.push('+'),
            // Everything else is percent-encoded
            _ => {
                result.push('%');
                result.push(
                    char::from_digit((byte >> 4) as u32, 16)
                        .unwrap_or('0')
                        .to_ascii_uppercase(),
                );
                result.push(
                    char::from_digit((byte & 0xF) as u32, 16)
                        .unwrap_or('0')
                        .to_ascii_uppercase(),
                );
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        let transformer = UrlEncode;
        assert_eq!(
            transformer.transform("Hello, World!").unwrap(),
            "Hello%2C+World%21"
        );
        assert_eq!(transformer.transform("a b").unwrap(), "a+b");
        assert_eq!(transformer.transform("100%").unwrap(), "100%25");
    }
}
