//! Private C4 controls. Reports are substituted only after physical operations.
//! Each terminal/signal fixture is a subprocess on its own PTY; no public switch.
use super::*;
use std::cell::Cell;
use std::os::unix::process::ExitStatusExt as _;

const ROLE: &str = "run::terminal_tests::terminal_recovery_role";
const QUERY: u8 = 1;
const HANDBACK: u8 = 2;
const MODES: u8 = 4;
const TTOU: u8 = 8;
thread_local! {
    static FAULTS: Cell<u8> = const { Cell::new(0) };
    static PROBE: Cell<Option<Probe>> = const { Cell::new(None) };
    static FOREIGN: Cell<i32> = const { Cell::new(0) };
    static CANCEL: Cell<bool> = const { Cell::new(false) };
    static PREPARED: Cell<bool> = const { Cell::new(false) };
    static DROP_REPORTS: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, Copy)]
pub(super) struct Probe {
    report: RawFd,
    release: RawFd,
    report_read: RawFd,
    release_write: RawFd,
    timeout: bool,
    before: bool,
    bad_record: bool,
    malformed: bool,
    failed_handoff: bool,
    signal_death: bool,
}

pub(super) fn probe() -> Option<Probe> {
    PROBE.get()
}
pub(super) fn record_fd(probe: Option<Probe>, fd: RawFd) -> RawFd {
    if probe.is_some_and(|p| p.bad_record) {
        -1
    } else {
        fd
    }
}
pub(super) fn record_group(probe: Option<Probe>, group: i32) -> i32 {
    if probe.is_some_and(|p| p.malformed) {
        -1
    } else {
        group
    }
}
pub(super) fn handoff_fd(probe: Option<Probe>, fd: RawFd) -> RawFd {
    if probe.is_some_and(|p| p.failed_handoff) {
        -1
    } else {
        fd
    }
}
pub(super) fn before_handoff(probe: Option<Probe>) -> std::io::Result<()> {
    if probe.is_some_and(|p| p.before) {
        Err(std::io::Error::from_raw_os_error(libc::EACCES))
    } else {
        Ok(())
    }
}

