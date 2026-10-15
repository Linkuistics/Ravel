//! Private post-teardown reports, exercised through the real choice and front.
//! Removing admission checks must fail success/missing-program cases; dropping
//! operational or superseded reports must fail the independent diagnostic cases.
use super::*;
use clap::Parser as _;
use std::cell::RefCell;

type ReportControl = Box<dyn FnOnce(&mut Teardown)>;
thread_local! {
    static REPORT: RefCell<Option<ReportControl>> = RefCell::new(None);
}

pub(super) fn reported_teardown(report: &mut Teardown, child: &mut Child) {
    if super::native_tests::active() {
        return;
    }
    if REPORT.with(|control| control.borrow().is_none()) {
        return;
    }
    if report.status.is_err() {
        // Emergency fixture cleanup is never substituted for the real receipt.
        if matches!(child.try_wait(), Ok(None)) {
            let _ = child.kill();
        }
        let emergency = wait_bounded(child, Duration::from_secs(2));
        eprintln!("emergency fixture wait: {emergency:?}");
        panic!("physical worker wait failed: {:?}", report.status);
    }
    assert!(
        report.removed.is_ok(),
        "physical cleanup failed: {:?}",
        report.removed
    );
    assert!(
        !report.directory.exists(),
        "physical worker directory remains"
    );
    eprintln!(
        "physical worker pid={} status={} directory={} removed=true",
        report.pid,
        report.status.as_ref().unwrap(),
        report.directory.display()
    );
    REPORT.with(|control| {
        if let Some(control) = control.borrow_mut().take() {
            control(report);
        }
    });
    eprintln!(
        "reported worker pid={} status={:?} removal={:?} operations={:?}",
        report.pid, report.status, report.removed, report.reports
    );
    eprintln!("END-REPORT-INPUT");
}

