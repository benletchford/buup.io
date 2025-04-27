use crate::{Transform, TransformError, TransformerCategory};

#[derive(Debug, Clone, Copy, Default)]
pub struct Rot13;

impl Rot13 {
    fn rot13_char(c: char) -> char {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let rotated = base + (c as u8 - base + 13) % 26;
            rotated as char
        } else {
            c
        }
    }
}

impl Transform for Rot13 {
    fn name(&self) -> &'static str {
        "ROT13 Cipher"
    }

    fn id(&self) -> &'static str {
        "rot13"
    }

    fn description(&self) -> &'static str {
        "Applies the ROT13 substitution cipher to the input text."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder // Or Other, could be debated
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let transformed: String = input.chars().map(Rot13::rot13_char).collect();
        Ok(transformed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transform;

    #[test]
    fn test_rot13_transformation() {
        let transformer = Rot13;
        assert_eq!(
            transformer.transform("Hello, World!").unwrap(),
            "Uryyb, Jbeyq!"
        );
        assert_eq!(
            transformer.transform("uryyb, jbeyq!").unwrap(),
            "hello, world!"
        );
        assert_eq!(transformer.transform("123 !@#").unwrap(), "123 !@#");
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(
            transformer
                .transform("The quick brown fox jumps over the lazy dog.")
                .unwrap(),
            "Gur dhvpx oebja sbk whzcf bire gur ynml qbt."
        );
    }

    #[test]
    fn test_rot13_metadata() {
        let transformer = Rot13;
        assert_eq!(transformer.id(), "rot13");
        assert_eq!(transformer.name(), "ROT13 Cipher");
        assert_eq!(transformer.category(), TransformerCategory::Encoder);
    }
}
