use crate::{Transform, TransformError, TransformerCategory};
// Import the shared base64 encoder
use super::base64_encode;

// Length and Distance Codes from RFC 1951 Section 3.2.5
pub(crate) const LENGTH_CODES: [(u16, u16, u8); 29] = [
    (257, 3, 0),
    (258, 4, 0),
    (259, 5, 0),
    (260, 6, 0),
    (261, 7, 0),
    (262, 8, 0),
    (263, 9, 0),
    (264, 10, 0),
    (265, 11, 1),
    (266, 13, 1),
    (267, 15, 1),
    (268, 17, 1),
    (269, 19, 2),
    (270, 23, 2),
    (271, 27, 2),
    (272, 31, 2),
    (273, 35, 3),
    (274, 43, 3),
    (275, 51, 3),
    (276, 59, 3),
    (277, 67, 4),
    (278, 83, 4),
    (279, 99, 4),
    (280, 115, 4),
    (281, 131, 5),
    (282, 163, 5),
    (283, 195, 5),
    (284, 227, 5),
    (285, 258, 0),
];

pub(crate) const DISTANCE_CODES: [(u16, u16, u8); 30] = [
    (0, 1, 0),
    (1, 2, 0),
    (2, 3, 0),
    (3, 4, 0),
    (4, 5, 1),
    (5, 7, 1),
    (6, 9, 2),
    (7, 13, 2),
    (8, 17, 3),
    (9, 25, 3),
    (10, 33, 4),
    (11, 49, 4),
    (12, 65, 5),
    (13, 97, 5),
    (14, 129, 6),
    (15, 193, 6),
    (16, 257, 7),
    (17, 385, 7),
    (18, 513, 8),
    (19, 769, 8),
    (20, 1025, 9),
    (21, 1537, 9),
    (22, 2049, 10),
    (23, 3073, 10),
    (24, 4097, 11),
    (25, 6145, 11),
    (26, 8193, 12),
    (27, 12289, 12),
    (28, 16385, 13),
    (29, 24577, 13),
];

// Finds the DEFLATE length code and extra bits for a given length (3-258).
fn get_length_code(length: u16) -> (u16, u32, u8) {
    assert!(
        (3..=258).contains(&length),
        "Length must be between 3 and 258 inclusive"
    );
    if length == 258 {
        return (285, 0, 0);
    }
    for i in 0..LENGTH_CODES.len() - 1 {
        let (code, base_len, num_extra_bits) = LENGTH_CODES[i];
        let next_base_len = if i + 1 < LENGTH_CODES.len() - 1 {
            LENGTH_CODES[i + 1].1
        } else {
            258
        };
        let range_limit = base_len + (1 << num_extra_bits) - 1;
        if length >= base_len && length <= range_limit {
            let extra_val = length - base_len;
            return (code, extra_val as u32, num_extra_bits);
        }
        if length > range_limit && length < next_base_len {
            panic!("Length {} falls between code ranges", length);
        }
    }
    panic!("Length code not found for {}", length);
}

// Finds the DEFLATE distance code and extra bits for a given distance (1-32768).
fn get_distance_code(distance: u16) -> (u16, u32, u8) {
    assert!(
        (1..=32768).contains(&distance),
        "Distance must be between 1 and 32768 inclusive"
    );
    for i in 0..DISTANCE_CODES.len() {
        let (code, base_dist, num_extra_bits) = DISTANCE_CODES[i];
        let range_limit = base_dist + (1 << num_extra_bits) - 1;
        if distance >= base_dist && distance <= range_limit {
            let extra_val = distance - base_dist;
            return (code, extra_val as u32, num_extra_bits);
        }
        if i + 1 < DISTANCE_CODES.len() {
            let next_base_dist = DISTANCE_CODES[i + 1].1;
            if distance > range_limit && distance < next_base_dist {
                panic!("Distance {} falls between code ranges", distance);
            }
        } else if distance > range_limit {
            panic!("Distance {} is out of bounds (> 32768?)", distance);
        }
    }
    panic!("Distance code not found for {}", distance);
}

/// Get length base and extra bits count from length code (257-285)
pub(crate) fn get_length_info(code: u16) -> (u16, u8) {
    // (base, extra_bits)
    assert!((257..=285).contains(&code));
    for &(c, base, extra) in LENGTH_CODES.iter() {
        if c == code {
            return (base, extra);
        }
    }
    unreachable!(); // Code asserted to be in range
}

