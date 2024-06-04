# dlprotoc: Download protoc for Cargo build scripts

[![Crates.io Version](https://img.shields.io/crates/v/dlprotoc)](https://crates.io/crates/dlprotoc)
[![Docs.rs Link](https://img.shields.io/docsrs/dlprotoc)](https://docs.rs/dlprotoc/latest/dlprotoc/)

This crate downloads the
[official binary releases of protoc from Google's protobuf Github repo](https://github.com/protocolbuffers/protobuf),
verifies a SHA256 hash, then extracts it. It easy to use with Prost or Tonic.

This fixes Cargo errors like the following:

```
error: failed to run custom build command for `example v0.1.0 (/home/user/example)`

Caused by:
  process didn't exit successfully: `target/debug/build/example-3f5090329e3c4e4b/build-script-build` (exit status: 1)
  --- stderr
  Error: Custom { kind: NotFound, error: "Could not find `protoc`. If `protoc` is installed, try setting the `PROTOC` environment variable to the path of the `protoc` binary. To install it on Debian, run `apt-get install protobuf-compiler`. It is also available at https://github.com/protocolbuffers/protobuf/releases  For more information: https://docs.rs/prost-build/#sourcing-protoc" }
```

An alternative is the [protobuf-src crate](https://crates.io/crates/protobuf-src), which compiles protoc from source. Unfortunately, compiling protoc is quite slow (approximately 2 minutes on my 4 core Intel desktop from 2020), and has more dependencies. Notably: `protobuf-src` requires `cmake` and a C++ compiler.


## Quick Start

In your `build.rs`, call `dlprotoc::download_protoc()` before calling `compile_protos(...)`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dlprotoc::download_protoc()?;
    prost_build::compile_protos(&["src/example.proto"], &["src/"])?;
    Ok(())
}
```

For a complete example, see [protoc-cargo-example-rs](https://github.com/evanj/protoc-cargo-example-rs).


## Trust/Security

This downloads pre-compiled executables on Github, which is somewhat dangerous. You need to trust:

* The author of this crate (me! Evan Jones)
  * I manually add SHA256 hashes for protoc releases, to allow them to be downloaded.
  * I wrote this crate. Let's hope it is mostly free from bugs
* Crates.io: To give you a non-malicious version of this crate.
* Github: To provide the protoc binaries that Google uploaded.
* Google Protobuf maintainers: To upload non-malicious protoc binaries to Github.


## Updating to new protoc releases (for maintainers)

1. Run: `cargo run -- (version e.g 27.0)`
2. Append the printed struct definitions into the `KNOWN_VERSIONS` array in `lib.rs`.
3. Run `make` to execute all checks.
4. Update the version in `Cargo.toml` to include the version of protoc. E.g. `"0.1.0+27.0"`.
6. Send a pull request.
