//! A captured job: its pipes are the caller's, and the caller stops it.
mod support;

use std::ffi::OsString;
use std::fs;
use std::os::unix::process::ExitStatusExt as _;
use std::path::Path;
use std::time::{Duration, Instant};

use keyed_launch::{
    Argv, CapturedLaunch, Channel, End, Escalation, Group, StopRequest, run_captured,
};

const VARIABLE: &str = "RUNNER_CHANNEL";

/// Run `script` under `sh -c` with `stdin`, a bound of `limit` bytes for each
/// stream and an escalation of `grace` and `kill_grace`.
fn launch(
    script: &str,
    stdin: &[u8],
    limit: usize,
    escalation: Escalation,
    stop: &StopRequest,
) -> keyed_launch::Captured {
    let dir = tempfile::tempdir().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let argv = Argv::new(
        "/bin/sh".into(),
        vec![OsString::from("-c"), OsString::from(script)],
    );
    run_captured(
        CapturedLaunch {
            argv: &argv,
            channel: (&channel, VARIABLE),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(dir.path()),
            escalation,
            stdin,
            limit,
            stop,
        },
        None,
        &mut |_| {},
    )
    .unwrap()
}

const PATIENT: Escalation = Escalation {
    grace: Duration::from_secs(30),
    kill_grace: Duration::from_secs(30),
};

fn quick(grace: u64, kill_grace: u64) -> Escalation {
    Escalation {
        grace: Duration::from_millis(grace),
        kill_grace: Duration::from_millis(kill_grace),
    }
}

fn gone(pid: i32) -> bool {
    // SAFETY: signal zero only tests whether the process exists.
    unsafe { libc::kill(pid, 0) == -1 }
}

#[test]
fn the_two_streams_and_the_status_come_back_apart() {
    let captured = launch(
        "echo out; echo err >&2; exit 7",
        b"",
        1024,
        PATIENT,
        &StopRequest::new(),
    );
    assert_eq!(captured.stdout, b"out\n");
    assert_eq!(captured.stderr, b"err\n");
    assert_eq!(captured.ended.status.code(), Some(7));
    assert_eq!(captured.ended.end, End::Exited);
    assert_eq!(captured.ended.group, Group::Gone);
    assert!(!captured.overflow);
}

#[test]
fn the_job_reads_the_text_it_was_given_and_then_end_of_file() {
    let captured = launch(
        "cat",
        b"first\nsecond\n",
        1024,
        PATIENT,
        &StopRequest::new(),
    );
    assert_eq!(captured.stdout, b"first\nsecond\n");
}

#[test]
fn without_text_standard_input_is_at_end_of_file() {
    // The control for the test above: `cat` ends only on an end of file.
    let began = Instant::now();
    let captured = launch("cat; echo done", b"", 1024, PATIENT, &StopRequest::new());
    assert_eq!(captured.stdout, b"done\n");
    assert!(began.elapsed() < Duration::from_secs(10));
}

#[test]
fn text_larger_than_a_pipe_reaches_a_job_that_reads_it_all() {
    let text = vec![b'x'; 300_000];
    let captured = launch("wc -c", &text, 1024, PATIENT, &StopRequest::new());
    assert_eq!(String::from_utf8_lossy(&captured.stdout).trim(), "300000");
}

#[test]
fn a_job_that_ignores_its_input_is_not_an_error() {
    let text = vec![b'x'; 300_000];
    let captured = launch("echo no", &text, 1024, PATIENT, &StopRequest::new());
    assert_eq!(captured.stdout, b"no\n");
    assert_eq!(captured.ended.status.code(), Some(0));
}

#[test]
fn the_first_byte_beyond_the_bound_stops_the_job() {
    let began = Instant::now();
    let captured = launch(
        "head -c 2000000 /dev/zero; sleep 30",
        b"",
        1000,
        quick(30_000, 5_000),
        &StopRequest::new(),
    );
    assert!(captured.overflow);
    assert_eq!(captured.stdout.len(), 1000);
    assert_eq!(captured.ended.end, End::Escalated);
    assert_eq!(captured.ended.group, Group::Gone);
    assert_eq!(captured.ended.status.signal(), Some(libc::SIGTERM));
    assert!(began.elapsed() < Duration::from_secs(10));
}

#[test]
fn the_error_stream_has_the_same_bound() {
    let captured = launch(
        "head -c 5000 /dev/zero >&2; sleep 30",
        b"",
        1000,
        PATIENT,
        &StopRequest::new(),
    );
    assert!(captured.overflow);
    assert_eq!(captured.stderr.len(), 1000);
    assert_eq!(captured.ended.end, End::Escalated);
}

#[test]
fn output_of_exactly_the_bound_is_no_overflow() {
    // The firing control of the two tests above.
    let captured = launch(
        "head -c 1000 /dev/zero",
        b"",
        1000,
        PATIENT,
        &StopRequest::new(),
    );
    assert!(!captured.overflow);
    assert_eq!(captured.stdout.len(), 1000);
    assert_eq!(captured.ended.end, End::Exited);
}