/// Get distance base and extra bits count from distance code (0-29)
pub(crate) fn get_distance_info(code: u16) -> (u16, u8) {
    // (base, extra_bits)
    assert!(code <= 29);
    for &(c, base, extra) in DISTANCE_CODES.iter() {
        if c == code {
            return (base, extra);
        }
    }
    unreachable!(); // Code asserted to be in range
}

// Reverses the lowest `num_bits` of `value`.
pub(crate) fn reverse_bits(value: u16, num_bits: u8) -> u16 {
    let mut result = 0u16;
    let mut v = value;
    for _ in 0..num_bits {
        result <<= 1;
        if (v & 1) == 1 {
            result |= 1;
        }
        v >>= 1;
    }
    result
}

// Returns the bit-reversed fixed Huffman code pattern and bit length for a given literal/length code (0-285).
fn get_fixed_literal_length_huffman_code(code: u16) -> (u16, u8) {
    let (pattern, num_bits) = match code {
        0..=143 => (0b00110000 + code, 8),
        144..=255 => (0b110010000 + (code - 144u16), 9),
        256..=279 => (code - 256u16, 7),
        280..=285 => (0b11000000 + (code - 280u16), 8),
        _ => panic!("Invalid literal/length code for fixed Huffman: {}", code),
    };
    (reverse_bits(pattern, num_bits), num_bits)
}

// Returns the bit-reversed fixed Huffman code pattern (5 bits) and bit length for a given distance code (0-29).
fn get_fixed_distance_huffman_code(distance_code: u16) -> (u16, u8) {
    let num_bits = 5;
    if distance_code <= 29 {
        (reverse_bits(distance_code, num_bits), num_bits)
    } else {
        panic!("Invalid distance code for fixed Huffman: {}", distance_code);
    }
}

// Writes bits LSB-first into a byte vector.
struct BitWriter {
    bytes: Vec<u8>,
    current_byte: u8,
    bit_position: u8, // Next bit position to write (0-7)
}

impl BitWriter {
    fn new() -> Self {
        BitWriter {
            bytes: Vec::new(),
            current_byte: 0,
            bit_position: 0,
        }
    }

    fn write_bits(&mut self, mut value: u32, mut num_bits: u8) {
        while num_bits > 0 {
            let remaining_bits_in_byte = 8 - self.bit_position;
            let bits_to_write = std::cmp::min(num_bits, remaining_bits_in_byte);
            let bit_mask = (1u32 << bits_to_write) - 1;
            let bits = (value & bit_mask) as u8;
            self.current_byte |= bits << self.bit_position;
            self.bit_position += bits_to_write;
            if self.bit_position == 8 {
                self.bytes.push(self.current_byte);
                self.current_byte = 0;
                self.bit_position = 0;
            }
            value >>= bits_to_write;
            num_bits -= bits_to_write;
        }
    }

