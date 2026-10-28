//! `ravel::run_harness` through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *Harness runs*, *Re-entry and completion
//! authority*, *Harness failures are data* and *Resource exhaustion*).
//!
//! Everything runs in a sandbox home with fake harness scripts on a private
//! `PATH`. A policy runs a harness that the settings name, and then hands what
//! it got to the recording agent as arguments, so a test sees what the script
//! saw. The real worker and the real runner run in every test. Each case that
//! proves a stop or a refusal has a firing control: the same policy under a
//! change that makes it succeed.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::rhai::{self, Recorded, gone, policy};
use support::{FRONT, Run, Sandbox, run};

const SETTINGS: &str = ".config/ravel/settings.json";

/// The settings that name `harnesses`, each as its own program, and allow `runs`.
fn settings(sandbox: &Sandbox, harnesses: &[&str], runs: u64) {
    let entries: serde_json::Map<String, Value> = harnesses
        .iter()
        .map(|name| ((*name).to_owned(), json!({ "program": name })))
        .collect();
    let contents = json!({ "harnesses": entries, "budgets": { "harnessRuns": runs } });
    support::write(&sandbox.home.join(SETTINGS), &contents.to_string());
}

/// A harness script `name` on the private `PATH`.
fn harness(sandbox: &Sandbox, name: &str, body: &str) -> PathBuf {
    let script = sandbox.bin.join(name);
    support::write(&script, &format!("#!/bin/sh\n{body}\n"));
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    script
}

/// A policy whose `select` runs `request` and routes to the recording agent with
/// `args`, a Rhai array expression over `run`.
fn running(request: &str, args: &str) -> String {
    policy(&format!(
        r#"fn select(request, context) {{
    let run = ravel::run_harness({request});
    #{{ status: "selected", program: "recording-agent", args: {args}, provider: "p",
       model: "m", effort: "e", reason: "r" }}
}}"#
    ))
}

/// The whole result map, as the one argument of the recording agent.
const WHOLE: &str = "[ravel::json_encode(run)]";

/// The parts of a result that stay small however much a harness wrote.
const SUMMARY: &str = r#"[ravel::json_encode(#{
    status: run.status, exitCode: run.exitCode, signal: run.signal,
    stdoutBytes: run.stdout.len(), stderrBytes: run.stderr.len(), result: run.result,
    error: run.error })]"#;

fn start(sandbox: &Sandbox, args: &[&str]) -> Run {
    let mut all = vec!["--kind", "check", "--json"];
    all.extend_from_slice(args);
    sandbox.start(&all)
}

/// The result map that the recording agent was handed.
fn handed(recorded: &Recorded) -> Value {
    let args = recorded.argv().expect("the managed process started");
    assert_eq!(args.len(), 1, "{args:?}");
    serde_json::from_str(&args[0]).unwrap()
}

/// `start`, expecting the session to end with the value of the managed process.
fn ran(sandbox: &Sandbox, recorded: &Recorded, args: &[&str]) -> (Value, Value) {
    let notice = start(sandbox, args).report();
    (handed(recorded), notice)
}

fn refused(run: &Run, exit: i32, code: &str, recorded: &Recorded) -> Value {
    let document = run.refusal(exit);
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

/// A background process id that a harness wrote.
fn pid_of(file: &Path) -> i32 {
    fs::read_to_string(file)
        .unwrap_or_else(|error| panic!("{}: {error}", file.display()))
        .trim()
        .parse()
        .unwrap()
}

/// Nothing of Ravel's is left in the sandbox's private `TMPDIR`: no result
/// channel, no confined run directory.
fn assert_clean(sandbox: &Sandbox) {
    let left: Vec<_> = fs::read_dir(&sandbox.tmp)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("ravel-"))
        .collect();
    assert!(left.is_empty(), "left in TMPDIR: {left:?}");
}

#[test]
fn a_run_gets_its_arguments_and_input_and_returns_everything_that_happened() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "echo-agent",
        &format!(
            "printf 'args=%s|%s\\n' \"$1\" \"$2\"\ncat\necho on-stderr >&2\n\
             {FRONT:?} end-session sent-by-run\nexit 3"
        ),
    );
    settings(&sandbox, &["echo-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "echo-agent", args: ["first arg", "second"], stdin: "line one\nline two" }"#,
        WHOLE,
    ));

    let (run, notice) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "exited");
    assert_eq!(run["exitCode"], 3);
    assert_eq!(run["signal"], Value::Null);
    assert_eq!(run["stdout"], "args=first arg|second\nline one\nline two");
    assert_eq!(run["stderr"], "on-stderr\n");
    assert_eq!(run["result"], "sent-by-run");
    assert_eq!(run["error"], Value::Null);
    assert!(run["elapsedMs"].as_u64().is_some(), "{run}");
    // The session returned the value of the managed process, not the run's.
    let listed = &notice["harnessRuns"];
    assert_eq!(listed.as_array().unwrap().len(), 1, "{notice}");
    assert_eq!(listed[0]["harness"], "echo-agent");
    assert_eq!(listed[0]["status"], "exited");
    assert_eq!(listed[0]["exitCode"], 3);
    assert_eq!(listed[0]["signal"], Value::Null);
    assert_eq!(listed[0]["sentResult"], true);
    assert!(listed[0]["elapsedMs"].as_u64().is_some(), "{listed}");
    assert_clean(&sandbox);
}

