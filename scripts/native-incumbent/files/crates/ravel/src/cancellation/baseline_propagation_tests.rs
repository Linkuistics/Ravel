//! F07 on the incumbent: exit propagation, judged by the repaired claims.
//!
//! Compiled only into the frozen incumbent subject (`scripts/native-incumbent.sh`).
//! The incumbent's `reraise` performs two operations to end the process by the
//! cancelling signal: it unblocks (`restore_mask`, ignoring the result) and it
//! raises (ignoring the result). The seams below place the row's one fault at
//! exactly those two calls and add no check, no report and no default-action
//! step: `reraise` performs no `sigaction` at all (the entry disposition was
//! restored earlier, by the handlers' `Drop`), so the TERM-default rows name an
//! operation the incumbent does not perform at propagation and are rejected
//! premises in the plan, never run here.
//!
//! Each row runs in a fresh process under a bounded controller of its own. A
//! fixture that never reached the point it was to fault is an instrument fault
//! that fails the test and files no record.
use super::*;
use std::cell::Cell;
use std::fs::{self, File};
use std::os::fd::AsRawFd as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fault {
    None,
    UnblockOmitted,
    UnblockPerformed,
    RaiseOmitted,
}
thread_local! {
    static FAULT: Cell<Fault> = const { Cell::new(Fault::None) };
}
fn set_errno(error: i32) {
    #[cfg(target_vendor = "apple")]
    unsafe {
        *libc::__error() = error;
    }
    #[cfg(target_os = "linux")]
    unsafe {
        *libc::__errno_location() = error;
    }
}

/// The incumbent's unblock before it raises. The result it gets is the
/// substituted EIO, which it ignores.
pub(super) fn unblock(real: impl FnOnce() -> io::Result<()>) -> io::Result<()> {
    match FAULT.get() {
        Fault::UnblockOmitted => {
            eprintln!("native baseline F07 unblock: performed=false substituted=EIO");
            Err(io::Error::from_raw_os_error(libc::EIO))
        }
        Fault::UnblockPerformed => {
            let actual = real();
            eprintln!(
                "native baseline F07 unblock: performed=true actual={actual:?} substituted=EIO"
            );
            Err(io::Error::from_raw_os_error(libc::EIO))
        }
        _ => real(),
    }
}
/// The incumbent's raise. Its result is never read.
pub(super) fn raise(signal: i32) -> i32 {
    if FAULT.get() == Fault::RaiseOmitted {
        eprintln!("native baseline F07 raise: performed=false substituted=EIO");
        set_errno(libc::EIO);
        return -1;
    }
    unsafe { libc::raise(signal) }
}

const ROLE: &str = "cancellation::baseline_propagation_tests::native_baseline_propagation_role";

#[test]
#[ignore = "private process fixture; run native-incumbent.sh test"]
fn native_baseline_propagation_role() {
    // Only the controller's child selects a case; run bare, this is a no-op.
    let Ok(case) = std::env::var("K28_PROPAGATE") else {
        return;
    };
    FAULT.set(match case.as_str() {
        "unblock-omitted" => Fault::UnblockOmitted,
        "unblock-performed" => Fault::UnblockPerformed,
        "raise-omitted" => Fault::RaiseOmitted,
        "signal-death" | "custom-handler" => Fault::None,
        _ => panic!("unknown fixed propagation case {case}"),
    });
    let handlers = Handlers::install().unwrap();
    assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);
    assert_eq!(received(), Signal::of(libc::SIGTERM));
    assert_eq!(handlers.block_and_check(), Signal::of(libc::SIGTERM));
    let mut pending = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    assert_eq!(unsafe { libc::sigpending(&mut pending) }, 0);
    assert_eq!(unsafe { libc::sigismember(&pending, libc::SIGTERM) }, 0);
    eprintln!("native baseline F07 ready: the incumbent's handler consumed the TERM, TERM is not pending, the handled signals are blocked");
    let path = std::env::var_os("K28_RESULT").unwrap();
    let output = File::create(path).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
    // The incumbent drops its handlers before it re-raises: that restores the
    // entry disposition, which is how the signal's default action is back.
    drop(handlers);
    if case == "custom-handler" {
        // The harness control for the TERM-default rows: a handler of the
        // fixture's own, put in place after the incumbent's handlers are gone.
        // If `reraise` reset TERM to its default action the raise would end the
        // process by the signal; the handler running instead shows it does not.
        extern "C" fn custom(_: libc::c_int) {
            let line = b"native baseline F07 custom TERM handler ran\n";
            // SAFETY: one write of a fixed buffer to stderr.
            unsafe { libc::write(2, line.as_ptr().cast(), line.len()) };
        }
        unsafe { libc::signal(libc::SIGTERM, custom as extern "C" fn(libc::c_int) as usize) };
    }
    reraise(Signal::of(libc::SIGTERM).unwrap());
}

fn collect(mut child: std::process::Child) -> Result<ExitStatus, String> {
    let at = Instant::now();
    let case_by = at + Duration::from_secs(25);
    let outer = at + Duration::from_secs(30);
    let mut term_at = None;
    let mut killed = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return if term_at.is_some() {
                    Err(format!(
                        "case watchdog expired; actual fixture reap confirmed: {status}"
                    ))
                } else {
                    Ok(status)
                }
            }
            Err(error) => {
                return Err(format!(
                    "fixture identity/wait uncertain: {error}; no further signalling"
                ))
            }
            Ok(None) => {}
        }
        let now = Instant::now();
        if term_at.is_none() && now >= case_by {
            let result = unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
            eprintln!("native watchdog TERM pid={} raw={result}", child.id());
            term_at = Some(now);
        }
        if !killed && term_at.is_some_and(|at| now.duration_since(at) >= Duration::from_secs(1)) {
            let result = child.kill();
            eprintln!("native watchdog KILL pid={} result={result:?}", child.id());
            killed = true;
        }
        if now >= outer {
            return Err("outer deadline expired; fixture cleanup unconfirmed".into());
        }
        std::thread::sleep(
            Duration::from_millis(1).min(outer.saturating_duration_since(Instant::now())),
        );
    }
}

