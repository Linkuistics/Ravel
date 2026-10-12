use std::ffi::{CString, OsStr};
use std::fs;
use std::io::ErrorKind;
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};

use crate::refusal::{Refusal, Stage, EXIT_NOT_FOUND, EXIT_UNEXECUTABLE};

#[derive(Debug)]
pub struct Executable {
    /// The program exactly as `select` returned it; also argv[0].
    pub program: String,
    /// The exact executable path passed to the runner.
    pub path: String,
    pub resolved_by: ResolvedBy,
}

#[derive(Debug)]
pub enum ResolvedBy {
    Absolute,
    /// A relative path with a separator, joined to the caller's cwd.
    Cwd,
    /// Found in this PATH entry, as it was spelled in PATH.
    Path {
        entry: PathBuf,
    },
}

impl Executable {
    pub fn to_text(&self) -> String {
        let path = &self.path;
        match &self.resolved_by {
            ResolvedBy::Absolute => format!("{path} (an absolute path)"),
            ResolvedBy::Cwd => format!("{path} (relative to the current directory)"),
            ResolvedBy::Path { entry } => {
                format!("{path} (found on PATH in {})", entry.display())
            }
        }
    }
}

enum Probe {
    Executable,
    Missing,
    /// It exists, but exec would refuse it.
    Unexecutable(String),
}

/// Resolve `program`, which the `select` of the policy at `source` returned,
/// against the caller's `cwd` and `path` (the PATH value).
pub fn resolve(
    program: &str,
    source: &str,
    cwd: &Path,
    path: Option<&OsStr>,
) -> Result<Executable, Refusal> {
    let refuse = |code, exit, message: String, remedy: String| {
        Refusal::new(code, Stage::Resolution, exit, message, remedy)
            .source(source)
            .location("result.program")
    };
    let not_found = |searched: String| {
        refuse(
            "program_not_found",
            EXIT_NOT_FOUND,
            format!("select in {source} returned the program {program:?}, which {searched}"),
            format!(
                "install {program}, or correct the program select returns in {source}; \
                 ravel never runs another command instead"
            ),
        )
    };
    let unexecutable = |file: &Path, why: String| {
        refuse(
            "program_unexecutable",
            EXIT_UNEXECUTABLE,
            format!(
                "select in {source} returned the program {program:?}, and {} cannot be \
                 executed: {why}",
                file.display()
            ),
            format!(
                "make {} an executable file, or correct the program select returns in {source}; \
                 ravel never runs another command instead",
                file.display()
            ),
        )
    };
    // Refused where it is found, not passed over: the caller's shell would
    // have run this file, and a later match is another command.
    let found = |file: PathBuf, resolved_by: ResolvedBy| match file.to_str() {
        Some(path) => Ok(Executable {
            program: program.to_owned(),
            path: path.to_owned(),
            resolved_by,
        }),
        None => Err(refuse(
            "program_unexecutable",
            EXIT_UNEXECUTABLE,
            format!(
                "select in {source} returned the program {program:?}, and {} is not run: its \
                 path is not valid UTF-8, so routing diagnostics cannot name it exactly",
                file.display()
            ),
            format!(
                "reach {program} through a path that is UTF-8, or correct the program select \
                 returns in {source}; ravel never runs another command instead"
            ),
        )),
    };

    if program.contains('/') {
        let (file, resolved_by) = if program.starts_with('/') {
            (PathBuf::from(program), ResolvedBy::Absolute)
        } else {
            (cwd.join(program), ResolvedBy::Cwd)
        };
        return match probe(&file) {
            Probe::Executable => found(file, resolved_by),
            Probe::Missing => Err(not_found(format!("does not exist at {}", file.display()))),
            Probe::Unexecutable(why) => Err(unexecutable(&file, why)),
        };
    }

    // An unset PATH leaves no caller's PATH to search. An empty one is not
    // unset: it is a single empty entry, the cwd, as below.
    let Some(path) = path else {
        return Err(not_found(
            "cannot be found because PATH is unset".to_owned(),
        ));
    };
    let mut first_unexecutable = None;
    for entry in std::env::split_paths(path) {
        // As execvp: an empty entry is the cwd, and a relative one is under it.
        let file = cwd.join(&entry).join(program);
        match probe(&file) {
            Probe::Executable => return found(file, ResolvedBy::Path { entry }),
            Probe::Missing => {}
            Probe::Unexecutable(why) => {
                first_unexecutable.get_or_insert((file, why));
            }
        }
    }
    match first_unexecutable {
        Some((file, why)) => Err(unexecutable(&file, why)),
        None => Err(not_found(format!(
            "is not in any PATH directory ({})",
            path.to_string_lossy()
        ))),
    }
}

fn probe(file: &Path) -> Probe {
    let metadata = match fs::metadata(file) {
        Ok(metadata) => metadata,
        Err(error)
            if error.kind() == ErrorKind::NotFound
                || error.raw_os_error() == Some(libc::ENOTDIR) =>
        {
            return Probe::Missing
        }
        Err(error) => return Probe::Unexecutable(error.to_string()),
    };
    if metadata.is_dir() {
        return Probe::Unexecutable("it is a directory".to_owned());
    }
    if !metadata.is_file() {
        return Probe::Unexecutable("it is not a regular file".to_owned());
    }
    let Ok(file) = CString::new(file.as_os_str().as_bytes()) else {
        return Probe::Unexecutable("its path contains a NUL byte".to_owned());
    };
    // Execute permission for the effective IDs, which are what exec checks.
    // SAFETY: faccessat reads the NUL-terminated path and nothing else.
    let access =
        unsafe { libc::faccessat(libc::AT_FDCWD, file.as_ptr(), libc::X_OK, libc::AT_EACCESS) };
    if access == 0 {
        Probe::Executable
    } else {
        Probe::Unexecutable(format!(
            "it is not executable ({})",
            std::io::Error::last_os_error()
        ))
    }
}
