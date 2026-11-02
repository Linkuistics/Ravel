//! Capture of one local source into private staging, its manifest, the check
//! of the written tree and the atomic publication of retained objects
//! (docs/specs/skill-discovery.md#sources-and-retained-resources).
//!
//! Capture reads the source through held directory descriptors and looks at
//! each entry twice: when it lists the entry and after it copies the entry
//! and its directory. An observable change of entry, type, identity, size or
//! change time between the two looks refuses as `skill_source_changed`. This
//! is not an atomic snapshot against every concurrent writer: a writer that
//! restores the size and both change times between the looks is not
//! observable, and a successful retry proves nothing about atomicity.

use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, ErrorKind, Read as _, Write as _};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::MetadataExt as _;
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

use super::bounds::{self, Actual, Limits};
use super::cache::{Cache, unavailable};
use super::dir::{self, Dir, Stat, Type};
use super::watch::{Watch, stopped};
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Directory,
    File { executable: bool, digest: String },
    Link { target: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    path: String,
    kind: Kind,
}

/// A private staging directory that the resolver which made it removes. It
/// holds its parent and its own directory open.
#[derive(Debug)]
pub struct Staging {
    parent: Dir,
    name: OsString,
    dir: Dir,
    armed: bool,
}

impl Staging {
    /// A new directory in the staging directory of `cache` that only the
    /// owner can open. Its name is `prefix` and 32 random hexadecimal digits.
    pub fn new(cache: &Cache, prefix: &str) -> Result<Staging, Refusal> {
        let parent = cache
            .staging
            .try_clone()
            .map_err(|error| unavailable(cache.staging.path(), &error))?;
        loop {
            let mut bytes = [0u8; 16];
            getrandom::fill(&mut bytes).map_err(|error| {
                refuse(
                    "skill_cache_unavailable",
                    format!("the random source of the operating system failed: {error}"),
                    "repair the random source of the operating system, then resolve again",
                )
            })?;
            let name = OsString::from(format!("{prefix}{}", hex(&bytes)));
            let path = parent.path().join(&name);
            match parent.mkdir(&name, 0o700) {
                Ok(()) => {}
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(unavailable(&path, &error)),
            }
            let dir = parent
                .open_dir(&name)
                .map_err(|error| unavailable(&path, &error))?;
            return Ok(Staging {
                parent,
                name,
                dir,
                armed: true,
            });
        }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Remove this staging tree, and nothing else.
    pub fn cleanup(&mut self) -> io::Result<()> {
        if !self.armed {
            return Ok(());
        }
        self.armed = false;
        self.parent.remove_tree(&self.name)
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
        self.staging.path().join(TREE_DIR)
    }
}

/// The bounds of one resolution and the regular-file bytes of the sources
/// that it captured before this one.
pub struct Budget {
    pub limits: Limits,
    pub total_bytes: u64,
}

/// Copy the whole canonical source directory `root` into new staging in
/// `cache`, then check the written tree against the manifest. `mandatory`
/// holds the configured mandatory paths of this source, so that an unreadable
/// one reports as mandatory guidance. The bytes of this source join
/// `budget` when the capture succeeds.
pub fn capture(
    root: &Path,
    cache: &Cache,
    source: &Located,
    mandatory: &BTreeSet<String>,
    watch: &Watch,
    budget: &mut Budget,
) -> Result<Capture, Refusal> {
    let mut staging = Staging::new(cache, "capture-")?;
    match staged(root, &staging, source, mandatory, watch, budget) {
        Ok((manifest, digest)) => Ok(Capture {
            staging,
            manifest,
            digest,
        }),
        Err(refusal) => Err(after_cleanup(refusal, &mut staging)),
    }
}

fn staged(
    root: &Path,
    staging: &Staging,
    source: &Located,
    mandatory: &BTreeSet<String>,
    watch: &Watch,
    budget: &mut Budget,
) -> Result<(Vec<u8>, String), Refusal> {
    let mut walk = Walk {
        root,
        source,
        mandatory,
        watch,
        limits: budget.limits,
        before: budget.total_bytes,
        bytes: 0,
        entries: Vec::new(),
    };
    let from = Dir::open(root).map_err(|error| walk.unreadable(root, &error))?;
    let first = std::fs::symlink_metadata(root).map_err(|error| walk.unreadable(root, &error))?;
    let held = from.stat().map_err(|error| walk.unreadable(root, &error))?;
    let identity = (
        std::os::unix::fs::MetadataExt::dev(&first),
        std::os::unix::fs::MetadataExt::ino(&first),
    );
    if !first.is_dir() || identity != held.identity() {
        return Err(walk.changed(root, "was replaced when Ravel opened it"));
    }
    let tree = OsStr::new(TREE_DIR);
    staging
        .dir
        .mkdir(tree, 0o700)
        .map_err(|error| walk.staging_failed(&staging.path().join(tree), &error))?;
    let to = staging
        .dir
        .open_dir(tree)
        .map_err(|error| walk.staging_failed(&staging.path().join(tree), &error))?;
    walk.directory(&from, "", &to)?;
    let mut entries = std::mem::take(&mut walk.entries);
    entries.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    let manifest = manifest(&entries);

    // The written tree must hold exactly the entries of the manifest, with
    // every directory segment, or the cache filesystem did not keep them.
    let mut written = Vec::new();
    read_tree(&to, "", &mut written, watch).map_err(|error| {
        stopped(error).unwrap_or_else(|error| walk.staging_failed(to.path(), &error))
    })?;
    written.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    if written != entries {
        return Err(not_kept(root, source, &entries, &written));
    }
    let digest = hex(&Sha256::digest(&manifest));
    budget.total_bytes += walk.bytes;
    Ok((manifest, digest))
}

/// The refusal for a written tree that differs from the source manifest: a
/// path that the cache filesystem stored with another spelling, or bytes
/// that storage did not keep.
fn not_kept(root: &Path, source: &Located, entries: &[Entry], written: &[Entry]) -> Refusal {
    let paths = |list: &[Entry]| -> BTreeSet<String> {
        list.iter().map(|entry| entry.path.clone()).collect()
    };
    let (expected, actual) = (paths(entries), paths(written));
    if let Some(missing) = expected.difference(&actual).next() {
        let stored = actual
            .difference(&expected)
            .next()
            .map_or_else(String::new, |stored| {
                format!("; the cache holds {stored:?} in its place")
            });
        let path = root.join(missing);
        return refuse(
            "skill_resource_invalid",
            format!(
                "the cache filesystem does not keep the exact spelling of the path {} in the \
                 source {:?}{stored}",
                path.display(),
                source.id
            ),
            "rename the entry in the source so that its spelling survives on the filesystem of \
             ~/.cache/ravel/skills/v1, or keep the cache on a filesystem that keeps names \
             exactly; Ravel never normalises a name",
        )
        .source(path.display().to_string())
        .location(source.location.clone());
    }
    refuse(
        "skill_cache_unavailable",
        format!(
            "the staged copy of the source {:?} does not hold the bytes that Ravel read",
            source.id
        ),
        "check the storage of ~/.cache/ravel/skills/v1, then resolve again",
    )
    .location(source.location.clone())
}

/// The second look at an entry differs from the first in a way that a change
/// would show.
fn moved(first: &Stat, second: &Stat) -> bool {
    first.identity() != second.identity()
        || first.kind() != second.kind()
        || first.size != second.size
        || first.mtime != second.mtime
        || first.ctime != second.ctime
}

struct Walk<'a> {
    root: &'a Path,
    source: &'a Located,
    mandatory: &'a BTreeSet<String>,
    watch: &'a Watch,
    limits: Limits,
    /// The regular-file bytes of the sources before this one.
    before: u64,
    /// The regular-file bytes of this source so far.
    bytes: u64,
    entries: Vec<Entry>,
}