// Restricted child code: receives copied descriptors/flags, never accesses TLS.
pub(super) fn after_handoff(probe: Option<Probe>, tty: Option<RawFd>) -> std::io::Result<()> {
    let (Some(probe), Some(tty)) = (probe, tty) else {
        return Ok(());
    };
    // SAFETY: owned test-only pipe/terminal descriptors and initialized storage.
    unsafe {
        // A failed controller must close the child's last release writer too,
        // so EOF can wake the barrier before std waits the failed child.
        libc::close(probe.report_read);
        libc::close(probe.release_write);
        let mut modes: libc::termios = std::mem::zeroed();
        os_result(libc::tcgetattr(tty, &mut modes))?;
        modes.c_lflag &= !(libc::ECHO | libc::ICANON);
        os_result(libc::tcsetattr(tty, libc::TCSANOW, &modes))?;
        let record = [libc::getpid(), libc::tcgetpgrp(tty)];
        let bytes = std::slice::from_raw_parts(
            record.as_ptr().cast::<u8>(),
            std::mem::size_of_val(&record),
        );
        let mut written = 0;
        while written < bytes.len() {
            let n = libc::write(
                probe.report,
                bytes[written..].as_ptr().cast(),
                bytes.len() - written,
            );
            if n == -1 {
                let error = std::io::Error::last_os_error();
                if error.kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
            if n == 0 {
                return Err(std::io::Error::from_raw_os_error(libc::EIO));
            }
            written += n as usize;
        }
        let mut poll = libc::pollfd {
            fd: probe.release,
            events: libc::POLLIN,
            revents: 0,
        };
        loop {
            let result = libc::poll(&mut poll, 1, if probe.timeout { 200 } else { 5000 });
            if result > 0 {
                break;
            }
            if result == -1
                && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
            {
                continue;
            }
            return Err(std::io::Error::from_raw_os_error(libc::ETIMEDOUT));
        }
        let mut byte = 0u8;
        loop {
            let n = libc::read(probe.release, (&mut byte as *mut u8).cast(), 1);
            if n == 1 {
                break;
            }
            if n == -1 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
            {
                continue;
            }
            return Err(std::io::Error::from_raw_os_error(libc::EIO));
        }
        if probe.signal_death {
            let mut pending: libc::sigset_t = std::mem::zeroed();
            os_result(libc::sigpending(&mut pending))?;
            if libc::sigismember(&pending, libc::SIGINT) != 1 {
                return Err(std::io::Error::from_raw_os_error(libc::EIO));
            }
        }
    }
    Ok(())
}

pub(super) fn after_spawn() {
    let holder = FOREIGN.get();
    if holder > 0 {
        Terminal::open().unwrap().hand_to(holder).unwrap();
    }
    if CANCEL.get() {
        // Real HUP is consumed before recovery; TERM arrives during its reports.
        unsafe {
            libc::raise(libc::SIGHUP);
        }
    }
}

pub(super) fn reported(operation: &str, result: std::io::Result<()>) -> std::io::Result<()> {
    if CANCEL.get() && operation == "terminal handback" {
        unsafe {
            libc::raise(libc::SIGTERM);
        }
    }
    let bit = match operation {
        "terminal foreground query" => QUERY,
        "terminal handback" => HANDBACK,
        "terminal mode restoration" => MODES,
        "temporary SIGTTOU restoration" => TTOU,
        _ => 0,
    };
    if FAULTS.get() & bit != 0 && !DROP_REPORTS.get() {
        result.and(Err(std::io::Error::other(format!(
            "C4 substituted {operation} report after physical operation"
        ))))
    } else {
        result
    }
}

pub(super) fn prepared(command: &mut Command, attempt: Option<&AttemptGroup>) {
    PREPARED.set(attempt.is_some());
    let Some(attempt) = attempt else { return };
    let mut entries = Vec::new();
    let mut descriptors = vec![attempt.reader.as_raw_fd(), attempt.writer.as_raw_fd()];
    if let Some(probe) = PROBE.get() {
        descriptors.extend([
            probe.report,
            probe.release,
            probe.report_read,
            probe.release_write,
        ]);
    }
    for (index, fd) in descriptors.into_iter().enumerate() {
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        if unsafe { libc::fstat(fd, &mut stat) } == -1 {
            // No-controller paths already dropped their unused test endpoints.
            assert!(index >= 2);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EBADF)
            );
            continue;
        }
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        entries.push(format!("{fd}:{}:{}", stat.st_dev, stat.st_ino));
    }
    command.env("C4_EXPECT_CLOSED", entries.join(";"));
    if DROP_REPORTS.get() && std::env::var("C4_CASE").as_deref() == Ok("success-fds") {
        // Deliberate descriptor-leak control, confined to this test process.
        assert_eq!(
            unsafe { libc::fcntl(attempt.writer.as_raw_fd(), libc::F_SETFD, 0) },
            0
        );
    }
}

fn inspect_exec_descriptors() {
    let entries = std::env::var("C4_EXPECT_CLOSED").unwrap();
    assert_eq!(entries.split(';').count(), 6);
    for entry in entries.split(';') {
        let fields: Vec<_> = entry.split(':').collect();
        let fd: i32 = fields[0].parse().unwrap();
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        let result = unsafe { libc::fstat(fd, &mut stat) };
        assert!(
            result == -1
                || stat.st_dev.to_string() != fields[1]
                || stat.st_ino.to_string() != fields[2],
            "attempt pipe descriptor leaked into exec: {entry}"
        );
    }
    eprintln!(
        "C4 exec: both attempted-group and all four barrier pipe descriptors closed across exec"
    );
}

