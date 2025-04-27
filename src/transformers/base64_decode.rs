use crate::{Transform, TransformError, TransformerCategory};

/// Base64 decode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base64Decode;

impl Transform for Base64Decode {
    fn name(&self) -> &'static str {
        "Base64 Decode"
    }

    fn id(&self) -> &'static str {
        "base64decode"
    }

    fn description(&self) -> &'static str {
        "Decode Base64 text to plain text"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let decoded = base64_decode(input).map_err(|_| TransformError::Base64DecodeError)?;
        String::from_utf8(decoded).map_err(|_| TransformError::Utf8Error)
    }
}

/// Decodes base64 string to bytes without external dependencies
fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    // Creates a mapping from each base64 character to its 6-bit value
    fn create_lookup_table() -> [i8; 256] {
        let mut table = [-1i8; 256];
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .iter()
            .enumerate()
            .for_each(|(i, &c)| table[c as usize] = i as i8);
        table
    }

    let lookup = create_lookup_table();
    let input = input.trim().as_bytes();

    // Calculate output length (removing padding)
    let padding = input.iter().rev().take_while(|&&c| c == b'=').count();
    let output_len = input.len() * 3 / 4 - padding;

    let mut output = vec![0u8; output_len];
    let mut output_index = 0;

    // Process 4 input bytes at a time
    for chunk in input.chunks(4) {
        if chunk.len() < 2 {
            return Err("Invalid base64 length");
        }

        // Convert each character to its 6-bit value
        let b0 = lookup[chunk[0] as usize];
        let b1 = lookup[chunk[1] as usize];

        // Check for invalid characters
        if b0 < 0 || b1 < 0 {
            return Err("Invalid base64 character");
        }

        // Handle the first byte
        if output_index < output_len {
            output[output_index] = ((b0 << 2) | (b1 >> 4)) as u8;
            output_index += 1;
        }

        if chunk.len() > 2 {
            // Handle padding or valid character
            if chunk[2] == b'=' {
                if chunk.len() < 4 || chunk[3] != b'=' {
                    return Err("Invalid base64 padding");
                }
                continue; // Done with this chunk
            }

            let b2 = lookup[chunk[2] as usize];
            if b2 < 0 {
                return Err("Invalid base64 character");
            }

            if output_index < output_len {
                output[output_index] = (((b1 & 0xF) << 4) | (b2 >> 2)) as u8;
                output_index += 1;
            }

            if chunk.len() > 3 {
                if chunk[3] == b'=' {
                    continue; // Done with this chunk
                }

                let b3 = lookup[chunk[3] as usize];
                if b3 < 0 {
                    return Err("Invalid base64 character");
                }

                if output_index < output_len {
                    output[output_index] = (((b2 & 0x3) << 6) | b3) as u8;
                    output_index += 1;
                }
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let transformer = Base64Decode;
        assert_eq!(
            transformer.transform("SGVsbG8sIFdvcmxkIQ==").unwrap(),
            "Hello, World!"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("YQ==").unwrap(), "a");
    }
}
