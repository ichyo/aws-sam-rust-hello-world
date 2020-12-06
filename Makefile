build:
	sam build

build-HelloWorldFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/hello $(ARTIFACTS_DIR)/bootstrap
