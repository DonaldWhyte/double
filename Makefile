default: test

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

.PHONY: clean run
