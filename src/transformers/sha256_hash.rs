use crate::{Transform, TransformError, TransformerCategory};

// SHA-256 constants (K values)
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Initial hash values (H0-H7)
const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// SHA-256 hash transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha256HashTransformer;

impl Sha256HashTransformer {
    fn pad_message(message: &[u8]) -> Vec<u8> {
        let message_len_bits = (message.len() as u64) * 8;
        let mut padded = message.to_vec();
        padded.push(0x80); // Append '1' bit

        // Append '0' bits until message length is congruent to 448 (mod 512)
        // Block size is 512 bits = 64 bytes
        // We need space for the 64-bit length, so pad until len % 64 == 56
        while padded.len() % 64 != 56 {
            padded.push(0x00);
        }

        // Append original message length as 64-bit big-endian integer
        padded.extend_from_slice(&message_len_bits.to_be_bytes());

        padded
    }

    fn process_block(h: &mut [u32; 8], block: &[u8]) {
        assert_eq!(block.len(), 64);

        let mut w = [0u32; 64];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7]; // Renamed to avoid conflict with the mutable slice `h`

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh); // Use the temporary variable hh here
    }
}

impl Transform for Sha256HashTransformer {
    fn name(&self) -> &'static str {
        "SHA-256 Hash"
    }

    fn id(&self) -> &'static str {
        "sha256hash"
    }

    fn description(&self) -> &'static str {
        "Computes the SHA-256 hash of the input text"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Crypto
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let message = input.as_bytes();
        let padded_message = Self::pad_message(message);

        let mut h = H; // Initial hash values

        for block in padded_message.chunks_exact(64) {
            Self::process_block(&mut h, block);
        }

        // Convert the final hash state (h0-h7) to a hex string
        let mut result = String::with_capacity(64);
        for val in h.iter() {
            result.push_str(&format!("{:08x}", val));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_empty_string() {
        let transformer = Sha256HashTransformer;
        let input = "";
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha256_simple_string() {
        let transformer = Sha256HashTransformer;
        let input = "hello world";
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha256_longer_string() {
        // String longer than 55 bytes to test padding across block boundary
        let transformer = Sha256HashTransformer;
        let input = "The quick brown fox jumps over the lazy dog";
        let expected = "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha256_long_string_multiple_blocks() {
        let transformer = Sha256HashTransformer;
        let input = "The quick brown fox jumps over the lazy dog."; // Note the added period
        let expected = "ef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha256_nist_short() {
        let transformer = Sha256HashTransformer;
        let input = "abc";
        // NIST FIPS 180-4 Example for SHA-256("abc")
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sha256_nist_long() {
        let transformer = Sha256HashTransformer;
        let input = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        // NIST FIPS 180-4 Example for SHA-256("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
        let expected = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
