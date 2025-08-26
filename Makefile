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
	@echo "  make run-06        - Run example 06 (demo mode)"
	@echo "  make run-06-live   - Run example 06 (live connection)"
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
	@echo "  06 - Polkadot Header Subscription"
	@echo ""
	@echo "Run with: make run-01, make run-02, make run-03, make run-04, make run-05, make run-06, or make run-06-live"

# Run individual examples
run-01:
	@echo "Running Example 01: Endianness Conversion"
	@echo "========================================"
	cargo run -p scale-examples --example 01_endianness_conversion

run-02:
	@echo "Running Example 02: SCALE Compact Encoding"
	@echo "========================================="
	cargo run -p scale-examples --example 02_scale_compact_encoding

run-03:
	@echo "Running Example 03: SCALE Enum Encoding"
	@echo "======================================"
	cargo run -p scale-examples --example 03_scale_enum_encoding

run-04:
	@echo "Running Example 04: SCALE Vector Encoding"
	@echo "========================================"
	cargo run -p scale-examples --example 04_scale_vector_encoding

run-05:
	@echo "Running Example 05: SCALE Array Encoding"
	@echo "======================================="
	cargo run -p scale-examples --example 05_scale_array_encoding

run-06:
	@echo "Running Example 06: Polkadot Header Subscription (Demo Mode)"
	@echo "=========================================================="
	DEMO_MODE=true cargo run -p json-rpc-examples --example 01_polkadot_header_subscription

run-06-live:
	@echo "Running Example 06: Polkadot Header Subscription (Live Mode)"
	@echo "=========================================================="
	DEMO_MODE=false cargo run -p json-rpc-examples --example 01_polkadot_header_subscription

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
	@make run-06
	@echo ""
	@echo "All examples completed!"

# Check that all examples compile
check:
	@echo "Checking all examples compile..."
	cargo check --workspace

# Build all examples
build:
	@echo "Building all examples..."
	cargo build --workspace

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
	cargo clippy --workspace

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
