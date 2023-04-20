default: build

all: build test

test: build
	cargo test
	cargo test --features testutils

build:
	cargo build --target wasm32-unknown-unknown --profile release-with-logs

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
