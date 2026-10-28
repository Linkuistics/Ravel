use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::refusal::{EXIT_REFUSED, Refusal, Stage};

/// The personal default of the Rhai path, relative to HOME. `XDG_CONFIG_HOME`
/// is deliberately not consulted: it would be an environment-selected entry.
pub const PERSONAL_RHAI_DEFAULT: &str = ".config/ravel/config.rhai";

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

/// What a refusal about the read of a Rhai entry says. (rhai)
#[derive(Debug)]
pub struct RhaiEntry {
    /// The path as it was selected, which the refusal names.
    shown: String,
    remedy: String,
}

impl RhaiEntry {
    /// The refusal for an entry that cannot be read, where `what` completes the
    /// sentence "the selected configuration entry PATH ...".
    pub fn unreadable(&self, what: &str) -> Refusal {
        Refusal::new(
            "config_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected configuration entry {} {what}", self.shown),
            self.remedy.clone(),
        )
        .source(self.shown.clone())
    }
}

#[derive(Debug)]
pub struct ConfigEntry {
    /// The canonical path; symlinks are resolved, so this is the file whose
    /// directory relative imports start from.
    pub path: String,
    pub admitted: RhaiEntry,
}

impl ConfigEntry {
    pub fn display(&self) -> String {
        self.path.clone()
    }
}

/// Resolve the selected entry and prove it is a readable regular file.
///
/// The personal Rhai entry wins even when it is unreadable or a dangling
/// symlink. A retired default is only inspected when that entry is absent.
pub fn resolve(
    config: Option<&Path>,
    cwd: &Path,
    home: Option<&OsStr>,
) -> Result<ConfigEntry, Refusal> {
    let (candidate, authority) = match config {
        Some(config) => (cwd.join(config), Authority::Explicit),
        None => {
            let rhai = personal_rhai_default(home)?;
            // Only NotFound means absent; permission errors must not select a
            // different entry. symlink_metadata also preserves dangling links.
            if fs::symlink_metadata(&rhai).is_err_and(|error| error.kind() == ErrorKind::NotFound) {
                // This names the shipped 1.x default, not another 2.0 entry.
                let retired = rhai.with_file_name("policy.ts");
                if fs::symlink_metadata(&retired).is_ok() {
                    return Err(retired_format(&retired));
                }
            }
            (rhai, Authority::Personal)
        }
    };
    match candidate.extension().and_then(OsStr::to_str) {
        Some(RHAI_EXTENSION) => resolve_rhai(&candidate, &authority, cwd),
        Some("ts" | "mts" | "cts" | "js" | "mjs" | "cjs") => Err(retired_format(&candidate)),
        _ => Err(Refusal::new(
            "config_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!(
                "the selected configuration entry {} has an unsupported extension",
                candidate.display()
            ),
            "name a .rhai file with --config PATH",
        )
        .source(candidate.to_string_lossy())),
    }
}

fn retired_format(path: &Path) -> Refusal {
    Refusal::new(
        "config_format_retired",
        Stage::Authority,
        EXIT_REFUSED,
        format!("the selected configuration entry {} is TypeScript or JavaScript, which this release no longer evaluates", path.display()),
        "write config.rhai using chapter 4 of the book, \"Write a configuration script\", and name a .rhai file with --config PATH if it is elsewhere",
    ).source(path.to_string_lossy())
}

/// Admit `candidate` as an entry of the Rhai path: a regular file, which the
/// front reads once later as UTF-8 text of at most [`RHAI_ENTRY_BYTES`]. A
/// symlink is admitted by its real file. (rhai)
fn resolve_rhai(
    candidate: &Path,
    authority: &Authority,
    cwd: &Path,
) -> Result<ConfigEntry, Refusal> {
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
            "config_unreadable",
            format!("the selected configuration entry {shown} {what}"),
            remedy.clone(),
        )
    };

    let metadata = fs::metadata(candidate).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            refuse(
                "config_missing",
                format!("the selected configuration entry {shown} does not exist"),
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

    Ok(ConfigEntry {
        path,
        admitted: RhaiEntry { shown, remedy },
    })
}

/// The personal default of the Rhai path under `home`, read by [`resolve`]. (rhai)
pub fn personal_rhai_default(home: Option<&OsStr>) -> Result<PathBuf, Refusal> {
    Ok(personal_home(home)?.join(PERSONAL_RHAI_DEFAULT))
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
                "HOME is not set, so the personal configuration script ~/{PERSONAL_RHAI_DEFAULT} has no location"
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
                "HOME is the relative path {}, so the personal configuration script would depend on the current directory",
                home.display()
            ),
            remedy,
        )
        .input("HOME"));
    }
    Ok(home)
}