    fn flush_byte(&mut self) {
        if self.bit_position > 0 {
            self.bytes.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    fn get_bytes(mut self) -> Vec<u8> {
        self.flush_byte();
        self.bytes
    }

    fn align_to_byte(&mut self) {
        if self.bit_position > 0 {
            self.bytes.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    fn write_bytes_raw(&mut self, bytes: &[u8]) {
        assert!(self.bit_position == 0, "Writer must be byte-aligned");
        self.bytes.extend_from_slice(bytes);
    }
}

// --- LZ77 Implementation ---
const MAX_WINDOW_SIZE: usize = 32 * 1024;
const MIN_MATCH_LEN: usize = 3;
const MAX_MATCH_LEN: usize = 258;
const HASH_TABLE_SIZE: usize = 1 << 15;

#[derive(Debug, Clone, PartialEq)]
enum Lz77Token {
    Literal(u8),
    Match(u16, u16), // length, distance
}

fn lz77_compress(input: &[u8]) -> Vec<Lz77Token> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut tokens = Vec::new();
    let mut head: Vec<Option<usize>> = vec![None; HASH_TABLE_SIZE];
    let mut prev: Vec<Option<usize>> = vec![None; MAX_WINDOW_SIZE];
    let mut current_pos = 0;
    while current_pos < input.len() {
        let window_start = if current_pos > MAX_WINDOW_SIZE {
            current_pos - MAX_WINDOW_SIZE
        } else {
            0
        };
        if current_pos + MIN_MATCH_LEN > input.len() {
            tokens.extend(input[current_pos..].iter().map(|&b| Lz77Token::Literal(b)));
            break;
        }
        let hash = calculate_hash(&input[current_pos..current_pos + MIN_MATCH_LEN]);
        let mut best_match_len = 0;
        let mut best_match_dist = 0;
        let mut match_pos_opt = head[hash];
        while let Some(match_pos) = match_pos_opt {
            if match_pos < window_start {
                break;
            }
            let current_match_len =
                calculate_match_length(input, match_pos, current_pos, MAX_MATCH_LEN);
            if current_match_len >= MIN_MATCH_LEN && current_match_len > best_match_len {
                best_match_len = current_match_len;
                best_match_dist = (current_pos - match_pos) as u16;
                if best_match_len == MAX_MATCH_LEN {
                    break;
                }
            }
            match_pos_opt = prev[match_pos % MAX_WINDOW_SIZE];
        }
        prev[current_pos % MAX_WINDOW_SIZE] = head[hash];
        head[hash] = Some(current_pos);
        if best_match_len >= MIN_MATCH_LEN {
            tokens.push(Lz77Token::Match(best_match_len as u16, best_match_dist));
            // Lazy update hash table for skipped bytes
            for i in 1..best_match_len {
                let pos_to_update = current_pos + i;
                if pos_to_update + MIN_MATCH_LEN <= input.len() {
                    let next_hash =
                        calculate_hash(&input[pos_to_update..pos_to_update + MIN_MATCH_LEN]);
                    prev[pos_to_update % MAX_WINDOW_SIZE] = head[next_hash];
                    head[next_hash] = Some(pos_to_update);
                }
            }
            current_pos += best_match_len;
        } else {
            tokens.push(Lz77Token::Literal(input[current_pos]));
            current_pos += 1;
        }
    }
    tokens
}

#[inline]
fn calculate_hash(bytes: &[u8]) -> usize {
    (((bytes[0] as usize) << 8) | ((bytes[1] as usize) << 4) | (bytes[2] as usize))
        % HASH_TABLE_SIZE
}

#[inline]
fn calculate_match_length(input: &[u8], pos1: usize, pos2: usize, max_len: usize) -> usize {
    let mut len = 0;
    let input_len = input.len();
    while len < max_len && pos2 + len < input_len && input[pos1 + len] == input[pos2 + len] {
        len += 1;
    }
    len
}

// Extracted core DEFLATE compression logic (without Base64 encoding)
pub(crate) fn deflate_bytes(input_bytes: &[u8]) -> Result<Vec<u8>, TransformError> {
    let mut writer = BitWriter::new();

    if input_bytes.is_empty() {
        // Minimal fixed block for empty input.
        writer.write_bits(1, 1); // BFINAL
        writer.write_bits(1, 2); // BTYPE=01 (Fixed Huffman)
        let (reversed_eob_huff, eob_bits) = get_fixed_literal_length_huffman_code(256); // EOB
        writer.write_bits(reversed_eob_huff as u32, eob_bits);
        return Ok(writer.get_bytes());
    }

    let lz77_tokens = lz77_compress(input_bytes);

    // Estimate size to choose between fixed Huffman and uncompressed block.
    let mut estimated_bits = 0;
    for token in &lz77_tokens {
        match token {
            Lz77Token::Literal(byte) => {
                let (_, bits) = get_fixed_literal_length_huffman_code(*byte as u16);
                estimated_bits += bits as usize;
            }
            Lz77Token::Match(length, distance) => {
                let (len_code, _, len_extra_bits) = get_length_code(*length);
                let (_, len_huff_bits) = get_fixed_literal_length_huffman_code(len_code);
                estimated_bits += len_huff_bits as usize + len_extra_bits as usize;

                let (dist_code, _, dist_extra_bits) = get_distance_code(*distance);
                let (_, dist_huff_bits) = get_fixed_distance_huffman_code(dist_code);
                estimated_bits += dist_huff_bits as usize + dist_extra_bits as usize;
            }
        }
    }
    let (_, eob_bits) = get_fixed_literal_length_huffman_code(256); // EOB marker
    estimated_bits += eob_bits as usize;
    estimated_bits += 3; // BFINAL + BTYPE bits

    let uncompressed_size_bytes = input_bytes.len() + 5;
    let uncompressed_size_bits = uncompressed_size_bytes * 8;

    // --- Write DEFLATE Stream ---
    writer.write_bits(1, 1); // BFINAL = 1

    if estimated_bits >= uncompressed_size_bits {
        // Write uncompressed block (BTYPE=00).
        writer.write_bits(0, 2); // BTYPE=00
        writer.align_to_byte();
        let len: u16 = input_bytes.len().try_into().map_err(|_| {
            TransformError::CompressionError(
                "Input too large for uncompressed block length (max 65535)".into(),
            )
        })?;
        let nlen = !len;
        writer.write_bytes_raw(&len.to_le_bytes());
        writer.write_bytes_raw(&nlen.to_le_bytes());
        writer.write_bytes_raw(input_bytes);
    } else {
        // Write fixed Huffman block (BTYPE=01).
        writer.write_bits(1, 2); // BTYPE=01
        for token in lz77_tokens {
            match token {
                Lz77Token::Match(length, distance) => {
                    let (len_code, len_extra_val, len_extra_bits) = get_length_code(length);
                    let (reversed_len_huff, len_huff_bits) =
                        get_fixed_literal_length_huffman_code(len_code);
                    writer.write_bits(reversed_len_huff as u32, len_huff_bits);
                    if len_extra_bits > 0 {
                        writer.write_bits(len_extra_val, len_extra_bits);
                    }

                    let (dist_code, dist_extra_val, dist_extra_bits) = get_distance_code(distance);
                    let (reversed_dist_huff, dist_huff_bits) =
                        get_fixed_distance_huffman_code(dist_code);
                    writer.write_bits(reversed_dist_huff as u32, dist_huff_bits);
                    if dist_extra_bits > 0 {
                        writer.write_bits(dist_extra_val, dist_extra_bits);
                    }
                }
                Lz77Token::Literal(byte) => {
                    let (reversed_huff, huff_bits) =
                        get_fixed_literal_length_huffman_code(byte as u16);
                    writer.write_bits(reversed_huff as u32, huff_bits);
                }
            }
        }
        // EOB marker.
        let (reversed_eob_huff, eob_bits) = get_fixed_literal_length_huffman_code(256);
        writer.write_bits(reversed_eob_huff as u32, eob_bits);
    }

    Ok(writer.get_bytes())
}

/// Compresses input using the DEFLATE algorithm (RFC 1951).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeflateCompress;

impl Transform for DeflateCompress {
    fn name(&self) -> &'static str {
        "DEFLATE Compress"
    }

    fn id(&self) -> &'static str {
        "deflatecompress"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Compression
    }

    fn description(&self) -> &'static str {
        "Compresses input using the DEFLATE algorithm (RFC 1951) and encodes the output as Base64."
    }

