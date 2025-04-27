# Creating Custom Transformers

This guide shows how to create custom transformers for the buup system without modifying the core library.

## Basic Structure

To create a custom transformer:

1. Create a new struct that implements the `Transform` trait
2. Add the struct to the registry in `lib.rs`

## Step 1: Example Implementation

Here's a simple example of a custom transformer that reverses text:

```rust
use buup::{Transform, TransformError};

/// Text Reverse transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextReverse;

impl Transform for TextReverse {
    fn name(&self) -> &'static str {
        "Text Reverse"
    }

    fn id(&self) -> &'static str {
        "textreverse"
    }

    fn description(&self) -> &'static str {
        "Reverses the input text"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input.chars().rev().collect())
    }
}
```

## Step 2: Add to Registry

In `lib.rs`, add your transformer to the `register_builtin_transformers` function:

```rust
fn register_builtin_transformers(registry: &mut Registry) {
    // Register built-in transformers
    registry.transformers.insert(Base64Encode.id(), &Base64Encode);
    registry.transformers.insert(Base64Decode.id(), &Base64Decode);
    registry.transformers.insert(UrlEncode.id(), &UrlEncode);
    registry.transformers.insert(UrlDecode.id(), &UrlDecode);
    registry.transformers.insert(TextReverse.id(), &TextReverse);

    // Add your new transformer
    registry.transformers.insert(MyNewTransformer.id(), &MyNewTransformer);
}
```

## Step 3: Export Your Transformer (Optional)

If your transformer will be used directly, add it to the exports in `transformers/mod.rs`:

```rust
mod base64_decode;
mod base64_encode;
mod text_reverse;
mod url_decode;
mod url_encode;
mod my_transformer; // Your new module

pub use base64_decode::Base64Decode;
pub use base64_encode::Base64Encode;
pub use text_reverse::TextReverse;
pub use url_decode::UrlDecode;
pub use url_encode::UrlEncode;
pub use my_transformer::MyNewTransformer; // Your new transformer
```

## Step 4: Add Inverse Support (Optional)

If your transformer has an inverse operation, update the `inverse_transformer` function in `lib.rs`:

```rust
pub fn inverse_transformer(t: &dyn Transform) -> Option<&'static dyn Transform> {
    match t.id() {
        "base64encode" => transformer_from_id("base64decode").ok(),
        "base64decode" => transformer_from_id("base64encode").ok(),
        "urlencode" => transformer_from_id("urldecode").ok(),
        "urldecode" => transformer_from_id("urlencode").ok(),
        "textreverse" => transformer_from_id("textreverse").ok(), // Self-inverting
        "my_encoder" => transformer_from_id("my_decoder").ok(), // Your transformer pair
        "my_decoder" => transformer_from_id("my_encoder").ok(), // Your transformer pair
        _ => None,
    }
}
```

## Example: Creating Pairs of Transformers

If your transformer has a logical inverse (like encoding/decoding), you'll want to define both:

```rust
use buup::{Transform, TransformError};

// Define the transformers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnakeToCamel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CamelToSnake;

// Implement Transform for SnakeToCamel
impl Transform for SnakeToCamel {
    fn name(&self) -> &'static str {
        "Snake to Camel Case"
    }

    fn id(&self) -> &'static str {
        "snake_to_camel"
    }

    fn description(&self) -> &'static str {
        "Converts snake_case to camelCase"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Implementation...
        let mut result = String::new();
        let mut capitalize = false;

        for c in input.chars() {
            if c == '_' {
                capitalize = true;
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }
}

// Implement Transform for CamelToSnake
impl Transform for CamelToSnake {
    fn name(&self) -> &'static str {
        "Camel to Snake Case"
    }

    fn id(&self) -> &'static str {
        "camel_to_snake"
    }

    fn description(&self) -> &'static str {
        "Converts camelCase to snake_case"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Implementation...
        let mut result = String::new();

        for c in input.chars() {
            if c.is_ascii_uppercase() {
                result.push('_');
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }
}

// Then register both in lib.rs
```

## Example: JSON Formatter and Minifier

Here's an example of implementing a pair of transformers for JSON processing:

```rust
use buup::{Transform, TransformError};

/// JSON Formatter transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonFormatter;

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

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Implementation that formats JSON with indentation...
        // See the actual implementation for details
        Ok(formatted_json)
    }
}

/// JSON Minifier transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonMinifier;

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

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Implementation that removes unnecessary whitespace...
        // See the actual implementation for details
        Ok(minified_json)
    }
}

// Then register both in lib.rs and define them as inverses of each other
```

## Tips for Creating Good Transformers

1. Follow the naming convention of existing transformers
2. Provide clear and concise descriptions
3. Make sure your transformer is thread-safe (impl Sync+Send)
4. Consider implementing pairs of transformers for inverse operations
5. Add comprehensive tests for your transformer
