.PHONY: all build test docker-build

all: build test

build:
	cargo build

test:
	cargo test

docker-build:
	sudo docker build -t something .
