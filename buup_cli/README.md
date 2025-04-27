# Buup CLI

A command-line interface for the Buup text transformation utilities.

## Installation

```
cargo install --path .
```

## Usage

### List available transformers

```
buup list
```

### Transform text using a specific transformer

```
# Using the transform command with a transformer ID
buup transform --transformer base64encode --input file.txt --output encoded.txt

# Direct transformer commands
buup base64encode --input file.txt --output encoded.txt
buup base64decode --input encoded.txt --output decoded.txt
buup urlencode --input file.txt --output encoded.txt
buup urldecode --input encoded.txt --output decoded.txt
```

### Input/Output Methods

Buup CLI supports three ways to provide input:

1. Directly on the command line:

```
buup base64encode "Hello, World!"
# Output: SGVsbG8sIFdvcmxkIQ==
```

2. From a file:

```
buup base64encode --input file.txt
```

3. From stdin (pipe):

```
echo "Hello, World!" | buup base64encode
# Output: SGVsbG8sIFdvcmxkIQ==
```

And two ways to output results:

1. To stdout (default):

```
buup base64encode "Hello, World!"
```

2. To a file:

```
buup base64encode "Hello, World!" --output encoded.txt
```

### Examples

```
# Encode text directly
buup base64encode "Hello, World!"
# Output: SGVsbG8sIFdvcmxkIQ==

# Decode base64 text
buup base64decode SGVsbG8sIFdvcmxkIQ==
# Output: Hello, World!

# Pipe data through transformers
echo "Hello, World!" | buup base64encode | buup base64decode
# Output: Hello, World!

# URL encode text with spaces and special characters
buup urlencode "Hello, World! ?&="
# Output: Hello%2C+World%21+%3F%26%3D

# URL decode text
buup urldecode "Hello%2C+World%21+%3F%26%3D"
# Output: Hello, World! ?&=
```

## Available Transformers

- `base64encode` - Encode text to Base64 format
- `base64decode` - Decode Base64 text to plain text
- `urlencode` - Encode text for use in URLs
- `urldecode` - Decode URL-encoded text
