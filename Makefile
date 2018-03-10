build:
	cargo build --release
	cp target/release/libvec_stubs.a src/libvec_stubs.a
	cp target/release/libvec_stubs.so src/dllvec_stubs.so
	jbuilder build

test: build
	jbuilder runtest

clean:
	rm -f src/*.a src/*.so
	cargo clean
	jbuilder clean

