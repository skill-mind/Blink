.PHONY: fmt fmt-check lint test build clean ci frontend backend smartcontract

# Format all components
fmt:
	@echo "🎨 Formatting frontend..."
	cd frontend && make fmt
	@echo "🎨 Formatting backend..."
	cd backend && make fmt
	@echo "🎨 Formatting smart contracts..."
	cd smartcontract/blink-contracts && make fmt

# Check formatting for all components
fmt-check:
	@echo "✨ Checking frontend formatting..."
	cd frontend && make fmt-check
	@echo "✨ Checking backend formatting..."
	cd backend && make fmt-check
	@echo "✨ Checking smart contract formatting..."
	cd smartcontract/blink-contracts && make fmt-check

# Lint all components
lint:
	@echo "🔍 Linting frontend..."
	cd frontend && make lint
	@echo "🔍 Linting backend..."
	cd backend && make lint
	@echo "🔍 Linting smart contracts..."
	cd smartcontract/blink-contracts && make lint

# Test all components
test:
	@echo "🧪 Testing frontend..."
	cd frontend && make test
	@echo "🧪 Testing backend..."
	cd backend && make test
	@echo "🧪 Testing smart contracts..."
	cd smartcontract/blink-contracts && make test

# Build all components
build:
	@echo "🏗️ Building frontend..."
	cd frontend && make build
	@echo "🏗️ Building backend..."
	cd backend && make build
	@echo "🏗️ Building smart contracts..."
	cd smartcontract/blink-contracts && make build

# Run CI checks for all components
ci: fmt-check lint test build
	@echo "✅ All CI checks passed!"

# Component-specific targets
frontend:
	cd frontend && make ci

backend:
	cd backend && make ci

smartcontract:
	cd smartcontract/blink-contracts && make ci

# Clean all components
clean:
	@echo "🧹 Cleaning frontend..."
	cd frontend && make clean
	@echo "🧹 Cleaning backend..."
	cd backend && make clean
	@echo "🧹 Cleaning smart contracts..."
	cd smartcontract/blink-contracts && make clean

# Install dependencies
install:
	@echo "📦 Installing frontend dependencies..."
	cd frontend && make install

# Help
help:
	@echo "Available commands:"
	@echo "  make fmt       - Format all code"
	@echo "  make fmt-check - Check formatting"
	@echo "  make lint      - Lint all code"
	@echo "  make test      - Run all tests"
	@echo "  make build     - Build all components"
	@echo "  make ci        - Run full CI pipeline"
	@echo "  make clean     - Clean all build artifacts"
	@echo "  make install   - Install dependencies"
	@echo ""
	@echo "Component-specific:"
	@echo "  make frontend     - Run CI for frontend only"
	@echo "  make backend      - Run CI for backend only"
	@echo "  make smartcontract - Run CI for smart contracts only"