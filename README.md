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

## Adding New Examples - Step-by-Step Checklist

When adding a new example, follow these steps in order:

### 1. Create the Example File
```bash
# Create new example file in examples/ directory
# Use format: XX_descriptive_name.rs (XX = two-digit number)
touch examples/03_your_example_name.rs
```

**File Structure:**
```rust
// Example XX: [Descriptive Name]
// Brief description of what this example demonstrates
// Key concepts and learning objectives

// Optional: Add any necessary imports
// use std::...

// Your example code here
fn main() {
    // Implementation
}
```

### 2. Update Cargo.toml
Add the new example to the `[[example]]` section:
```toml
[[example]]
name = "03_your_example_name"
path = "examples/03_your_example_name.rs"
```

### 3. Update Makefile
Add a new `run-XX` target:
```makefile
run-03:
	@echo "Running Example 03: [Your Example Name]"
	@echo "======================================"
	cargo run --example 03_your_example_name
```

**Also update:**
- **`list` target**: Add the new example to the list
- **`run-all` target**: Add `@make run-03` to run all examples

### 4. Update README.md
Add the new example to the "Examples List" section:
```markdown
### 3. [Your Example Name] (`examples/03_your_example_name.rs`)
- **Description**: Brief description of what the example demonstrates
- **Key Concepts**: 
  - Concept 1
  - Concept 2
  - Concept 3
- **Run with**: `cargo run --example 03_your_example_name`
```

### 5. Test Your Changes
```bash
# Check that everything compiles
make check

# Test the new example
make run-03

# Test all examples still work
make run-all

# Verify the list shows your new example
make list
```

### 6. Optional: Add Dependencies
If your example needs external crates, add them to `Cargo.toml`:
```toml
[dependencies]
your_crate = "1.0.0"
```

### 7. Commit Your Changes
```bash
git add .
git commit -m "Add example 03: [Your Example Name]"
```

## Quick Reference Template

**File Name:** `examples/03_your_example_name.rs`

**Cargo.toml Addition:**
```toml
[[example]]
name = "03_your_example_name"
path = "examples/03_your_example_name.rs"
```

**Makefile Addition:**
```makefile
run-03:
	@echo "Running Example 03: [Your Example Name]"
	@echo "======================================"
	cargo run --example 03_your_example_name
```

**README.md Addition:**
```markdown
### 3. [Your Example Name] (`examples/03_your_example_name.rs`)
- **Description**: [Description]
- **Key Concepts**: 
  - [Concept 1]
  - [Concept 2]
- **Run with**: `cargo run --example 03_your_example_name`
```

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