fn pipe() -> (File, File) {
    let mut fds = [-1; 2];
    assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
    for fd in fds {
        assert_eq!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            0
        );
    }
    unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
}

fn ready(fd: RawFd) -> bool {
    let mut poll = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    unsafe { libc::poll(&mut poll, 1, 5000) > 0 && poll.revents & libc::POLLIN != 0 }
}

struct FixtureChild(Option<Child>);
impl FixtureChild {
    fn id(&self) -> u32 {
        self.0.as_ref().unwrap().id()
    }
    fn finish(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            child.wait().unwrap();
        }
    }
}
impl Drop for FixtureChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn local_modes(terminal: &Terminal) -> libc::tcflag_t {
    terminal.attributes().unwrap().c_lflag & !libc::PENDIN
}

#[test]
#[ignore = "private subprocess fixture"]
fn terminal_recovery_role() {
    let Ok(case) = std::env::var("C4_CASE") else {
        return;
    };
    if std::env::var_os("C4_JOB").is_some() {
        inspect_exec_descriptors();
        return;
    }
    if case == "no-terminal" {
        assert!(Terminal::open().is_none());
        let argv = crate::Argv::new("/ravel-c4-missing-program".into(), vec![]);
        let error = run(Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: &[],
            transparent: None,
            cwd: None,
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::ZERO,
            },
        })
        .unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::ENOENT));
        assert!(!error.to_string().contains("terminal"));
        assert!(!PREPARED.get());
        return;
    }
    let root = std::path::PathBuf::from(std::env::var_os("C4_ROOT").unwrap());
    let terminal = Terminal::open().unwrap();
    let own = own_group();
    let saved = terminal.attributes().unwrap();
    let background = case == "background";
    let foreign = matches!(
        case.as_str(),
        "foreign" | "controller-failure" | "barrier-timeout" | "fixture-timeout"
    );
    let successful = case.starts_with("success");
    let signal_death = case == "signal-death";
    let before = case == "before";
    let bad_record = case == "bad-record";
    let malformed = case == "malformed";
    let failed_handoff = case == "handoff-failure";
    let pipe_failure = case == "pipe-failure";
    let faults = match case.as_str() {
        "query" => QUERY,
        "handback" => HANDBACK,
        "modes" => MODES,
        "ttou" => TTOU,
        "combined" | "cancel" | "success-fault" | "success-cancel-fault" => {
            QUERY | HANDBACK | MODES | TTOU
        }
        _ => 0,
    };
    // Query failure precludes handback/modes; combined action reports use all
    // three operations while the separate query case exercises that refusal.
    let faults = if faults.count_ones() > 1 {
        faults & !QUERY
    } else {
        faults
    };
    FAULTS.set(faults);
    DROP_REPORTS.set(std::env::var_os("C4_BROKEN").is_some());
    CANCEL.set(case == "cancel" || case == "success-cancel-fault");
    let mut holder = (foreign || background).then(|| {
        FixtureChild(Some(
            Command::new("/bin/sleep")
                .arg("30")
                .process_group(0)
                .spawn()
                .unwrap(),
        ))
    });
    if background {
        terminal
            .hand_to(holder.as_ref().unwrap().id() as i32)
            .unwrap();
    }
    if foreign {
        FOREIGN.set(holder.as_ref().unwrap().id() as i32);
    }
    let (mut receipt, receipt_writer) = pipe();
    let (release_reader, mut release) = pipe();
    PROBE.set(Some(Probe {
        report: receipt_writer.as_raw_fd(),
        release: release_reader.as_raw_fd(),
        report_read: receipt.as_raw_fd(),
        release_write: release.as_raw_fd(),
        timeout: case == "barrier-timeout",
        before,
        bad_record,
        malformed,
        failed_handoff,
        signal_death,
    }));
    if signal_death {
        unsafe {
            let mut mask: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut mask);
            libc::sigaddset(&mut mask, libc::SIGINT);
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, &mask, std::ptr::null_mut()),
                0
            );
        }
    }
    let observed = std::sync::Arc::new(std::sync::Mutex::new(None));
    let observed_writer = observed.clone();
    let expected_handoff = !(background || before || bad_record || failed_handoff || pipe_failure);
    if case == "controller-failure" {
        std::fs::write(root.join("fail-controller"), "private control").unwrap();
    }
    if case == "fixture-timeout" {
        std::fs::write(root.join("abort-controller"), "private control").unwrap();
    }
    if case == "barrier-timeout" {
        std::fs::write(root.join("timeout-barrier"), "private control").unwrap();
    }
    let controller = expected_handoff.then(|| {
        let root = root.clone();
        std::thread::spawn(move || {
            assert!(ready(receipt.as_raw_fd()), "handoff receipt missing");
            let mut record = [0; 8];
            receipt.read_exact(&mut record).unwrap();
            let child = i32::from_ne_bytes(record[..4].try_into().unwrap());
            let child_holder = i32::from_ne_bytes(record[4..].try_into().unwrap());
            let tty = Terminal::open().unwrap();
            let parent_holder = tty.foreground();
            assert_eq!(child, child_holder);
            assert_eq!(
                parent_holder, child,
                "parent observes actual handoff before exec"
            );
            assert_eq!(local_modes(&tty) & (libc::ECHO | libc::ICANON), 0);
            *observed_writer.lock().unwrap() = Some(child);
            std::fs::write(root.join("active-child"), child.to_string()).unwrap();
            if root.join("abort-controller").exists() {
                let until = Instant::now() + Duration::from_secs(5);
                while !root.join("abort").exists() && Instant::now() < until {
                    std::thread::sleep(Duration::from_millis(2));
                }
                assert!(root.join("abort").exists());
                return; // Closing the only release writer wakes the child with EOF.
            }
            if root.join("fail-controller").exists() {
                panic!("deliberate controller failure");
            }
            if root.join("timeout-barrier").exists() {
                std::thread::sleep(Duration::from_millis(400));
            }
            if signal_death {
                std::fs::write(root.join("handoff"), child.to_string()).unwrap();
                let until = Instant::now() + Duration::from_secs(5);
                while !root.join("release").exists() && Instant::now() < until {
                    std::thread::sleep(Duration::from_millis(2));
                }
                assert!(root.join("release").exists(), "terminal INT not delivered");
            }
            release.write_all(b"x").unwrap();
        })
    });
    let entry = EntrySignals::new([], []);
    let argv = if case == "success-fds" {
        crate::Argv::new(
            std::env::current_exe().unwrap().into_os_string(),
            vec![
                "--exact".into(),
                ROLE.into(),
                "--ignored".into(),
                "--nocapture".into(),
                "--test-threads=1".into(),
            ],
        )
    } else {
        crate::Argv::new(
            if successful {
                "/usr/bin/true"
            } else {
                "/ravel-c4-missing-program"
            }
            .into(),
            vec![],
        )
    };
    let grant = [(OsStr::new("C4_JOB"), OsStr::new("1"))];
    let mut events = Vec::new();
    let mut limits: libc::rlimit = unsafe { std::mem::zeroed() };
    if pipe_failure {
        assert_eq!(
            unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limits) },
            0
        );
        let next = File::open("/dev/null").unwrap();
        let restricted = libc::rlimit {
            rlim_cur: (next.as_raw_fd() + 2) as _,
            rlim_max: limits.rlim_max,
        };
        drop(next);
        assert_eq!(
            unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &restricted) },
            0
        );
    }
    let result = run_observed(
        Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: if case == "success-fds" { &grant } else { &[] },
            transparent: Some(&entry),
            cwd: None,
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::ZERO,
            },
        },
        &mut |event| events.push(event),
    );
    if pipe_failure {
        assert_eq!(unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &limits) }, 0);
    }
    let after = terminal.foreground();
    let foreign_group = holder.as_ref().map(|child| child.id() as i32);
    let modes = local_modes(&terminal);
    // Cleanup is independent of the reporting substitution and precedes assertions.
    FAULTS.set(0);
    FOREIGN.set(0);
    CANCEL.set(false);
    PROBE.set(None);
    terminal.hand_to(own).unwrap();
    terminal.restore(&saved).unwrap();
    if let Some(holder) = &mut holder {
        holder.finish();
    }
    if let Some(controller) = controller {
        let joined = controller.join();
        if case == "controller-failure" {
            assert!(joined.is_err());
        } else {
            joined.unwrap();
        }
    }
    assert_eq!(terminal.foreground(), own);
    assert_eq!(local_modes(&terminal), saved.c_lflag & !libc::PENDIN);
    let receipt = *observed.lock().unwrap();
    assert_eq!(PREPARED.get(), !(background || pipe_failure));
    if !DROP_REPORTS.get() {
        if let Some(destination) = std::env::var_os("C4_RECEIPTS") {
            let text = match &result {
                Err(error) => format!(
                    "errno={}\nsignal={}\nstarted={}\nerror={}\n",
                    error.raw_os_error().unwrap_or(0),
                    error.consumed_signal().unwrap_or(0),
                    events.contains(&LaunchEvent::Started),
                    error
                ),
                Ok(ended) => format!(
                    "status={}\nsignalled={}\ninterrupted={}\n",
                    ended.status.into_raw(),
                    ended.signalled,
                    matches!(ended.end, End::Interrupted { .. })
                ),
            };
            std::fs::write(
                std::path::Path::new(&destination).join(format!("{case}.txt")),
                text,
            )
            .unwrap();
        }
    }
    eprintln!("C4 {case}: own={own} parent_observed_handoff={receipt:?} after={after} modes_restored={} events={events:?} result={result:?}", modes == saved.c_lflag & !libc::PENDIN);
    assert_eq!(receipt.is_some(), expected_handoff);
    if foreign || background {
        assert_eq!(
            Some(after),
            foreign_group,
            "unrelated holder must be preserved exactly"
        );
    }
    if let Some(child) = receipt {
        let mut status = 0;
        assert_eq!(
            unsafe { libc::waitpid(child, &mut status, libc::WNOHANG) },
            -1
        );
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
        eprintln!("C4 {case}: recorded direct child is no longer waitable (ECHILD); runner events remain {events:?}");
    }
    if successful || signal_death {
        assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);
    } else {
        assert!(
            events.is_empty(),
            "std's failed-child wait is not a runner event"
        );
    }
    // All direct-child waits and uninjected terminal recovery above have completed.
    std::fs::write(
        root.join("cleanup-confirmed"),
        "foreground/modes restored; actual children waited",
    )
    .unwrap();
    if signal_death {
        let ended = result.unwrap();
        assert_eq!(ended.status.signal(), Some(libc::SIGINT));
        assert_eq!(ended.end, End::Exited);
        assert_eq!(ended.group, Group::Gone);
        assert!(!ended.signalled);
    } else if successful && faults == 0 {
        assert!(result.unwrap().status.success());
    } else {
        let error = result.unwrap_err();
        if !successful {
            assert_eq!(
                error.raw_os_error(),
                if pipe_failure {
                    None
                } else {
                    Some(if before {
                        libc::EACCES
                    } else if bad_record || failed_handoff {
                        libc::EBADF
                    } else if case == "controller-failure" || case == "fixture-timeout" {
                        libc::EIO
                    } else if case == "barrier-timeout" {
                        libc::ETIMEDOUT
                    } else {
                        libc::ENOENT
                    })
                }
            );
        }
        assert_eq!(
            error.consumed_signal(),
            if case == "cancel" || case == "success-cancel-fault" {
                Some(libc::SIGHUP)
            } else if case == "fixture-timeout" {
                Some(libc::SIGTERM)
            } else {
                None
            }
        );
        let text = error.to_string();
        for (bit, needle) in [
            (QUERY, "terminal foreground query"),
            (HANDBACK, "terminal handback"),
            (MODES, "terminal mode restoration"),
            (TTOU, "temporary SIGTTOU restoration"),
        ] {
            if faults & bit != 0 {
                assert!(text.contains(needle), "missing {needle}: {text}");
            }
        }
        if foreign {
            assert!(text.contains("unrelated foreground"));
        }
        if pipe_failure {
            assert!(text.contains("cannot prepare terminal attempted-group pipe"));
        }
        if before || bad_record || malformed {
            assert!(text.contains("record"));
        }
    }
    if faults & QUERY != 0 || foreign || background || malformed {
        assert_ne!(after, own);
        if background {
            assert_eq!(modes, saved.c_lflag & !libc::PENDIN);
        }
    } else {
        assert_eq!(after, own, "failed exec must reclaim launcher foreground");
        assert_eq!(modes, saved.c_lflag & !libc::PENDIN);
    }
}

