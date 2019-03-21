arch ?= mipsel

.PHONY: build

build:
	cargo xbuild --target=targets/${arch}.json
