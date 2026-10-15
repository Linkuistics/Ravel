//! A failed parent transition still belongs to the owner of the spawned job.
use std::error::Error as _;
use std::ffi::OsString;
use std::io;
use std::time::Duration;

use keyed_launch::{run_observed_with_start_check, Argv, Escalation, Launch, LaunchEvent};

#[test]
fn failed_start_check_retains_io_source_and_confirms_real_reap() {
    // Removing error settlement or reporting the callback as a spawn failure
    // must fail these public boundary assertions.
    let argv = Argv::new(OsString::from("/bin/sleep"), vec!["20".into()]);
    let mut events = Vec::new();
    let mut checks = 0;
    let before = std::time::Instant::now();
    let error = run_observed_with_start_check(
        Launch {
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
        },
        &mut |event| events.push(event),
        &mut || {
            checks += 1;
            Err(io::Error::from_raw_os_error(libc::EIO))
        },
    )
    .unwrap_err();
    assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);
    assert_eq!(checks, 1);
    assert_eq!(error.raw_os_error(), None);
    assert_eq!(
        error
            .source()
            .unwrap()
            .downcast_ref::<io::Error>()
            .unwrap()
            .raw_os_error(),
        Some(libc::EIO)
    );
    assert!(error.to_string().contains("Started"));
    assert!(before.elapsed() < Duration::from_secs(4));
}

#[test]
fn failed_spawn_never_calls_either_start_callback() {
    let argv = Argv::new(OsString::from("/ravel-missing-start-check-program"), vec![]);
    let mut events = Vec::new();
    let mut checks = 0;
    let error = run_observed_with_start_check(
        Launch {
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
        },
        &mut |event| events.push(event),
        &mut || {
            checks += 1;
            Ok(())
        },
    )
    .unwrap_err();
    assert_eq!(checks, 0);
    assert!(events.is_empty());
    assert_eq!(error.raw_os_error(), Some(libc::ENOENT));
}
