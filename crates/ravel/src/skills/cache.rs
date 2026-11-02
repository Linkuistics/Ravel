//! The private cache root and its held directories
//! (docs/specs/skill-discovery.md#publication-concurrency-and-cleanup).
//!
//! Ravel reaches `$HOME/.cache/ravel/skills/v1` one segment at a time and
//! checks each directory that it passes, so that another account cannot
//! substitute a segment. It then holds the root and its three directories
//! open. Every later cache write goes through those descriptors, and each
//! publication first checks that the cache path still reaches the held root.

use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::io::{self, ErrorKind};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Component, Path, PathBuf};

use super::dir::{Dir, Stat, Type};
use super::{CACHE_ROOT, refuse};
use crate::refusal::Refusal;

/// More link hops than this on the cache path is a cycle.
const LINK_HOPS: usize = 40;
/// The directories of the cache root.
pub const STAGING: &str = "staging";
pub const SNAPSHOTS: &str = "snapshots";
pub const DESCRIPTORS: &str = "descriptors";

/// The held cache root and its three directories.
#[derive(Debug)]
pub struct Cache {
    /// `$HOME/.cache/ravel/skills/v1` as the account spells it.
    lexical: PathBuf,
    /// The real path of the root that Ravel holds.
    pub path: PathBuf,
    identity: (u64, u64),
    pub staging: Dir,
    pub snapshots: Dir,
    pub descriptors: Dir,
}

impl Cache {
    /// Reach, create where absent and hold the cache root of `home`.
    pub fn open(home: &Path) -> Result<Cache, Refusal> {
        let lexical = home.join(CACHE_ROOT);
        let (root, path) = walk(&lexical)?;
        let stat = root.stat().map_err(|error| unavailable(&path, &error))?;
        private(&path, &stat)?;
        let open = |name: &str| -> Result<Dir, Refusal> {
            let name = OsStr::new(name);
            let at = path.join(name);
            match root.mkdir(name, 0o700) {
                Ok(()) => {}
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => return Err(unavailable(&at, &error)),
            }
            let first = root
                .stat_at(name)
                .map_err(|error| unavailable(&at, &error))?;
            if first.kind() != Type::Directory {
                return Err(unsafe_path(&at, "is not a directory"));
            }
            let dir = root
                .open_dir(name)
                .map_err(|error| unavailable(&at, &error))?;
            let held = dir.stat().map_err(|error| unavailable(&at, &error))?;
            if held.identity() != first.identity() {
                return Err(replaced(&at));
            }
            private(&at, &held)?;
            Ok(dir)
        };
        let (staging, snapshots, descriptors) =
            (open(STAGING)?, open(SNAPSHOTS)?, open(DESCRIPTORS)?);
        Ok(Cache {
            lexical,
            path,
            identity: stat.identity(),
            staging,
            snapshots,
            descriptors,
        })
    }

    /// Check that the cache path of the account and the real path that
    /// Ravel reports both still reach the held root.
    pub fn check(&self) -> Result<(), Refusal> {
        #[cfg(test)]
        super::faults::publishing(&self.lexical);
        for path in [&self.lexical, &self.path] {
            let reached = std::fs::metadata(path).map(|reached| {
                use std::os::unix::fs::MetadataExt as _;
                (reached.dev(), reached.ino())
            });
            if reached.ok() != Some(self.identity) {
                return Err(replaced(path));
            }
        }
        Ok(())
    }
}

fn euid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and cannot fail.
    unsafe { libc::geteuid() }
}

/// A directory on the path must belong to the account or to root. A directory
/// that others can write must be sticky.
fn passable(path: &Path, stat: &Stat) -> Result<(), Refusal> {
    let uid = euid();
    if stat.uid != uid && stat.uid != 0 {
        return Err(unsafe_path(path, "belongs to another account"));
    }
    if stat.permissions() & 0o022 != 0 && stat.permissions() & 0o1000 == 0 {
        return Err(unsafe_path(
            path,
            "is writable by other accounts and is not sticky",
        ));
    }
    Ok(())
}

/// An entry in a directory that others can write must belong to the account
/// or to root, because the sticky bit protects only such entries.
fn kept(parent: &Path, parent_stat: &Stat, path: &Path, stat: &Stat) -> Result<(), Refusal> {
    let uid = euid();
    if parent_stat.permissions() & 0o022 != 0 && stat.uid != uid && stat.uid != 0 {
        return Err(unsafe_path(
            path,
            &format!(
                "belongs to another account inside {}, which other accounts can write",
                parent.display()
            ),
        ));
    }
    Ok(())
}

/// The cache root and its directories belong to the account and give no
/// access to other accounts.
fn private(path: &Path, stat: &Stat) -> Result<(), Refusal> {
    if stat.uid != euid() {
        return Err(unsafe_path(path, "belongs to another account"));
    }
    if stat.permissions() & 0o077 != 0 {
        return Err(unsafe_path(
            path,
            &format!(
                "gives other accounts access (mode {:o})",
                stat.permissions()
            ),
        ));
    }
    Ok(())
}

