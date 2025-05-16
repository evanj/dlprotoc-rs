use crate::error::Error;
use hex_literal::hex;
use std::fmt::Display;

/// Operating system used to run protoc. The Display trait returns the string used for protoc URLs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "29.0",
        hash: hex!("305f1be5ae7b2f39451870b312b45c1e0ba269901c83ba16d85f9f9d1441b348"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "29.0",
        hash: hex!("3c51065af3b9a606d9e18a1bf628143734ff4b9e69725d6459857430ba7a78df"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "29.0",
        hash: hex!("b2b59f03b030c8a748623d682a8b5bc9cc099e4bcfd06b8964ce89ec065b3103"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "29.0",
        hash: hex!("e7a1cffc82e21daa67833011449c70ddff1eba3b115934387e6e8141efab092f"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "29.2",
        hash: hex!("29cf483e2fb21827e5fac4964e35eae472a238e28c762f02fb17dcd93ff8b89f"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "29.2",
        hash: hex!("52e9e7ece55c7e30e7e8bbd254b4b21b408a5309bca826763c7124b696a132e9"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "29.2",
        hash: hex!("0e153a38d6da19594c980e7f7cd3ea0ddd52c9da1068c03c0d8533369fbfeb20"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "29.2",
        hash: hex!("ba2bd983b5f06ec38d663b602884a597dea3990a43803d7e153ed8f7c54269e1"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "29.3",
        hash: hex!("6427349140e01f06e049e707a58709a4f221ae73ab9a0425bc4a00c8d0e1ab32"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "29.3",
        hash: hex!("3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "29.3",
        hash: hex!("2b8a3403cd097f95f3ba656e14b76c732b6b26d7f183330b11e36ef2bc028765"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "29.3",
        hash: hex!("9a788036d8f9854f7b03c305df4777cf0e54e5b081e25bf15252da87e0e90875"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "30.0",
        hash: hex!("5ab347b71fb8a87139cec36aac4bd0ee3ac3f4f2af9fc68ebdf556e1c0a665c6"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "30.0",
        hash: hex!("2fbbc1818463d7e6d93c19a8dea839e663ca5f8579a52ef78c7688188335fa6c"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "30.0",
        hash: hex!("7eb5b51d37bac410ba70ef91c404f90b1fabcb823712ff656582d34acc87ca74"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "30.0",
        hash: hex!("96bf3a5fbeefd57d7dc0c20a2c7bb3f226ad84b79e5b509386824322017b9417"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "30.1",
        hash: hex!("e866d3dc4775e8032721915e83e3fb6e1ab4def7199a49b4f95c4d1f6cf4c03a"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "30.1",
        hash: hex!("5537e15ab0c0e610f809573948d3ec7d6ef387a07991e1c361a2a0e8cad983e5"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "30.1",
        hash: hex!("03467cfd967de12a61406b7473e80204d3ae38f30f82855318186d696237e3b9"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "30.1",
        hash: hex!("a4aeefd2f59ccce59cfa01a89fe58adb40bb9010f43adfca3c4fee7fd37ec2c5"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "30.2",
        hash: hex!("a3173ea338ef91b1605b88c4f8120d6c8ccf36f744d9081991d595d0d4352996"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "30.2",
        hash: hex!("327e9397c6fb3ea2a542513a3221334c6f76f7aa524a7d2561142b67b312a01f"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "30.2",
        hash: hex!("92728c650f6cf2b6c37891ae04ef5bc2d4b5f32c5fbbd101eda623f90bb95f63"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "30.2",
        hash: hex!("65675c3bb874a2d5f0c941e61bce6175090be25fe466f0ec2d4a6f5978333624"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::AArch64,
        version: "31.0",
        hash: hex!("999f4c023366b0b68c5c65272ead7877e47a2670245a79904b83450575da7e19"),
    },
    KnownVersion {
        os: OS::Linux,
        cpu: CPUArch::X86_64,
        version: "31.0",
        hash: hex!("24e2ed32060b7c990d5eb00d642fde04869d7f77c6d443f609353f097799dd42"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::AArch64,
        version: "31.0",
        hash: hex!("1fbe70a8d646875f91b6fd57294f763145292b2c9e1374ab09d6e2124afdd950"),
    },
    KnownVersion {
        os: OS::OSX,
        cpu: CPUArch::X86_64,
        version: "31.0",
        hash: hex!("0360d9b6d9e3d66958cf6274d8514da49e76d475fd0d712181dcc7e9e056f2c8"),
    },
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_known_hash() {
        // ensure we know a hash for the current platform
        known_hash(OS::current(), CPUArch::current(), LATEST_VERSION).unwrap();
    }

    #[test]
    fn test_known_versions_constant() {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct KnownVersionKey {
            os: OS,
            cpu: CPUArch,
            version: String,
        }
        // check that KNOWN_VERSIONS is increasing and unique
        let mut all_versions = HashSet::new();
        let mut last_version = KNOWN_VERSIONS[0].version;
        for known_version in KNOWN_VERSIONS {
            // This should be a semver comparsion, but is testing a string comparion.
            // This will work with protoc versions since they are two digits, but can easily fail
            // e.g. if there are a lot of point releases, because "27.10" should be greater than "27.9"
            assert!(known_version.version >= last_version);
            last_version = known_version.version;

            let key = KnownVersionKey {
                os: known_version.os,
                cpu: known_version.cpu,
                version: known_version.version.to_string(),
            };
            let newly_inserted = all_versions.insert(key.clone());
            assert!(newly_inserted, "duplicate version: {key:?}");
        }

        assert_eq!(LATEST_VERSION, last_version);
    }
}