/// What one fresh process left behind.
struct Seen {
    status: ExitStatus,
    log: String,
    result_bytes: usize,
}
fn run(case: &str) -> Seen {
    let root = tempfile::tempdir().unwrap();
    let result = root.path().join("result");
    let log = root.path().join("log");
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", ROLE, "--ignored", "--nocapture"])
        .env("K28_PROPAGATE", case)
        .env("K28_RESULT", &result)
        .stdout(Stdio::null())
        .stderr(File::create(&log).unwrap())
        .process_group(0);
    let status = collect(command.spawn().unwrap()).unwrap();
    let log = fs::read_to_string(log).unwrap();
    let bytes = fs::read(result).unwrap();
    eprintln!(
        "native baseline F07 case={case} raw_wait={} result_bytes={}\n{log}",
        status.into_raw(),
        bytes.len()
    );
    Seen {
        status,
        log,
        result_bytes: bytes.len(),
    }
}

fn ids() -> Vec<String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
            .unwrap();
    manifest["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["family"] == "F07")
        .map(|row| row["id"].as_str().unwrap().to_owned())
        .collect()
}

/// The named claims the repaired instrument judges, restated for what the
/// incumbent can expose. They are the same claims: the process exits with the
/// signal's status having reported the failed transition (`reported`), or, for
/// the control, ends by the real signal.
#[test]
#[ignore = "private process fixture; run native-incumbent.sh test"]
fn native_baseline_f07_exit_propagation_transitions_and_real_signal_death() {
    let ids = ids();
    assert_eq!(ids.len(), 6);
    let find = |suffix: &str| -> String {
        let matching: Vec<_> = ids.iter().filter(|id| id.ends_with(suffix)).collect();
        assert_eq!(matching.len(), 1, "{suffix}: {matching:?}");
        matching[0].clone()
    };
    for (case, suffix, marker) in [
        ("unblock-omitted", "operation=TERM-unblock__mode=omit-EIO", Some("F07 unblock: performed=false")),
        ("unblock-performed", "operation=TERM-unblock__mode=perform-report-EIO", Some("F07 unblock: performed=true")),
        ("raise-omitted", "operation=TERM-raise__mode=omit-EIO", Some("F07 raise: performed=false")),
        ("signal-death", "operation=TERM-raise__mode=unmodified-signal-death-control", None),
    ] {
        let id = find(suffix);
        let seen = run(case);
        // Instrument integrity: the process reached the boundary and, for a
        // fault row, the selected operation. Anything else files no record.
        assert!(
            seen.log.contains("native baseline F07 ready"),
            "{id}: the fixture never reached the propagation boundary: {}",
            seen.log
        );
        if let Some(marker) = marker {
            assert!(seen.log.contains(marker), "{id}: the site was never reached: {}", seen.log);
        }
        assert_eq!(seen.result_bytes, 0, "{id}");
        let conforms = if case == "signal-death" {
            seen.status.signal() == Some(libc::SIGTERM)
        } else {
            seen.status.code() == Some(128 + libc::SIGTERM)
                && seen.log.contains("ravel: cannot propagate SIGTERM:")
                && seen.log.contains("os error 5")
        };
        let ended = match (seen.status.code(), seen.status.signal()) {
            (Some(code), _) => format!("exit code {code}"),
            (_, Some(signal)) => format!("signal death {signal}"),
            _ => "unknown ending".to_owned(),
        };
        crate::native_barrier::row(
            &id,
            "baseline",
            conforms,
            &format!(
                "{ended}; reported a failed transition: {}; result bytes {}",
                seen.log.contains("ravel: cannot propagate"),
                seen.result_bytes
            ),
        );
    }
}

// A harness control: with no fault the same fixture ends by the real signal, so
// the exit-code rows above are the faulted operation's doing and not a fixture
// that cannot die by TERM. It files no row.
#[test]
#[ignore = "private process fixture; run native-incumbent.sh test"]
fn native_baseline_control_f07_the_unfaulted_fixture_dies_by_the_signal() {
    let seen = run("signal-death");
    assert_eq!(seen.status.signal(), Some(libc::SIGTERM), "{}", seen.log);
    let faulted = run("unblock-omitted");
    assert_eq!(faulted.status.code(), Some(128 + libc::SIGTERM), "{}", faulted.log);
}

/// The F07 TERM-default rows are rejected premises: `reraise` performs no step
/// that sets the signal's default action (the entry disposition was restored
/// earlier, by the handlers' `Drop`). Seen here as a fact: with a handler of the
/// fixture's own installed just before `reraise`, the raise runs that handler and
/// the process ends by `exit(143)`, not by the signal.
#[test]
#[ignore = "private process fixture; run native-incumbent.sh test"]
fn native_baseline_control_f07_reraise_performs_no_default_action_step() {
    let seen = run("custom-handler");
    assert!(
        seen.log.contains("native baseline F07 custom TERM handler ran"),
        "the raise did not reach the fixture's own handler: {}",
        seen.log
    );
    assert_eq!(seen.status.code(), Some(128 + libc::SIGTERM), "{}", seen.log);
    assert_eq!(seen.status.signal(), None);
}
