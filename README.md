# dlprotoc: Download protoc for Cargo build scripts

I am tried of separately installing protoc on build machines. This crate downloads the
[official binary release of protoc from Google's protobuf Github repo](https://github.com/protocolbuffers/protobuf),
verifies a SHA256 hash, then extracts it. It easy to use with Prost or Tonic.

## Quick Start

In your `build.rs`:

```rust
TODO
```

## Updating to new releases

1. Run: `cargo run -- (version e.g 27.0)`
2. Copy the resulting structs into the `KNOWN_VERSIONS` array in `lib.rs`.