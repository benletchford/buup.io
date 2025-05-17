use crate::{Transform, TransformError, TransformerCategory};

/// A transformer that compresses XML by removing unnecessary whitespace
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XmlMinifier;

impl Transform for XmlMinifier {
    fn name(&self) -> &'static str {
        "XML Minifier"
    }

    fn id(&self) -> &'static str {
        "xmlminifier"
    }

    fn description(&self) -> &'static str {
        "Compress XML by removing unnecessary whitespace"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        // Simple tokenizer to minify XML
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        let mut in_tag = false;
        let mut in_string = false;
        let mut string_char = '"';
        let mut in_comment = false;
        let mut comment_end_check = 0;
        let mut in_cdata = false;
        let mut cdata_end_check = 0;
        let mut in_processing = false;
        let mut in_content = false;
        let mut prev_char_was_space = false;

        while let Some(c) = chars.next() {
            // Handle comments
            if in_comment {
                if c == '-' && comment_end_check == 0 {
                    comment_end_check = 1;
                } else if c == '-' && comment_end_check == 1 {
                    comment_end_check = 2;
                } else if c == '>' && comment_end_check == 2 {
                    // End of comment, don't include comments in the minified output
                    in_comment = false;
                    comment_end_check = 0;
                }
                continue;
            }

            // Handle CDATA
            if in_cdata {
                if c == ']' && cdata_end_check == 0 {
                    cdata_end_check = 1;
                } else if c == ']' && cdata_end_check == 1 {
                    cdata_end_check = 2;
                } else if c == '>' && cdata_end_check == 2 {
                    // End of CDATA
                    result.push(']');
                    result.push(']');
                    result.push('>');
                    in_cdata = false;
                    cdata_end_check = 0;
                    in_content = false;
                } else {
                    if cdata_end_check == 1 {
                        result.push(']');
                        cdata_end_check = 0;
                    } else if cdata_end_check == 2 {
                        result.push(']');
                        result.push(']');
                        cdata_end_check = 0;
                    }
                    result.push(c);
                }
                continue;
            }

            // Handle whitespace
            if c.is_whitespace() {
                if in_string {
                    // Preserve whitespace in strings
                    result.push(c);
                } else if in_content && !prev_char_was_space {
                    // Collapse multiple whitespace in content to a single space
                    result.push(' ');
                    prev_char_was_space = true;
                }
                continue;
            }

            prev_char_was_space = false;

            // Handle string literals inside tags
            if in_tag && (c == '"' || c == '\'') {
                if !in_string {
                    in_string = true;
                    string_char = c;
                } else if c == string_char {
                    in_string = false;
                }
                result.push(c);
                continue;
            }

            // Check for comment start
            if in_tag && c == '-' && chars.peek() == Some(&'-') && result.ends_with('<') {
                chars.next(); // consume second '-'
                result.push('-');
                result.push('-');
                in_comment = true;
                in_tag = false;
                continue;
            }

            // Check for CDATA start
            if c == '[' && result.ends_with("![CDATA") {
                in_cdata = true;
                continue;
            }

            // Check for processing instruction
            if c == '?' && result.ends_with('<') {
                in_processing = true;
                result.push(c);
                continue;
            }

            // Check for end of processing instruction
            if in_processing && c == '>' && result.ends_with('?') {
                in_processing = false;
                in_tag = false;
                result.push(c);
                continue;
            }

            // Tag start
            if c == '<' {
                in_tag = true;
                in_content = false;
                result.push(c);
                continue;
            }

            // Tag end
            if c == '>' && in_tag && !in_string && !in_processing {
                in_tag = false;
                in_content = true;
                result.push(c);
                continue;
            }

            // Normal character
            result.push(c);
        }

        Ok(result)
    }

    fn default_test_input(&self) -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <element attribute="value">
        text
    </element>
    <empty-element/>
    <nested>
        <child>content</child>
    </nested>
</root>"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_minifier() {
        let transformer = XmlMinifier;
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <element attribute="value">
        text
    </element>
    <empty-element/>
</root>"#;

        let result = transformer.transform(input).unwrap();

        // Check minification
        assert!(!result.contains("\n"));
        assert!(!result.contains("    "));

        // Make sure content is preserved
        assert!(result.contains("text"));
        assert!(result.contains("attribute=\"value\""));

        // Make sure XML structure is preserved
        assert!(result.contains("<?xml"));
        assert!(result.contains("version=\"1.0\""));
        assert!(result.contains("encoding=\"UTF-8\""));
        assert!(result.contains("<root>"));
        assert!(result.contains("</root>"));
        assert!(result.contains("<empty-element/>"));

        // Test empty input
        assert_eq!(transformer.transform("").unwrap(), "");
    }
}
