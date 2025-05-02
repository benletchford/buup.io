use crate::{Transform, TransformError, TransformerCategory};

/// JSON to CSV transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonToCsv;

/// Default test input for JSON to CSV
pub const DEFAULT_TEST_INPUT: &str = r#"[{"id":1,"name":"apple","color":"red"},{"id":2,"name":"banana","color":"yellow"},{"id":3,"name":"grape"}]"#;

impl Transform for JsonToCsv {
    fn name(&self) -> &'static str {
        "JSON to CSV"
    }

    fn id(&self) -> &'static str {
        "jsontocsv"
    }

    fn description(&self) -> &'static str {
        "Converts a JSON array of objects into CSV format."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn default_test_input(&self) -> &'static str {
        DEFAULT_TEST_INPUT
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Early return for empty or whitespace-only input
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(String::new());
        }

        // Ensure input starts with array
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return Err(TransformError::JsonParseError(
                "Input must be a JSON array of objects".to_string(),
            ));
        }

        let content = &trimmed[1..trimmed.len() - 1].trim();
        if content.is_empty() {
            return Ok(String::new()); // Empty array
        }

        // Parse the array of objects manually
        let objects = parse_json_array(content)?;
        if objects.is_empty() {
            return Ok(String::new());
        }

        // Collect all unique keys across all objects
        let mut headers = Vec::new();
        for obj in &objects {
            for (key, _) in obj {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
            }
        }

        // Sort headers for consistent output
        headers.sort();

        // Build CSV header row
        let mut csv = headers.join(",");
        csv.push('\n');

        // Build data rows
        for obj in objects {
            let mut first = true;
            for header in &headers {
                if !first {
                    csv.push(',');
                }
                first = false;

                // Find the value for this header
                let value = obj
                    .iter()
                    .find_map(|(key, val)| if key == header { Some(val) } else { None });

                if let Some(value) = value {
                    // Format value according to CSV rules
                    let formatted = format_csv_value(value);
                    csv.push_str(&formatted);
                }
                // If key isn't present, leave field empty
            }
            csv.push('\n');
        }

        // Remove trailing newline
        if csv.ends_with('\n') {
            csv.pop();
        }

        Ok(csv)
    }
}

/// A simple representation of JSON values
#[derive(Debug, Clone)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(String), // Store as string to preserve original format
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

/// Formats a JSON value for CSV output
fn format_csv_value(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => String::new(),
        JsonValue::Boolean(b) => b.to_string(),
        JsonValue::Number(n) => n.clone(),
        JsonValue::String(s) => {
            // Escape quotes and wrap in quotes if necessary
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                let escaped = s.replace('"', "\"\"");
                format!("\"{}\"", escaped)
            } else {
                s.clone()
            }
        }
        JsonValue::Array(arr) => {
            let values: Vec<String> = arr.iter().map(format_csv_value).collect();
            format!("\"{}\"", values.join(";").replace('"', "\"\""))
        }
        JsonValue::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{}:{}", k, format_csv_value(v)))
                .collect();
            format!("\"{}\"", pairs.join(";").replace('"', "\"\""))
        }
    }
}

/// Parses a JSON array into a vector of objects
fn parse_json_array(input: &str) -> Result<Vec<Vec<(String, JsonValue)>>, TransformError> {
    let mut objects = Vec::new();
    let mut pos = 0;
    let input = input.trim();

    // Handle empty array case
    if input.is_empty() {
        return Ok(objects);
    }

    while pos < input.len() {
        // Find start of object
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            break;
        }

        if input.as_bytes()[pos] != b'{' {
            return Err(TransformError::JsonParseError(format!(
                "Expected '{{' at position {}, found '{}'",
                pos,
                &input[pos..pos + 1]
            )));
        }

        // Parse object
        let (object, new_pos) = parse_json_object(input, pos)?;
        objects.push(object);
        pos = new_pos;

        // Skip to next object or end
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            break;
        }

        // Check for comma separator
        if input.as_bytes()[pos] == b',' {
            pos += 1;
        }
    }

    Ok(objects)
}

/// Parses a JSON object into a vector of key-value pairs
fn parse_json_object(
    input: &str,
    start_pos: usize,
) -> Result<(Vec<(String, JsonValue)>, usize), TransformError> {
    let mut pairs = Vec::new();
    let mut pos = start_pos + 1; // Skip opening '{'
    let bytes = input.as_bytes();

    loop {
        // Skip whitespace
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            return Err(TransformError::JsonParseError(
                "Unexpected end of input".to_string(),
            ));
        }

        // Check for closing brace
        if bytes[pos] == b'}' {
            return Ok((pairs, pos + 1));
        }

        // Parse key (must be a string)
        if bytes[pos] != b'"' {
            return Err(TransformError::JsonParseError(format!(
                "Expected '\"' at position {}, found '{}'",
                pos,
                &input[pos..pos + 1]
            )));
        }

        let (key, new_pos) = parse_json_string(input, pos)?;
        pos = new_pos;

        // Skip whitespace and expect colon
        pos = skip_whitespace(input, pos);
        if pos >= input.len() || bytes[pos] != b':' {
            return Err(TransformError::JsonParseError("Expected ':'".to_string()));
        }
        pos += 1;

        // Parse value
        let (value, new_pos) = parse_json_value(input, skip_whitespace(input, pos))?;
        pairs.push((key, value));
        pos = new_pos;

        // Skip whitespace and expect comma or closing brace
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            return Err(TransformError::JsonParseError(
                "Unexpected end of input".to_string(),
            ));
        }

        if bytes[pos] == b',' {
            pos += 1;
        } else if bytes[pos] != b'}' {
            return Err(TransformError::JsonParseError(format!(
                "Expected '}}' or ',' at position {}, found '{}'",
                pos,
                &input[pos..pos + 1]
            )));
        }
    }
}

