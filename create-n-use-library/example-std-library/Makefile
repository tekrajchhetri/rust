SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean
build: ## Build the project using cargo
	cargo build 

doc: ## Build the documentation using cargo
	cargo doc --no-deps --open

release: ## Build the project using cargo. Release version
	cargo build --release

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

format: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

test: ## Test the project using cargo
	cargo test
	
bump: ## Bump the version of the project 
	@echo "Current version: $$(cargo pkgid | cut -d# -f2)"
	@read -p "New version: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New updated version is $(shell cargo pkgid | cut -d# -f2)"