# chef 🧑‍🍳

A CyberChef-inspired command-line tool for data transformation, encoding, decoding, hashing, extraction, and analysis. Written in Rust.

> **chef** lets you apply common data operations like Base64 encoding, hex conversion, URL encoding, hash calculation, ROT13, XOR, IP/URL/email extraction, text statistics, and entropy estimation — all from the terminal.

## Features

- **23 built-in operations** covering encoding, decoding, hashing, extraction, transformation, and analysis
- **Pipeline support**: chain multiple operations together like CyberChef recipes
- **Standard I/O**: pipe data in and out, combine with other tools
- **Clear help messages** and meaningful error handling
- **Fully tested**: 69 unit tests covering all operations, edge cases, and round-trips
- **Zero clippy warnings**: passes `cargo clippy -- -D warnings`

## Installation

### Using Cargo

```bash
cargo install chef
```

### From Source

```bash
git clone <repo-url>
cd chef
cargo build --release
./target/release/chef list
```

### Using Docker

```bash
docker build -t chef .
docker run --rm chef list
```

## Usage

### Single Operation

```bash
chef op <operation> [input]
```

### Pipeline (Chained Operations)

```bash
chef run <op1,op2,...> [input]
```

### List Operations

```bash
chef list
```

### Reading from stdin

```bash
echo "hello" | chef op base64-encode
cat file.txt | chef op hex-encode
```

### Passing Arguments to Operations

For operations requiring additional arguments (like XOR key), use `--`:

```bash
chef op xor "hello" -- mykey
```

## Available Operations

```
  base64-encode   : Encode input data to Base64
  base64-decode   : Decode Base64 encoded data
  hex-encode      : Encode input data to hexadecimal
  hex-decode      : Decode hexadecimal data
  url-encode      : URL encode input data
  url-decode      : URL decode input data
  rot13           : Apply ROT13 (Caesar cipher shift by 13)
  xor             : Apply XOR operation with a key
  md5             : Compute MD5 hash
  sha1            : Compute SHA-1 hash
  sha256          : Compute SHA-256 hash
  extract-ips     : Extract IPv4 addresses
  extract-urls    : Extract URLs
  extract-emails  : Extract email addresses
  text-stats      : Compute text statistics
  entropy         : Estimate Shannon entropy
  binary-encode   : Encode to binary (bits)
  binary-decode   : Decode from binary (bits)
  json-pretty     : Pretty-print JSON
  json-minify     : Minify JSON
  lower-case      : Convert to lowercase
  upper-case      : Convert to uppercase
  reverse         : Reverse the input string
```

## Examples

### Encoding/Decoding

```bash
# Base64
$ chef op base64-encode "hello"
aGVsbG8=

$ chef op base64-decode "aGVsbG8="
hello

# Hex
$ chef op hex-encode "hello"
68656c6c6f

$ chef op hex-decode "68656c6c6f"
hello

# URL
$ chef op url-encode "hello world"
hello%20world

$ chef op url-decode "hello%20world"
hello world
```

### Hashing

```bash
$ chef op md5 "hello"
5d41402abc4b2a76b9719d911017c592

$ chef op sha256 "hello"
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

### Extraction

```bash
$ chef op extract-ips "Server 192.168.1.1 and 10.0.0.1"
192.168.1.1
10.0.0.1

$ chef op extract-emails "Contact support@example.com"
support@example.com
```

### Analysis

```bash
$ chef op text-stats "Hello, World!"
Characters: 13
Words: 2
Lines: 1
Bytes: 13

$ chef op entropy "Hello, World!"
2.8454
```

### Pipelining (Chaining)

Chain operations together like CyberChef recipes:

```bash
# Encode to Base64, then hex-encode the result
$ chef run "base64-encode,hex-encode" "hello"
614756736247383d

# Hex encode then reverse
$ chef run "hex-encode,reverse" "hello"
f6c6c65686
```

### Working with Files

```bash
$ chef op extract-emails < samples/input.txt
support@example.com
admin@test.org

$ chef op json-pretty < samples/input.json
{
  "name": "John Doe",
  "age": 30,
  ...
}
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed technical documentation including:

- Project structure and design patterns
- Operation trait and pipeline architecture
- Data flow diagrams
- Testing strategy
- Limitations and future work

## Testing

```bash
# Run all tests (69 unit + integration tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run clippy
cargo clippy -- -D warnings
```

## Building with Docker

```bash
# Build the Docker image
docker build -t chef .

# Run a single operation
docker run --rm chef op base64-encode "hello"

# Run with stdin
echo "hello" | docker run --rm -i chef op base64-encode

# List operations
docker run --rm chef list
```

## Project Structure

```
chef/
├── Cargo.toml           # Rust dependencies and metadata
├── Dockerfile           # Multi-stage Docker build
├── README.md            # This file
├── ARCHITECTURE.md      # Technical documentation
├── samples/
│   ├── input.txt        # Sample text input
│   └── input.json       # Sample JSON input
├── examples/
│   └── README.md        # Example command outputs
└── src/
    ├── main.rs           # CLI entry point
    ├── lib.rs            # Library root
    ├── cli.rs            # CLI definition and dispatch
    ├── pipeline.rs       # Operation chaining
    └── operations/       # Operation implementations
        ├── mod.rs
        ├── base64_op.rs
        ├── hex_op.rs
        ├── url_op.rs
        ├── rot13_op.rs
        ├── xor_op.rs
        ├── hash_op.rs
        ├── extract_op.rs
        ├── text_op.rs
        ├── binary_op.rs
        ├── json_op.rs
        └── case_op.rs
```

## Requirements

- Rust 1.96+ (for building from source)
- Docker (optional, for containerized usage)

## License

MIT
