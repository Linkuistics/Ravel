//! Fixed exit-propagation controls, each in a fresh executable fixture.
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
    DefaultOmitted,
    DefaultPerformed,
    UnblockOmitted,
    UnblockPerformed,
    RaiseOmitted,
}
thread_local! {
    static FAULT: Cell<Fault> = const { Cell::new(Fault::None) };
    static BROKEN: Cell<bool> = const { Cell::new(false) };
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
pub(super) fn default_action(signal: i32, action: &libc::sigaction) -> i32 {
    let fault = FAULT.get();
    let performed = fault != Fault::DefaultOmitted;
    let result = if performed {
        unsafe { libc::sigaction(signal, action, ptr::null_mut()) }
    } else {
        0
    };
    if matches!(fault, Fault::DefaultOmitted | Fault::DefaultPerformed) {
        eprintln!(
            "native F07 default: performed={performed} actual={result} substituted=EIO broken={}",
            BROKEN.get()
        );
        if !BROKEN.get() {
            set_errno(libc::EIO);
            return -1;
        }
        return 0;
    }
    result
}
pub(super) fn unblock(selected: &libc::sigset_t) -> i32 {
    let fault = FAULT.get();
    let performed = fault != Fault::UnblockOmitted;
    let result = if performed {
        unsafe { libc::pthread_sigmask(libc::SIG_UNBLOCK, selected, ptr::null_mut()) }
    } else {
        0
    };
    if matches!(fault, Fault::UnblockOmitted | Fault::UnblockPerformed) {
        eprintln!(
            "native F07 unblock: performed={performed} actual={result} substituted=EIO broken={}",
            BROKEN.get()
        );
        if !BROKEN.get() {
            return libc::EIO;
        }
        return 0;
    }
    result
}
pub(super) fn raise(signal: i32) -> i32 {
    if FAULT.get() == Fault::RaiseOmitted {
        eprintln!(
            "native F07 raise: performed=false substituted=EIO broken={}",
            BROKEN.get()
        );
        return if BROKEN.get() { 0 } else { libc::EIO };
    }
    unsafe { libc::raise(signal) }
}

const ROLE: &str = "cancellation::propagation_tests::native_propagation_role";
#[test]
#[ignore = "private process fixture"]
fn native_propagation_role() {
    let case = std::env::var("K28_PROPAGATE").unwrap();
    FAULT.set(match case.as_str() {
        "default-omitted" => Fault::DefaultOmitted,
        "default-performed" => Fault::DefaultPerformed,
        "unblock-omitted" => Fault::UnblockOmitted,
        "unblock-performed" => Fault::UnblockPerformed,
        "raise-omitted" => Fault::RaiseOmitted,
        "signal-death" => Fault::None,
        _ => panic!("unknown fixed propagation case {case}"),
    });
    BROKEN.set(std::env::var_os("K28_PROPAGATE_BROKEN").is_some());
    let mut scope = Handlers::install().unwrap();
    assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);
    assert_eq!(received(), Signal::of(libc::SIGTERM));
    assert_eq!(
        scope.try_block_and_check().unwrap(),
        Signal::of(libc::SIGTERM)
    );
    let mut pending = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    assert_eq!(unsafe { libc::sigpending(&mut pending) }, 0);
    assert_eq!(unsafe { libc::sigismember(&pending, libc::SIGTERM) }, 0);
    eprintln!("native F07 ready: first actual TERM handler consumption confirmed, selected TERM pending=false, main handled mask blocked");
    let path = std::env::var_os("K28_RESULT").unwrap();
    let output = File::create(path).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
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
fn case(case: &str, broken: bool) -> bool {
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
    if broken {
        command.env("K28_PROPAGATE_BROKEN", "1");
    }
    let status = collect(command.spawn().unwrap()).unwrap();
    let log = fs::read_to_string(log).unwrap();
    let bytes = fs::read(result).unwrap();
    eprintln!(
        "native F07 case={case} broken={broken} raw_wait={} result_bytes={}\n{log}",
        status.into_raw(),
        bytes.len()
    );
    assert!(bytes.is_empty());
    if case == "signal-death" {
        status.signal() == Some(libc::SIGTERM)
    } else {
        status.code() == Some(128 + libc::SIGTERM)
            && log.contains("ravel: cannot propagate SIGTERM:")
            && log.contains("os error 5")
    }
}
#[test]
fn native_f07_checks_each_exit_transition_and_real_signal_death() {
    for (name, id) in [
        (
            "default-omitted",
            "F07__operation=TERM-default__mode=omit-EIO",
        ),
        (
            "default-performed",
            "F07__operation=TERM-default__mode=perform-report-EIO",
        ),
        (
            "unblock-omitted",
            "F07__operation=TERM-unblock__mode=omit-EIO",
        ),
        (
            "unblock-performed",
            "F07__operation=TERM-unblock__mode=perform-report-EIO",
        ),
        ("raise-omitted", "F07__operation=TERM-raise__mode=omit-EIO"),
    ] {
        let held = case(name, false);
        crate::native_barrier::row(id, "repaired", held, name);
        assert!(held, "failed checked transition: {name}");
        let seen = case(name, true);
        crate::native_barrier::row(id, "broken:ignore-fault", seen, name);
        assert!(!seen, "broken transition control was not detected: {name}");
    }
    let death = case("signal-death", false);
    crate::native_barrier::row(
        "F07__operation=TERM-raise__mode=unmodified-signal-death-control",
        "repaired",
        death,
        "signal-death",
    );
    assert!(death);
}