/// Reach `lexical` one segment at a time from `/`, following links by hand and
/// creating each absent directory with mode 0700. Return the held directory
/// and its real path.
fn walk(lexical: &Path) -> Result<(Dir, PathBuf), Refusal> {
    let root = PathBuf::from("/");
    let mut current = Dir::open(&root).map_err(|error| unavailable(&root, &error))?;
    let mut stat = current.stat().map_err(|error| unavailable(&root, &error))?;
    passable(&root, &stat)?;
    let mut pending: VecDeque<OsString> = segments(lexical);
    let mut hops = 0;
    while let Some(name) = pending.pop_front() {
        let parent = current.path().to_owned();
        if name == ".." {
            let next = current
                .open_dir(&name)
                .map_err(|error| unavailable(&parent, &error))?;
            let mut path = parent.clone();
            path.pop();
            current = Dir::open(&path).map_err(|error| unavailable(&path, &error))?;
            let held = next.stat().map_err(|error| unavailable(&path, &error))?;
            stat = current.stat().map_err(|error| unavailable(&path, &error))?;
            if held.identity() != stat.identity() {
                return Err(replaced(&path));
            }
            passable(&path, &stat)?;
            continue;
        }
        let path = parent.join(&name);
        let entry = match current.stat_at(&name) {
            Ok(entry) => entry,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                match current.mkdir(&name, 0o700) {
                    Ok(()) => {}
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(unavailable(&path, &error)),
                }
                current
                    .stat_at(&name)
                    .map_err(|error| unavailable(&path, &error))?
            }
            Err(error) => return Err(unavailable(&path, &error)),
        };
        kept(&parent, &stat, &path, &entry)?;
        match entry.kind() {
            Type::Link => {
                hops += 1;
                if hops > LINK_HOPS {
                    return Err(unsafe_path(&path, "is a link in a cycle"));
                }
                let target = current
                    .read_link(&name)
                    .map_err(|error| unavailable(&path, &error))?;
                let target = PathBuf::from(target);
                if target.is_absolute() {
                    current = Dir::open(&root).map_err(|error| unavailable(&root, &error))?;
                    stat = current.stat().map_err(|error| unavailable(&root, &error))?;
                }
                for segment in segments(&target).into_iter().rev() {
                    pending.push_front(segment);
                }
            }
            Type::Directory => {
                let next = current
                    .open_dir(&name)
                    .map_err(|error| unavailable(&path, &error))?;
                let held = next.stat().map_err(|error| unavailable(&path, &error))?;
                if held.identity() != entry.identity() {
                    return Err(replaced(&path));
                }
                passable(&path, &held)?;
                current = next;
                stat = held;
            }
            Type::File | Type::Other => {
                return Err(unsafe_path(&path, "is not a directory"));
            }
        }
    }
    let path = current.path().to_owned();
    Ok((current, path))
}

/// The normal segments of `path`, with `..` kept and `.` and `/` left out.
fn segments(path: &Path) -> VecDeque<OsString> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(name) => Some(name.to_owned()),
            Component::ParentDir => Some(OsString::from("..")),
            Component::RootDir | Component::CurDir | Component::Prefix(_) => None,
        })
        .collect()
}

/// A cache path that Ravel cannot use. The remedy follows the cause: full
/// storage, a permission or another failure.
pub fn unavailable(path: &Path, error: &io::Error) -> Refusal {
    let remedy = match error.raw_os_error() {
        Some(libc::ENOSPC | libc::EDQUOT) => {
            "free storage on the filesystem of the cache; to remove unused snapshots and \
             descriptors, first stop every reader of them, then resolve again"
        }
        Some(libc::EACCES | libc::EPERM | libc::EROFS) => {
            "give this account write permission on the cache root \
             ~/.cache/ravel/skills/v1 and its directories, then resolve again"
        }
        _ => "make the cache root a writable private directory of this account, or free storage",
    };
    refuse(
        "skill_cache_unavailable",
        format!(
            "Ravel cannot use the cache path {}: {error}",
            path.display()
        ),
        remedy,
    )
    .source(lossy(path))
}

fn unsafe_path(path: &Path, why: &str) -> Refusal {
    refuse(
        "skill_cache_unavailable",
        format!(
            "Ravel does not use the cache path {}, because it {why}",
            path.display()
        ),
        "make every directory on the path to ~/.cache/ravel/skills/v1 belong to this account \
         or to root and keep other accounts from writing it; the cache root and its \
         directories must give other accounts no access (chmod 700)",
    )
    .source(lossy(path))
}

fn replaced(path: &Path) -> Refusal {
    refuse(
        "skill_cache_unavailable",
        format!(
            "the cache path {} no longer reaches the directory that Ravel opened; a link or \
             a directory on it was replaced during the resolution",
            path.display()
        ),
        "keep the path to ~/.cache/ravel/skills/v1 unchanged while Ravel resolves, then \
         resolve again",
    )
    .source(lossy(path))
}

fn lossy(path: &Path) -> String {
    String::from_utf8_lossy(path.as_os_str().as_bytes()).into_owned()
}
