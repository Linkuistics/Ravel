//! The default skill directory: the installation record that names it, and
//! the listing of its entries at each resolution. The skill discovery
//! specification owns each rule here
//! (docs/specs/skill-discovery.md#default-skill-directory).
//!
//! Ravel knows no package manager. It reads the one path that the record of
//! its installation names, and it treats each entry there as a source.

use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::refuse;
use super::settings::source_id;
use super::watch::Watch;
use crate::refusal::Refusal;
use crate::strict_json;

/// The installation record, relative to the prefix of the installation.
pub const RECORD: &str = "share/ravel/installation.json";

/// The roots of a default source, in the order that Ravel shows them, each
/// where it exists as a directory.
pub const ROOTS: [&str; 2] = ["skills", "craft"];

/// The code of every refusal in this module.
const INVALID: &str = "skill_directory_invalid";

/// The remedy of a refusal about the record.
const RECORD_REMEDY: &str = "reinstall Ravel, so that its package writes the installation record \
     again, or name the skill sources in the settings, which makes Ravel read no record";

/// The remedy of a refusal about an entry.
const ENTRY_REMEDY: &str = "remove the entry, or reinstall the package that installed it; \
     `brew cleanup --prune-prefix` removes a dangling link of Homebrew";

/// One source of the default skill directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The entry name, which is the source ID.
    pub id: String,
    /// The path of the entry, which is the source path. A link stays a link
    /// here, and capture canonicalises it.
    pub path: String,
    /// The roots that exist, in the order of [`ROOTS`].
    pub roots: Vec<String>,
}

/// The default skill directory that the installation record names, or
/// `None` when the installation holds no record.
pub fn directory() -> Result<Option<String>, Refusal> {
    read_record(&prefix()?.join(RECORD))
}

/// The prefix of the installation: the parent of the directory that holds
/// the canonical path of the running executable.
fn prefix() -> Result<PathBuf, Refusal> {
    #[cfg(test)]
    if let Some(prefix) = super::faults::prefix() {
        return Ok(prefix);
    }
    let executable = std::env::current_exe()
        .and_then(fs::canonicalize)
        .map_err(|error| {
            refuse(
                INVALID,
                format!(
                    "Ravel cannot find the canonical path of its own executable, so it cannot \
                     find its installation record: {error}"
                ),
                "run Ravel from an executable whose path exists, or name the skill sources in \
                 the settings, which makes Ravel read no record",
            )
        })?;
    executable
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            refuse(
                INVALID,
                format!(
                    "the executable {} has no installation prefix two levels above it",
                    executable.display()
                ),
                RECORD_REMEDY,
            )
            .source(executable.display().to_string())
        })
}

/// Read the record at `path`. Only a path that does not exist is an absent
/// record. A dangling link there is an unreadable record.
pub fn read_record(path: &Path) -> Result<Option<String>, Refusal> {
    let invalid = |what: String| {
        refuse(
            INVALID,
            format!("the installation record {} {what}", path.display()),
            RECORD_REMEDY,
        )
        .source(path.display().to_string())
    };
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(invalid(format!("cannot be read: {error}"))),
        Ok(_) => {}
    }
    // A regular file only, as for the settings file: a FIFO there would hold
    // the resolution before any deadline applies.
    match fs::metadata(path) {
        Err(error) => return Err(invalid(format!("cannot be read: {error}"))),
        Ok(metadata) if !metadata.is_file() => {
            return Err(invalid("is not a regular file".to_owned()));
        }
        Ok(_) => {}
    }
    let bytes = fs::read(path).map_err(|error| invalid(format!("cannot be read: {error}")))?;
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid("is not UTF-8".to_owned()))?;
    let document =
        strict_json::parse(text.as_bytes()).map_err(|error| invalid(error.to_string()))?;
    skill_directory(&document).map(Some).map_err(invalid)
}

/// The `skillDirectory` of a record with exactly `schemaVersion` 1 and
/// `skillDirectory`, or what is wrong with it.
fn skill_directory(document: &Value) -> Result<String, String> {
    let Value::Object(members) = document else {
        return Err(format!("is {document}, not a JSON object"));
    };
    if let Some(key) = members
        .keys()
        .find(|key| !matches!(key.as_str(), "schemaVersion" | "skillDirectory"))
    {
        return Err(format!(
            "holds the unknown member {key:?}; a record holds exactly schemaVersion and \
             skillDirectory"
        ));
    }
    match members.get("schemaVersion") {
        None => return Err("has no schemaVersion".to_owned()),
        Some(version) if version.as_u64() != Some(1) => {
            return Err(format!(
                "has the schemaVersion {version}, not the integer 1"
            ));
        }
        Some(_) => {}
    }
    let directory = match members.get("skillDirectory") {
        None => return Err("has no skillDirectory".to_owned()),
        Some(Value::String(directory)) => directory,
        Some(value) => return Err(format!("has the skillDirectory {value}, not a string")),
    };
    if !absolute(directory) {
        return Err(format!(
            "has the skillDirectory {directory:?}, not an absolute path without an empty \
             segment, \".\", \"..\", a trailing separator, a tilde or a control character"
        ));
    }
    Ok(directory.clone())
}

