//! Online exact acquisition of one pinned Git source into private staging
//! (docs/specs/skill-discovery.md#exact-git-acquisition-and-offline-reuse).
//!
//! Ravel fetches the configured commit into a bare repository of its own,
//! checks the object format, the object type and `subdir`, and writes the
//! tracked objects of that tree directly. No working tree, index, checkout,
//! hook, filter, attribute conversion or submodule command takes part. The
//! extracted tree then goes through the capture of a local source, which owns
//! the link rules, the reserved names, the bounds and the manifest.
//!
//! Each Git child has null standard input and no controlling terminal, closes
//! every inherited descriptor above standard error at exec and inherits no
//! `GIT_*`, `RAVEL_*` or `HARNESS_DISPATCH_*` variable. The system and global
//! Git configuration stay, because they hold the owner's noninteractive
//! credential helpers and SSH settings.

use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::io::{self, ErrorKind, Read as _};
use std::os::unix::ffi::{OsStrExt as _, OsStringExt as _};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};

use super::cache::{Cache, unavailable};
use super::dir::{self, Dir};
use super::snapshot::{RESERVED, Staging, after_cleanup};
use super::watch::Watch;
use super::{Located, refuse};
use crate::refusal::Refusal;

/// The extracted tree inside the acquisition staging.
const TREE: &str = "tree";
/// The private bare repository inside the acquisition staging.
const REPOSITORY: &str = "repository";
/// An empty directory: the template of the repository and its hooks path.
const EMPTY: &str = "empty";
/// The namespace that the fallback fetch writes the advertised refs to.
const FETCHED: &str = "+refs/*:refs/fetched/*";
/// The bytes of standard error that a child keeps for its refusal.
const STDERR_KEPT: usize = 64 * 1024;
/// The bytes of standard error that a refusal shows, from its end.
const STDERR_SHOWN: usize = 1000;
/// Git reports a pin of the other object format with this text, under
/// `LC_ALL=C`.
const MISMATCHED: &str = "mismatched algorithms";

/// The pin of one Git source, as the settings hold it.
pub struct Pin<'a> {
    pub url: &'a str,
    pub revision: &'a str,
    pub subdir: &'a str,
}

impl Pin<'_> {
    /// How refusals name a path of the source: `<url>@<revision>`, then
    /// `subdir` unless it is the root.
    pub fn shown(&self) -> PathBuf {
        let base = PathBuf::from(format!("{}@{}", self.url, self.revision));
        if self.subdir == "." {
            base
        } else {
            base.join(self.subdir)
        }
    }

    fn format(&self) -> &'static str {
        if self.revision.len() == 64 {
            "sha256"
        } else {
            "sha1"
        }
    }
}

/// An extracted source: the staging that holds it, and the tree to capture.
#[derive(Debug)]
pub struct Acquired {
    pub staging: Staging,
    pub tree: PathBuf,
}

/// Fetch the exact pin into new staging in `cache` and extract `subdir`.
pub fn acquire(
    cache: &Cache,
    pin: &Pin<'_>,
    source: &Located,
    watch: &Watch,
) -> Result<Acquired, Refusal> {
    let mut staging = Staging::new(cache, "git-")?;
    match acquired(&staging, pin, source, watch) {
        Ok(tree) => Ok(Acquired { staging, tree }),
        Err(refusal) => Err(after_cleanup(refusal, &mut staging)),
    }
}

fn acquired(
    staging: &Staging,
    pin: &Pin<'_>,
    source: &Located,
    watch: &Watch,
) -> Result<PathBuf, Refusal> {
    let root = Dir::open(staging.path()).map_err(|error| unavailable(staging.path(), &error))?;
    for name in [EMPTY, TREE] {
        root.mkdir(OsStr::new(name), 0o700)
            .map_err(|error| unavailable(&staging.path().join(name), &error))?;
    }
    let git = Git::new(staging.path());
    let acquisition = Acquisition {
        git: &git,
        pin,
        source,
        watch,
        shown: pin.shown(),
    };
    acquisition.fetch()?;
    let tree = acquisition.subdir_tree()?;
    let to = root
        .open_dir(OsStr::new(TREE))
        .map_err(|error| unavailable(&staging.path().join(TREE), &error))?;
    acquisition.extract(&tree, "", &to)?;
    Ok(staging.path().join(TREE))
}

