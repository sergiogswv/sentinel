# Sentinel Makefile
# Quick commands for common tasks

.PHONY: help install build test clean update run release

# Default target
help:
	@echo "🛡️  Sentinel - Available Commands"
	@echo ""
	@echo "  make install    - Install Sentinel (auto-detects OS)"
	@echo "  make build      - Build in debug mode"
	@echo "  make release    - Build in release mode"
	@echo "  make test       - Run tests"
	@echo "  make run        - Run Sentinel in debug mode"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make update     - Update Sentinel from git"
	@echo ""

# Install using the appropriate script
install:
ifeq ($(OS),Windows_NT)
	@echo "Installing on Windows..."
	@powershell -ExecutionPolicy Bypass -File install.ps1
else
	@echo "Installing on Unix-like system..."
	@chmod +x install.sh
	@./install.sh
endif

# Build commands
build:
	@echo "Building Sentinel (debug)..."
	cargo build

release:
	@echo "Building Sentinel (release)..."
	cargo build --release

# Test
test:
	@echo "Running tests..."
	cargo test

# Run
run:
	@echo "Running Sentinel..."
	cargo run

# Clean
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Update from git and rebuild
update:
	@echo "Updating Sentinel..."
	git pull origin master
	@echo "Rebuilding..."
	cargo build --release
	@echo "✅ Update complete!"
	@echo "Run: make install (to reinstall binary)"
