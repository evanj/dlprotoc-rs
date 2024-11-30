/*!
Downloads protoc binaries for use in Cargo build scripts.

This crate is intended to be used in Cargo build scripts (`build.rs`) with
[`prost-build`](https://docs.rs/prost-build/latest/prost_build/) or
[`tonic-build`](https://docs.rs/tonic-build/latest/tonic_build/), so you don't need `protoc`
installed to build projects that use Protocol Buffers.

# Usage

Add the `dlprotoc` crate to `build-dependencies` in `Cargo.toml`:
```toml
[build-dependencies]
dlprotoc = "0"
```

In `build.rs`, call [`download_protoc`] before calling `compile_protos`:

```no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dlprotoc::download_protoc()?;
    prost_build::compile_protos(&["src/example.proto"], &["src/"])?;
    Ok(())
}
```
*/

use std::{io::Cursor, path::Path};

use sha2::{Digest, Sha256};

mod error;
mod versions;

use error::Error;

pub type CPUArch = versions::CPUArch;
pub type OS = versions::OS;
use versions::known_hash;

// Cargo's build output environment variable. See:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
const CARGO_BUILD_OUT_ENV_VAR: &str = "OUT_DIR";

// Prost uses the PROTOC env var to find the protoc executable. See:
// https://docs.rs/prost-build/latest/prost_build/#sourcing-protoc
const PROST_PROTOC_ENV_VAR: &str = "PROTOC";

/// Returns the URL to download the protoc release. The version is the format major.minor, such as "27.0".
fn make_url(os: OS, cpu: CPUArch, version: &str) -> String {
    format!("https://github.com/protocolbuffers/protobuf/releases/download/v{version}/protoc-{version}-{os}-{cpu}.zip")
}

/// Downloads protoc without verifying the hash. This should only be used by the dlprotoc
/// crate, and by the `protochashes` tool.
///
/// # Errors
///
/// Returns an error if it fails to fetch protoc over the Internet.
pub fn download_unverified(os: OS, cpu: CPUArch, version: &str) -> Result<Vec<u8>, Error> {
    let url = make_url(os, cpu, version);
    let response = reqwest::blocking::get(url)?.error_for_status()?;
    let bytes = response.bytes()?;

    // Convert the Bytes struct into a plain Vec<u8> to avoid exposing dependencies
    Ok(bytes.as_ref().to_vec())
}

fn fetch_current() -> Result<Vec<u8>, Error> {
    let os = OS::current();
    let cpu = CPUArch::current();
    let version = versions::LATEST_VERSION;

    let expected_hash = known_hash(os, cpu, version)?;
    let data = download_unverified(OS::current(), CPUArch::current(), version)?;
    let actual_hash = protoc_hash(&data);
    if expected_hash != actual_hash {
        return Err(Error::from_string(format!(
            "hash mismatch for {os} {cpu} {version}",
        )));
    }
    Ok(data)
}

/// Hashes data using the algorithm used to verify protoc binaries (currently SHA-256). This should
/// only be used by the `protochashes` tool.
#[must_use]
pub fn protoc_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut result = [0; 32];
    result.copy_from_slice(&hash);
    result
}

fn write_protoc(destination_dir: &Path) -> Result<(), Error> {
    // downloads protoc for the current platform, checking the hashes
    let protoc_zip_bytes = fetch_current()?;

    write_protoc_zip_data(destination_dir, &protoc_zip_bytes)
}

/// Downloads protoc to the `OUT_DIR` environment variable and sets the `PROTOC` environment
/// variable so prost-build or tonic-build can find it.
///
/// Intended to be called from a Cargo build script (`build.rs`).
///
/// # Errors
///
/// Returns an [`Error`] if it fails to fetch protoc over the Internet, fails to verify it, or
/// fails to unzip it.
pub fn download_protoc() -> Result<(), Error> {
    let out_dir = std::env::var(CARGO_BUILD_OUT_ENV_VAR)
        .map_err(|e| Error::with_prefix(format!("env var {CARGO_BUILD_OUT_ENV_VAR}"), e))?;
    let protoc_distribution_path = Path::new(&out_dir).join("protoc_zip");
    if protoc_distribution_path.exists() {
        print!("dlprotoc: not downloading; protoc already exists at {protoc_distribution_path:?}");
    } else {
        write_protoc(&protoc_distribution_path)?;
    }

    let protoc_path = protoc_distribution_path.join("bin").join("protoc");
    std::env::set_var(PROST_PROTOC_ENV_VAR, protoc_path);

    Ok(())
}

