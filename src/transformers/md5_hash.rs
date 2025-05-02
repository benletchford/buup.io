use crate::{Transform, TransformError, TransformerCategory};

/// MD5 hash transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Md5HashTransformer;

/// Default test input for MD5 Hash
pub const DEFAULT_TEST_INPUT: &str = "buup text utility";

// MD5 Constants
// Shift amounts for each round
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

// Constants derived from the binary integer part of the sines of integers
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

// Initial hash values (A, B, C, D)
const INITIAL_STATE: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

impl Md5HashTransformer {
    fn pad_message(message: &[u8]) -> Vec<u8> {
        let message_len_bits = (message.len() as u64) * 8;
        let mut padded = message.to_vec();

        // Append '1' bit
        padded.push(0x80);

        // Append '0' bits until message length is congruent to 448 (mod 512)
        // Block size is 512 bits = 64 bytes
        // We need space for the 64-bit length, so pad until len % 64 == 56
        while padded.len() % 64 != 56 {
            padded.push(0x00);
        }

        // Append original message length as 64-bit little-endian integer
        padded.extend_from_slice(&message_len_bits.to_le_bytes());

        padded
    }

    fn process_block(state: &mut [u32; 4], block: &[u8]) {
        assert_eq!(block.len(), 64);

        // Convert the block to 16 32-bit words (little-endian)
        let mut x = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate().take(16) {
            x[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        // Initialize hash value for this chunk
        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];

        // Main loop
        for i in 0..64 {
            let (mut f, g): (u32, usize);

            if i < 16 {
                f = (b & c) | (!b & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            }

            f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(x[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }

        // Add the compressed chunk to the current hash value
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
    }
}

impl Transform for Md5HashTransformer {
    fn name(&self) -> &'static str {
        "MD5 Hash"
    }

    fn id(&self) -> &'static str {
        "md5hash"
    }

    fn description(&self) -> &'static str {
        "Computes the MD5 hash of the input text"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Crypto
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let message = input.as_bytes();
        let padded_message = Self::pad_message(message);

        // Initialize state (A, B, C, D)
        let mut state = INITIAL_STATE;

        // Process each 64-byte block
        for block in padded_message.chunks_exact(64) {
            Self::process_block(&mut state, block);
        }

        // Convert the final state to a hex string (little-endian)
        let mut result = String::with_capacity(32);
        for val in state.iter() {
            // Format with little-endian byte order
            let bytes = val.to_le_bytes();
            for byte in bytes {
                result.push_str(&format!("{:02x}", byte));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_empty_string() {
        let transformer = Md5HashTransformer;
        let input = "";
        let expected = "d41d8cd98f00b204e9800998ecf8427e";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_md5_simple_string() {
        let transformer = Md5HashTransformer;
        let input = DEFAULT_TEST_INPUT;
        let expected = "9da2993109f7a900639e09276ead55a8";
        assert_eq!(transformer.transform(input).unwrap(), expected);

        let input_hw = "hello world";
        let expected_hw = "5eb63bbbe01eeed093cb22bb8f5acdc3";
        assert_eq!(transformer.transform(input_hw).unwrap(), expected_hw);
    }

    #[test]
    fn test_md5_longer_string() {
        // String longer than 55 bytes to test padding across block boundary
        let transformer = Md5HashTransformer;
        let input = "The quick brown fox jumps over the lazy dog";
        let expected = "9e107d9d372bb6826bd81d3542a419d6";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_md5_another_test_case() {
        let transformer = Md5HashTransformer;
        let input = "The quick brown fox jumps over the lazy dog."; // Note the added period
        let expected = "e4d909c290d0fb1ca068ffaddf22cbd0";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_md5_rfc_test_case_1() {
        let transformer = Md5HashTransformer;
        let input = "abc";
        // RFC 1321 test vector
        let expected = "900150983cd24fb0d6963f7d28e17f72";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_md5_rfc_test_case_2() {
        let transformer = Md5HashTransformer;
        let input = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        // Common test vector
        let expected = "8215ef0796a20bcaaae116d3876c664a";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
