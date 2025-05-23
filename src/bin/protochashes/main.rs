//! downloads a version of protoc and prints hashes for all OSes and CPU architectures.

use std::fmt::Write;

use dlprotoc::{CPUArch, OS, download_unverified, protoc_hash};

fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(s, "{byte:02x}").unwrap();
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: protochashes (protoc version e.g 27.0)");
        std::process::exit(1);
    }
    let version = args[1].as_str();

    for os in OS::all() {
        for cpu in CPUArch::all() {
            let bytes = download_unverified(*os, *cpu, version)?;
            let hash = protoc_hash(&bytes);

            println!("KnownVersion {{");
            println!("    os: OS::{},", os.rust_identifier());
            println!("    cpu: CPUArch::{},", cpu.code_label());
            println!("    version: {version:#?},");
            println!("    hash: hex!(\"{}\"),", hex_string(&hash));
            println!("}},");
        }
    }

    Ok(())
}