/// Parses a JSON value
fn parse_json_value(input: &str, start_pos: usize) -> Result<(JsonValue, usize), TransformError> {
    let pos = skip_whitespace(input, start_pos);
    if pos >= input.len() {
        return Err(TransformError::JsonParseError(
            "Unexpected end of input".to_string(),
        ));
    }

    match input.as_bytes()[pos] {
        b'"' => {
            let (string, new_pos) = parse_json_string(input, pos)?;
            Ok((JsonValue::String(string), new_pos))
        }
        b'{' => {
            let (object, new_pos) = parse_json_object(input, pos)?;
            Ok((JsonValue::Object(object), new_pos))
        }
        b'[' => {
            let (array, new_pos) = parse_json_array_values(input, pos)?;
            Ok((JsonValue::Array(array), new_pos))
        }
        b't' => {
            if pos + 4 <= input.len() && &input[pos..pos + 4] == "true" {
                Ok((JsonValue::Boolean(true), pos + 4))
            } else {
                Err(TransformError::JsonParseError(
                    "Invalid 'true' literal".to_string(),
                ))
            }
        }
        b'f' => {
            if pos + 5 <= input.len() && &input[pos..pos + 5] == "false" {
                Ok((JsonValue::Boolean(false), pos + 5))
            } else {
                Err(TransformError::JsonParseError(
                    "Invalid 'false' literal".to_string(),
                ))
            }
        }
        b'n' => {
            if pos + 4 <= input.len() && &input[pos..pos + 4] == "null" {
                Ok((JsonValue::Null, pos + 4))
            } else {
                Err(TransformError::JsonParseError(
                    "Invalid 'null' literal".to_string(),
                ))
            }
        }
        b'-' | b'0'..=b'9' => parse_json_number(input, pos),
        _ => Err(TransformError::JsonParseError(format!(
            "Unexpected character at position {}: '{}'",
            pos,
            &input[pos..pos + 1]
        ))),
    }
}

/// Parses a JSON array of values
fn parse_json_array_values(
    input: &str,
    start_pos: usize,
) -> Result<(Vec<JsonValue>, usize), TransformError> {
    let mut values = Vec::new();
    let mut pos = start_pos + 1; // Skip opening '['
    let bytes = input.as_bytes();

    loop {
        // Skip whitespace
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            return Err(TransformError::JsonParseError(
                "Unexpected end of input".to_string(),
            ));
        }

        // Check for closing bracket
        if bytes[pos] == b']' {
            return Ok((values, pos + 1));
        }

        // Parse value
        let (value, new_pos) = parse_json_value(input, pos)?;
        values.push(value);
        pos = new_pos;

        // Skip whitespace and expect comma or closing bracket
        pos = skip_whitespace(input, pos);
        if pos >= input.len() {
            return Err(TransformError::JsonParseError(
                "Unexpected end of input".to_string(),
            ));
        }

        if bytes[pos] == b',' {
            pos += 1;
        } else if bytes[pos] != b']' {
            return Err(TransformError::JsonParseError(format!(
                "Expected ']' or ',' at position {}",
                pos
            )));
        }
    }
}

