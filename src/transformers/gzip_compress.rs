use super::base64_encode;
use super::deflate_compress;
use crate::utils::crc32::calculate_crc32;
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

/// Default test input for Gzip Compress
pub const DEFAULT_TEST_INPUT: &str = "Hello, Gzip World!";

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
        // RFC 1952 states, if the modification time is not available, MTIME is set to zero:
        // MTIME (Modification TIME)
        // This gives the most recent modification time of the original
        // file being compressed.  The time is in Unix format, i.e.,
        // seconds since 00:00:00 GMT, Jan.  1, 1970.  (Note that this
        // may cause problems for MS-DOS and other systems that use
        // local rather than Universal time.)  If the compressed data
        // did not come from a file, MTIME is set to the time at which
        // compression started.  MTIME = 0 means no time stamp is
        // available.
        #[cfg(target_arch = "wasm32")]
        let mtime: u32 = 0;

        #[cfg(not(target_arch = "wasm32"))]
        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                TransformError::CompressionError(format!("Failed to get system time: {}", e))
            })?
            .as_secs()
            .try_into()
            .unwrap_or(0u32); // Use 0 if conversion fails (e.g., time before epoch)

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
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = DEFAULT_TEST_INPUT;
        let compressed_b64 = compressor.transform(input).unwrap();
        let decompressed = decompressor.transform(&compressed_b64).unwrap();
        assert_eq!(decompressed, input);

        // Original simple test
        let input_hw = "Hello, world!";
        let compressed_hw_b64 = compressor.transform(input_hw).unwrap();
        let decompressed_hw = decompressor.transform(&compressed_hw_b64).unwrap();
        assert_eq!(decompressed_hw, input_hw);
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
