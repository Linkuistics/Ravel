//! Local ownership of ephemeral directories. Identity checks are snapshots;
//! concurrent pathname mutation between check and removal is outside this contract.
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt as _;
use std::path::{Path, PathBuf};

pub(crate) const CLEANUP_ADVICE: &str = "inspect the named path and locate the moved owned directory if necessary; verify ownership before manual removal or retry";

pub(crate) struct PrivateDirectory {
    path: PathBuf,
    identity: Option<(u64, u64)>,
    armed: bool,
    #[cfg(test)]
    role: &'static str,
}

impl PrivateDirectory {
    pub(crate) fn new(temporary: tempfile::TempDir, _role: &'static str) -> Self {
        // keep consumes and disarms TempDir before even identity capture can fail.
        // Verified against the pinned crate source, src/dir/mod.rs:
        // https://docs.rs/tempfile/3.27.0/tempfile/struct.TempDir.html#method.keep
        let path = temporary.keep();
        #[cfg(test)]
        crate::directory_control::created(_role, &path);
        Self {
            path,
            identity: None,
            armed: true,
            #[cfg(test)]
            role: _role,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn record_identity(&mut self) -> io::Result<()> {
        let metadata = fs::symlink_metadata(&self.path);
        #[cfg(test)]
        crate::directory_control::receipt(format!(
            "actual identity capture path={} result={metadata:?}",
            self.path.display()
        ));
        #[cfg(test)]
        let metadata =
            crate::directory_control::operation(self.role, "identity", &self.path, metadata);
        let metadata = metadata?;
        if !metadata.is_dir() {
            return Err(io::Error::other("new private directory is not a directory"));
        }
        self.identity = Some((metadata.dev(), metadata.ino()));
        Ok(())
    }

    pub(crate) fn cleanup(&mut self) -> io::Result<()> {
        if !self.armed {
            return Ok(());
        }
        // An explicit failed attempt must not be retried by best-effort Drop.
        self.armed = false;
        let result = (|| {
            #[cfg(test)]
            let controlled = crate::directory_control::cleanup(self.role, &self.path);
            let identity = self.identity.ok_or_else(|| {
                io::Error::other("owned identity unknown; deletion cannot be confirmed")
            })?;
            #[cfg(test)]
            controlled?;
            let current = fs::symlink_metadata(&self.path);
            #[cfg(test)]
            crate::directory_control::receipt(format!(
                "actual identity check path={} result={current:?}",
                self.path.display()
            ));
            let current = current.map_err(|original| {
                context(
                    original,
                    "symlink_metadata: owned directory missing/moved or identity unreadable".into(),
                )
            })?;
            if !current.is_dir() || (current.dev(), current.ino()) != identity {
                return Err(io::Error::other(
                    "identity mismatch: owned directory renamed or replaced; cleanup refused",
                ));
            }
            let removed = fs::remove_dir_all(&self.path);
            #[cfg(test)]
            crate::directory_control::receipt(format!(
                "actual remove_dir_all path={} result={removed:?}",
                self.path.display()
            ));
            removed.map_err(|original| context(original, "remove_dir_all".into()))
        })();
        #[cfg(test)]
        crate::directory_control::receipt(format!(
            "physical/reported cleanup role={} path={} recorded={:?} result={result:?}",
            self.role,
            self.path.display(),
            self.identity
        ));
        result.map_err(|original| {
            context(
                original,
                format!(
                    "private directory {}; {CLEANUP_ADVICE}",
                    self.path.display()
                ),
            )
        })
    }
}

impl Drop for PrivateDirectory {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[derive(Debug)]
struct ContextError {
    original: io::Error,
    context: String,
}
impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.original)
    }
}
impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.original)
    }
}
pub(crate) fn context(original: io::Error, context: String) -> io::Error {
    // Error::new retains a typed source and the original kind. The original
    // errno remains in that source; raw_os_error on the contextual wrapper is None.
    // https://doc.rust-lang.org/std/io/struct.Error.html#method.new
    io::Error::new(original.kind(), ContextError { original, context })
}
