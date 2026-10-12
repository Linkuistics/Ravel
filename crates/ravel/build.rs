//! Embed the digest of the worker source this front process must be paired with.
//!
//! The worker is compiled by `scripts/dispatch.sh` with the pinned Bun, never by
//! cargo: the Rust package builds without Bun, and nothing here runs it. What
//! this script does instead is compute the same digest the build script injects
//! into the worker, over the same files, so the front can refuse a worker built
//! from any other source. The two definitions must agree byte for byte: the
//! `shasum -a 256` line of each file in `source_files` order, hashed again.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn main() {
    let crate_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets it"));
    let files = source_files(&crate_dir);

    let mut listing = String::new();
    for file in &files {
        let bytes = fs::read(crate_dir.join(file)).unwrap_or_else(|error| {
            panic!("cannot read worker source {file}: {error}");
        });
        listing.push_str(&format!("{}  {file}\n", hex(&Sha256::digest(&bytes))));
    }
    let build_id = hex(&Sha256::digest(listing.as_bytes()));
    println!("cargo:rustc-env=RAVEL_WORKER_BUILD_ID={build_id}");

    // Directories are scanned recursively, so a new source file reruns this too.
    for watched in ["worker/src", "worker/sdk", "worker/examples"] {
        println!("cargo:rerun-if-changed={watched}");
    }
    for file in &files {
        println!("cargo:rerun-if-changed={file}");
    }
}

/// The worker's source set, relative to the crate and sorted bytewise, exactly
/// as `source_files` in `scripts/dispatch.sh` lists it.
fn source_files(crate_dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for dir in ["worker/src", "worker/sdk", "worker/examples"] {
        collect_typescript(crate_dir, Path::new(dir), &mut files);
    }
    for fixed in [
        "worker/tsconfig.json",
        "worker/tsconfig.declarations.json",
        "scripts/dispatch.sh",
    ] {
        files.push(fixed.to_owned());
    }
    files.sort();
    files
}

fn collect_typescript(crate_dir: &Path, relative: &Path, files: &mut Vec<String>) {
    let entries = fs::read_dir(crate_dir.join(relative))
        .unwrap_or_else(|error| panic!("cannot list {}: {error}", relative.display()));
    for entry in entries {
        let entry = entry.expect("readable directory entry");
        let path = relative.join(entry.file_name());
        let kind = entry.file_type().expect("file type");
        if kind.is_dir() {
            collect_typescript(crate_dir, &path, files);
        } else if kind.is_file() && path.extension().is_some_and(|ext| ext == "ts") {
            files.push(path.to_str().expect("UTF-8 source path").to_owned());
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
