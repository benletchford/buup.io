use crate::{Transform, TransformError, TransformerCategory};

/// HTML decode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtmlDecode;

impl Transform for HtmlDecode {
    fn name(&self) -> &'static str {
        "HTML Decode"
    }

    fn id(&self) -> &'static str {
        "htmldecode"
    }

    fn description(&self) -> &'static str {
        "Decodes HTML entities back to special characters"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        // Initial capacity is input length (a reasonable guess, might be smaller after decoding)
        let mut result = String::with_capacity(input.len());

        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '&' {
                let mut entity = String::with_capacity(10); // Typical entity length is small
                entity.push(c);

                // Collect characters until ';' or max entity length (safety)
                let mut entity_length = 1; // Already pushed '&'
                const MAX_ENTITY_LENGTH: usize = 12; // Practical limit for an HTML entity

                while let Some(&next_char) = chars.peek() {
                    if next_char == ';' || entity_length >= MAX_ENTITY_LENGTH {
                        entity.push(next_char);
                        chars.next(); // Consume the character
                        break;
                    }
                    entity.push(next_char);
                    chars.next(); // Consume the character
                    entity_length += 1;
                }

                // Attempt to decode the entity
                if let Some(decoded) = decode_html_entity(&entity) {
                    result.push(decoded);
                } else {
                    // If we can't decode, pass through the original entity
                    result.push_str(&entity);
                }
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }
}

// Decodes a single HTML entity to a character
fn decode_html_entity(entity: &str) -> Option<char> {
    match entity {
        "&amp;" => Some('&'),
        "&lt;" => Some('<'),
        "&gt;" => Some('>'),
        "&quot;" => Some('"'),
        "&#39;" => Some('\''),
        "&#47;" => Some('/'),
        "&#96;" => Some('`'),
        "&#61;" => Some('='),
        // Add support for numeric entities
        _ if entity.starts_with("&#x") && entity.ends_with(';') => {
            // Handle hexadecimal numeric entity (e.g., &#x20AC;)
            let hex_str = &entity[3..entity.len() - 1];
            u32::from_str_radix(hex_str, 16)
                .ok()
                .and_then(std::char::from_u32)
        }
        _ if entity.starts_with("&#") && entity.ends_with(';') => {
            // Handle decimal numeric entity (e.g., &#8364;)
            let num_str = &entity[2..entity.len() - 1];
            num_str.parse::<u32>().ok().and_then(std::char::from_u32)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_decode() {
        let decoder = HtmlDecode;

        // Basic test with various entities
        assert_eq!(
            decoder
                .transform("&lt;script&gt;alert(&quot;XSS attack&quot;);&lt;/script&gt;")
                .unwrap(),
            "<script>alert(\"XSS attack\");</script>"
        );

        // Test with various special characters
        assert_eq!(
            decoder.transform("a &lt; b &amp;&amp; c &gt; d").unwrap(),
            "a < b && c > d"
        );

        // Test with single quotes and other characters
        assert_eq!(
            decoder
                .transform("Don&#39;t use &#96;eval(input)&#96; or query&#61;&#39;unsafe&#39;")
                .unwrap(),
            "Don't use `eval(input)` or query='unsafe'"
        );

        // Test with numeric entities
        assert_eq!(
            decoder
                .transform("Euro symbol: &#8364; or &#x20AC;")
                .unwrap(),
            "Euro symbol: € or €"
        );

        // Test with no entities
        assert_eq!(
            decoder.transform("Normal text with no entities").unwrap(),
            "Normal text with no entities"
        );

        // Test with empty input
        assert_eq!(decoder.transform("").unwrap(), "");

        // Test with incomplete entities (should remain as is)
        assert_eq!(
            decoder.transform("This is an &incomplete entity").unwrap(),
            "This is an &incomplete entity"
        );

        // Test with invalid entities (should remain as is)
        assert_eq!(
            decoder
                .transform("This is &invalid; and &#invalid;")
                .unwrap(),
            "This is &invalid; and &#invalid;"
        );
    }
}
