//! Exercise the operating-system boundary, not just the generated policy text.
mod support;

use std::fs;
use std::time::Duration;

use std::ffi::OsString;

use keyed_launch::{Argv, Channel, Escalation, Launch};

#[test]
fn confined_child_and_descendants_cannot_read_or_write_outside_the_invocation() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let secret = root.join("parent-project.txt");
    fs::write(&secret, "parent contents").unwrap();
    std::os::unix::fs::symlink(&secret, work.join("link")).unwrap();
    let script = work.join("probe.sh");
    fs::write(&script, format!(
        "set -eu\nif cat '{}' ; then exit 41; fi\nif cat link; then exit 42; fi\nif sh -c 'echo changed > \"$1\"' probe '{}'; then exit 43; fi\nprintf allowed > result\n: > \"$TEST_CHANNEL\"\n",
        secret.display(), secret.display()
    )).unwrap();
    let argv = Argv::new(OsString::from("/bin/sh"), vec![script.into_os_string()]);
    let channel = Channel::allocate(&work).unwrap();
    let ended = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "TEST_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: &[],
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(ended.status.success(), "sandbox failed: {:?}", ended.status);
    assert!(ended.signalled);
    assert_eq!(fs::read_to_string(secret).unwrap(), "parent contents");
    assert_eq!(fs::read_to_string(work.join("result")).unwrap(), "allowed");
}

#[test]
fn explicit_runtime_file_grant_allows_reads_but_denies_writes() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let runtime_file = root.join("runtime-credential.txt");
    fs::write(&runtime_file, "runtime contents").unwrap();
    let script = work.join("runtime-probe.sh");
    fs::write(
        &script,
        "set -eu\n\
         test \"$(cat \"$1\")\" = 'runtime contents'\n\
         if sh -c 'printf changed > \"$1\"' probe \"$1\"; then exit 41; fi\n\
         if sh -c 'rm \"$1\"' probe \"$1\"; then exit 42; fi\n\
         printf read-only > result\n\
         : > \"$TEST_CHANNEL\"\n",
    )
    .unwrap();
    let argv = Argv::new(
        OsString::from("/bin/sh"),
        vec![
            script.into_os_string(),
            runtime_file.clone().into_os_string(),
        ],
    );
    let channel = Channel::allocate(&work).unwrap();
    let ended = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "TEST_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: std::slice::from_ref(&runtime_file),
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(ended.status.success(), "sandbox failed: {:?}", ended.status);
    assert!(ended.signalled);
    assert_eq!(
        fs::read_to_string(runtime_file).unwrap(),
        "runtime contents"
    );
    assert_eq!(
        fs::read_to_string(work.join("result")).unwrap(),
        "read-only"
    );
}

/// A confined launch grants and runs the file its caller named. A name would
/// need a lookup, and the runner has none: a relative path refuses as a bare
/// name does, although `sh` is on PATH and the relative file exists.
#[test]
fn a_confined_program_that_is_not_an_absolute_path_refuses() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let marker = work.join("ran");
    fs::write(work.join("probe.sh"), "#!/bin/sh\n: > ran\n").unwrap();
    for program in ["sh", "./probe.sh", "work/probe.sh"] {
        let argv = Argv::new(OsString::from(program), vec![]);
        let channel = Channel::allocate(&work).unwrap();
        let error = keyed_launch::run_confined_observed(
            Launch {
                argv: &argv,
                channel: Some((&channel, "TEST_CHANNEL")),
                scrub: support::controls(),
                grant: &[],
                transparent: None,
                cwd: Some(&work),
                escalation: Escalation {
                    grace: Duration::ZERO,
                    kill_grace: Duration::from_millis(100),
                },
            },
            &keyed_launch::FilesystemGrants {
                writable: std::slice::from_ref(&work),
                runtime_read: &[],
            },
            &mut |_| {},
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("absolute program path"), "{error}");
        assert!(error.contains(program), "{error}");
        assert!(!marker.exists(), "{program} was launched");
    }
}
