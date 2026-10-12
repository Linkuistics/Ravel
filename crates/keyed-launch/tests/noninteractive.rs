//! A standalone invocation must not share its caller's terminal or stdin.
mod support;

use std::ffi::OsString;
use std::fs;
use std::io::{Read, Seek, Write};
use std::os::fd::AsRawFd;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use keyed_launch::{Argv, Channel, End, Escalation, Group, Launch};

#[test]
#[ignore = "subprocess fixture"]
fn child() {
    if std::env::var("RUNNER_ROLE").as_deref() != Ok("child") {
        return;
    }
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // SAFETY: queries with no mutable memory or ownership transferred.
    let (sid, pid, inherited) = unsafe {
        (
            libc::getsid(0),
            libc::getpid(),
            libc::fcntl(197, libc::F_GETFD),
        )
    };
    fs::write(
        std::env::var_os("RUNNER_REPORT").unwrap(),
        format!("{sid} {pid} {inherited}\n{input}"),
    )
    .unwrap();
}

fn publish_ready() {
    let report = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
    let temporary = report.with_extension("tmp");
    // SAFETY: queries of this fixture's own process and process group.
    let (pid, group) = unsafe { (libc::getpid(), libc::getpgrp()) };
    fs::write(&temporary, format!("{pid} {group}\n")).unwrap();
    fs::rename(temporary, report).unwrap();
}

