.PHONY: all build test docker-build

all: build test

build:
	cargo build

test:
	cargo test

docker-build:
	docker build -t my-rust-app .