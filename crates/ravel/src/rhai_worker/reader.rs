//! The reader thread: every filesystem call of the host functions
//! (`docs/specs/policy-evaluation.md`, *The bridge between the script and the CLI*).
//!
//! The runtime thread that serves the worker makes no filesystem call. It hands
//! a closure to the reader and awaits the answer, so the deadline, the
//! cancellation latch and the health of the worker stay watched while a read is
//! in progress. The reader is a plain thread and not a blocking task of the
//! runtime, because Tokio cannot stop a running blocking task, and the shutdown
//! of its runtime waits for one
//! (<https://docs.rs/tokio/1/tokio/task/fn.spawn_blocking.html#cancellation>).
//!
//! A stop drops the receiver and then the reader. The thread finishes the call
//! it is in, drops what that call opened and ends, or ends with the process.
//! Every file is opened by `std`, which sets close-on-exec, so no harness run
//! inherits a descriptor that a stalled read still holds.

use std::fs;
use std::io::{self, Read as _};
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use tokio::sync::oneshot;

type Job = Box<dyn FnOnce() + Send>;

/// The thread that runs the filesystem work of one evaluation.
pub struct Reader {
    jobs: mpsc::Sender<Job>,
}

impl Reader {
    pub fn start() -> io::Result<Self> {
        let (jobs, queue) = mpsc::channel::<Job>();
        thread::Builder::new()
            .name("ravel-reader".into())
            .spawn(move || queue.into_iter().for_each(|job| job()))?;
        Ok(Reader { jobs })
    }

    /// Run `work` on the reader thread. The receiver gets its value, or an
    /// error if the thread ended first. Dropping the receiver abandons the call:
    /// the thread still finishes `work`.
    pub fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> oneshot::Receiver<T> {
        let (answer, receiver) = oneshot::channel();
        // A send fails only when the thread has ended, and then the answer is
        // dropped with the job, which the receiver reports as an error.
        let _ = self.jobs.send(Box::new(move || {
            let _ = answer.send(work());
        }));
        receiver
    }
}

/// A regular file as read: its canonical name and at most the bound's bytes.
#[derive(Debug)]
pub struct Content {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// Why a file could not be read, with the reason as text.
#[derive(Debug, PartialEq, Eq)]
pub enum FileFailure {
    Missing(String),
    NotRegular,
    Unreadable(String),
    /// The file holds more than the bound. `actual` is what was seen: the bytes
    /// read, or the size the file reports if that is more.
    TooLarge {
        name: String,
        actual: u64,
    },
}

/// Read the regular file at `path` once, and at most `bound` bytes of it.
///
/// The open is nonblocking, so a FIFO that no process writes cannot hold the
/// call. The open file is then checked to be a regular file, which a FIFO is
/// not. The read stops one byte beyond the bound.
pub fn read_file(path: &Path, bound: u64) -> Result<Content, FileFailure> {
    let file = match fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(FileFailure::Missing(error.to_string()));
        }
        // A socket or another special file refuses the open itself.
        Err(error) => {
            return Err(match fs::metadata(path) {
                Ok(metadata) if !metadata.is_file() => FileFailure::NotRegular,
                _ => FileFailure::Unreadable(format!("cannot be opened: {error}")),
            });
        }
    };
    let metadata = file
        .metadata()
        .map_err(|error| FileFailure::Unreadable(format!("cannot be examined: {error}")))?;
    if !metadata.is_file() {
        return Err(FileFailure::NotRegular);
    }
    let name = canonical(path)?;
    let mut bytes = Vec::new();
    file.take(bound.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| FileFailure::Unreadable(format!("cannot be read: {error}")))?;
    if bytes.len() as u64 > bound {
        let actual = metadata.len().max(bytes.len() as u64);
        return Err(FileFailure::TooLarge { name, actual });
    }
    Ok(Content { name, bytes })
}

/// The canonical name of `path`, which must be UTF-8 for a failure or a notice
/// to be able to name it.
pub fn canonical(path: &Path) -> Result<String, FileFailure> {
    fs::canonicalize(path)
        .map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => FileFailure::Missing(error.to_string()),
            _ => FileFailure::Unreadable(format!("cannot be resolved: {error}")),
        })?
        .into_os_string()
        .into_string()
        .map_err(|_| {
            FileFailure::Unreadable(
                "resolves to a path that is not valid UTF-8, which no failure can name".to_owned(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt as _;

    fn fifo(path: &Path) {
        let path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: a valid NUL-terminated path in a temporary directory.
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
    }

    #[test]
    fn a_regular_file_reads_with_its_canonical_name() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "four").unwrap();
        let content = read_file(&file, 4).unwrap();
        assert_eq!(content.bytes, b"four");
        assert_eq!(
            content.name,
            fs::canonicalize(&file).unwrap().to_str().unwrap()
        );
    }

    #[test]
    fn a_file_over_the_bound_reports_what_was_seen() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "twelve bytes").unwrap();
        match read_file(&file, 4) {
            Err(FileFailure::TooLarge { actual, .. }) => assert_eq!(actual, 12),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_fifo_with_no_writer_is_not_regular_and_does_not_block() {
        let dir = tempfile::tempdir().unwrap();
        let pipe = dir.path().join("pipe");
        fifo(&pipe);
        assert_eq!(read_file(&pipe, 4).unwrap_err(), FileFailure::NotRegular);
    }

    #[test]
    fn a_missing_file_and_a_directory_are_told_apart() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            read_file(&dir.path().join("none"), 4),
            Err(FileFailure::Missing(_))
        ));
        assert_eq!(
            read_file(dir.path(), 4).unwrap_err(),
            FileFailure::NotRegular
        );
    }

    #[test]
    fn a_job_runs_on_another_thread_and_an_abandoned_call_still_finishes() {
        let reader = Reader::start().unwrap();
        let here = thread::current().id();
        let there = reader.run(move || thread::current().id());
        let there = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(there)
            .unwrap();
        assert_ne!(here, there);

        let (done, finished) = mpsc::channel();
        drop(reader.run(move || {
            thread::sleep(std::time::Duration::from_millis(50));
            done.send(()).unwrap();
        }));
        assert!(
            finished
                .recv_timeout(std::time::Duration::from_secs(5))
                .is_ok()
        );
    }
}
