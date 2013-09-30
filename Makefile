OUTDIR=build

all: demo

demo: clean blue
	rustc src/examples/demo.rs -L lib/rust-http/build/ -L build/blue --out-dir=$(OUTDIR)/examples

blue: src/blue/lib.rs
	rustc src/blue/lib.rs -L lib/rust-http/build --out-dir=$(OUTDIR)/blue

http: http-build
	rustc --lib lib/rust-http/src/libhttp/lib.rs

http-build: http-repo
	cd lib/rust-http && make && cd ../..

http-repo:
	test -d lib/rust-http || git clone git@github.com:chris-morgan/rust-http lib/rust-http

clean:
	rm -rf build/blue/*