/// An absolute path with no empty segment, `.`, `..`, trailing separator,
/// tilde or control character.
fn absolute(path: &str) -> bool {
    path.strip_prefix('/').is_some_and(|rest| {
        !path.contains('~')
            && !path.chars().any(char::is_control)
            && rest
                .split('/')
                .all(|segment| !matches!(segment, "" | "." | ".."))
    })
}

/// List the default skill directory: each source in byte order of its ID.
///
/// An absent directory supplies no source. Each entry that is not a source
/// refuses, and so does a directory that exists and cannot be listed.
pub fn list(directory: &str, watch: &Watch) -> Result<Vec<Entry>, Refusal> {
    let path = Path::new(directory);
    let unlisted = |what: String| {
        refuse(
            INVALID,
            format!("the default skill directory {directory} {what}"),
            "make the default skill directory a readable directory, or remove it, or reinstall \
             the package that made it",
        )
        .source(directory.to_owned())
        .location(directory.to_owned())
    };
    let listing = match fs::read_dir(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) if error.kind() == ErrorKind::NotADirectory => {
            return Err(unlisted("is not a directory".to_owned()));
        }
        Err(error) => return Err(unlisted(format!("cannot be listed: {error}"))),
        Ok(listing) => listing,
    };
    let mut names: Vec<OsString> = Vec::new();
    for entry in listing {
        watch.check()?;
        let entry = entry.map_err(|error| unlisted(format!("cannot be listed: {error}")))?;
        let name = entry.file_name();
        // Operating systems and file managers write such entries.
        if !name.as_bytes().starts_with(b".") {
            names.push(name);
        }
    }
    // The first refusing entry is the same on every run.
    names.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    let mut sources = Vec::with_capacity(names.len());
    for name in names {
        watch.check()?;
        sources.push(entry(path, &name)?);
    }
    Ok(sources)
}