#[test]
fn a_stop_request_ends_the_job_and_its_descendants() {
    let dir = tempfile::tempdir().unwrap();
    let pids = dir.path().join("pids");
    let script = format!(
        "sleep 30 & echo $! > {pids:?}; wait",
        pids = pids.display().to_string()
    );
    let stop = StopRequest::new();
    let requester = {
        let stop = stop.clone();
        let pids = pids.clone();
        std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(10);
            while fs::read_to_string(&pids).map_or(true, |text| text.trim().is_empty()) {
                assert!(Instant::now() < deadline, "the job never started");
                std::thread::sleep(Duration::from_millis(10));
            }
            stop.request();
        })
    };
    let began = Instant::now();
    let captured = launch(&script, b"", 1024, quick(30_000, 5_000), &stop);
    requester.join().unwrap();
    assert_eq!(captured.ended.end, End::Escalated);
    assert_eq!(captured.ended.group, Group::Gone);
    assert_eq!(captured.ended.status.signal(), Some(libc::SIGTERM));
    assert!(!captured.overflow);
    let descendant: i32 = fs::read_to_string(&pids).unwrap().trim().parse().unwrap();
    assert!(gone(descendant), "the descendant {descendant} survived");
    assert!(began.elapsed() < Duration::from_secs(10));
}

#[test]
fn without_a_stop_request_the_same_job_runs_to_its_end() {
    // The firing control of the test above.
    let captured = launch("sleep 0.2", b"", 1024, PATIENT, &StopRequest::new());
    assert_eq!(captured.ended.end, End::Exited);
    assert_eq!(captured.ended.status.code(), Some(0));
}

#[test]
fn a_job_that_ignores_terminate_is_killed_after_the_kill_grace() {
    let dir = tempfile::tempdir().unwrap();
    let ready = dir.path().join("ready");
    let script = format!(
        "trap '' TERM; : > {ready:?}; while :; do :; done",
        ready = ready.display().to_string()
    );
    let stop = StopRequest::new();
    // The stop comes after the trap is set, or the first TERM kills the shell.
    let requester = {
        let stop = stop.clone();
        std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(10);
            while !ready.exists() {
                assert!(Instant::now() < deadline, "the job never started");
                std::thread::sleep(Duration::from_millis(10));
            }
            stop.request();
        })
    };
    let began = Instant::now();
    let captured = launch(&script, b"", 1024, quick(30_000, 400), &stop);
    requester.join().unwrap();
    assert_eq!(captured.ended.end, End::Escalated);
    assert_eq!(captured.ended.status.signal(), Some(libc::SIGKILL));
    assert_eq!(captured.ended.group, Group::Gone);
    assert!(began.elapsed() >= Duration::from_millis(400));
}

#[test]
fn a_value_sent_on_the_channel_starts_the_escalation() {
    let began = Instant::now();
    let captured = launch(
        &format!(": > \"${VARIABLE}\"; sleep 30"),
        b"",
        1024,
        quick(100, 5_000),
        &StopRequest::new(),
    );
    assert!(captured.ended.signalled);
    assert_eq!(captured.ended.end, End::Escalated);
    assert_eq!(captured.ended.status.signal(), Some(libc::SIGTERM));
    assert!(began.elapsed() < Duration::from_secs(10));
}

#[test]
fn the_job_leads_its_own_group_and_has_no_terminal() {
    let captured = launch(
        "ps -o pid=,pgid=,tty= -p $$; [ -t 0 ] || echo no-input-tty",
        b"",
        1024,
        PATIENT,
        &StopRequest::new(),
    );
    let text = String::from_utf8_lossy(&captured.stdout).into_owned();
    let mut fields = text.lines().next().unwrap().split_whitespace();
    let (pid, group, tty) = (
        fields.next().unwrap(),
        fields.next().unwrap(),
        fields.next().unwrap(),
    );
    assert_eq!(pid, group, "the job does not lead its group: {text}");
    assert!(
        tty.starts_with('?'),
        "the job has the terminal {tty}: {text}"
    );
    assert!(text.contains("no-input-tty"), "{text}");
}

#[test]
fn the_environment_is_the_launchers_minus_the_scrub_plus_the_grant() {
    let dir = tempfile::tempdir().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let argv = Argv::new(
        "/bin/sh".into(),
        vec![
            OsString::from("-c"),
            OsString::from("echo \"[$RAVEL_EXIT_FILE][$GRANTED]\""),
        ],
    );
    // SAFETY: this binary runs its tests on threads, and no other test reads
    // this variable. It is set only so that the scrub has something to remove.
    unsafe { std::env::set_var("RAVEL_EXIT_FILE", "enclosing") };
    let granted = [(std::ffi::OsStr::new("GRANTED"), std::ffi::OsStr::new("yes"))];
    let captured = run_captured(
        CapturedLaunch {
            argv: &argv,
            channel: (&channel, "NOT_THE_ENCLOSING_ONE"),
            scrub: support::controls(),
            grant: &granted,
            transparent: None,
            cwd: Some(Path::new("/")),
            escalation: PATIENT,
            stdin: b"",
            limit: 1024,
            stop: &StopRequest::new(),
        },
        None,
        &mut |_| {},
    )
    .unwrap();
    assert_eq!(captured.stdout, b"[][yes]\n");
}
