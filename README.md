# Buup - The Text Utility Belt

Buup is a modern web application that tries to do one thing well - it provides a suite of text transformation tools in a clean, user-friendly interface.

<img src="public/buup-screenshot.png" alt="Buup Screenshot"/>

## Supported Transformers

Buup includes a wide range of text transformation tools:

### Encoding/Decoding

- **Base64 Encode/Decode**: Convert text to and from Base64 format
- **Base64 to Hex**: Convert Base64 encoded data to hexadecimal
- **URL Encode/Decode**: Encode and decode URL components
- **HTML Encode/Decode**: Convert special characters to and from HTML entities
- **JWT Decode**: Decode and display JWT token contents

### Formatting

- **JSON Format/Minify**: Format JSON for readability or minify by removing whitespace
- **Markdown to HTML**: Convert Markdown syntax to HTML
- **CSV to JSON**: Convert CSV data to JSON format

### Case Conversion

- **camelCase**: Convert text to camelCase
- **PascalCase**: Convert text to PascalCase
- **snake_case**: Convert text to snake_case
- **kebab-case**: Convert text to kebab-case
- **CONSTANT_CASE**: Convert text to CONSTANT_CASE

### Cryptography & Hashing

- **MD5 Hash**: Generate MD5 hash from text input
- **SHA-1 Hash**: Generate SHA-1 hash from text input
- **SHA-256 Hash**: Generate SHA-256 hash from text input

### Number Systems

- **Number Base Converter**: Convert numbers between different bases (binary, octal, decimal, hex)

### Generators & Utilities

- **UUID Generator**: Generate random UUIDs
- **UUID to Timestamp**: Extract timestamp from UUID v1
- **Date/Time Transformers**: Various date and time format conversions

## Development

```bash
# Clone the repository
git clone https://github.com/benletchford/buup.git

# Navigate to the project directory
cd buup

# Install dependencies
npm install

# Start the development server
npm run dev

# Build for production
npm run build
```

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.

## License

This project is open source and available under the MIT license.
