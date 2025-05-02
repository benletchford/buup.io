use crate::{Transform, TransformError, TransformerCategory};

/// JSON Formatter transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonFormatter;

/// Default test input for JSON Formatter
pub const DEFAULT_TEST_INPUT: &str = "{\"name\":\"buup\",\"version\":0.1,\"features\":[\"cli\",\"web\",\"lib\"],\"active\":true,\"config\":null}";

impl Transform for JsonFormatter {
    fn name(&self) -> &'static str {
        "JSON Formatter"
    }

    fn id(&self) -> &'static str {
        "jsonformatter"
    }

    fn description(&self) -> &'static str {
        "Formats JSON with proper indentation"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Skip empty input
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        // First, parse the JSON into tokens
        let tokens = tokenize_json(input)?;

        // Then format the tokens with indentation
        format_json(&tokens)
    }
}

/// Different types of JSON tokens
#[derive(Debug, PartialEq, Eq)]
enum JsonToken {
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Colon,        // :
    Comma,        // ,
    String(String),
    Number(String),
    Bool(bool),
    Null,
    Whitespace,
}

/// Tokenize JSON string into tokens
fn tokenize_json(input: &str) -> Result<Vec<JsonToken>, TransformError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut pos = 0;

    while let Some(c) = chars.next() {
        pos += 1;

        match c {
            '{' => tokens.push(JsonToken::OpenBrace),
            '}' => tokens.push(JsonToken::CloseBrace),
            '[' => tokens.push(JsonToken::OpenBracket),
            ']' => tokens.push(JsonToken::CloseBracket),
            ':' => tokens.push(JsonToken::Colon),
            ',' => tokens.push(JsonToken::Comma),
            '"' => {
                let mut string = String::new();
                let mut escaped = false;

                while let Some(ch) = chars.next() {
                    pos += 1;
                    if escaped {
                        // Handle escape sequences
                        string.push(match ch {
                            '"' | '\\' | '/' => ch,
                            'b' => '\u{0008}',
                            'f' => '\u{000C}',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => {
                                // Unicode escape: \uXXXX
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    if let Some(h) = chars.next() {
                                        pos += 1;
                                        hex.push(h);
                                    } else {
                                        return Err(TransformError::JsonParseError(
                                            "Unexpected end of unicode escape sequence".into(),
                                        ));
                                    }
                                }

                                // Parse the hex digits to a char
                                match u32::from_str_radix(&hex, 16) {
                                    Ok(n) => match char::from_u32(n) {
                                        Some(unicode_char) => unicode_char,
                                        None => {
                                            return Err(TransformError::JsonParseError(
                                                "Invalid unicode escape sequence".into(),
                                            ))
                                        }
                                    },
                                    Err(_) => {
                                        return Err(TransformError::JsonParseError(
                                            "Invalid unicode escape sequence".into(),
                                        ))
                                    }
                                }
                            }
                            _ => {
                                return Err(TransformError::JsonParseError(format!(
                                    "Invalid escape sequence: \\{}",
                                    ch
                                )))
                            }
                        });
                        escaped = false;
                    } else if ch == '\\' {
                        escaped = true;
                    } else if ch == '"' {
                        break;
                    } else {
                        string.push(ch);
                    }
                }

                tokens.push(JsonToken::String(string));
            }
            '-' | '0'..='9' => {
                let mut number = String::new();
                number.push(c);

                // Parse the rest of the number
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit()
                        || ch == '.'
                        || ch == 'e'
                        || ch == 'E'
                        || ch == '+'
                        || ch == '-'
                    {
                        number.push(ch);
                        chars.next();
                        pos += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(JsonToken::Number(number));
            }
            't' => {
                // Parse "true"
                if chars.next() == Some('r')
                    && chars.next() == Some('u')
                    && chars.next() == Some('e')
                {
                    pos += 3;
                    tokens.push(JsonToken::Bool(true));
                } else {
                    return Err(TransformError::JsonParseError(format!(
                        "Invalid token at position {}",
                        pos
                    )));
                }
            }
            'f' => {
                // Parse "false"
                if chars.next() == Some('a')
                    && chars.next() == Some('l')
                    && chars.next() == Some('s')
                    && chars.next() == Some('e')
                {
                    pos += 4;
                    tokens.push(JsonToken::Bool(false));
                } else {
                    return Err(TransformError::JsonParseError(format!(
                        "Invalid token at position {}",
                        pos
                    )));
                }
            }
            'n' => {
                // Parse "null"
                if chars.next() == Some('u')
                    && chars.next() == Some('l')
                    && chars.next() == Some('l')
                {
                    pos += 3;
                    tokens.push(JsonToken::Null);
                } else {
                    return Err(TransformError::JsonParseError(format!(
                        "Invalid token at position {}",
                        pos
                    )));
                }
            }
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                tokens.push(JsonToken::Whitespace);
            }
            _ => {
                return Err(TransformError::JsonParseError(format!(
                    "Invalid character at position {}",
                    pos
                )))
            }
        }
    }

    Ok(tokens)
}

