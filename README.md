# Buup - The Text Utility Belt

Buup is a versatile text transformation toolkit that provides a dependency-free core library for common text manipulations, with additional CLI and web interfaces.

## Architecture

```bash
buup-rs/
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

- **[Web UI](buup_web/README.md)** - A sleek, modern interface built with Dioxus
  - Responsive design that works on mobile and desktop
  - Customizable dark/light themes
- **[CLI](buup_cli/README.md)** - A command-line interface for scripting and automation

## Available Transformers

The following transformers are currently available in Buup:

```bash
Available transformers:

ENCODERS:
  hexencode       - Encode text to hexadecimal representation
  base64encode    - Encode text to Base64 format
  urlencode       - Encode text for use in URLs
  htmlencode      - Encodes special characters to HTML entities

DECODERS:
  base64decode    - Decode Base64 text to plain text
  htmldecode      - Decodes HTML entities back to special characters
  urldecode       - Decode URL-encoded text
  hexdecode       - Decode hexadecimal to original text

FORMATTERS:
  jsonminifier    - Minifies JSON by removing whitespace
  jsonformatter   - Formats JSON with proper indentation

CRYPTOGRAPHY:
  md5hash         - Computes the MD5 hash of the input text
  sha256hash      - Computes the SHA-256 hash of the input text

COMPRESSION:
  lzwcompress     - Compresses byte sequence using LZW algorithm (variable width, max 12-bit) and outputs as hex
  lzwdecompress   - Decompresses hex LZW-compressed data (variable width, max 12-bit) back to original text

OTHERS:
  cameltosnake    - Converts camelCase or PascalCase to snake_case
  snaketocamel    - Converts snake_case to camelCase
  csvtojson       - Converts CSV data to JSON format
  textreverse     - Reverses the input text
  jsontocsv       - Converts JSON data to CSV format

EXAMPLES:
  buup base64encode "Hello, world!"     # Encode text directly
  buup urldecode -i encoded.txt         # Decode from file
  echo "Hello" | buup hexencode         # Pipe from stdin
```

## Usage as a Library

The foundation of Buup is a dependency-free Rust library implementing common text transformations:

```rust
// In your Cargo.toml:
// [dependencies]
// buup = "0.1.0"

use buup::{transformer_from_id, Transform};

// You can directly use the library like this:
let transformer = transformer_from_id("base64encode").unwrap();
let result = transformer.transform("Hello, World!").unwrap();
println!("{}", result); // Outputs: SGVsbG8sIFdvcmxkIQ==
```

## Extending with Custom Transformers

Buup features a modular transformer system that allows you to add new transformers by following a standard pattern.

```rust
use buup::{Transform, TransformError, TransformerCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MyCustomTransformer;

impl Transform for MyCustomTransformer {
    fn name(&self) -> &'static str {
        "My Custom Transformer"
    }

    fn id(&self) -> &'static str {
        "my_custom_transformer"
    }

    fn description(&self) -> &'static str {
        "Describes what your transformer does"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Your transformation logic here
        Ok(input.to_uppercase())
    }
}
```

For more details on creating custom transformers, see the [transformer documentation](src/transformers/examples.md).

## Using in Your Projects

The `buup` crate can be used as a standalone dependency in any Rust project:

```toml
[dependencies]
buup = "0.1.0"
```

This will give you access to all transformers without pulling in any UI-related dependencies. The core library is designed to be lightweight and focused, making it ideal for:

- Embedding in larger applications that need text transformation capabilities
- Server-side processing where UIs aren't needed
- Building your own specialized interfaces on top of the transformation engine
- Adding text utilities to embedded systems or WebAssembly modules

## API Examples

### Basic Usage

```rust
use buup::{transformer_from_id, Transform};

// Example of basic transformer usage:
// Get a transformer by ID
let transformer = transformer_from_id("base64encode").unwrap();

// Transform some text
let result = transformer.transform("Hello, World!").unwrap();
assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");

// Find the inverse transformer
if let Some(inverse) = buup::inverse_transformer(transformer) {
    let original = inverse.transform(&result).unwrap();
    assert_eq!(original, "Hello, World!");
}
```

### Listing All Transformers

```rust
use buup::{all_transformers, Transform};

// Example of listing all available transformers:
for transformer in all_transformers() {
    println!(
        "{} ({}): {}",
        transformer.name(),
        transformer.id(),
        transformer.description()
    );
}
```

All implementations are written in pure Rust with zero external dependencies, ensuring security, portability, and performance.

## Building From Source

```bash
# Clone the repository
git clone https://github.com/benletchford/buup-rs.git
cd buup-rs

# Build everything
cargo build --release

# Run the CLI
cd buup_cli
cargo run -- list

# Run the web UI
cd buup_web
dx serve
```

## Contributing

When adding new transformers, follow these guidelines:

1. **No external dependencies**: Implement everything in pure Rust
2. **Comprehensive tests**: Each transformer must have thorough unit tests
3. **Clear error handling**: All potential errors should be properly handled with descriptive messages
4. **Documentation**: Include examples and edge cases in documentation

## Documentation Generation

The README.md is automatically updated with the latest list of transformers. This happens in two ways:

1. **GitHub Actions**: Whenever changes are pushed to the main branch that affect the transformers, GitHub Actions automatically updates the README
2. **Manual Update**: You can manually update the README by running:
   ```bash
   cargo run --bin update_readme
   ```

This ensures the documentation always stays in sync with the actual available transformers.
