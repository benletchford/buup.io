use crate::{Transform, TransformError, TransformerCategory};

/// Slugify transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slugify;

impl Transform for Slugify {
    fn name(&self) -> &'static str {
        "Slugify"
    }

    fn id(&self) -> &'static str {
        "slugify"
    }

    fn description(&self) -> &'static str {
        "Converts text into a URL-friendly slug (lowercase, dashes, removes special chars)"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut slug = String::with_capacity(input.len());
        let mut last_char_was_dash = true; // Treat beginning as if preceded by a dash

        for c in input.chars() {
            if c.is_ascii_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
                last_char_was_dash = false;
            } else if c.is_whitespace() || c == '-' || c == '_' {
                if !last_char_was_dash {
                    slug.push('-');
                    last_char_was_dash = true;
                }
            } else {
                // Ignore other characters
                // We could attempt transliteration here (e.g., 'é' to 'e')
                // but keeping it simple and dependency-free for now.
            }
        }

        // Remove trailing dash if exists and the slug is not just a dash
        if slug.ends_with('-') && slug.len() > 1 {
            slug.pop();
        }

        // Handle cases where the input consisted *only* of characters that were removed or replaced by dashes
        // which might result in an empty slug or just a dash after trimming.
        if slug.is_empty() || slug == "-" {
            return Ok(String::new()); // Or decide on alternative output like "n-a"
        }

        Ok(slug)
    }

    fn default_test_input(&self) -> &'static str {
        "This is a Test String! 123?"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_basic() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform(transformer.default_test_input()),
            Ok("this-is-a-test-string-123".to_string())
        );
        assert_eq!(
            transformer.transform("Hello World"),
            Ok("hello-world".to_string())
        );
    }

    #[test]
    fn test_slugify_punctuation() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("Test! String?"),
            Ok("test-string".to_string())
        );
    }

    #[test]
    fn test_slugify_uppercase() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("UPPERCASE"),
            Ok("uppercase".to_string())
        );
    }

    #[test]
    fn test_slugify_consecutive_spaces() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("Multiple  Spaces"),
            Ok("multiple-spaces".to_string())
        );
    }

    #[test]
    fn test_slugify_leading_trailing_spaces() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("  Leading and Trailing  "),
            Ok("leading-and-trailing".to_string())
        );
    }

    #[test]
    fn test_slugify_leading_trailing_hyphens() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("-Hyphens-"),
            Ok("hyphens".to_string())
        ); // Inner logic handles this
        assert_eq!(
            transformer.transform(" Leading-Hyphen"),
            Ok("leading-hyphen".to_string())
        );
        assert_eq!(
            transformer.transform("Trailing-Hyphen- "),
            Ok("trailing-hyphen".to_string())
        );
    }

    #[test]
    fn test_slugify_underscores() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("snake_case_string"),
            Ok("snake-case-string".to_string())
        );
    }

    #[test]
    fn test_slugify_mixed() {
        let transformer = Slugify;
        assert_eq!(
            transformer.transform("  Mixed CASE with Punctuations! and _underscores- "),
            Ok("mixed-case-with-punctuations-and-underscores".to_string())
        );
    }

    #[test]
    fn test_slugify_empty() {
        let transformer = Slugify;
        assert_eq!(transformer.transform(""), Ok("".to_string()));
    }

    #[test]
    fn test_slugify_only_special_chars() {
        let transformer = Slugify;
        assert_eq!(transformer.transform("!@#$%^"), Ok("".to_string()));
        assert_eq!(transformer.transform(" - _ - "), Ok("".to_string()));
    }

    #[test]
    fn test_slugify_non_ascii() {
        let transformer = Slugify;
        // Basic implementation ignores non-ASCII
        assert_eq!(
            transformer.transform("Héllö Wörld"),
            Ok("hll-wrld".to_string())
        );
    }
}
