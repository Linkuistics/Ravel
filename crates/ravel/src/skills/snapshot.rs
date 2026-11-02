//! Capture of one local source into private staging, its manifest, and the
//! atomic publication of retained objects
//! (docs/specs/skill-discovery.md#sources-and-retained-resources).
//!
//! Detection of concurrent source edits, path-identity collisions on the cache
//! filesystem, cache tampering and unsafe cache roots belongs to a later stage.
//! Its checks attach to `capture` and `publish` here.

use std::collections::BTreeSet;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read as _, Write as _};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::{DirBuilderExt as _, OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

use super::{Located, refuse};
use crate::refusal::Refusal;

/// The names that capture leaves out at every level.
const RESERVED: [&str; 2] = [".git", ".jj"];
/// More link hops than this in one chain is a cycle.
const LINK_HOPS: usize = 40;
const MANIFEST_HEADER: &[u8] = b"ravel-skill-manifest\n1\n";
/// The file beside the captured tree in a snapshot object.
pub const MANIFEST_FILE: &str = "manifest";
/// The captured tree in a snapshot object.
pub const TREE_DIR: &str = "tree";

#[derive(Debug)]
enum Kind {
    Directory,
    File { executable: bool, digest: String },
    Link { target: String },
}

#[derive(Debug)]
struct Entry {
    path: String,
    kind: Kind,
}

/// A private staging directory that the resolver which made it removes.
#[derive(Debug)]
pub struct Staging {
    path: PathBuf,
    armed: bool,
}

impl Staging {
    /// A new directory beneath `parent` that only the owner can open.
    pub fn new(parent: &Path, prefix: &str) -> io::Result<Staging> {
        let directory = tempfile::Builder::new()
            .prefix(prefix)
            .permissions(fs::Permissions::from_mode(0o700))
            .tempdir_in(parent)?;
        Ok(Staging {
            path: directory.keep(),
            armed: true,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Remove the staging tree, restoring write permission where publication
    /// had already removed it.
    pub fn cleanup(&mut self) -> io::Result<()> {
        if !self.armed {
            return Ok(());
        }
        self.armed = false;
        writable(&self.path)?;
        fs::remove_dir_all(&self.path)
    }

    /// The directory is published, so it is not staging any longer.
    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for Staging {
    fn drop(&mut self) {
        // Explicit cleanup reports failures; this only covers an unwinding
        // resolver.
        let _ = self.cleanup();
    }
}

fn writable(path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
        for entry in fs::read_dir(path)? {
            writable(&entry?.path())?;
        }
    }
    Ok(())
}

/// One captured source, staged and checked, ready for publication.
#[derive(Debug)]
pub struct Capture {
    pub staging: Staging,
    pub manifest: Vec<u8>,
    /// The lowercase SHA-256 digest of the manifest.
    pub digest: String,
}

impl Capture {
    pub fn tree(&self) -> PathBuf {
        self.staging.path.join(TREE_DIR)
    }
}

/// Copy the whole canonical source directory `root` into new staging beneath
/// `staging_parent`. `mandatory` holds the configured mandatory paths of this
/// source, so that an unreadable one reports as mandatory guidance.
pub fn capture(
    root: &Path,
    staging_parent: &Path,
    source: &Located,
    mandatory: &BTreeSet<String>,
) -> Result<Capture, Refusal> {
    let mut staging = Staging::new(staging_parent, "capture-").map_err(|error| {
        refuse(
            "skill_cache_unavailable",
            format!(
                "Ravel cannot create private staging beneath {}: {error}",
                staging_parent.display()
            ),
            "make the cache root a writable private directory of this account, or free storage",
        )
        .source(staging_parent.display().to_string())
    })?;
    let tree = staging.path.join(TREE_DIR);
    let mut walk = Walk {
        root,
        source,
        mandatory,
        entries: Vec::new(),
    };
    if let Err(refusal) = walk.directory(root, "", &tree) {
        return Err(after_cleanup(refusal, &mut staging));
    }
    let mut entries = walk.entries;
    entries.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    let manifest = manifest(&entries);
    let digest = hex(&Sha256::digest(&manifest));
    Ok(Capture {
        staging,
        manifest,
        digest,
    })
}

struct Walk<'a> {
    root: &'a Path,
    source: &'a Located,
    mandatory: &'a BTreeSet<String>,
    entries: Vec<Entry>,
}

impl Walk<'_> {
    fn directory(&mut self, from: &Path, relative: &str, to: &Path) -> Result<(), Refusal> {
        fs::DirBuilder::new()
            .mode(0o700)
            .create(to)
            .map_err(|error| self.staging_failed(to, &error))?;
        let listing = fs::read_dir(from).map_err(|error| self.unreadable(from, &error))?;
        for entry in listing {
            let entry = entry.map_err(|error| self.unreadable(from, &error))?;
            let raw = entry.file_name();
            let Some(name) = raw.to_str() else {
                return Err(self.invalid(
                    &entry.path(),
                    "has a name that is not exact UTF-8".to_owned(),
                ));
            };
            if name.chars().any(char::is_control) {
                return Err(self.invalid(
                    &entry.path(),
                    "has a name that holds a control character".to_owned(),
                ));
            }
            if RESERVED.contains(&name) {
                continue;
            }
            let path = if relative.is_empty() {
                name.to_owned()
            } else {
                format!("{relative}/{name}")
            };
            let origin = entry.path();
            let target = to.join(name);
            let metadata =
                fs::symlink_metadata(&origin).map_err(|error| self.unreadable(&origin, &error))?;
            let kind = metadata.file_type();
            if kind.is_dir() {
                self.entries.push(Entry {
                    path: path.clone(),
                    kind: Kind::Directory,
                });
                self.directory(&origin, &path, &target)?;
            } else if kind.is_file() {
                let executable = metadata.permissions().mode() & 0o111 != 0;
                let digest = self.copy(&origin, &path, &target, executable)?;
                self.entries.push(Entry {
                    path,
                    kind: Kind::File { executable, digest },
                });
            } else if kind.is_symlink() {
                let link = self.link(&origin, &path)?;
                std::os::unix::fs::symlink(&link, &target)
                    .map_err(|error| self.staging_failed(&target, &error))?;
                self.entries.push(Entry {
                    path,
                    kind: Kind::Link { target: link },
                });
            } else {
                return Err(self.invalid(
                    &origin,
                    "is not a regular file, a directory or a symbolic link".to_owned(),
                ));
            }
        }
        Ok(())
    }

    /// Copy the bytes of one regular file and return their digest. Capture
    /// copies bytes, so a hard link in the source never backs a snapshot.
    fn copy(
        &self,
        from: &Path,
        relative: &str,
        to: &Path,
        executable: bool,
    ) -> Result<String, Refusal> {
        let mut input = File::open(from).map_err(|error| {
            if self.mandatory.contains(relative) {
                refuse(
                    "skill_mandatory_invalid",
                    format!(
                        "the mandatory file {} of the source {:?} cannot be read: {error}",
                        from.display(),
                        self.source.id
                    ),
                    "make the mandatory file readable, or change the mandatory reference deliberately",
                )
                .source(from.display().to_string())
                .location(self.source.location.clone())
            } else {
                self.unreadable(from, &error)
            }
        })?;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            // The staged mode carries the normalised executable bit to `seal`.
            .mode(if executable { 0o700 } else { 0o600 })
            .open(to)
            .map_err(|error| self.staging_failed(to, &error))?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 64 * 1024];
        loop {
            let read = input
                .read(&mut buffer)
                .map_err(|error| self.unreadable(from, &error))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
            output
                .write_all(&buffer[..read])
                .map_err(|error| self.staging_failed(to, &error))?;
        }
        output
            .sync_all()
            .map_err(|error| self.staging_failed(to, &error))?;
        Ok(hex(&hasher.finalize()))
    }

    /// Admit a relative link whose whole chain of targets ends inside the
    /// source, with no cycle and no reserved entry, and return its target.
    fn link(&self, origin: &Path, relative: &str) -> Result<String, Refusal> {
        let target = fs::read_link(origin).map_err(|error| self.unreadable(origin, &error))?;
        let target = self.link_text(origin, &target)?;
        let mut current: Vec<String> = relative.split('/').map(str::to_owned).collect();
        current.pop();
        let mut pending: Vec<String> = segments(&target);
        let mut hops = 0;
        while !pending.is_empty() {
            let segment = pending.remove(0);
            match segment.as_str() {
                "" | "." => {}
                ".." => {
                    if current.pop().is_none() {
                        return Err(self.link_refused(origin, &target, "leaves the source"));
                    }
                }
                name => {
                    if RESERVED.contains(&name) {
                        return Err(self.link_refused(
                            origin,
                            &target,
                            "reaches an entry that capture leaves out",
                        ));
                    }
                    current.push(name.to_owned());
                    let reached = self.root.join(current.join("/"));
                    let metadata = match fs::symlink_metadata(&reached) {
                        Ok(metadata) => metadata,
                        Err(error) if error.kind() == ErrorKind::NotFound => {
                            return Err(self.link_refused(origin, &target, "is dangling"));
                        }
                        Err(error) => return Err(self.unreadable(&reached, &error)),
                    };
                    if metadata.file_type().is_symlink() {
                        hops += 1;
                        if hops > LINK_HOPS {
                            return Err(self.link_refused(origin, &target, "is a cycle"));
                        }
                        let next = fs::read_link(&reached)
                            .map_err(|error| self.unreadable(&reached, &error))?;
                        let next = self.link_text(&reached, &next)?;
                        current.pop();
                        let mut chained = segments(&next);
                        chained.extend(pending);
                        pending = chained;
                    } else if !pending.is_empty() && !metadata.is_dir() {
                        return Err(self.link_refused(origin, &target, "is dangling"));
                    }
                }
            }
        }
        Ok(target)
    }

    fn link_text(&self, origin: &Path, target: &Path) -> Result<String, Refusal> {
        let Some(text) = target.to_str() else {
            return Err(self.invalid(
                origin,
                "is a link whose target is not exact UTF-8".to_owned(),
            ));
        };
        if text.is_empty() || text.chars().any(char::is_control) {
            return Err(self.invalid(
                origin,
                format!("is a link whose target {text:?} is empty or holds a control character"),
            ));
        }
        if text.starts_with('/') {
            return Err(self.link_refused(origin, text, "is absolute"));
        }
        Ok(text.to_owned())
    }

    fn link_refused(&self, origin: &Path, target: &str, why: &str) -> Refusal {
        self.invalid(origin, format!("is a link to {target:?} that {why}"))
    }

    fn invalid(&self, path: &Path, what: String) -> Refusal {
        refuse(
            "skill_resource_invalid",
            format!(
                "the path {} in the source {:?} {what}",
                path.display(),
                self.source.id
            ),
            "repair or remove the named entry in the source; a snapshot keeps only regular files, \
             directories and relative links that stay inside the source",
        )
        .source(path.display().to_string())
        .location(self.source.location.clone())
    }

    fn unreadable(&self, path: &Path, error: &io::Error) -> Refusal {
        self.invalid(path, format!("cannot be read: {error}"))
    }

    fn staging_failed(&self, path: &Path, error: &io::Error) -> Refusal {
        refuse(
            "skill_cache_unavailable",
            format!(
                "Ravel cannot write the private staging path {} for the source {:?}: {error}",
                path.display(),
                self.source.id
            ),
            "make the cache root writable for this account, or free storage",
        )
        .source(path.display().to_string())
        .location(self.source.location.clone())
    }
}

fn segments(text: &str) -> Vec<String> {
    text.split('/').map(str::to_owned).collect()
}

/// The versioned, sorted and length-delimited manifest: one line for each
/// entry, with its type, its normalised executable bit, its path and either
/// its content digest or its link target.
fn manifest(entries: &[Entry]) -> Vec<u8> {
    let mut manifest = MANIFEST_HEADER.to_vec();
    for entry in entries {
        let (kind, executable, payload) = match &entry.kind {
            Kind::Directory => ('d', false, ""),
            Kind::File { executable, digest } => ('f', *executable, digest.as_str()),
            Kind::Link { target } => ('l', false, target.as_str()),
        };
        let line = format!(
            "{kind} {} {}:{} {}:{}\n",
            u8::from(executable),
            entry.path.len(),
            entry.path,
            payload.len(),
            payload
        );
        manifest.extend_from_slice(line.as_bytes());
    }
    manifest
}

pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut text, byte| {
        let _ = write!(text, "{byte:02x}");
        text
    })
}