#[test]
fn a_run_that_sends_no_result_returns_unit_and_a_clean_exit_is_zero() {
    // The firing control of the result: the same harness without the send.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "quiet-agent", "echo fine");
    settings(&sandbox, &["quiet-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "quiet-agent", args: [] }"#, WHOLE));

    let (run, notice) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "exited");
    assert_eq!(run["exitCode"], 0);
    assert_eq!(run["result"], Value::Null);
    assert_eq!(notice["harnessRuns"][0]["sentResult"], false);
}

#[test]
fn a_run_killed_by_a_signal_is_signalled() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "dying-agent", "echo before\nkill -KILL $$");
    settings(&sandbox, &["dying-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "dying-agent", args: [] }"#, WHOLE));

    let (run, notice) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "signalled");
    assert_eq!(run["signal"], "SIGKILL");
    assert_eq!(run["exitCode"], Value::Null);
    assert_eq!(run["stdout"], "before\n");
    assert_eq!(notice["harnessRuns"][0]["status"], "signalled");
    assert_eq!(notice["harnessRuns"][0]["signal"], "SIGKILL");
}

#[test]
fn without_stdin_standard_input_is_at_end_of_file_and_invalid_utf8_shows_as_replacement() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "reading-agent", "cat\nprintf 'a\\377\\376b'");
    settings(&sandbox, &["reading-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "reading-agent", args: [] }"#,
        WHOLE,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    // `cat` ended, so its input was at end of file.
    assert_eq!(run["status"], "exited");
    assert_eq!(run["stdout"], "a\u{fffd}\u{fffd}b");
}

#[test]
fn a_run_works_in_the_cwd_of_the_request_with_no_terminal_and_a_session_of_its_own() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "where-agent",
        "pwd\nps -o pid=,pgid=,tty= -p $$\n[ -t 0 ] && echo terminal-input || echo no-terminal-input",
    );
    settings(&sandbox, &["where-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "where-agent", args: [] }"#, WHOLE));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    let stdout = run["stdout"].as_str().unwrap();
    let mut lines = stdout.lines();
    let cwd = fs::canonicalize(&sandbox.cwd).unwrap();
    assert_eq!(
        fs::canonicalize(lines.next().unwrap()).unwrap(),
        cwd,
        "{stdout}"
    );
    let mut fields = lines.next().unwrap().split_whitespace();
    let (pid, group, tty) = (
        fields.next().unwrap(),
        fields.next().unwrap(),
        fields.next().unwrap(),
    );
    assert_eq!(pid, group, "the run does not lead its group: {stdout}");
    assert!(tty.starts_with('?'), "the run has the terminal {tty}");
    assert_eq!(lines.next(), Some("no-terminal-input"));
}

#[test]
fn a_run_gets_the_caller_environment_the_marker_and_a_channel_of_its_own() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let enclosing = sandbox.root.join("enclosing-session");
    harness(
        &sandbox,
        "env-agent",
        "echo \"caller=${CALLER_VAR-unset}\"\necho \"marker=${RAVEL_CONFIG_RUN-unset}\"\n\
         echo \"run-id=${RAVEL_RUN_ID-unset}\"\necho \"exit-file=${RAVEL_EXIT_FILE-unset}\"",
    );
    settings(&sandbox, &["env-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "env-agent", args: [] }"#, WHOLE));
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .env("CALLER_VAR", "from-the-caller")
        .env("RAVEL_RUN_ID", "enclosing-run")
        .env("RAVEL_EXIT_FILE", &enclosing);

    let output = run(&mut command);
    output.report();

    let run = handed(&recorded);
    let stdout = run["stdout"].as_str().unwrap();
    assert!(stdout.contains("caller=from-the-caller"), "{stdout}");
    assert!(stdout.contains("marker=1"), "{stdout}");
    assert!(stdout.contains("run-id=unset"), "{stdout}");
    let exit_file = stdout
        .lines()
        .find_map(|line| line.strip_prefix("exit-file="))
        .unwrap();
    assert_ne!(exit_file, "unset", "the run has no channel: {stdout}");
    assert_ne!(
        Path::new(exit_file),
        enclosing,
        "the run kept the enclosing channel"
    );
    assert!(!enclosing.exists(), "the enclosing channel was touched");
}

#[test]
fn the_managed_process_never_receives_the_marker_or_the_channel_of_a_run() {
    let sandbox = Sandbox::new();
    let seen = sandbox.root.join("managed-env");
    harness(&sandbox, "quiet-agent", "echo fine");
    harness(
        &sandbox,
        "env-dumping-agent",
        &format!("env > {seen:?}\nexec {FRONT:?} end-session done"),
    );
    settings(&sandbox, &["quiet-agent"], 1);
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let run = ravel::run_harness(#{ harness: "quiet-agent", args: [] });
    #{ status: "selected", program: "env-dumping-agent", args: [], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    ));

    start(&sandbox, &[]).report();

    let environment = fs::read_to_string(seen).unwrap();
    assert!(
        !environment.contains("RAVEL_CONFIG_RUN"),
        "the managed process has the marker:\n{environment}"
    );
    // The control: the managed process does have the channel of its own session.
    assert!(environment.contains("RAVEL_EXIT_FILE="), "{environment}");
}

#[test]
fn a_missing_program_is_data_and_uses_no_budget() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "present-agent", "echo present");
    support::write(
        &sandbox.home.join(SETTINGS),
        r#"{ "harnesses": { "absent-agent": { "program": "no-such-program" },
                            "present-agent": { "program": "present-agent" } },
             "budgets": { "harnessRuns": 1 } }"#,
    );
    // The budget is 1: if the missing program had used it, the second run would
    // end the evaluation.
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let missing = ravel::run_harness(#{ harness: "absent-agent", args: [] });
    let remaining = ravel::remaining().harnessRuns;
    let present = ravel::run_harness(#{ harness: "present-agent", args: [] });
    #{ status: "selected", program: "recording-agent", provider: "p", model: "m",
       effort: "e", reason: "r",
       args: [ravel::json_encode(#{ missing: missing, remaining: remaining, present: present })] }
}"#,
    ));

    let (handed, notice) = ran(&sandbox, &recorded, &[]);

    let missing = &handed["missing"];
    assert_eq!(missing["status"], "unavailable");
    assert_eq!(missing["error"]["code"], "harness_not_found");
    assert!(
        missing["error"]["message"]
            .as_str()
            .unwrap()
            .contains("no-such-program")
    );
    for field in ["exitCode", "signal", "result"] {
        assert_eq!(missing[field], Value::Null, "{field}");
    }
    assert_eq!(missing["stdout"], "");
    assert_eq!(handed["remaining"], 1);
    assert_eq!(handed["present"]["status"], "exited");
    // The notice lists the unavailable run too, in order.
    let listed = notice["harnessRuns"].as_array().unwrap();
    assert_eq!(listed.len(), 2, "{notice}");
    assert_eq!(listed[0]["status"], "unavailable");
    assert_eq!(listed[1]["status"], "exited");
}

