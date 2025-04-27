# Buup - The Text Utility Belt

Buup is a versatile text transformation toolkit that provides a dependency-free core library for common text manipulations, with additional CLI and web interfaces.

|                            Dark Mode                            |                            Light Mode                            |
| :-------------------------------------------------------------: | :--------------------------------------------------------------: |
| <img src="buup_web/assets/web-screenshot-dark.png" width="400"> | <img src="buup_web/assets/web-screenshot-light.png" width="400"> |

## Architecture

```bash
buup/
|- src/                # Core transformation library with zero dependencies (published as "buup")
|- buup_web/           # Web UI implementation (Dioxus)
|- buup_cli/           # Command-line interface
```

The project is structured so that the root crate (`buup`) contains all the core transformation functionality, while the UI implementations are separate workspace members. This architecture provides several key advantages:

- **Clean API Surface**: Developers can depend solely on the core `buup` crate without pulling in UI components
- **Minimal Dependencies**: Users only get the zero-dependency core when using `buup` as a library
- **Clear Separation**: Core transformation logic is isolated from interface concerns
- **Simple Integration**: Add powerful text transformations to any Rust project with a single dependency

## Implementation Philosophy: Zero Dependencies

The heart of Buup is the `buup` library, which implements all transformations with **strictly zero external dependencies**. This architectural decision provides several key advantages:

- **Blazingly Fast**: Pure Rust implementation with no overhead from external libraries
- **Provably Secure**: All code is auditable without trusting external packages
- **Highly Portable**: Can be compiled for virtually any target, including embedded systems
- **Minimal Binary Size**: No bloat from transitive dependencies
- **Fully Deterministic**: Same input always produces the same output without external influence
- **Thread-Safe**: All transformers are designed to be safely used across multiple threads
- **100% Safe Rust**: No unsafe code blocks, providing memory safety guarantees

This zero-dependency approach makes Buup suitable for security-critical applications, air-gapped systems, embedded devices, and anywhere reliability and auditability are paramount.

## Interfaces

Buup offers two primary interfaces:

- **[Web UI](buup_web/README.md)**: A sleek, modern web interface built with Dioxus. Ideal for interactive use. See the [Web UI README](buup_web/README.md) for setup and usage details.
- **[CLI](buup_cli/README.md)**: A command-line interface suitable for scripting, automation, and terminal-based workflows. See the [CLI README](buup_cli/README.md) for installation and command details.

## Available Transformers

The following transformers are currently available in Buup:

```bash
Available transformers:

ENCODERS:
  base64encode    - Encode text to Base64 format
  hexencode       - Encode text to hexadecimal representation
  htmlencode      - Encodes special characters to HTML entities
  urlencode       - Encode text for use in URLs

DECODERS:
  base64decode    - Decode Base64 text to plain text
  hexdecode       - Decode hexadecimal to original text
  htmldecode      - Decodes HTML entities back to special characters
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

The core `buup` crate can be used directly in your Rust projects.

```rust
// In your Cargo.toml:
// [dependencies]
// buup = { version = "0.1" } // Replace with the desired version

use buup::{transformer_from_id, Transform, Base64Encode};

// Option 1: Use a specific transformer struct
let encoded = Base64Encode.transform("Hello, Library!").unwrap();
println!("{}", encoded); // Outputs: SGVsbG8sIExpYnJhcnkh

// Option 2: Look up a transformer by its ID
let transformer = transformer_from_id("base64decode").unwrap();
let decoded = transformer.transform(&encoded).unwrap();
println!("{}", decoded); // Outputs: Hello, Library!
```

## API Examples

### Basic Usage

```rust
use buup::{transformer_from_id, Transform};

// Get a transformer by ID
let transformer = transformer_from_id("urlencode").unwrap();

// Transform some text
let result = transformer.transform("query string?").unwrap();
assert_eq!(result, "query+string%3F");

// Find the inverse transformer
if let Some(inverse) = buup::inverse_transformer(transformer) {
    let original = inverse.transform(&result).unwrap();
    assert_eq!(original, "query string?");
} else {
    println!("Transformer '{}' has no inverse.", transformer.id());
}
```

### Listing All Transformers

```rust
use buup::{all_transformers, Transform};

println!("Available Transformers:");
for transformer in all_transformers() {
    println!(
        "- {} ({}) [{}]: {}",
        transformer.name(),
        transformer.id(),
        transformer.category(), // Display category
        transformer.description()
    );
}
```

### Categorized Transformers

```rust
use buup::categorized_transformers;

let categorized = categorized_transformers();

for (category, transformers) in categorized {
    println!("
{}:", category.to_string().to_uppercase());
    for transformer in transformers {
        println!("  - {}", transformer.id());
    }
}
```

## Extending with Custom Transformers

Buup is designed to be extensible. You can add your own transformers by implementing the `Transform` trait.

```rust
use buup::{Transform, TransformError, TransformerCategory};

// Define your transformer struct
#[derive(Debug, Clone, Copy)]
pub struct UppercaseTransformer;

impl Transform for UppercaseTransformer {
    fn name(&self) -> &'static str { "Uppercase" }
    fn id(&self) -> &'static str { "uppercase" }
    fn description(&self) -> &'static str { "Converts text to uppercase." }
    fn category(&self) -> TransformerCategory { TransformerCategory::Other }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input.to_uppercase())
        // Handle potential errors by returning Err(TransformError::...)
    }
}

// To integrate, you would typically add it to the registry
// (This requires modifying the core `buup` library or managing your own registry)
```

For detailed guidance, refer to the `buup` library documentation.

## Building From Source

```bash
# Clone the repository
git clone https://github.com/benletchford/buup.git
cd buup

# Build the entire workspace (library, CLI, web)
cargo build --release

# Run the CLI (from workspace root)
cargo run --bin buup -- list

# Serve the web UI (requires Dioxus CLI)
# cargo install dioxus-cli # If not already installed
cd buup_web
dx serve
```

## Contributing

Contributions are welcome! When adding new transformers or modifying core logic, please ensure:

1.  **Zero external dependencies** in the core `buup` library.
2.  **Comprehensive tests** covering functionality and edge cases.
3.  **Clear error handling** using `TransformError`.
4.  **Appropriate documentation** (doc comments, README updates if applicable).
5.  Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`.

## Documentation Generation

The list of available transformers in this README is automatically generated. To update it manually after adding or modifying transformers, run:

```bash
cargo run --bin update_readme
```

This ensures the documentation stays synchronized with the code.