// Each fixture is a separate process, including global signals, HOME, output
// descriptors and the actual locate() path. Environment controls exist only in
// this ignored test role, never in the product or owner policy contract.
pub(super) fn cases(scenarios: &[&str], faults: &[&str], routes: &[&str]) {
    let exe = std::env::current_exe().unwrap();
    let worker = exe
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libexec/ravel/ravel-policy");
    assert!(worker.is_file(), "run task worker: {}", worker.display());
    for scenario in scenarios {
        for fault in faults {
            for route in routes {
                let root = tempfile::tempdir().unwrap();
                let bin = root.path().join("bin");
                let libexec = root.path().join("libexec/ravel");
                fs::create_dir_all(&bin).unwrap();
                fs::create_dir_all(&libexec).unwrap();
                let fixture = bin.join("fixture");
                fs::copy(&exe, &fixture).unwrap();
                fs::copy(&worker, libexec.join("ravel-policy")).unwrap();
                fs::create_dir(root.path().join("home")).unwrap();
                fs::create_dir(root.path().join("tmp")).unwrap();
                let started = Instant::now();
                let mut child = Command::new(&fixture)
                    .args([
                        "--exact",
                        "worker::admission_tests::worker_reap_admission_fixture",
                        "--ignored",
                        "--nocapture",
                    ])
                    .env_clear()
                    .env("PATH", "/usr/bin:/bin")
                    .env("HOME", root.path().join("home"))
                    .env("TMPDIR", root.path().join("tmp"))
                    .env("C2_SCENARIO", scenario)
                    .env("C2_FAULT", fault)
                    .env("C2_ROUTE", route)
                    .env("C2_FRONT", exe.ancestors().nth(2).unwrap().join("ravel"))
                    .current_dir(root.path())
                    // process_group(0) reserves the fixture PID as PGID (stable since Rust 1.64):
                    // https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.process_group
                    .process_group(0)
                    .stdout(Stdio::from(
                        fs::File::create(root.path().join("fixture-stdout")).unwrap(),
                    ))
                    .stderr(Stdio::from(
                        fs::File::create(root.path().join("fixture-stderr")).unwrap(),
                    ))
                    .spawn()
                    .unwrap();
                let status = match wait_bounded(&mut child, Duration::from_secs(8)) {
                    Ok(status) => status,
                    Err(error) => {
                        // The direct fixture is still unreaped on timeout and
                        // reserves this group identity for its policy workers.
                        if error.kind() == ErrorKind::TimedOut {
                            let pid = libc::pid_t::try_from(child.id()).unwrap();
                            unsafe { libc::kill(-pid, libc::SIGKILL) };
                        }
                        let emergency = wait_bounded(&mut child, Duration::from_secs(2));
                        panic!(
                            "fixture deadline/wait failure: {error}; emergency reap: {emergency:?}"
                        );
                    }
                };
                let output = std::process::Output {
                    status,
                    stdout: fs::read(root.path().join("fixture-stdout")).unwrap(),
                    stderr: fs::read(root.path().join("fixture-stderr")).unwrap(),
                };
                let log = String::from_utf8_lossy(&output.stderr);
                eprintln!("case scenario={scenario} fault={fault} route={route} status={} elapsed={:?}\n{log}", output.status, started.elapsed());
                assert!(
                    started.elapsed() < Duration::from_secs(10),
                    "fixture exceeded bound"
                );
                assert!(
                    output.status.success(),
                    "{scenario}/{fault}/{route}: {log}\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
                if scenario.starts_with("nofault-") {
                    assert!(
                        root.path().join("started").exists(),
                        "no-fault managed start missing"
                    );
                    let want = match *scenario {
                        "nofault-empty" => "",
                        "nofault-unicode" => "résultat 🪶",
                        _ => "first\nsecond\n",
                    };
                    assert_eq!(
                        fs::read(root.path().join("result-output")).unwrap(),
                        want.as_bytes()
                    );
                } else {
                    assert!(
                        !root.path().join("started").exists(),
                        "managed start escaped admission"
                    );
                    assert!(
                        fs::read(root.path().join("result-output"))
                            .unwrap()
                            .is_empty(),
                        "result stdout escaped failure"
                    );
                }
                if fault.starts_with("cancel-") || *scenario == "teardown-cancel" {
                    let log = log.split_once("END-REPORT-INPUT\n").unwrap().1;
                    for expected in [
                        "os error 10",
                        "os error 13",
                        "ravel-worker.",
                        "pid",
                        "retry",
                    ] {
                        assert!(
                            log.contains(expected),
                            "cancellation lost {expected}: {log}"
                        );
                    }
                    assert!(!log.contains("was stopped and reaped"));
                    match *scenario {
                        "refusal" => {
                            for expected in [
                                "owner_declined",
                                "owner original detail",
                                "owner original remedy",
                                "policy stdout: owner stdout",
                                "policy stderr: owner stderr",
                            ] {
                                assert!(log.contains(expected), "cancellation lost {expected}");
                            }
                        }
                        "timeout" => {
                            assert!(log.contains("selection_timeout") && log.contains("1000 ms"))
                        }
                        "timeout-overflow" => assert!(
                            log.contains("selection_timeout")
                                && log.contains("1000 ms")
                                && log.contains("output_limit"),
                            "deadline details lost across overflow: {log}"
                        ),
                        "eof-overflow" => assert!(
                            log.contains("without a result") && log.contains("output_limit"),
                            "EOF details lost across overflow: {log}"
                        ),
                        "overflow" => {
                            assert!(log.contains("output_limit") && log.contains("262144"))
                        }
                        "teardown-cancel" => assert!(
                            log.contains("policy stdout: owner stdout")
                                && log.contains("policy stderr: owner stderr")
                        ),
                        _ => unreachable!(),
                    }
                }
                assert_eq!(fs::read_dir(root.path().join("tmp")).unwrap().count(), 0);
            }
        }
    }
}

// Explicit names make nextest listing expose every fixed parameter case.
macro_rules! admission_case {
    ($name:ident, $scenario:literal, $fault:literal, $route:literal) => {
        #[test]
        fn $name() {
            cases(&[$scenario], &[$fault], &[$route]);
        }
    };
}

admission_case!(
    worker_reap_admission_success_gate_wait_choice_missing,
    "success",
    "wait",
    "choice-missing"
);
admission_case!(
    worker_reap_admission_success_gate_wait_choice_valid,
    "success",
    "wait",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_success_gate_wait_front,
    "success",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_success_gate_remove_choice_missing,
    "success",
    "remove",
    "choice-missing"
);
admission_case!(
    worker_reap_admission_success_gate_remove_choice_valid,
    "success",
    "remove",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_success_gate_remove_front,
    "success",
    "remove",
    "front"
);
admission_case!(
    worker_reap_admission_success_gate_both_choice_missing,
    "success",
    "both",
    "choice-missing"
);
admission_case!(
    worker_reap_admission_success_gate_both_choice_valid,
    "success",
    "both",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_success_gate_both_front,
    "success",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_refusal_wait,
    "refusal",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_refusal_both,
    "refusal",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_timeout_wait,
    "timeout",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_timeout_both,
    "timeout",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_protocol_wait,
    "protocol",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_protocol_both,
    "protocol",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_eof_wait,
    "eof",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_eof_both,
    "eof",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_overflow_wait,
    "overflow",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_overflow_both,
    "overflow",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_pre_ready_timeout_wait,
    "pre-ready-timeout",
    "wait",
    "front"
);
admission_case!(
    worker_reap_admission_primary_reports_pre_ready_timeout_both,
    "pre-ready-timeout",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_recovered_poll_refusal,
    "refusal",
    "poll",
    "front"
);
admission_case!(
    worker_reap_admission_recovered_poll_success,
    "success",
    "poll",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_nonzero_confirmed_wait,
    "nonzero",
    "none",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_cancellation_during_real_teardown,
    "teardown-cancel",
    "both",
    "front"
);
admission_case!(
    worker_reap_admission_cancellation_int,
    "refusal",
    "cancel-int",
    "front"
);
admission_case!(
    worker_reap_admission_cancellation_term,
    "timeout",
    "cancel-term",
    "front"
);
admission_case!(
    worker_reap_admission_cancellation_hup,
    "overflow",
    "cancel-hup",
    "front"
);
admission_case!(
    worker_reap_admission_late_choice_cancellation,
    "late-refusal",
    "both",
    "choice-valid"
);
admission_case!(
    worker_reap_admission_nofault_empty,
    "nofault-empty",
    "none",
    "front"
);
admission_case!(
    worker_reap_admission_nofault_unicode,
    "nofault-unicode",
    "none",
    "front"
);
admission_case!(
    worker_reap_admission_nofault_multiline,
    "nofault-multiline",
    "none",
    "front"
);
admission_case!(
    worker_reap_admission_cancellation_timeout_overflow,
    "timeout-overflow",
    "cancel-term",
    "front"
);
admission_case!(
    worker_reap_admission_cancellation_eof_overflow,
    "eof-overflow",
    "cancel-hup",
    "front"
);

fn selected(program: &str, args: Vec<Value>) -> Value {
    json!({"status":"selected", "program":program, "args":args,
        "provider":"fixture", "model":"fixture", "effort":"fixture", "reason":"admission"})
}

#[test]
#[ignore = "process-isolated role invoked by worker_reap_admission cases"]
fn worker_reap_admission_fixture() {
    let scenario = std::env::var("C2_SCENARIO").unwrap();
    let fault = std::env::var("C2_FAULT").unwrap();
    let route = std::env::var("C2_ROUTE").unwrap();
    if scenario == "native-reader" {
        super::native_tests::configure(&fault);
    }
    let root = std::env::current_dir().unwrap();
    let missing = root.join("missing-program");
    let program = if route == "choice-missing" {
        missing.to_str().unwrap()
    } else {
        "/bin/sh"
    };
    let mut command = selected(
        program,
        vec![
            "-c".into(),
            format!("echo started > {:?}", root.join("started")).into(),
        ],
    );
    if scenario.starts_with("nofault-") {
        let value = match scenario.as_str() {
            "nofault-empty" => "",
            "nofault-unicode" => "résultat 🪶",
            _ => "first\nsecond\n",
        };
        command = selected(
            "/bin/sh",
            vec![
                "-c".into(),
                "echo started > \"$1\"; \"$2\" end-session -- \"$3\"; exit 7".into(),
                "fixture".into(),
                root.join("started").to_string_lossy().into_owned().into(),
                std::env::var("C2_FRONT").unwrap().into(),
                value.into(),
            ],
        );
    }
    let body = match scenario.as_str() {
        "refusal" | "late-refusal" => "return {status:'refused', code:'owner_declined', message:'owner original detail', remedy:'owner original remedy'};".to_owned(),
        "timeout" => "for (;;) {}".to_owned(),
        "overflow" => "console.log('x'.repeat(1048576)); for (;;) {}".to_owned(),
        _ => format!("return {command};"),
    };
    let policy = root.join("policy.ts");
    fs::write(&policy, format!("console.log('owner stdout'); console.error('owner stderr'); export const policy = {{schemaVersion:2, version:'fixture', select(){{{body}}}}};")).unwrap();
    if [
        "protocol",
        "eof",
        "pre-ready-timeout",
        "nonzero",
        "teardown-cancel",
        "timeout-overflow",
        "eof-overflow",
    ]
    .contains(&scenario.as_str())
    {
        let worker = root.join("libexec/ravel/ravel-policy");
        fs::write(&worker, format!("#!/bin/sh\nC2_WORKER_MODE='{scenario}' exec '{}' --exact worker::admission_tests::worker_reap_admission_protocol_fixture --ignored --nocapture\n", std::env::current_exe().unwrap().display())).unwrap();
        fs::set_permissions(worker, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let config = root.join("home/.config/ravel");
    fs::create_dir_all(&config).unwrap();
    fs::write(config.join("settings.json"), r#"{"timeoutMs":1000}"#).unwrap();
    let cli = crate::cli::Cli::try_parse_from([
        "ravel",
        "start-session",
        "--kind",
        "check",
        "--prompt",
        "input",
        "--config",
        policy.to_str().unwrap(),
        "--json",
    ])
    .unwrap();
    let crate::cli::Command::StartSession(args) = cli.command else {
        unreachable!()
    };
    let injected = fault.clone();
    REPORT.with(|control| {
        *control.borrow_mut() = Some(Box::new(move |report| {
            if scenario == "nonzero" {
                assert_eq!(report.status.as_ref().unwrap().code(), Some(17));
            }
            if ["wait", "both", "cancel-int", "cancel-term", "cancel-hup"]
                .contains(&injected.as_str())
            {
                report.status = Err(io::Error::from_raw_os_error(libc::ECHILD));
            }
            if ["remove", "both", "cancel-int", "cancel-term", "cancel-hup"]
                .contains(&injected.as_str())
            {
                report.removed = Err(io::Error::from_raw_os_error(libc::EACCES));
            }
            if injected == "poll" {
                report
                    .reports
                    .push("try_wait: injected recoverable poll error (os error 5)".into());
            }
            let signal = match injected.as_str() {
                "cancel-int" => Some(libc::SIGINT),
                "cancel-term" => Some(libc::SIGTERM),
                "cancel-hup" => Some(libc::SIGHUP),
                _ => None,
            };
            if let Some(signal) = signal {
                // SAFETY: the fixture installed production handlers in this process.
                assert_eq!(unsafe { libc::raise(signal) }, 0);
                assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);
            }
        }))
    });
    let scenario = std::env::var("C2_SCENARIO").unwrap();
    // Capture only product result stdout; libtest has its own surrounding text.
    io::stdout().flush().unwrap();
    let saved = unsafe { libc::fcntl(1, libc::F_DUPFD_CLOEXEC, 10) };
    assert!(saved >= 0);
    let output = fs::File::create(root.join("result-output")).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
    let outcome = if route == "front" {
        crate::run::run(&args)
    } else {
        let selected =
            crate::choice::choose_prepared(crate::choice::prepare(&args.selection).unwrap());
        eprintln!("Selected={}", selected.is_ok());
        if scenario == "success" && fault == "poll" {
            let reports = &selected.as_ref().unwrap().choice.operational;
            assert!(reports
                .iter()
                .any(|report| report.contains("injected recoverable poll error")
                    && report.contains("confirmed reap")));
        }
        selected.map(|_| ())
    };
    io::stdout().flush().unwrap();
    assert_eq!(unsafe { libc::dup2(saved, 1) }, 1);
    unsafe { libc::close(saved) };
    drop(output);
    if scenario == "native-reader" {
        let failure = outcome.expect_err("reader failure admitted a selection");
        let text = failure.to_text();
        eprintln!("native reader {fault}: {text}");
        assert!(text.contains("cannot admit diagnostic readers"));
        assert!(text.contains("reader") && text.contains("pid"));
        match fault.as_str() {
            "stop-omitted" | "stop-performed" => {
                assert!(text.contains("stop pid") && text.contains("os error 1"))
            }
            "kill-omitted" | "kill-performed" => {
                assert!(text.contains("kill pid") && text.contains("os error 1"))
            }
            "poll-EIO" => assert!(text.contains("try_wait pid") && text.contains("os error 5")),
            "poll-ECHILD" => assert!(text.contains("os error 10") && text.contains("unconfirmed")),
            "poll-EINTR" => assert!(text.contains("deadline") && text.contains("EINTR retries")),
            "always-running" => assert!(text.contains("deadline")),
            "first-create" | "second-create" => {
                assert!(text.contains("os error 35") || text.contains("os error 11"))
            }
            _ => assert!(text.contains("os error 5")),
        }
        assert!(!root.join("started").exists());
        assert!(fs::read(root.join("result-output")).unwrap().is_empty());
        return;
    }
    if scenario.starts_with("nofault-") {
        assert!(outcome.is_ok(), "no-fault session failed: {outcome:?}");
        return;
    }
    if scenario == "success" && fault == "poll" {
        assert!(
            outcome.is_ok(),
            "recoverable poll blocked confirmed admission: {outcome:?}"
        );
        return;
    }
    if scenario == "nonzero" {
        assert!(
            outcome.is_ok(),
            "nonzero confirmed reap rejected: {outcome:?}"
        );
        assert_eq!(fs::read_dir(root.join("tmp")).unwrap().count(), 0);
        return;
    }
    let mut failure = outcome.expect_err("successful evaluation escaped required teardown gate");
    if scenario == "late-refusal" {
        let _handlers = cancellation::Handlers::install().unwrap();
        assert_eq!(unsafe { libc::raise(libc::SIGHUP) }, 0);
        failure = crate::choice::overruled(failure, policy.to_str().unwrap());
    }
    let (code, stage, exit) = if scenario == "teardown-cancel" {
        ("session_cancelled", "exec", 129)
    } else if fault.starts_with("cancel-") && route == "front" {
        (
            "session_cancelled",
            "exec",
            match fault.as_str() {
                "cancel-int" => 130,
                "cancel-term" => 143,
                _ => 129,
            },
        )
    } else if scenario == "late-refusal" {
        ("selection_cancelled", "evaluation", 129)
    } else {
        match scenario.as_str() {
            "refusal" => ("policy_refused", "selection", 3),
            "timeout" | "pre-ready-timeout" => ("selection_timeout", "evaluation", 124),
            "protocol" => ("protocol_error", "worker", 5),
            "overflow" => ("output_limit", "evaluation", 3),
            _ => ("worker_failed", "worker", 5),
        }
    };
    let document = failure.to_json();
    assert_eq!(document["error"]["code"], code);
    assert_eq!(document["error"]["stage"], stage);
    assert_eq!(document["error"]["exit"], exit);
    let text = failure.to_text();
    eprintln!("failure-json={document}\nfailure-text={text}");
    // Front cancellation emits the original failure through settle's actual
    // stderr boundary. The parent checks that stream, rather than a new path.
    if !fault.starts_with("cancel-") && scenario != "teardown-cancel" {
        if fault != "none" {
            assert!(
                text.contains("worker") && text.contains("pid"),
                "worker identity lost: {text}"
            );
        }
        if ["wait", "both"].contains(&fault.as_str()) {
            assert!(
                text.contains("wait") && text.contains("os error 10"),
                "wait report lost: {text}"
            );
            assert!(
                !text.contains("was stopped"),
                "false stopped/reap claim: {text}"
            );
        }
        if ["remove", "both"].contains(&fault.as_str()) {
            assert!(
                text.contains("ravel-worker.") && text.contains("os error 13"),
                "cleanup report lost: {text}"
            );
            assert!(
                text.contains("remove") && text.contains("retry"),
                "cleanup remedy lost: {text}"
            );
        }
        if fault == "poll" {
            assert!(
                text.contains("injected recoverable poll error"),
                "recovered report lost: {text}"
            );
        }
        if ["refusal", "late-refusal"].contains(&scenario.as_str()) {
            assert!(
                text.contains("owner original detail") && text.contains("owner original remedy"),
                "original refusal lost: {text}"
            );
            assert!(text.contains("owner_declined"), "owner code lost: {text}");
            if scenario == "refusal" {
                assert_eq!(document["error"]["policyCode"], "owner_declined");
            }
        }
        if ["timeout", "pre-ready-timeout"].contains(&scenario.as_str()) {
            assert_eq!(document["error"]["bound"]["ms"], 1000);
            assert!(document["error"]["input"].is_string());
            assert!(document["error"]["source"].is_string());
        }
        if scenario == "eof" {
            assert!(text.contains("without a result"));
        }
        if scenario == "protocol" {
            assert!(text.contains("malformed"));
        }
        let diagnostics = failure.diagnostics.as_ref().unwrap();
        assert!(diagnostics.stdout.len() + diagnostics.stderr.len() <= 262_144);
        assert!(
            !diagnostics.to_text().contains("os error"),
            "operations contaminated policy output"
        );
        if !["pre-ready-timeout", "eof", "protocol", "overflow"].contains(&scenario.as_str()) {
            assert!(
                text.contains("policy stdout: owner stdout")
                    && text.contains("policy stderr: owner stderr")
            );
        }
    }
    assert!(!root.join("started").exists());
    assert_eq!(fs::read_dir(root.join("tmp")).unwrap().count(), 0);
}

#[test]
#[ignore = "fake protocol worker process, invoked only by the isolated fixture"]
fn worker_reap_admission_protocol_fixture() {
    let mode = std::env::var("C2_WORKER_MODE").unwrap();
    if mode == "pre-ready-timeout" {
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
    if mode == "timeout-overflow" {
        unsafe { libc::signal(libc::SIGTERM, libc::SIG_IGN) };
        thread::sleep(Duration::from_millis(1200));
        io::stdout().write_all(&vec![b'x'; 1_048_576]).unwrap();
        std::process::exit(17);
    }
    let mut channel = unsafe { UnixStream::from_raw_fd(CHANNEL_FD) };
    if mode == "eof-overflow" {
        io::stdout().write_all(&vec![b'x'; 1_048_576]).unwrap();
        return;
    }
    if mode == "protocol" {
        channel.write_all(&[0, 0, 0, 1, b'!']).unwrap();
        return;
    }
    if mode == "eof" {
        return;
    }
    write_frame(&mut channel, &json!({"type":"hello", "protocol":1, "packageVersion":PACKAGE_VERSION, "buildId":BUILD_ID, "bunVersion":"1.4.2"})).unwrap();
    read_frame(&mut channel, 1_048_576).unwrap();
    println!("owner stdout");
    eprintln!("owner stderr");
    write_frame(&mut channel, &json!({"type":"policy", "policy":{"schemaVersion":2,"version":"fixture","select":{"$ravel":"function"}}})).unwrap();
    read_frame(&mut channel, 1_048_576).unwrap();
    write_frame(
        &mut channel,
        &json!({"type":"selection", "result":selected("/bin/sh", vec!["-c".into(), "exit 0".into()])}),
    )
    .unwrap();
    let mut byte = [0];
    assert_eq!(channel.read(&mut byte).unwrap(), 0);
    if mode == "teardown-cancel" {
        // The protocol is actually closed; cancellation reaches the front
        // while stop() is still polling this unreaped worker.
        assert_eq!(unsafe { libc::kill(libc::getppid(), libc::SIGHUP) }, 0);
        thread::sleep(Duration::from_millis(50));
    }
    std::process::exit(17);
}

fn wait_bounded(child: &mut Child, bound: Duration) -> io::Result<ExitStatus> {
    let deadline = Instant::now() + bound;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            return Err(ErrorKind::TimedOut.into());
        }
        thread::sleep(Duration::from_millis(5));
    }
}

// A control for the fixture's own timeout/emergency-reap instrument.
#[test]
fn worker_reap_admission_fixture_deadline() {
    let started = Instant::now();
    let mut child = Command::new("/bin/sleep")
        .arg("30")
        .process_group(0)
        .spawn()
        .unwrap();
    let error = wait_bounded(&mut child, Duration::from_millis(50)).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::TimedOut);
    let pid = libc::pid_t::try_from(child.id()).unwrap();
    // The unreaped fixture reserves the group while emergency KILL is sent.
    assert_eq!(unsafe { libc::kill(-pid, libc::SIGKILL) }, 0);
    let status = wait_bounded(&mut child, Duration::from_secs(2)).unwrap();
    assert!(!status.success());
    assert!(started.elapsed() < Duration::from_secs(3));
    eprintln!(
        "fixture deadline detected; emergency group cleanup and real reap confirmed: {status}"
    );
}
