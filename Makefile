install:
	cargo build --release && cp target/release/rost /usr/bin/rost

test:
	cargo test
