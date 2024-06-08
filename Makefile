rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

build-release:
	@echo "Building release version for platfomr $(shell uname -s)"
	cargo build --release 

all: format lint test run