/// One acquisition: the runner, the pin and how refusals name it.
struct Acquisition<'a> {
    git: &'a Git,
    pin: &'a Pin<'a>,
    source: &'a Located,
    watch: &'a Watch,
    shown: PathBuf,
}

/// One entry of a non-recursive `ls-tree -z -l` listing.
struct Listed {
    mode: String,
    kind: String,
    object: String,
    size: Option<u64>,
    name: Vec<u8>,
}

impl Acquisition<'_> {
    /// Initialise the private repository in the object format of the pin and
    /// fetch the commit: first by its full ID, then through the advertised
    /// refs, which can reach a commit that the remote does not advertise.
    fn fetch(&self) -> Result<(), Refusal> {
        let repository = self.git.cwd.join(REPOSITORY);
        let template = format!("--template={}", self.git.cwd.join(EMPTY).display());
        let format = format!("--object-format={}", self.pin.format());
        let init = self.run(
            false,
            &[
                OsStr::new("init"),
                OsStr::new("--bare"),
                OsStr::new("--quiet"),
                OsStr::new(&template),
                OsStr::new(&format),
                repository.as_os_str(),
            ],
            &mut Vec::new(),
        )?;
        if !init.status.success() {
            return Err(self.unavailable(
                "Git cannot create its private repository for the acquisition",
                &init,
                "check the Git installation on the PATH of ravel and the storage of \
                 ~/.cache/ravel/skills/v1, then resolve again",
                None,
            ));
        }
        let fetch = |extra: &[&str], target: &str| {
            let mut args: Vec<&OsStr> = [
                "fetch",
                "--quiet",
                "--no-tags",
                "--no-recurse-submodules",
                "--no-write-fetch-head",
            ]
            .iter()
            .chain(extra)
            .map(OsStr::new)
            .collect();
            args.extend([
                OsStr::new("--end-of-options"),
                OsStr::new(self.pin.url),
                OsStr::new(target),
            ]);
            self.run(true, &args, &mut Vec::new())
        };
        let direct = fetch(&["--depth=1"], self.pin.revision)?;
        if direct.status.success() {
            return Ok(());
        }
        if direct.reported(MISMATCHED) {
            return Err(self.mismatched(&direct));
        }
        let advertised = fetch(&[], FETCHED)?;
        if advertised.status.success() {
            return Ok(());
        }
        if advertised.reported(MISMATCHED) {
            return Err(self.mismatched(&advertised));
        }
        Err(self.unavailable(
            &format!("Git cannot fetch from {}", self.pin.url),
            &advertised,
            &format!(
                "restore access to {} with noninteractive authentication, or change the source \
                 in the settings deliberately",
                self.pin.url
            ),
            None,
        ))
    }

    /// The tree of `subdir` in the exact commit, after the checks of the
    /// object type and of each segment of `subdir`.
    fn subdir_tree(&self) -> Result<String, Refusal> {
        let revision = self.pin.revision;
        let mut kind = Vec::new();
        let typed = self.run(
            true,
            &[
                OsStr::new("cat-file"),
                OsStr::new("-t"),
                OsStr::new(revision),
            ],
            &mut kind,
        )?;
        if !typed.status.success() {
            return Err(self.unavailable(
                &format!(
                    "the repository {} does not hold the commit {revision}",
                    self.pin.url
                ),
                &typed,
                &format!(
                    "make the commit {revision} available at {}, or change the pin in the \
                     settings deliberately",
                    self.pin.url
                ),
                Some("revision"),
            ));
        }
        let kind = String::from_utf8_lossy(&kind).trim().to_owned();
        if kind != "commit" {
            return Err(refuse(
                "skill_revision_unavailable",
                format!(
                    "the revision {revision} of the source {:?} at {} names a {kind} object, not \
                     a commit",
                    self.source.id, self.pin.url
                ),
                "pin the full ID of a commit object; Ravel never peels a tag object or uses \
                 another object type, so change the pin in the settings deliberately",
            )
            .source(self.pin.url.to_owned())
            .location(format!("{}.revision", self.source.location)));
        }
        let mut commit = Vec::new();
        let read = self.run(
            true,
            &[
                OsStr::new("cat-file"),
                OsStr::new("commit"),
                OsStr::new(revision),
            ],
            &mut commit,
        )?;
        let tree = commit
            .strip_prefix(b"tree ")
            .and_then(|rest| rest.split(|byte| *byte == b'\n').next())
            .and_then(|tree| std::str::from_utf8(tree).ok())
            .filter(|_| read.status.success())
            .map(str::to_owned)
            .ok_or_else(|| {
                self.unavailable(
                    &format!("Git cannot read the commit {revision}"),
                    &read,
                    &format!(
                        "make the commit {revision} available at {}, or change the pin in the \
                         settings deliberately",
                        self.pin.url
                    ),
                    Some("revision"),
                )
            })?;
        if self.pin.subdir == "." {
            return Ok(tree);
        }
        let mut current = tree;
        let mut walked = String::new();
        for segment in self.pin.subdir.split('/') {
            if !walked.is_empty() {
                walked.push('/');
            }
            walked.push_str(segment);
            let entry = self
                .list(&current)?
                .into_iter()
                .find(|entry| entry.name == segment.as_bytes());
            let problem = match entry {
                None => "does not exist".to_owned(),
                Some(entry) if entry.mode == "040000" => {
                    current = entry.object;
                    continue;
                }
                Some(entry) if entry.mode == "160000" => "is a Git submodule entry".to_owned(),
                Some(entry) if entry.mode == "120000" => "is a symbolic link".to_owned(),
                Some(_) => "is not a directory".to_owned(),
            };
            return Err(refuse(
                "skill_revision_unavailable",
                format!(
                    "the subdir {:?} of the source {:?} is not a directory in the commit \
                     {revision} of {}: {walked:?} {problem}",
                    self.pin.subdir, self.source.id, self.pin.url
                ),
                "change the subdir or the pin in the settings deliberately",
            )
            .source(self.pin.url.to_owned())
            .location(format!("{}.subdir", self.source.location)));
        }
        Ok(current)
    }

    /// The entries of one tree object, without recursion, so that a name can
    /// never pass as a path.
    fn list(&self, tree: &str) -> Result<Vec<Listed>, Refusal> {
        let mut out = Vec::new();
        let listed = self.run(
            true,
            &[
                OsStr::new("ls-tree"),
                OsStr::new("-z"),
                OsStr::new("-l"),
                OsStr::new("--end-of-options"),
                OsStr::new(tree),
            ],
            &mut out,
        )?;
        let malformed = || {
            self.unavailable(
                &format!("Git cannot list the tree {tree} of the commit"),
                &listed,
                &format!(
                    "make the commit {} available at {}, or change the pin in the settings \
                     deliberately",
                    self.pin.revision, self.pin.url
                ),
                Some("revision"),
            )
        };
        if !listed.status.success() {
            return Err(malformed());
        }
        let mut entries = Vec::new();
        for record in out
            .split(|byte| *byte == 0)
            .filter(|record| !record.is_empty())
        {
            let tab = record
                .iter()
                .position(|byte| *byte == b'\t')
                .ok_or_else(malformed)?;
            let meta = std::str::from_utf8(&record[..tab]).map_err(|_| malformed())?;
            let mut fields = meta.split_ascii_whitespace();
            let (Some(mode), Some(kind), Some(object), Some(size), None) = (
                fields.next(),
                fields.next(),
                fields.next(),
                fields.next(),
                fields.next(),
            ) else {
                return Err(malformed());
            };
            entries.push(Listed {
                mode: mode.to_owned(),
                kind: kind.to_owned(),
                object: object.to_owned(),
                size: size.parse().ok(),
                name: record[tab + 1..].to_vec(),
            });
        }
        Ok(entries)
    }

    /// Write the tracked contents of `tree` into `to`, then check that the
    /// directory holds exactly the listed names, spelled as Git spells them.
    fn extract(&self, tree: &str, relative: &str, to: &Dir) -> Result<(), Refusal> {
        let mut expected: BTreeSet<OsString> = BTreeSet::new();
        // The identity of what Ravel wrote for each entry, so that a
        // collision can name the entry that it met.
        let mut written: Vec<(String, (u64, u64))> = Vec::new();
        for entry in self.list(tree)? {
            self.watch.check()?;
            let name = self.name(&entry.name, relative)?;
            if RESERVED.contains(&name.as_str()) {
                continue;
            }
            let path = if relative.is_empty() {
                name.clone()
            } else {
                format!("{relative}/{name}")
            };
            let raw = OsString::from(&name);
            let made = match (entry.mode.as_str(), entry.kind.as_str()) {
                ("040000", "tree") => match to.mkdir(&raw, 0o700) {
                    Ok(()) => {
                        let inner = to
                            .open_dir(&raw)
                            .map_err(|error| self.staging_failed(&to.path().join(&raw), &error))?;
                        self.extract(&entry.object, &path, &inner)?;
                        Ok(Made::Done)
                    }
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(Made::Exists),
                    Err(error) => Err(self.staging_failed(&to.path().join(&raw), &error)),
                },
                ("100644" | "100755", "blob") => {
                    self.file(to, &raw, &path, &entry, entry.mode == "100755")
                }
                ("120000", "blob") => self.link(to, &raw, &path, &entry),
                ("160000", "commit") => {
                    return Err(self.invalid(&path, "is a Git submodule entry (a gitlink)"));
                }
                (mode, kind) => {
                    return Err(self.invalid(
                        &path,
                        &format!(
                            "has the Git mode {mode} and type {kind}, which a snapshot cannot keep"
                        ),
                    ));
                }
            }?;
            match made {
                Made::Done => {}
                Made::Exists => return Err(self.collision(to, &raw, &path, &written)),
            }
            let identity = to
                .stat_at(&raw)
                .map_err(|error| self.staging_failed(&to.path().join(&raw), &error))?
                .identity();
            written.push((path, identity));
            expected.insert(raw);
        }
        let actual: BTreeSet<OsString> = to
            .list()
            .map_err(|error| self.staging_failed(to.path(), &error))?
            .into_iter()
            .collect();
        if let Some(missing) = expected.difference(&actual).next() {
            let path = self.shown.join(relative).join(missing);
            let stored = actual
                .difference(&expected)
                .next()
                .map_or_else(String::new, |stored| {
                    format!("; the cache holds {stored:?} in its place")
                });
            return Err(refuse(
                "skill_resource_invalid",
                format!(
                    "the cache filesystem does not keep the exact spelling of the path {} in the \
                     source {:?}{stored}",
                    path.display(),
                    self.source.id
                ),
                "rename the entry in the repository so that its spelling survives on the \
                 filesystem of ~/.cache/ravel/skills/v1, then change the pin deliberately, or \
                 keep the cache on a filesystem that keeps names exactly; Ravel never normalises \
                 a name",
            )
            .source(path.display().to_string())
            .location(self.source.location.clone()));
        }
        Ok(())
    }

    /// The name of a tree entry as text, or the refusal of a name that no
    /// snapshot entry can hold.
    fn name(&self, raw: &[u8], relative: &str) -> Result<String, Refusal> {
        let shown = String::from_utf8_lossy(raw);
        let path = if relative.is_empty() {
            shown.clone().into_owned()
        } else {
            format!("{relative}/{shown}")
        };
        let Ok(name) = std::str::from_utf8(raw) else {
            return Err(self.invalid(&path, "has a name that is not exact UTF-8"));
        };
        if name.chars().any(char::is_control) {
            return Err(self.invalid(&path, "has a name that holds a control character"));
        }
        if matches!(name, "" | "." | "..") || name.contains('/') {
            return Err(self.invalid(
                &path,
                &format!("has the name {name:?}, which no directory entry can hold"),
            ));
        }
        Ok(name.to_owned())
    }

    /// Write the bytes of one blob as a new file with the executable bit of
    /// its tree mode. A Git LFS pointer is an ordinary blob here.
    fn file(
        &self,
        to: &Dir,
        raw: &OsStr,
        path: &str,
        entry: &Listed,
        executable: bool,
    ) -> Result<Made, Refusal> {
        let mut output = match to.create(raw, if executable { 0o700 } else { 0o600 }) {
            Ok(output) => output,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => return Ok(Made::Exists),
            Err(error) => return Err(self.staging_failed(&to.path().join(raw), &error)),
        };
        let mut counted = Counted {
            inner: &mut output,
            bytes: 0,
        };
        let read = self.blob(&entry.object, &mut counted, &to.path().join(raw))?;
        if !read.status.success() || entry.size != Some(counted.bytes) {
            return Err(self.unreadable(path, &read));
        }
        dir::flush_file(&output)
            .map_err(|error| self.staging_failed(&to.path().join(raw), &error))?;
        Ok(Made::Done)
    }

    /// Write one symbolic link. Capture applies the chain rules of a local
    /// source to it.
    fn link(&self, to: &Dir, raw: &OsStr, path: &str, entry: &Listed) -> Result<Made, Refusal> {
        let mut target = Vec::new();
        let read = self.blob(&entry.object, &mut target, &to.path().join(raw))?;
        if !read.status.success() || entry.size != Some(target.len() as u64) {
            return Err(self.unreadable(path, &read));
        }
        if target.is_empty() || target.contains(&0) {
            return Err(self.invalid(path, "is a link whose target is empty or holds NUL"));
        }
        match to.symlink(OsStr::from_bytes(&target), raw) {
            Ok(()) => Ok(Made::Done),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(Made::Exists),
            Err(error) if error.raw_os_error() == Some(libc::ENAMETOOLONG) => Err(self.invalid(
                path,
                "is a link whose target is longer than the cache filesystem permits",
            )),
            Err(error) => Err(self.staging_failed(&to.path().join(raw), &error)),
        }
    }

    /// Stream the raw bytes of a blob, with no filter and no conversion.
    fn blob(&self, object: &str, out: &mut dyn io::Write, at: &Path) -> Result<Ran, Refusal> {
        self.git
            .run(
                true,
                &[
                    OsStr::new("cat-file"),
                    OsStr::new("blob"),
                    OsStr::new(object),
                ],
                out,
                self.watch,
            )
            .map_err(|failure| self.failed(failure, Some(at)))
    }

    fn run(
        &self,
        repository: bool,
        args: &[&OsStr],
        out: &mut dyn io::Write,
    ) -> Result<Ran, Refusal> {
        self.git
            .run(repository, args, out, self.watch)
            .map_err(|failure| self.failed(failure, None))
    }

    /// The refusal of a child that Ravel could not run to its end.
    fn failed(&self, failure: Failure, at: Option<&Path>) -> Refusal {
        match failure {
            Failure::Stopped(refusal) => refusal,
            Failure::Missing(error) => refuse(
                "skill_revision_unavailable",
                format!(
                    "the source {:?} needs Git to fetch the commit {} from {}, and no git \
                     executable is on the PATH of ravel: {error}",
                    self.source.id, self.pin.revision, self.pin.url
                ),
                "put Git on the PATH of ravel, or change the source in the settings deliberately",
            )
            .source(self.pin.url.to_owned())
            .location(self.source.location.clone()),
            Failure::Spawn(error) => refuse(
                "skill_revision_unavailable",
                format!(
                    "Ravel cannot start Git for the source {:?} at {}: {error}",
                    self.source.id, self.pin.url
                ),
                "check the Git installation on the PATH of ravel, then resolve again",
            )
            .source(self.pin.url.to_owned())
            .location(self.source.location.clone()),
            Failure::Write(error) => {
                let path = at.unwrap_or(&self.git.cwd);
                self.staging_failed(path, &error)
            }
            Failure::Read(error) => refuse(
                "skill_revision_unavailable",
                format!(
                    "Ravel cannot read the output of Git for the source {:?} at {}: {error}",
                    self.source.id, self.pin.url
                ),
                "check the Git installation on the PATH of ravel, then resolve again",
            )
            .source(self.pin.url.to_owned())
            .location(self.source.location.clone()),
        }
    }

    /// A `skill_revision_unavailable` refusal with what Git reported.
    fn unavailable(&self, what: &str, ran: &Ran, remedy: &str, field: Option<&str>) -> Refusal {
        let location = match field {
            Some(field) => format!("{}.{field}", self.source.location),
            None => self.source.location.clone(),
        };
        refuse(
            "skill_revision_unavailable",
            format!(
                "the source {:?} cannot supply the commit {}: {what}{}",
                self.source.id,
                self.pin.revision,
                ran.excerpt()
            ),
            remedy,
        )
        .source(self.pin.url.to_owned())
        .location(location)
    }

    fn mismatched(&self, ran: &Ran) -> Refusal {
        let digits = self.pin.revision.len();
        self.unavailable(
            &format!(
                "the pin has {digits} hexadecimal digits, the {} object format, and the \
                 repository {} uses the other format",
                self.pin.format(),
                self.pin.url
            ),
            ran,
            "change the pin in the settings deliberately to the full ID of a commit in the \
             object format of the repository",
            Some("revision"),
        )
    }

    fn unreadable(&self, path: &str, ran: &Ran) -> Refusal {
        self.unavailable(
            &format!(
                "Git cannot read the object of the path {}",
                self.shown.join(path).display()
            ),
            ran,
            &format!(
                "make the commit {} available at {}, or change the pin in the settings \
                 deliberately",
                self.pin.revision, self.pin.url
            ),
            Some("revision"),
        )
    }

    fn invalid(&self, path: &str, what: &str) -> Refusal {
        let path = self.shown.join(path);
        refuse(
            "skill_resource_invalid",
            format!(
                "the path {} in the source {:?} {what}",
                path.display(),
                self.source.id
            ),
            "repair or remove the named entry in the repository, then change the pin in the \
             settings deliberately; a snapshot keeps only regular files, directories and \
             relative links that stay inside the source",
        )
        .source(path.display().to_string())
        .location(self.source.location.clone())
    }

    /// Two tree entries met one name on the cache filesystem. Name both.
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
        let this = self.shown.join(path);
        let message = match &met {
            Some(other) => format!(
                "the paths {} and {} in the source {:?} are distinct, but the cache filesystem \
                 cannot keep both",
                self.shown.join(other).display(),
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
            "rename one of the entries in the repository so that their names differ in more \
             than case or Unicode normalisation, then change the pin deliberately, or keep the \
             cache on a filesystem that keeps both; Ravel never writes one entry over another",
        )
        .source(this.display().to_string())
        .location(self.source.location.clone())
    }

    fn staging_failed(&self, path: &Path, error: &io::Error) -> Refusal {
        unavailable(path, error).location(self.source.location.clone())
    }
}

