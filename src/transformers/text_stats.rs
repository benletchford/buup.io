use crate::{Transform, TransformError, TransformerCategory};

/// Text Stats transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStats;

impl Transform for TextStats {
    fn name(&self) -> &'static str {
        "Text Stats"
    }

    fn id(&self) -> &'static str {
        "text_stats"
    }

    fn description(&self) -> &'static str {
        "Calculates basic text statistics (lines, words, chars, sentences)"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let lines = input.lines().count();
        let words = input.split_whitespace().count();
        let chars = input.chars().count();

        // Only count sentences if input is not empty.
        let sentences = if input.is_empty() {
            0
        } else {
            input
                .chars()
                .filter(|&c| c == '.' || c == '!' || c == '?')
                .count()
                .max(1) // Assume at least one sentence if there's non-empty text
        };

        Ok(format!(
            "Lines: {}\nWords: {}\nCharacters: {}\nSentences: {}",
            lines, words, chars, sentences
        ))
    }

    fn default_test_input(&self) -> &'static str {
        "Buup is great. Buup is fast! Is buup easy? Yes."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_stat(output: &str, label: &str) -> usize {
        output
            .lines()
            .find(|line| line.starts_with(label))
            .map(|line| {
                line.split(':')
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse()
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }

    #[test]
    fn test_text_stats_empty() {
        let transformer = TextStats;
        let result = transformer.transform("").unwrap();
        assert_eq!(get_stat(&result, "Characters"), 0);
        assert_eq!(get_stat(&result, "Lines"), 0);
        assert_eq!(get_stat(&result, "Words"), 0);
        assert_eq!(get_stat(&result, "Sentences"), 0);
    }

    #[test]
    fn test_text_stats_simple() {
        let transformer = TextStats;
        let result = transformer.transform("Hello world.").unwrap();
        assert_eq!(get_stat(&result, "Characters"), 12);
        assert_eq!(get_stat(&result, "Lines"), 1);
        assert_eq!(get_stat(&result, "Words"), 2);
        assert_eq!(get_stat(&result, "Sentences"), 1);
    }

    #[test]
    fn test_text_stats_multiline() {
        let transformer = TextStats;
        let input = transformer.default_test_input();
        let result = transformer.transform(input).unwrap();
        assert_eq!(get_stat(&result, "Characters"), 47);
        assert_eq!(get_stat(&result, "Lines"), 1);
        assert_eq!(get_stat(&result, "Words"), 10);
        assert_eq!(get_stat(&result, "Sentences"), 4);

        let result_orig = transformer.transform("First line.\nSecond line!").unwrap();
        assert_eq!(get_stat(&result_orig, "Characters"), 24);
        assert_eq!(get_stat(&result_orig, "Lines"), 2);
        assert_eq!(get_stat(&result_orig, "Words"), 4);
        assert_eq!(get_stat(&result_orig, "Sentences"), 2);
    }

    #[test]
    fn test_text_stats_multiple_sentences() {
        let transformer = TextStats;
        let result = transformer
            .transform("Sentence one. Sentence two? Sentence three!")
            .unwrap();
        assert_eq!(get_stat(&result, "Characters"), 43);
        assert_eq!(get_stat(&result, "Lines"), 1);
        assert_eq!(get_stat(&result, "Words"), 6);
        assert_eq!(get_stat(&result, "Sentences"), 3);
    }

    #[test]
    fn test_text_stats_no_terminator() {
        let transformer = TextStats;
        let result = transformer.transform("Just some words").unwrap();
        assert_eq!(get_stat(&result, "Characters"), 15);
        assert_eq!(get_stat(&result, "Lines"), 1);
        assert_eq!(get_stat(&result, "Words"), 3);
        assert_eq!(get_stat(&result, "Sentences"), 1);
    }

    #[test]
    fn test_text_stats_whitespace() {
        let transformer = TextStats;
        let result = transformer
            .transform("  Lots\nof\n  whitespace.  ")
            .unwrap();
        assert_eq!(get_stat(&result, "Characters"), 25);
        assert_eq!(get_stat(&result, "Lines"), 3);
        assert_eq!(get_stat(&result, "Words"), 3);
        assert_eq!(get_stat(&result, "Sentences"), 1);
    }
}
