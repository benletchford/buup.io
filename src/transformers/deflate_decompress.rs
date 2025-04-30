use super::base64_decode;
use super::deflate_compress;
use crate::{Transform, TransformError, TransformerCategory};
use std::collections::HashMap;

/// Decompresses DEFLATE compressed input (RFC 1951).
/// Supports Base64 encoded input containing uncompressed (BTYPE=00)
/// and fixed Huffman (BTYPE=01) blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeflateDecompress;

// Reads bits LSB-first from a byte slice.
pub(crate) struct BitReader<'a> {
    bytes: &'a [u8],
    byte_index: usize, // Start at the beginning
    bit_position: u8,  // Next bit to read (0-7)
}

impl<'a> BitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        BitReader {
            bytes,
            byte_index: 0,
            bit_position: 0,
        }
    }

    // Reads `num_bits` (up to 32) from the stream.
    fn read_bits(&mut self, num_bits: u8) -> Result<u32, TransformError> {
        if num_bits > 32 {
            return Err(TransformError::CompressionError(
                "Cannot read more than 32 bits at once".to_string(),
            ));
        }
        let mut value = 0u32;
        let mut bits_read = 0u8;
        while bits_read < num_bits {
            if self.byte_index >= self.bytes.len() {
                // Original logic: Handle potential EOF by checking for padding bits.
                if bits_read < num_bits {
                    // Allow reading up to 7 padding bits (zeros)
                    if num_bits - bits_read > 7 {
                        return Err(TransformError::CompressionError(
                            "Unexpected end of DEFLATE stream (large bit request past EOF)"
                                .to_string(),
                        ));
                    }
                    // Assume remaining bits are 0, effectively padding the value.
                    break; // Stop reading for this call
                }
                // If bits_read == num_bits, we finished reading before hitting EOF, which is fine.
            }

            let current_byte = self.bytes[self.byte_index];
            let bits_to_read_from_byte = 8 - self.bit_position;
            let bits_needed = num_bits - bits_read;
            let bits_to_read = std::cmp::min(bits_needed, bits_to_read_from_byte);

            // Extract bits from current_byte
            let mask = (1u32 << bits_to_read) - 1;
            let byte_part = (current_byte >> self.bit_position) & (mask as u8);
            value |= (byte_part as u32) << bits_read;

            self.bit_position += bits_to_read;
            bits_read += bits_to_read;

            if self.bit_position == 8 {
                self.bit_position = 0;
                self.byte_index += 1;
            }
        }
        Ok(value)
    }

    // Discards bits to align to the next byte boundary.
    fn align_to_byte(&mut self) {
        if self.bit_position > 0 {
            self.bit_position = 0;
            self.byte_index += 1;
        }
    }

    // Returns the number of bytes remaining, including the current partial byte.
    fn remaining_bytes(&self) -> usize {
        self.bytes.len().saturating_sub(self.byte_index)
    }
}

// --- Fixed Huffman Decode Tables ---
const MAX_BITS_LITLEN: u8 = 9;
const MAX_BITS_DIST: u8 = 5;

#[derive(Clone)]
struct HuffmanCode {
    symbol: u16,
    length: u8,
}

// Fixed Huffman decoder using HashMap lookup.
struct FixedHuffmanDecoder {
    litlen_lookup: HashMap<u16, HuffmanCode>,
    dist_lookup: HashMap<u16, HuffmanCode>,
}

impl FixedHuffmanDecoder {
    fn new() -> Self {
        let (litlen_table, dist_table) = Self::build_fixed_tables();
        FixedHuffmanDecoder {
            litlen_lookup: litlen_table,
            dist_lookup: dist_table,
        }
    }

