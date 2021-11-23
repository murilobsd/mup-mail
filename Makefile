build:
		@cargo build

clean:
		@cargo clean

TESTS = ""
test:
		@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
		@cargo doc --no-deps

style-check:
		cargo fmt --all -- --check

lint:
		cargo clippy --all-targets --all-features -- -D warnings

dev:
		cargo run

.PHONY: build test docs style-check lint 
