use super::base64_decode;
use super::deflate_decompress;
use crate::utils::crc32::calculate_crc32;
use crate::{Transform, TransformError, TransformerCategory};

// Constants from Gzip spec (RFC 1952)
const ID1: u8 = 0x1f;
const ID2: u8 = 0x8b;
const CM_DEFLATE: u8 = 8;
// FLG bits
#[allow(dead_code)] // May be used later if header parsing is expanded
const FTEXT: u8 = 0x01;
const FHCRC: u8 = 0x02;
const FEXTRA: u8 = 0x04;
const FNAME: u8 = 0x08;
const FCOMMENT: u8 = 0x10;

/// Decompresses Gzip formatted input (RFC 1952). Expects Base64 input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GzipDecompress;

/// Default test input for Gzip Decompress (Dynamically generated in test)
pub const DEFAULT_TEST_INPUT_TEXT: &str = "Hello, Gzip World!";

impl Transform for GzipDecompress {
    fn name(&self) -> &'static str {
        "Gzip Decompress"
    }

    fn id(&self) -> &'static str {
        "gzipdecompress"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Compression
    }

    fn description(&self) -> &'static str {
        "Decompresses Gzip formatted input (RFC 1952). Expects Base64 input."
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let compressed_bytes = base64_decode::base64_decode(input).map_err(|e| {
            TransformError::InvalidArgument(format!("Invalid Base64 input: {}", e).into())
        })?;

        if compressed_bytes.len() < 18 {
            // Minimum Gzip size: 10 header + 8 footer + >=0 data
            return Err(TransformError::CompressionError(
                "Input too short to be Gzip".into(),
            ));
        }

        // --- Parse Header ---
        let mut current_pos = 0;

        // Magic number (2 bytes)
        if compressed_bytes.get(current_pos) != Some(&ID1)
            || compressed_bytes.get(current_pos + 1) != Some(&ID2)
        {
            return Err(TransformError::CompressionError(
                "Invalid Gzip magic number".into(),
            ));
        }
        current_pos += 2;

        // Compression method (1 byte)
        let cm = *compressed_bytes
            .get(current_pos)
            .ok_or_else(|| TransformError::CompressionError("Missing CM".into()))?;
        if cm != CM_DEFLATE {
            return Err(TransformError::CompressionError(format!(
                "Unsupported compression method: {}",
                cm
            )));
        }
        current_pos += 1;

        // Flags (1 byte)
        let flg = *compressed_bytes
            .get(current_pos)
            .ok_or_else(|| TransformError::CompressionError("Missing FLG".into()))?;
        current_pos += 1;

        // MTIME (4 bytes), XFL (1 byte), OS (1 byte) - total 6 bytes
        if compressed_bytes.len() < current_pos + 6 {
            return Err(TransformError::CompressionError(
                "Incomplete Gzip header (MTIME/XFL/OS)".into(),
            ));
        }
        // let mtime = u32::from_le_bytes(compressed_bytes[current_pos..current_pos+4].try_into().unwrap()); // Assign to _ as unused
        current_pos += 4; // Skip MTIME
                          // let xfl = compressed_bytes[current_pos]; // Assign to _ as unused
        current_pos += 1; // Skip XFL
                          // let os = compressed_bytes[current_pos]; // Assign to _ as unused
        current_pos += 1; // Skip OS

        // --- Optional Header Fields ---

        // FEXTRA (Variable length)
        if flg & FEXTRA != 0 {
            if compressed_bytes.len() < current_pos + 2 {
                return Err(TransformError::CompressionError(
                    "Input too short for FEXTRA length".into(),
                ));
            }
            let xlen = u16::from_le_bytes(
                compressed_bytes[current_pos..current_pos + 2]
                    .try_into()
                    .unwrap(),
            ) as usize;
            current_pos += 2;
            if compressed_bytes.len() < current_pos + xlen {
                return Err(TransformError::CompressionError(
                    "Input too short for FEXTRA data".into(),
                ));
            }
            current_pos += xlen; // Skip FEXTRA data
        }

        // FNAME (Null-terminated string)
        if flg & FNAME != 0 {
            let _start = current_pos; // Mark as unused
            while current_pos < compressed_bytes.len() && compressed_bytes[current_pos] != 0 {
                current_pos += 1;
            }
            if current_pos >= compressed_bytes.len() {
                // Need space for null terminator + footer
                return Err(TransformError::CompressionError(
                    "Unterminated FNAME field or missing footer".into(),
                ));
            }
            current_pos += 1; // Skip null terminator
        }

        // FCOMMENT (Null-terminated string)
        if flg & FCOMMENT != 0 {
            let _start = current_pos; // Mark as unused
            while current_pos < compressed_bytes.len() && compressed_bytes[current_pos] != 0 {
                current_pos += 1;
            }
            if current_pos >= compressed_bytes.len() {
                // Need space for null terminator + footer
                return Err(TransformError::CompressionError(
                    "Unterminated FCOMMENT field or missing footer".into(),
                ));
            }
            current_pos += 1; // Skip null terminator
        }

        // FHCRC (2 bytes)
        if flg & FHCRC != 0 {
            if compressed_bytes.len() < current_pos + 2 {
                return Err(TransformError::CompressionError(
                    "Input too short for FHCRC field".into(),
                ));
            }
            let header_crc16_expected = u16::from_le_bytes(
                compressed_bytes[current_pos..current_pos + 2]
                    .try_into()
                    .unwrap(),
            );
            // CRC32 calculation reused for header CRC16 check (lower 16 bits of CRC32)
            let header_crc32_actual = calculate_crc32(&compressed_bytes[0..current_pos]);
            let header_crc16_actual = (header_crc32_actual & 0xFFFF) as u16; // Check lower 16 bits
            if header_crc16_actual != header_crc16_expected {
                return Err(TransformError::CompressionError(format!(
                    "Gzip header CRC16 mismatch: expected {:04x}, got {:04x}",
                    header_crc16_expected, header_crc16_actual
                )));
            }
            current_pos += 2;
        }

        let header_len = current_pos;

        // Minimum length check
        if compressed_bytes.len() < header_len + 8 {
            return Err(TransformError::CompressionError(
                "Input too short for Gzip footer".into(),
            ));
        }

        // --- Find the end of the DEFLATE stream ---
        // Gzip always ends with a 8-byte footer: 4 bytes CRC32 + 4 bytes ISIZE
        // DEFLATE will *always* end with a '1' bit followed by a valid EOB code (usually 0)
        // We only need to process until we find a valid DEFLATE end, and then add 8 bytes for the footer

        // Create a safety limit - in case there's extra data, don't read all the way to the end
        // This allows us to handle cases where garbage data is appended to a valid Gzip stream
        let deflate_data = &compressed_bytes[header_len..];

        // Decompress and check if it succeeded
        let (decompressed_bytes, consumed_deflate_bytes) =
            deflate_decompress::deflate_decode_bytes(deflate_data).map_err(|e| {
                TransformError::CompressionError(format!("DEFLATE decompression failed: {}", e))
            })?;

        // --- Parse Footer ---
        // Since we successfully decompressed the DEFLATE stream, we need to extract the footer data
        // Gzip footer is always 8 bytes (4 for CRC32, 4 for ISIZE) after the deflate stream
        // We need to find the position right after the DEFLATE data to locate the footer

        // Since the footer is 8 bytes, ensure we have enough data
        // DEFLATE decoder should have stopped exactly at the end of the DEFLATE stream,
        // the next 8 bytes should be the footer
        let deflate_end_pos = header_len + consumed_deflate_bytes;

        if compressed_bytes.len() < deflate_end_pos + 8 {
            return Err(TransformError::CompressionError(
                "Input too short for Gzip footer after DEFLATE stream".into(),
            ));
        }

        let crc32_expected = u32::from_le_bytes(
            compressed_bytes[deflate_end_pos..deflate_end_pos + 4]
                .try_into()
                .unwrap(),
        );
        let isize_expected = u32::from_le_bytes(
            compressed_bytes[deflate_end_pos + 4..deflate_end_pos + 8]
                .try_into()
                .unwrap(),
        );

        // --- Verify Footer ---
        let crc32_actual = calculate_crc32(&decompressed_bytes);
        if crc32_actual != crc32_expected {
            return Err(TransformError::CompressionError(format!(
                "CRC32 checksum mismatch: expected {:08x}, got {:08x}",
                crc32_expected, crc32_actual
            )));
        }

        // ISIZE is the size of the original (uncompressed) input data modulo 2^32.
        let isize_actual = (decompressed_bytes.len() as u64 % (1u64 << 32)) as u32;
        if isize_actual != isize_expected {
            return Err(TransformError::CompressionError(format!(
                "ISIZE mismatch: expected {}, got {} (from decompressed length {})",
                isize_expected,
                isize_actual,
                decompressed_bytes.len()
            )));
        }

        // We assume the input was UTF-8 if FTEXT was set or by default.
        // If FTEXT is *not* set, it could be binary, but this tool focuses on text.
        String::from_utf8(decompressed_bytes).map_err(|_| TransformError::Utf8Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base64_encode;
    use crate::transformers::gzip_compress::GzipCompress;
    use crate::Transform; // Bring trait into scope

    // Helper to manually Gzip compress with fixed MTIME=0
    fn manually_gzip_compress(input: &str) -> Result<String, TransformError> {
        use super::super::base64_encode;
        use super::super::deflate_compress;
        use crate::utils::crc32::calculate_crc32;

        const ID1: u8 = 0x1f;
        const ID2: u8 = 0x8b;
        const CM_DEFLATE: u8 = 8;
        const OS_UNKNOWN: u8 = 255;

        let input_bytes = input.as_bytes();
        let deflated_data = deflate_compress::deflate_bytes(input_bytes)?;
        let crc32_checksum = calculate_crc32(input_bytes);
        let isize: u32 = input_bytes
            .len()
            .try_into()
            .map_err(|_| TransformError::CompressionError("Input too large".into()))?;
        let mtime: u32 = 0; // Fixed MTIME

        let mut output = Vec::with_capacity(10 + deflated_data.len() + 8);
        output.push(ID1);
        output.push(ID2);
        output.push(CM_DEFLATE);
        output.push(0); // FLG
        output.extend_from_slice(&mtime.to_le_bytes());
        output.push(0); // XFL
        output.push(OS_UNKNOWN);
        output.extend_from_slice(&deflated_data);
        output.extend_from_slice(&crc32_checksum.to_le_bytes());
        output.extend_from_slice(&isize.to_le_bytes());

        Ok(base64_encode::base64_encode(&output))
    }

    #[test]
    fn test_decompress_empty() {
        let transformer = GzipDecompress;
        let input = manually_gzip_compress("").unwrap();
        let result = transformer.transform(&input);
        assert!(result.is_ok(), "Decompression failed: {:?}", result.err());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_decompress_simple() {
        let decompressor = GzipDecompress;
        let compressor = GzipCompress; // Use the actual compressor

        // Test with dynamically generated default input
        let expected_output = DEFAULT_TEST_INPUT_TEXT;
        let input_b64 = compressor.transform(expected_output).unwrap(); // Generate input
        let result = decompressor.transform(&input_b64);
        assert!(result.is_ok(), "Decompression failed: {:?}", result.err());
        assert_eq!(result.unwrap(), expected_output);

        // Original simple test with manually gzipped MTIME=0 string
        let input_hw = "Hello, world!";
        let input_hw_b64 = manually_gzip_compress(input_hw).unwrap();
        let result_hw = decompressor.transform(&input_hw_b64);
        assert!(
            result_hw.is_ok(),
            "Decompression failed: {:?}",
            result_hw.err()
        );
        assert_eq!(result_hw.unwrap(), input_hw);
    }

    #[test]
    fn test_decompress_repeated() {
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 50 'a's
        let base64_input = compressor.transform(input).unwrap();
        let result = decompressor.transform(&base64_input);
        assert!(result.is_ok(), "Decompression failed: {:?}", result.err());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_decompress_longer_text() {
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = "This is a longer test sentence to check Gzip round-tripping with more data. It includes punctuation and numbers 12345.";
        let base64_input = compressor.transform(input).unwrap();
        let result = decompressor.transform(&base64_input);
        assert!(result.is_ok(), "Decompression failed: {:?}", result.err());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_invalid_magic() {
        let decompressor = GzipDecompress;
        // Corrupt magic number (first byte)
        let bad_data = vec![
            0x2f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]; // Min length spoof
        let base64_input = base64_encode::base64_encode(&bad_data);
        let result = decompressor.transform(&base64_input);
        assert!(matches!(result, Err(TransformError::CompressionError(_))));
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid Gzip magic number"));
    }

    #[test]
    fn test_unsupported_method() {
        let decompressor = GzipDecompress;
        // Invalid compression method (9 instead of 8)
        let bad_data = vec![
            0x1f, 0x8b, 9, 0, 0, 0, 0, 0, 0, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]; // Min length spoof
        let base64_input = base64_encode::base64_encode(&bad_data);
        let result = decompressor.transform(&base64_input);
        assert!(matches!(result, Err(TransformError::CompressionError(_))));
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported compression method"));
    }

    #[test]
    fn test_crc_mismatch() {
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = "Some data where CRC will be flipped";
        let base64_input = compressor.transform(input).unwrap();
        let mut compressed_bytes = base64_decode::base64_decode(&base64_input).unwrap();

        // Corrupt the CRC32 footer (bytes at len-8 to len-5)
        let len = compressed_bytes.len();
        if len >= 8 {
            compressed_bytes[len - 8] = compressed_bytes[len - 8].wrapping_add(1);
            // Flip a bit in CRC
        }

        let corrupted_base64 = base64_encode::base64_encode(&compressed_bytes);
        let result = decompressor.transform(&corrupted_base64);
        assert!(
            matches!(result, Err(TransformError::CompressionError(_))),
            "Expected CRC error, got: {:?}",
            result
        );
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("CRC32 checksum mismatch"));
    }

    #[test]
    fn test_isize_mismatch() {
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = "Some different data where ISIZE will be flipped";
        let base64_input = compressor.transform(input).unwrap();
        let mut compressed_bytes = base64_decode::base64_decode(&base64_input).unwrap();

        // Corrupt the ISIZE footer (bytes at len-4 to len-1)
        let len = compressed_bytes.len();
        if len >= 4 {
            compressed_bytes[len - 1] = compressed_bytes[len - 1].wrapping_add(1);
            // Flip a bit in ISIZE
        }

        let corrupted_base64 = base64_encode::base64_encode(&compressed_bytes);
        let result = decompressor.transform(&corrupted_base64);
        assert!(
            matches!(result, Err(TransformError::CompressionError(_))),
            "Expected ISIZE error, got: {:?}",
            result
        );
        assert!(result.unwrap_err().to_string().contains("ISIZE mismatch"));
    }

    #[test]
    fn test_input_too_short() {
        let decompressor = GzipDecompress;
        let short_data = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 255]; // Only 10 bytes
        let base64_input = base64_encode::base64_encode(&short_data);
        let result = decompressor.transform(&base64_input);
        assert!(matches!(result, Err(TransformError::CompressionError(_))));
        assert!(result.unwrap_err().to_string().contains("Input too short"));
    }

    #[test]
    fn test_data_after_footer() {
        let compressor = GzipCompress;
        let decompressor = GzipDecompress;
        let input = "Valid data";
        let base64_input = compressor.transform(input).unwrap();
        let mut compressed_bytes = base64_decode::base64_decode(&base64_input).unwrap();

        // Save the original gzip footer before appending
        let original_len = compressed_bytes.len();
        let original_crc32 = u32::from_le_bytes(
            compressed_bytes[original_len - 8..original_len - 4]
                .try_into()
                .unwrap(),
        );
        eprintln!("Original CRC32 value from footer: 0x{:08x}", original_crc32);

        // Append extra garbage data
        compressed_bytes.extend_from_slice(b"GARBAGE");

        let base64_with_garbage = base64_encode::base64_encode(&compressed_bytes);
        let result = decompressor.transform(&base64_with_garbage);

        // Should succeed and ignore the garbage
        assert!(
            result.is_ok(),
            "Decompression failed unexpectedly: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_header_fname_flag() {
        let original_data = b"test data";
        let filename = b"test.txt";
        let mut output = Vec::new();
        // Header with FNAME flag
        let header_prefix = [0x1f, 0x8b, 8, FNAME, 0, 0, 0, 0, 0, 255]; // Added FNAME flag
        output.extend_from_slice(&header_prefix);
        // Add filename and null terminator
        output.extend_from_slice(filename);
        output.push(0); // Null terminator
                        // Add deflated data for "test data"
        let comp_data = GzipCompress.transform("test data").unwrap(); // Use actual compressor to get real deflated data
        let decoded_comp = base64_decode::base64_decode(&comp_data).unwrap(); // This is Gzip compressed, header is 10 bytes
        let actual_deflated_data = &decoded_comp[10..decoded_comp.len() - 8]; // Extract deflated part (header=10, footer=8)

        output.extend_from_slice(actual_deflated_data);
        // Footer (CRC32 and ISIZE of "test data")
        let crc = calculate_crc32(original_data);
        let isize = original_data.len() as u32;
        output.extend_from_slice(&crc.to_le_bytes());
        output.extend_from_slice(&isize.to_le_bytes());

        let base64_input = base64_encode::base64_encode(&output);
        let decompressor = GzipDecompress;
        let result = decompressor.transform(&base64_input);

        assert!(
            result.is_ok(),
            "Decompression failed with FNAME flag: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), "test data");
    }

    // TODO: Add tests for FCOMMENT, FEXTRA, FHCRC later.
}