#[test]
fn an_unknown_harness_and_a_program_that_cannot_run_are_data_with_their_own_codes() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    // Present, and not executable.
    let flat = sandbox.bin.join("flat-agent");
    support::write(&flat, "#!/bin/sh\necho never\n");
    fs::set_permissions(&flat, fs::Permissions::from_mode(0o644)).unwrap();
    // Executable, and its interpreter does not exist: the spawn itself fails.
    let broken = sandbox.bin.join("broken-agent");
    support::write(&broken, "#!/no/such/interpreter\n");
    fs::set_permissions(&broken, fs::Permissions::from_mode(0o755)).unwrap();
    support::write(
        &sandbox.home.join(SETTINGS),
        &json!({ "harnesses": {
            "flat": { "program": flat },
            "broken": { "program": broken },
        }, "budgets": { "harnessRuns": 1 } })
        .to_string(),
    );
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let unknown = ravel::run_harness(#{ harness: "nobody", args: [] });
    let flat = ravel::run_harness(#{ harness: "flat", args: [] });
    let broken = ravel::run_harness(#{ harness: "broken", args: [] });
    #{ status: "selected", program: "recording-agent", provider: "p", model: "m",
       effort: "e", reason: "r",
       args: [ravel::json_encode(#{ unknown: unknown.error.code, flat: flat.error.code,
                                    broken: broken.error.code,
                                    remaining: ravel::remaining().harnessRuns })] }
}"#,
    ));

    let (handed, _) = ran(&sandbox, &recorded, &[]);

    assert_eq!(handed["unknown"], "unknown_harness");
    assert_eq!(handed["flat"], "harness_not_executable");
    assert_eq!(handed["broken"], "harness_launch_failed");
    // None of the three used the budget.
    assert_eq!(handed["remaining"], 1);
    assert_clean(&sandbox);
}

#[test]
fn the_output_bound_stops_the_run_and_clears_its_group_and_the_route_launches() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let pids = sandbox.root.join("flooding.pids");
    harness(
        &sandbox,
        "flooding-agent",
        &format!("sleep 60 &\necho $! > {pids:?}\nhead -c 2097152 /dev/zero | tr '\\0' x\nwait"),
    );
    settings(&sandbox, &["flooding-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "flooding-agent", args: [] }"#,
        SUMMARY,
    ));
    let began = Instant::now();

    let (run, notice) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "output_limit");
    // Ravel stopped it at the first byte beyond 1 MiB, so the script has exactly
    // 1 MiB.
    assert_eq!(run["stdoutBytes"], 1024 * 1024);
    assert_eq!(notice["harnessRuns"][0]["status"], "output_limit");
    assert!(
        gone(pid_of(&pids)),
        "the background process of the run survived"
    );
    assert!(began.elapsed() < Duration::from_secs(30));
    assert_clean(&sandbox);
}

#[test]
fn output_of_exactly_the_bound_is_no_breach() {
    // The firing control of the output bound.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "exact-agent",
        "head -c 1048576 /dev/zero | tr '\\0' x\nhead -c 1048576 /dev/zero | tr '\\0' y >&2",
    );
    settings(&sandbox, &["exact-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "exact-agent", args: [] }"#,
        SUMMARY,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "exited");
    assert_eq!(run["stdoutBytes"], 1024 * 1024);
    assert_eq!(run["stderrBytes"], 1024 * 1024);
}

#[test]
fn the_error_stream_has_the_same_bound() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "chatty-agent",
        "head -c 1048577 /dev/zero | tr '\\0' e >&2\nsleep 60",
    );
    settings(&sandbox, &["chatty-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "chatty-agent", args: [] }"#,
        SUMMARY,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "output_limit");
    assert_eq!(run["stderrBytes"], 1024 * 1024);
}

