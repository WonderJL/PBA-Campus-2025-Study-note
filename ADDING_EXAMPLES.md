# Adding New Examples - Quick Reference

## Complete Checklist

### ✅ Step 1: Create Example File
```bash
# Create: examples/03_your_example_name.rs
touch examples/03_your_example_name.rs
```

**Template:**
```rust
// Example 03: [Your Example Name]
// Brief description of what this example demonstrates

fn main() {
    // Your implementation here
}
```

### ✅ Step 2: Update Cargo.toml
```toml
[[example]]
name = "03_your_example_name"
path = "examples/03_your_example_name.rs"
```

### ✅ Step 3: Update Makefile
```makefile
# Add to list target:
@echo "  03 - [Your Example Name]"

# Add new run target:
run-03:
	@echo "Running Example 03: [Your Example Name]"
	@echo "======================================"
	cargo run --example 03_your_example_name

# Add to run-all target:
@make run-03
```

### ✅ Step 4: Update README.md
```markdown
### 3. [Your Example Name] (`examples/03_your_example_name.rs`)
- **Description**: Brief description of what the example demonstrates
- **Key Concepts**: 
  - Concept 1
  - Concept 2
- **Run with**: `cargo run --example 03_your_example_name`
```

### ✅ Step 5: Test Everything
```bash
make check          # Verify compilation
make run-03         # Test new example
make run-all        # Test all examples
make list           # Verify listing
```

### ✅ Step 6: Commit Changes
```bash
git add .
git commit -m "Add example 03: [Your Example Name]"
```

## File Locations to Update

1. **`examples/03_your_example_name.rs`** - Your example code
2. **`Cargo.toml`** - Add `[[example]]` section
3. **`Makefile`** - Add `run-03` target and update `list`/`run-all`
4. **`README.md`** - Add to Examples List section

## Naming Convention

- **File**: `XX_descriptive_name.rs` (XX = two-digit number)
- **Cargo name**: `XX_descriptive_name` (same as filename without .rs)
- **Make target**: `run-XX`
- **Display name**: `XX - [Descriptive Name]`

## Example Numbers Used

- 01: Endianness Conversion
- 02: SCALE Compact Encoding
- 03: [Your next example]
- 04: [Future example]
- ...

## Quick Commands

```bash
# Create and edit new example
touch examples/03_your_example_name.rs
code examples/03_your_example_name.rs

# Test your changes
make check && make run-03 && make list

# Add to git
git add . && git commit -m "Add example 03: [Your Example Name]"
```
