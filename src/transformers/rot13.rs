use crate::{Transform, TransformError, TransformerCategory};

/// Rot13 transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rot13;

impl Transform for Rot13 {
    fn name(&self) -> &'static str {
        "Rot13"
    }

    fn id(&self) -> &'static str {
        "rot13"
    }

    fn description(&self) -> &'static str {
        "Applies the ROT13 substitution cipher to the input text."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder // Or Other, depends on classification
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((c as u8 - base + 13) % 26) + base) as char
                } else {
                    c
                }
            })
            .collect())
    }

    fn default_test_input(&self) -> &'static str {
        "The quick brown fox jumps over the lazy dog"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_transformation() {
        let transformer = Rot13;
        let input = transformer.default_test_input();
        let expected_output = "Gur dhvpx oebja sbk whzcf bire gur ynml qbt";
        assert_eq!(transformer.transform(input).unwrap(), expected_output);

        // Test that applying Rot13 twice returns the original string
        let double_transformed = transformer.transform(expected_output).unwrap();
        assert_eq!(double_transformed, input);

        // Test with numbers and symbols
        let mixed_input = "Hello 123! - World?";
        let expected_mixed_output = "Uryyb 123! - Jbeyq?";
        assert_eq!(
            transformer.transform(mixed_input).unwrap(),
            expected_mixed_output
        );
    }

    #[test]
    fn test_rot13_metadata() {
        let transformer = Rot13;
        assert_eq!(transformer.name(), "Rot13");
        assert_eq!(transformer.id(), "rot13");
        assert_eq!(transformer.category(), TransformerCategory::Encoder);
    }
}
