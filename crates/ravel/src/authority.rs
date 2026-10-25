use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::refusal::{EXIT_REFUSED, Refusal, Stage};

// While both policy paths live, the extension of the entry selects the path.
// The removal increment deletes every item marked `typescript` here, and keeps
// every item marked `rhai`.

/// The personal default of the Rhai path, relative to HOME. `XDG_CONFIG_HOME`
/// is deliberately not consulted: it would be an environment-selected entry.
/// It wins over [`PERSONAL_DEFAULT`] when both files exist. (rhai)
pub const PERSONAL_RHAI_DEFAULT: &str = ".config/ravel/policy.rhai";

/// The personal default of the TypeScript path, relative to HOME. (typescript)
pub const PERSONAL_DEFAULT: &str = ".config/ravel/policy.ts";

/// The extension that takes the Rhai path. (rhai)
const RHAI_EXTENSION: &str = "rhai";

/// The most an entry of the Rhai path may hold: 1 MiB. (rhai)
pub const RHAI_ENTRY_BYTES: u64 = 1024 * 1024;

#[derive(Debug)]
pub enum Authority {
    Personal,
    /// The `--config` argument exactly as the caller gave it.
    Explicit,
}

/// Which worker evaluates the entry, and what that worker needs of it.
#[derive(Debug)]
pub enum Format {
    /// The Bun worker imports the entry itself, by its canonical path. (typescript)
    TypeScript,
    /// The Rhai worker reads no file, so the front reads the entry once, on its
    /// reader thread after the worker has started, and sends exactly those
    /// bytes. The entry is proven to be a regular file here, and a failure of
    /// the read itself refuses as this describes. (rhai)
    Rhai(RhaiEntry),
}

/// What a refusal about the read of a Rhai entry says. (rhai)
#[derive(Debug)]
pub struct RhaiEntry {
    /// The path as it was selected, which the refusal names.
    shown: String,
    remedy: String,
}

impl RhaiEntry {
    /// The refusal for an entry that cannot be read, where `what` completes the
    /// sentence "the selected policy entry PATH ...".
    pub fn unreadable(&self, what: &str) -> Refusal {
        Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {} {what}", self.shown),
            self.remedy.clone(),
        )
        .source(self.shown.clone())
    }
}

#[derive(Debug)]
pub struct PolicyEntry {
    /// The canonical path; symlinks are resolved, so this is the file whose
    /// directory relative imports start from. On the TypeScript path it is
    /// exactly the string the worker imports, and [`import_specifier`] has
    /// proven that it names this file.
    pub path: String,
    pub format: Format,
}

impl PolicyEntry {
    pub fn display(&self) -> String {
        self.path.clone()
    }
}

/// Resolve the selected entry and prove it is a readable regular file.
///
/// A `.rhai` file given to `--config` takes the Rhai path, and so does the
/// personal `policy.rhai` when it exists, whether or not `policy.ts` does.
/// Every other entry keeps the TypeScript path.
pub fn resolve(
    config: Option<&Path>,
    cwd: &Path,
    home: Option<&OsStr>,
) -> Result<PolicyEntry, Refusal> {
    let (candidate, authority) = match config {
        Some(config) => (cwd.join(config), Authority::Explicit),
        None => {
            let rhai = personal_rhai_default(home)?;
            let typescript = personal_default(home)?;
            // `exists` is true for a dangling symlink, so that one reports
            // itself as missing instead of falling through to another file.
            if rhai.symlink_metadata().is_ok() || typescript.symlink_metadata().is_err() {
                return resolve_rhai(&rhai, &Authority::Personal, cwd);
            }
            (typescript, Authority::Personal)
        }
    };
    if candidate.extension() == Some(OsStr::new(RHAI_EXTENSION)) {
        return resolve_rhai(&candidate, &authority, cwd);
    }
    let shown = candidate.to_string_lossy().into_owned();
    let remedy = match &authority {
        Authority::Personal => {
            format!("write {shown} exporting a `policy`, or name another entry with --config PATH")
        }
        Authority::Explicit => format!(
            "check the --config path; a relative path resolves against the current directory {}",
            cwd.display()
        ),
    };

    let metadata = fs::metadata(&candidate).map_err(|error| {
        let (code, what) = if error.kind() == ErrorKind::NotFound {
            ("policy_missing", "does not exist".to_owned())
        } else {
            ("policy_unreadable", format!("cannot be read: {error}"))
        };
        Refusal::new(
            code,
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} {what}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    if !metadata.is_file() {
        return Err(Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} is not a regular file"),
            remedy,
        )
        .source(shown));
    }
    let path = fs::canonicalize(&candidate).map_err(|error| {
        Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} cannot be resolved: {error}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    let path = import_specifier(&path)
        .map_err(|why| {
            Refusal::new(
                "policy_unreadable",
                Stage::Authority,
                EXIT_REFUSED,
                format!(
                    "the selected policy entry {shown} resolves to {}, which {why}, so the policy \
                     worker cannot import that exact file",
                    path.display()
                ),
                "rename the entry, or the directories above it, so that its resolved path is \
                 UTF-8 with no `?`, or name another entry with --config PATH",
            )
            .source(shown.clone())
        })?
        .to_owned();
    fs::read(&path).map_err(|error| {
        Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} cannot be read: {error}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    Ok(PolicyEntry {
        path,
        format: Format::TypeScript,
    })
}

/// Admit `candidate` as an entry of the Rhai path: a regular file, which the
/// front reads once later as UTF-8 text of at most [`RHAI_ENTRY_BYTES`]. A
/// symlink is admitted by its real file. (rhai)
fn resolve_rhai(
    candidate: &Path,
    authority: &Authority,
    cwd: &Path,
) -> Result<PolicyEntry, Refusal> {
    let shown = candidate.to_string_lossy().into_owned();
    let remedy = match authority {
        Authority::Personal => format!(
            "write {shown} defining SCHEMA_VERSION, VERSION and `fn select(request, context)`, \
             or name another entry with --config PATH"
        ),
        Authority::Explicit => format!(
            "check the --config path; a relative path resolves against the current directory {}",
            cwd.display()
        ),
    };
    let refuse = |code: &'static str, message: String, remedy: String| {
        Refusal::new(code, Stage::Authority, EXIT_REFUSED, message, remedy).source(shown.clone())
    };
    let unreadable = |what: String| {
        refuse(
            "policy_unreadable",
            format!("the selected policy entry {shown} {what}"),
            remedy.clone(),
        )
    };

    let metadata = fs::metadata(candidate).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            refuse(
                "policy_missing",
                format!("the selected policy entry {shown} does not exist"),
                remedy.clone(),
            )
        } else {
            unreadable(format!("cannot be read: {error}"))
        }
    })?;
    if !metadata.is_file() {
        return Err(unreadable("is not a regular file".to_owned()));
    }
    let real = fs::canonicalize(candidate)
        .map_err(|error| unreadable(format!("cannot be resolved: {error}")))?;
    let path = real
        .to_str()
        .ok_or_else(|| {
            unreadable(format!(
                "resolves to {}, which is not valid UTF-8, so no failure could name it exactly",
                real.display()
            ))
        })?
        .to_owned();

    Ok(PolicyEntry {
        path,
        format: Format::Rhai(RhaiEntry { shown, remedy }),
    })
}

