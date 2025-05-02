use crate::{Transform, TransformError, TransformerCategory};

/// Adds line numbers to the beginning of each line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineNumberAdder;

/// Default test input for Line Number Adder
pub const DEFAULT_TEST_INPUT: &str = "First line\nSecond line\nThird line";

impl Transform for LineNumberAdder {
    fn name(&self) -> &'static str {
        "Line Number Adder"
    }

    fn id(&self) -> &'static str {
        "linenumberadder"
    }

    fn description(&self) -> &'static str {
        "Adds line numbers (1-based) to the beginning of each line."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut output = String::new();
        for (i, line) in input.lines().enumerate() {
            output.push_str(&format!("{} {}", i + 1, line));
            // Add newline back unless it's the last line and the input didn't end with a newline
            if i < input.lines().count() - 1 || input.ends_with('\n') {
                output.push('\n');
            }
        }
        // Handle case where input is empty or only contains newlines
        if input.is_empty() {
            return Ok("".to_string());
        } else if output.is_empty() && input.contains('\n') {
            // Input contains only newlines
            for (i, _) in input.lines().enumerate() {
                output.push_str(&format!("{} \n", i + 1));
            }
            // If the original input ended with a newline, the last added newline is correct.
            // If it didn't, we need to remove the last added newline.
            if !input.ends_with('\n') {
                output.pop(); // Remove the trailing newline
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_number_adder() {
        let transformer = LineNumberAdder;
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "1 First line\n2 Second line\n3 Third line"
        );
        assert_eq!(
            transformer.transform("Hello\nWorld").unwrap(),
            "1 Hello\n2 World"
        );
        assert_eq!(
            transformer.transform("First line\nSecond line\n").unwrap(),
            "1 First line\n2 Second line\n"
        );
        assert_eq!(
            transformer.transform("Single line").unwrap(),
            "1 Single line"
        );
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("\n").unwrap(), "1 \n"); // Single newline
        assert_eq!(transformer.transform("\n\n").unwrap(), "1 \n2 \n"); // Multiple newlines
        assert_eq!(
            transformer.transform("Line1\n\nLine3").unwrap(),
            "1 Line1\n2 \n3 Line3"
        ); // Empty line
    }
}
