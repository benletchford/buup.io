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
        "Encodes special HTML characters into their entity representation (e.g., < to &lt;)."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn default_test_input(&self) -> &'static str {
        "<p>Hello & Welcome!</p>"
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
        assert_eq!(
            encoder.transform(encoder.default_test_input()).unwrap(),
            "&lt;p&gt;Hello &amp; Welcome!&lt;&#47;p&gt;"
        );
        assert_eq!(
            encoder.transform("No special chars").unwrap(),
            "No special chars"
        );
        assert_eq!(encoder.transform("").unwrap(), "");
    }
}
