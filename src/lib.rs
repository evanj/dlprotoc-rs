//! Download protoc for Cargo build scripts.

use hex_literal::hex;
use std::{borrow::Borrow, fmt::Display, io::Cursor, path::Path};
use zip::result::ZipError;

use sha2::{Digest, Sha256};

// The most recent version of protoc that we know about.
const LATEST_VERSION: &str = "27.1";

/// Operating system used to run protoc. The Display trait returns the string used for protoc URLs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OS {
    /// Linux: "linux" in protoc URLs.
    Linux,
    /// Mac OS X / Darwin: "macos" in protoc URLs.
    OSX,
    // TODO: Windows,
}

impl OS {
    /// Returns the operating system executing this function.
    ///
    /// # Panics
    /// If this is run on an unsupported operating system.
    #[must_use]
    pub fn current() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "macos" => Self::OSX,
            unsupported_os => panic!("unsupported OS: {unsupported_os}"),
        }
    }

    /// Returns all defined enum values.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Linux, Self::OSX]
    }

    /// Returns the Rust enum identifier as used in code.
    #[must_use]
    pub const fn rust_identifier(&self) -> &'static str {
        match self {
            Self::Linux => "Linux",
            Self::OSX => "OSX",
        }
    }
}

impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Linux => "linux",
            Self::OSX => "osx",
        };
        write!(f, "{s}")
    }
}

/// CPU architecture used to run protoc. The Display trait returns the string used for protoc URLs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CPUArch {
    /// ARM AArch64: "aarch64" in protoc URLs.
    #[allow(clippy::doc_markdown)]
    AArch64,
    /// Intel/AMD x86-64: "x86_64" in protoc URLs.
    #[allow(clippy::doc_markdown)]
    X86_64,
}

impl CPUArch {
    /// Returns the CPU architecture executing this function.
    ///
    /// # Panics
    /// If run on an unsupported architecture.
    #[must_use]
    pub fn current() -> Self {
        match std::env::consts::ARCH {
            "aarch64" => Self::AArch64,
            "x86_64" => Self::X86_64,
            unsupported_arch => panic!("unsupported arch: {unsupported_arch}"),
        }
    }

    /// Returns all defined enum values.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::AArch64, Self::X86_64]
    }

    /// Returns the Rust enum identifier as used in code.
    #[must_use]
    pub const fn code_label(&self) -> &'static str {
        match self {
            Self::AArch64 => "AArch64",
            Self::X86_64 => "X86_64",
        }
    }
}

impl Display for CPUArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AArch64 => "aarch_64",
            Self::X86_64 => "x86_64",
        };
        write!(f, "{s}")
    }
}

/// Returns the URL to download the protoc release. The version is the format major.minor, such as "27.0".
fn make_url(os: OS, cpu: CPUArch, version: &str) -> String {
    format!("https://github.com/protocolbuffers/protobuf/releases/download/v{version}/protoc-{version}-{os}-{cpu}.zip")
}

