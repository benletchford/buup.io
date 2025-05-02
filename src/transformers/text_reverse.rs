use crate::{Transform, TransformError, TransformerCategory};

/// Text Reverse transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextReverse;

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

    fn default_test_input(&self) -> &'static str {
        "Hello, World!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_reverse() {
        let transformer = TextReverse;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "!dlroW ,olleH"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("a").unwrap(), "a");
        assert_eq!(transformer.transform("ab").unwrap(), "ba");
    }
}
