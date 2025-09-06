# Install wasm target and trunk, then run web dev server
web:
	rustup target add wasm32-unknown-unknown
	cargo install trunk --locked || true
	trunk serve

# Run desktop app with Tauri
desktop:
	npm install
	cargo tauri dev

# Run crate tests
test:
	cargo test -p bert

# Format Rust code
fmt:
	cargo fmt --all

# Lint with clippy as errors
lint:
	cargo clippy --all-targets -- -D warnings