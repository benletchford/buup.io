# Buup CLI

Command-line interface for Buup text transformation utilities.

## Installation

```bash
# From the repository root
cargo install --path buup_cli

# Verify installation
buup --version
```

## Usage

### List Available Transformers

```bash
buup list
```

### Apply Transformations

```bash
# Direct transformer subcommand (recommended)
buup <transformer_id> [input_text]

# Via transform command
buup transform --transformer <transformer_id> [input_text]
```

### Input Methods

```bash
# 1. Direct argument
buup base64encode "Hello, CLI!"

# 2. From file
buup urlencode --input input.txt

# 3. From stdin (pipe)
echo "Piped input." | buup hexencode
```

### Output Methods

```bash
# 1. To stdout (default)
buup textreverse "Hello world"

# 2. To file
buup jsonformatter '{"a":1,"b":2}' --output formatted.json
```

### Examples

```bash
# Encode text to base64
buup base64encode "Hello, World!"
# Output: SGVsbG8sIFdvcmxkIQ==

# URL encode special characters
echo "test@example.com?subject=test" | buup urlencode
# Output: test%40example.com%3Fsubject%3Dtest

# Convert JSON to CSV
echo '[{"name":"Alice","age":30},{"name":"Bob","age":25}]' | buup jsontocsv > users.csv

# Chain transformations
echo "Chain Example" | buup base64encode | buup textreverse
```

## Command Reference

For complete command details:

```bash
buup --help
buup <transformer_id> --help
```