    /// Builds the lookup tables for Fixed Huffman codes as per RFC 1951 Sec 3.2.6
    fn build_fixed_tables() -> (HashMap<u16, HuffmanCode>, HashMap<u16, HuffmanCode>) {
        let mut litlen_lookup = HashMap::new();
        let mut dist_lookup = HashMap::new();

        // Literal/Length codes
        for symbol in 0..=287u16 {
            let (code, len) = match symbol {
                0..=143 => (0x30 + symbol, 8),
                144..=255 => (0x190 + (symbol - 144), 9),
                256..=279 => (symbol - 256, 7),
                280..=285 => (0xC0 + (symbol - 280), 8),
                _ => (0, 0), // Unused symbols
            };
            if len > 0 {
                let reversed_code = deflate_compress::reverse_bits(code, len);
                litlen_lookup.insert(
                    reversed_code,
                    HuffmanCode {
                        symbol,
                        length: len,
                    },
                );
            }
        }

        // Distance codes
        for symbol in 0..=31u16 {
            let code = symbol;
            let len = 5;
            let reversed_code = deflate_compress::reverse_bits(code, len);
            dist_lookup.insert(
                reversed_code,
                HuffmanCode {
                    symbol,
                    length: len,
                },
            );
        }

        (litlen_lookup, dist_lookup)
    }

    // Decodes the next literal/length symbol using bit-by-bit lookup.
    fn decode_literal_length(&self, reader: &mut BitReader) -> Result<u16, TransformError> {
        let mut current_bits = 0u16;
        let mut len = 0u8;
        loop {
            let bit = reader.read_bits(1)? as u16;
            current_bits |= bit << len;
            len += 1;
            if let Some(code) = self.litlen_lookup.get(&current_bits) {
                if code.length == len {
                    return Ok(code.symbol);
                }
            }
            if len > MAX_BITS_LITLEN {
                return Err(TransformError::CompressionError(format!(
                    "Invalid Huffman code found (litlen prefix: {:b}, len: {})",
                    current_bits, len
                )));
            }
        }
    }

    // Decodes the next distance symbol using bit-by-bit lookup.
    fn decode_distance(&self, reader: &mut BitReader) -> Result<u16, TransformError> {
        let mut current_bits = 0u16;
        let mut len = 0u8;
        loop {
            let bit = reader.read_bits(1)? as u16;
            current_bits |= bit << len;
            len += 1;
            if let Some(code) = self.dist_lookup.get(&current_bits) {
                if code.length == len {
                    if code.symbol <= 29 {
                        // Check valid distance symbol range
                        return Ok(code.symbol);
                    } else {
                        return Err(TransformError::CompressionError(format!(
                            "Invalid distance symbol {} decoded",
                            code.symbol
                        )));
                    }
                }
            }
            if len > MAX_BITS_DIST {
                return Err(TransformError::CompressionError(format!(
                    "Invalid fixed Huffman distance code found (prefix: {:b}, len: {})",
                    current_bits, len
                )));
            }
        }
    }
}

