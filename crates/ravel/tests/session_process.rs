//! Real processes exercise inherited context and the existing runner's reap.
use std::fs;
use std::process::Command;
use std::time::Duration;

use keyed_launch::{Argv, End, Escalation, Group, Launch};
use ravel::{end_session, SessionChannel, EXIT_FILE_VARIABLE};

#[path = "../../keyed-launch/tests/support/mod.rs"]
mod support;

fn fixture(role: &str, root: &std::path::Path) -> Command {
    let mut command = support::command(std::env::current_exe().unwrap());
    command.args([role, "--exact", "--ignored", "--nocapture"]);
    command.env("RAVEL_TEST_ROOT", root);
    command
}

fn successful(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn opaque_results_return_after_the_signalling_process_is_reaped() {
    for result in ["ordinary", "", "one\ntwo\n", "λ 🧶"] {
        let root = tempfile::tempdir().unwrap();
        successful(fixture("supervisor_fixture", root.path()).env("RAVEL_TEST_RESULT", result));
        assert_eq!(
            fs::read(root.path().join("returned")).unwrap(),
            result.as_bytes()
        );
        let path = fs::read_to_string(root.path().join("channel-path")).unwrap();
        assert!(!std::path::Path::new(&path).parent().unwrap().exists());
        let pid: i32 = fs::read_to_string(root.path().join("child-pid"))
            .unwrap()
            .parse()
            .unwrap();
        // SAFETY: signal zero queries liveness, never delivers a signal.
        assert_eq!(
            unsafe { libc::kill(pid, 0) },
            -1,
            "returned beside a living child"
        );
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH)
        );
    }
}

#[test]
fn nested_session_signals_only_its_own_inherited_context() {
    let root = tempfile::tempdir().unwrap();
    successful(
        fixture("supervisor_fixture", root.path())
            .env("RAVEL_TEST_RESULT", "outer")
            .env("RAVEL_TEST_NESTED", "1"),
    );
    assert_eq!(
        fs::read_to_string(root.path().join("returned")).unwrap(),
        "outer"
    );
    assert_eq!(
        fs::read_to_string(root.path().join("nested/returned")).unwrap(),
        "inner"
    );
    let outer = fs::read_to_string(root.path().join("channel-path")).unwrap();
    let inner = fs::read_to_string(root.path().join("nested/channel-path")).unwrap();
    assert_ne!(outer, inner);
}

#[test]
fn end_session_refuses_missing_empty_and_stale_inherited_context() {
    let root = tempfile::tempdir().unwrap();
    successful(&mut fixture("missing_context_fixture", root.path()));
    successful(fixture("missing_context_fixture", root.path()).env(EXIT_FILE_VARIABLE, ""));
    let channel = SessionChannel::allocate(root.path()).unwrap();
    let stale = channel.channel().path().to_owned();
    channel.close().unwrap();
    successful(fixture("missing_context_fixture", root.path()).env(EXIT_FILE_VARIABLE, stale));
}

#[test]
fn a_child_that_exits_without_signalling_has_no_opaque_result() {
    let root = tempfile::tempdir().unwrap();
    successful(fixture("supervisor_fixture", root.path()).env("RAVEL_TEST_NO_SIGNAL", "1"));
    assert!(root.path().join("no-result").exists());
    assert!(!root.path().join("returned").exists());
}

