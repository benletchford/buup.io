use crate::{Transform, TransformError, TransformerCategory};

/// Removes leading line numbers (and optional whitespace) from each line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineNumberRemover;

/// Default test input for Line Number Remover
pub const DEFAULT_TEST_INPUT: &str = "1. First line\n2. Second line\n3. Third line";

impl Transform for LineNumberRemover {
    fn name(&self) -> &'static str {
        "Line Number Remover"
    }

    fn id(&self) -> &'static str {
        "linenumberremover"
    }

    fn description(&self) -> &'static str {
        "Removes line numbers (and optional delimiters) from the beginning of each line."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn default_test_input(&self) -> &'static str {
        ""
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut output = String::new();
        for (i, line) in input.lines().enumerate() {
            // Find the first non-digit, non-whitespace character
            let trimmed_line = line.trim_start();
            let first_char_idx = trimmed_line
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(trimmed_line.len());

            // Check if the characters before it are all digits
            if trimmed_line[..first_char_idx]
                .chars()
                .all(|c| c.is_ascii_digit())
            {
                // Skip the number and the following whitespace/punctuation
                let content_start_idx = trimmed_line[first_char_idx..]
                    .find(|c: char| !c.is_whitespace() && !matches!(c, '.' | ':' | '-' | ')'))
                    .map(|idx| first_char_idx + idx)
                    .unwrap_or(trimmed_line.len()); // If only number/whitespace/punct, result is empty line
                output.push_str(&trimmed_line[content_start_idx..]);
            } else {
                // Line doesn't start with a number, keep it as is (minus original leading whitespace)
                output.push_str(line.trim_start()); // Keep the original line if no number prefix
            }

            // Add newline back unless it's the last line and the input didn't end with a newline
            if i < input.lines().count() - 1 || input.ends_with('\n') {
                output.push('\n');
            }
        }
        // Handle case where input is empty
        if input.is_empty() {
            return Ok("".to_string());
        }
        // Handle case where input contains only newlines
        if output.is_empty() && input.chars().all(|c| c == '\n') {
            return Ok(input.to_string()); // Return the original newlines
        } else if !input.ends_with('\n') && output.ends_with('\n') {
            // If the original didn't end with newline but we added one, remove it.
            output.pop();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_number_remover() {
        let transformer = LineNumberRemover;
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "First line\nSecond line\nThird line"
        );
        assert_eq!(
            transformer.transform("1 Hello\n2 World").unwrap(),
            "Hello\nWorld"
        );
        assert_eq!(
            transformer
                .transform("1. First line\n2. Second line\n")
                .unwrap(),
            "First line\nSecond line\n"
        );
        assert_eq!(
            transformer.transform("3:\tThird line").unwrap(),
            "Third line"
        );
        assert_eq!(
            transformer.transform("No leading number").unwrap(),
            "No leading number"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("1 \n2 \n").unwrap(), "\n\n"); // Lines with only numbers
        assert_eq!(
            transformer.transform("1 Line1\n\n3 Line3").unwrap(),
            "Line1\n\nLine3"
        ); // Skips empty line
        assert_eq!(transformer.transform("  4) Item 4").unwrap(), "Item 4"); // Leading whitespace and parenthesis
        assert_eq!(transformer.transform("5.").unwrap(), ""); // Only number and dot
        assert_eq!(
            transformer
                .transform("Line without number\n6 Line with number")
                .unwrap(),
            "Line without number\nLine with number"
        );
        assert_eq!(transformer.transform("10- Item ten").unwrap(), "Item ten");
    }
}
