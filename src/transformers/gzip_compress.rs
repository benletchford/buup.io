use super::base64_encode;
use super::deflate_compress;
#[allow(unused_imports)] // Allowed because it's used in cfg(test)
use crate::transformers::gzip_decompress::GzipDecompress;
use crate::{Transform, TransformError, TransformerCategory};
use std::time::{SystemTime, UNIX_EPOCH};

const ID1: u8 = 0x1f;
const ID2: u8 = 0x8b;
const CM_DEFLATE: u8 = 8;
const OS_UNKNOWN: u8 = 255;

/// Compresses input using the Gzip algorithm (RFC 1952).
/// Wraps DEFLATE-compressed data with a Gzip header and footer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GzipCompress;

impl Transform for GzipCompress {
    fn name(&self) -> &'static str {
        "Gzip Compress"
    }

    fn id(&self) -> &'static str {
        "gzipcompress"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Compression
    }

    fn description(&self) -> &'static str {
        "Compresses input using Gzip (RFC 1952) and encodes the output as Base64."
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input_bytes = input.as_bytes();

        // Compress the data using the core DEFLATE logic
        let deflated_data = deflate_compress::deflate_bytes(input_bytes)
            .map_err(|e| TransformError::CompressionError(format!("DEFLATE failed: {}", e)))?;

        let crc32_checksum = calculate_crc32(input_bytes);

        let isize: u32 = input_bytes.len().try_into().map_err(|_| {
            TransformError::CompressionError("Input too large for ISIZE (max 2^32 - 1)".into())
        })?;

        // Get current timestamp (seconds since epoch) for MTIME
        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                TransformError::CompressionError(format!("Failed to get timestamp: {}", e))
            })?
            .as_secs()
            .try_into()
            .unwrap_or(0u32); // Use 0 if conversion fails or time is before epoch

        let mut output = Vec::with_capacity(10 + deflated_data.len() + 8);

        // Write Gzip header
        output.push(ID1);
        output.push(ID2);
        output.push(CM_DEFLATE);
        output.push(0); // FLG (FTEXT=0, FHCRC=0, FEXTRA=0, FNAME=0, FCOMMENT=0)
        output.extend_from_slice(&mtime.to_le_bytes());
        output.push(0); // XFL (deflate flags, 0 for this strategy)
        output.push(OS_UNKNOWN);

        // Append compressed data
        output.extend_from_slice(&deflated_data);

        // Append Gzip footer
        output.extend_from_slice(&crc32_checksum.to_le_bytes());
        output.extend_from_slice(&isize.to_le_bytes());

        Ok(base64_encode::base64_encode(&output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformers::gzip_decompress::GzipDecompress;
    use crate::Transform;

    #[test]
    fn test_gzip_empty() {
        let transformer = GzipCompress;
        let result_base64 = transformer.transform("").unwrap();

        let decompressor = GzipDecompress;
        let decompressed_result = decompressor.transform(&result_base64);
        assert!(
            decompressed_result.is_ok(),
            "Decompression failed: {:?}",
            decompressed_result.err()
        );
        assert_eq!(decompressed_result.unwrap(), "");
    }

    #[test]
    fn test_gzip_simple() {
        let transformer = GzipCompress;
        let input = "Hello, world!";
        let result_base64 = transformer.transform(input).unwrap();

        let decompressor = GzipDecompress;
        let decompressed_result = decompressor.transform(&result_base64);
        assert!(
            decompressed_result.is_ok(),
            "Decompression failed: {:?}",
            decompressed_result.err()
        );
        assert_eq!(decompressed_result.unwrap(), input);
    }

    #[test]
    fn test_gzip_repeated() {
        let transformer = GzipCompress;
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 50 'a's
        let result_base64 = transformer.transform(input).unwrap();

        let decompressor = GzipDecompress;
        let decompressed_result = decompressor.transform(&result_base64);
        assert!(
            decompressed_result.is_ok(),
            "Decompression failed: {:?}",
            decompressed_result.err()
        );
        assert_eq!(decompressed_result.unwrap(), input);
    }
}

// --- CRC32 Implementation (without external dependencies) ---

const CRC32_POLYNOMIAL: u32 = 0xEDB88320; // Standard CRC32 polynomial (reversed)

static CRC32_TABLE: [u32; 256] = generate_crc32_table();

const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32; // Start with inverted value (0xFFFFFFFF)
    for &byte in data {
        let index = (crc ^ (byte as u32)) & 0xFF;
        crc = CRC32_TABLE[index as usize] ^ (crc >> 8);
    }
    !crc // Final inversion
}

#[cfg(test)]
mod crc_tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        assert_eq!(calculate_crc32(b""), 0x00000000); // CRC of empty string is 0 before inversion
    }

    #[test]
    fn test_crc32_known_values() {
        // Known values from various sources (e.g., online calculators)
        assert_eq!(
            calculate_crc32(b"The quick brown fox jumps over the lazy dog"),
            0x414FA339
        );
        assert_eq!(calculate_crc32(b"hello"), 0x3610A686);
        assert_eq!(calculate_crc32(b"123456789"), 0xCBF43926);
    }
}
