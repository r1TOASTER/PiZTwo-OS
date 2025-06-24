test:
	cargo +nightly build --test integration
	qemu-system-aarch64 -M raspi3b -kernel target/aarch64-unknown-none/debug/piztwo-os -serial stdio -display none

build:
	cargo +nightly build

release:
	cargo +nightly build --release

run:
	cargo +nightly run

rr:
	cargo +nightly run --release

fmt:
	cargo +nightly fmt

clean:
	cargo clean