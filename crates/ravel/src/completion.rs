//! An ephemeral opaque result carried by the runner's appearance-only channel.

use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;

use keyed_launch::{Channel, End, Ended, Group};
use tempfile::NamedTempFile;

/// Inherited authority for exactly the enclosing session; nested launches replace it.
pub const EXIT_FILE_VARIABLE: &str = "RAVEL_EXIT_FILE";

/// A generous bound above the supported systems' command-argument limits.
/// Also bounds reads of a channel modified by the managed process.
pub const MAX_RESULT_BYTES: usize = 1024 * 1024;

/// Owns a fresh private directory, a completion path and an anchored reader.
/// Drop removes the ephemeral directory; no durable state is created.
pub struct SessionChannel {
    directory: File,
    channel: Channel,
    temporary: crate::private_directory::PrivateDirectory,
}

impl SessionChannel {
    /// Allocate beneath an existing caller-selected directory, so a sandbox
    /// can grant this location. The child receives the channel path, not a
    /// policy-controlled destination. The path is absolute even for a relative parent.
    pub fn allocate(parent: &Path) -> io::Result<Self> {
        let parent = fs::canonicalize(parent)?;
        #[cfg(test)]
        let parent = crate::directory_control::parent("session", parent);
        let temporary = tempfile::Builder::new()
            .prefix("ravel-session-")
            .tempdir_in(parent)?;
        let mut temporary = crate::private_directory::PrivateDirectory::new(temporary, "session");
        let prepared = (|| {
            temporary.record_identity()?;
            let directory = open_directory(temporary.path())?;
            #[cfg(test)]
            let directory = crate::directory_control::operation(
                "session",
                "reader",
                temporary.path(),
                Ok(directory),
            )?;
            #[cfg(test)]
            crate::directory_control::before_channel(temporary.path());
            let channel = Channel::allocate(temporary.path()).map_err(|error| {
                let kind = std::error::Error::source(&error)
                    .and_then(|source| source.downcast_ref::<io::Error>())
                    .map_or(io::ErrorKind::Other, io::Error::kind);
                io::Error::new(kind, error)
            })?;
            #[cfg(test)]
            let channel = crate::directory_control::operation(
                "session",
                "channel",
                temporary.path(),
                Ok(channel),
            )?;
            Ok((directory, channel))
        })();
        match prepared {
            Ok((directory, channel)) => Ok(Self {
                directory,
                channel,
                temporary,
            }),
            Err(original) => {
                let cleanup = temporary.cleanup();
                Err(match cleanup {
                    Ok(()) => original,
                    Err(error) => crate::private_directory::context(
                        original,
                        format!("session allocation failed; cleanup: {error}"),
                    ),
                })
            }
        }
    }

    /// Pass this channel with [`EXIT_FILE_VARIABLE`] in `keyed_launch::Launch`.
    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    /// Read only after the corresponding runner launch has returned. A live
    /// process group or cancellation never yields a successful session result.
    /// `None` is distinct from an explicitly published empty string.
    /// The caller must pass this channel's launch ending, not another launch's.
    pub fn result_after(&self, ended: &Ended) -> io::Result<Option<String>> {
        if ended.group != Group::Gone {
            return Err(io::Error::other("session process group is still present"));
        }
        if matches!(ended.end, End::Interrupted { .. }) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "session was cancelled; no result is returned",
            ));
        }
        if !ended.signalled {
            return Ok(None);
        }
        self.read_result()
    }

    /// Remove ephemeral files explicitly when cleanup errors must be reported.
    pub fn close(mut self) -> io::Result<()> {
        self.cleanup()
    }

    fn cleanup(&mut self) -> io::Result<()> {
        self.temporary.cleanup()
    }

    fn read_result(&self) -> io::Result<Option<String>> {
        let name = self
            .channel
            .path()
            .file_name()
            .ok_or_else(|| io::Error::other("completion path has no file name"))?;
        let name = CString::new(name.as_bytes())
            .map_err(|_| io::Error::other("completion file name contains NUL"))?;
        // SAFETY: directory is held for this call; name is NUL-terminated.
        // O_NOFOLLOW refuses symlinks, O_NONBLOCK prevents FIFO reads hanging.
        let fd = unsafe {
            libc::openat(
                self.directory.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            let error = io::Error::last_os_error();
            return if error.kind() == io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(error)
            };
        }
        // SAFETY: openat returned a new owned descriptor, transferred once.
        read_regular(unsafe { File::from_raw_fd(fd) }).map(Some)
    }
}

impl Drop for SessionChannel {
    fn drop(&mut self) {
        // Like TempDir, Drop is best effort. Use close to report cleanup errors.
        let _ = self.cleanup();
    }
}

/// Publish the sole opaque argument from inherited enclosing-session context.
/// Missing/empty/stale context refuses; it never targets another session.
pub fn end_session(result: &str) -> io::Result<()> {
    let path = std::env::var_os(EXIT_FILE_VARIABLE)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "no enclosing Ravel session (RAVEL_EXIT_FILE is unset or empty)",
            )
        })?;
    publish_result(Path::new(&path), result)
}

/// Private runtime mechanism; the command front exposes no path/target argument.
/// First successful publication wins. Repeated calls validate the existing
/// regular result and succeed without changing it, regardless of their value.
pub fn publish_result(path: &Path, result: &str) -> io::Result<()> {
    if result.len() > MAX_RESULT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("session result exceeds {MAX_RESULT_BYTES} UTF-8 bytes"),
        ));
    }
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "session channel needs a directory",
            )
        })?;
    // Refuse a substituted final directory symlink before creating any file.
    // Ancestors may legitimately include the system's temporary-directory link.
    let _directory = open_directory(parent)?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(result.as_bytes())?;
    temporary.flush()?;
    // link(2) adds one name for a fully written inode without replacing an
    // existing name. The source is in the same private directory/filesystem.
    // https://pubs.opengroup.org/onlinepubs/9799919799/functions/link.html
    // Avoid persist_noclobber's weaker cross-platform atomicity promise:
    // https://docs.rs/tempfile/3.27.0/tempfile/struct.NamedTempFile.html#method.persist_noclobber
    match fs::hard_link(temporary.path(), path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            let existing = OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
                .open(path)?;
            read_regular(existing).map(|_| ())
        }
        Err(error) => Err(error),
    }
}

fn open_directory(path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(path)
}

fn read_regular(file: File) -> io::Result<String> {
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "session result must be a regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.take(MAX_RESULT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_RESULT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "session result exceeds its byte bound",
        ));
    }
    String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

#[cfg(test)]
#[path = "completion_tests.rs"]
mod tests;
