use crate::{Transform, TransformError, TransformerCategory};

/// Removes duplicate lines from text, preserving the order of the first occurrence.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UniqueLines;

impl Transform for UniqueLines {
    fn name(&self) -> &'static str {
        "Unique Lines"
    }

    fn id(&self) -> &'static str {
        "uniquelines"
    }

    fn description(&self) -> &'static str {
        "Removes duplicate lines, preserving the order of first occurrence."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other // Or maybe Formatter? Other seems more general.
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut seen = std::collections::HashSet::new();
        let mut result = String::new();
        for line in input.lines() {
            if seen.insert(line) {
                result.push_str(line);
                result.push('\n');
            }
        }
        // Remove the trailing newline if the input was not empty
        if !input.is_empty() {
            result.pop();
        }
        Ok(result)
    }

    fn default_test_input(&self) -> &'static str {
        "apple\nbanana\napple\norange\nbanana"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_lines_empty() {
        let transformer = UniqueLines;
        assert_eq!(transformer.transform("").unwrap(), "");
    }

    #[test]
    fn test_unique_lines_no_duplicates() {
        let transformer = UniqueLines;
        let input = "line1\nline2\nline3";
        assert_eq!(transformer.transform(input).unwrap(), input);
    }

    #[test]
    fn test_unique_lines_with_duplicates() {
        let transformer = UniqueLines;
        let input = transformer.default_test_input();
        let expected = "apple\nbanana\norange";
        assert_eq!(transformer.transform(input).unwrap(), expected);

        let input_orig = "line1\nline2\nline1\nline3\nline2";
        let expected_orig = "line1\nline2\nline3";
        assert_eq!(transformer.transform(input_orig).unwrap(), expected_orig);
    }

    #[test]
    fn test_unique_lines_trailing_newline() {
        let transformer = UniqueLines;
        let input = "line1\nline2\nline1\n";
        let expected = "line1\nline2";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_unique_lines_only_duplicates() {
        let transformer = UniqueLines;
        let input = "dup\ndup\ndup";
        let expected = "dup";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_unique_lines_blank_lines() {
        let transformer = UniqueLines;
        let input = "line1\n\nline2\n\nline1";
        let expected = "line1\n\nline2"; // Blank lines are treated as distinct lines
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
