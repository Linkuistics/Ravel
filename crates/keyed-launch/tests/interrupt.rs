//! The launcher's own SIGTERM: where it is reported, and where it must not be.
//!
//! **One test function, deliberately.** The interrupt latch is process-global,
//! because a signal disposition is, so two test functions in one binary would
//! race each other for it. The phases below therefore run in order inside a
//! single test, and this file is its own integration-test binary so nothing
//! else shares the process.
//!
//! `raise(SIGTERM)` is only safe here *after* a first `run`, which is what
//! installs the handler. Before that the signal takes its default disposition
//! and would kill the test runner — so phase 1 exists to be run, not to assert.

mod support;

use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::time::Duration;

use keyed_launch::{
    run, run_observed, take_interrupt, Argv, Channel, End, Escalation, Launch, LaunchEvent,
};
use tempfile::TempDir;

const FAST: Escalation = Escalation {
    grace: Duration::from_millis(600),
    kill_grace: Duration::from_millis(900),
};

/// SIGTERM by number, so this file takes no `libc` dependency of its own for
/// the assertion side.
const SIGTERM: i32 = 15;

fn argv_for(dir: &Path, body: &str) -> Argv {
    let script = dir.join("child.sh");
    fs::write(&script, body).unwrap();
    Argv::new(OsString::from("sh"), vec![script.into_os_string()])
}

fn launch<'a>(argv: &'a Argv, channel: &'a Channel) -> Launch<'a> {
    Launch {
        argv,
        channel: Some((channel, "TEST_CHANNEL")),
        scrub: support::controls(),
        grant: &[],
        transparent: None,
        cwd: None,
        escalation: FAST,
    }
}

#[test]
fn an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other() {
    let dir = TempDir::new().unwrap();
    let control = dir.path().join("control");
    fs::create_dir(&control).unwrap();

    // Phase 1 — a launch, purely to install the handler. Raising SIGTERM before
    // this point would kill the test process outright.
    let quick = argv_for(dir.path(), "exit 0\n");
    let first = Channel::allocate(&control).unwrap();
    let ended = run(launch(&quick, &first)).unwrap();
    assert_eq!(ended.end, End::Exited);

    // Phase 2 — a signal arriving between launches belongs to the launcher, and
    // `take_interrupt` is where it collects it. Exactly once.
    raise_sigterm();
    assert_eq!(
        take_interrupt(),
        Some(SIGTERM),
        "a signal arriving outside a launch must be collectable, and must say which"
    );
    assert_eq!(
        take_interrupt(),
        None,
        "collecting the interrupt must consume it, or a launcher stops twice"
    );

    // Phase 3 — the case a latch that outlived its launch would get wrong. The
    // signal arrives while nothing is running and is *not* collected; the next
    // launch must not spend it on a fresh child that has done nothing.
    raise_sigterm();
    let untouched = Channel::allocate(&control).unwrap();
    let ended = run(launch(&quick, &untouched)).unwrap();
    assert_eq!(
        ended.end,
        End::Exited,
        "a stale interrupt was charged to a child that was never signalled"
    );
    assert!(
        ended.status.success(),
        "the child was killed for a signal that predated it: {:?}",
        ended.status
    );
    assert_eq!(
        take_interrupt(),
        None,
        "the launch must have consumed the stale latch rather than leaving it armed"
    );

    // Phase 4 — the case the latch is *for*: a signal arriving mid-launch is
    // forwarded to the child, which is reaped through the ordinary escalation
    // rather than orphaned onto the terminal.
    let patient = argv_for(dir.path(), "while : ; do sleep 0.05 ; done\n");
    let interrupted = Channel::allocate(&control).unwrap();
    let mut events = Vec::new();
    let ended = run_observed(launch(&patient, &interrupted), &mut |event| {
        events.push(event);
        if event == LaunchEvent::Started {
            raise_sigterm();
        }
    })
    .unwrap();
    assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);

    assert_eq!(
        ended.end,
        End::Interrupted { signal: SIGTERM },
        "a signal arriving during a launch is that launch's ending, and names itself"
    );
    assert!(!ended.signalled, "the interrupted child did not signal");
    assert_eq!(
        exit_signal(&ended),
        Some(SIGTERM),
        "the child must be forwarded the signal, not left running"
    );
    assert_eq!(
        take_interrupt(),
        None,
        "an interrupt reported as an ending must not also stop the next launch"
    );
}

fn raise_sigterm() {
    // SAFETY: `raise(3)` against a handler this crate installed, which performs
    // one relaxed atomic store.
    unsafe { libc::raise(libc::SIGTERM) };
}

fn exit_signal(ended: &keyed_launch::Ended) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt as _;
    ended.status.signal()
}
