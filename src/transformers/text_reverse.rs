use crate::{Transform, TransformError, TransformerCategory};

/// Text Reverse transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextReverse;

/// Default test input for Text Reverse
pub const DEFAULT_TEST_INPUT: &str = "Hello, World!";

impl Transform for TextReverse {
    fn name(&self) -> &'static str {
        "Text Reverse"
    }

    fn id(&self) -> &'static str {
        "textreverse"
    }

    fn description(&self) -> &'static str {
        "Reverses the input text"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input.chars().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_reverse() {
        let transformer = TextReverse;
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "!dlroW ,olleH"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("a").unwrap(), "a");
        assert_eq!(transformer.transform("ab").unwrap(), "ba");
    }
}
