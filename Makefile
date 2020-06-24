all:
	@echo all

alpha:
	@export TARGET_CC=x86_64-linux-musl-gcc
	@export RUSTFLAGS="-C linker=x86_64-linux-musl-gcc"
	@cargo build --target=x86_64-unknown-linux-musl
	@docker build --build-arg RUST_ENV=alpha -t rust/hello-world:v1.0.0 .

alpha-run:
	@docker run -P --name test-rust-hello-world -it --rm rust/hello-world:v1.0.0  