/// Remove write permission and set-ID bits from a staged tree, deepest first.
/// A file keeps the difference between executable and not.
fn seal(path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    let kind = metadata.file_type();
    if kind.is_dir() {
        for entry in fs::read_dir(path)? {
            seal(&entry?.path())?;
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o555))
    } else if kind.is_file() {
        let mode = if metadata.permissions().mode() & 0o111 != 0 {
            0o555
        } else {
            0o444
        };
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
    } else {
        Ok(())
    }
}

/// Where a published object came from: a new publication or an existing
/// object with the same identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Published {
    New,
    Existing,
}

/// Publish a staged object directory as `parent/name`, with an atomic rename
/// that never replaces an existing object. `marker` is the file inside the
/// object whose bytes identify it, and `expected` its bytes. An existing
/// object with the same bytes is reused, and one with other bytes is corrupt.
pub fn publish(
    mut staging: Staging,
    parent: &Path,
    name: &str,
    marker: &str,
    expected: &[u8],
) -> Result<(PathBuf, Published), Refusal> {
    let object = parent.join(name);
    let sealed = seal_contents(staging.path());
    let renamed = sealed.and_then(|()| rename_noreplace(staging.path(), &object));
    match renamed {
        Ok(()) => {
            staging.disarm();
            fs::set_permissions(&object, fs::Permissions::from_mode(0o555))
                .map_err(|error| unavailable(&object, &error))?;
            Ok((object, Published::New))
        }
        Err(error) if matches!(error.raw_os_error(), Some(libc::EEXIST | libc::ENOTEMPTY)) => {
            let existing = fs::read(object.join(marker));
            if existing.as_deref().ok() != Some(expected) {
                let failure = refuse(
                    "skill_cache_corrupt",
                    format!(
                        "the cached object {} does not hold the content that its name identifies",
                        object.display()
                    ),
                    "stop the readers of the object, then quarantine or remove it and resolve again",
                )
                .source(object.display().to_string());
                return Err(after_cleanup(failure, &mut staging));
            }
            staging
                .cleanup()
                .map_err(|error| cleanup_failed(staging.path(), &error))?;
            Ok((object, Published::Existing))
        }
        Err(error) => Err(after_cleanup(unavailable(&object, &error), &mut staging)),
    }
}

