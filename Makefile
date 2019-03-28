arch ?= mipsel
mode ?= debug

target := $(arch)

build_args := --target targets/$(target).json --features "$(features)"

ifeq ($(mode), release)
build_args += --release
endif

.PHONY: build

build:
	cargo xbuild $(build_args)

clean:
	rm -rf target/${arch}

dist-clean:
	rm -rf target
