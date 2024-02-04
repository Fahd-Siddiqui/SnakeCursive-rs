# Makefile

# Commands
.PHONY: lint test

lint:
	@echo "Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo "Running tests..."
	@cargo test
