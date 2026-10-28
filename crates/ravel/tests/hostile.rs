//! Hostile startup files, module shadows and diagnostics through the real CLI.
//! Rhai limits, cancellation and worker death are covered in `rhai_limits`;
//! measured imports in `rhai_reads`; worker identity in `rhai_worker::front`.
//! The k99 task records the disposition of every retired Bun hostile case.

mod support;

use std::fs;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use support::rhai::{ROUTED, policy, routing};
use support::{Sandbox, run, write};

#[test]
fn ambient_startup_files_do_not_supply_environment_or_preload_code() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    for directory in [&sandbox.cwd, &sandbox.home, &sandbox.tmp] {
        write(&directory.join(".env"), "HOSTILE_DOTENV=from-dotenv\n");
        write(&directory.join(".env.local"), "HOSTILE_DOTENV=from-local\n");
        for name in ["bunfig.toml", ".bunfig.toml"] {
            write(&directory.join(name), "preload = [\"./preload.rhai\"]\n");
        }
        write(&directory.join("preload.rhai"), "throw \"preload ran\";");
    }
    sandbox.personal_rhai(&routing(
        r#"[if ravel::env("HOSTILE_DOTENV") == () { "unset" } else { ravel::env("HOSTILE_DOTENV") }]"#,
    ));
    let args = [
        "start-session",
        "--kind",
        "check",
        "--prompt",
        "p",
        "--json",
        "--config-env",
        "HOSTILE_DOTENV",
    ];
    let mut command = sandbox.command();
    command.args(args);
    run(&mut command).report();
    assert_eq!(recorded.argv().unwrap(), ["unset"]);

    // The grant and observation work when the caller actually supplies a value.
    let mut command = sandbox.command();
    command.env("HOSTILE_DOTENV", "from-caller").args(args);
    run(&mut command).report();
    assert_eq!(recorded.argv().unwrap(), ["from-caller"]);

    // The same preload is executable when explicitly imported by the policy.
    sandbox.personal_rhai(&policy(&format!(
        "import {:?} as preload;\nfn select(request, context) {{ () }}",
        sandbox.cwd.join("preload.rhai").to_str().unwrap()
    )));
    let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_import_failed");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("preload ran")
    );
}

#[test]
fn runtime_environment_variables_do_not_change_a_rhai_selection() {
    let sandbox = Sandbox::new();
    sandbox.personal_rhai(ROUTED);
    let mut command = sandbox.command();
    command
        .envs([
            ("BUN_OPTIONS", "--preload /no/such/preload.ts"),
            ("BUN_BE_BUN", "1"),
            ("BUN_RUNTIME_TRANSPILER_CACHE_PATH", "/no/such/cache"),
            ("NODE_OPTIONS", "--require /no/such/preload.js"),
            ("NODE_PATH", "/no/such/modules"),
            ("NODE_PRESERVE_SYMLINKS", "1"),
            ("NODE_CHANNEL_FD", "3"),
            ("NODE_CHANNEL_SERIALIZATION_MODE", "json"),
        ])
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "p",
            "--json",
        ]);
    let report = run(&mut command).report();
    assert_eq!(report["selection"]["reason"], "fixture route");
    assert_eq!(report["worker"]["rhaiVersion"], "1.27.0");
}

