# Adding New Examples - Quick Reference

## Complete Checklist

### ✅ Step 1: Create Example File
```bash
# Create: examples/<TOPIC>/XX_your_example_name.rs
# For SCALE examples: examples/SCALE/XX_your_example_name.rs
# For other topics: examples/<TOPIC>/XX_your_example_name.rs
mkdir -p examples/<TOPIC>
touch examples/<TOPIC>/XX_your_example_name.rs
```

**Template:**
```rust
// Example XX: [Your Example Name]
// Brief description of what this example demonstrates

fn main() {
    // Your implementation here
}
```

### ✅ Step 2: Update Cargo.toml
```toml
[[example]]
name = "XX_your_example_name"
path = "examples/<TOPIC>/XX_your_example_name.rs"
```

### ✅ Step 3: Update Makefile
```makefile
# Add to list target:
@echo "  XX - [Your Example Name]"

# Add new run target:
run-XX:
	@echo "Running Example XX: [Your Example Name]"
	@echo "======================================"
	cargo run --example XX_your_example_name

# Add to run-all target:
@make run-XX
```

### ✅ Step 4: Update README.md
```markdown
### XX. [Your Example Name] (`examples/<TOPIC>/XX_your_example_name.rs`)
- **Description**: Brief description of what the example demonstrates
- **Key Concepts**: 
  - Concept 1
  - Concept 2
- **Run with**: `cargo run --example XX_your_example_name`
```

### ✅ Step 5: Test Everything
```bash
make check          # Verify compilation
make run-XX         # Test new example
make run-all        # Test all examples
make list           # Verify listing
```

### ✅ Step 6: Commit Changes
```bash
git add .
git commit -m "Add example XX: [Your Example Name]"
```

## File Locations to Update

1. **`examples/<TOPIC>/XX_your_example_name.rs`** - Your example code
2. **`Cargo.toml`** - Add `[[example]]` section
3. **`Makefile`** - Add `run-XX` target and update `list`/`run-all`
4. **`README.md`** - Add to Examples List section

## Directory Structure

Examples are organized by topic in subdirectories:

```
examples/
├── SCALE/                    # SCALE encoding examples
│   ├── 01_endianness_conversion.rs
│   ├── 02_scale_compact_encoding.rs
│   ├── 03_scale_enum_encoding.rs
│   ├── 04_scale_vector_encoding.rs
│   └── 05_scale_array_encoding.rs
├── <TOPIC>/                  # Other topic examples
│   ├── XX_example_name.rs
│   └── YY_another_example.rs
└── ...
```

## Naming Convention

- **Directory**: `examples/<TOPIC>/` (e.g., `examples/SCALE/`)
- **File**: `XX_descriptive_name.rs` (XX = two-digit number)
- **Cargo name**: `XX_descriptive_name` (same as filename without .rs)
- **Make target**: `run-XX`
- **Display name**: `XX - [Descriptive Name]`

## Example Numbers Used

### SCALE Topic
- 01: Endianness Conversion
- 02: SCALE Compact Encoding
- 03: SCALE Enum Encoding
- 04: SCALE Vector Encoding
- 05: SCALE Array Encoding

### Other Topics
- Use appropriate numbering for each topic (e.g., 01, 02, 03...)

## Quick Commands

```bash
# Create and edit new example
mkdir -p examples/<TOPIC>
touch examples/<TOPIC>/XX_your_example_name.rs
code examples/<TOPIC>/XX_your_example_name.rs

# Test your changes
make check && make run-XX && make list

# Add to git
git add . && git commit -m "Add example XX: [Your Example Name]"
```

## Topic Organization Guidelines

- **SCALE**: All SCALE encoding related examples
- **Async**: Asynchronous programming examples
- **Error**: Error handling patterns
- **Macro**: Macro system examples
- **FFI**: Foreign Function Interface examples
- **Web**: Web development examples
- **CLI**: Command Line Interface examples
- **Data**: Data structures and algorithms
- **Concurrency**: Threading and concurrency examples
- **Testing**: Testing and mocking examples

Create new topic directories as needed for different categories of examples.
