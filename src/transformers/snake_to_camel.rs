use crate::{Transform, TransformError, TransformerCategory};

/// SnakeToCamel transformer converts snake_case to camelCase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnakeToCamel;

/// Default test input for Snake to Camel
pub const DEFAULT_TEST_INPUT: &str = "my_variable_name";

impl Transform for SnakeToCamel {
    fn name(&self) -> &'static str {
        "Snake Case to CamelCase"
    }

    fn id(&self) -> &'static str {
        "snaketocamel"
    }

    fn description(&self) -> &'static str {
        "Converts snake_case to camelCase"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut result = String::with_capacity(input.len());
        let mut capitalize_next = false;

        for c in input.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
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
    fn test_snake_to_camel() {
        let transformer = SnakeToCamel;

        // Test default input
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "myVariableName"
        );

        // Test basic snake_case inputs
        assert_eq!(transformer.transform("hello_world").unwrap(), "helloWorld");
        assert_eq!(
            transformer
                .transform("this_is_a_snake_case_string")
                .unwrap(),
            "thisIsASnakeCaseString"
        );

        // Test empty input
        assert_eq!(transformer.transform("").unwrap(), "");

        // Test single word
        assert_eq!(transformer.transform("hello").unwrap(), "hello");

        // Test with numbers
        assert_eq!(transformer.transform("get_2_items").unwrap(), "get2Items");
        assert_eq!(
            transformer.transform("find_item_42_in_list").unwrap(),
            "findItem42InList"
        );

        // Test with consecutive underscores
        assert_eq!(transformer.transform("hello__world").unwrap(), "helloWorld");

        // Test with trailing underscore
        assert_eq!(transformer.transform("hello_").unwrap(), "hello");

        // Test with leading underscore
        assert_eq!(transformer.transform("_hello").unwrap(), "Hello");
    }
}