/// What a write of one entry met.
enum Made {
    Done,
    Exists,
}

/// A writer that counts the bytes that pass through it.
struct Counted<'a> {
    inner: &'a mut std::fs::File,
    bytes: u64,
}

impl io::Write for Counted<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buffer)?;
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// The end of one Git child.
pub struct Ran {
    pub status: ExitStatus,
    /// The first [`STDERR_KEPT`] bytes of standard error.
    pub stderr: Vec<u8>,
}

impl Ran {
    fn reported(&self, text: &str) -> bool {
        String::from_utf8_lossy(&self.stderr).contains(text)
    }

    /// The end of standard error as one line for a refusal, or nothing.
    /// k72 replaces this with the bounded and redacted diagnostics of the
    /// specification.
    fn excerpt(&self) -> String {
        let text = String::from_utf8_lossy(&self.stderr);
        let text = text.trim();
        if text.is_empty() {
            return String::new();
        }
        let start = text
            .char_indices()
            .map(|(index, _)| index)
            .find(|index| text.len() - index <= STDERR_SHOWN)
            .unwrap_or(0);
        let shown: String = text[start..]
            .chars()
            .map(|char| if char.is_control() { ' ' } else { char })
            .collect();
        format!("; Git reported: {shown}")
    }
}

/// Why a child did not run to its end.
pub enum Failure {
    /// The watch refused between operations.
    Stopped(Refusal),
    /// No `git` executable is on the `PATH`.
    Missing(io::Error),
    Spawn(io::Error),
    /// Standard output could not be written to its destination.
    Write(io::Error),
    Read(io::Error),
}

