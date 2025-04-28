# Buup - The Text Utility Belt

Buup is a versatile text transformation toolkit that provides a dependency-free core library and CLI for common text manipulations like encoding/decoding, formatting, cryptography, (coming soon compression/decompression), and more written in pure dependency-free Rust.

It is designed to be a simple, lightweight, **open**, **secure**, **provably fast** and easy to integrate.

Drop-in replacement for all of those dodgy online text transformation tools you've ever used in the past except the batteries are included (and they are all in pure Rust).

It includes a [web application](https://buup.io) which is of course written in pure Rust (WASM via [Dioxus](https://dioxuslabs.com/)) as a separate workspace member.

<div align="center">
    <a href="https://buup.io">
        <img src="buup_web/assets/web-screenshot-dark.png">
    </a>
</div>

## Key Features

- **Zero Dependencies**: The core `buup` library and its CLI implement all transformations without external dependencies
- **Multiple Interfaces**: CLI for terminal workflows and Web UI for interactive use
- **Extensible Design**: Easy to add new transformers by implementing the `Transform` trait
- **Strong Typing**: Full type safety with comprehensive error handling
- **Thread Safety**: All transformers are designed to be safely used concurrently
- **Performance**: Optimized for speed and memory usage

## Ways to Use Buup

Buup offers three distinct ways to transform your text:

### 1. Web Application

A modern, responsive web application for interactive text transformations proudly built with [Dioxus](https://dioxuslabs.com/).

|                            Dark Mode                            |                            Light Mode                            |
| :-------------------------------------------------------------: | :--------------------------------------------------------------: |
| <img src="buup_web/assets/web-screenshot-dark.png" width="400"> | <img src="buup_web/assets/web-screenshot-light.png" width="400"> |

From source:

```bash
# Serve the web UI (requires Dioxus CLI)
cd buup_web
dx serve
```

Build for production:

```bash
dx build
```

### 2. Command Line Interface

Zero-dependency CLI for quick transformations in your terminal workflows.

```bash
# Installation
cargo binstall buup # or cargo install buup

# List available transformers
buup list

# Examples
buup base64encode "Hello, world!"     # Encode text directly
buup urldecode -i encoded.txt         # Decode from file
echo "Hello" | buup hexencode         # Pipe from stdin
```

### 3. Rust Library

Integrate Buup's transformers directly into your Rust applications.

```bash
# Add to your project
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
  linenumberadder - Adds line numbers (1-based) to the beginning of each line.
  linenumberremover - Removes leading line numbers (e.g., '1 ', '2. ', '3:	') from each line.

CRYPTOGRAPHY:
  md5hash         - Computes the MD5 hash of the input text
  sha256hash      - Computes the SHA-256 hash of the input text
  uuid5_generate  - Generates a version 5 UUID based on namespace and name using SHA-1. Input format: "namespace|name". Namespace can be a UUID or one of: dns, url, oid, x500.

OTHERS:
  cameltosnake    - Converts camelCase or PascalCase to snake_case
  csvtojson       - Converts CSV data to JSON format
  jsontocsv       - Converts JSON data to CSV format
  linesorter      - Sorts lines of text alphabetically (ascending).
  slugify         - Converts text into a URL-friendly slug (lowercase, dashes, removes special chars)
  snaketocamel    - Converts snake_case to camelCase
  text_stats      - Calculates basic text statistics (lines, words, chars, sentences)
  textreverse     - Reverses the input text
  uniquelines     - Removes duplicate lines, preserving the order of first occurrence.
  urlparser       - Parses a URL into its components (scheme, authority, path, query, fragment)
  uuid_generate   - Generates a version 4 UUID. Input is ignored. WARNING: Uses a non-cryptographically secure PRNG.
  whitespaceremover - Removes all whitespace (spaces, tabs, newlines) from the input text.

EXAMPLES:
  buup base64encode "Hello, world!"     # Encode text directly
  buup urldecode -i encoded.txt         # Decode from file
  echo "Hello" | buup hexencode         # Pipe from stdin
```

## Update README.md with `buup list`

```bash
cargo run --bin update_readme
```
