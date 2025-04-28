use crate::{Transform, TransformError, TransformerCategory};

/// Sorts lines of text alphabetically.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LineSorter;

impl Transform for LineSorter {
    fn name(&self) -> &'static str {
        "Line Sorter"
    }

    fn id(&self) -> &'static str {
        "linesorter"
    }

    fn description(&self) -> &'static str {
        "Sorts lines of text alphabetically (ascending)."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other // Could also be Formatter
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut lines: Vec<&str> = input.lines().collect();

        // Perform standard lexicographical sort
        lines.sort_unstable();

        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_sorter_empty() {
        let transformer = LineSorter;
        assert_eq!(transformer.transform("").unwrap(), "");
    }

    #[test]
    fn test_line_sorter_single_line() {
        let transformer = LineSorter;
        assert_eq!(transformer.transform("hello").unwrap(), "hello");
    }

    #[test]
    fn test_line_sorter_basic_sort() {
        let transformer = LineSorter;
        let input = "c\nb\na";
        let expected = "a\nb\nc";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_line_sorter_with_duplicates() {
        let transformer = LineSorter;
        let input = "c\nb\na\nc\nb";
        let expected = "a\nb\nb\nc\nc";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_line_sorter_trailing_newline() {
        let transformer = LineSorter;
        let input = "c\nb\na\n";
        let expected = "a\nb\nc"; // Trailing newline is ignored by .lines(), preserved by join
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_line_sorter_blank_lines() {
        let transformer = LineSorter;
        let input = "c\n\na\nb";
        let expected = "\na\nb\nc"; // Blank lines are sorted too
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_line_sorter_case_sensitivity() {
        let transformer = LineSorter;
        let input = "C\nb\nA";
        let expected = "A\nC\nb"; // Uppercase comes before lowercase
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
