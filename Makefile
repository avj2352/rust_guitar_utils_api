help:
	@echo "Rust command list..."
	@echo "clean             -  Cleanup dependencies"
	@echo "compile           -  Create Rust build"

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

clean:
	@echo "remove rust dependencies..."
	rm -rf target
	rm -f Cargo.lock


tree:
	@echo "show project structure..."
	tree -L 3 -I target

compile:
	@echo "creating rust build..."
	cargo build

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	@echo 'Testing using http_client...'
	cargo test http_client -- --nocapture

server:
	@echo 'starting http server...'
	cargo run

release:
	cargo build --release

dev: format lint test run
deploy: format lint test release