// Catch loss of the lease on spawn error with an actual pre-exec parent receipt.
#[test]
fn terminal_recovery_failed_exec_restores_foreground() {
    pty_case("missing", false);
}

#[test]
fn terminal_recovery_reports_and_controls() {
    for case in [
        "before",
        "bad-record",
        "malformed",
        "handoff-failure",
        "pipe-failure",
        "background",
        "foreign",
        "controller-failure",
        "barrier-timeout",
        "fixture-timeout",
        "query",
        "handback",
        "modes",
        "ttou",
        "combined",
        "cancel",
        "success",
        "success-fault",
        "success-cancel-fault",
        "success-fds",
        "signal-death",
    ] {
        pty_case(case, false);
    }
}

// Explicit broken control: discard each injected failure. Every assertion must
// reject it; expectations come from the spec, not from the reporting helper.
#[test]
fn terminal_recovery_controls_detect_discarded_reports() {
    for case in [
        "query",
        "handback",
        "modes",
        "ttou",
        "combined",
        "cancel",
        "success-fault",
        "success-fds",
    ] {
        pty_case(case, true);
    }
}

fn pty_case(case: &str, broken: bool) {
    let root = tempfile::tempdir().unwrap();
    let (mut master, mut slave) = (-1, -1);
    assert_eq!(
        unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        },
        0
    );
    let (mut master, slave) = unsafe { (File::from_raw_fd(master), File::from_raw_fd(slave)) };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        assert_eq!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            0
        );
    }
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            ROLE,
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("C4_CASE", case)
        .env("C4_ROOT", root.path())
        .stdin(slave.try_clone().unwrap())
        .stdout(slave.try_clone().unwrap())
        .stderr(slave);
    if broken {
        command.env("C4_BROKEN", "1");
    }
    // SAFETY: only child-safe syscalls, establishing this fixture's own PTY.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 || libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command.spawn().unwrap();
    drop(command);
    let flags = unsafe { libc::fcntl(master.as_raw_fd(), libc::F_GETFL) };
    assert_eq!(
        unsafe { libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) },
        0
    );
    let until = Instant::now() + Duration::from_secs(12);
    let mut output = Vec::new();
    let mut interrupted = false;
    let mut abort_requested = None;
    let status = loop {
        drain(&mut master, &mut output);
        if case == "signal-death" && !interrupted && root.path().join("handoff").exists() {
            master.write_all(&[3]).unwrap(); // PTY line discipline generates foreground INT.
            std::fs::write(root.path().join("release"), "terminal INT written").unwrap();
            interrupted = true;
        }
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        let now = Instant::now();
        if abort_requested.is_none()
            && (now >= until
                || case == "fixture-timeout" && root.path().join("active-child").exists())
        {
            std::fs::write(
                root.path().join("abort"),
                "request cooperative fixture cleanup",
            )
            .unwrap();
            // The fixture retains actual child handles/receipt and completes their
            // waits. Killing only its group would strand separately grouped children.
            unsafe {
                libc::kill(child.id() as i32, libc::SIGTERM);
            }
            abort_requested = Some(now);
        }
        if abort_requested
            .is_some_and(|requested| now.duration_since(requested) > Duration::from_secs(6))
        {
            let _ = child.kill();
            let _ = child.wait();
            panic!(
                "C4 fixture cleanup unconfirmed after cooperative abort: {}",
                String::from_utf8_lossy(&output)
            );
        }
        std::thread::sleep(Duration::from_millis(2));
    };
    drain(&mut master, &mut output);
    eprintln!(
        "C4 case={case} broken={broken}: {}",
        String::from_utf8_lossy(&output)
    );
    if case == "fixture-timeout" {
        assert!(abort_requested.is_some());
        assert!(
            root.path().join("cleanup-confirmed").exists(),
            "timeout did not confirm fixture child waits and terminal recovery"
        );
    } else if abort_requested.is_some() {
        panic!(
            "C4 deadline expired, cleanup={}: {}",
            root.path().join("cleanup-confirmed").exists(),
            String::from_utf8_lossy(&output)
        );
    }
    if broken {
        assert!(
            !status.success() && String::from_utf8_lossy(&output).contains("panicked at"),
            "broken control was not detected: {case}: {status:?}"
        );
    } else {
        assert!(
            status.success(),
            "C4 fixture failed: {status:?}: {}",
            String::from_utf8_lossy(&output)
        );
    }
}

