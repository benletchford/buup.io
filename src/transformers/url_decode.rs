use crate::{Transform, TransformError, TransformerCategory};

/// URL decode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlDecode;

impl Transform for UrlDecode {
    fn name(&self) -> &'static str {
        "URL Decode"
    }

    fn id(&self) -> &'static str {
        "urldecode"
    }

    fn description(&self) -> &'static str {
        "Decode URL-encoded text"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        url_decode(input).map_err(|_| TransformError::UrlDecodeError)
    }
}

/// URL decodes a string without external dependencies
fn url_decode(input: &str) -> Result<String, &'static str> {
    // We'll collect decoded bytes and convert to UTF-8 at the end
    let mut decoded_bytes = Vec::with_capacity(input.len());
    let mut bytes = input.bytes();

    while let Some(byte) = bytes.next() {
        match byte {
            // '+' is decoded as space
            b'+' => decoded_bytes.push(b' '),
            // '%' begins a percent-encoded sequence
            b'%' => {
                // Get the two hex digits
                let hi = bytes
                    .next()
                    .ok_or("Invalid URL encoding: unexpected end of input")?;
                let lo = bytes
                    .next()
                    .ok_or("Invalid URL encoding: unexpected end of input")?;

                // Parse the hex digits to get the byte value
                let hex_to_digit = |b| match b {
                    b'0'..=b'9' => Ok(b - b'0'),
                    b'A'..=b'F' => Ok(b - b'A' + 10),
                    b'a'..=b'f' => Ok(b - b'a' + 10),
                    _ => Err("Invalid URL encoding: invalid hex digit"),
                };

                let high_nibble = hex_to_digit(hi)?;
                let low_nibble = hex_to_digit(lo)?;

                let decoded_byte = (high_nibble << 4) | low_nibble;
                decoded_bytes.push(decoded_byte);
            }
            // Regular characters
            _ => decoded_bytes.push(byte),
        }
    }

    // Convert the collected bytes to a UTF-8 string
    String::from_utf8(decoded_bytes).map_err(|_| "Invalid UTF-8 sequence in decoded URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_decode() {
        let transformer = UrlDecode;
        assert_eq!(
            transformer.transform("Hello%2C+World%21").unwrap(),
            "Hello, World!"
        );
        assert_eq!(transformer.transform("a+b").unwrap(), "a b");
        assert_eq!(transformer.transform("100%25").unwrap(), "100%");
    }
}
