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

rustup-add-targets:
	rustup target add x86_64-unknown-linux-gnu armv7-unknown-linux-gnueabihf x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-gnu

# build release executables
release-executables:
	rm -f target/shinkansen-*.tar.gz
	rm -f target/shinkansen-*.zip

	# cargo install cross --git https://github.com/cross-rs/cross.git
	cross build --target x86_64-unknown-linux-gnu --release
	tar -czvf target/shinkansen-linux-amd64.tar.gz target/x86_64-unknown-linux-gnu/release/shinkansen

	cross build --target armv7-unknown-linux-gnueabihf --release
	tar -czvf target/shinkansen-linux-arm64.tar.gz target/armv7-unknown-linux-gnueabihf/release/shinkansen

	cargo build --target x86_64-apple-darwin --release
	tar -czvf target/shinkansen-darwin-amd64.tar.gz target/x86_64-apple-darwin/release/shinkansen

	cargo build --target aarch64-apple-darwin --release
	tar -czvf target/shinkansen-darwin-arm64.tar.gz target/aarch64-apple-darwin/release/shinkansen

	cross build --target x86_64-pc-windows-gnu --release
	zip -j target/shinkansen-windows-amd64.zip target/x86_64-pc-windows-gnu/release/shinkansen.exe

# create a github release for version (without the v)
release-create VERSION: release-executables
	gh release create v{{VERSION}} \
		target/shinkansen-linux-amd64.tar.gz \
		target/shinkansen-linux-arm64.tar.gz \
		target/shinkansen-darwin-amd64.tar.gz \
		target/shinkansen-darwin-arm64.tar.gz \
		target/shinkansen-windows-amd64.zip \
		--title "v{{VERSION}}" \
		--notes "Release v{{VERSION}}"

# show the shas for the release assets for the specified version (without the v)
release-shas VERSION:
	@gh release view v{{VERSION}} --json assets --jq '.assets | map("\(.name),\(.digest)")|.[]' | sed "s/sha256://"
