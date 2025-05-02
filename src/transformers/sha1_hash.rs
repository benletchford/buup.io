use crate::{Transform, TransformError, TransformerCategory};

// SHA-1 constants
const H0: u32 = 0x67452301;
const H1: u32 = 0xEFCDAB89;
const H2: u32 = 0x98BADCFE;
const H3: u32 = 0x10325476;
const H4: u32 = 0xC3D2E1F0;

/// SHA-1 hash transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha1Hash;

// Default test input for SHA1 Hash
// pub const DEFAULT_TEST_INPUT: &str = "buup text utility";

impl Sha1Hash {
    // Pads the message according to SHA-1 standard (RFC 3174)
    fn pad_message(message: &[u8]) -> Vec<u8> {
        let message_len_bits = (message.len() as u64) * 8;
        let mut padded = message.to_vec();
        padded.push(0x80); // Append '1' bit

        // Append '0' bits until message length is congruent to 448 (mod 512)
        while padded.len() % 64 != 56 {
            padded.push(0x00);
        }

        // Append original message length as 64-bit big-endian integer
        padded.extend_from_slice(&message_len_bits.to_be_bytes());

        padded
    }

    // Processes a single 512-bit (64-byte) block
    fn process_block(h: &mut [u32; 5], block: &[u8]) {
        assert_eq!(block.len(), 64);

        let mut w = [0u32; 80];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];

        for (i, w_i) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => (((b & c) | (!b & d)), 0x5A827999),
                20..=39 => ((b ^ c ^ d), 0x6ED9EBA1),
                40..=59 => (((b & c) | (b & d) | (c & d)), 0x8F1BBCDC),
                60..=79 => ((b ^ c ^ d), 0xCA62C1D6),
                _ => unreachable!(), // Should not happen
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(*w_i);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
}

impl Transform for Sha1Hash {
    fn name(&self) -> &'static str {
        "SHA-1 Hash"
    }

    fn id(&self) -> &'static str {
        "sha1hash"
    }

    fn description(&self) -> &'static str {
        "Computes the SHA-1 hash of the input text (Warning: SHA-1 is cryptographically weak)"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Crypto
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let message = input.as_bytes();
        let padded_message = Self::pad_message(message);

        let mut h = [H0, H1, H2, H3, H4]; // Initial hash values

        for block in padded_message.chunks_exact(64) {
            Self::process_block(&mut h, block);
        }

        // Convert the final hash state (h0-h4) to a hex string
        let mut result = String::with_capacity(40); // SHA-1 output is 160 bits = 20 bytes = 40 hex chars
        for val in h.iter() {
            result.push_str(&format!("{:08x}", val));
        }

        Ok(result)
    }

    fn default_test_input(&self) -> &'static str {
        "buup"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_empty_string() {
        let transformer = Sha1Hash;
        let input = "";
        let expected = "da39a3ee5e6b4b0d3255bfef95601890afd80709";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha1_simple_string() {
        let transformer = Sha1Hash;
        let input = transformer.default_test_input();
        let expected = "fb68687a3bc7428da3ddeecabc907bea236ae70b";
        assert_eq!(transformer.transform(input).unwrap(), expected);

        let input_hw = "hello world";
        let expected_hw = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
        assert_eq!(transformer.transform(input_hw).unwrap(), expected_hw);
    }

    #[test]
    fn test_sha1_rfc3174_test_case_1() {
        // Test Case 1 from RFC 3174
        let transformer = Sha1Hash;
        let input = "abc";
        let expected = "a9993e364706816aba3e25717850c26c9cd0d89d";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha1_rfc3174_test_case_2() {
        // Test Case 2 from RFC 3174
        let transformer = Sha1Hash;
        let input = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let expected = "84983e441c3bd26ebaae4aa1f95129e5e54670f1";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha1_million_a_chars() {
        // Test Case: 1 million 'a' characters
        // This is a common, unofficial stress test vector, not from RFC 3174.
        let transformer = Sha1Hash;
        let input = String::from("a").repeat(1_000_000);
        let expected = "34aa973cd4c4daa4f61eeb2bdbad27316534016f";
        assert_eq!(transformer.transform(&input).unwrap(), expected);
    }

    #[test]
    fn test_sha1_long_string_multiple_blocks() {
        // String requiring multiple blocks processing
        let transformer = Sha1Hash;
        let input = "The quick brown fox jumps over the lazy dog.";
        let expected = "408d94384216f890ff7a0c3528e8bed1e0b01621";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
