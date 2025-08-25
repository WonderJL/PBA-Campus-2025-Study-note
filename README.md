# PBA Campus 2025 Study Notes - Rust Examples

This repository contains various Rust examples for learning and reference purposes. Each example demonstrates different concepts and features of the Rust programming language.

## Examples List

### 1. Endianness Conversion (`examples/01_endianness_conversion.rs`)
- **Description**: Demonstrates how to convert between different byte orderings (endianness) in Rust using built-in methods.
- **Key Concepts**: 
  - `to_le_bytes()` - Convert to little-endian
  - `to_be_bytes()` - Convert to big-endian  
  - `to_ne_bytes()` - Convert to native-endian
- **Run with**: `cargo run --example 01_endianness_conversion`

### 2. SCALE Compact Encoding (`examples/02_scale_compact_encoding.rs`)
- **Description**: Implements the SCALE Compact encoding scheme for efficient integer storage using variable-length encoding.
- **Key Concepts**: 
  - Variable-length encoding based on value magnitude
  - Bit manipulation and byte-level operations
  - Error handling for malformed data
  - Binary representation analysis
- **Run with**: `cargo run --example 02_scale_compact_encoding`

## How to Run Examples

### Method 1: Using Makefile (Recommended)
```bash
# Show all available commands
make help

# List all examples
make list

# Run a specific example
make run-01

# Run all examples
make run-all

# Check that all examples compile
make check
```

### Method 2: Using Cargo Examples
```bash
# Run a specific example
cargo run --example 01_endianness_conversion

# List all available examples
cargo run --example
```

### Method 3: Direct Compilation
```bash
# Compile and run directly
rustc examples/01_endianness_conversion.rs -o endianness_example
./endianness_example
```

### Method 4: Using Cargo Run
```bash
# Copy the example content to src/main.rs temporarily
cp examples/01_endianness_conversion.rs src/main.rs
cargo run
```

## Project Structure

```
PBA-Campus-2025-Study-note/
├── Cargo.toml              # Project configuration
├── README.md              # This file
├── src/
│   └── main.rs           # Main entry point
└── examples/             # Example files
    └── 01_endianness_conversion.rs
```

## Adding New Examples

When adding new examples:

1. Create a new file in the `examples/` directory with a descriptive name
2. Use the format: `XX_descriptive_name.rs` where XX is a two-digit number
3. Include a header comment explaining what the example demonstrates
4. Add the example to `Cargo.toml` in the `[[example]]` section
5. Add a new `run-XX` target to the `Makefile`
6. Update this README.md file to include the new example in the list above

## Makefile Features

The included `Makefile` provides convenient commands for managing your examples:

- **`make help`** - Show all available commands
- **`make list`** - List all available examples
- **`make run-XX`** - Run a specific example (e.g., `make run-01`)
- **`make run-all`** - Run all examples sequentially
- **`make check`** - Verify all examples compile
- **`make build`** - Build all examples
- **`make clean`** - Clean build artifacts
- **`make fmt`** - Format all Rust files
- **`make lint`** - Lint all Rust files with clippy
- **`make validate`** - Run check, format, and lint
- **`make info`** - Show project information

## Requirements

- Rust 1.70+ (recommended)
- Cargo package manager

## License

This project is for educational purposes as part of PBA Campus 2025 study materials.