#[test]
fn the_deadline_of_one_run_ends_only_the_run_and_clears_its_group() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let pids = sandbox.root.join("hanging.pids");
    harness(
        &sandbox,
        "hanging-agent",
        &format!("echo started\nsleep 60 &\necho $! > {pids:?}\nwait"),
    );
    settings(&sandbox, &["hanging-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "hanging-agent", args: [], timeoutMs: 1000 }"#,
        WHOLE,
    ));
    let began = Instant::now();

    let (run, notice) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "timed_out");
    assert!(run["elapsedMs"].as_u64().unwrap() >= 800, "{run}");
    // What it wrote before the stop reaches the script.
    assert_eq!(run["stdout"], "started\n");
    assert_eq!(notice["harnessRuns"][0]["status"], "timed_out");
    assert!(
        gone(pid_of(&pids)),
        "the background process of the run survived"
    );
    assert!(began.elapsed() < Duration::from_secs(30));
    assert_clean(&sandbox);
}

#[test]
fn a_run_that_ends_inside_its_deadline_is_not_timed_out() {
    // The firing control of the deadline of one run.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "brisk-agent", "echo brisk");
    settings(&sandbox, &["brisk-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "brisk-agent", args: [], timeoutMs: 20000 }"#,
        WHOLE,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "exited");
    assert_eq!(run["stdout"], "brisk\n");
}

#[test]
fn the_budget_counts_launched_runs_and_remaining_reports_it() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "brisk-agent", "echo brisk");
    settings(&sandbox, &["brisk-agent"], 3);
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let before = ravel::remaining().harnessRuns;
    ravel::run_harness(#{ harness: "brisk-agent", args: [] });
    let between = ravel::remaining().harnessRuns;
    ravel::run_harness(#{ harness: "brisk-agent", args: [] });
    #{ status: "selected", program: "recording-agent", provider: "p", model: "m",
       effort: "e", reason: "r",
       args: [ravel::json_encode([before, between, ravel::remaining().harnessRuns,
                                  request.limits.harnessRuns])] }
}"#,
    ));

    let (counts, notice) = ran(&sandbox, &recorded, &[]);

    // `request.limits` is the budget, whole, and `remaining` is what is left.
    assert_eq!(counts, json!([3, 2, 1, 3]));
    assert_eq!(notice["harnessRuns"].as_array().unwrap().len(), 2);
}

#[test]
fn a_breach_of_the_budget_ends_the_evaluation_and_try_cannot_catch_it() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "brisk-agent", "echo brisk");
    settings(&sandbox, &["brisk-agent"], 1);
    let source = |budget_note: &str| {
        policy(&format!(
            r#"fn select(request, context) {{
    ravel::run_harness(#{{ harness: "brisk-agent", args: [] }});
    try {{
        ravel::run_harness(#{{ harness: "brisk-agent", args: [] }});
    }} catch (error) {{
        // {budget_note}
    }}
    #{{ status: "selected", program: "recording-agent", args: ["went on"], provider: "p",
       model: "m", effort: "e", reason: "r" }}
}}"#
        ))
    };
    sandbox.personal_rhai(&source("the script goes on"));

    let error = refused(
        &start(&sandbox, &[]),
        3,
        "harness_budget_exhausted",
        &recorded,
    );

    assert_eq!(error["stage"], "evaluation");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains("harness run 2"), "{message}");
    assert!(message.contains("budget of 1"), "{message}");
    assert_eq!(error["bound"]["name"], "harnessRuns");
    assert_eq!(error["bound"]["runs"], 1);
    assert_eq!(error["bound"]["from"], "settings.json");

    // The firing control: with a budget of 2 the same policy goes on.
    settings(&sandbox, &["brisk-agent"], 2);
    start(&sandbox, &[]).report();
    assert_eq!(recorded.argv().unwrap(), ["went on"]);
}

