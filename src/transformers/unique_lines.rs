use crate::{Transform, TransformError, TransformerCategory};

/// Removes duplicate lines from text, preserving the order of the first occurrence.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UniqueLines;

/// Default test input for Unique Lines
pub const DEFAULT_TEST_INPUT: &str = "apple\nbanana\napple\norange\nbanana\ngrape\napple";

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
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut seen_lines = Vec::new();
        let mut result_lines = Vec::new();

        for line in input.lines() {
            // Only add the line if it hasn't been seen before
            if !seen_lines.contains(&line) {
                seen_lines.push(line); // Add to seen list
                result_lines.push(line); // Add to result list
            }
        }

        Ok(result_lines.join("\n"))
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
        let input = DEFAULT_TEST_INPUT;
        let expected = "apple\nbanana\norange\ngrape";
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
