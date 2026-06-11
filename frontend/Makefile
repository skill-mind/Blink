.PHONY: install fmt lint test build clean

# Install dependencies
install:
	npm ci

# Format code
fmt:
	npm run format

# Check formatting
fmt-check:
	npm run format:check

# Lint code
lint:
	npm run lint

# Run tests
test:
	npm run test:ci

# Type check
type-check:
	npm run type-check

# Build (compile TypeScript)
build: type-check

# Run all CI checks
ci: fmt-check lint test build

# Clean
clean:
	rm -rf node_modules
	rm -rf coverage