/// Extracts files from the protoc distribution Zip data into `destination_dir`. This makes it
/// easier to test the code without downloading anything.
fn write_protoc_zip_data(destination_dir: &Path, protoc_zip_bytes: &[u8]) -> Result<(), Error> {
    let mut zip = zip::ZipArchive::new(Cursor::new(&protoc_zip_bytes))?;
    zip.extract(destination_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, io::Write, process::Command};
    use zip::{write::SimpleFileOptions, ZipWriter};

    use super::*;
    use versions::LATEST_VERSION;

    #[test]
    fn test_make_url() {
        let url = make_url(OS::Linux, CPUArch::X86_64, "27.0");
        assert_eq!(url, "https://github.com/protocolbuffers/protobuf/releases/download/v27.0/protoc-27.0-linux-x86_64.zip");

        let url = make_url(OS::OSX, CPUArch::AArch64, "26.1");
        assert_eq!(url, "https://github.com/protocolbuffers/protobuf/releases/download/v26.1/protoc-26.1-osx-aarch_64.zip");
    }

    struct SetEnvForTest<'a> {
        name: &'a str,
        previous: Option<String>,
    }

    impl<'a> SetEnvForTest<'a> {
        fn set(name: &'a str, value: impl AsRef<OsStr>) -> Result<Self, std::env::VarError> {
            let previous = match std::env::var(name) {
                Ok(value) => Some(value),
                Err(std::env::VarError::NotPresent) => None,
                Err(e) => return Err(e),
            };
            std::env::set_var(name, value);
            Ok(Self { name, previous })
        }
    }

    impl Drop for SetEnvForTest<'_> {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => std::env::set_var(self.name, value),
                None => std::env::remove_var(self.name),
            }
        }
    }

    // this tests actually downloads protoc and sets environment variables
    // just like build scripts will. It is slow, so it is ignored by default.
    #[test]
    #[ignore = "requires network access"]
    fn test_write_protoc_real_network_access() -> Result<(), Box<dyn std::error::Error>> {
        // actually downloads from the real URL
        let tempdir = tempfile::tempdir()?;
        // set the PROTOC env var to "" so it gets reset when the test ends
        let reset_protoc_env_var = SetEnvForTest::set(PROST_PROTOC_ENV_VAR, "");
        let reset_out_dir_env_var = SetEnvForTest::set(CARGO_BUILD_OUT_ENV_VAR, tempdir.path());
        download_protoc()?;
        drop(reset_out_dir_env_var);

        let example_proto_path = tempdir.path().join("foo.proto");
        std::fs::write(
            &example_proto_path,
            br#"syntax = "proto3";
package examplepb;
import "google/protobuf/duration.proto";
message M {
    google.protobuf.Duration example_duration = 1;
}
"#,
        )?;

        // run protoc to compile the test proto file
        let protoc_path = std::env::var(PROST_PROTOC_ENV_VAR)?;
        let output = Command::new(protoc_path)
            .arg(&example_proto_path)
            .arg(format!("--proto_path={}", tempdir.path().display()))
            .arg("--descriptor_set_out=/dev/null")
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.is_empty() && stderr.is_empty(),
            "expected no output from protoc\nstdout: {stdout}\n stderr: {stderr}\n"
        );

        drop(reset_protoc_env_var);

        Ok(())
    }

    /// Returns a helpful message when env vars not set.
    #[test]
    fn test_download_protoc_not_build_script() {
        let err = download_protoc().expect_err("must return an error");
        assert!(
            err.to_string().contains("env var OUT_DIR"),
            "download_protoc unexpected error message: {err}"
        );
    }

    /// Tests most of the code without downloading anything.
    #[test]
    fn test_unpack_fetch_fake() {
        let mut zip_data = Vec::new();
        let mut zip_w = ZipWriter::new(Cursor::new(&mut zip_data));
        let exe_options = SimpleFileOptions::default().unix_permissions(0o755);
        zip_w.start_file("bin/protoc", exe_options).unwrap();
        let script_contents = format!("#!/bin/sh\necho protoc fake version {LATEST_VERSION}\n");
        zip_w.write_all(script_contents.as_bytes()).unwrap();

        zip_w
            .start_file(
                "include/google/protobuf/duration.proto",
                SimpleFileOptions::default(),
            )
            .unwrap();
        let fake_duration_proto = br#"syntax = "proto3";"#;
        zip_w.write_all(fake_duration_proto).unwrap();
        zip_w.finish().unwrap();

        check_write_protoc(|destination| write_protoc_zip_data(destination, &zip_data));
    }

    fn check_write_protoc(write_protoc_fn: impl Fn(&Path) -> Result<(), Error>) {
        let tempdir = tempfile::tempdir().unwrap();
        let protoc_zip_dir_path = tempdir.path().join("protoc_zip");

        write_protoc_fn(&protoc_zip_dir_path).unwrap();

        // check that the include dir exists
        assert!(protoc_zip_dir_path.join("include").is_dir());

        // run protoc and make sure it "works"
        let protoc_path = protoc_zip_dir_path.join("bin").join("protoc");
        let output = Command::new(protoc_path).arg("--version").output().unwrap();
        let version_output = String::from_utf8_lossy(&output.stdout);
        let expected_end = format!("{LATEST_VERSION}\n");
        assert!(
            version_output.ends_with(&expected_end),
            "unexpected version output: {version_output}"
        );
    }

    #[test]
    fn test_error_implements_std_error() {
        // ensures we can use this error as a std Error
        let err: Box<dyn std::error::Error> = Box::new(Error::from_string(String::from("test")));
        assert_eq!("test", err.to_string());
    }
}
