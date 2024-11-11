.PHONY: build run test fmt clippy

build:
cargo build

run:
cargo run -- parse http://example.com/products/electronics?query=1#reviews

test:
cargo test

fmt:
cargo fmt --all

clippy:
cargo clippy --all -- -D warnings

all: fmt clippy build test