init:
	./scripts/init.sh

check:
	SKIP_WASM_BUILD= cargo check

test:
	SKIP_WASM_BUILD= cargo test --all

run:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- --dev --tmp

build:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