/// The Error type returned by the dlprotoc crate.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    const fn from_string(message: String) -> Self {
        Self { message }
    }

    fn with_prefix(prefix: impl Borrow<str>, e: impl Display) -> Self {
        Self {
            message: format!("{}: {e}", prefix.borrow()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Self::with_prefix("zip error", e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::with_prefix("io error", e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        let message = if let Some(url) = e.url() {
            format!("failed downloading protoc from url: {url}: {e}")
        } else {
            format!("failed downloading protoc: {e}")
        };
        Self::from_string(message)
    }
}

/// Downloads the protoc binary without verifying the hash. This should only be used by the dlprotoc
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

type Sha256HashResult = [u8; 32];

/// Defines an expected hash for a specific protoc binary release.
struct KnownVersion {
    os: OS,
    cpu: CPUArch,
    version: &'static str,
    hash: Sha256HashResult,
}

static KNOWN_VERSIONS: [KnownVersion; 8] = [
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "27.0",
        hash: hex!("e2bdce49564dbad4676023d174d9cdcf932238bc0b56a8349a5cb27bbafc26b0"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "27.0",
        hash: hex!("1e4b2d8b145afe99a36602f305165761e46d2525aa94cbb907e2e983be6717ac"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "27.0",
        hash: hex!("d956cf3a9e91a687aa4d1026e9261e5a587e4e0b545db0174509f6c448b8ce21"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "27.0",
        hash: hex!("2cf59e3e3463bede1f10b7517efdddd97a3bd8cfd9cacc286407b657290dc781"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "27.1",
        hash: hex!("8809c2ec85368c6b6e9af161b6771a153aa92670a24adbe46dd34fa02a04df2f"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "27.1",
        hash: hex!("8970e3d8bbd67d53768fe8c2e3971bdd71e51cfe2001ca06dacad17258a7dae3"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "27.1",
        hash: hex!("03b7af1bf469e7285dc51976ee5fa99412704dbd1c017105114852a37b165c12"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "27.1",
        hash: hex!("8520d944f3a3890fa296a3b3b0d4bb18337337e2526bbbf1b507eeea3c2a1ec4"),
    },
];

fn known_hash(os: OS, cpu: CPUArch, version: &str) -> Result<Sha256HashResult, Error> {
    for known in &KNOWN_VERSIONS {
        if known.os == os && known.cpu == cpu && known.version == version {
            return Ok(known.hash);
        }
    }
    Err(Error::from_string(format!(
        "unknown hash for {os} {cpu} {version}"
    )))
}

fn fetch_current() -> Result<Vec<u8>, Error> {
    let os = OS::current();
    let cpu = CPUArch::current();
    let version = LATEST_VERSION;

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

/// Downloads protoc binary to the `OUT_DIR` environment variable and sets the `PROTOC` environment
/// variable so prost/tonic can find it. Intended to be used from a Cargo build script (`build.rs`).
///
/// # Errors
///
/// Returns an error if it fails to fetch protoc over the Internet, fails to verify it, or fails to
/// unzip it.
pub fn download_protoc() -> Result<(), Error> {
    let out_dir = std::env::var("OUT_DIR").map_err(|e| Error::with_prefix("env", e))?;
    let protoc_distribution_path = Path::new(&out_dir).join("protoc_zip");
    if protoc_distribution_path.exists() {
        print!("dlprotoc: not downloading; protoc already exists at {protoc_distribution_path:?}");
    } else {
        write_protoc(&protoc_distribution_path)?;
    }

    std::env::set_var("PROTOC", &protoc_distribution_path);

    Ok(())
}

/// Extracts files from the protoc distribution Zip data into `destination_dir`. This makes it
/// easier to test the code without downloading anything.
fn write_protoc_zip_data(destination_dir: &Path, protoc_zip_bytes: &[u8]) -> Result<(), Error> {
    let mut zip = zip::ZipArchive::new(Cursor::new(&protoc_zip_bytes))?;
    zip.extract(destination_dir)?;
    // let mut protoc_f = zip.by_name(PROTOC_ZIP_PATH)?;
    // let mut protoc_bytes = Vec::new();
    // protoc_f.read_to_end(&mut protoc_bytes)?;

    // fs::write(destination_dir, &protoc_bytes)?;

    // // make protoc executable
    // fs::set_permissions(destination_dir, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::PathBuf, process::Command};

    use tempfile::TempDir;
    use zip::{write::SimpleFileOptions, ZipWriter};

    use super::*;

    #[test]
    fn test_make_url() {
        let url = make_url(OS::Linux, CPUArch::X86_64, "27.0");
        assert_eq!(url, "https://github.com/protocolbuffers/protobuf/releases/download/v27.0/protoc-27.0-linux-x86_64.zip");

        let url = make_url(OS::OSX, CPUArch::AArch64, "26.1");
        assert_eq!(url, "https://github.com/protocolbuffers/protobuf/releases/download/v26.1/protoc-26.1-osx-aarch_64.zip");
    }

    #[test]
    #[ignore = "requires network access"]
    fn test_write_protoc_real_network_access() -> Result<(), std::io::Error> {
        // actually downloads from the real URL
        // TODO: run a fake web server to test more stuff?
        let protoc_temp = check_write_protoc(write_protoc);

        let well_known_types_protobuf_path = protoc_temp.tempdir.path().join("foo.proto");
        std::fs::write(
            &well_known_types_protobuf_path,
            br#"syntax = "proto3";
package examplepb;
import "google/protobuf/duration.proto";
message M {
    google.protobuf.Duration example_duration = 1;
}
"#,
        )?;

        // run protoc to compile the test proto file
        let output = Command::new(&protoc_temp.protoc_path)
            .arg(&well_known_types_protobuf_path)
            .arg(format!(
                "--proto_path={}",
                protoc_temp.tempdir.path().display()
            ))
            .arg("--descriptor_set_out=/dev/null")
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.is_empty() && stderr.is_empty(),
            "expected no output from protoc\nstdout: {stdout}\n stderr: {stderr}\n"
        );

        Ok(())
    }

    #[test]
    fn test_known_hash() {
        // ensure we know a hash for the current platform
        known_hash(OS::current(), CPUArch::current(), LATEST_VERSION).unwrap();
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

    struct TempProtocResult {
        tempdir: TempDir,
        protoc_path: PathBuf,
    }

    fn check_write_protoc(
        write_protoc_fn: impl Fn(&Path) -> Result<(), Error>,
    ) -> TempProtocResult {
        let tempdir = tempfile::tempdir().unwrap();
        let protoc_zip_dir_path = tempdir.path().join("protoc_zip");

        write_protoc_fn(&protoc_zip_dir_path).unwrap();

        // check that the include dir exists
        assert!(protoc_zip_dir_path.join("include").is_dir());

        // run protoc and make sure it "works"
        let protoc_path = protoc_zip_dir_path.join("bin").join("protoc");
        let output = Command::new(&protoc_path)
            .arg("--version")
            .output()
            .unwrap();
        let version_output = String::from_utf8_lossy(&output.stdout);
        let expected_end = format!("{LATEST_VERSION}\n");
        assert!(
            version_output.ends_with(&expected_end),
            "unexpected version output: {version_output}"
        );

        TempProtocResult {
            tempdir,
            protoc_path,
        }
    }

    #[test]
    fn test_error_implements_std_error() {
        // ensures we can use this error as a std Error
        let err: Box<dyn std::error::Error> = Box::new(Error::from_string(String::from("test")));
        assert_eq!("test", err.to_string());
    }
}