#[test]
fn the_default_budget_of_zero_runs_nothing() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let witness = sandbox.root.join("witness");
    harness(&sandbox, "brisk-agent", &format!("echo ran > {witness:?}"));
    support::write(
        &sandbox.home.join(SETTINGS),
        r#"{ "harnesses": { "brisk-agent": { "program": "brisk-agent" } } }"#,
    );
    sandbox.personal_rhai(&running(r#"#{ harness: "brisk-agent", args: [] }"#, WHOLE));

    let error = refused(
        &start(&sandbox, &[]),
        3,
        "harness_budget_exhausted",
        &recorded,
    );

    assert_eq!(error["bound"]["name"], "harnessRuns");
    assert_eq!(error["bound"]["runs"], 0);
    assert_eq!(error["bound"]["from"], "default");
    assert!(!witness.exists(), "the harness ran with no budget");
}

#[test]
fn a_malformed_request_ends_the_evaluation_before_anything_runs() {
    let cases = [
        ("a request that is no map", r#""brisk-agent""#, "request"),
        ("no harness", "#{ args: [] }", "harness"),
        (
            "a harness that is no string",
            "#{ harness: 4, args: [] }",
            "harness",
        ),
        ("no args", r#"#{ harness: "brisk-agent" }"#, "args"),
        (
            "args that are no array",
            r#"#{ harness: "brisk-agent", args: "x" }"#,
            "args",
        ),
        (
            "an argument that is no string",
            r#"#{ harness: "brisk-agent", args: [1] }"#,
            "args[0]",
        ),
        (
            "a NUL in an argument",
            r#"#{ harness: "brisk-agent", args: ["a\x00b"] }"#,
            "args[0]",
        ),
        (
            "a stdin that is no string",
            r#"#{ harness: "brisk-agent", args: [], stdin: 3 }"#,
            "stdin",
        ),
        (
            "a timeout of zero",
            r#"#{ harness: "brisk-agent", args: [], timeoutMs: 0 }"#,
            "timeoutMs",
        ),
        (
            "a timeout that is no integer",
            r#"#{ harness: "brisk-agent", args: [], timeoutMs: 1.5 }"#,
            "timeoutMs",
        ),
        (
            "a field of no request",
            r#"#{ harness: "brisk-agent", args: [], model: "x" }"#,
            "model",
        ),
    ];
    for (label, request, field) in cases {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let witness = sandbox.root.join("witness");
        harness(&sandbox, "brisk-agent", &format!("echo ran > {witness:?}"));
        settings(&sandbox, &["brisk-agent"], 2);
        sandbox.personal_rhai(&running(request, WHOLE));

        let error = refused(&start(&sandbox, &[]), 3, "host_request_invalid", &recorded);

        let location = error["location"].as_str().unwrap();
        assert!(
            location.contains(&format!("ravel::run_harness({field})")),
            "{label}: {error}"
        );
        assert!(!witness.exists(), "{label}: the harness ran");
    }
    // The firing control: the request of the first case, made well-formed, runs.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "brisk-agent", "echo ran");
    settings(&sandbox, &["brisk-agent"], 2);
    sandbox.personal_rhai(&running(r#"#{ harness: "brisk-agent", args: [] }"#, WHOLE));
    assert_eq!(ran(&sandbox, &recorded, &[]).0["stdout"], "ran\n");
}

#[test]
fn start_session_inside_a_run_is_refused_and_the_script_gets_the_status() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "nesting-agent",
        &format!(
            "{FRONT:?} start-session --kind check --prompt nested --json 2>&1 >/dev/null\n\
             echo \"nested-exit=$?\" >&2\nexit 9"
        ),
    );
    settings(&sandbox, &["nesting-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "nesting-agent", args: [] }"#,
        WHOLE,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    // The nested invocation exited 3 with the refusal, and the run's own exit
    // status is what the script receives.
    assert_eq!(run["status"], "exited");
    assert_eq!(run["exitCode"], 9);
    let stderr = run["stderr"].as_str().unwrap();
    assert!(stderr.contains("nested-exit=3"), "{stderr}");
}

#[test]
fn the_refusal_comes_before_settings_and_before_a_policy() {
    // No policy exists, and the settings file is malformed. Without the marker
    // the settings are what refuses.
    let sandbox = Sandbox::new();
    support::write(&sandbox.home.join(SETTINGS), "not json");
    let mut command = sandbox.command();
    command.args([
        "start-session",
        "--prompt",
        "p",
        "--kind",
        "check",
        "--json",
    ]);
    let control = run(&mut command);
    assert_eq!(control.refusal(2)["error"]["code"], "settings_invalid");

    for value in ["1", ""] {
        let mut command = sandbox.command();
        command
            .args([
                "start-session",
                "--prompt",
                "p",
                "--kind",
                "check",
                "--json",
            ])
            .env("RAVEL_CONFIG_RUN", value);
        let refusal = run(&mut command).refusal(3);

        assert_eq!(refusal["error"]["code"], "policy_reentry_refused");
        assert_eq!(refusal["error"]["stage"], "authority");
    }
}

#[test]
fn a_run_cannot_complete_the_enclosing_session() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let enclosing = sandbox.root.join("enclosing-channel");
    harness(
        &sandbox,
        "completing-agent",
        &format!("{FRONT:?} end-session other\n{FRONT:?} end-session ignored"),
    );
    settings(&sandbox, &["completing-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "completing-agent", args: [] }"#,
        WHOLE,
    ));
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .env("RAVEL_EXIT_FILE", &enclosing);

    let output = run(&mut command);

    // The session returned the value of the managed process, and the run's value
    // reached the script. The first value of a channel wins.
    output.report();
    let run = handed(&recorded);
    assert_eq!(run["result"], "other");
    assert!(
        !enclosing.exists(),
        "the run completed the enclosing session"
    );
}

#[test]
fn the_worker_has_no_completion_channel_to_read() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "brisk-agent", "echo brisk");
    settings(&sandbox, &["brisk-agent"], 1);
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    ravel::run_harness(#{ harness: "brisk-agent", args: [] });
    let seen = "readable";
    try { ravel::env("RAVEL_EXIT_FILE"); } catch (error) { seen = error.code; }
    #{ status: "selected", program: "recording-agent", args: [seen], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    ));
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .env("RAVEL_EXIT_FILE", sandbox.root.join("enclosing"));

    run(&mut command).report();

    assert_eq!(recorded.argv().unwrap(), ["env_not_granted"]);
}

#[test]
fn a_run_that_hangs_past_the_whole_deadline_ends_the_evaluation_and_clears_its_group() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let pids = sandbox.root.join("stalling.pids");
    harness(
        &sandbox,
        "stalling-agent",
        &format!("sleep 60 &\necho $! > {pids:?}\nwait"),
    );
    settings(&sandbox, &["stalling-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "stalling-agent", args: [] }"#,
        WHOLE,
    ));
    let began = Instant::now();

    let error = refused(
        &start(&sandbox, &["--timeout-ms", "1500"]),
        124,
        "selection_timeout",
        &recorded,
    );

    let _ = error;
    assert!(
        gone(pid_of(&pids)),
        "the background process of the run survived the stop"
    );
    assert!(began.elapsed() < Duration::from_secs(30));
    assert_clean(&sandbox);
}

#[test]
fn a_confined_run_has_a_private_run_directory_and_the_confined_environment() {
    let sandbox = Sandbox::new();
    // The harness writes what it saw into its working directory, which a
    // confined run may write, because the managed process is confined too and
    // could not record it.
    let script = harness(
        &sandbox,
        "confined-agent",
        &format!(
            "touch ./inside || echo no-write-in-cwd > ./observed\n\
             touch \"$HOME/outside\" 2>/dev/null && echo wrote-home >> ./observed\n\
             echo \"token=${{PRIVATE_TOKEN-unset}}\" >> ./observed\n\
             echo \"marker=${{RAVEL_CONFIG_RUN-unset}}\" >> ./observed\n\
             echo \"tmp=$TMPDIR\" >> ./observed\n\
             {FRONT:?} end-session confined-result"
        ),
    );
    settings(&sandbox, &["confined-agent"], 1);
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let run = ravel::run_harness(#{ harness: "confined-agent", args: [] });
    if run.status != "exited" || run.result != "confined-result" {
        throw "the confined run did not finish: " + ravel::json_encode(run);
    }
    #{ status: "selected", program: "fake-agent", args: [], provider: "p", model: "m",
       effort: "e", reason: "r" }
}"#,
    ));
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
            "--confine",
            "--runtime-read",
        ])
        .arg(&script)
        .arg("--runtime-read")
        .arg(sandbox.bin.join("fake-agent"))
        .env("PRIVATE_TOKEN", "must not reach a confined run");

    let output = run(&mut command);
    assert_eq!(output.code, Some(0), "{}", output.stderr);

    let observed = fs::read_to_string(sandbox.cwd.join("observed")).unwrap();
    assert!(!observed.contains("no-write-in-cwd"), "{observed}");
    assert!(!observed.contains("wrote-home"), "{observed}");
    assert!(observed.contains("token=unset"), "{observed}");
    assert!(observed.contains("marker=1"), "{observed}");
    // The temporary directory is the run's own, under the private run directory.
    let tmp = observed
        .lines()
        .find_map(|line| line.strip_prefix("tmp="))
        .unwrap();
    assert!(tmp.contains("ravel-confined-"), "{observed}");
    assert!(sandbox.cwd.join("inside").exists());
    assert!(!sandbox.home.join("outside").exists());
    assert_clean(&sandbox);

    // The firing control: without `--confine` the same run writes to the home
    // and sees the caller variable, so the confinement is what stopped both.
    fs::remove_file(sandbox.cwd.join("observed")).unwrap();
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .env("PRIVATE_TOKEN", "visible");
    let open = run(&mut command);
    assert_eq!(open.code, Some(0), "{}", open.stderr);
    let observed = fs::read_to_string(sandbox.cwd.join("observed")).unwrap();
    assert!(observed.contains("wrote-home"), "{observed}");
    assert!(observed.contains("token=visible"), "{observed}");
    assert!(observed.contains("marker=1"), "{observed}");
    assert!(sandbox.home.join("outside").exists());
}

