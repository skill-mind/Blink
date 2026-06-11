.PHONY: fmt fmt-check lint test build clean

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Lint code with Clippy
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	cargo test --all-features

# Build contracts
build:
	cargo build --target wasm32-unknown-unknown --release

# Run all CI checks
ci: fmt-check lint test build

# Clean
clean:
	cargo clean