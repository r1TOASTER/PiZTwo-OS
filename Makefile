test:
	cargo +nightly test

build:
	cargo +nightly build

release:
	cargo +nightly build --release

fmt:
	cargo +nightly fmt

clean:
	cargo clean