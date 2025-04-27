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

## Key Features

- **Zero Dependencies**: The core `buup` library implements all transformations without external dependencies
- **Multiple Interfaces**: CLI for terminal workflows and Web UI for interactive use
- **Extensible Design**: Easy to add new transformers by implementing the `Transform` trait
- **Strong Typing**: Full type safety with comprehensive error handling
- **Thread Safety**: All transformers are designed to be safely used concurrently

## Available Transformers

The following transformers are currently available in Buup:

```bash
Available transformers:

ENCODERS:
  base64encode    - Encode text to Base64 format
  bin_to_hex      - Convert binary numbers to hexadecimal.
  dec_to_bin      - Convert decimal numbers to binary.
  dec_to_hex      - Convert decimal numbers to hexadecimal.
  hex_to_bin      - Convert hexadecimal numbers to binary.
  hexencode       - Encode text to hexadecimal representation
  htmlencode      - Encodes special characters to HTML entities
  rot13           - Applies the ROT13 substitution cipher to the input text.
  urlencode       - Encode text for use in URLs

DECODERS:
  base64decode    - Decode Base64 text to plain text
  bin_to_dec      - Convert binary numbers to decimal.
  hex_to_dec      - Convert hexadecimal numbers to decimal.
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

```rust
// Add to your Cargo.toml:
// [dependencies]
// buup = { version = "0.1" }

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

- **[CLI](buup_cli/README.md)**: Command-line interface for scripting and terminal workflows
- **[Web UI](buup_web/README.md)**: Modern web interface built with Dioxus

## Building From Source

```bash
# Clone the repository
git clone https://github.com/benletchford/buup.git
cd buup

# Build the entire workspace
cargo build --release

# Run the CLI
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

## Documentation Generation

To update the transformer list in this README after modifications:

```bash
cargo run --bin update_readme
```
