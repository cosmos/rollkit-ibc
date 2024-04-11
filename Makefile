.PHONY: help

help: ## Display this help message
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

clean: ## Cleans compiled
	@cargo clean
	
install-dev-tools:  ## Installs all necessary cargo helpers
	cargo install wasm-opt

build: ## Build the the entire project
	@cargo build

build-cw: ## Build the WASM file for the rollkit light client
	@echo "Building the WASM file for the rollkit light client"
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release --lib --locked
	@mkdir -p contracts
	@cp target/wasm32-unknown-unknown/release/rollkit_ibc.wasm contracts/

optimize-contracts: ## Optimize WASM files in contracts directory
	@echo "Optimizing WASM files..."
	@for wasm_file in contracts/*.wasm; do \
		optimized_file="contracts/$$(basename $$wasm_file .wasm).opt.wasm"; \
		wasm-opt "$$wasm_file" -o "$$optimized_file" -Os; \
	done

lint:  ## cargo check and clippy. Skip clippy on guest code since it's not supported by risc0
	## fmt first, because it's the cheapest
	cargo +nightly fmt --all --check
	cargo check --all-targets --all-features
	CI_SKIP_GUEST_BUILD=1 cargo clippy --all-targets --all-features

lint-fix:  ## cargo fmt, fix and clippy. Skip clippy on guest code since it's not supported by risc0
	cargo +nightly fmt --all
	cargo fix --allow-dirty
	CI_SKIP_GUEST_BUILD=1 cargo clippy --fix --allow-dirty

find-unused-deps: ## Prints unused dependencies for project. Note: requires nightly
	cargo udeps --all-targets --all-features

check-features: ## Checks that project compiles with all combinations of features. default is not needed because we never check `cfg(default)`, we only use it as an alias.
	cargo hack check --workspace --feature-powerset --exclude-features default

test: ## Run tests with all features and without default features.
	@cargo test --all-targets --no-default-features

docs:  ## Generates documentation locally
	cargo doc --all-features --no-deps --release --open