impl Walk<'_> {
    fn directory(&mut self, from: &Dir, relative: &str, to: &Dir) -> Result<(), Refusal> {
        let here = self.root.join(relative);
        let before = from
            .stat()
            .map_err(|error| self.unreadable(&here, &error))?;
        let names = from
            .list()
            .map_err(|error| self.unreadable(&here, &error))?;
        // The first look at each entry, and the identity of what Ravel wrote
        // for it, so that a collision can name the entry that it met.
        let mut looks: Vec<(OsString, Stat)> = Vec::with_capacity(names.len());
        let mut written: Vec<(String, (u64, u64))> = Vec::with_capacity(names.len());
        for raw in names {
            let origin = here.join(&raw);
            let Some(name) = raw.to_str() else {
                return Err(self.invalid(&origin, "has a name that is not exact UTF-8".to_owned()));
            };
            if name.chars().any(char::is_control) {
                return Err(self.invalid(
                    &origin,
                    "has a name that holds a control character".to_owned(),
                ));
            }
            if RESERVED.contains(&name) {
                continue;
            }
            self.watch.check()?;
            if self.entries.len() as u64 >= self.limits.source_entries {
                return Err(self.exceeded(
                    bounds::SOURCE_ENTRIES,
                    self.limits.source_entries,
                    Actual::AtLeast(self.entries.len() as u64 + 1),
                    &format!("the entries of the source {:?}", self.source.id),
                    self.root,
                ));
            }
            let name = name.to_owned();
            let path = if relative.is_empty() {
                name.clone()
            } else {
                format!("{relative}/{name}")
            };
            let first = match from.stat_at(&raw) {
                Ok(first) => first,
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    return Err(self.changed(&origin, "was removed during capture"));
                }
                Err(error) => return Err(self.unreadable(&origin, &error)),
            };
            let made = match first.kind() {
                Type::Directory => {
                    self.entries.push(Entry {
                        path: path.clone(),
                        kind: Kind::Directory,
                    });
                    let made = to.mkdir(&raw, 0o700);
                    if made.is_ok() {
                        let inner_from = from.open_dir(&raw).map_err(|error| {
                            self.changed(&origin, &format!("is no longer a directory: {error}"))
                        })?;
                        let opened = inner_from
                            .stat()
                            .map_err(|error| self.unreadable(&origin, &error))?;
                        if opened.identity() != first.identity() {
                            return Err(self.changed(&origin, "was replaced during capture"));
                        }
                        let inner_to = to
                            .open_dir(&raw)
                            .map_err(|error| self.staging_failed(&to.path().join(&raw), &error))?;
                        self.directory(&inner_from, &path, &inner_to)?;
                    }
                    made
                }
                Type::File => {
                    let made = self.copy(from, to, &raw, &origin, &path, &first);
                    match made {
                        Ok(entry) => {
                            self.entries.push(entry);
                            Ok(())
                        }
                        Err(Copy::Exists(error)) => Err(error),
                        Err(Copy::Refused(refusal)) => return Err(refusal),
                    }
                }
                Type::Link => {
                    let target = from
                        .read_link(&raw)
                        .map_err(|error| self.unreadable(&origin, &error))?;
                    let link = self.link(&origin, &path, &target)?;
                    let made = to.symlink(OsStr::new(&link), &raw);
                    self.entries.push(Entry {
                        path: path.clone(),
                        kind: Kind::Link { target: link },
                    });
                    made
                }
                Type::Other => {
                    return Err(self.invalid(
                        &origin,
                        "is not a regular file, a directory or a symbolic link".to_owned(),
                    ));
                }
            };
            match made {
                Ok(()) => {}
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                    return Err(self.collision(to, &raw, &path, &written));
                }
                Err(error) => return Err(self.staging_failed(&to.path().join(&raw), &error)),
            }
            let identity = to
                .stat_at(&raw)
                .map_err(|error| self.staging_failed(&to.path().join(&raw), &error))?
                .identity();
            written.push((path, identity));
            looks.push((raw, first));
        }
        #[cfg(test)]
        super::faults::listed(&here);

        // The second look: the same names, and each entry and the directory
        // itself unchanged since the first look.
        let again = from
            .list()
            .map_err(|error| self.unreadable(&here, &error))?;
        let kept: Vec<&OsString> = again
            .iter()
            .filter(|name| {
                !RESERVED
                    .iter()
                    .any(|reserved| name.as_bytes() == reserved.as_bytes())
            })
            .collect();
        if kept.len() != looks.len()
            || kept
                .iter()
                .zip(&looks)
                .any(|(name, (seen, _))| *name != seen)
        {
            return Err(self.changed(&here, "gained or lost an entry during capture"));
        }
        for (raw, first) in &looks {
            let origin = here.join(raw);
            let second = from
                .stat_at(raw)
                .map_err(|_| self.changed(&origin, "was removed during capture"))?;
            if moved(first, &second) {
                return Err(self.changed(&origin, "changed during capture"));
            }
        }
        let after = from
            .stat()
            .map_err(|error| self.unreadable(&here, &error))?;
        if moved(&before, &after) {
            return Err(self.changed(&here, "changed during capture"));
        }
        Ok(())
    }

    /// Copy the bytes of one regular file and return its entry. Capture
    /// copies bytes, so a hard link in the source never backs a snapshot.
    fn copy(
        &mut self,
        from: &Dir,
        to: &Dir,
        raw: &OsStr,
        origin: &Path,
        relative: &str,
        first: &Stat,
    ) -> Result<Entry, Copy> {
        let size = u64::try_from(first.size).unwrap_or(0);
        self.admit(origin, size).map_err(Copy::Refused)?;
        let mut input = from.open_file(raw).map_err(|error| {
            Copy::Refused(match error.raw_os_error() {
                Some(libc::ELOOP | libc::ENOENT) => {
                    self.changed(origin, "was replaced or removed during capture")
                }
                _ if self.mandatory.contains(relative) => refuse(
                    "skill_mandatory_invalid",
                    format!(
                        "the mandatory file {} of the source {:?} cannot be read: {error}",
                        origin.display(),
                        self.source.id
                    ),
                    "make the mandatory file readable, or change the mandatory reference \
                     deliberately",
                )
                .source(origin.display().to_string())
                .location(self.source.location.clone()),
                _ => self.unreadable(origin, &error),
            })
        })?;
        let opened = dir::stat_file(&input)
            .map_err(|error| Copy::Refused(self.unreadable(origin, &error)))?;
        if moved(first, &opened) {
            return Err(Copy::Refused(
                self.changed(origin, "changed when Ravel opened it"),
            ));
        }
        let executable = first.permissions() & 0o111 != 0;
        // The staged mode carries the normalised executable bit to `seal`.
        let mut output = match to.create(raw, if executable { 0o700 } else { 0o600 }) {
            Ok(output) => output,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                return Err(Copy::Exists(error));
            }
            Err(error) => {
                return Err(Copy::Refused(
                    self.staging_failed(&to.path().join(raw), &error),
                ));
            }
        };
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 64 * 1024];
        let mut copied: u64 = 0;
        loop {
            self.watch.check().map_err(Copy::Refused)?;
            let read = input
                .read(&mut buffer)
                .map_err(|error| Copy::Refused(self.unreadable(origin, &error)))?;
            if read == 0 {
                break;
            }
            copied += read as u64;
            // A file that grows past its first size changed, and Ravel stops
            // before the growth can pass a bound.
            if copied > size {
                return Err(Copy::Refused(
                    self.changed(origin, "changed while Ravel copied it"),
                ));
            }
            hasher.update(&buffer[..read]);
            output.write_all(&buffer[..read]).map_err(|error| {
                Copy::Refused(self.staging_failed(&to.path().join(raw), &error))
            })?;
        }
        dir::flush_file(&output)
            .map_err(|error| Copy::Refused(self.staging_failed(&to.path().join(raw), &error)))?;
        #[cfg(test)]
        super::faults::copied(origin);
        let after = dir::stat_file(&input)
            .map_err(|error| Copy::Refused(self.unreadable(origin, &error)))?;
        if moved(first, &after) || u64::try_from(after.size).ok() != Some(copied) {
            return Err(Copy::Refused(
                self.changed(origin, "changed while Ravel copied it"),
            ));
        }
        self.bytes += copied;
        Ok(Entry {
            path: relative.to_owned(),
            kind: Kind::File {
                executable,
                digest: hex(&hasher.finalize()),
            },
        })
    }

    /// Refuse a regular file of `size` bytes that would pass a byte bound,
    /// before Ravel copies any of its bytes. The bound of a mandatory file
    /// has its check with the mandatory reference, which a link can reach.
    fn admit(&self, origin: &Path, size: u64) -> Result<(), Refusal> {
        let limits = self.limits;
        if size > limits.resource_bytes {
            return Err(self.exceeded(
                bounds::RESOURCE_BYTES,
                limits.resource_bytes,
                Actual::Exact(size),
                &format!("the file {}", origin.display()),
                origin,
            ));
        }
        let source = self.bytes + size;
        if source > limits.source_bytes {
            return Err(self.exceeded(
                bounds::SOURCE_BYTES,
                limits.source_bytes,
                Actual::AtLeast(source),
                &format!("the regular files of the source {:?}", self.source.id),
                origin,
            ));
        }
        let total = self.before + source;
        if total > limits.total_bytes {
            return Err(self.exceeded(
                bounds::TOTAL_BYTES,
                limits.total_bytes,
                Actual::AtLeast(total),
                "the regular files of all sources",
                origin,
            ));
        }
        Ok(())
    }

    fn exceeded(
        &self,
        subject: bounds::Subject,
        bound: u64,
        actual: Actual,
        what: &str,
        path: &Path,
    ) -> Refusal {
        bounds::exceeded(subject, bound, actual, what)
            .source(path.display().to_string())
            .location(self.source.location.clone())
    }

    /// Admit a relative link whose whole chain of targets ends inside the
    /// source, with no cycle and no reserved entry, and return its target.
    fn link(&self, origin: &Path, relative: &str, target: &OsStr) -> Result<String, Refusal> {
        let target = self.link_text(origin, target)?;
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
                    let metadata = match std::fs::symlink_metadata(&reached) {
                        Ok(metadata) => metadata,
                        Err(error) if error.kind() == ErrorKind::NotFound => {
                            return Err(self.link_refused(origin, &target, "is dangling"));
                        }
                        Err(error) => return Err(self.unreadable(&reached, &error)),
                    };
                    if self.reserved_alias(&reached, name, &metadata)? {
                        return Err(self.link_refused(
                            origin,
                            &target,
                            "reaches an entry that capture leaves out",
                        ));
                    }
                    if metadata.file_type().is_symlink() {
                        hops += 1;
                        if hops > LINK_HOPS {
                            return Err(self.link_refused(origin, &target, "is a cycle"));
                        }
                        let next = std::fs::read_link(&reached)
                            .map_err(|error| self.unreadable(&reached, &error))?;
                        let next = self.link_text(&reached, next.as_os_str())?;
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

    /// Whether the entry `reached`, which a link names `name`, is a reserved
    /// entry of its directory under another spelling. A case-insensitive or
    /// normalising filesystem resolves such a spelling, but capture leaves
    /// the stored entry out. A listed entry with the exact name is a distinct
    /// entry, such as a hard link to a reserved file, and capture keeps it.
    fn reserved_alias(
        &self,
        reached: &Path,
        name: &str,
        metadata: &std::fs::Metadata,
    ) -> Result<bool, Refusal> {
        let Some(parent) = reached.parent() else {
            return Ok(false);
        };
        let identity = (metadata.dev(), metadata.ino());
        let reserved = RESERVED.iter().any(|reserved| {
            std::fs::symlink_metadata(parent.join(reserved))
                .is_ok_and(|entry| (entry.dev(), entry.ino()) == identity)
        });
        if !reserved {
            return Ok(false);
        }
        let listing = std::fs::read_dir(parent).map_err(|error| self.unreadable(parent, &error))?;
        for entry in listing {
            let entry = entry.map_err(|error| self.unreadable(parent, &error))?;
            if entry.file_name().as_bytes() == name.as_bytes() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn link_text(&self, origin: &Path, target: &OsStr) -> Result<String, Refusal> {
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

    /// Two source entries met one name on the cache filesystem. Name both.
    fn collision(
        &self,
        to: &Dir,
        raw: &OsStr,
        path: &str,
        written: &[(String, (u64, u64))],
    ) -> Refusal {
        let met = to
            .stat_at(raw)
            .ok()
            .and_then(|stat| {
                written
                    .iter()
                    .find(|(_, identity)| *identity == stat.identity())
            })
            .map(|(other, _)| other.clone());
        let this = self.root.join(path);
        let message = match &met {
            Some(other) => format!(
                "the paths {} and {} in the source {:?} are distinct, but the cache filesystem \
                 cannot keep both",
                self.root.join(other).display(),
                this.display(),
                self.source.id
            ),
            None => format!(
                "the cache filesystem cannot keep the path {} in the source {:?} apart from \
                 another entry",
                this.display(),
                self.source.id
            ),
        };
        refuse(
            "skill_resource_invalid",
            message,
            "rename one of the entries in the source so that their names differ in more than \
             case or Unicode normalisation, or keep the cache on a filesystem that keeps both; \
             Ravel never writes one entry over another",
        )
        .source(this.display().to_string())
        .location(self.source.location.clone())
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

    fn changed(&self, path: &Path, what: &str) -> Refusal {
        refuse(
            "skill_source_changed",
            format!(
                "the path {} in the source {:?} {what}",
                path.display(),
                self.source.id
            ),
            "keep the source directory quiet while Ravel captures it, then try again",
        )
        .source(path.display().to_string())
        .location(self.source.location.clone())
    }

    fn unreadable(&self, path: &Path, error: &io::Error) -> Refusal {
        self.invalid(path, format!("cannot be read: {error}"))
    }

    fn staging_failed(&self, path: &Path, error: &io::Error) -> Refusal {
        unavailable(path, error).location(self.source.location.clone())
    }
}

/// A file copy that met an existing name, or that refused.
enum Copy {
    Exists(io::Error),
    Refused(Refusal),
}

fn segments(text: &str) -> Vec<String> {
    text.split('/').map(str::to_owned).collect()
}

/// The entries of a written tree, with the content digest of each file read
/// back from storage. A name that is not UTF-8 keeps its lossy spelling, which
/// can match no manifest path.
fn read_tree(dir: &Dir, relative: &str, entries: &mut Vec<Entry>, watch: &Watch) -> io::Result<()> {
    for raw in dir.list()? {
        watch.tick()?;
        let name = String::from_utf8_lossy(raw.as_bytes());
        let path = if relative.is_empty() {
            name.into_owned()
        } else {
            format!("{relative}/{name}")
        };
        let stat = dir.stat_at(&raw)?;
        match stat.kind() {
            Type::Directory => {
                entries.push(Entry {
                    path: path.clone(),
                    kind: Kind::Directory,
                });
                read_tree(&dir.open_dir(&raw)?, &path, entries, watch)?;
            }
            Type::File => {
                let mut file = dir.open_file(&raw)?;
                let mut hasher = Sha256::new();
                io::copy(&mut file, &mut hasher)?;
                entries.push(Entry {
                    path,
                    kind: Kind::File {
                        executable: stat.permissions() & 0o111 != 0,
                        digest: hex(&hasher.finalize()),
                    },
                });
            }
            Type::Link => {
                let target = dir.read_link(&raw)?;
                entries.push(Entry {
                    path,
                    kind: Kind::Link {
                        target: String::from_utf8_lossy(target.as_bytes()).into_owned(),
                    },
                });
            }
            Type::Other => entries.push(Entry {
                path,
                kind: Kind::Link {
                    target: "\0special".to_owned(),
                },
            }),
        }
    }
    Ok(())
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

/// Remove write permission and set-ID bits from every entry beneath `dir`,
/// deepest first, but not from `dir`, which the rename must still be able to
/// move. A file keeps the difference between executable and not.
fn seal_contents(dir: &Dir, watch: &Watch) -> io::Result<()> {
    for raw in dir.list()? {
        watch.tick()?;
        match dir.stat_at(&raw)?.kind() {
            Type::Directory => {
                let inner = dir.open_dir(&raw)?;
                seal_contents(&inner, watch)?;
                inner.chmod(0o555)?;
            }
            Type::File => {
                let file = dir.open_file(&raw)?;
                let executable = dir::stat_file(&file)?.permissions() & 0o111 != 0;
                dir::chmod_file(&file, if executable { 0o555 } else { 0o444 })?;
            }
            Type::Link | Type::Other => {}
        }
    }
    Ok(())
}

/// Where a published object came from: a new publication or an existing
/// object with the same identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Published {
    New,
    Existing,
}

/// What an object must hold: a snapshot with its manifest and tree, or a
/// descriptor directory with its one file.
pub enum Expected<'a> {
    Snapshot { manifest: &'a [u8] },
    Descriptor { name: &'a str, bytes: &'a [u8] },
}

impl Expected<'_> {
    /// Whether the object `dir` holds exactly what it must, read back in
    /// full.
    fn holds(&self, dir: &Dir, watch: &Watch) -> io::Result<bool> {
        let names = dir.list()?;
        let read = |name: &str| -> io::Result<Vec<u8>> {
            let mut bytes = Vec::new();
            dir.open_file(OsStr::new(name))?.read_to_end(&mut bytes)?;
            Ok(bytes)
        };
        match self {
            Expected::Snapshot { manifest: expected } => {
                if names != [OsStr::new(MANIFEST_FILE), OsStr::new(TREE_DIR)] {
                    return Ok(false);
                }
                if read(MANIFEST_FILE)? != *expected {
                    return Ok(false);
                }
                let mut entries = Vec::new();
                read_tree(
                    &dir.open_dir(OsStr::new(TREE_DIR))?,
                    "",
                    &mut entries,
                    watch,
                )?;
                entries.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
                Ok(manifest(&entries) == *expected)
            }
            Expected::Descriptor { name, bytes } => {
                Ok(names == [OsStr::new(name)] && read(name)? == *bytes)
            }
        }
    }
}

/// Publish a staged object directory as `name` in `parent`, with an atomic
/// rename that never replaces an existing object. An existing object is a
/// winner: the loser reads it back in full and reuses it when it holds what
/// `expected` describes, or refuses it as corrupt. Ravel never repairs it.
pub fn publish(
    mut staging: Staging,
    cache: &Cache,
    parent: &Dir,
    name: &str,
    expected: &Expected<'_>,
    watch: &Watch,
) -> Result<(PathBuf, Published), Refusal> {
    let object = parent.path().join(name);
    if let Err(error) = seal_contents(&staging.dir, watch) {
        let refusal = stopped(error).unwrap_or_else(|error| unavailable(staging.path(), &error));
        return Err(after_cleanup(refusal, &mut staging));
    }
    // Each file had its own flush. One full flush of the device now orders
    // them before the rename.
    if let Err(error) = staging.dir.flush_device() {
        return Err(after_cleanup(
            unavailable(staging.path(), &error),
            &mut staging,
        ));
    }
    if let Err(refusal) = watch.check().and_then(|()| cache.check()) {
        return Err(after_cleanup(refusal, &mut staging));
    }
    let renamed = staging
        .parent
        .rename_noreplace(&staging.name, parent, OsStr::new(name));
    match renamed {
        Ok(()) => {
            staging.disarm();
            staging
                .dir
                .chmod(0o555)
                .map_err(|error| unavailable(&object, &error))?;
            Ok((object, Published::New))
        }
        Err(error) if matches!(error.raw_os_error(), Some(libc::EEXIST | libc::ENOTEMPTY)) => {
            let holds = parent
                .open_dir(OsStr::new(name))
                .and_then(|winner| expected.holds(&winner, watch));
            let holds = match holds {
                Ok(holds) => Ok(holds),
                Err(error) => match stopped(error) {
                    Ok(refusal) => return Err(after_cleanup(refusal, &mut staging)),
                    Err(error) => Err(error),
                },
            };
            match holds {
                Ok(true) => {
                    staging
                        .cleanup()
                        .map_err(|error| cleanup_failed(staging.path(), &error))?;
                    Ok((object, Published::Existing))
                }
                Ok(false) | Err(_) => {
                    let failure = refuse(
                        "skill_cache_corrupt",
                        format!(
                            "the cached object {} does not hold the content that its name \
                             identifies",
                            object.display()
                        ),
                        "stop the readers of the object, then move it out of the cache to \
                         quarantine it, or remove it, and resolve again",
                    )
                    .source(object.display().to_string());
                    Err(after_cleanup(failure, &mut staging))
                }
            }
        }
        Err(error) => Err(after_cleanup(unavailable(&object, &error), &mut staging)),
    }
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

/// Write `bytes` as the new file `name` in the staging directory.
pub fn write_new(staging: &Staging, name: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file: File = staging.dir.create(OsStr::new(name), 0o600)?;
    file.write_all(bytes)?;
    dir::flush_file(&file)
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
        let home = tempfile::tempdir().unwrap();
        let home = home.path().canonicalize().unwrap();
        let cache = Cache::open(&home).unwrap();
        let watch = Watch::unbounded();
        let stage = |text: &[u8]| {
            let staging = Staging::new(&cache, "s-").unwrap();
            write_new(&staging, "m", text).unwrap();
            staging
        };
        let expect = |bytes| Expected::Descriptor { name: "m", bytes };
        let (object, how) = publish(
            stage(b"one"),
            &cache,
            &cache.descriptors,
            "obj",
            &expect(b"one"),
            &watch,
        )
        .unwrap();
        assert_eq!(how, Published::New);
        let (again, how) = publish(
            stage(b"one"),
            &cache,
            &cache.descriptors,
            "obj",
            &expect(b"one"),
            &watch,
        )
        .unwrap();
        assert_eq!((again, how), (object.clone(), Published::Existing));
        let refusal = publish(
            stage(b"two"),
            &cache,
            &cache.descriptors,
            "obj",
            &expect(b"two"),
            &watch,
        )
        .unwrap_err();
        assert_eq!(refusal.code, "skill_cache_corrupt");
        assert_eq!(std::fs::read(object.join("m")).unwrap(), b"one");
        // Only the published object remains, and each loser removed its own
        // staging.
        assert_eq!(cache.descriptors.list().unwrap(), [OsStr::new("obj")]);
        assert!(cache.staging.list().unwrap().is_empty());
        let _ = std::process::Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&home)
            .status();
    }
}
