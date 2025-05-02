use crate::{Transform, TransformError, TransformerCategory};

/// JSON Minifier transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonMinifier;

/// Default test input for JSON Minifier
pub const DEFAULT_TEST_INPUT: &str = r#"{
  "name": "buup",
  "version": 0.1,
  "features": [
    "cli",
    "web",
    "lib"
  ],
  "active": true,
  "config": null
}"#;

impl Transform for JsonMinifier {
    fn name(&self) -> &'static str {
        "JSON Minifier"
    }

    fn id(&self) -> &'static str {
        "jsonminifier"
    }

    fn description(&self) -> &'static str {
        "Minifies JSON by removing whitespace"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Skip empty input
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        minify_json(input)
    }
}

/// Minify JSON by removing all unnecessary whitespace
fn minify_json(input: &str) -> Result<String, TransformError> {
    let mut result = String::with_capacity(input.len());
    let chars = input.chars();
    let mut in_string = false;
    let mut escaped = false;

    for c in chars {
        if in_string {
            // Always include characters within strings
            result.push(c);

            if escaped {
                // Previous character was escape - this character is always included
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
        } else {
            match c {
                // Start of a string - always include the quote and set flag
                '"' => {
                    result.push(c);
                    in_string = true;
                }
                // Structural characters - always include
                '{' | '}' | '[' | ']' | ':' | ',' => {
                    result.push(c);
                }
                // Whitespace outside a string - skip
                ' ' | '\t' | '\n' | '\r' => {
                    // Skip whitespace
                }
                // Numbers, booleans, null - include
                '0'..='9' | '-' | '+' | '.' | 'e' | 'E' | 't' | 'f' | 'n' => {
                    result.push(c);
                }
                // Other characters - could be part of literals (true, false, null)
                'a'..='z' | 'A'..='Z' => {
                    result.push(c);
                }
                // Invalid characters
                _ => {
                    return Err(TransformError::JsonParseError(format!(
                        "Invalid character: '{}'",
                        c
                    )))
                }
            }
        }
    }

    // Ensure we're not in the middle of a string
    if in_string {
        return Err(TransformError::JsonParseError("Unterminated string".into()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_minifier_empty() {
        let transformer = JsonMinifier;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("  ").unwrap(), "");
    }

    #[test]
    fn test_json_minifier_simple() {
        let transformer = JsonMinifier;
        let input = DEFAULT_TEST_INPUT;
        let expected = "{\"name\":\"buup\",\"version\":0.1,\"features\":[\"cli\",\"web\",\"lib\"],\"active\":true,\"config\":null}";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_minifier_nested() {
        let transformer = JsonMinifier;
        let input = r#"{
  "person": {
    "name": "John",
    "age": 30,
    "address": {
      "city": "New York",
      "zip": "10001"
    }
  },
  "active": true
}"#;
        let expected = r#"{"person":{"name":"John","age":30,"address":{"city":"New York","zip":"10001"}},"active":true}"#;
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_minifier_array() {
        let transformer = JsonMinifier;
        let input = r#"[
  1,
  2,
  3,
  {
    "name": "John"
  }
]"#;
        let expected = r#"[1,2,3,{"name":"John"}]"#;
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_minifier_preserve_strings() {
        let transformer = JsonMinifier;
        let input = r#"{
  "text": "This has   spaces   and \n newlines \t tabs"
}"#;
        let expected = r#"{"text":"This has   spaces   and \n newlines \t tabs"}"#;
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