    // Updated transform method uses deflate_bytes
    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input_bytes = input.as_bytes();
        let compressed_data = deflate_bytes(input_bytes)?; // Call extracted function
        Ok(base64_encode::base64_encode(&compressed_data)) // Base64 encode result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deflate_empty() {
        let transformer = DeflateCompress;
        let result = transformer.transform("");
        assert!(result.is_ok());
        // Expected raw DEFLATE for empty fixed block is [0x03, 0x00]
        assert_eq!(result.unwrap(), "AwA=");
    }

    #[test]
    fn test_deflate_simple() {
        let transformer = DeflateCompress;
        let input = "Hello, world!";
        let expected_base64 = "80jNycnXUSjPL8pJUQQA";
        match transformer.transform(input) {
            Ok(actual_base64) => {
                assert_eq!(actual_base64, expected_base64);
            }
            Err(e) => {
                panic!("transform failed for input '{}': {:?}", input, e);
            }
        }
    }

    #[test]
    fn test_deflate_repeated() {
        let transformer = DeflateCompress;
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let expected_base64 = "SyQZAAA=";
        match transformer.transform(input) {
            Ok(actual_base64) => {
                assert_eq!(actual_base64, expected_base64);
            }
            Err(e) => {
                panic!("transform failed for input '{}': {:?}", input, e);
            }
        }
    }

    #[test]
    fn test_deflate_longer_text() {
        let transformer = DeflateCompress;
        let input =
            "This is a slightly longer test string to see how DEFLATE compression handles it.";
        let expected_base64 = "C8nILFYAokSF4pzM9IySnEqFnPy89NQihZLU4hKF4pKizLx0hZJ8heLUVIWM/HIFF1c3H8cQV4Xk/NyCotTi4sz8PIWMxLyUnFSgOSV6AA==";
        match transformer.transform(input) {
            Ok(actual_base64) => {
                assert_eq!(actual_base64, expected_base64);
            }
            Err(e) => {
                panic!("transform failed for input '{}': {:?}", input, e);
            }
        }
    }
}
