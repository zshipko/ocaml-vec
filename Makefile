build:
	cargo build --release
	jbuilder build

test: build
	jbuilder runtest

