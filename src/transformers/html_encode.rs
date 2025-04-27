use crate::{Transform, TransformError, TransformerCategory};

/// HTML encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtmlEncode;

impl Transform for HtmlEncode {
    fn name(&self) -> &'static str {
        "HTML Encode"
    }

    fn id(&self) -> &'static str {
        "htmlencode"
    }

    fn description(&self) -> &'static str {
        "Encodes special characters to HTML entities"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut result = String::with_capacity(input.len() * 2);

        for c in input.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#39;"),
                '/' => result.push_str("&#47;"),
                '`' => result.push_str("&#96;"),
                '=' => result.push_str("&#61;"),
                _ => result.push(c),
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_encode() {
        let encoder = HtmlEncode;

        // Basic test with various special characters - note that '/' is encoded to '&#47;'
        assert_eq!(
            encoder
                .transform("<script>alert(\"XSS attack\");</script>")
                .unwrap(),
            "&lt;script&gt;alert(&quot;XSS attack&quot;);&lt;&#47;script&gt;"
        );

        // Test with various special characters
        assert_eq!(
            encoder.transform("a < b && c > d").unwrap(),
            "a &lt; b &amp;&amp; c &gt; d"
        );

        // Test with single quotes and other characters
        assert_eq!(
            encoder
                .transform("Don't use `eval(input)` or query='unsafe'")
                .unwrap(),
            "Don&#39;t use &#96;eval(input)&#96; or query&#61;&#39;unsafe&#39;"
        );

        // Test with no special characters
        assert_eq!(
            encoder
                .transform("Normal text with no special chars")
                .unwrap(),
            "Normal text with no special chars"
        );

        // Test with empty input
        assert_eq!(encoder.transform("").unwrap(), "");
    }
}