/// Seal every entry inside the staged object, but not the object directory,
/// which the rename must still be able to move.
fn seal_contents(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        seal(&entry?.path())?;
    }
    Ok(())
}

pub fn unavailable(path: &Path, error: &io::Error) -> Refusal {
    refuse(
        "skill_cache_unavailable",
        format!(
            "Ravel cannot use the cache path {}: {error}",
            path.display()
        ),
        "make the cache root a writable private directory of this account, or free storage",
    )
    .source(path.display().to_string())
}

/// Remove the staging of a refused resolution. A cleanup failure joins the
/// original refusal, which stays the reported failure.
pub fn after_cleanup(refusal: Refusal, staging: &mut Staging) -> Refusal {
    match staging.cleanup() {
        Ok(()) => refusal,
        Err(error) => refusal.report(
            &format!(
                "the staging {} was not removed: {error}",
                staging.path().display()
            ),
            "remove the named staging directory when no resolver runs",
        ),
    }
}

pub fn cleanup_failed(path: &Path, error: &io::Error) -> Refusal {
    Refusal::new(
        "skill_cleanup_failed",
        crate::refusal::Stage::Skills,
        crate::refusal::EXIT_WORKER,
        format!(
            "Ravel cannot remove its staging {}: {error}",
            path.display()
        ),
        "remove the named staging directory when no resolver runs, then resolve again",
    )
    .source(path.display().to_string())
}

