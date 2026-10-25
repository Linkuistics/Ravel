//! The pipes of a captured job: standard input written from memory, and
//! standard output and error read into bounded buffers.
//!
//! One thread serves all three pipes with `poll(2)`, so that no pipe can stall
//! another. It never blocks for longer than [`TICK`], and it ends when the
//! caller sets `done` after the job is reaped. A descendant that left the
//! group of the job and still holds a pipe therefore cannot hold the thread.

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::fd::{AsRawFd as _, OwnedFd};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

/// How long the thread waits in `poll(2)` before it looks at `done`.
const TICK: Duration = Duration::from_millis(20);

/// The most that one read takes from a pipe.
const CHUNK: usize = 64 * 1024;

/// The most reads of one pass over a pipe. A pipe that never runs dry, because
/// a writer keeps it full, cannot hold the thread: the pass ends, the overflow
/// is published, and `done` is looked at. 16 chunks are 1 MiB, more than any
/// pipe holds at rest.
const CHUNKS_PER_PASS: usize = 16;

/// A request to stop a job, set by one thread and read by the runner. Clones
/// share the request.
#[derive(Clone, Debug, Default)]
pub struct StopRequest(Arc<AtomicBool>);

impl StopRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ask the runner to stop the job. Asking again changes nothing.
    pub fn request(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[must_use]
    pub fn requested(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// What the thread read.
pub(crate) struct Piped {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// Whether a stream carried a byte beyond its bound. Those bytes are
    /// dropped, and the bound is the length of the buffer.
    pub overflow: bool,
}

/// A stream the thread reads, and what it has kept of it.
struct Sink {
    file: Option<File>,
    kept: Vec<u8>,
}

impl Sink {
    fn new(fd: OwnedFd) -> std::io::Result<Self> {
        let file = File::from(fd);
        set_nonblocking(&file)?;
        Ok(Sink {
            file: Some(file),
            kept: Vec::new(),
        })
    }

    /// Read what the pipe holds now. Answers whether a byte went past `limit`.
    /// An end of file or an error closes the stream.
    fn read(&mut self, limit: usize) -> bool {
        let Some(file) = self.file.as_mut() else {
            return false;
        };
        let mut buffer = [0u8; CHUNK];
        let mut over = false;
        for _ in 0..CHUNKS_PER_PASS {
            match file.read(&mut buffer) {
                Ok(0) => {
                    self.file = None;
                    break;
                }
                Ok(count) => {
                    let room = limit.saturating_sub(self.kept.len()).min(count);
                    self.kept.extend_from_slice(&buffer[..room]);
                    over |= room < count;
                    if count < buffer.len() {
                        break;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => {
                    self.file = None;
                    break;
                }
            }
        }
        over
    }
}

pub(crate) struct Pipes {
    done: Arc<AtomicBool>,
    handle: JoinHandle<Piped>,
}

impl Pipes {
    /// Start the thread. `input` is written to `stdin` and the pipe is closed
    /// after the last byte. `overflow` is set at the first byte beyond `limit`
    /// on either stream.
    pub(crate) fn start(
        stdin: Option<(OwnedFd, Vec<u8>)>,
        stdout: OwnedFd,
        stderr: OwnedFd,
        limit: usize,
        overflow: StopRequest,
    ) -> std::io::Result<Self> {
        let mut input = match stdin {
            Some((fd, bytes)) => {
                let file = File::from(fd);
                set_nonblocking(&file)?;
                Some((file, bytes, 0usize))
            }
            None => None,
        };
        let mut out = Sink::new(stdout)?;
        let mut err = Sink::new(stderr)?;
        let done = Arc::new(AtomicBool::new(false));
        let stop = Arc::clone(&done);
        let handle = std::thread::Builder::new()
            .name("keyed-launch-pipes".to_owned())
            .spawn(move || {
                let mut over = false;
                loop {
                    let finished = stop.load(Ordering::SeqCst);
                    let mut fds: Vec<libc::pollfd> = Vec::with_capacity(3);
                    let mut roles = Vec::with_capacity(3);
                    for (role, sink) in [(0, &out), (1, &err)] {
                        if let Some(file) = &sink.file {
                            fds.push(libc::pollfd {
                                fd: file.as_raw_fd(),
                                events: libc::POLLIN,
                                revents: 0,
                            });
                            roles.push(role);
                        }
                    }
                    if let Some((file, _, _)) = &input {
                        fds.push(libc::pollfd {
                            fd: file.as_raw_fd(),
                            events: libc::POLLOUT,
                            revents: 0,
                        });
                        roles.push(2);
                    }
                    if fds.is_empty() {
                        break;
                    }
                    if !finished {
                        // SAFETY: `fds` is a live array of `fds.len()` entries.
                        // A signal that interrupts the wait only ends it early.
                        unsafe {
                            libc::poll(
                                fds.as_mut_ptr(),
                                fds.len() as libc::nfds_t,
                                TICK.as_millis() as libc::c_int,
                            );
                        }
                    }
                    // After `done` nothing is waited for: one more pass takes
                    // what the pipes hold, and the thread ends.
                    for role in roles {
                        match role {
                            0 => over |= out.read(limit),
                            1 => over |= err.read(limit),
                            _ => {
                                if let Some((file, bytes, at)) = input.as_mut() {
                                    let mut closed = false;
                                    while *at < bytes.len() {
                                        match file.write(&bytes[*at..]) {
                                            Ok(count) => *at += count,
                                            Err(error)
                                                if error.kind()
                                                    == std::io::ErrorKind::WouldBlock =>
                                            {
                                                break;
                                            }
                                            Err(error)
                                                if error.kind()
                                                    == std::io::ErrorKind::Interrupted => {}
                                            // A child that closed its input is
                                            // not an error of the job.
                                            Err(_) => {
                                                closed = true;
                                                break;
                                            }
                                        }
                                    }
                                    if closed || *at >= bytes.len() {
                                        input = None;
                                    }
                                }
                            }
                        }
                    }
                    if over {
                        overflow.request();
                    }
                    if finished {
                        break;
                    }
                }
                Piped {
                    stdout: out.kept,
                    stderr: err.kept,
                    overflow: over,
                }
            })?;
        Ok(Pipes { done, handle })
    }

    /// Tell the thread that the job is gone, and take what it read.
    pub(crate) fn finish(self) -> std::io::Result<Piped> {
        self.done.store(true, Ordering::SeqCst);
        self.handle
            .join()
            .map_err(|_| std::io::Error::other("the thread that reads the job's output panicked"))
    }
}

fn set_nonblocking(file: &File) -> std::io::Result<()> {
    let fd = file.as_raw_fd();
    // SAFETY: `fcntl(2)` on a descriptor that `file` owns.
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        if flags == -1 || libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) == -1 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}