/// Format JSON tokens with proper indentation
fn format_json(tokens: &[JsonToken]) -> Result<String, TransformError> {
    let mut result = String::new();
    let mut indent_level = 0;
    let indent = "  "; // Two spaces per indent level
    let mut idx = 0;
    let tokens_len = tokens.len();

    while idx < tokens_len {
        let token = &tokens[idx];

        match token {
            JsonToken::OpenBrace | JsonToken::OpenBracket => {
                result.push(if token == &JsonToken::OpenBrace {
                    '{'
                } else {
                    '['
                });

                // Check if the next non-whitespace token is a closing bracket
                let mut peek_idx = idx + 1;
                let mut empty = false;
                while peek_idx < tokens_len {
                    match &tokens[peek_idx] {
                        JsonToken::Whitespace => peek_idx += 1,
                        JsonToken::CloseBrace | JsonToken::CloseBracket => {
                            empty = true;
                            break;
                        }
                        _ => break,
                    }
                }

                if !empty {
                    indent_level += 1;
                    result.push('\n');
                    result.push_str(&indent.repeat(indent_level));
                }
            }
            JsonToken::CloseBrace | JsonToken::CloseBracket => {
                if indent_level > 0 {
                    // Check if previous non-whitespace token was an opening bracket (empty array/object)
                    let mut peek_idx = idx - 1;
                    let mut is_empty = false;
                    while peek_idx > 0 {
                        match &tokens[peek_idx] {
                            JsonToken::Whitespace => peek_idx -= 1,
                            JsonToken::OpenBrace | JsonToken::OpenBracket => {
                                is_empty = true;
                                break;
                            }
                            _ => break,
                        }
                    }

                    if !is_empty {
                        indent_level -= 1;
                        result.push('\n');
                        result.push_str(&indent.repeat(indent_level));
                    }
                }
                result.push(if token == &JsonToken::CloseBrace {
                    '}'
                } else {
                    ']'
                });
            }
            JsonToken::Colon => {
                result.push(':');
                result.push(' '); // Add space after colon
            }
            JsonToken::Comma => {
                result.push(',');
                result.push('\n');
                result.push_str(&indent.repeat(indent_level));
            }
            JsonToken::String(s) => {
                result.push('"');
                result.push_str(s);
                result.push('"');
            }
            JsonToken::Number(n) => {
                result.push_str(n);
            }
            JsonToken::Bool(b) => {
                result.push_str(if *b { "true" } else { "false" });
            }
            JsonToken::Null => {
                result.push_str("null");
            }
            JsonToken::Whitespace => {
                // Skip whitespace tokens
            }
        }

        idx += 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_formatter_empty() {
        let transformer = JsonFormatter;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("  ").unwrap(), "");
    }

    #[test]
    fn test_json_formatter_simple() {
        let formatter = JsonFormatter;
        let input = DEFAULT_TEST_INPUT;
        let expected = r#"{
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
        assert_eq!(formatter.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_formatter_nested() {
        let transformer = JsonFormatter;
        let input = r#"{"person":{"name":"John","age":30,"address":{"city":"New York","zip":"10001"}},"active":true}"#;
        let expected = "{\n  \"person\": {\n    \"name\": \"John\",\n    \"age\": 30,\n    \"address\": {\n      \"city\": \"New York\",\n      \"zip\": \"10001\"\n    }\n  },\n  \"active\": true\n}";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_formatter_array() {
        let transformer = JsonFormatter;
        let input = r#"[1,2,3,{"name":"John"}]"#;
        let expected = "[\n  1,\n  2,\n  3,\n  {\n    \"name\": \"John\"\n  }\n]";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_formatter_empty_objects() {
        let transformer = JsonFormatter;
        let input = r#"{"empty":{},"emptyArray":[],"nonempty":{"key":"value"}}"#;
        let expected = "{\n  \"empty\": {},\n  \"emptyArray\": [],\n  \"nonempty\": {\n    \"key\": \"value\"\n  }\n}";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
