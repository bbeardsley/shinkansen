# List all the recipes
default:
	@just --list

# removes the build files in target/
clean:
	rm -rf target/

# build production release
build:
	cargo build --release

# build development release
build-dev:
	cargo build

# format the source code
format:
	cargo fmt

# check for common coding errors
check:
	cargo fmt --check && cargo clippy

# fix common problems
fix:
	cargo fix

# run the tests
test:
	cargo test

# install the binary locally
install:
	cargo install --path .
