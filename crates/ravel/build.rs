//! Embed the digests that identify the workers this front process is paired with.
//!
//! Two workers, two digests. The Rhai worker is this binary in an internal role,
//! so its identity is a digest of the crate's own sources (`RAVEL_BUILD_ID`).
//! The TypeScript worker is described below (`RAVEL_WORKER_BUILD_ID`). The
//! removal increment deletes the second digest and keeps the first.
//!
//! The worker is compiled by `scripts/dispatch.sh` with the pinned Bun, never by
//! cargo: the Rust package builds without Bun, and nothing here runs it. What
//! this script does instead is compute the same digest the build script injects
//! into the worker, over the same files, so the front can refuse a worker built
//! from any other source. The two definitions must agree byte for byte: the
//! `shasum -a 256` line of each file in `source_files` order, hashed again.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn main() {
    let crate_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets it"));
    embed_crate_digest(&crate_dir);
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

/// The identity of the binary: a digest of the crate sources, its manifest, the
/// build script and the lockfile, each as a `sha256` line in bytewise path order
/// and hashed again, as for the TypeScript worker. The runner crate is part of
/// the binary too, so its sources count.
fn embed_crate_digest(crate_dir: &Path) {
    let mut files = Vec::new();
    collect_rust(crate_dir, Path::new("src"), &mut files);
    collect_rust(crate_dir, Path::new("../keyed-launch/src"), &mut files);
    for fixed in ["Cargo.toml", "build.rs", "../../Cargo.lock"] {
        files.push(fixed.to_owned());
    }
    files.sort();
    let mut listing = String::new();
    for file in &files {
        let bytes = fs::read(crate_dir.join(file)).unwrap_or_else(|error| {
            panic!("cannot read crate source {file}: {error}");
        });
        listing.push_str(&format!("{}  {file}\n", hex(&Sha256::digest(&bytes))));
    }
    println!(
        "cargo:rustc-env=RAVEL_BUILD_ID={}",
        hex(&Sha256::digest(listing.as_bytes()))
    );
    // Directories are scanned recursively, so a new source file reruns this too.
    for watched in ["src", "../keyed-launch/src"] {
        println!("cargo:rerun-if-changed={watched}");
    }
    for file in &files {
        println!("cargo:rerun-if-changed={file}");
    }
}

fn collect_rust(crate_dir: &Path, relative: &Path, files: &mut Vec<String>) {
    let entries = fs::read_dir(crate_dir.join(relative))
        .unwrap_or_else(|error| panic!("cannot list {}: {error}", relative.display()));
    for entry in entries {
        let entry = entry.expect("readable directory entry");
        let path = relative.join(entry.file_name());
        let kind = entry.file_type().expect("file type");
        if kind.is_dir() {
            collect_rust(crate_dir, &path, files);
        } else if kind.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path.to_str().expect("UTF-8 source path").to_owned());
        }
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
    let mut rendered = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Infallible: `String`'s `Write` never errors, and the format is fixed.
        let _ = write!(&mut rendered, "{byte:02x}");
    }
    rendered
}