#[test]
fn an_unconfined_run_has_the_caller_variables_that_a_confined_run_lacks() {
    // The firing control of the confined case above.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "plain-agent",
        "echo \"token=${PRIVATE_TOKEN-unset}\"",
    );
    settings(&sandbox, &["plain-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "plain-agent", args: [] }"#, WHOLE));
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .env("PRIVATE_TOKEN", "visible");

    run(&mut command).report();

    assert_eq!(handed(&recorded)["stdout"], "token=visible\n");
}

#[test]
fn runs_are_listed_in_the_order_they_ran_and_a_run_in_load_context_counts() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(&sandbox, "first-agent", "echo one");
    harness(&sandbox, "second-agent", "echo two");
    settings(&sandbox, &["first-agent", "second-agent"], 2);
    sandbox.personal_rhai(&policy(
        r#"fn load_context(request) {
    #{ schemaVersion: 1,
       facts: #{ loaded: ravel::run_harness(#{ harness: "first-agent", args: [] }).stdout } }
}
fn select(request, context) {
    let run = ravel::run_harness(#{ harness: "second-agent", args: [] });
    let joined = context.facts.loaded + run.stdout;
    joined.replace("\n", "|");
    #{ status: "selected", program: "recording-agent", provider: "p", model: "m",
       effort: "e", reason: "r", args: [joined] }
}"#,
    ));

    let notice = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["one|two|"]);
    let harnesses: Vec<_> = notice["harnessRuns"]
        .as_array()
        .unwrap()
        .iter()
        .map(|run| run["harness"].as_str().unwrap())
        .collect();
    assert_eq!(harnesses, ["first-agent", "second-agent"]);
    let _ = rhai::HEADER;
}