#[test]
fn cancellation_reaps_the_child_and_discards_an_already_published_result() {
    let root = tempfile::tempdir().unwrap();
    let mut command = fixture("supervisor_fixture", root.path());
    command
        .env("RAVEL_TEST_RESULT", "must not return")
        .env("RAVEL_TEST_CANCEL", "1");
    let mut supervisor = command.stdout(std::process::Stdio::null()).spawn().unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        let published = fs::read_to_string(root.path().join("channel-path"))
            .is_ok_and(|path| std::path::Path::new(&path).exists());
        if published {
            break;
        }
        if std::time::Instant::now() >= deadline {
            // SAFETY: signal this fixture's live child; runner owns its cleanup.
            unsafe {
                libc::kill(supervisor.id() as i32, libc::SIGTERM);
            }
            let _ = supervisor.wait();
            panic!("fixture did not publish before deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    // SAFETY: this is the live supervisor just spawned by the test.
    assert_eq!(
        unsafe { libc::kill(supervisor.id() as i32, libc::SIGTERM) },
        0
    );
    assert!(supervisor.wait().unwrap().success());
    assert!(root.path().join("cancelled").exists());
    assert!(!root.path().join("returned").exists());
}

#[test]
#[ignore = "subprocess supervisor fixture"]
fn supervisor_fixture() {
    let root = std::path::PathBuf::from(std::env::var_os("RAVEL_TEST_ROOT").unwrap());
    let channel = SessionChannel::allocate(&root).unwrap();
    fs::write(
        root.join("channel-path"),
        channel.channel().path().as_os_str().as_encoded_bytes(),
    )
    .unwrap();
    let argv = Argv::new(
        std::env::current_exe().unwrap().into_os_string(),
        ["child_fixture", "--exact", "--ignored", "--nocapture"]
            .into_iter()
            .map(Into::into)
            .collect(),
    );
    let ended = keyed_launch::run(Launch {
        argv: &argv,
        channel: Some((channel.channel(), EXIT_FILE_VARIABLE)),
        scrub: support::controls(),
        grant: &[],
        transparent: None,
        cwd: Some(&root),
        escalation: Escalation {
            grace: if std::env::var_os("RAVEL_TEST_CANCEL").is_some() {
                Duration::from_secs(10)
            } else {
                Duration::from_millis(20)
            },
            kill_grace: Duration::from_millis(20),
        },
    })
    .unwrap();
    assert_eq!(ended.group, Group::Gone);
    if std::env::var_os("RAVEL_TEST_CANCEL").is_some() {
        assert_eq!(
            ended.end,
            End::Interrupted {
                signal: libc::SIGTERM
            }
        );
        assert_eq!(
            channel.result_after(&ended).unwrap_err().kind(),
            std::io::ErrorKind::Interrupted
        );
        fs::write(root.join("cancelled"), "").unwrap();
    } else if std::env::var_os("RAVEL_TEST_NO_SIGNAL").is_some() {
        assert!(!ended.signalled);
        assert_eq!(channel.result_after(&ended).unwrap(), None);
        fs::write(root.join("no-result"), "").unwrap();
    } else {
        assert!(ended.signalled);
        assert_eq!(ended.end, End::Escalated);
        fs::write(
            root.join("returned"),
            channel.result_after(&ended).unwrap().unwrap(),
        )
        .unwrap();
    }
    channel.close().unwrap();
}

#[test]
#[ignore = "managed process fixture"]
fn child_fixture() {
    let root = std::path::PathBuf::from(std::env::var_os("RAVEL_TEST_ROOT").unwrap());
    fs::write(root.join("child-pid"), std::process::id().to_string()).unwrap();
    if std::env::var_os("RAVEL_TEST_NO_SIGNAL").is_some() {
        return;
    }
    if std::env::var_os("RAVEL_TEST_NESTED").is_some() {
        let outer = std::env::var_os(EXIT_FILE_VARIABLE).unwrap();
        let nested_root = root.join("nested");
        fs::create_dir(&nested_root).unwrap();
        let mut nested = fixture("supervisor_fixture", &nested_root);
        // This is the inherited enclosing session, deliberately restored after
        // the fixture boundary scrub. The inner runner must replace it.
        nested
            .env(EXIT_FILE_VARIABLE, &outer)
            .env("RAVEL_TEST_RESULT", "inner")
            .env_remove("RAVEL_TEST_NESTED");
        successful(&mut nested);
        assert!(
            !std::path::Path::new(&outer).exists(),
            "inner ended its outer session"
        );
        assert_eq!(std::env::var_os(EXIT_FILE_VARIABLE).unwrap(), outer);
    }
    end_session(&std::env::var("RAVEL_TEST_RESULT").unwrap()).unwrap();
    // Remain alive after publication so the actual runner must escalate/reap.
    loop {
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[test]
#[ignore = "missing-context fixture"]
fn missing_context_fixture() {
    for name in support::CONTROL_ENV {
        if *name != EXIT_FILE_VARIABLE {
            assert!(
                std::env::var_os(name).is_none(),
                "fixture inherited authority {name}"
            );
        }
    }
    assert!(end_session("anything").is_err());
}
