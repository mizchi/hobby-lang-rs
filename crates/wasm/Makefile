build:
	wasm-pack build --out-name mod --target web --release -- --features wee_alloc

test:
	wasm-pack test --node

run: build
	deno run --allow-read run.ts
