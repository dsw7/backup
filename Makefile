.PHONY = lint install

lint:
	@cargo fmt
	@cargo check
	@cargo clippy

install:
	@cargo install --path . --root ~/.local
