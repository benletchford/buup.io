use crate::{Transform, TransformError, TransformerCategory};

/// CamelToSnake transformer converts camelCase/PascalCase to snake_case
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CamelToSnake;

impl Transform for CamelToSnake {
    fn name(&self) -> &'static str {
        "CamelCase to Snake Case"
    }

    fn id(&self) -> &'static str {
        "cameltosnake"
    }

    fn description(&self) -> &'static str {
        "Converts camelCase or PascalCase to snake_case"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut result = String::with_capacity(input.len() + (input.len() / 2));
        let mut chars = input.chars();

        // Handle first character without adding an underscore
        if let Some(first_char) = chars.next() {
            result.push(first_char.to_ascii_lowercase());
        }

        // Process remaining characters
        for c in chars {
            if c.is_uppercase() {
                result.push('_');
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_snake() {
        let transformer = CamelToSnake;

        // Test camelCase inputs
        assert_eq!(transformer.transform("helloWorld").unwrap(), "hello_world");
        assert_eq!(
            transformer.transform("thisIsACamelCaseString").unwrap(),
            "this_is_a_camel_case_string"
        );

        // Test PascalCase inputs
        assert_eq!(transformer.transform("HelloWorld").unwrap(), "hello_world");
        assert_eq!(
            transformer.transform("ThisIsAPascalCaseString").unwrap(),
            "this_is_a_pascal_case_string"
        );

        // Test empty input
        assert_eq!(transformer.transform("").unwrap(), "");

        // Test single word
        assert_eq!(transformer.transform("hello").unwrap(), "hello");
        assert_eq!(transformer.transform("Hello").unwrap(), "hello");

        // Test with numbers
        assert_eq!(transformer.transform("get2Items").unwrap(), "get2_items");
        assert_eq!(
            transformer.transform("findItem42InList").unwrap(),
            "find_item42_in_list"
        );
    }
}