fn c_path(path: &Path) -> io::Result<CString> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "a path holds NUL"))
}

/// Rename `from` to `to`, failing with `EEXIST` when `to` exists.
// https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/renamex_np.2.html
#[cfg(target_os = "macos")]
fn rename_noreplace(from: &Path, to: &Path) -> io::Result<()> {
    let (from, to) = (c_path(from)?, c_path(to)?);
    // SAFETY: both arguments are valid NUL-terminated paths for this call.
    let result = unsafe { libc::renamex_np(from.as_ptr(), to.as_ptr(), libc::RENAME_EXCL) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Rename `from` to `to`, failing with `EEXIST` when `to` exists.
// https://man7.org/linux/man-pages/man2/renameat2.2.html
#[cfg(target_os = "linux")]
fn rename_noreplace(from: &Path, to: &Path) -> io::Result<()> {
    let (from, to) = (c_path(from)?, c_path(to)?);
    // SAFETY: both arguments are valid NUL-terminated paths for this call.
    let result = unsafe {
        libc::renameat2(
            libc::AT_FDCWD,
            from.as_ptr(),
            libc::AT_FDCWD,
            to.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_manifest_is_versioned_sorted_and_length_delimited() {
        let entries = [
            Entry {
                path: "a".into(),
                kind: Kind::Directory,
            },
            Entry {
                path: "a/x y".into(),
                kind: Kind::File {
                    executable: true,
                    digest: "ab".into(),
                },
            },
            Entry {
                path: "l".into(),
                kind: Kind::Link {
                    target: "a/x y".into(),
                },
            },
        ];
        assert_eq!(
            String::from_utf8(manifest(&entries)).unwrap(),
            "ravel-skill-manifest\n1\nd 0 1:a 0:\nf 1 5:a/x y 2:ab\nl 0 1:l 5:a/x y\n"
        );
    }

    #[test]
    fn publication_never_replaces_an_existing_object() {
        let cache = tempfile::tempdir().unwrap();
        let stage = |text: &[u8]| {
            let staging = Staging::new(cache.path(), "s-").unwrap();
            fs::write(staging.path().join("m"), text).unwrap();
            staging
        };
        let (object, how) = publish(stage(b"one"), cache.path(), "obj", "m", b"one").unwrap();
        assert_eq!(how, Published::New);
        let (again, how) = publish(stage(b"one"), cache.path(), "obj", "m", b"one").unwrap();
        assert_eq!((again, how), (object.clone(), Published::Existing));
        let refusal = publish(stage(b"two"), cache.path(), "obj", "m", b"two").unwrap_err();
        assert_eq!(refusal.code, "skill_cache_corrupt");
        assert_eq!(fs::read(object.join("m")).unwrap(), b"one");
        // Only the published object remains: each loser removed its staging.
        let names: Vec<_> = fs::read_dir(cache.path())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(names, ["obj"]);
        writable(&object).unwrap();
    }
}
