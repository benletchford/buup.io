use crate::{Transform, TransformError, TransformerCategory};

/// Base64 encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base64Encode;

impl Transform for Base64Encode {
    fn name(&self) -> &'static str {
        "Base64 Encode"
    }

    fn id(&self) -> &'static str {
        "base64encode"
    }

    fn description(&self) -> &'static str {
        "Encode text to Base64 format"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(base64_encode(input.as_bytes()))
    }

    fn default_test_input(&self) -> &'static str {
        "Hello, World!"
    }
}

/// Encodes bytes to base64 without external dependencies
pub(crate) fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const PAD: u8 = b'=';

    let mut output = Vec::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        let b0 = chunk.first().copied().unwrap_or(0);
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        output.push(ALPHABET[((n >> 18) & 0x3F) as usize]);
        output.push(ALPHABET[((n >> 12) & 0x3F) as usize]);

        output.push(if chunk.len() >= 2 {
            ALPHABET[((n >> 6) & 0x3F) as usize]
        } else {
            PAD
        });

        output.push(if chunk.len() >= 3 {
            ALPHABET[(n & 0x3F) as usize]
        } else {
            PAD
        });
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let transformer = Base64Encode;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "SGVsbG8sIFdvcmxkIQ=="
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("a").unwrap(), "YQ==");
    }
}
