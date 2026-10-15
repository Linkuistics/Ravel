//! Owner-local acknowledged barrier and bounded case controller for the native
//! entry campaign. Compiled only into unit-test builds (`cfg(test)`); the ravel
//! crate includes this same file by `#[path]` so both crates' fixtures share one
//! controller. There is no installed switch: the fixture side acts only when the
//! controller's private directory variable is present in a test process.
//!
//! The exchange is two-way and ordered by acknowledgements, never by sleeps:
//!
//! 1. the fixture writes `READY <boundary> <facts>` and then waits;
//! 2. the controller acts (delivers a signal, selects nothing else) and sends
//!    `P`; the fixture answers `ACK <boundary> <facts-now>`;
//! 3. the controller sends `R`; the fixture answers `RELEASED <boundary>` and
//!    continues.
//!
//! Every wait has an absolute deadline: a case has 30 seconds in all, each
//! acknowledgement 5 seconds, and the last 5 seconds are reserved for owned
//! cleanup. Polling slices below only bound how often a deadline is examined.
#![allow(dead_code)]

use std::fs::File;
use std::io::{self, Read as _, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _, RawFd};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::time::{Duration, Instant};

/// Names the private barrier directory for a fixture process.
pub const DIRECTORY_VARIABLE: &str = "K28_BARRIER_DIR";

const READY: &str = "ready.fifo";
const RELEASE: &str = "release.fifo";
const SLICE: Duration = Duration::from_millis(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub outer: Duration,
    pub ack: Duration,
    pub cleanup: Duration,
}
impl Limits {
    /// The frozen envelope: 30 seconds per case, 5 per acknowledgement, and the
    /// final 5 seconds reserved for cleanup inside the same outer deadline.
    pub const FROZEN: Limits = Limits {
        outer: Duration::from_secs(30),
        ack: Duration::from_secs(5),
        cleanup: Duration::from_secs(5),
    };
}

fn open(path: &Path, flags: i32) -> io::Result<File> {
    let name = std::ffi::CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains NUL"))?;
    // SAFETY: a valid NUL-terminated path; the descriptor is owned immediately.
    let fd = unsafe { libc::open(name.as_ptr(), flags | libc::O_CLOEXEC) };
    if fd == -1 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `fd` is a fresh descriptor not owned elsewhere.
    Ok(unsafe { File::from_raw_fd(fd) })
}

/// Wait for readability until an absolute deadline; EINTR never restarts it.
fn readable(fd: RawFd, by: Instant) -> io::Result<bool> {
    loop {
        let remaining = by.saturating_duration_since(Instant::now());
        let mut poll = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let milliseconds = i32::try_from(remaining.as_millis()).unwrap_or(i32::MAX);
        // SAFETY: one initialised pollfd.
        match unsafe { libc::poll(&mut poll, 1, milliseconds) } {
            -1 => {
                let error = io::Error::last_os_error();
                if error.kind() != io::ErrorKind::Interrupted {
                    return Err(error);
                }
                if Instant::now() >= by {
                    return Ok(false);
                }
            }
            0 => return Ok(false),
            _ => return Ok(true),
        }
    }
}

