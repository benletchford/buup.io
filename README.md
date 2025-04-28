# Buup - The Text Utility Belt

Buup is a versatile text transformation toolkit that provides a dependency-free core library for common text manipulations, with a built-in CLI and web interface.

|                            Dark Mode                            |                            Light Mode                            |
| :-------------------------------------------------------------: | :--------------------------------------------------------------: |
| <img src="buup_web/assets/web-screenshot-dark.png" width="400"> | <img src="buup_web/assets/web-screenshot-light.png" width="400"> |

## Architecture

```bash
buup/
|- src/                # Core transformation library with zero dependencies (published as "buup")
|  |- cli.rs           # Zero-dependency CLI implementation integrated with the core library
|- buup_web/           # Web UI implementation (Dioxus)
```

## Key Features

- **Zero Dependencies**: The core `buup` library and its CLI implement all transformations without external dependencies
- **Multiple Interfaces**: CLI for terminal workflows and Web UI for interactive use
- **Extensible Design**: Easy to add new transformers by implementing the `Transform` trait
- **Strong Typing**: Full type safety with comprehensive error handling
- **Thread Safety**: All transformers are designed to be safely used concurrently

## Available Transformers

The following transformers are currently available in Buup:

```bash
Available transformers:

ENCODERS:
  ascii_to_hex    - Convert ASCII characters to their hexadecimal representation.
  base64encode    - Encode text to Base64 format
  bin_to_hex      - Convert binary numbers to hexadecimal.
  binaryencode    - Encode text into its binary representation (space-separated bytes).
  dec_to_bin      - Convert decimal numbers to binary.
  dec_to_hex      - Convert decimal numbers to hexadecimal.
  hex_to_bin      - Convert hexadecimal numbers to binary.
  hexencode       - Encode text to hexadecimal representation
  htmlencode      - Encodes special characters to HTML entities
  morseencode     - Encode text to Morse code
  rot13           - Applies the ROT13 substitution cipher to the input text.
  urlencode       - Encode text for use in URLs

DECODERS:
  base64decode    - Decode Base64 text to plain text
  bin_to_dec      - Convert binary numbers to decimal.
  binarydecode    - Decode space-separated binary representation back to text.
  hex_to_ascii    - Convert hexadecimal representation back to ASCII characters.
  hex_to_dec      - Convert hexadecimal numbers to decimal.
  hexdecode       - Decode hexadecimal to original text
  htmldecode      - Decodes HTML entities back to special characters
  morsedecode     - Decode Morse code to text
  urldecode       - Decode URL-encoded text

FORMATTERS:
  jsonformatter   - Formats JSON with proper indentation
  jsonminifier    - Minifies JSON by removing whitespace

CRYPTOGRAPHY:
  md5hash         - Computes the MD5 hash of the input text
  sha256hash      - Computes the SHA-256 hash of the input text

OTHERS:
  cameltosnake    - Converts camelCase or PascalCase to snake_case
  csvtojson       - Converts CSV data to JSON format
  jsontocsv       - Converts JSON data to CSV format
  snaketocamel    - Converts snake_case to camelCase
  textreverse     - Reverses the input text

EXAMPLES:
  buup base64encode "Hello, world!"     # Encode text directly
  buup urldecode -i encoded.txt         # Decode from file
  echo "Hello" | buup hexencode         # Pipe from stdin
```

## Usage as a Library

```bash
cargo add buup
```

```rust
use buup::{transformer_from_id, Transform, Base64Encode};

// Option 1: Use a specific transformer struct
let encoded = Base64Encode.transform("Hello, Library!").unwrap();
println!("{}", encoded); // SGVsbG8sIExpYnJhcnkh

// Option 2: Look up a transformer by its ID
let transformer = transformer_from_id("base64decode").unwrap();
let decoded = transformer.transform(&encoded).unwrap();
println!("{}", decoded); // Hello, Library!
```

## Interfaces

- **CLI**: Zero-dependency CLI included directly in the core library
- **[Web UI](buup_web/README.md)**: Modern web interface built with Dioxus

## Building From Source

```bash
# Clone the repository
git clone https://github.com/benletchford/buup.git
cd buup

# Build the entire workspace
cargo build --release

# Run the built-in CLI
cargo run --bin buup -- list

# Serve the web UI (requires Dioxus CLI)
cd buup_web
dx serve
```

## Contributing

Contributions are welcome! When adding new transformers or modifying code, please ensure:

1. **Zero external dependencies** in the core `buup` library
2. **Comprehensive tests** covering functionality and edge cases
3. **Clear error handling** using `TransformError`
4. Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`

### Creating Custom Transformers

This guide shows how to create custom transformers for the buup system without modifying the core library.

**Basic Structure**

To create a custom transformer:

1. Create a new struct that implements the `Transform` trait
2. Add the struct to the registry in `lib.rs`

**Step 1: Example Implementation**

Here's a simple example of a custom transformer that reverses text:

```rust,ignore
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

**Step 2: Add to Registry**

In `lib.rs`, add your transformer to the `register_builtin_transformers` function:

```rust,ignore
fn register_builtin_transformers() -> Registry {
    let mut registry = Registry {
        transformers: HashMap::new(),
    };
    // ... existing registrations ...

    // Import your new transformer struct
    use crate::transformers::my_transformer::MyNewTransformer;

    // Add your new transformer instance
    registry.transformers.insert(MyNewTransformer.id(), &MyNewTransformer);

    registry
}
```

**Step 3: Export Your Transformer (Optional)**

If your transformer will be used directly, add it to the exports in `transformers/mod.rs`:

```rust,ignore
// src/transformers/mod.rs
mod base64_decode;
// ... other mods ...
mod my_transformer; // Your new module

pub use base64_decode::Base64Decode;
// ... other uses ...
pub use my_transformer::MyNewTransformer; // Your new transformer
```

**Step 4: Add Inverse Support (Optional)**

If your transformer has an inverse operation, update the `inverse_transformer` function in `lib.rs`:

```rust,ignore
pub fn inverse_transformer(t: &dyn Transform) -> Option<&'static dyn Transform> {
    match t.id() {
        // ... existing matches ...
        "my_encoder" => transformer_from_id("my_decoder").ok(), // Your transformer pair
        "my_decoder" => transformer_from_id("my_encoder").ok(), // Your transformer pair
        _ => None,
    }
}
```

**Example: Creating Pairs of Transformers**

If your transformer has a logical inverse (like encoding/decoding), you'll want to define both:

```rust,ignore
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
    // ... rest of implementation ...
}

// Implement Transform for CamelToSnake
impl Transform for CamelToSnake {
    fn name(&self) -> &'static str {
        "Camel to Snake Case"
    }
    // ... rest of implementation ...
}

// Then register both in lib.rs and define their inverse relationship
```

**Tips for Creating Good Transformers**

1. Follow the naming convention of existing transformers
2. Provide clear and concise descriptions
3. Make sure your transformer is thread-safe (impl Sync+Send)
4. Consider implementing pairs of transformers for inverse operations
5. Add comprehensive tests for your transformer

## Documentation Generation

To update the transformer list in this README after modifications:

```bash
cargo run --bin update_readme
```
