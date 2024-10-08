use crate::error::Error;
use hex_literal::hex;
use std::fmt::Display;

/// Operating system used to run protoc. The Display trait returns the string used for protoc URLs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OS {
    /// Linux: "linux" in protoc URLs.
    Linux,
    /// Mac OS X / Darwin: "macos" in protoc URLs.
    #[expect(
        clippy::upper_case_acronyms,
        reason = "OSX is a permitted all-caps acronym"
    )]
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
    pub const fn rust_identifier(self) -> &'static str {
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
    pub const fn code_label(self) -> &'static str {
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

type Sha256HashResult = [u8; 32];

/// Defines an expected hash for a specific protoc binary release.
struct KnownVersion {
    os: OS,
    cpu: CPUArch,
    version: &'static str,
    hash: Sha256HashResult,
}

/// The most recent version of protoc that we know about.
pub const LATEST_VERSION: &str = KNOWN_VERSIONS[KNOWN_VERSIONS.len() - 1].version;

pub fn known_hash(os: OS, cpu: CPUArch, version: &str) -> Result<Sha256HashResult, Error> {
    for known in KNOWN_VERSIONS {
        if known.os == os && known.cpu == cpu && known.version == version {
            return Ok(known.hash);
        }
    }
    Err(Error::from_string(format!(
        "unknown hash for {os} {cpu} {version}"
    )))
}

/// All binary releases of protoc we know about. This is in increasing version number order.
const KNOWN_VERSIONS: &[KnownVersion] = &[
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
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "27.2",
        hash: hex!("ff4760bd4ae510d533e528cc6deb8e32e53f383f0ec01b0327233b4c2e8db314"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "27.2",
        hash: hex!("4a95e0ea2e51720af86a92f48d4997c8756923a9d0c58fd8a850657cd7479caf"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "27.2",
        hash: hex!("877de17b5d2662b96e68a6e208cb1851437ab3e2b419c2ef5b7b873ffac5357d"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "27.2",
        hash: hex!("abc25a236571612d45eb4b6b6e6abe3ac9aecc34b195f76f248786844f5619c7"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "27.3",
        hash: hex!("bdad36f3ad7472281d90568c4956ea2e203c216e0de005c6bd486f1920f2751c"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "27.3",
        hash: hex!("6dab2adab83f915126cab53540d48957c40e9e9023969c3e84d44bfb936c7741"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "27.3",
        hash: hex!("b22116bd97cdbd7ea25346abe635a9df268515fe5ef5afa93cd9a68fc2513f84"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "27.3",
        hash: hex!("ce282648fed0e7fbd6237d606dc9ec168dd2c1863889b04efa0b19c47da65d1b"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "28.2",
        hash: hex!("91d8253cdc0f0f0fc51c2b69c80677996632f525ad84504bfa5b4ee38ad3e49c"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "28.2",
        hash: hex!("2febfd42b59ce93a28eb789019a470a3dd0449619bc04f84dad1333da261dec1"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "28.2",
        hash: hex!("7bb048f52841789d9ec61983be0ce4c9e4fb3bd9a143462820ba9a3be0a03797"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "28.2",
        hash: hex!("232f07d12bf4806207a79ec2c7378301c52e6f2f7efdd21c0dd416f0bda103ec"),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_hash() {
        // ensure we know a hash for the current platform
        known_hash(OS::current(), CPUArch::current(), LATEST_VERSION).unwrap();
    }

    #[test]
    fn test_known_versions_constant() {
        // check that KNOWN_VERSIONS is increasing
        let mut last_version = KNOWN_VERSIONS[0].version;
        for known_version in KNOWN_VERSIONS {
            // This should be a semver comparsion, but is testing a string comparion.
            // This will work with protoc versions since they are two digits, but can easily fail
            // e.g. if there are a lot of point releases, because "27.10" should be greater than "27.9"
            assert!(known_version.version >= last_version);
            last_version = known_version.version;
        }

        assert_eq!(LATEST_VERSION, last_version);
    }
}
