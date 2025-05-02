// WARNING: This uses a non-cryptographically secure pseudo-random number generator.
// It is purely for demonstration purposes within the zero-dependency constraint.
// Do NOT use these UUIDs for security-sensitive applications.

use crate::{Transform, TransformError, TransformerCategory};
use core::cell::Cell; // Using Cell for interior mutability for the PRNG state
use core::fmt::Write;

// Simple Linear Congruential Generator (LCG) state
// Parameters from POSIX `rand()` - not great, but simple and dependency-free
// We use Cell for interior mutability without needing &mut self in transform
thread_local!(static LCG_STATE: Cell<u32> = const { Cell::new(12345) });

fn lcg_rand() -> u32 {
    LCG_STATE.with(|state_cell| {
        let current_state = state_cell.get();
        // LCG formula: X_{n+1} = (a * X_n + c) mod m
        // Using m = 2^31, a = 1103515245, c = 12345 from POSIX standard
        // We compute using u64 to avoid overflow during multiplication
        let next_state = ((1103515245u64 * current_state as u64 + 12345) % 2147483648u64) as u32;
        state_cell.set(next_state);
        // Return the upper 16 bits like some `rand()` implementations do
        // to get slightly better distribution in higher bits
        // but for UUID we need 32 bits, so let's just return next_state for now.
        next_state
    })
}

// Function to generate 16 bytes of pseudo-random data
fn generate_random_bytes() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for chunk in bytes.chunks_mut(4) {
        let random_u32 = lcg_rand();
        chunk.copy_from_slice(&random_u32.to_be_bytes());
    }
    bytes
}

/// UUID Generate transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidGenerate;

impl Transform for UuidGenerate {
    fn name(&self) -> &'static str {
        "UUID Generate (v4)"
    }

    fn id(&self) -> &'static str {
        "uuid_generate"
    }

    fn description(&self) -> &'static str {
        "Generates a version 4 UUID. Input is ignored. WARNING: Uses a non-cryptographically secure PRNG."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, _input: &str) -> Result<String, TransformError> {
        // Seed the LCG minimally on first call per thread if needed,
        // using something slightly varying. Still very weak.
        // A proper seed would ideally use system time or /dev/urandom if allowed.
        LCG_STATE.with(|state_cell| {
            if state_cell.get() == 12345 {
                // Default initial value
                // Use address of input string XORed with a constant as a *very weak* seed attempt
                let seed = (_input.as_ptr() as u32) ^ 0xDEADBEEF;
                state_cell.set(seed.wrapping_add(1)); // Avoid 0 if possible
            }
        });

        let mut bytes = generate_random_bytes();

        // Set version (4) and variant (RFC 4122)
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 1 (RFC 4122)

        // Format as UUID string xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        let mut uuid_str = String::with_capacity(36);
        write!(
            &mut uuid_str,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ).map_err(|e| TransformError::InvalidArgument(format!("Failed to format UUID: {}", e).into()))?;

        Ok(uuid_str)
    }

    fn default_test_input(&self) -> &'static str {
        "" // Input is ignored, so empty string is fine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_uuid_generate_format() {
        let transformer = UuidGenerate;
        let uuid_str = transformer.transform("test").unwrap();

        // Check length
        assert_eq!(uuid_str.len(), 36);

        // Check hyphens
        assert_eq!(uuid_str.chars().nth(8), Some('-'));
        assert_eq!(uuid_str.chars().nth(13), Some('-'));
        assert_eq!(uuid_str.chars().nth(18), Some('-'));
        assert_eq!(uuid_str.chars().nth(23), Some('-'));

        // Check version (char 14 should be '4')
        assert_eq!(uuid_str.chars().nth(14), Some('4'));

        // Check variant (char 19 should be '8', '9', 'a', or 'b')
        let variant_char = uuid_str.chars().nth(19).unwrap();
        assert!(matches!(variant_char, '8' | '9' | 'a' | 'b'));

        // Check if all other chars are hex
        for (i, c) in uuid_str.chars().enumerate() {
            if ![8, 13, 18, 23].contains(&i) {
                assert!(
                    c.is_ascii_hexdigit(),
                    "Char at index {} is not hex: {}",
                    i,
                    c
                );
            }
        }
    }

    #[test]
    fn test_uuid_generate_uniqueness_basic() {
        // This test is weak due to the poor PRNG, but checks for basic differences.
        let transformer = UuidGenerate;
        let mut generated_uuids = HashSet::new();
        for i in 0..100 {
            let uuid_str = transformer.transform(&format!("seed_{}", i)).unwrap(); // Vary input slightly
            assert!(
                generated_uuids.insert(uuid_str),
                "Duplicate UUID generated (basic check)"
            );
        }
        assert_eq!(generated_uuids.len(), 100);
    }
}
