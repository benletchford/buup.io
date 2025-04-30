const CRC32_POLYNOMIAL: u32 = 0xEDB88320; // Standard CRC32 polynomial (reversed)

static CRC32_TABLE: [u32; 256] = generate_crc32_table(); // Use static array + const fn

// Function to generate the CRC32 lookup table
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

/// Calculate CRC32 checksum for the given byte slice using the standard algorithm.
pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF; // Start with all bits set

    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }

    !crc // Final inversion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        assert_eq!(calculate_crc32(b""), 0x00000000);
    }

    #[test]
    fn test_crc32_known_values() {
        // Known values verified against Python's zlib.crc32 implementation
        assert_eq!(
            calculate_crc32(b"The quick brown fox jumps over the lazy dog"),
            0x414FA339
        );
        assert_eq!(calculate_crc32(b"hello"), 0x3610A686);
        assert_eq!(calculate_crc32(b"123456789"), 0xCBF43926);
        assert_eq!(calculate_crc32(b"Valid data"), 0x5BE1F96B);
    }
}
