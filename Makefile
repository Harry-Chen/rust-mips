arch ?= mipsel

.PHONY: build

build:
	cargo xbuild --target=targets/${arch}.json

clean:
	rm -rf target/${arch}

dist-clean:
	rm -rf target
