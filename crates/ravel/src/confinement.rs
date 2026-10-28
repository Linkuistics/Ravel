//! Pre-selection canonical grant validation and the confined harness's grants.
//! Owner resources are resolved from the same Prepared selection that runs.
use std::ffi::OsString;
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Component, Path, PathBuf};

use crate::choice::Prepared;
use crate::refusal::{EXIT_MALFORMED, Refusal, Stage};

pub struct Confined {
    private: crate::private_directory::PrivateDirectory,
    pub writable: Vec<PathBuf>,
    pub runtime_read: Vec<PathBuf>,
    executable: PathBuf,
    temporary: PathBuf,
}

fn unusable(path: &Path, problem: impl std::fmt::Display) -> Refusal {
    Refusal::new(
        "confinement_unusable", Stage::Cli, EXIT_MALFORMED,
        format!("cannot confine with {}: {problem}; nothing was selected", path.display()),
        "supply existing writable directories other than /, regular runtime files, and the system confinement backend; no unconfined fallback is available",
    ).input("--confine")
}

/// What a confined launch protects and grants, from the selection that it
/// belongs to: the working directory, the configuration entry and the owner settings.
/// The managed process and each harness run of one selection share it.
#[derive(Clone, Debug)]
pub struct Basis {
    pub cwd: PathBuf,
    pub entry: String,
    pub settings: Option<PathBuf>,
}

impl From<&Prepared> for Basis {
    fn from(selection: &Prepared) -> Self {
        Basis {
            cwd: selection.inputs.cwd.clone(),
            entry: selection.entry.path.clone(),
            settings: selection.settings_path.clone(),
        }
    }
}

impl Confined {
    pub fn prepare(
        selection: &Prepared,
        exit_dir: Option<&Path>,
        runtime_read: &[PathBuf],
    ) -> Result<Self, Refusal> {
        Self::prepare_for(&Basis::from(selection), exit_dir, runtime_read, true)
    }

