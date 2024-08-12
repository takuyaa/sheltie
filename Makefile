.PHONY: clean
clean:
	@cargo clean

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: lint
lint:
	@cargo fmt --all --check

.PHONY: format
format:
	@cargo fmt --all

.PHONY: all
all: clean lint build test