/// Fixture side. A no-op returning `Ok` when no controller owns this process.
///
/// `facts` is evaluated fresh at each probe, so a signal delivered between the
/// ready record and the probe is visible in the acknowledgement.
pub fn exchange(boundary: &str, facts: &mut dyn FnMut() -> String) -> Result<(), String> {
    let Some(directory) = std::env::var_os(DIRECTORY_VARIABLE).map(PathBuf::from) else {
        return Ok(());
    };
    let mut ready = open(&directory.join(READY), libc::O_WRONLY | libc::O_NONBLOCK)
        .map_err(|error| format!("barrier ready channel: {error}"))?;
    let mut release = open(&directory.join(RELEASE), libc::O_RDONLY | libc::O_NONBLOCK)
        .map_err(|error| format!("barrier release channel: {error}"))?;
    let say = |file: &mut File, line: String| {
        file.write_all(format!("{line}\n").as_bytes())
            .map_err(|error| format!("barrier write: {error}"))
    };
    say(&mut ready, format!("READY {boundary} {}", facts()))?;
    let ack = Limits::FROZEN.ack;
    loop {
        let by = Instant::now() + ack;
        let mut command = [0u8; 1];
        loop {
            match readable(release.as_raw_fd(), by) {
                Ok(true) => match release.read(&mut command) {
                    Ok(1) => break,
                    Ok(_) => {}
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                    Err(error) => return Err(format!("barrier command read: {error}")),
                },
                Ok(false) => {
                    let _ = say(&mut ready, format!("TIMEOUT {boundary}"));
                    return Err(format!("controller silent at {boundary} for {ack:?}"));
                }
                Err(error) => return Err(format!("barrier command wait: {error}")),
            }
        }
        match command[0] {
            b'P' => say(&mut ready, format!("ACK {boundary} {}", facts()))?,
            b'R' => {
                say(&mut ready, format!("RELEASED {boundary}"))?;
                return Ok(());
            }
            other => return Err(format!("barrier command {other:#x} is not defined")),
        }
    }
}

/// How the controller's wait for a record ended, other than with a record.
#[derive(Debug)]
pub enum Wait {
    Timeout,
    Exited(ExitStatus),
    Io(io::Error),
}
impl std::fmt::Display for Wait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wait::Timeout => f.write_str("deadline expired"),
            Wait::Exited(status) => write!(f, "fixture exited first: {status}"),
            Wait::Io(error) => write!(f, "controller i/o: {error}"),
        }
    }
}

/// The controller's final, ordered account of one case.
#[derive(Debug)]
pub struct Settlement {
    /// The fixture's own actual exit, if the controller observed one.
    pub status: Option<ExitStatus>,
    pub watchdog_expired: bool,
    pub stopped_by_controller: bool,
    /// Identities whose reap the controller could not confirm.
    pub unresolved: Vec<String>,
    /// Controller-owned escaped holders: pid and whether its reap is confirmed.
    pub holders: Vec<(u32, bool)>,
    /// Barrier faults: acknowledgement or release that never came.
    pub faults: Vec<String>,
    pub elapsed: Duration,
}
impl Settlement {
    /// A timeout, an unconfirmed identity or a failed acknowledgement is a
    /// failed receipt. Nothing ambiguous counts as passing.
    pub fn clean(&self) -> bool {
        !self.watchdog_expired
            && self.unresolved.is_empty()
            && self.faults.is_empty()
            && self.status.is_some()
            && self.elapsed <= Limits::FROZEN.outer
    }
}

/// Owns one fixture process, its escaped holders and the barrier channels.
pub struct Controller {
    limits: Limits,
    started: Instant,
    directory: PathBuf,
    ready: File,
    release: File,
    pending: Vec<u8>,
    child: Option<Child>,
    status: Option<ExitStatus>,
    holders: Vec<Child>,
    holder_reaped: Vec<(u32, bool)>,
    faults: Vec<String>,
    stopped: bool,
    expired: bool,
    unresolved: Vec<String>,
}