/// The source that the entry `name` of `directory` holds, or its refusal.
fn entry(directory: &Path, name: &OsString) -> Result<Entry, Refusal> {
    let path = directory.join(name);
    let shown = path.display().to_string();
    let invalid = |what: &str| {
        refuse(
            INVALID,
            format!("the entry {shown} of the default skill directory {what}"),
            ENTRY_REMEDY,
        )
        .source(shown.clone())
        .location(shown.clone())
    };
    let id = name.to_str().filter(|id| source_id(id)).ok_or_else(|| {
        invalid(&format!(
            "has a name outside the source-ID grammar: {}",
            super::settings::ID_GRAMMAR
        ))
    })?;
    let link = fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.is_symlink());
    match fs::metadata(&path) {
        Err(error) if link && error.kind() == ErrorKind::NotFound => {
            return Err(invalid("is a dangling link"));
        }
        Err(error) if link => {
            return Err(invalid(&format!(
                "is a link that Ravel cannot follow: {error}"
            )));
        }
        Err(error) => return Err(invalid(&format!("cannot be examined: {error}"))),
        Ok(metadata) if !metadata.is_dir() => return Err(invalid("is not a directory")),
        Ok(_) => {}
    }
    let mut roots = Vec::with_capacity(ROOTS.len());
    for root in ROOTS {
        match fs::metadata(path.join(root)) {
            Ok(metadata) if metadata.is_dir() => roots.push(root.to_owned()),
            Ok(_) => {}
            Err(error)
                if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {}
            Err(error) => {
                return Err(invalid(&format!(
                    "has a root {root:?} that cannot be examined: {error}"
                )));
            }
        }
    }
    if roots.is_empty() {
        return Err(invalid(&format!(
            "holds neither the root {:?} nor the root {:?} as a directory",
            ROOTS[0], ROOTS[1]
        )));
    }
    let path = path
        .to_str()
        .ok_or_else(|| invalid("has a path that is not exact UTF-8"))?
        .to_owned();
    Ok(Entry {
        id: id.to_owned(),
        path,
        roots,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::os::unix::fs::symlink;

    fn record(dir: &Path, bytes: &[u8]) -> PathBuf {
        let path = dir.join("installation.json");
        fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn a_valid_record_names_its_directory_and_an_absent_one_names_none() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(read_record(&dir.path().join("absent.json")).unwrap(), None);
        let path = record(
            dir.path(),
            json!({"schemaVersion": 1, "skillDirectory": "/opt/homebrew/share/ravel/skills"})
                .to_string()
                .as_bytes(),
        );
        assert_eq!(
            read_record(&path).unwrap().as_deref(),
            Some("/opt/homebrew/share/ravel/skills")
        );
    }

    #[test]
    fn each_malformed_record_refuses_as_skill_directory_invalid() {
        let dir = tempfile::tempdir().unwrap();
        let cases: Vec<(&str, Vec<u8>)> = vec![
            ("not JSON", b"{".to_vec()),
            (
                "not UTF-8",
                b"{\"schemaVersion\": 1, \"x\": \"\xff\"}".to_vec(),
            ),
            ("an array", b"[]".to_vec()),
            (
                "a repeated member",
                br#"{"schemaVersion": 1, "schemaVersion": 1, "skillDirectory": "/a"}"#.to_vec(),
            ),
            (
                "an unknown member",
                br#"{"schemaVersion": 1, "skillDirectory": "/a", "x": 1}"#.to_vec(),
            ),
            ("no version", br#"{"skillDirectory": "/a"}"#.to_vec()),
            (
                "version 2",
                br#"{"schemaVersion": 2, "skillDirectory": "/a"}"#.to_vec(),
            ),
            (
                "version 1.0",
                br#"{"schemaVersion": 1.0, "skillDirectory": "/a"}"#.to_vec(),
            ),
            (
                "version text",
                br#"{"schemaVersion": "1", "skillDirectory": "/a"}"#.to_vec(),
            ),
            ("no directory", br#"{"schemaVersion": 1}"#.to_vec()),
            (
                "a number",
                br#"{"schemaVersion": 1, "skillDirectory": 1}"#.to_vec(),
            ),
            (
                "relative",
                br#"{"schemaVersion": 1, "skillDirectory": "a/b"}"#.to_vec(),
            ),
            (
                "empty",
                br#"{"schemaVersion": 1, "skillDirectory": ""}"#.to_vec(),
            ),
            (
                "the root",
                br#"{"schemaVersion": 1, "skillDirectory": "/"}"#.to_vec(),
            ),
            (
                "an empty segment",
                br#"{"schemaVersion": 1, "skillDirectory": "/a//b"}"#.to_vec(),
            ),
            (
                "a dot",
                br#"{"schemaVersion": 1, "skillDirectory": "/a/./b"}"#.to_vec(),
            ),
            (
                "a parent",
                br#"{"schemaVersion": 1, "skillDirectory": "/a/../b"}"#.to_vec(),
            ),
            (
                "a trailing separator",
                br#"{"schemaVersion": 1, "skillDirectory": "/a/b/"}"#.to_vec(),
            ),
            (
                "a tilde",
                br#"{"schemaVersion": 1, "skillDirectory": "/~/b"}"#.to_vec(),
            ),
            (
                "a control character",
                br#"{"schemaVersion": 1, "skillDirectory": "/a\nb"}"#.to_vec(),
            ),
        ];
        for (case, bytes) in cases {
            let path = record(dir.path(), &bytes);
            let refusal = read_record(&path).unwrap_err();
            assert_eq!(refusal.code, INVALID, "{case}");
            assert_eq!(refusal.exit, crate::refusal::EXIT_REFUSED, "{case}");
            assert_eq!(refusal.stage, crate::refusal::Stage::Skills, "{case}");
            assert_eq!(refusal.source.as_deref(), path.to_str(), "{case}");
        }
        // Not a regular file, and a dangling link, which is not an absent record.
        let directory = dir.path().join("directory.json");
        fs::create_dir(&directory).unwrap();
        assert_eq!(read_record(&directory).unwrap_err().code, INVALID);
        let dangling = dir.path().join("dangling.json");
        symlink(dir.path().join("nothing"), &dangling).unwrap();
        assert_eq!(read_record(&dangling).unwrap_err().code, INVALID);
        // A link to a valid record is read.
        let valid = record(
            dir.path(),
            br#"{"skillDirectory": "/a/b", "schemaVersion": 1}"#,
        );
        let linked = dir.path().join("linked.json");
        symlink(&valid, &linked).unwrap();
        assert_eq!(read_record(&linked).unwrap().as_deref(), Some("/a/b"));
    }

    #[test]
    fn an_absent_directory_supplies_no_source_and_a_listing_keeps_byte_order() {
        let dir = tempfile::tempdir().unwrap();
        let watch = Watch::unbounded();
        let absent = dir.path().join("absent");
        assert_eq!(list(absent.to_str().unwrap(), &watch).unwrap(), []);
        let skills = dir.path().join("skills");
        for (name, roots) in [
            ("zeta", &["skills"][..]),
            ("alpha-2", &["craft"][..]),
            ("alpha", &["craft", "skills"][..]),
        ] {
            for root in roots {
                fs::create_dir_all(skills.join(name).join(root)).unwrap();
            }
        }
        fs::write(skills.join(".DS_Store"), "x").unwrap();
        fs::create_dir(skills.join(".hidden")).unwrap();
        let listed = list(skills.to_str().unwrap(), &watch).unwrap();
        let ids: Vec<&str> = listed.iter().map(|entry| entry.id.as_str()).collect();
        assert_eq!(ids, ["alpha", "alpha-2", "zeta"]);
        assert_eq!(listed[0].roots, ["skills", "craft"]);
        assert_eq!(listed[1].roots, ["craft"]);
        assert_eq!(listed[2].roots, ["skills"]);
        assert_eq!(
            listed[0].path,
            skills.join("alpha").to_str().unwrap().to_owned()
        );
    }
}
