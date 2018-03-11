build:
	cargo build --release
	jbuilder build

test: build
	jbuilder runtest

clean:
	rm -f src/*.a src/*.so
	cargo clean
	jbuilder clean

