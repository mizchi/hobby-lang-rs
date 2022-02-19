build:
	wasm-pack build --out-name mod --target web --release -- --features console_error_panic_hook

build-release:
	wasm-pack build --out-name mod --target web -- --features wee_alloc

test: build
	wasm-pack test --node
	deno test --allow-read

run: build
	deno run --allow-read run.ts