/// The runner of the Git children of one acquisition.
pub struct Git {
    /// The private acquisition staging: the working directory of each child.
    cwd: PathBuf,
    environment: Vec<(OsString, OsString)>,
    config: Vec<String>,
}

impl Git {
    pub fn new(cwd: &Path) -> Git {
        let empty = cwd.join(EMPTY);
        Git {
            cwd: cwd.to_owned(),
            environment: environment(std::env::vars_os()),
            config: configuration(&empty),
        }
    }

    /// Run `git` with `args`, after `--git-dir` of the private repository
    /// when `repository` is set, and copy its standard output to `out`.
    pub fn run(
        &self,
        repository: bool,
        args: &[&OsStr],
        out: &mut dyn io::Write,
        watch: &Watch,
    ) -> Result<Ran, Failure> {
        watch.check().map_err(Failure::Stopped)?;
        let mut command = Command::new("git");
        command
            .current_dir(&self.cwd)
            .env_clear()
            .envs(self.environment.iter().map(|(name, value)| (name, value)))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if repository {
            let mut git_dir = OsString::from("--git-dir=");
            git_dir.push(self.cwd.join(REPOSITORY));
            command.arg(git_dir);
        }
        for setting in &self.config {
            command.arg("-c").arg(setting);
        }
        command.args(args);
        let inherited = crate::worker::held_descriptors().map_err(|error| {
            Failure::Spawn(io::Error::other(format!(
                "cannot list the open descriptors of ravel in {}, so a Git child could inherit \
                 one: {error}",
                crate::worker::DESCRIPTORS
            )))
        })?;
        // SAFETY: the closure runs between fork and exec. It calls only the
        // async-signal-safe `setsid` and `fcntl` on descriptors that the
        // parent listed before the fork, and it does not allocate.
        unsafe {
            command.pre_exec(move || {
                // A new session has no controlling terminal, so no prompt of
                // Git or SSH can reach the terminal of the caller.
                if libc::setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }
                for &fd in inherited.iter().filter(|&&fd| fd > 2) {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1
                        && io::Error::last_os_error().raw_os_error() != Some(libc::EBADF)
                    {
                        return Err(io::Error::last_os_error());
                    }
                }
                Ok(())
            });
        }
        let mut child = match command.spawn() {
            Ok(child) => Reaped(child),
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Err(Failure::Missing(error));
            }
            Err(error) => return Err(Failure::Spawn(error)),
        };
        let mut stderr = child.0.stderr.take().expect("standard error is piped");
        let drain = std::thread::spawn(move || {
            let mut kept = Vec::new();
            let mut buffer = [0u8; 8192];
            loop {
                match stderr.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(read) => {
                        let room = STDERR_KEPT.saturating_sub(kept.len());
                        kept.extend_from_slice(&buffer[..read.min(room)]);
                    }
                }
            }
            kept
        });
        let mut stdout = child.0.stdout.take().expect("standard output is piped");
        let mut buffer = vec![0u8; 64 * 1024];
        let copied = loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break Ok(()),
                Ok(read) => {
                    if let Err(error) = out.write_all(&buffer[..read]) {
                        break Err(Failure::Write(error));
                    }
                }
                Err(error) if error.kind() == ErrorKind::Interrupted => {}
                Err(error) => break Err(Failure::Read(error)),
            }
        };
        if copied.is_err() {
            let _ = child.0.kill();
        }
        drop(stdout);
        let status = child.0.wait().map_err(Failure::Read);
        let stderr = drain.join().unwrap_or_default();
        copied?;
        Ok(Ran {
            status: status?,
            stderr,
        })
    }
}

