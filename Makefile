build-wasm:
	cd crates/wasm && make build

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check