    /// A private run directory of its own, with the grants of `basis`. The
    /// probe runs a program in the sandbox, so a harness run skips it: the
    /// managed process proved the backend before the worker started, and the
    /// probe changes the disposition of SIGCHLD while the worker is alive.
    pub fn prepare_for(
        selection: &Basis,
        exit_dir: Option<&Path>,
        runtime_read: &[PathBuf],
        probe: bool,
    ) -> Result<Self, Refusal> {
        let cwd = selection
            .cwd
            .canonicalize()
            .map_err(|error| unusable(&selection.cwd, error))?;
        if cwd.parent().is_none() {
            return Err(unusable(&cwd, "the cwd must not be /"));
        }
        if probe {
            keyed_launch::confinement_available().map_err(|error| unusable(&cwd, error))?;
        }
        #[cfg(test)]
        let parent = crate::directory_control::parent("confined", std::env::temp_dir());
        let private = tempfile::Builder::new()
            .prefix("ravel-confined-")
            .permissions(std::fs::Permissions::from_mode(0o700))
            .tempdir_in({
                #[cfg(test)]
                {
                    parent
                }
                #[cfg(not(test))]
                {
                    std::env::temp_dir()
                }
            })
            .map_err(|error| unusable(&std::env::temp_dir(), error))?;
        let mut private = crate::private_directory::PrivateDirectory::new(private, "confined");
        let prepared = (|| -> Result<_, Refusal> {
            private
                .record_identity()
                .map_err(|error| unusable(private.path(), error))?;
            let root = private
                .path()
                .canonicalize()
                .map_err(|error| unusable(private.path(), error))?;
            #[cfg(test)]
            let root = crate::directory_control::operation(
                "confined",
                "canonical",
                private.path(),
                Ok(root),
            )
            .map_err(|e| unusable(private.path(), e))?;
            #[cfg(test)]
            crate::directory_control::operation("confined", "runtime", private.path(), Ok(()))
                .map_err(|e| unusable(private.path(), e))?;
            let mut writable = vec![cwd, root.clone()];
            if let Some(exit_dir) = exit_dir {
                writable.push(exit_dir.to_owned());
            }
            let mut reads = Vec::new();
            for path in runtime_read {
                let resolved = path.canonicalize().map_err(|error| unusable(path, error))?;
                if !resolved.is_file() {
                    return Err(unusable(path, "a runtime read must name a regular file"));
                }
                if resolved.to_str().is_none() {
                    return Err(unusable(
                        path,
                        "a runtime path must be UTF-8 to be named exactly",
                    ));
                }
                reads.push(resolved);
            }
            let mut protected = vec![prospective(
                Path::new(&selection.entry)
                    .parent()
                    .expect("absolute policy has a parent"),
            )?];
            if let Some(settings) = &selection.settings {
                protected.push(prospective(settings)?);
            }
            #[cfg(test)]
            crate::directory_control::operation("confined", "protected", private.path(), Ok(()))
                .map_err(|e| unusable(private.path(), e))?;
            let system_reads = keyed_launch::confinement_system_reads()
                .iter()
                .map(|path| prospective(Path::new(path)))
                .collect::<Result<Vec<_>, _>>()?;
            for grant in writable.iter().chain(&reads).chain(&system_reads) {
                for protected in &protected {
                    if grant.starts_with(protected) || protected.starts_with(grant) {
                        return Err(Refusal::new(
                        "confinement_overlap", Stage::Cli, EXIT_MALFORMED,
                        format!("confinement grant {} overlaps protected path {}; nothing was selected", grant.display(), protected.display()),
                        "use a working directory and grants separate from the configuration script directory, owner settings, owner settings",
                    ).input("--confine"));
                    }
                }
            }
            let executable = std::env::current_exe()
                .and_then(std::fs::canonicalize)
                .map_err(|error| unusable(Path::new("ravel"), error))?;
            let temporary = root.join("tmp");
            std::fs::create_dir(&temporary).map_err(|error| unusable(&temporary, error))?;
            #[cfg(test)]
            crate::directory_control::operation("confined", "temporary", private.path(), Ok(()))
                .map_err(|e| unusable(private.path(), e))?;
            Ok((writable, reads, executable, temporary))
        })();
        match prepared {
            Ok((writable, runtime_read, executable, temporary)) => Ok(Self {
                private,
                writable,
                runtime_read,
                executable,
                temporary,
            }),
            Err(original) => {
                let cleanup = private.cleanup();
                Err(match cleanup {
                    Ok(()) => original,
                    Err(error) => original
                        .report(&error.to_string(), crate::private_directory::CLEANUP_ADVICE),
                })
            }
        }
    }

    pub fn exit_dir(&self) -> &Path {
        &self.writable[1]
    }

    pub fn reads(&self) -> Vec<PathBuf> {
        let mut reads = self.runtime_read.clone();
        reads.push(self.executable.clone());
        reads
    }

    pub fn environment(&self) -> Vec<(OsString, OsString)> {
        let mut environment: Vec<_> = std::env::vars_os()
            .filter(|(name, _)| {
                matches!(
                    name.to_str(),
                    Some("HOME" | "USER" | "LOGNAME" | "PATH" | "LANG")
                ) || name.to_str().is_some_and(|name| name.starts_with("LC_"))
            })
            .collect();
        for name in ["TMPDIR", "TMP", "TEMP"] {
            environment.push((name.into(), self.temporary.as_os_str().to_owned()));
        }
        environment
    }

    pub fn remove(mut self) -> Result<(), String> {
        self.private
            .cleanup()
            .map_err(|error| format!("cannot remove confined run directory: {error}"))
    }
}

/// Canonicalize every existing component, retaining a normalized absent tail.
/// Settings can be absent on first use; symlinked ancestors and `..`
/// still refer to their actual filesystem locations. A dangling symlink is an
/// unusable path rather than a missing ordinary component.
fn prospective(path: &Path) -> Result<PathBuf, Refusal> {
    let mut resolved = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir => resolved.push("/"),
            Component::CurDir => {}
            Component::ParentDir => {
                resolved.pop();
            }
            Component::Normal(name) => {
                resolved.push(name);
                match std::fs::symlink_metadata(&resolved) {
                    Ok(_) => {
                        resolved = resolved
                            .canonicalize()
                            .map_err(|error| unusable(path, error))?
                    }
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                    Err(error) => return Err(unusable(path, error)),
                }
            }
            Component::Prefix(_) => return Err(unusable(path, "expected a POSIX path")),
        }
    }
    Ok(resolved)
}