#[test]
fn a_signal_during_a_run_stops_the_run_with_its_group_and_cancels_the_session() {
    use std::os::unix::process::ExitStatusExt as _;
    use std::process::Stdio;

    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let pids = sandbox.root.join("waiting.pids");
        harness(
            &sandbox,
            "waiting-agent",
            &format!("sleep 60 &\necho $! > {pids:?}\necho $$ >> {pids:?}\nwait"),
        );
        settings(&sandbox, &["waiting-agent"], 1);
        sandbox.personal_rhai(&running(
            r#"#{ harness: "waiting-agent", args: [] }"#,
            WHOLE,
        ));
        let front = sandbox
            .command()
            .args([
                "start-session",
                "--kind",
                "check",
                "--prompt",
                "p",
                "--json",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        while fs::read_to_string(&pids).map_or(true, |text| text.lines().count() < 2) {
            assert!(Instant::now() < deadline, "signal {signal}: no run started");
            std::thread::sleep(Duration::from_millis(20));
        }
        let began = Instant::now();
        // SAFETY: the pid is this test's live front.
        assert_eq!(unsafe { libc::kill(front.id() as i32, signal) }, 0);

        let output = front.wait_with_output().unwrap();

        assert_eq!(output.status.signal(), Some(signal), "signal {signal}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("selection_cancelled"), "{stderr}");
        let text = fs::read_to_string(&pids).unwrap();
        for pid in text.lines() {
            assert!(
                gone(pid.parse().unwrap()),
                "signal {signal}: process {pid} of the run survived"
            );
        }
        assert_eq!(recorded.argv(), None, "signal {signal}: a harness started");
        assert_clean(&sandbox);
        assert!(began.elapsed() < Duration::from_secs(10), "signal {signal}");
    }
}

#[test]
fn the_largest_reply_of_a_run_still_reaches_the_script() {
    // Each control byte encodes to six bytes of JSON, and the output, the error
    // output and the result can each hold 1 MiB. The reply is about 18 MiB.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    harness(
        &sandbox,
        "controlling-agent",
        "head -c 1048576 /dev/zero | tr '\\0' '\\001'\n\
         head -c 1048576 /dev/zero | tr '\\0' '\\001' >&2\n\
         head -c 1048576 /dev/zero | tr '\\0' '\\001' > \"$RAVEL_EXIT_FILE\"",
    );
    settings(&sandbox, &["controlling-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "controlling-agent", args: [] }"#,
        r#"[ravel::json_encode(#{ status: run.status, stdoutBytes: run.stdout.len(),
            stderrBytes: run.stderr.len(), resultBytes: run.result.len() })]"#,
    ));

    let (run, _) = ran(&sandbox, &recorded, &[]);

    assert_eq!(run["status"], "exited");
    assert_eq!(run["stdoutBytes"], 1024 * 1024);
    assert_eq!(run["stderrBytes"], 1024 * 1024);
    assert_eq!(run["resultBytes"], 1024 * 1024);
}

// The ordered stop of an evaluation (`docs/specs/policy-evaluation.md`,
// *Cancellation and deadlines*). The runner stops a run with TERM and escalates
// to KILL after its kill-grace, which is 5 seconds.

/// The kill-grace of the runner, and the margin that the spec adds to it.
const KILL_GRACE: Duration = Duration::from_secs(5);
const TEARDOWN_MARGIN: Duration = Duration::from_secs(1);
/// What a loaded machine adds to the process start and to the reap of the worker.
/// An idle machine adds about 0.3 seconds.
const SLACK: Duration = Duration::from_secs(2);

