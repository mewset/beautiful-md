.PHONY: check test fmt clippy build release clean help

# Default target
help:
	@echo "Available targets:"
	@echo "  check    - Run all CI checks locally"
	@echo "  test     - Run tests"
	@echo "  fmt      - Format code"
	@echo "  clippy   - Run clippy lints"
	@echo "  build    - Build debug binary"
	@echo "  release  - Build release binary"
	@echo "  clean    - Clean build artifacts"

# Run all CI checks
check:
	@bash scripts/check.sh

# Run tests
test:
	cargo test --verbose

# Format code
fmt:
	cargo fmt --all

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Build debug
build:
	cargo build

# Build release
release:
	cargo build --release

# Clean
clean:
	cargo clean
