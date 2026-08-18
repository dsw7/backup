.PHONY = lint

lint:
	@cargo fmt
	@cargo check
	@cargo clippy
