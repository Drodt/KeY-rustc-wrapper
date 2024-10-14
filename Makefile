# Source: https://github.com/sureshg/java-rust-ffi/blob/master/rust/Makefile

EXT := a

all: target/librust_wrapper.$(EXT)

target/librust_wrapper.$(EXT): src/lib.rs Cargo.toml
	RUSTC_BOOTSTRAP=1 CFG_RELEASE="1.84.0" CFG_RELEASE_CHANNEL=nightly RUSTC_INSTALL_BINDIR="bin" cargo build
	(cd target && ln -nsf debug/librust_wrapper.$(EXT) librust_wrapper.$(EXT))

clean:
	rm -rf target