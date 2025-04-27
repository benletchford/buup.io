# Buup CLI

A command-line interface for the `buup` text transformation utilities.

## Installation

Make sure you have Rust and Cargo installed. Then, you can install `buup_cli` from the source:

```bash
# Navigate to the root of the buup repository
cd /path/to/buup

# Install the CLI binary
cargo install --path buup_cli

# Verify installation
buup --version
```

Alternatively, you can run it directly from the source using `cargo run`:

```bash
# From the workspace root
cargo run --bin buup -- --help
```

## Usage

### Listing Transformers

To see all available transformers, categorized by type:

```bash
buup list
```

### Transforming Text

There are two main ways to apply a transformation:

1.  **Using the `transform` subcommand:** Specify the transformer ID.

    ```bash
    buup transform --transformer <TRANSFORMER_ID> [options] [input_text]
    ```

2.  **Using direct transformer subcommands:** Most transformers have their own subcommand named after their ID.

    ```bash
    buup <TRANSFORMER_ID> [options] [input_text]
    ```

### Input Methods

Buup CLI accepts input in three ways:

1.  **Direct Argument:** Provide the text directly after the command.

    ```bash
    buup base64encode "Hello, CLI!"
    # Output: SGVsbG8sIENMSSENQ==
    ```

2.  **From a File:** Use the `--input` or `-i` option.

    ```bash
    echo "Input from file." > input.txt
    buup urlencode --input input.txt
    # Output: Input+from+file.
    ```

3.  **From Stdin (Pipe):** Pipe the output of another command into `buup`.

    ```bash
    echo "Piped input." | buup hexencode
    # Output: 506970656420696e7075742e
    ```

_Note: If input is provided via multiple methods (e.g., direct argument and `-i`), the direct argument takes precedence._

### Output Methods

Results can be output in two ways:

1.  **To Stdout (Default):** The transformed text is printed to the terminal.

    ```bash
    buup textreverse "Output to stdout."
    # Output: .tuodts ot tuptuO
    ```

2.  **To a File:** Use the `--output` or `-o` option.

    ```bash
    buup jsonformatter '{"a": 1, "b": 2}' --output formatted.json
    # formatted.json will contain the indented JSON
    ```

### Chaining Transformations

You can easily pipe the output of one `buup` command into another:

```bash
echo "Chain Example" | buup base64encode | buup textreverse
# Output: ==QklESBhZWFlOiJBY0

# Verify by decoding
echo "==QklESBhZWFlOiJBY0" | buup textreverse | buup base64decode
# Output: Chain Example
```

### Examples

```bash
# Encode text directly
buup base64encode "Hello, World!"
# Output: SGVsbG8sIFdvcmxkIQ==

# Decode base64 text from a file
echo "SGVsbG8sIFdvcmxkIQ==" > encoded.txt
buup base64decode --input encoded.txt
# Output: Hello, World!

# URL encode text with special characters via pipe
echo "Email test@example.com?subject=test" | buup urlencode
# Output: Email+test%40example.com%3Fsubject%3Dtest

# Convert JSON to CSV and save to file
echo '[{"name":"Alice","age":30},{"name":"Bob","age":25}]' | buup jsontocsv --output users.csv
# users.csv will contain:
# name,age
# Alice,30
# Bob,25

# Calculate SHA-256 hash
buup sha256hash "my secret password"
# Output: 5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8
```

## Available Transformers

To get the most up-to-date list of available transformers and their descriptions, run:

```bash
buup list
```
