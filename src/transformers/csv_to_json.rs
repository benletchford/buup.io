use crate::{Transform, TransformError, TransformerCategory};

/// CSV to JSON transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsvToJson;

impl Transform for CsvToJson {
    fn name(&self) -> &'static str {
        "CSV to JSON"
    }

    fn id(&self) -> &'static str {
        "csvtojson"
    }

    fn description(&self) -> &'static str {
        "Converts CSV data to JSON format"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.trim().is_empty() {
            return Ok("[]".to_string());
        }

        let mut lines = input.lines().collect::<Vec<_>>();
        if lines.is_empty() {
            return Ok("[]".to_string());
        }

        // Extract header row
        let header = lines.remove(0);
        let headers = parse_csv_row(header);

        if headers.is_empty() {
            return Ok("[]".to_string());
        }

        // Process data rows
        let mut json = String::from("[");
        let mut first_row = true;

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let values = parse_csv_row(line);
            if values.is_empty() {
                continue;
            }

            if !first_row {
                json.push(',');
            } else {
                first_row = false;
            }

            // Create JSON object for this row
            json.push_str("\n  {");
            let mut first_field = true;

            for (i, value) in values.iter().enumerate() {
                if i >= headers.len() {
                    break;
                }

                if !first_field {
                    json.push(',');
                } else {
                    first_field = false;
                }

                // Escape JSON field name
                json.push_str(&format!("\n    \"{}\":", escape_json_string(&headers[i])));

                // Handle value based on content
                if value.trim().is_empty() {
                    json.push_str("null");
                } else if value == "true"
                    || value == "false"
                    || value == "null"
                    || value.parse::<f64>().is_ok()
                {
                    // Numbers, booleans, and null can be added directly
                    json.push_str(value);
                } else {
                    // String values need to be quoted and escaped
                    json.push_str(&format!("\"{}\"", escape_json_string(value)));
                }
            }

            json.push_str("\n  }");
        }

        if first_row {
            // No rows were processed, return an empty array without newlines
            return Ok("[]".to_string());
        }

        json.push_str("\n]");
        Ok(json)
    }

    fn default_test_input(&self) -> &'static str {
        "id,name,value\n1,apple,1.5\n2,banana,0.75"
    }
}

/// Parses a CSV row into fields, handling quoted values
fn parse_csv_row(row: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = row.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    // Escaped quote inside quoted field
                    chars.next(); // Consume the second quote
                    current_field.push('"');
                } else {
                    // Toggle quote mode
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                // End of field
                fields.push(current_field);
                current_field = String::new();
            }
            _ => {
                current_field.push(c);
            }
        }
    }

    // Add the last field
    fields.push(current_field);
    fields
}

/// Escapes special characters in a JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);

    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{0008}' => result.push_str("\\b"),
            '\u{000C}' => result.push_str("\\f"),
            _ if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_json_basic() {
        let transformer = CsvToJson;
        let input = transformer.default_test_input(); // Use default for a basic check
        let expected_default = r#"[
  {
    "id":1,
    "name":"apple",
    "value":1.5
  },
  {
    "id":2,
    "name":"banana",
    "value":0.75
  }
]"#;
        assert_eq!(transformer.transform(input).unwrap(), expected_default);

        let input_complex = "name,age,active\nAlice,30,true\nBob,25,false";
        let expected_complex = r#"[
  {
    "name":"Alice",
    "age":30,
    "active":true
  },
  {
    "name":"Bob",
    "age":25,
    "active":false
  }
]"#;
        assert_eq!(
            transformer.transform(input_complex).unwrap(),
            expected_complex
        );
    }

    #[test]
    fn test_csv_to_json_with_quotes() {
        let transformer = CsvToJson;
        let input = r#"name,quote
Alice,"Hello, world"
Bob,"Quoted ""text"" here""#;
        let expected = r#"[
  {
    "name":"Alice",
    "quote":"Hello, world"
  },
  {
    "name":"Bob",
    "quote":"Quoted \"text\" here"
  }
]"#;
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_csv_to_json_empty() {
        let transformer = CsvToJson;
        assert_eq!(transformer.transform("").unwrap(), "[]");
    }

    #[test]
    fn test_csv_to_json_header_only() {
        let transformer = CsvToJson;
        assert_eq!(transformer.transform("name,age").unwrap(), "[]");
    }

    #[test]
    fn test_csv_to_json_with_null() {
        let transformer = CsvToJson;
        let input = "name,age\nAlice,\nBob,25";
        let expected = r#"[
  {
    "name":"Alice",
    "age":null
  },
  {
    "name":"Bob",
    "age":25
  }
]"#;
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
