test-lib:
	cargo +nightly test --lib

test-integration:
	cargo +nightly test --test integration

test: test-lib test-integration
	@echo "All tests completed"

build:
	cargo +nightly build

release:
	cargo +nightly build --release

fmt:
	cargo +nightly fmt

clean:
	cargo clean