//! Allocator-local controls; only this ignored role reads fixture environment.
use std::fs;
use std::io;
use std::os::unix::process::CommandExt as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::directory_control::{Control, CONTROL};

fn case(role: &str, step: &str, mutation: &str, route: &str) {
    let exe = std::env::current_exe().unwrap();
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("bin")).unwrap();
    fs::create_dir_all(root.path().join("libexec/ravel")).unwrap();
    fs::create_dir(root.path().join("home")).unwrap();
    fs::create_dir(root.path().join("tmp")).unwrap();
    let fixture = root.path().join("bin/fixture");
    fs::copy(&exe, &fixture).unwrap();
    fs::copy(
        exe.ancestors()
            .nth(3)
            .unwrap()
            .join("libexec/ravel/ravel-policy"),
        root.path().join("libexec/ravel/ravel-policy"),
    )
    .unwrap();
    let mut child = Command::new(&fixture)
        .args([
            "--exact",
            "partial_acquisition_tests::partial_acquisition_cleanup_fixture",
            "--ignored",
            "--nocapture",
        ])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", root.path().join("home"))
        .env("TMPDIR", root.path().join("tmp"))
        .env("C3_ROLE", role)
        .env("C3_STEP", step)
        .env("C3_MUTATION", mutation)
        .env("C3_ROUTE", route)
        .env("C3_FRONT", exe.ancestors().nth(2).unwrap().join("ravel"))
        .current_dir(root.path())
        .process_group(0)
        .stdout(Stdio::from(
            fs::File::create(root.path().join("stdout")).unwrap(),
        ))
        .stderr(Stdio::from(
            fs::File::create(root.path().join("stderr")).unwrap(),
        ))
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(8);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            unsafe {
                libc::kill(-i32::try_from(child.id()).unwrap(), libc::SIGKILL);
            }
            let reap = child.wait();
            panic!("fixture timed out; emergency reap={reap:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let log = fs::read_to_string(root.path().join("stderr")).unwrap();
    eprintln!(
        "CAPTURED-PRODUCT\n{}",
        fs::read_to_string(root.path().join("product-stderr")).unwrap_or_default()
    );
    eprintln!(
        "RECEIPTS\n{}",
        fs::read_to_string(root.path().join("physical-receipts")).unwrap_or_default()
    );
    eprintln!("case {role}/{step}/{mutation}/{route} status={status}\n{log}");
    assert!(
        status.success(),
        "{log}\n{}",
        fs::read_to_string(root.path().join("stdout")).unwrap()
    );
    if step != "none" {
        assert!(!root.path().join("work/started").exists());
        assert!(fs::read(root.path().join("result")).unwrap().is_empty());
    }
}

