build:
	wasm-pack build --out-name mod --target web --release -- --features wee_alloc

run: build
	deno run --allow-read run.ts