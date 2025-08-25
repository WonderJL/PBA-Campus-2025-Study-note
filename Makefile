# PBA Campus 2025 Study Notes - Rust Examples Makefile
# Provides convenient commands for managing and running examples

.PHONY: help list run clean check build test all

# Default target
help:
	@echo "PBA Campus 2025 Study Notes - Rust Examples"
	@echo "=========================================="
	@echo ""
	@echo "Available commands:"
	@echo "  make list          - List all available examples"
	@echo "  make run-01        - Run example 01 (endianness conversion)"
	@echo "  make run-XX        - Run example XX (replace XX with example number)"
	@echo "  make run-all       - Run all examples sequentially"
	@echo "  make check         - Check all examples compile without running"
	@echo "  make build         - Build all examples"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make test          - Run all examples as tests"
	@echo ""

# List all available examples
list:
	@echo "Available examples:"
	@echo "  01 - Endianness Conversion"
	@echo "  02 - SCALE Compact Encoding"
	@echo "  03 - SCALE Enum Encoding"
	@echo "  04 - SCALE Vector Encoding"
	@echo "  05 - SCALE Array Encoding"
	@echo ""
	@echo "Run with: make run-01, make run-02, make run-03, make run-04 or make run-05"

# Run individual examples
run-01:
	@echo "Running Example 01: Endianness Conversion"
	@echo "========================================"
	cargo run --example 01_endianness_conversion

run-02:
	@echo "Running Example 02: SCALE Compact Encoding"
	@echo "========================================="
	cargo run --example 02_scale_compact_encoding

run-03:
	@echo "Running Example 03: SCALE Enum Encoding"
	@echo "======================================"
	cargo run --example 03_scale_enum_encoding

run-04:
	@echo "Running Example 04: SCALE Vector Encoding"
	@echo "========================================"
	cargo run --example 04_scale_vector_encoding

run-05:
	@echo "Running Example 05: SCALE Array Encoding"
	@echo "======================================="
	cargo run --example 05_scale_array_encoding

# Template for adding more examples (uncomment and modify as needed)
# run-06:
# 	@echo "Running Example 06: [Example Name]"
# 	@echo "================================"
# 	cargo run --example 06_example_name

# run-03:
# 	@echo "Running Example 03: [Example Name]"
# 	@echo "================================"
# 	cargo run --example 03_example_name

# Run all examples sequentially
run-all:
	@echo "Running all examples..."
	@echo "======================"
	@make run-01
	@echo ""
	@make run-02
	@echo ""
	@make run-03
	@echo ""
	@make run-04
	@echo ""
	@make run-05
	@echo ""
	@echo "All examples completed!"

# Check that all examples compile
check:
	@echo "Checking all examples compile..."
	cargo check --examples

# Build all examples
build:
	@echo "Building all examples..."
	cargo build --examples

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run all examples as tests (useful for CI/CD)
test:
	@echo "Running all examples as tests..."
	@make run-all

# Development helpers
dev-setup:
	@echo "Setting up development environment..."
	@echo "Installing cargo-watch for development..."
	cargo install cargo-watch || echo "cargo-watch already installed or failed to install"

# Watch mode for development (requires cargo-watch)
watch:
	@echo "Watching for changes in examples..."
	cargo watch -x "run --example 01_endianness_conversion"

# Format all example files
fmt:
	@echo "Formatting all Rust files..."
	cargo fmt

# Lint all example files
lint:
	@echo "Linting all Rust files..."
	cargo clippy --examples

# Quick validation
validate: check fmt lint
	@echo "All validation checks passed!"

# Show project info
info:
	@echo "Project Information:"
	@echo "==================="
	@echo "Project: $(shell cargo pkgid | cut -d# -f1)"
	@echo "Version: $(shell cargo pkgid | cut -d# -f2)"
	@echo "Rust Edition: $(shell grep 'edition =' Cargo.toml | cut -d'"' -f2)"
	@echo "Examples Directory: examples/"
	@echo "Total Examples: $(shell ls examples/*.rs 2>/dev/null | wc -l | tr -d ' ')"