#[test]
#[ignore = "subprocess fixture"]
fn stubborn_child() {
    // SAFETY: this fixture is an isolated process and intentionally ignores
    // both cancellation signals to require supervisor-driven SIGKILL cleanup.
    unsafe {
        libc::signal(libc::SIGTERM, libc::SIG_IGN);
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
    publish_ready();
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// A child that keeps every default disposition, so a forwarded cancelling
/// signal ends it, and it dies of that signal rather than of a SIGKILL.
#[test]
#[ignore = "subprocess fixture"]
fn cooperative_child() {
    publish_ready();
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[test]
#[ignore = "subprocess fixture"]
fn writing_helper() {
    let mut heartbeat = fs::File::create(std::env::var_os("RUNNER_HEARTBEAT").unwrap()).unwrap();
    heartbeat.write_all(b"alive\n").unwrap();
    publish_ready();
    loop {
        heartbeat.write_all(b"alive\n").unwrap();
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
#[ignore = "subprocess fixture"]
fn exiting_leader() {
    let helper = support::command(std::env::current_exe().unwrap())
        .args(["--exact", "writing_helper", "--ignored", "--nocapture"])
        .spawn()
        .unwrap();
    let ready = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
    let deadline = Instant::now() + Duration::from_secs(5);
    while !ready.exists() {
        assert!(Instant::now() < deadline, "helper never became ready");
        std::thread::sleep(Duration::from_millis(10));
    }
    // Dropping Child does not reap or terminate it: the launcher must clean up
    // the still-running group member after this group leader exits.
    drop(helper);
}

#[test]
#[ignore = "subprocess fixture"]
fn supervisor() {
    if std::env::var("RUNNER_ROLE").as_deref() != Ok("supervisor") {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let executable = std::env::current_exe().unwrap();
    let scenario = std::env::var("RUNNER_SCENARIO").unwrap_or_else(|_| "child".into());
    let kill_grace = Duration::from_millis(
        std::env::var("RUNNER_KILL_GRACE_MS").map_or(10_000, |grace| grace.parse().unwrap()),
    );
    let expected_signal = || -> i32 {
        std::env::var("RUNNER_EXPECTED_SIGNAL")
            .unwrap()
            .parse()
            .unwrap()
    };
    if scenario == "confined_stubborn" {
        confined_supervisor(kill_grace, expected_signal());
        return;
    }
    let argv = Argv::new(
        "env".into(),
        vec![
            "RUNNER_ROLE=child".into(),
            executable.into_os_string(),
            "--exact".into(),
            OsString::from(&scenario),
            "--ignored".into(),
            "--nocapture".into(),
        ],
    );
    let result = keyed_launch::run_noninteractive(
        keyed_launch::NoninteractiveLaunch {
            argv: &argv,
            scrub: support::controls(),
            cwd: Some(dir.path()),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace,
            },
        },
        fs::File::create(dir.path().join("harness.log")).unwrap(),
    )
    .unwrap();
    assert_eq!(result.group, Group::Gone);
    match scenario.as_str() {
        // Forwarded, then killed once the kill-grace has run out.
        "stubborn_child" => {
            assert_eq!(
                result.end,
                End::Interrupted {
                    signal: expected_signal()
                }
            );
            assert_eq!(result.status.signal(), Some(libc::SIGKILL));
            assert!(
                result.elapsed >= kill_grace,
                "the stubborn child was killed before its kill-grace: {:?}",
                result.elapsed
            );
        }
        // Forwarded, and the child died of it: no SIGKILL was needed.
        "cooperative_child" => {
            assert_eq!(
                result.end,
                End::Interrupted {
                    signal: expected_signal()
                }
            );
            assert_eq!(result.status.signal(), Some(expected_signal()));
        }
        _ => assert!(result.status.success()),
    }
    if scenario != "child" {
        let ready = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
        let (pid, _) = read_ready(&ready).unwrap();
        // Check at the return boundary, before an outer wait could hide late
        // cleanup. An existing PID, including a zombie, is not confirmed gone.
        assert_process_gone(pid);
    }
    if scenario == "exiting_leader" {
        assert_eq!(result.end, End::Exited);
        let heartbeat = PathBuf::from(std::env::var_os("RUNNER_HEARTBEAT").unwrap());
        let contents = fs::read(&heartbeat).unwrap();
        assert!(!contents.is_empty(), "helper never wrote anything");
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(
            fs::read(heartbeat).unwrap(),
            contents,
            "helper kept writing after return"
        );
    }
}

/// The confined half of the supervisor fixture: a shell that ignores both
/// cancelling signals, launched under confinement with a long kill-grace.
///
/// Inside the sandbox the child's pid may be a namespace's, so readiness is a
/// marker file in the writable directory, and that the group is gone is the
/// runner's own report.
fn confined_supervisor(kill_grace: Duration, expected_signal: i32) {
    let work = PathBuf::from(std::env::var_os("RUNNER_WORK").unwrap());
    let script = work.join("stubborn.sh");
    fs::write(
        &script,
        "trap '' TERM INT\n: > started\nwhile : ; do sleep 0.05 ; done\n",
    )
    .unwrap();
    let argv = Argv::new("/bin/sh".into(), vec![script.into_os_string()]);
    let channel = Channel::allocate(&work).unwrap();
    let result = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "RUNNER_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace,
            },
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: &[],
        },
        &mut |_| {},
    )
    .unwrap();
    assert_eq!(
        result.end,
        End::Interrupted {
            signal: expected_signal
        }
    );
    assert_eq!(result.status.signal(), Some(libc::SIGKILL));
    assert!(
        result.elapsed < kill_grace,
        "a confined cancellation waited out the kill-grace: {:?}",
        result.elapsed
    );
    assert_eq!(result.group, Group::Gone);
}

/// The control's supervisor: it launches the `child` fixture as any process
/// launches another, with nothing of `run_noninteractive` between them.
#[test]
#[ignore = "subprocess fixture"]
fn plain_supervisor() {
    if std::env::var("RUNNER_ROLE").as_deref() != Ok("supervisor") {
        return;
    }
    let status = support::command(std::env::current_exe().unwrap())
        .args(["--exact", "child", "--ignored", "--nocapture"])
        .env("RUNNER_ROLE", "child")
        .status()
        .unwrap();
    assert!(status.success());
}

fn read_ready(path: &Path) -> Option<(libc::pid_t, libc::pid_t)> {
    let contents = fs::read_to_string(path).ok()?;
    let mut fields = contents.split_whitespace();
    Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
}

fn assert_process_gone(pid: libc::pid_t) {
    // SAFETY: signal zero only checks the recorded fixture process's presence.
    assert_eq!(
        unsafe { libc::kill(pid, 0) },
        -1,
        "fixture process {pid} survived runner return"
    );
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH)
    );
}

struct RunningSupervisor {
    process: Child,
    ready: PathBuf,
    log: PathBuf,
}

impl RunningSupervisor {
    fn start(directory: &Path, scenario: &str, signal: i32) -> Self {
        Self::start_with(directory, scenario, signal, &[])
    }

