# Makefile for a Rust project

.PHONY: build format

format:
	cargo fmt

build: format
	cargo build

run: format
	cargo run