/// A child that is killed and reaped if Ravel leaves it early.
struct Reaped(Child);

impl Drop for Reaped {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

/// The environment of a Git child: the caller's, without any Git
/// redirection or completion authority, and with every prompt disabled.
fn environment(inherited: impl Iterator<Item = (OsString, OsString)>) -> Vec<(OsString, OsString)> {
    const SET: [(&str, &str); 5] = [
        // https://git-scm.com/docs/git#Documentation/git.txt-GITTERMINALPROMPT
        ("GIT_TERMINAL_PROMPT", "0"),
        // A set but empty GIT_ASKPASS stops core.askPass and SSH_ASKPASS too:
        // https://github.com/git/git/blob/master/prompt.c (git_prompt).
        ("GIT_ASKPASS", ""),
        // OpenSSH 8.4 and later: never run an askpass program.
        ("SSH_ASKPASS_REQUIRE", "never"),
        ("GIT_NO_REPLACE_OBJECTS", "1"),
        // Stable messages, such as "mismatched algorithms".
        ("LC_ALL", "C"),
    ];
    let mut environment: Vec<(OsString, OsString)> = inherited
        .filter(|(name, _)| {
            let name = name.as_bytes();
            !name.starts_with(b"GIT_")
                && !name.starts_with(b"RAVEL_")
                && !name.starts_with(b"HARNESS_DISPATCH_")
                && !SET.iter().any(|(set, _)| name == set.as_bytes())
        })
        .collect();
    environment.extend(
        SET.iter()
            .map(|(name, value)| (OsString::from(name), OsString::from(value))),
    );
    environment
}

/// The command-line configuration of every Git child. It outranks the
/// system, global and repository configuration
/// (https://git-scm.com/docs/git-config#FILES).
fn configuration(empty: &Path) -> Vec<String> {
    let mut config = vec![format!("core.hooksPath={}", empty.display())];
    config.extend(
        [
            "core.fsmonitor=false",
            "core.askPass=",
            "credential.interactive=false",
            "protocol.allow=never",
            "protocol.https.allow=always",
            "protocol.ssh.allow=always",
            "protocol.file.allow=always",
            "fetch.recurseSubmodules=false",
            "submodule.recurse=false",
            "maintenance.auto=false",
            "gc.auto=0",
            "fetch.writeCommitGraph=false",
            "transfer.bundleURI=false",
        ]
        .map(str::to_owned),
    );
    config
}

/// The local repository path of a locator, for the protected-overlap rules:
/// an absolute path, or the percent-decoded path of a `file://` URL.
pub fn local_path(url: &str) -> Option<PathBuf> {
    if url.starts_with('/') {
        return Some(PathBuf::from(url));
    }
    let rest = url.strip_prefix("file://")?;
    let path = rest.get(rest.find('/')?..)?;
    let bytes = path.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let hex = |at: usize| bytes.get(at).and_then(|byte| (*byte as char).to_digit(16));
        match (bytes[index], hex(index + 1), hex(index + 2)) {
            (b'%', Some(high), Some(low)) => {
                decoded.push((high * 16 + low) as u8);
                index += 3;
            }
            (byte, _, _) => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    Some(PathBuf::from(OsString::from_vec(decoded)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_child_inherits_no_git_redirection_or_completion_authority() {
        let inherited = [
            ("PATH", "/usr/bin"),
            ("HOME", "/h"),
            ("SSH_AUTH_SOCK", "/s"),
            ("GIT_DIR", "/elsewhere"),
            ("GIT_CONFIG_PARAMETERS", "'core.hooksPath'='/x'"),
            ("GIT_SSH_COMMAND", "ssh -v"),
            ("RAVEL_EXIT_FILE", "/c"),
            ("RAVEL_SKILL_DIRECTORIES", "/d"),
            ("HARNESS_DISPATCH_EXIT_FILE", "/e"),
            ("LC_ALL", "fr_FR.UTF-8"),
            ("GIT_ASKPASS", "/usr/bin/prompt"),
        ]
        .map(|(name, value)| (OsString::from(name), OsString::from(value)));
        let environment = environment(inherited.into_iter());
        let names: Vec<&str> = environment
            .iter()
            .map(|(name, _)| name.to_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "PATH",
                "HOME",
                "SSH_AUTH_SOCK",
                "GIT_TERMINAL_PROMPT",
                "GIT_ASKPASS",
                "SSH_ASKPASS_REQUIRE",
                "GIT_NO_REPLACE_OBJECTS",
                "LC_ALL"
            ]
        );
        let value = |name: &str| {
            environment
                .iter()
                .find(|(set, _)| set == name)
                .map(|(_, value)| value.to_str().unwrap().to_owned())
        };
        assert_eq!(value("GIT_ASKPASS").as_deref(), Some(""));
        assert_eq!(value("LC_ALL").as_deref(), Some("C"));
    }

    #[test]
    fn a_local_locator_names_its_repository_path() {
        assert_eq!(local_path("/srv/r.git"), Some(PathBuf::from("/srv/r.git")));
        assert_eq!(
            local_path("file:///srv/a%20b.git"),
            Some(PathBuf::from("/srv/a b.git"))
        );
        assert_eq!(
            local_path("file://host/srv/r.git"),
            Some(PathBuf::from("/srv/r.git"))
        );
        assert_eq!(local_path("https://example.com/r.git"), None);
        assert_eq!(local_path("git@example.com:r.git"), None);
    }

    #[test]
    fn the_shown_root_names_the_pin_and_the_subdir() {
        let pin = |subdir| Pin {
            url: "https://h/r.git",
            revision: "0123456789abcdef0123456789abcdef01234567",
            subdir,
        };
        assert_eq!(
            pin(".").shown(),
            PathBuf::from("https://h/r.git@0123456789abcdef0123456789abcdef01234567")
        );
        assert_eq!(
            pin("a/b").shown(),
            PathBuf::from("https://h/r.git@0123456789abcdef0123456789abcdef01234567/a/b")
        );
    }
}
