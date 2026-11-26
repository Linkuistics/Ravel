//! The reader thread: every filesystem call of the host functions
//! (`docs/specs/configuration-evaluation.md`, *The bridge between the script and the CLI*).
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
//!
//! A stop can leave the thread alive through the finalisation of the CLI, so
//! it must never be eligible for the handled signals. The creator blocks them
//! before the thread exists, the thread inherits the block for its whole life,
//! and only the creator is restored, as for the diagnostic readers
//! (`docs/specs/result-job-completion.md`, *Checked native-entry transitions*).

use std::fs;
use std::io::{self, Read as _};
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use tokio::sync::oneshot;

use crate::cancellation::Shield;

type Job = Box<dyn FnOnce() + Send>;

/// The thread that runs the filesystem work of one evaluation.
pub struct Reader {
    jobs: mpsc::Sender<Job>,
}

impl Reader {
    /// Start the thread with the handled signals blocked, and leave the calling
    /// thread's mask as it was. A failure of the block, of the creation or of
    /// the restoration is an error. No thread outlives an error, because the
    /// queue closes when it returns.
    pub fn start() -> io::Result<Self> {
        let (jobs, queue) = mpsc::channel::<Job>();
        let shield = Shield::block()
            .map_err(|error| io::Error::other(format!("reader mask block: {error}")))?;
        let created = thread::Builder::new()
            .name("ravel-reader".into())
            .spawn(move || queue.into_iter().for_each(|job| job()));
        let restored = shield.restore();
        match (created, restored) {
            (Ok(_), Ok(())) => Ok(Reader { jobs }),
            (Err(created), Ok(())) => {
                Err(io::Error::other(format!("reader thread create: {created}")))
            }
            (Ok(_), Err(restored)) => Err(io::Error::other(format!(
                "reader creator mask restore: {restored}"
            ))),
            (Err(created), Err(restored)) => Err(io::Error::other(format!(
                "reader thread create: {created}; reader creator mask restore: {restored}"
            ))),
        }
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

    /// Which of INT, TERM and HUP the calling thread blocks, in that order.
    fn blocked() -> [bool; 3] {
        let mut mask = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        // SAFETY: a null new set only reads the mask of the calling thread.
        assert_eq!(
            unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &mut mask) },
            0
        );
        [libc::SIGINT, libc::SIGTERM, libc::SIGHUP]
            .map(|signal| unsafe { libc::sigismember(&mask, signal) } == 1)
    }

    fn mask_of(reader: &Reader) -> [bool; 3] {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(reader.run(blocked))
            .unwrap()
    }

    #[test]
    fn the_reader_thread_blocks_the_handled_signals_and_the_creator_keeps_its_mask() {
        // The firing control: a thread that is made the plain way is eligible,
        // so the instrument tells a shielded thread from an eligible one.
        assert_eq!(
            thread::spawn(blocked).join().unwrap(),
            [false; 3],
            "the test thread must start with the handled signals unblocked"
        );
        let before = blocked();

        let reader = Reader::start().unwrap();

        assert_eq!(mask_of(&reader), [true; 3]);
        assert_eq!(blocked(), before);
    }

    #[test]
    fn a_failed_block_creates_no_reader() {
        crate::worker::native_tests::configure("block-omitted");
        let before = blocked();

        let error = Reader::start().err().expect("the block fails");

        assert!(error.to_string().contains("reader mask block"), "{error}");
        assert_eq!(blocked(), before);
    }

    #[test]
    fn a_failed_creator_restore_is_an_error() {
        crate::worker::native_tests::configure("restore-performed");
        let before = blocked();

        let error = Reader::start().err().expect("the restore fails");

        assert!(
            error.to_string().contains("reader creator mask restore"),
            "{error}"
        );
        assert_eq!(blocked(), before);
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