macro_rules! c3_case {
    ($name:ident,$role:literal,$step:literal,$mutation:literal,$route:literal) => {
        #[test]
        fn $name() {
            case($role, $step, $mutation, $route);
        }
    };
}
c3_case!(
    partial_acquisition_cleanup_confined_create_clean,
    "confined",
    "create",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_identity_clean,
    "confined",
    "identity",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_identity_remove,
    "confined",
    "identity",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_canonical_clean,
    "confined",
    "canonical",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_canonical_remove,
    "confined",
    "canonical",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_runtime_clean,
    "confined",
    "runtime",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_runtime_remove,
    "confined",
    "runtime",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_protected_clean,
    "confined",
    "protected",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_protected_remove,
    "confined",
    "protected",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_temporary_clean,
    "confined",
    "temporary",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_temporary_remove,
    "confined",
    "temporary",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_confined_partial_move,
    "confined",
    "temporary",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transferred_move,
    "confined",
    "cleanup",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_partial_replace,
    "confined",
    "temporary",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transferred_replace,
    "confined",
    "cleanup",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_partial_symlink,
    "confined",
    "temporary",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transferred_symlink,
    "confined",
    "cleanup",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_partial_unknown,
    "confined",
    "temporary",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transferred_unknown,
    "confined",
    "cleanup",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transferred_remove,
    "confined",
    "cleanup",
    "remove",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_confined_transfer_success,
    "confined",
    "none",
    "clean",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_create_clean,
    "session",
    "create",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_identity_clean,
    "session",
    "identity",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_identity_remove,
    "session",
    "identity",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_reader_clean,
    "session",
    "reader",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_reader_remove,
    "session",
    "reader",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_channel_clean,
    "session",
    "channel",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_channel_remove,
    "session",
    "channel",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_partial_move,
    "session",
    "reader",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transferred_move,
    "session",
    "cleanup",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_partial_replace,
    "session",
    "reader",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transferred_replace,
    "session",
    "cleanup",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_partial_symlink,
    "session",
    "reader",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transferred_symlink,
    "session",
    "cleanup",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_partial_unknown,
    "session",
    "reader",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transferred_unknown,
    "session",
    "cleanup",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transferred_remove,
    "session",
    "cleanup",
    "remove",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_session_transfer_success,
    "session",
    "none",
    "clean",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_create_clean,
    "worker",
    "create",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_identity_clean,
    "worker",
    "identity",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_identity_remove,
    "worker",
    "identity",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_pair_clean,
    "worker",
    "pair",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_pair_remove,
    "worker",
    "pair",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_duplicate_clean,
    "worker",
    "duplicate",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_duplicate_remove,
    "worker",
    "duplicate",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_descriptors_clean,
    "worker",
    "descriptors",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_descriptors_remove,
    "worker",
    "descriptors",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_spawn_missing_clean,
    "worker",
    "spawn_missing",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_spawn_missing_remove,
    "worker",
    "spawn_missing",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_spawn_other_clean,
    "worker",
    "spawn_other",
    "clean",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_spawn_other_remove,
    "worker",
    "spawn_other",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_worker_partial_move,
    "worker",
    "pair",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transferred_move,
    "worker",
    "cleanup",
    "move",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_partial_replace,
    "worker",
    "pair",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transferred_replace,
    "worker",
    "cleanup",
    "replace",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_partial_symlink,
    "worker",
    "pair",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transferred_symlink,
    "worker",
    "cleanup",
    "symlink",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_partial_unknown,
    "worker",
    "pair",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transferred_unknown,
    "worker",
    "cleanup",
    "unknown",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transferred_remove,
    "worker",
    "cleanup",
    "remove",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_transfer_success,
    "worker",
    "none",
    "clean",
    "direct"
);
c3_case!(
    partial_acquisition_cleanup_worker_cleanup_gate_choice_missing,
    "worker",
    "cleanup",
    "remove",
    "choice-missing"
);
c3_case!(
    partial_acquisition_cleanup_worker_cleanup_gate_choice_valid,
    "worker",
    "cleanup",
    "remove",
    "choice-valid"
);
c3_case!(
    partial_acquisition_cleanup_worker_cleanup_gate_front,
    "worker",
    "cleanup",
    "remove",
    "front"
);
c3_case!(
    partial_acquisition_cleanup_session_cancellation_int,
    "session",
    "reader",
    "remove",
    "cancel-int"
);
c3_case!(
    partial_acquisition_cleanup_session_cancellation_term,
    "session",
    "reader",
    "remove",
    "cancel-term"
);
c3_case!(
    partial_acquisition_cleanup_session_cancellation_hup,
    "session",
    "reader",
    "remove",
    "cancel-hup"
);
c3_case!(
    partial_acquisition_cleanup_session_cancellation_late,
    "session",
    "reader",
    "remove",
    "cancel-late"
);
c3_case!(
    partial_acquisition_cleanup_nofault_empty,
    "worker",
    "none",
    "clean",
    "nofault-empty"
);
c3_case!(
    partial_acquisition_cleanup_nofault_unicode,
    "worker",
    "none",
    "clean",
    "nofault-unicode"
);
c3_case!(
    partial_acquisition_cleanup_nofault_multiline,
    "worker",
    "none",
    "clean",
    "nofault-multiline"
);