impl Controller {
    pub fn new(directory: &Path, limits: Limits) -> io::Result<Self> {
        for name in [READY, RELEASE] {
            let path = std::ffi::CString::new(directory.join(name).as_os_str().as_bytes())
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains NUL"))?;
            // SAFETY: a valid NUL-terminated path.
            if unsafe { libc::mkfifo(path.as_ptr(), 0o600) } == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        // The controller holds the release side read-write so a fixture's open
        // never blocks and a write never raises SIGPIPE.
        let ready = open(&directory.join(READY), libc::O_RDONLY | libc::O_NONBLOCK)?;
        let release = open(&directory.join(RELEASE), libc::O_RDWR | libc::O_NONBLOCK)?;
        Ok(Self {
            limits,
            started: Instant::now(),
            directory: directory.to_owned(),
            ready,
            release,
            pending: Vec::new(),
            child: None,
            status: None,
            holders: Vec::new(),
            holder_reaped: Vec::new(),
            faults: Vec::new(),
            stopped: false,
            expired: false,
            unresolved: Vec::new(),
        })
    }

    /// Name the barrier directory for a command about to be spawned.
    pub fn configure(&self, command: &mut Command) {
        command.env(DIRECTORY_VARIABLE, &self.directory);
    }

    pub fn adopt(&mut self, child: Child) {
        self.child = Some(child);
    }

    /// An escaped pipe-holder or similar fixture child this controller owns.
    pub fn adopt_holder(&mut self, child: Child) {
        self.holders.push(child);
    }

    pub fn pid(&self) -> Option<u32> {
        self.child.as_ref().map(Child::id)
    }

    /// The point past which no new work begins; cleanup owns what follows.
    fn work_by(&self) -> Instant {
        self.started + self.limits.outer - self.limits.cleanup
    }
    fn outer_by(&self) -> Instant {
        self.started + self.limits.outer
    }

    fn line(&mut self, by: Instant) -> Result<String, Wait> {
        loop {
            if let Some(end) = self.pending.iter().position(|&byte| byte == b'\n') {
                let line: Vec<u8> = self.pending.drain(..=end).collect();
                return Ok(String::from_utf8_lossy(&line[..line.len() - 1]).into_owned());
            }
            let mut chunk = [0u8; 4096];
            match self.ready.read(&mut chunk) {
                Ok(0) => {}
                Ok(count) => {
                    self.pending.extend_from_slice(&chunk[..count]);
                    continue;
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => return Err(Wait::Io(error)),
            }
            if let Some(child) = self.child.as_mut() {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        self.status = Some(status);
                        // Drain what the fixture wrote before it exited.
                        match self.ready.read(&mut chunk) {
                            Ok(count) if count > 0 => {
                                self.pending.extend_from_slice(&chunk[..count]);
                                continue;
                            }
                            _ => return Err(Wait::Exited(status)),
                        }
                    }
                    Ok(None) => {}
                    Err(error) => return Err(Wait::Io(error)),
                }
            }
            let now = Instant::now();
            if now >= by {
                return Err(Wait::Timeout);
            }
            let slice = SLICE.min(by - now);
            readable(self.ready.as_raw_fd(), Instant::now() + slice).map_err(Wait::Io)?;
        }
    }

    fn expect(&mut self, kind: &str, by: Instant) -> Result<String, String> {
        match self.line(by) {
            Ok(line) => match line.split_once(' ') {
                Some((word, rest)) if word == kind => Ok(rest.to_owned()),
                _ => {
                    let fault = format!("expected {kind}, observed {line:?}");
                    self.faults.push(fault.clone());
                    Err(fault)
                }
            },
            Err(wait) => {
                let fault = format!("no {kind} record: {wait}");
                self.faults.push(fault.clone());
                Err(fault)
            }
        }
    }

    /// Wait for the fixture's ready record, bounded by the case's work deadline.
    /// Returns `<boundary> <facts>`.
    pub fn ready(&mut self) -> Result<String, String> {
        let by = self.work_by();
        self.expect("READY", by)
    }

    /// Ask for the fixture's current facts; bounded by the acknowledgement limit.
    pub fn probe(&mut self) -> Result<String, String> {
        self.command(b'P')?;
        let by = (Instant::now() + self.limits.ack).min(self.outer_by());
        self.expect("ACK", by)
    }

    /// Release the fixture and require its acknowledgement that it continued.
    pub fn release(&mut self) -> Result<(), String> {
        self.command(b'R')?;
        let by = (Instant::now() + self.limits.ack).min(self.outer_by());
        self.expect("RELEASED", by).map(|_| ())
    }

    fn command(&mut self, byte: u8) -> Result<(), String> {
        self.release.write_all(&[byte]).map_err(|error| {
            let fault = format!("barrier command not delivered: {error}");
            self.faults.push(fault.clone());
            fault
        })
    }

