//! The job contract at its process boundary: the terminal a launch hands over
//! and takes back, and the entry signal state it respects.
//!
//! **Every launch here runs in a launcher process of its own**, because both
//! subjects are process-global. A controlling terminal belongs to a session,
//! and a test can make one only by starting a session leader on a fresh
//! pseudo-terminal. A signal disposition belongs to the process, and a
//! launcher that ignores SIGHUP or SIGCHLD would change every other test in a
//! shared binary. So the test re-runs its own binary in a fixture role, as
//! `noninteractive.rs` does. The `#[ignore]`d functions below are those roles,
//! each answering only to its `JOB_ROLE`. Each test drives one and reads back
//! the report it writes.

mod support;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::Read as _;
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use keyed_launch::{run, Argv, Channel, End, EntrySignals, Escalation, Group, Launch};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Roles

fn role() -> Option<String> {
    std::env::var("JOB_ROLE").ok()
}

fn var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("the fixture needs {name}"))
}

/// This process's controlling terminal, if it has one.
fn tty() -> Option<OwnedFd> {
    // SAFETY: `open(2)` on a constant path; the descriptor is owned from here.
    let fd = unsafe { libc::open(c"/dev/tty".as_ptr(), libc::O_RDWR | libc::O_NOCTTY) };
    // SAFETY: a fresh descriptor this process owns.
    (fd >= 0).then(|| unsafe { OwnedFd::from_raw_fd(fd) })
}

fn foreground(tty: &OwnedFd) -> libc::pid_t {
    // SAFETY: `tcgetpgrp(3)` on an owned descriptor.
    unsafe { libc::tcgetpgrp(tty.as_raw_fd()) }
}

/// The terminal's local modes, which hold ICANON and ECHO, less PENDIN.
///
/// PENDIN is the driver's own transient state, not a mode anyone sets. BSD sets
/// it when a terminal goes from raw back to canonical with input queued, which
/// is exactly what a restore does, and clears it on the next read. It is
/// measured to appear after the restore on macOS.
fn local_modes(tty: &OwnedFd) -> u64 {
    // SAFETY: `tcgetattr(3)` filling an initialised struct.
    unsafe {
        let mut attributes: libc::termios = std::mem::zeroed();
        assert_eq!(libc::tcgetattr(tty.as_raw_fd(), &mut attributes), 0);
        #[allow(clippy::useless_conversion)]
        u64::from(attributes.c_lflag & !libc::PENDIN)
    }
}

fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` cannot fail.
    unsafe { libc::getpgrp() }
}

/// A disposition by name, read without changing it.
fn disposition(signal: libc::c_int) -> &'static str {
    // SAFETY: `sigaction(2)` with a null new action only reads.
    let handler = unsafe {
        let mut current: libc::sigaction = std::mem::zeroed();
        libc::sigaction(signal, std::ptr::null(), &mut current);
        current.sa_sigaction
    };
    match handler {
        libc::SIG_IGN => "ignored",
        libc::SIG_DFL => "default",
        _ => "handled",
    }
}

/// Whether this thread has `signal` blocked.
fn blocked(signal: libc::c_int) -> bool {
    // SAFETY: a null new set only reads the current mask into an initialised
    // set.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &mut mask);
        libc::sigismember(&mask, signal) == 1
    }
}

/// Write `lines` to `path` whole, so a reader polling for it never sees half.
fn publish(path: &Path, lines: &[String]) {
    let partial = path.with_extension("partial");
    fs::write(&partial, lines.join("\n") + "\n").unwrap();
    fs::rename(partial, path).unwrap();
}

/// The launcher under test: one `run`, configured by its environment, then a
/// report of what it saw before and after.
///
/// - `JOB_IGNORE`: comma-separated signals to ignore before the launch, from
///   `hup`, `chld` and `usr2` — the launcher's entry state.
/// - `JOB_TRANSPARENT`: when set, the launch is transparent, handing the child
///   an entry state with SIGHUP ignored and SIGUSR1 blocked, which is not the
///   launcher's own.
/// - `JOB_CHILD`: `reporter` for the reporter role, otherwise a path to a
///   shell script.
/// - `JOB_REPORT`: where the report goes.
#[test]
#[ignore = "subprocess fixture"]
fn launcher() {
    if role().as_deref() != Some("launcher") {
        return;
    }
    for name in std::env::var("JOB_IGNORE").unwrap_or_default().split(',') {
        let signal = match name {
            "" => continue,
            "hup" => libc::SIGHUP,
            "chld" => libc::SIGCHLD,
            "usr2" => libc::SIGUSR2,
            other => panic!("unknown signal {other:?}"),
        };
        // SAFETY: setting this fixture process's own entry state.
        unsafe { libc::signal(signal, libc::SIG_IGN) };
    }

    let tty = tty();
    let mut report = vec![format!("group={}", own_group())];
    if let Some(tty) = &tty {
        report.push(format!("before_foreground={}", foreground(tty)));
        report.push(format!("before_modes={}", local_modes(tty)));
    }

    let child = var("JOB_CHILD");
    let argv = if child == "reporter" {
        Argv::new(
            "env".into(),
            vec![
                "JOB_ROLE=reporter".into(),
                std::env::current_exe().unwrap().into_os_string(),
                "--exact".into(),
                "reporter".into(),
                "--ignored".into(),
                "--nocapture".into(),
            ],
        )
    } else {
        Argv::new("sh".into(), vec![OsString::from(child)])
    };
    let control = TempDir::new().unwrap();
    let channel = Channel::allocate(control.path()).unwrap();
    let entry = EntrySignals::new([libc::SIGHUP], [libc::SIGUSR1]);
    let ended = run(Launch {
        argv: &argv,
        channel: Some((&channel, "TEST_CHANNEL")),
        scrub: support::controls(),
        grant: &[],
        transparent: std::env::var_os("JOB_TRANSPARENT").map(|_| &entry),
        cwd: None,
        escalation: Escalation {
            grace: Duration::from_millis(600),
            kill_grace: Duration::from_millis(900),
        },
    })
    .unwrap();

    report.push(format!(
        "end={}",
        match ended.end {
            End::Exited => "exited".to_owned(),
            End::Escalated => "escalated".to_owned(),
            End::Interrupted { signal } => format!("interrupted:{signal}"),
        }
    ));
    if let Some(code) = ended.status.code() {
        report.push(format!("code={code}"));
    }
    if let Some(signal) = ended.status.signal() {
        report.push(format!("signal={signal}"));
    }
    report.push(format!(
        "child_group={}",
        match ended.group {
            Group::Gone => "gone",
            Group::Present { .. } => "present",
        }
    ));
    if let Some(tty) = &tty {
        report.push(format!("after_foreground={}", foreground(tty)));
        report.push(format!("after_modes={}", local_modes(tty)));
    }
    report.push(format!("hup={}", disposition(libc::SIGHUP)));
    report.push(format!("term={}", disposition(libc::SIGTERM)));
    report.push(format!("chld={}", disposition(libc::SIGCHLD)));
    publish(Path::new(&var("JOB_REPORT")), &report);
}

/// A child that reports what it inherited, optionally leaves a TERM-ignoring
/// descendant in its group, and exits with `REPORTER_EXIT`.
#[test]
#[ignore = "subprocess fixture"]
fn reporter() {
    if role().as_deref() != Some("reporter") {
        return;
    }
    let mut report = vec![
        format!("group={}", own_group()),
        format!("chld={}", disposition(libc::SIGCHLD)),
        format!("hup={}", disposition(libc::SIGHUP)),
        format!("usr2={}", disposition(libc::SIGUSR2)),
        format!("int={}", disposition(libc::SIGINT)),
        format!("usr1_blocked={}", blocked(libc::SIGUSR1)),
    ];
    if let Some(tty) = tty() {
        report.push(format!("foreground={}", foreground(&tty)));
    }
    if std::env::var_os("REPORTER_DESCENDANT").is_some() {
        // Never waited for, by design: it is the survivor the launcher has to
        // end, and this process exits without it.
        #[allow(clippy::zombie_processes)]
        let descendant = support::command("sh")
            .args(["-c", "trap '' TERM ; while : ; do sleep 0.05 ; done"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        report.push(format!("descendant={}", descendant.id()));
    }
    publish(Path::new(&var("REPORTER_OUT")), &report);
    std::process::exit(
        std::env::var("REPORTER_EXIT")
            .ok()
            .and_then(|code| code.parse().ok())
            .unwrap_or(0),
    );
}

/// A process that leads a group of its own, takes the terminal for it, says
/// so in `GRABBER_READY`, and waits to be killed. It stands for any group other
/// than the child's that holds the terminal when the child ends: a nested
/// job-control shell's job, or a supervisor's orphaned child.
#[test]
#[ignore = "subprocess fixture"]
fn grabber() {
    if role().as_deref() != Some("grabber") {
        return;
    }
    let tty = tty().expect("the grabber needs the controlling terminal");
    // SAFETY: making this process a group leader, then taking the terminal
    // for that group with SIGTTOU ignored, as any job-control shell does.
    unsafe {
        libc::setpgid(0, 0);
        libc::signal(libc::SIGTTOU, libc::SIG_IGN);
        assert_eq!(libc::tcsetpgrp(tty.as_raw_fd(), libc::getpid()), 0);
    }
    publish(
        Path::new(&var("GRABBER_READY")),
        &[format!("{}", own_group())],
    );
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// The session leader for a launcher started in the background: it holds the
/// terminal's foreground itself, runs the launcher in a group of its own, and
/// reports who holds the foreground afterwards.
#[test]
#[ignore = "subprocess fixture"]
fn background_session() {
    if role().as_deref() != Some("background_session") {
        return;
    }
    let status = support::command(std::env::current_exe().unwrap())
        .args(["--exact", "launcher", "--ignored", "--nocapture"])
        .env("JOB_ROLE", "launcher")
        .process_group(0)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "the background launcher failed: {status:?}"
    );
    let tty = tty().unwrap();
    publish(
        Path::new(&var("SESSION_REPORT")),
        &[
            format!("group={}", own_group()),
            format!("after_foreground={}", foreground(&tty)),
        ],
    );
}

// ---------------------------------------------------------------------------
// Driving a role

/// A role of this binary, with its environment.
fn role_command(role: &str, envs: &[(&str, &OsString)]) -> Command {
    let mut command = support::command(std::env::current_exe().unwrap());
    command
        .args(["--exact", role, "--ignored", "--nocapture"])
        .env("JOB_ROLE", role)
        .env("JOB_EXE", std::env::current_exe().unwrap());
    for (name, value) in envs {
        command.env(name, value);
    }
    command
}

/// Run `command` as the leader of a new session on a fresh pseudo-terminal,
/// which becomes its controlling terminal. Never the developer's terminal:
/// what a launch does to the foreground and the modes is then this test's
/// alone to observe.
///
/// `during` runs once the session has started, given its leader's pid.
/// Returns the process's status and everything it wrote to the terminal, for
/// a failure message.
fn in_pty(
    mut command: Command,
    deadline: Duration,
    during: impl FnOnce(libc::pid_t),
) -> (ExitStatus, String) {
    let (mut master, mut slave) = (-1, -1);
    // SAFETY: valid out pointers; default terminal settings and size.
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
    // SAFETY: openpty returned two fresh owned descriptors.
    let (master, slave) = unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        // SAFETY: live descriptors; neither endpoint should leak on exec.
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            -1
        );
    }
    command
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave));
    // SAFETY: only async-signal-safe calls between fork and exec.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1
                || libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY as _, 0) == -1
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command.spawn().unwrap();
    drop(command);
    // Drained throughout, so a full terminal buffer never stops the session.
    // The read ends in EIO once the session leader and every holder of the
    // slave are gone.
    let mut reader = master;
    let output = std::thread::spawn(move || {
        let mut output = Vec::new();
        let mut buffer = [0u8; 4096];
        while let Ok(read) = reader.read(&mut buffer) {
            if read == 0 {
                break;
            }
            output.extend_from_slice(&buffer[..read]);
        }
        String::from_utf8_lossy(&output).into_owned()
    });
    during(child.id() as libc::pid_t);
    let until = Instant::now() + deadline;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= until {
            // SAFETY: the session this test started; its leader leads its
            // group.
            unsafe { libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL) };
            let _ = child.wait();
            panic!("the session on the pseudo-terminal did not finish");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    // Survivors holding the slave would keep the reader blocked; give it a
    // bound rather than a hang.
    let until = Instant::now() + Duration::from_secs(5);
    while !output.is_finished() && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = if output.is_finished() {
        output.join().unwrap()
    } else {
        String::from("(terminal output still open)")
    };
    (status, output)
}

/// A role with no controlling terminal at all: a new session on no terminal,
/// with null input and its output in `log`.
fn detached(mut command: Command, log: &Path) -> Command {
    let output = fs::File::create(log).unwrap();
    command
        .stdin(Stdio::null())
        .stdout(output.try_clone().unwrap())
        .stderr(output);
    // SAFETY: setsid is async-signal-safe.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    command
}

fn report(path: &Path) -> HashMap<String, String> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("no report at {}: {error}", path.display()))
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
}

fn wait_for(path: &Path, within: Duration) {
    let until = Instant::now() + within;
    while !path.exists() {
        assert!(Instant::now() < until, "{} never appeared", path.display());
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A scratch directory, with a shell script for the launcher's child.
struct Scratch {
    dir: TempDir,
}

impl Scratch {
    fn new() -> Self {
        Self {
            dir: TempDir::new().unwrap(),
        }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    fn os(&self, name: &str) -> OsString {
        self.path(name).into_os_string()
    }

    fn script(&self, body: &str) -> OsString {
        let path = self.path("child.sh");
        fs::write(&path, body).unwrap();
        path.into_os_string()
    }
}

/// Kill a grabber's group, which outlives the launch by design.
fn kill_group(pgid: &str) {
    let pgid: libc::pid_t = pgid.trim().parse().unwrap();
    // SAFETY: the group the grabber fixture created and reported.
    unsafe { libc::kill(-pgid, libc::SIGKILL) };
}

// ---------------------------------------------------------------------------
// The terminal comes back as it was handed over

/// The case the saved attributes exist for. A child that sets the terminal raw
/// and is then killed cannot undo it, so the human would get a shell that does
/// not echo and does not wait for a line. The launcher takes the terminal back
/// from the child's group and restores the modes it saved at the handover.
///
/// The child's own `stty -a` is the positive control: it shows the terminal
/// really was raw while the child held it, so equal modes afterwards are a
/// restore and not a terminal nothing touched.
#[test]
fn a_raw_mode_child_killed_by_the_escalation_leaves_the_terminal_restored() {
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        "stty raw -echo\nstty -a > '{modes}'\n: > \"$TEST_CHANNEL\"\ntrap '' TERM\n\
         while : ; do sleep 0.05 ; done\n",
        modes = scratch.path("raw-modes").display()
    ));
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        Duration::from_secs(20),
        |_| {},
    );
    assert!(status.success(), "{status:?}\n{output}");

    let raw = fs::read_to_string(scratch.path("raw-modes")).unwrap();
    assert!(
        raw.contains("-icanon") && raw.contains("-echo "),
        "control: the child never set the terminal raw:\n{raw}"
    );
    let report = report(&scratch.path("report"));
    assert_eq!(report["before_foreground"], report["group"], "{report:?}");
    assert_eq!(
        report["signal"], "9",
        "the escalation's SIGKILL ended it: {report:?}"
    );
    assert_eq!(report["end"], "escalated");
    assert_eq!(
        report["after_foreground"], report["group"],
        "the launcher did not take the terminal back: {report:?}"
    );
    assert_eq!(
        report["after_modes"], report["before_modes"],
        "the launcher did not restore the modes it saved: {report:?}"
    );
}

/// A child that exits normally hands back nothing it did not hold. When a
/// group other than the child's holds the terminal at that moment, it took the
/// terminal on purpose, as a nested job-control shell does. So the terminal
/// stays with it.
///
/// This test and the next are each other's control: the same grabber, the
/// same terminal, and only how the child ends differs.
#[test]
fn a_child_that_exits_normally_leaves_the_foreground_with_another_group() {
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        "JOB_ROLE=grabber GRABBER_READY='{ready}' \"$JOB_EXE\" --exact grabber --ignored --nocapture &\n\
         while [ ! -f '{ready}' ] ; do sleep 0.01 ; done\nexit 0\n",
        ready = scratch.path("grabber").display()
    ));
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        Duration::from_secs(20),
        |_| {},
    );
    let grabber = fs::read_to_string(scratch.path("grabber")).unwrap_or_default();
    if !grabber.is_empty() {
        kill_group(&grabber);
    }
    assert!(status.success(), "{status:?}\n{output}");

    let report = report(&scratch.path("report"));
    assert_eq!(report["code"], "0", "{report:?}");
    assert_eq!(report["child_group"], "gone");
    assert_eq!(
        report["after_foreground"],
        grabber.trim(),
        "after an ordinary exit the launcher took the terminal from a group \
         that was not its child's: {report:?}"
    );
}

/// After a death by signal, the launcher takes the terminal from whichever
/// group holds it, unless that is its own or the session leader's. A child
/// that is itself a supervisor and was killed leaves its own child's group in
/// the foreground. Nothing else would ever take it back, and that group would
/// go on reading the human's input.
#[test]
fn after_a_death_by_signal_the_launcher_takes_the_terminal_from_another_group() {
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        "JOB_ROLE=grabber GRABBER_READY='{ready}' \"$JOB_EXE\" --exact grabber --ignored --nocapture &\n\
         while [ ! -f '{ready}' ] ; do sleep 0.01 ; done\nkill -KILL $$\n",
        ready = scratch.path("grabber").display()
    ));
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        Duration::from_secs(20),
        |_| {},
    );
    let grabber = fs::read_to_string(scratch.path("grabber")).unwrap_or_default();
    if !grabber.is_empty() {
        kill_group(&grabber);
    }
    assert!(status.success(), "{status:?}\n{output}");

    let report = report(&scratch.path("report"));
    assert_eq!(report["signal"], "9", "{report:?}");
    assert_ne!(
        grabber.trim(),
        report["group"],
        "the grabber must hold a group of its own"
    );
    assert_eq!(
        report["after_foreground"], report["group"],
        "after a death by signal the launcher left the terminal with another \
         group: {report:?}"
    );
    assert_eq!(report["after_modes"], report["before_modes"]);
}

/// A launcher that never held the foreground lent nothing, so it takes nothing
/// back, and its child is never handed the terminal. Here the session leader
/// holds the foreground throughout, as a shell does for its own prompt while a
/// job runs with `&`.
#[test]
fn a_launcher_started_in_the_background_takes_no_foreground() {
    let scratch = Scratch::new();
    let (status, output) = in_pty(
        role_command(
            "background_session",
            &[
                ("JOB_CHILD", &OsString::from("reporter")),
                ("JOB_REPORT", &scratch.os("launcher")),
                ("REPORTER_OUT", &scratch.os("child")),
                ("SESSION_REPORT", &scratch.os("session")),
            ],
        ),
        Duration::from_secs(20),
        |_| {},
    );
    assert!(status.success(), "{status:?}\n{output}");

    let session = report(&scratch.path("session"));
    let launcher = report(&scratch.path("launcher"));
    let child = report(&scratch.path("child"));
    assert_ne!(
        launcher["group"], session["group"],
        "control: the launcher must run in a group of its own"
    );
    assert_eq!(
        launcher["before_foreground"], session["group"],
        "control: the launcher must start in the background"
    );
    assert_ne!(
        child["foreground"], child["group"],
        "a background launcher handed its child the terminal"
    );
    assert_eq!(
        session["after_foreground"], session["group"],
        "a background launcher took the foreground: {launcher:?}"
    );
}

// ---------------------------------------------------------------------------
// Entry signal state is respected

/// An inherited ignored SIGCHLD makes the kernel reap a child unwatched: no
/// zombie to observe, and nothing reserving the group's ID while the rest of
/// the group is killed. The launcher restores the default for itself, so it
/// still sees its child's exit and ends its group. The child still receives
/// the ignore it would have inherited.
#[test]
fn a_launcher_with_sigchld_ignored_still_supervises_its_child_to_an_end() {
    let scratch = Scratch::new();
    let log = scratch.path("log");
    let status = detached(
        role_command(
            "launcher",
            &[
                ("JOB_IGNORE", &OsString::from("chld")),
                ("JOB_CHILD", &OsString::from("reporter")),
                ("JOB_REPORT", &scratch.os("launcher")),
                ("REPORTER_OUT", &scratch.os("child")),
                ("REPORTER_DESCENDANT", &OsString::from("1")),
                ("REPORTER_EXIT", &OsString::from("3")),
            ],
        ),
        &log,
    )
    .status()
    .unwrap();
    let child = report(&scratch.path("child"));
    let descendant: libc::pid_t = child["descendant"].parse().unwrap();
    // SAFETY: probing, then if need be killing, the fixture's own descendant.
    let survived = unsafe { libc::kill(descendant, 0) } == 0;
    if survived {
        unsafe { libc::kill(descendant, libc::SIGKILL) };
    }
    assert!(
        status.success(),
        "{status:?}\n{}",
        fs::read_to_string(&log).unwrap()
    );

    let launcher = report(&scratch.path("launcher"));
    assert_eq!(launcher["code"], "3", "{launcher:?}");
    assert_eq!(launcher["end"], "exited");
    assert_eq!(launcher["child_group"], "gone");
    assert!(!survived, "the child's TERM-ignoring descendant survived");
    assert_eq!(
        launcher["chld"], "default",
        "the launcher must restore SIGCHLD's default, never a handler"
    );
    assert_eq!(
        child["chld"], "ignored",
        "the child must still receive the disposition it would have inherited"
    );
}

/// No handler goes over a disposition the launcher ignores. Ignoring SIGHUP is
/// a statement the launcher made, and a handler would turn a hangup it chose to
/// survive into a cancellation. SIGTERM, which it did not ignore, is the
/// positive control: it shows this launch did install its handlers. The child
/// still gets SIGHUP at its default, as every terminal-generated signal is.
#[test]
fn a_launcher_with_hup_ignored_at_entry_installs_no_hup_handler() {
    let scratch = Scratch::new();
    let log = scratch.path("log");
    let status = detached(
        role_command(
            "launcher",
            &[
                ("JOB_IGNORE", &OsString::from("hup")),
                ("JOB_CHILD", &OsString::from("reporter")),
                ("JOB_REPORT", &scratch.os("launcher")),
                ("REPORTER_OUT", &scratch.os("child")),
            ],
        ),
        &log,
    )
    .status()
    .unwrap();
    assert!(
        status.success(),
        "{status:?}\n{}",
        fs::read_to_string(&log).unwrap()
    );

    let launcher = report(&scratch.path("launcher"));
    let child = report(&scratch.path("child"));
    assert_eq!(launcher["term"], "handled", "control: {launcher:?}");
    assert_eq!(
        launcher["hup"], "ignored",
        "a handler was installed over an ignored SIGHUP"
    );
    assert_eq!(child["hup"], "default");
}

// ---------------------------------------------------------------------------
// Cancellation has its modes

/// INT cancels a launch with no terminal. With no terminal there is no
/// foreground group for a typed Ctrl-C to reach, so the launcher's own SIGINT
/// is the interrupt, and it is forwarded like TERM.
///
/// The control is the same launcher on a terminal. There INT is not caught:
/// it belongs to the foreground group, the child's, and the launcher, holding
/// no handler for it, dies of it.
#[test]
fn int_cancels_a_launch_with_no_terminal() {
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        ": > '{ready}'\nwhile : ; do sleep 0.05 ; done\n",
        ready = scratch.path("ready").display()
    ));
    let log = scratch.path("log");
    let mut launcher = detached(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        &log,
    )
    .spawn()
    .unwrap();
    wait_for(&scratch.path("ready"), Duration::from_secs(10));
    // SAFETY: the launcher is this test's live child, whose handlers were
    // installed before it spawned the child that wrote the marker.
    unsafe { libc::kill(launcher.id() as libc::pid_t, libc::SIGINT) };
    let status = launcher.wait().unwrap();
    assert!(
        status.success(),
        "{status:?}\n{}",
        fs::read_to_string(&log).unwrap()
    );
    let report = report(&scratch.path("report"));
    assert_eq!(report["end"], "interrupted:2", "{report:?}");
    assert_eq!(report["child_group"], "gone");

    // The control, on a terminal.
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        ": > '{ready}'\nwhile : ; do sleep 0.05 ; done\n",
        ready = scratch.path("ready").display()
    ));
    let ready = scratch.path("ready");
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        Duration::from_secs(20),
        |launcher| {
            wait_for(&ready, Duration::from_secs(10));
            // SAFETY: the session leader this test just started.
            unsafe { libc::kill(launcher, libc::SIGINT) };
        },
    );
    assert_eq!(
        status.signal(),
        Some(libc::SIGINT),
        "control: a launcher with a terminal must not catch SIGINT: {status:?}\n{output}"
    );
    assert!(!scratch.path("report").exists());
}

// ---------------------------------------------------------------------------
// A transparent launcher hands on its entry state

/// A transparent launcher's child receives the entry state its caller
/// recorded, in place of the crate's defaults and in place of whatever the
/// launcher's own state has become: SIGHUP ignored and SIGUSR1 blocked, as the
/// entry state says, and SIGUSR2 at its default although the launcher itself
/// ignores it.
///
/// The control is the same launcher, not transparent. Its child gets the
/// defaults for the terminal-generated signals and inherits the launcher's own
/// ignored SIGUSR2 and its mask, so each difference the transparent case shows
/// is the option's doing.
#[test]
fn a_transparent_launcher_hands_its_child_the_entry_state_it_was_given() {
    let child_of = |transparent: bool| {
        let scratch = Scratch::new();
        let log = scratch.path("log");
        let mut envs = vec![
            ("JOB_IGNORE", OsString::from("usr2")),
            ("JOB_CHILD", OsString::from("reporter")),
            ("JOB_REPORT", scratch.os("launcher")),
            ("REPORTER_OUT", scratch.os("child")),
        ];
        if transparent {
            envs.push(("JOB_TRANSPARENT", OsString::from("1")));
        }
        let envs: Vec<(&str, &OsString)> =
            envs.iter().map(|(name, value)| (*name, value)).collect();
        let status = detached(role_command("launcher", &envs), &log)
            .status()
            .unwrap();
        assert!(
            status.success(),
            "{status:?}\n{}",
            fs::read_to_string(&log).unwrap()
        );
        report(&scratch.path("child"))
    };

    let control = child_of(false);
    assert_eq!(control["hup"], "default", "control: {control:?}");
    assert_eq!(control["usr2"], "ignored", "control: {control:?}");
    assert_eq!(control["usr1_blocked"], "false", "control: {control:?}");

    let child = child_of(true);
    assert_eq!(child["hup"], "ignored", "{child:?}");
    assert_eq!(
        child["usr2"], "default",
        "the launcher's own ignore reached the child: {child:?}"
    );
    assert_eq!(child["usr1_blocked"], "true", "{child:?}");
    assert_eq!(child["int"], "default", "{child:?}");
}

/// A transparent launcher with a terminal is cancelled by SIGINT. A typed
/// Ctrl-C reaches its child's group, so an interrupt the launcher receives was
/// sent to it, and it forwards that like TERM. The control is
/// `int_cancels_a_launch_with_no_terminal`'s second half: the same launcher,
/// not transparent, dies of the INT.
#[test]
fn a_transparent_launcher_with_a_terminal_is_cancelled_by_int() {
    let scratch = Scratch::new();
    let child = scratch.script(&format!(
        ": > '{ready}'\nwhile : ; do sleep 0.05 ; done\n",
        ready = scratch.path("ready").display()
    ));
    let ready = scratch.path("ready");
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[
                ("JOB_CHILD", &child),
                ("JOB_REPORT", &scratch.os("report")),
                ("JOB_TRANSPARENT", &OsString::from("1")),
            ],
        ),
        Duration::from_secs(20),
        |launcher| {
            wait_for(&ready, Duration::from_secs(10));
            // SAFETY: the session leader this test just started.
            unsafe { libc::kill(launcher, libc::SIGINT) };
        },
    );
    assert!(status.success(), "{status:?}\n{output}");
    let report = report(&scratch.path("report"));
    assert_eq!(report["end"], "interrupted:2", "{report:?}");
    assert_eq!(report["child_group"], "gone");
    assert_eq!(
        report["after_foreground"], report["group"],
        "the terminal came back: {report:?}"
    );
}