#[test]
#[ignore = "isolated role invoked by partial_acquisition_cleanup cases"]
fn partial_acquisition_cleanup_fixture() {
    use clap::Parser as _;
    use std::io::Write as _;
    use std::os::fd::AsRawFd as _;
    let role = std::env::var("C3_ROLE").unwrap();
    let step = std::env::var("C3_STEP").unwrap();
    let mutation = std::env::var("C3_MUTATION").unwrap();
    let route = std::env::var("C3_ROUTE").unwrap();
    let root = std::env::current_dir().unwrap();
    let value = match route.as_str() {
        "nofault-empty" => "",
        "nofault-unicode" => "résultat 🪶",
        _ => "first\nsecond\n",
    };
    let program = if route == "choice-missing" {
        root.join("missing-program")
    } else {
        PathBuf::from("/bin/sh")
    };
    let command = serde_json::json!({"status":"selected","program":program,"args":["-c","echo started > \"$1\"; \"$2\" end-session -- \"$3\"; exit 7","fixture",root.join("work/started"),std::env::var("C3_FRONT").unwrap(),value],"provider":"fixture","model":"fixture","effort":"fixture","reason":"cleanup"});
    let policy_dir = root.join("owner");
    fs::create_dir(&policy_dir).unwrap();
    let policy = policy_dir.join("policy.ts");
    let body = if route.contains("refusal") {
        "return {status:'refused',code:'owner_declined',message:'owner original detail',remedy:'owner original remedy'};".to_owned()
    } else if route.contains("timeout") {
        "for (;;) {}".to_owned()
    } else if route.contains("overflow") {
        "console.log('x'.repeat(1048576)); for (;;) {}".to_owned()
    } else {
        format!("return {command};")
    };
    fs::write(&policy,format!("console.log('owner stdout'); console.error('owner stderr'); export const policy = {{schemaVersion:2,version:'fixture',select(){{{body}}}}};")).unwrap();
    if route.contains("protocol") || route.contains("nonzero") {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = if route.contains("protocol") {
            "protocol"
        } else {
            "nonzero"
        };
        let worker = root.join("libexec/ravel/ravel-policy");
        fs::write(&worker,format!("#!/bin/sh\nC2_WORKER_MODE='{mode}' exec '{}' --exact worker::admission_tests::worker_reap_admission_protocol_fixture --ignored --nocapture\n",std::env::current_exe().unwrap().display())).unwrap();
        fs::set_permissions(worker, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let cwd = root.join("work");
    fs::create_dir(&cwd).unwrap();
    let descriptors = descriptors();
    let mut words = vec![
        "ravel".to_owned(),
        "start-session".into(),
        "--kind".into(),
        "check".into(),
        "--prompt".into(),
        "input".into(),
        "--config".into(),
        policy.to_str().unwrap().into(),
        "--json".into(),
    ];
    if role == "session" {
        words.extend([
            "--exit-dir".into(),
            root.join("tmp").to_str().unwrap().into(),
        ]);
    }
    if step == "real-runtime" {
        words.extend([
            "--runtime-read".into(),
            root.join("work").to_str().unwrap().into(),
        ]);
    }
    if !route.starts_with("nofault") || route == "nofault-confined" {
        words.push("--confine".into());
    }
    if route == "nofault-confined" {
        words.extend(["--runtime-read".into(), std::env::var("C3_FRONT").unwrap()]);
    }
    if route.contains("timeout") {
        words.extend(["--timeout-ms".into(), "1000".into()]);
    }
    std::env::set_current_dir(&cwd).unwrap();
    let cli = crate::cli::Cli::try_parse_from(words).unwrap();
    let crate::cli::Command::StartSession(args) = cli.command else {
        unreachable!()
    };
    CONTROL.with(|c| {
        *c.borrow_mut() = Control {
            role: role.clone(),
            step: step.clone(),
            mutation: mutation.clone(),
            route: route.clone(),
            ..Control::default()
        }
    });
    io::stdout().flush().unwrap();
    let saved = unsafe { libc::fcntl(1, libc::F_DUPFD_CLOEXEC, 10) };
    assert!(saved >= 0);
    let output = fs::File::create(root.join("result")).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
    io::stderr().flush().unwrap();
    let saved_stderr = unsafe { libc::fcntl(2, libc::F_DUPFD_CLOEXEC, 10) };
    assert!(saved_stderr >= 0);
    let product_stderr = fs::File::create(root.join("product-stderr")).unwrap();
    assert_eq!(unsafe { libc::dup2(product_stderr.as_raw_fd(), 2) }, 2);
    let outcome: Result<(), String> = if route == "direct" && role == "session" {
        match crate::completion::SessionChannel::allocate(&root.join("tmp")) {
            Ok(channel) => channel.close().map_err(|e| e.to_string()),
            Err(error) => {
                let errno = if step == "channel-real" {
                    libc::ENOENT
                } else {
                    libc::EIO
                };
                assert_eq!(error.kind(), io::Error::from_raw_os_error(errno).kind());
                let mut source: Option<&(dyn std::error::Error + 'static)> = Some(&error);
                let mut found = false;
                while let Some(current) = source {
                    found |= current
                        .downcast_ref::<io::Error>()
                        .is_some_and(|e| e.raw_os_error() == Some(errno));
                    source = current.source();
                }
                assert!(found, "original errno vanished from source chain");
                Err(error.to_string())
            }
        }
    } else if route == "direct" && role == "confined" {
        let prepared = crate::choice::prepare(&args.selection).unwrap();
        crate::confinement::Confined::prepare(&prepared, None, &[])
            .map_err(|e| crate::refusal::Failure::from(e).to_text())
            .and_then(|c| c.remove())
    } else if route.starts_with("choice")
        || route.ends_with("late-choice")
        || (route == "direct" && role == "worker")
    {
        let selected =
            crate::choice::choose_prepared(crate::choice::prepare(&args.selection).unwrap());
        eprintln!("Selected={}", selected.is_ok());
        assert_eq!(
            selected.is_ok(),
            step == "none",
            "Selected admission mismatch"
        );
        selected.map(|_| ()).map_err(|e| {
            if route == "choice-missing" && step == "cleanup" {
                assert_eq!(
                    e.refusal.code, "worker_failed",
                    "resolution escaped failed worker admission"
                );
                assert_eq!(e.refusal.stage.as_str(), "worker");
                assert_eq!(e.refusal.exit, 5);
            }
            if route.ends_with("late-choice") {
                let _handlers = crate::cancellation::Handlers::install().unwrap();
                assert_eq!(unsafe { libc::raise(libc::SIGHUP) }, 0);
                let e = crate::choice::overruled(e, policy.to_str().unwrap());
                assert_eq!(e.refusal.code, "selection_cancelled");
                e.to_text()
            } else {
                e.to_text()
            }
        })
    } else {
        crate::run::run(&args).map_err(|e| {
            if route.starts_with("cancel") || route.contains("cancel-") {
                assert_eq!(e.refusal.code, "session_cancelled");
                let first = if route.contains("cancel-int") {
                    "SIGINT"
                } else if route.contains("cancel-hup") {
                    "SIGHUP"
                } else {
                    "SIGTERM"
                };
                assert_eq!(e.refusal.signal.unwrap().name(), first);
            } else {
                let expected = if role == "confined" {
                    "confinement_unusable"
                } else if role == "session" {
                    "session_channel_unavailable"
                } else if route.contains("refusal") {
                    "policy_refused"
                } else if route.contains("timeout") {
                    "selection_timeout"
                } else if route.contains("overflow") {
                    "output_limit"
                } else if route.contains("protocol") {
                    "protocol_error"
                } else if step == "spawn_missing" {
                    "worker_missing"
                } else {
                    "worker_failed"
                };
                assert_eq!(e.refusal.code, expected);
                assert_eq!(
                    e.refusal.stage.as_str(),
                    if role == "confined" {
                        "cli"
                    } else if role == "session" {
                        "exec"
                    } else if route.contains("timeout") || route.contains("overflow") {
                        "evaluation"
                    } else if route.contains("refusal") {
                        "selection"
                    } else {
                        "worker"
                    }
                );
                assert_eq!(
                    e.refusal.exit,
                    if role == "confined" {
                        2
                    } else if route.contains("timeout") {
                        124
                    } else if route.contains("refusal") || route.contains("overflow") {
                        3
                    } else {
                        5
                    }
                );
            }
            e.to_text()
        })
    };
    io::stdout().flush().unwrap();
    assert_eq!(unsafe { libc::dup2(saved, 1) }, 1);
    unsafe { libc::close(saved) };
    drop(output);
    io::stderr().flush().unwrap();
    assert_eq!(unsafe { libc::dup2(saved_stderr, 2) }, 2);
    unsafe { libc::close(saved_stderr) };
    drop(product_stderr);
    fs::write(
        root.join("physical-receipts"),
        crate::directory_control::receipts().join("\n"),
    )
    .unwrap();
    let product = fs::read_to_string(root.join("product-stderr")).unwrap();
    eprintln!("PRODUCT-OUTPUT\n{product}");
    if !route.starts_with("choice")
        && route != "direct"
        && !route.ends_with("late-choice")
        && !route.starts_with("nofault")
        && role != "confined"
    {
        CONTROL.with(|c| {
            let c = c.borrow();
            let outer = c
                .all_created
                .iter()
                .find(|(role, _)| role == "confined")
                .unwrap();
            assert_eq!(
                c.all_attempts
                    .iter()
                    .filter(|role| *role == "confined")
                    .count(),
                1,
                "outer cleanup missing/retried"
            );
            if route.starts_with("worker-") || route.starts_with("cancel-") {
                assert!(outer.1.is_dir());
                assert!(
                    product.contains("os error 16") && product.contains(outer.1.to_str().unwrap()),
                    "outer cleanup report lost: {product}"
                );
            } else {
                assert!(!outer.1.exists(), "outer confined directory remains");
            }
        });
    }
    if step == "none" {
        assert!(outcome.is_ok(), "{outcome:?}");
        if route.starts_with("nofault") {
            assert!(root.join("work/started").exists());
            assert_eq!(fs::read(root.join("result")).unwrap(), value.as_bytes());
        }
    } else {
        let error = format!("{product}\n{}", outcome.unwrap_err());
        eprintln!("PRODUCT-DIAGNOSTICS\n{error}");
        if route.starts_with("worker-") {
            for expected in ["os error 10", "os error 5", "worker", "pid", "retry"] {
                assert!(
                    error.contains(expected),
                    "lost wait/operational report {expected}"
                );
            }
            if route.contains("refusal") {
                for expected in [
                    "owner_declined",
                    "owner original detail",
                    "owner original remedy",
                ] {
                    assert!(error.contains(expected), "lost {expected}");
                }
            }
            if route.contains("timeout") {
                assert!(error.contains("1000 ms") && error.contains("selection_timeout"));
            }
            if route.contains("overflow") {
                assert!(error.contains("262144") && error.contains("output_limit"));
            }
            if route.contains("protocol") {
                assert!(error.contains("malformed"));
            }
            if !route.contains("protocol")
                && !route.contains("overflow")
                && !route.contains("nonzero")
            {
                assert!(
                    error.contains("policy stdout: owner stdout")
                        && error.contains("policy stderr: owner stderr")
                );
            }
        }
        CONTROL.with(|c| {
            let c = c.borrow();
            if step == "create" {
                assert!(c.paths.is_empty());
                assert!(!error.contains("remove_dir_all"));
                return;
            }
            assert_eq!(c.paths.len(), 1);
            let path = &c.paths[0];
            if step != "cleanup" {
                assert!(
                    error.contains("os error 5")
                        || step.starts_with("spawn")
                        || (step == "channel-real" && error.contains("os error 2"))
                        || (step == "real-runtime" && error.contains("regular file")),
                    "lost original acquisition: {error}"
                );
            }
            if step == "identity" {
                assert!(path.is_dir(), "unknown identity was deleted unsafely");
                assert!(
                    error.contains("identity")
                        && error.contains("inspect")
                        && error.contains("locate")
                );
                assert_eq!(c.attempts, 1, "explicit cleanup missing or retried");
            } else if mutation == "clean" {
                assert!(!path.exists(), "partial directory remains");
            } else {
                assert!(
                    error.contains(&path.display().to_string()),
                    "lost owned path: {error}"
                );
                assert!(
                    error.contains("inspect") && error.contains("locate"),
                    "lost manual cleanup advice: {error}"
                );
                assert_eq!(
                    c.attempts, 1,
                    "explicit cleanup missing or destructor retried"
                );
                match mutation.as_str() {
                    "remove" => {
                        assert!(
                            path.is_dir(),
                            "destructor silently retried reported removal failure"
                        );
                        assert!(error.contains("os error 13") && error.contains("remove_dir_all"));
                    }
                    "unknown" => {
                        assert!(path.is_dir());
                        assert!(error.contains("identity"));
                    }
                    "move" => {
                        assert!(!path.exists());
                        assert!(path.with_extension("moved").is_dir());
                    }
                    "replace" => {
                        assert_eq!(fs::read_to_string(path.join("unrelated")).unwrap(), "keep");
                        assert!(path.with_extension("moved").is_dir());
                    }
                    "symlink" => {
                        assert!(fs::symlink_metadata(path).unwrap().file_type().is_symlink());
                        assert_eq!(fs::read_to_string(path.join("unrelated")).unwrap(), "keep");
                        assert!(path.with_extension("moved").is_dir());
                    }
                    _ => unreachable!(),
                }
            }
        });
    }
    assert_eq!(
        descriptors,
        self::descriptors(),
        "acquired descriptors leaked"
    );
    // Uninjected parent fixture owns all residual original/replacement/moved debris.
    // Descriptors must already have closed before it removes them.
}

fn descriptors() -> Vec<i32> {
    (0..256)
        .filter(|&fd| unsafe { libc::fcntl(fd, libc::F_GETFD) } >= 0)
        .collect()
}
c3_case!(
    partial_acquisition_cleanup_confined_real_runtime_validation,
    "confined",
    "real-runtime",
    "clean",
    "front"
);

c3_case!(
    partial_acquisition_cleanup_session_real_channel_errno,
    "session",
    "channel-real",
    "move",
    "direct"
);

c3_case!(
    partial_acquisition_cleanup_worker_refusal_remove_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-refusal"
);

c3_case!(
    partial_acquisition_cleanup_worker_refusal_unknown_wait,
    "worker",
    "cleanup",
    "unknown",
    "worker-refusal"
);

c3_case!(
    partial_acquisition_cleanup_worker_refusal_cancel_int_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-refusal-cancel-int"
);

c3_case!(
    partial_acquisition_cleanup_worker_refusal_cancel_term_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-refusal-cancel-term"
);

c3_case!(
    partial_acquisition_cleanup_worker_refusal_cancel_hup_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-refusal-cancel-hup"
);

c3_case!(
    partial_acquisition_cleanup_worker_timeout_remove_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-timeout"
);

c3_case!(
    partial_acquisition_cleanup_worker_timeout_unknown_wait,
    "worker",
    "cleanup",
    "unknown",
    "worker-timeout"
);

c3_case!(
    partial_acquisition_cleanup_worker_timeout_cancel_int_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-timeout-cancel-int"
);

c3_case!(
    partial_acquisition_cleanup_worker_timeout_cancel_term_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-timeout-cancel-term"
);

c3_case!(
    partial_acquisition_cleanup_worker_timeout_cancel_hup_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-timeout-cancel-hup"
);

c3_case!(
    partial_acquisition_cleanup_worker_protocol_remove_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-protocol"
);

c3_case!(
    partial_acquisition_cleanup_worker_protocol_unknown_wait,
    "worker",
    "cleanup",
    "unknown",
    "worker-protocol"
);

c3_case!(
    partial_acquisition_cleanup_worker_protocol_cancel_int_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-protocol-cancel-int"
);

c3_case!(
    partial_acquisition_cleanup_worker_protocol_cancel_term_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-protocol-cancel-term"
);

c3_case!(
    partial_acquisition_cleanup_worker_protocol_cancel_hup_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-protocol-cancel-hup"
);

c3_case!(
    partial_acquisition_cleanup_worker_overflow_remove_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-overflow"
);

c3_case!(
    partial_acquisition_cleanup_worker_overflow_unknown_wait,
    "worker",
    "cleanup",
    "unknown",
    "worker-overflow"
);

c3_case!(
    partial_acquisition_cleanup_worker_overflow_cancel_int_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-overflow-cancel-int"
);

c3_case!(
    partial_acquisition_cleanup_worker_overflow_cancel_term_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-overflow-cancel-term"
);

c3_case!(
    partial_acquisition_cleanup_worker_overflow_cancel_hup_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-overflow-cancel-hup"
);

c3_case!(
    partial_acquisition_cleanup_worker_late_cancellation_wait,
    "worker",
    "cleanup",
    "remove",
    "worker-refusal-late-choice"
);
c3_case!(
    partial_acquisition_cleanup_worker_nonzero_cleanup_gate,
    "worker",
    "cleanup",
    "remove",
    "worker-nonzero"
);
c3_case!(
    partial_acquisition_cleanup_nofault_confined,
    "worker",
    "none",
    "clean",
    "nofault-confined"
);

// Restoring the owned inode to its original pathname makes a retry destructive,
// rather than letting repeated identity failure hide an armed destructor.
#[test]
fn partial_acquisition_cleanup_moved_path_destructor_disarmed() {
    let root = tempfile::tempdir().unwrap();
    let temp = tempfile::tempdir_in(root.path()).unwrap();
    CONTROL.with(|c| {
        *c.borrow_mut() = Control {
            role: "owner".into(),
            ..Control::default()
        }
    });
    let mut owner = crate::private_directory::PrivateDirectory::new(temp, "owner");
    owner.record_identity().unwrap();
    let path = owner.path().to_owned();
    let moved = root.path().join("moved");
    fs::write(path.join("owned"), "preserve").unwrap();
    fs::rename(&path, &moved).unwrap();
    assert!(owner.cleanup().is_err());
    fs::rename(&moved, &path).unwrap();
    drop(owner);
    assert_eq!(fs::read_to_string(path.join("owned")).unwrap(), "preserve");
    CONTROL.with(|c| assert_eq!(c.borrow().attempts, 1));
}
