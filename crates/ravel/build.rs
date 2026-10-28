//! Embed the source identity of this executable and its internal Rhai worker.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn main() {
    let crate_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets it"));
    embed_crate_digest(&crate_dir);
}

/// The identity of the binary: a digest of the crate sources, its manifest, the
/// build script and the lockfile, each as a `sha256` line in bytewise path order
/// and hashed again. The runner crate is part of
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

fn hex(bytes: &[u8]) -> String {
    let mut rendered = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Infallible: `String`'s `Write` never errors, and the format is fixed.
        let _ = write!(&mut rendered, "{byte:02x}");
    }
    rendered
}