// Decodes raw DEFLATE data (supports BTYPE 00 and 01)
// Returns the decompressed data and the number of bytes consumed from the input.
pub(crate) fn deflate_decode_bytes(
    compressed_bytes: &[u8],
) -> Result<(Vec<u8>, usize), TransformError> {
    if compressed_bytes.is_empty() {
        return Ok((Vec::new(), 0)); // Return 0 consumed bytes
    }

    let mut reader = BitReader::new(compressed_bytes);
    let mut output: Vec<u8> = Vec::with_capacity(compressed_bytes.len() * 3);
    let fixed_decoder = FixedHuffmanDecoder::new();

    loop {
        let bfinal = reader.read_bits(1)?;
        let btype = reader.read_bits(2)?;

        match btype {
            0b00 => {
                // Handle uncompressed block
                reader.align_to_byte();
                let len = reader.read_bits(16)? as u16;
                let nlen = reader.read_bits(16)? as u16;
                if len != !nlen {
                    return Err(TransformError::CompressionError("LEN/NLEN mismatch".into()));
                }
                let len_usize = len as usize;
                // Check remaining bytes needed
                let remaining_bytes = reader.remaining_bytes();
                let bytes_needed = if reader.bit_position == 0 {
                    len_usize
                } else {
                    // If mid-byte, we need the current byte + len full bytes
                    len_usize + 1
                };
                if remaining_bytes < bytes_needed {
                    return Err(TransformError::CompressionError(
                        "Unexpected end of stream reading uncompressed data".into(),
                    ));
                }
                output.reserve(len_usize);
                for _ in 0..len_usize {
                    if reader.bit_position != 0 {
                        return Err(TransformError::CompressionError(
                            "Misaligned stream reading uncompressed data byte".into(),
                        ));
                    }
                    let byte = reader.read_bits(8)? as u8;
                    output.push(byte);
                }
            }
            0b01 => {
                // Handle fixed Huffman block
                loop {
                    let lit_len_code = fixed_decoder.decode_literal_length(&mut reader)?;
                    match lit_len_code {
                        0..=255 => {
                            output.push(lit_len_code as u8);
                        }
                        256 => {
                            break; // EOB marker
                        }
                        257..=285 => {
                            // Length/Distance pair
                            let (len_base, len_extra_bits) =
                                deflate_compress::get_length_info(lit_len_code);
                            let len_extra_val = if len_extra_bits > 0 {
                                reader.read_bits(len_extra_bits)?
                            } else {
                                0
                            };
                            let length = len_base + len_extra_val as u16;

                            let dist_code = fixed_decoder.decode_distance(&mut reader)?;
                            let (dist_base, dist_extra_bits) =
                                deflate_compress::get_distance_info(dist_code);
                            let dist_extra_val = if dist_extra_bits > 0 {
                                reader.read_bits(dist_extra_bits)?
                            } else {
                                0
                            };
                            let distance = dist_base + dist_extra_val as u16;

                            let current_len = output.len();
                            if distance as usize > current_len {
                                return Err(TransformError::CompressionError(format!(
                                    "Invalid back-reference distance {} > {}",
                                    distance, current_len
                                )));
                            }
                            let start = current_len - distance as usize;
                            output.reserve(length as usize);
                            for i in 0..length {
                                let copied_byte = output[start + i as usize];
                                output.push(copied_byte);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            0b10 => {
                // Dynamic Huffman Tables - Not Supported
                return Err(TransformError::CompressionError(
                    "Dynamic Huffman codes (BTYPE=10) are not supported".into(),
                ));
            }
            _ => {
                // Reserved BTYPE=11
                return Err(TransformError::CompressionError(
                    "Invalid or reserved block type (BTYPE=11)".into(),
                ));
            }
        }

        if bfinal == 1 {
            break;
        }
    }

    let consumed_bytes = if reader.bit_position > 0 {
        reader.byte_index + 1 // Consumed the partial byte as well
    } else {
        reader.byte_index
    };

    Ok((output, consumed_bytes)) // Return output and consumed bytes
}

impl Transform for DeflateDecompress {
    fn name(&self) -> &'static str {
        "DEFLATE Decompress"
    }

    fn id(&self) -> &'static str {
        "deflatedecompress"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Compression
    }

    fn description(&self) -> &'static str {
        "Decompresses DEFLATE input (RFC 1951). Expects Base64 input."
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let compressed_bytes = base64_decode::base64_decode(input).map_err(|e| {
            TransformError::InvalidArgument(format!("Invalid Base64 input: {}", e).into())
        })?;
        // Call modified function, ignore consumed bytes count here
        let (output, _consumed_bytes) = deflate_decode_bytes(&compressed_bytes)?;
        String::from_utf8(output).map_err(|_| TransformError::Utf8Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformers::base64_encode;

    #[test]
    fn test_decompress_uncompressed_block() {
        let transformer = DeflateDecompress;
        let input_str = "test";
        // Manually construct DEFLATE stream: BFINAL=1, BTYPE=00, LEN=4, NLEN=!4, DATA="test"
        let compressed_bytes = vec![0x01, 0x04, 0x00, 0xFB, 0xFF, 0x74, 0x65, 0x73, 0x74];
        let base64_input = base64_encode::base64_encode(&compressed_bytes);

        match transformer.transform(&base64_input) {
            Ok(decompressed) => {
                assert_eq!(decompressed, input_str);
            }
            Err(e) => {
                panic!("Decompression failed for uncompressed block: {:?}", e);
            }
        }
    }

    #[test]
    fn test_decompress_empty() {
        let transformer = DeflateDecompress;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("AwA=").unwrap(), "");
    }

    #[test]
    fn test_decompress_fixed_simple() {
        let transformer = DeflateDecompress;
        let base64_input = "80jNycnXUSjPL8pJUQQA"; // Compressed "Hello, world!"
        let expected = "Hello, world!";
        match transformer.transform(base64_input) {
            Ok(decompressed) => {
                assert_eq!(decompressed, expected);
            }
            Err(e) => {
                panic!("Decompression failed for fixed block: {:?}", e);
            }
        }
    }
}