    /// Deliver a signal to the owned, still-unreaped fixture process.
    pub fn signal(&mut self, signal: i32) -> Result<(), String> {
        let Some(child) = self.child.as_mut() else {
            return Err("no fixture is owned".to_owned());
        };
        match child.try_wait() {
            Ok(None) => {}
            Ok(Some(status)) => {
                self.status = Some(status);
                return Err(format!("fixture already ended: {status}"));
            }
            Err(error) => return Err(format!("fixture state unknown: {error}")),
        }
        // SAFETY: the child is unreaped, so the identity is still reserved.
        if unsafe { libc::kill(child.id() as i32, signal) } == -1 {
            return Err(format!("signal {signal}: {}", io::Error::last_os_error()));
        }
        Ok(())
    }

    fn reap_by(child: &mut Child, by: Instant) -> Option<ExitStatus> {
        loop {
            match child.try_wait() {
                Ok(Some(status)) => return Some(status),
                Ok(None) => {}
                Err(_) => return None,
            }
            if Instant::now() >= by {
                return None;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// Wait for the fixture's natural end within the work deadline, then settle
    /// whatever the controller owns. Never signals a reaped identity and never
    /// extends the outer deadline.
    pub fn finish(&mut self) -> Settlement {
        self.settle(true)
    }

    /// Stop everything the controller owns now. A failed case does not wait out
    /// its work deadline; the identities are still unreaped, so this is safe.
    pub fn abort(&mut self) -> Settlement {
        self.settle(false)
    }

    fn settle(&mut self, natural: bool) -> Settlement {
        let work_by = if natural {
            self.work_by()
        } else {
            Instant::now()
        };
        let outer_by = self.outer_by();
        if let Some(child) = self.child.as_mut() {
            if self.status.is_none() {
                match Self::reap_by(child, work_by) {
                    Some(status) => self.status = Some(status),
                    None => {
                        self.expired = true;
                        // The unreaped direct child still reserves its identity
                        // and its group's: group first, then the child itself.
                        let pid = child.id() as i32;
                        // SAFETY: unreaped child; identity reserved.
                        unsafe {
                            libc::kill(-pid, libc::SIGKILL);
                            libc::kill(pid, libc::SIGKILL);
                        }
                        self.stopped = true;
                        match Self::reap_by(child, outer_by) {
                            Some(status) => self.status = Some(status),
                            None => self.unresolved.push(format!("fixture pid {pid}")),
                        }
                    }
                }
            }
        }
        for holder in &mut self.holders {
            let pid = holder.id();
            if self.holder_reaped.iter().any(|&(p, _)| p == pid) {
                continue;
            }
            let reaped = match holder.try_wait() {
                Ok(Some(_)) => true,
                Ok(None) => {
                    let pid = pid as i32;
                    // SAFETY: an unreaped, controller-owned child.
                    unsafe {
                        libc::kill(-pid, libc::SIGKILL);
                        libc::kill(pid, libc::SIGKILL);
                    }
                    self.stopped = true;
                    Self::reap_by(holder, outer_by).is_some()
                }
                Err(_) => false,
            };
            self.holder_reaped.push((holder.id(), reaped));
            if !reaped {
                self.unresolved.push(format!("holder pid {}", holder.id()));
            }
        }
        Settlement {
            status: self.status,
            watchdog_expired: self.expired,
            stopped_by_controller: self.stopped,
            unresolved: self.unresolved.clone(),
            holders: self.holder_reaped.clone(),
            faults: self.faults.clone(),
            elapsed: self.started.elapsed(),
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        // Never leave an owned, unreaped identity behind a failed assertion.
        if self.child.is_some() || !self.holders.is_empty() {
            // A dropped controller means a failed or abandoned case: stop what
            // it owns now rather than waiting out the work deadline.
            let _ = self.settle(false);
        }
    }
}
