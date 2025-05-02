use crate::{Transform, TransformError, TransformerCategory};

/// Removes all whitespace characters from the input string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhitespaceRemover;

impl Transform for WhitespaceRemover {
    fn name(&self) -> &'static str {
        "Whitespace Remover"
    }

    fn id(&self) -> &'static str {
        "whitespaceremover"
    }

    fn description(&self) -> &'static str {
        "Removes all whitespace (spaces, tabs, newlines) from the input text."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other // Or perhaps Formatter? Let's stick with Other for now.
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input.chars().filter(|c| !c.is_whitespace()).collect())
    }

    fn default_test_input(&self) -> &'static str {
        "  Remove \t all \n whitespace  "
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_remover() {
        let transformer = WhitespaceRemover;
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            "Removeallwhitespace"
        );
        assert_eq!(
            transformer.transform("Hello\n World\t!").unwrap(),
            "HelloWorld!"
        );
        assert_eq!(transformer.transform("   ").unwrap(), "");
        assert_eq!(
            transformer.transform("NoWhitespace").unwrap(),
            "NoWhitespace"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