    fn start_with(
        directory: &Path,
        scenario: &str,
        signal: i32,
        extra: &[(&str, &std::ffi::OsStr)],
    ) -> Self {
        let ready = directory.join("ready");
        let log = directory.join("supervisor.log");
        let stdout = fs::File::create(&log).unwrap();
        let stderr = stdout.try_clone().unwrap();
        let process = support::command(std::env::current_exe().unwrap())
            .args(["--exact", "supervisor", "--ignored", "--nocapture"])
            .env("RUNNER_ROLE", "supervisor")
            .env("RUNNER_SCENARIO", scenario)
            .env("RUNNER_READY", &ready)
            .env("RUNNER_HEARTBEAT", directory.join("heartbeat"))
            .env("RUNNER_EXPECTED_SIGNAL", signal.to_string())
            .envs(extra.iter().copied())
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .unwrap();
        Self {
            process,
            ready,
            log,
        }
    }

    fn await_ready(&mut self) -> (libc::pid_t, libc::pid_t) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(ready) = read_ready(&self.ready) {
                return ready;
            }
            assert!(
                self.process.try_wait().unwrap().is_none(),
                "supervisor ended before readiness: {}",
                fs::read_to_string(&self.log).unwrap()
            );
            assert!(Instant::now() < deadline, "fixture did not become ready");
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn await_exit(&mut self, deadline: Instant) -> ExitStatus {
        loop {
            if let Some(status) = self.process.try_wait().unwrap() {
                assert!(
                    status.success(),
                    "supervisor failed: {}",
                    fs::read_to_string(&self.log).unwrap()
                );
                return status;
            }
            assert!(
                Instant::now() < deadline,
                "supervisor exceeded cleanup deadline: {}",
                fs::read_to_string(&self.log).unwrap()
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for RunningSupervisor {
    fn drop(&mut self) {
        if let Some((_, group)) = read_ready(&self.ready) {
            // SAFETY: the fixture reports the private group created by the
            // launcher. This also prevents a failing test from leaking it.
            unsafe { libc::kill(-group, libc::SIGKILL) };
        }
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

/// Cancel the named noninteractive scenario with `signal` once its child is
/// ready, and require the supervisor to finish inside `within`.
fn assert_cancellation_cleanup(scenario: &str, signal: i32, kill_grace_ms: &str, within: Duration) {
    let directory = tempfile::tempdir().unwrap();
    let mut supervisor = RunningSupervisor::start_with(
        directory.path(),
        scenario,
        signal,
        &[("RUNNER_KILL_GRACE_MS", kill_grace_ms.as_ref())],
    );
    let (child, _) = supervisor.await_ready();
    let deadline = Instant::now() + within;
    // SAFETY: the supervisor is our live direct child and readiness confirms
    // its cancellation handler has already been installed.
    assert_eq!(
        unsafe { libc::kill(supervisor.process.id() as libc::pid_t, signal) },
        0
    );
    supervisor.await_exit(deadline);
    assert_process_gone(child);
}

/// A noninteractive child may itself be a supervisor, which needs the
/// cancelling signal and time to end its own child. So the signal is forwarded:
/// a child that dies of it is never killed, and the supervisor returns well
/// inside its ten-second kill-grace.
#[test]
fn sigint_cancellation_is_forwarded_to_a_noninteractive_child() {
    assert_cancellation_cleanup(
        "cooperative_child",
        libc::SIGINT,
        "10000",
        Duration::from_secs(4),
    );
}

#[test]
fn sigterm_cancellation_is_forwarded_to_a_noninteractive_child() {
    assert_cancellation_cleanup(
        "cooperative_child",
        libc::SIGTERM,
        "10000",
        Duration::from_secs(4),
    );
}

/// A noninteractive child that ignores the forwarded signal is killed once the
/// kill-grace has run out, and not before: the supervisor asserts both.
#[test]
fn a_stubborn_noninteractive_child_is_killed_after_the_kill_grace() {
    assert_cancellation_cleanup(
        "stubborn_child",
        libc::SIGTERM,
        "1000",
        Duration::from_secs(4),
    );
}

/// A confined child's group is killed at once, so a cancellation nested inside
/// another supervisor's grace finishes inside it. The kill-grace here is ten
/// seconds, and the supervisor must finish in four.
#[test]
fn a_confined_child_is_killed_at_once_on_cancellation() {
    let directory = tempfile::tempdir().unwrap();
    let work = directory.path().canonicalize().unwrap().join("work");
    fs::create_dir(&work).unwrap();
    let mut supervisor = RunningSupervisor::start_with(
        directory.path(),
        "confined_stubborn",
        libc::SIGTERM,
        &[
            ("RUNNER_KILL_GRACE_MS", "10000".as_ref()),
            ("RUNNER_WORK", work.as_os_str()),
        ],
    );
    let started = work.join("started");
    let ready_by = Instant::now() + Duration::from_secs(5);
    while !started.exists() {
        assert!(
            supervisor.process.try_wait().unwrap().is_none(),
            "supervisor ended before readiness: {}",
            fs::read_to_string(&supervisor.log).unwrap()
        );
        assert!(Instant::now() < ready_by, "confined child never started");
        std::thread::sleep(Duration::from_millis(10));
    }
    let deadline = Instant::now() + Duration::from_secs(4);
    // SAFETY: the supervisor is our live direct child, which installed its
    // handlers before it spawned the child that wrote the marker.
    assert_eq!(
        unsafe { libc::kill(supervisor.process.id() as libc::pid_t, libc::SIGTERM) },
        0
    );
    supervisor.await_exit(deadline);
}

#[test]
fn leader_exit_stops_a_group_helper_before_runner_returns() {
    let directory = tempfile::tempdir().unwrap();
    let mut supervisor = RunningSupervisor::start(directory.path(), "exiting_leader", 0);
    let (helper, group) = supervisor.await_ready();
    assert_ne!(helper, group, "fixture did not create a separate helper");
    supervisor.await_exit(Instant::now() + Duration::from_secs(4));
    assert_process_gone(helper);
}

const CALLERS_INPUT: &str = "parent input";

/// What a `child` fixture found under one supervisor, and what was left of the
/// caller's input afterwards.
struct Observed {
    session: libc::pid_t,
    pid: libc::pid_t,
    /// `fcntl(197, F_GETFD)` in the child: -1 unless it holds the descriptor.
    descriptor: i32,
    /// What the child read from its own stdin.
    read: String,
    /// How far into the caller's input any process read.
    consumed: u64,
}

/// Run the `child` fixture under the named supervisor fixture, whose stdin is
/// the caller's input, and return what the child found.
///
/// The input is a file, complete before the supervisor exists. It used to be
/// written to a pipe after the spawn. The supervisor holds that pipe's only
/// read end, so one that ended first, in its ordinary course or at once because
/// it failed, broke the pipe, and the write reported that in place of the
/// supervisor's own failure (noninteractive-stdin-flake-k55).
fn observe_child_under(supervisor: &str) -> Observed {
    let dir = tempfile::tempdir().unwrap();
    let report = dir.path().join("report");
    let input = dir.path().join("callers-input");
    fs::write(&input, CALLERS_INPUT).unwrap();
    let mut callers_stdin = fs::File::open(input).unwrap();
    let secret = fs::File::create(dir.path().join("parent-secret")).unwrap();
    let descriptor = secret.as_raw_fd();
    let mut command = support::command(std::env::current_exe().unwrap());
    // SAFETY: dup2 is async-signal-safe and both descriptor numbers are valid.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(descriptor, 197) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let result = command
        .args(["--exact", supervisor, "--ignored", "--nocapture"])
        .env("RUNNER_ROLE", "supervisor")
        .env("RUNNER_REPORT", &report)
        // A duplicate shares its offset with the original, which is how
        // `consumed` below sees what any process downstream read.
        .stdin(callers_stdin.try_clone().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let report = fs::read_to_string(report).unwrap();
    let (found, read) = report.split_once('\n').unwrap();
    let mut fields = found.split(' ').map(|field| field.parse().unwrap());
    Observed {
        session: fields.next().unwrap(),
        pid: fields.next().unwrap(),
        descriptor: fields.next().unwrap(),
        read: read.to_owned(),
        consumed: callers_stdin.stream_position().unwrap(),
    }
}

#[test]
fn standalone_child_has_a_new_session_and_cannot_consume_callers_stdin() {
    let child = observe_child_under("supervisor");
    assert_eq!(child.read, "", "child consumed parent input");
    assert_eq!(child.consumed, 0, "the caller's input was read");
    assert_eq!(child.session, child.pid, "child is not a session leader");
    assert_eq!(child.descriptor, -1, "child inherited a parent descriptor");
}

/// The control for the test above. A child launched as any process launches
/// another shares its launcher's stdin, session and descriptors, so each
/// observation that test makes has to come back the other way here.
#[test]
fn a_plainly_launched_child_consumes_callers_stdin_and_shares_its_session_and_descriptors() {
    let child = observe_child_under("plain_supervisor");
    assert_eq!(child.read, CALLERS_INPUT);
    assert_eq!(child.consumed, CALLERS_INPUT.len() as u64);
    assert_ne!(child.session, child.pid);
    assert_ne!(child.descriptor, -1);
}
