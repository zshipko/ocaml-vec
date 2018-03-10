build:
	cargo build
	jbuilder build

test: build
	jbuilder runtest

