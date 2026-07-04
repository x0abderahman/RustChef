# Architecture & Technical Report

## Overview

`chef` is a command-line tool written in Rust, inspired by CyberChef. It provides a modular pipeline for data transformation operations such as encoding, decoding, hashing, extraction, and analysis. The tool is designed to be extensible, well-tested, and easy to use from the terminal.

## Architecture

```
src/
├── main.rs              # Entry point, CLI argument parsing, stdin handling
├── lib.rs               # Library root, re-exports modules
├── cli.rs               # CLI definition (clap), operation dispatch, validation
├── pipeline.rs           # Operation chaining pipeline
└── operations/
    ├── mod.rs            # Operation trait + OperationResult type
    ├── base64_op.rs      # Base64 Encode/Decode
    ├── hex_op.rs         # Hex Encode/Decode
    ├── url_op.rs         # URL Encode/Decode
    ├── rot13_op.rs       # ROT13 Caesar cipher
    ├── xor_op.rs         # XOR with key
    ├── hash_op.rs        # MD5, SHA1, SHA256
    ├── extract_op.rs     # IP, URL, Email extraction
    ├── text_op.rs        # Text statistics, Entropy estimation
    ├── binary_op.rs      # Binary Encode/Decode
    ├── json_op.rs        # JSON Pretty-print/Minify
    └── case_op.rs        # Lowercase, Uppercase, Reverse
```

### Design Pattern

The application follows a **Strategy + Pipeline** pattern:

1. **Operation Trait** (`src/operations/mod.rs`): All operations implement the `Operation` trait, which defines three methods:
   - `name()` - returns a unique identifier string
   - `description()` - human-readable explanation
   - `perform(input)` - takes a string input, returns `Result<OperationResult, String>`

2. **Pipeline** (`src/pipeline.rs`): Chains multiple operations together. Each operation receives the output of the previous one as input, enabling complex transformations with a single command.

3. **CLI** (`src/cli.rs`): Uses `clap` for argument parsing. Supports three subcommands:
   - `op` - Run a single operation
   - `run` - Run a pipeline (comma-separated operations)
   - `list` - List all available operations

4. **Factory** (`cli.rs`): The `create_single_operation()` function acts as a factory, instantiating the correct operation struct based on the operation name string.

### Data Flow

```
User Input (CLI arg or stdin)
    → CLI parser (clap)
    → Single operation: create_single_operation() + perform()
    → Pipeline: create_operations() + Pipeline::run()
    → Output to stdout
    → Metadata to stderr (operation description, step info)
```

## Implemented Features (23 operations)

| # | Operation | Category | Description |
|---|-----------|----------|-------------|
| 1 | `base64-encode` | Encoding | Encode to Base64 |
| 2 | `base64-decode` | Decoding | Decode from Base64 |
| 3 | `hex-encode` | Encoding | Encode to hexadecimal |
| 4 | `hex-decode` | Decoding | Decode from hexadecimal |
| 5 | `url-encode` | Encoding | URL percent-encoding |
| 6 | `url-decode` | Decoding | URL percent-decoding |
| 7 | `rot13` | Transformation | ROT13 Caesar cipher |
| 8 | `xor` | Transformation | XOR with a key |
| 9 | `md5` | Hashing | MD5 hash |
| 10 | `sha1` | Hashing | SHA-1 hash |
| 11 | `sha256` | Hashing | SHA-256 hash |
| 12 | `extract-ips` | Extraction | Extract IPv4 addresses |
| 13 | `extract-urls` | Extraction | Extract URLs |
| 14 | `extract-emails` | Extraction | Extract email addresses |
| 15 | `text-stats` | Analysis | Character/word/line/byte counts |
| 16 | `entropy` | Analysis | Shannon entropy estimation |
| 17 | `binary-encode` | Encoding | Encode to binary bits |
| 18 | `binary-decode` | Decoding | Decode from binary bits |
| 19 | `json-pretty` | Formatting | Pretty-print JSON |
| 20 | `json-minify` | Formatting | Minify JSON |
| 21 | `lower-case` | Transformation | Convert to lowercase |
| 22 | `upper-case` | Transformation | Convert to uppercase |
| 23 | `reverse` | Transformation | Reverse string |

## Design Choices

### Why Rust?
- Memory safety without garbage collection
- Excellent performance for data processing
- Strong type system enables robust error handling
- `clap` provides ergonomic CLI argument parsing

### Why `trait` + `Box<dyn Operation>`?
- Enables the pipeline pattern by storing heterogeneous operations in a `Vec`
- Each operation is self-contained and independently testable
- New operations can be added without modifying existing code (Open/Closed principle)

### Operation Naming Convention
- Lowercase, hyphen-separated names (e.g., `base64-encode`)
- Consistent with common CLI tool conventions

### Output Convention
- Main output goes to **stdout** (pipeable)
- Metadata/descriptions go to **stderr** (doesn't interfere with piping)
- Errors go to **stderr** with exit code 1

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.6 | CLI argument parsing with derive macros |
| `base64` | 0.22 | Base64 encoding/decoding |
| `hex` | 0.4 | Hexadecimal encoding/decoding |
| `urlencoding` | 2 | URL percent-encoding/decoding |
| `sha-1` | 0.10 | SHA-1 hashing |
| `sha2` | 0.10 | SHA-256 hashing |
| `md-5` | 0.10 | MD5 hashing |
| `serde_json` | 1 | JSON parsing and formatting |
| `regex` | 1 | Pattern matching for extraction operations |
| `itertools` | 0.13 | Iterator utilities |
| `assert_cmd` | 2 (dev) | Integration testing |
| `predicates` | 3 (dev) | Test assertion predicates |
| `tempfile` | 3 (dev) | Temporary file testing |

## Limitations

1. **Binary data**: The tool operates primarily on UTF-8 text. Binary data is converted via `String::from_utf8_lossy()` which may lose data.
2. **Large inputs**: All operations load the entire input into memory - not suitable for very large files (streaming is not implemented).
3. **XOR complexity**: XOR key is passed as a single argument - no key file support.
4. **Extraction accuracy**: Email and URL extraction use simplified regex patterns that may miss some edge cases.
5. **Encoding detection**: The tool does not auto-detect input encoding; assumes UTF-8.
6. **No GUI**: Unlike CyberChef, this is a pure CLI tool.
7. **No recipe save/load**: Pipeline recipes cannot be saved to files for reuse.

## Testing Strategy

### Unit Tests (69 tests)
- Each operation module has its own test suite
- Tests cover:
  - Normal operation (happy path)
  - Edge cases (empty input, special characters)
  - Error conditions (invalid input, missing arguments)
  - Round-trip verification (encode then decode returns original)
- Pipeline tests verify operation chaining
- CLI tests verify operation registration and validation

### Running Tests
```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # Show output
cargo test <test_name>        # Run specific test
```

### Clippy
The code passes `cargo clippy -- -D warnings` with zero warnings.

## Building

### Native Build
```bash
cargo build --release
./target/release/chef list
```

### Docker Build
```bash
docker build -t chef .
docker run --rm chef list
docker run --rm chef op base64-encode "hello"
echo "hello" | docker run --rm -i chef op base64-encode
```

## Future Enhancements

1. Support for binary file input/output
2. Additional hash algorithms (SHA-512, BLAKE3)
3. Compression operations (gzip, deflate)
4. More extraction patterns (phone numbers, credit cards, MAC addresses)
5. Regular expression find/replace
6. Recipe save/load from YAML or JSON files
7. Streaming support for large files
8. Auto-detection of encoding schemes (like CyberChef's "Magic" operation)
