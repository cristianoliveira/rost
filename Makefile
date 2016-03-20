install:
	cargo build --release && cp target/release/rost /usr/local/bin/rost

test:
	cargo test