/// A harness that ignores TERM, so that only the KILL after the kill-grace ends
/// it. It leaves a background process of its own, and writes both pids.
fn deaf_harness(sandbox: &Sandbox, pids: &Path) {
    harness(
        sandbox,
        "deaf-agent",
        &format!("trap '' TERM\nsleep 60 &\necho $! > {pids:?}\necho $$ >> {pids:?}\nwait"),
    );
    settings(sandbox, &["deaf-agent"], 1);
    sandbox.personal_rhai(&running(r#"#{ harness: "deaf-agent", args: [] }"#, WHOLE));
}

fn spawn_front(sandbox: &Sandbox, extra: &[&str]) -> std::process::Child {
    use std::process::Stdio;
    sandbox
        .command()
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "p",
            "--json",
        ])
        .args(extra)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

/// Wait until the harness has written both pids, which is when the run is in
/// progress.
fn await_run(pids: &Path) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while fs::read_to_string(pids).map_or(true, |text| text.lines().count() < 2) {
        assert!(Instant::now() < deadline, "no run started");
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn send(front: &std::process::Child, signal: i32) {
    // SAFETY: the pid is this test's live front.
    assert_eq!(unsafe { libc::kill(front.id() as i32, signal) }, 0);
}

fn assert_all_gone(pids: &Path) {
    for pid in fs::read_to_string(pids).unwrap().lines() {
        assert!(
            gone(pid.parse().unwrap()),
            "process {pid} of the run survived"
        );
    }
}

#[test]
fn a_run_that_ignores_term_is_killed_after_the_kill_grace_and_the_deadline_refuses() {
    // The firing control of the cases below: without a signal, the window
    // between the deadline and the KILL exists, because the harness holds it.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let pids = sandbox.root.join("deaf.pids");
    deaf_harness(&sandbox, &pids);
    let began = Instant::now();

    let run = start(&sandbox, &["--timeout-ms", "1500"]);

    let elapsed = began.elapsed();
    refused(&run, 124, "selection_timeout", &recorded);
    assert_all_gone(&pids);
    assert!(
        elapsed >= KILL_GRACE,
        "the run ended after {elapsed:?}, before the kill-grace: the harness did not ignore TERM"
    );
    assert!(
        elapsed < Duration::from_millis(1500) + KILL_GRACE + TEARDOWN_MARGIN + SLACK,
        "the stop took {elapsed:?}"
    );
    assert_clean(&sandbox);
}

#[test]
fn a_signal_after_the_deadline_wins_and_the_report_keeps_the_timeout() {
    use std::os::unix::process::ExitStatusExt as _;

    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let pids = sandbox.root.join("deaf.pids");
        deaf_harness(&sandbox, &pids);
        let began = Instant::now();
        let front = spawn_front(&sandbox, &["--timeout-ms", "1500"]);
        await_run(&pids);
        // The deadline has passed, and the kill-grace has not.
        std::thread::sleep(Duration::from_millis(3000).saturating_sub(began.elapsed()));
        send(&front, signal);

        let output = front.wait_with_output().unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.signal(),
            Some(signal),
            "signal {signal}: {stderr}"
        );
        assert!(
            stderr.contains("selection_cancelled"),
            "signal {signal}: {stderr}"
        );
        assert!(
            stderr.contains("selection_timeout"),
            "signal {signal}: the report lost the original failure: {stderr}"
        );
        assert_all_gone(&pids);
        assert_eq!(recorded.argv(), None, "signal {signal}: a harness started");
        assert_clean(&sandbox);
        assert!(
            began.elapsed() < Duration::from_millis(1500) + KILL_GRACE + TEARDOWN_MARGIN + SLACK,
            "signal {signal}: the stop took {:?}",
            began.elapsed()
        );
    }
}

#[test]
fn the_first_consumed_signal_stays_the_cause() {
    use std::os::unix::process::ExitStatusExt as _;

    let sandbox = Sandbox::new();
    let pids = sandbox.root.join("deaf.pids");
    deaf_harness(&sandbox, &pids);
    let front = spawn_front(&sandbox, &["--timeout-ms", "20000"]);
    await_run(&pids);
    send(&front, libc::SIGINT);
    std::thread::sleep(Duration::from_millis(300));
    send(&front, libc::SIGTERM);

    let output = front.wait_with_output().unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.signal(), Some(libc::SIGINT), "{stderr}");
    assert!(stderr.contains("selection_cancelled"), "{stderr}");
    assert_all_gone(&pids);
    assert_clean(&sandbox);
}

/// A harness that leaves a directory in its private run directory that nobody
/// can remove, and then runs `after`. The test must restore it.
fn locking_harness(sandbox: &Sandbox, after: &str) {
    harness(
        sandbox,
        "locking-agent",
        &format!(
            "d=$(dirname \"$RAVEL_EXIT_FILE\")\nmkdir \"$d/locked\"\ntouch \"$d/locked/file\"\n\
             chmod 000 \"$d/locked\"\n{after}"
        ),
    );
    settings(sandbox, &["locking-agent"], 1);
    sandbox.personal_rhai(&running(
        r#"#{ harness: "locking-agent", args: [] }"#,
        WHOLE,
    ));
}

/// Make the directories that `locking_harness` locked removable again, and
/// remove them.
fn unlock(sandbox: &Sandbox) {
    for entry in fs::read_dir(&sandbox.tmp).unwrap() {
        let path = entry.unwrap().path();
        let locked = path.join("locked");
        if locked.symlink_metadata().is_ok() {
            fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
        }
        let _ = fs::remove_dir_all(&path);
    }
}

#[test]
fn a_cleanup_failure_after_a_run_ends_the_evaluation_with_exit_5() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    locking_harness(&sandbox, "exit 0");

    let run = start(&sandbox, &[]);

    let error = refused(&run, 5, "harness_cleanup_failed", &recorded);
    unlock(&sandbox);
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("ravel-session-"),
        "the report names no directory: {error}"
    );
}

#[test]
fn a_cleanup_failure_is_reported_when_the_deadline_stops_the_run() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    locking_harness(&sandbox, "sleep 60");

    let run = start(&sandbox, &["--timeout-ms", "1500"]);

    refused(&run, 5, "harness_cleanup_failed", &recorded);
    unlock(&sandbox);
}

#[test]
fn a_signal_wins_over_a_cleanup_failure_and_the_report_keeps_the_failure() {
    use std::os::unix::process::ExitStatusExt as _;

    let sandbox = Sandbox::new();
    let pids = sandbox.root.join("locking.pids");
    locking_harness(
        &sandbox,
        &format!("echo $$ > {pids:?}\necho $$ >> {pids:?}\nsleep 60"),
    );
    let front = spawn_front(&sandbox, &[]);
    await_run(&pids);
    send(&front, libc::SIGTERM);

    let output = front.wait_with_output().unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.signal(), Some(libc::SIGTERM), "{stderr}");
    assert!(stderr.contains("selection_cancelled"), "{stderr}");
    assert!(stderr.contains("harness_cleanup_failed"), "{stderr}");
    unlock(&sandbox);
}