fn drain(master: &mut File, output: &mut Vec<u8>) {
    let mut buffer = [0; 4096];
    while let Ok(n) = master.read(&mut buffer) {
        if n == 0 {
            break;
        }
        output.extend_from_slice(&buffer[..n]);
    }
}

#[test]
fn terminal_recovery_without_terminal_has_no_lease() {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", ROLE, "--ignored", "--test-threads=1"])
        .env("C4_CASE", "no-terminal")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// Reject absent, partial, oversized and invalid IDs without waiting for EOF.
#[test]
fn terminal_recovery_record_validation_is_bounded() {
    for bytes in [
        vec![],
        vec![1],
        vec![1, 2, 3],
        vec![0; 4],
        (-1i32).to_ne_bytes().to_vec(),
        vec![1, 2, 3, 4, 5],
    ] {
        let attempt = AttemptGroup::prepare().unwrap();
        if !bytes.is_empty() {
            assert_eq!(
                unsafe {
                    libc::write(
                        attempt.writer.as_raw_fd(),
                        bytes.as_ptr().cast(),
                        bytes.len(),
                    )
                },
                bytes.len() as isize
            );
        }
        let start = Instant::now();
        assert!(
            attempt.receive().is_err(),
            "accepted malformed record {bytes:?}"
        );
        assert!(start.elapsed() < Duration::from_secs(1));
    }
    let attempt = AttemptGroup::prepare().unwrap();
    record_attempt(attempt.writer.as_raw_fd(), 12345).unwrap();
    assert_eq!(attempt.receive().unwrap(), 12345);
}

#[test]
fn terminal_recovery_retains_lending_query_failure() {
    let terminal = Terminal(File::open("/dev/null").unwrap().into());
    let mut lease = Lease {
        terminal: Some(terminal),
        child: 12345,
        held: true,
        saved: None,
        reports: None,
    };
    lease.lend();
    let error = lease.reclaim(None, false).unwrap_err().to_string();
    assert!(
        error.contains("while lending"),
        "lost earlier physical foreground-query failure: {error}"
    );
    assert!(error.contains("terminal foreground query failed"));
}
