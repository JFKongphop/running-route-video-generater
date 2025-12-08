# Runarium Makefile

.PHONY: help build run clean test examples video image check fmt

help:
	@echo "Available commands:"
	@echo "  make build        - Build the project"
	@echo "  make run          - Run the main application"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make test         - Run tests"
	@echo "  make examples     - Build all examples"
	@echo "  make video        - Run video generation example"
	@echo "  make image        - Run image generation example"
	@echo "  make check        - Run cargo check"
	@echo "  make fmt          - Format code"

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

test:
	cargo test

examples:
	cargo build --examples

video:
	cargo run --example video_config

image:
	cargo run --example image_config

check:
	cargo check

fmt:
	cargo fmt
