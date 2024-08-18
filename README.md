# dlprotoc: Download protoc for Cargo build scripts

[![Crates.io Version](https://img.shields.io/crates/v/dlprotoc)](https://crates.io/crates/dlprotoc)
[![Docs.rs Link](https://img.shields.io/docsrs/dlprotoc)](https://docs.rs/dlprotoc/latest/dlprotoc/)

This crate downloads the
[official binary releases of `protoc` from Google's protobuf Github repo](https://github.com/protocolbuffers/protobuf),
verifies a SHA256 hash, then extracts it. It is intended to be used in Cargo build scripts (`build.rs`) with Prost or Tonic, so you don't need to have `protoc` installed to build Rust projects that use Protocol Buffers.

This fixes Cargo errors like the following:

```
error: failed to run custom build command for `example v0.1.0 (/home/user/example)`

Caused by:
  process didn't exit successfully: `target/debug/build/example-3f5090329e3c4e4b/build-script-build` (exit status: 1)
  --- stderr
  Error: Custom { kind: NotFound, error: "Could not find `protoc`. If `protoc` is installed, try setting the `PROTOC` environment variable to the path of the `protoc` binary. To install it on Debian, run `apt-get install protobuf-compiler`. It is also available at https://github.com/protocolbuffers/protobuf/releases  For more information: https://docs.rs/prost-build/#sourcing-protoc" }
```

An alternative is the [protobuf-src crate](https://crates.io/crates/protobuf-src), which compiles protoc from source. Unfortunately, compiling protoc is quite slow (approximately 2 minutes on my 4 core Intel desktop from 2020), and requires `cmake` and a C++ compiler. This crate only requires Rust.


## Quick Start

Add `dlprotoc` to
In `build.rs`, call `dlprotoc::download_protoc()` before calling `compile_protos(...)`:

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

* The SHA256 hashes embedded in the source code are for protoc binaries that are non-malicious. The hashes are from Google's Github releases.
* The crate does not have bugs, and will not run protoc binaries that have not been seen before.
* Crates.io: Must give you a non-malicious version of this crate.
* Google Protobuf maintainers: Uploads non-malicious protoc binaries to Github.


## Updating to new protoc releases (for maintainers)

1. Run: `cargo run -- (version e.g 27.0)`
2. Append the printed struct definitions into the `KNOWN_VERSIONS` array in `lib.rs`.
3. Run `make` to execute all checks.
4. Update the version in `Cargo.toml` to include the version of protoc. E.g. `"0.1.0+27.0"`.
5. Send a pull request.


## Releasing the crate (for maintainers)

1. Test it: `make && cargo publish --dry-run`
2. Publish to crates.io: `cargo publish`
3. Tag the release: `(VERSION=$(cargo pkgid | sed 's/.*@//'); git tag -a "v$VERSION" -m "release version $VERSION")`
4. Push the tag: `git push --tags`
5. Create a release from the existing tag on Github: Go to [Tags](https://github.com/evanj/dlprotoc-rs/tags), click the "..." menu, choose Create Release.