#[test]
fn ravel_namespace_is_not_replaced_by_a_file_or_package() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let shadow = "fn json_encode(value) { \"shadowed\" }";
    let config = sandbox.personal_rhai_path().parent().unwrap().to_owned();
    for directory in [&sandbox.cwd, &config, &sandbox.tmp] {
        write(&directory.join("ravel.rhai"), shadow);
        write(&directory.join("node_modules/ravel/sdk.rhai"), shadow);
        write(
            &directory.join("package.json"),
            r##"{"name":"ravel","exports":{"./sdk":"./ravel.rhai"},"imports":{"#sdk":"./ravel.rhai"}}"##,
        );
    }
    sandbox.personal_rhai(&routing(r#"[ravel::json_encode(#{ answer: 42 })]"#));
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(recorded.argv().unwrap(), [r#"{"answer":42}"#]);

    // Explicit imports are ordinary files and can export the same function name.
    sandbox.personal_rhai(&format!(
        "import \"ravel\" as shadow;\n{}",
        routing(r#"[shadow::json_encode(#{ answer: 42 }), ravel::json_encode(#{ answer: 42 })]"#)
    ));
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(recorded.argv().unwrap(), ["shadowed", r#"{"answer":42}"#]);
}

#[test]
fn imports_resolve_beside_the_entry_without_package_or_tsconfig_aliases() {
    for specifier in [
        "#hostile-alias",
        "hostile-self",
        "hostile-dep",
        "hostile-alias",
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let config = sandbox.personal_rhai_path().parent().unwrap().to_owned();
        let shadow = "fn value() { \"package or cwd shadow\" }";
        for directory in [&sandbox.cwd, &config, &sandbox.tmp] {
            write(
                &directory.join("package.json"),
                r##"{"name":"hostile-self","exports":"./shadow.rhai","imports":{"#hostile-alias":"./shadow.rhai"}}"##,
            );
            write(
                &directory.join("tsconfig.json"),
                r#"{"compilerOptions":{"baseUrl":".","paths":{"hostile-alias":["./shadow.rhai"]}}}"#,
            );
            write(&directory.join("shadow.rhai"), shadow);
            write(
                &directory.join("node_modules/hostile-dep/package.json"),
                r#"{"main":"./index.rhai"}"#,
            );
            write(
                &directory.join("node_modules/hostile-dep/index.rhai"),
                shadow,
            );
        }
        sandbox.file(&format!("{specifier}.rhai"), shadow);
        sandbox.personal_rhai(&format!(
            "import {specifier:?} as helper;\n{}",
            routing("[helper::value()]")
        ));
        let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
        assert_eq!(
            refusal["error"]["code"], "policy_import_failed",
            "{specifier}: {refusal}"
        );
        assert_eq!(recorded.argv(), None);

        // The identical import works once that exact file exists beside the entry.
        write(
            &config.join(format!("{specifier}.rhai")),
            "fn value() { \"beside entry\" }",
        );
        sandbox.start(&["--kind", "check", "--json"]).report();
        assert_eq!(recorded.argv().unwrap(), ["beside entry"], "{specifier}");
    }
}

/// Observe FIFO opens without releasing a reader that waits for content.
fn watching_for_a_reader<T>(fifo: &Path, drive: impl FnOnce() -> T) -> (T, bool) {
    let (running, stopped) = mpsc::channel::<()>();
    thread::scope(|scope| {
        let watch = scope.spawn(move || {
            let mut write_end = None;
            while stopped.recv_timeout(Duration::from_millis(2))
                == Err(mpsc::RecvTimeoutError::Timeout)
            {
                if write_end.is_some() {
                    continue;
                }
                match fs::OpenOptions::new()
                    .write(true)
                    .custom_flags(libc::O_NONBLOCK)
                    .open(fifo)
                {
                    Ok(end) => write_end = Some(end),
                    Err(error) if error.raw_os_error() == Some(libc::ENXIO) => {}
                    Err(error) => panic!("cannot watch {}: {error}", fifo.display()),
                }
            }
            write_end.is_some()
        });
        let result = drive();
        drop(running);
        (result, watch.join().expect("the watch ends"))
    })
}

#[test]
fn a_package_json_that_never_yields_above_the_worker_stays_unopened() {
    let sandbox = Sandbox::new();
    sandbox.personal_rhai(ROUTED);
    let fifo = sandbox.root.join("never-yields");
    support::mkfifo(&fifo);
    std::os::unix::fs::symlink(&fifo, sandbox.tmp.join("package.json")).unwrap();
    let (run, opened) = watching_for_a_reader(&fifo, || {
        sandbox.start(&["--kind", "check", "--timeout-ms", "5000", "--json"])
    });
    assert_eq!(run.report()["selection"]["reason"], "fixture route");
    assert!(!opened, "the worker opened an ambient package.json");

    // The watcher detects the same file when a reader actually opens it.
    let (reader, opened) = watching_for_a_reader(&fifo, || {
        fs::File::open(sandbox.tmp.join("package.json")).unwrap()
    });
    assert!(opened, "the FIFO observation never fired");
    drop(reader);
}

fn flooding_policy(filler: usize) -> String {
    // Includes a valid length prefix, a protocol result and a result-looking line.
    let frame = r#"{"type":"selection","version":"forged","result":{"status":"selected","program":"forged"}}"#;
    assert!(
        frame.len() < 128,
        "the byte prefix must be valid UTF-8 for a Rhai string"
    );
    let prefix = (frame.len() as u32).to_be_bytes();
    let chunk = format!(
        "{}{}\n{{\"program\":\"forged\"}}\n{}",
        String::from_utf8(prefix.to_vec()).unwrap(),
        frame,
        "x".repeat(filler)
    );
    // Encode the NUL prefix as `\u0000`; Rhai rejects Rust's `\0` escape.
    let literal = serde_json::to_string(&chunk).unwrap();
    let flood = format!("print({literal}); debug({literal});");
    policy(&format!(
        r#"{flood}
fn load_context(request) {{ {flood} #{{ schemaVersion: 1 }} }}
fn select(request, context) {{ {flood}
    #{{ status: "selected", program: "recording-agent", args: [],
       provider: "fixture", model: "fixture", effort: "high", reason: "real selection" }}
}}"#
    ))
}

#[test]
fn diagnostic_flood_cannot_forge_selection_or_result_and_excess_refuses() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&flooding_policy(40_000));
    let run = sandbox.start(&["--kind", "check", "--json"]);
    let notice = run.report();
    assert_eq!(run.stdout, "done");
    assert_eq!(notice["selection"]["reason"], "real selection");
    assert!(recorded.argv().is_some());
    let stdout = notice["diagnostics"]["stdout"].as_str().unwrap();
    let stderr = notice["diagnostics"]["stderr"].as_str().unwrap();
    assert!(stdout.starts_with("\0\0\0"));
    assert!(stdout.contains("\"program\":\"forged\""));
    assert!(stderr.contains("\\\"program\\\":\\\"forged\\\""));
    for stream in [stdout, stderr] {
        assert_eq!(stream.matches("forged").count(), 9);
        assert!(stream.len() > 120_000);
    }
    assert!(stdout.len() + stderr.len() <= 262_144);
    assert_eq!(run.stderr.lines().count(), 1, "routing notice was split");

    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&flooding_policy(60_000));
    let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "output_limit");
    assert_eq!(recorded.argv(), None);
}