/// The canonical `path` as the exact string the worker imports, or why no
/// string names it. The worker's runtime takes a string, and reads a `?` in it
/// as the start of a query even through a `file:` URL: under Bun 1.4.2,
/// `policy.ts?x` loads `policy.ts`, and `d?q/policy.ts` loads `d.ts`. A path
/// that is not UTF-8 has no exact string, only a lossy one naming another
/// file. Either would evaluate code other than the file admitted and hashed
/// here, so both refuse before a worker starts. (`#`, `%`, spaces and newlines
/// import exactly; a `\` fails to load rather than loading another file.)
fn import_specifier(path: &Path) -> Result<&str, &'static str> {
    let path = path.to_str().ok_or("is not valid UTF-8")?;
    if path.contains('?') {
        return Err("contains `?`, read by the worker's runtime as a query");
    }
    Ok(path)
}

/// The personal default of the Rhai path under `home`, read by [`resolve`]. (rhai)
pub fn personal_rhai_default(home: Option<&OsStr>) -> Result<PathBuf, Refusal> {
    Ok(personal_home(home)?.join(PERSONAL_RHAI_DEFAULT))
}

/// The personal default's path under `home`, read by [`resolve`]. (typescript)
pub fn personal_default(home: Option<&OsStr>) -> Result<PathBuf, Refusal> {
    Ok(personal_home(home)?.join(PERSONAL_DEFAULT))
}

/// `home`, once it is proven to be the absolute directory the personal
/// defaults are under.
fn personal_home(home: Option<&OsStr>) -> Result<&Path, Refusal> {
    let remedy = "set HOME to your absolute home directory, or name an entry with --config PATH";
    let home = home.filter(|home| !home.is_empty()).ok_or_else(|| {
        Refusal::new(
            "home_unset",
            Stage::Authority,
            EXIT_REFUSED,
            format!(
                "HOME is not set, so the personal policy ~/{PERSONAL_RHAI_DEFAULT} has no location"
            ),
            remedy,
        )
        .input("HOME")
    })?;
    let home = Path::new(home);
    // A relative HOME would resolve against the cwd, which is exactly the
    // repository-selected entry this module exists to rule out.
    if !home.is_absolute() {
        return Err(Refusal::new(
            "home_unset",
            Stage::Authority,
            EXIT_REFUSED,
            format!(
                "HOME is the relative path {}, so the personal policy would depend on the current directory",
                home.display()
            ),
            remedy,
        )
        .input("HOME"));
    }
    Ok(home)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt as _;

    #[test]
    fn only_a_path_the_worker_imports_exactly_is_a_specifier() {
        for exact in ["/p/policy.ts", "/p/a b#c%41\n.ts", "/p/é/policy.ts"] {
            assert_eq!(import_specifier(Path::new(exact)), Ok(exact));
        }
        for queried in ["/p/policy.ts?x", "/p/d?q/policy.ts"] {
            let why = import_specifier(Path::new(queried)).unwrap_err();
            assert!(why.contains('?'), "{queried}: {why}");
        }
        // The lossy form of this path is a different, UTF-8 file name.
        let native = Path::new(OsStr::from_bytes(b"/p/policy-\xff.ts"));
        assert_eq!(import_specifier(native), Err("is not valid UTF-8"));
    }
}