/// Parses a JSON string
fn parse_json_string(input: &str, start_pos: usize) -> Result<(String, usize), TransformError> {
    let mut result = String::new();
    let mut pos = start_pos + 1; // Skip opening quote
    let bytes = input.as_bytes();

    while pos < input.len() {
        let byte = bytes[pos];

        if byte == b'"' {
            // End of string
            return Ok((result, pos + 1));
        } else if byte == b'\\' {
            // Escape sequence
            pos += 1;
            if pos >= input.len() {
                return Err(TransformError::JsonParseError(
                    "Unexpected end of input".to_string(),
                ));
            }

            match bytes[pos] {
                b'"' => result.push('"'),
                b'\\' => result.push('\\'),
                b'/' => result.push('/'),
                b'b' => result.push('\u{0008}'),
                b'f' => result.push('\u{000C}'),
                b'n' => result.push('\n'),
                b'r' => result.push('\r'),
                b't' => result.push('\t'),
                b'u' => {
                    // Unicode escape sequence
                    if pos + 4 >= input.len() {
                        return Err(TransformError::JsonParseError(
                            "Invalid Unicode escape".to_string(),
                        ));
                    }

                    let hex = &input[pos + 1..pos + 5];
                    let code_point = u32::from_str_radix(hex, 16).map_err(|_| {
                        TransformError::JsonParseError("Invalid Unicode escape".to_string())
                    })?;

                    if let Some(c) = std::char::from_u32(code_point) {
                        result.push(c);
                    } else {
                        return Err(TransformError::JsonParseError(
                            "Invalid Unicode codepoint".to_string(),
                        ));
                    }

                    pos += 4; // Skip the 4 hex digits
                }
                _ => {
                    return Err(TransformError::JsonParseError(
                        "Invalid escape sequence".to_string(),
                    ))
                }
            }
        } else {
            // Regular character
            result.push(input[pos..].chars().next().unwrap());
        }

        pos += 1;
    }

    Err(TransformError::JsonParseError(
        "Unterminated string".to_string(),
    ))
}

/// Parses a JSON number
fn parse_json_number(input: &str, start_pos: usize) -> Result<(JsonValue, usize), TransformError> {
    let mut end = start_pos;
    let bytes = input.as_bytes();

    // Sign
    if end < input.len() && bytes[end] == b'-' {
        end += 1;
    }

    // Integer part
    let mut has_digits = false;
    while end < input.len() && bytes[end] >= b'0' && bytes[end] <= b'9' {
        has_digits = true;
        end += 1;
    }

    if !has_digits {
        return Err(TransformError::JsonParseError("Invalid number".to_string()));
    }

    // Fraction part
    if end < input.len() && bytes[end] == b'.' {
        end += 1;
        let mut has_fraction_digits = false;
        while end < input.len() && bytes[end] >= b'0' && bytes[end] <= b'9' {
            has_fraction_digits = true;
            end += 1;
        }

        if !has_fraction_digits {
            return Err(TransformError::JsonParseError(
                "Invalid number: expected digit after decimal point".to_string(),
            ));
        }
    }

    // Exponent
    if end < input.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
        end += 1;

        if end < input.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }

        let mut has_exp_digits = false;
        while end < input.len() && bytes[end] >= b'0' && bytes[end] <= b'9' {
            has_exp_digits = true;
            end += 1;
        }

        if !has_exp_digits {
            return Err(TransformError::JsonParseError(
                "Invalid number: expected digit in exponent".to_string(),
            ));
        }
    }

    let num_str = input[start_pos..end].to_string();
    Ok((JsonValue::Number(num_str), end))
}

/// Skips whitespace characters
fn skip_whitespace(input: &str, start_pos: usize) -> usize {
    let bytes = input.as_bytes();
    let mut pos = start_pos;

    while pos < input.len() {
        match bytes[pos] {
            b' ' | b'\t' | b'\n' | b'\r' => pos += 1,
            _ => break,
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_csv_basic() {
        let transformer = JsonToCsv;
        let input = transformer.default_test_input();
        let result = transformer.transform(input).unwrap();

        let lines: Vec<&str> = result.trim_end().split('\n').collect(); // Trim trailing newline before split
        assert!(
            lines.len() >= 1,
            "CSV output should have at least a header line"
        );

        let header = lines[0];
        // Check header contains each field (alphabetically sorted)
        assert_eq!(header, "color,id,name");

        // Check row content matches the sorted header order
        assert_eq!(lines[1], "red,1,apple");
        assert_eq!(lines[2], "yellow,2,banana");
        assert_eq!(lines[3], ",3,grape");
    }

    #[test]
    fn test_json_to_csv_with_quotes() {
        let transformer = JsonToCsv;
        let input = r#"[
            {"name": "Alice", "quote": "Hello, world"},
            {"name": "Bob", "quote": "Quoted \"text\" here"}
        ]"#;
        let expected = "name,quote\nAlice,\"Hello, world\"\nBob,\"Quoted \"\"text\"\" here\"";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_to_csv_empty() {
        let transformer = JsonToCsv;
        assert_eq!(transformer.transform("[]").unwrap(), "");
        assert_eq!(transformer.transform("").unwrap(), "");
    }

    #[test]
    fn test_json_to_csv_missing_fields() {
        let transformer = JsonToCsv;
        let input = r#"[
            {"name": "Alice", "age": 30},
            {"name": "Bob", "city": "New York"}
        ]"#;
        let expected = "age,city,name\n30,,Alice\n,New York,Bob";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_to_csv_null_values() {
        let transformer = JsonToCsv;
        let input = r#"[
            {"name": "Alice", "age": null},
            {"name": "Bob", "age": 25}
        ]"#;
        let expected = "age,name\n,Alice\n25,Bob";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_json_to_csv_invalid_input() {
        let transformer = JsonToCsv;
        assert!(transformer.transform("{\"name\": \"Alice\"}").is_err());
    }
}
