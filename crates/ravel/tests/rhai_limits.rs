//! A hostile or careless Rhai policy through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *The policy worker*, *Evaluation budgets
//! and their composition* and *Cancellation and deadlines*).
//!
//! Every hostile class keeps a control in its own test: the same shape of policy
//! with a harmless amount of the hostile step runs to a selection and starts the
//! harness. A breach therefore comes from the amount, and the refusal comes from
//! the bound that the test names. The engine limits that need no process (the
//! depth of an expression, the number of functions, the map size) are proven in
//! the unit tests of `rhai_worker::engine`.

mod support;

use std::os::unix::process::ExitStatusExt as _;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;
use support::rhai::{Recorded, policy};
use support::{Run, Sandbox};

/// The `arg0` of the worker role, which `ps` shows as its command.
const WORKER_ARG0: &str = "ravel-rhai-policy-worker";

/// A policy whose `select` runs `body`, and then routes to the recording harness
/// with `reason` as the reason.
fn selecting(body: &str, reason: &str) -> String {
    policy(&format!(
        r#"fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: [], provider: "p",
       model: "m", effort: "e", reason: {reason} }}
}}"#
    ))
}

fn start(sandbox: &Sandbox, source: &str, extra: &[&str]) -> Run {
    sandbox.personal_rhai(source);
    let mut args = vec!["--kind", "check", "--json"];
    args.extend_from_slice(extra);
    sandbox.start(&args)
}

/// The refusal that `run` must be, with its harness never started.
fn refused(run: &Run, exit: i32, code: &str, recorded: &Recorded) -> Value {
    let document = run.refusal(exit);
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(document["error"]["exit"], exit, "{document}");
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

/// A `script_limit` that names `bound`.
fn limit(run: &Run, bound: &str, recorded: &Recorded) -> Value {
    let error = refused(run, 3, "script_limit", recorded);
    assert_eq!(error["stage"], "evaluation", "{error}");
    assert_eq!(error["bound"]["name"], bound, "{error}");
    assert_eq!(error["bound"]["from"], "fixed", "{error}");
    error
}

/// The control: `source` ran to the end and the harness started.
fn survives(sandbox: &Sandbox, source: &str, recorded: &Recorded) {
    let run = start(sandbox, source, &[]);
    let report = run.report();
    assert_eq!(report["schemaVersion"], 2, "{report}");
    assert!(
        recorded.argv().is_some(),
        "the control did not start the harness"
    );
}

#[test]
fn an_endless_loop_ends_at_the_operations_bound() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(
        &sandbox,
        &selecting("    let i = 0;\n    while i < 1000 { i += 1; }", "\"r\""),
        &recorded,
    );
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(&sandbox, &selecting("    loop { }", "\"r\""), &[]);
    let error = limit(&run, "operations", &recorded);
    assert_eq!(error["bound"]["operations"], 100_000_000, "{error}");
}

#[test]
fn unbounded_recursion_ends_at_the_call_depth() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let control = policy(
        "fn down(n) { if n == 0 { 0 } else { down(n - 1) } }\n\
         fn select(request, context) { down(40); #{ status: \"selected\", program: \"recording-agent\", \
         args: [], provider: \"p\", model: \"m\", effort: \"e\", reason: \"r\" } }",
    );
    survives(&sandbox, &control, &recorded);
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(
        &sandbox,
        &policy("fn up(n) { up(n + 1) }\nfn select(request, context) { up(0) }"),
        &[],
    );
    let error = limit(&run, "call depth", &recorded);
    assert_eq!(error["bound"]["levels"], 64, "{error}");
}

#[test]
fn a_string_that_doubles_ends_at_the_string_size() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(
        &sandbox,
        &selecting(
            "    let s = \"x\";\n    for i in 0..10 { s += s; }",
            "\"r\"",
        ),
        &recorded,
    );
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(
        &sandbox,
        &selecting("    let s = \"x\";\n    loop { s += s; }", "\"r\""),
        &[],
    );
    let error = limit(&run, "string size", &recorded);
    assert_eq!(error["bound"]["bytes"], 16 * 1024 * 1024, "{error}");
}

#[test]
fn an_array_that_doubles_ends_at_the_array_size() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(
        &sandbox,
        &selecting(
            "    let a = [1];\n    for i in 0..10 { a = a + a; }",
            "\"r\"",
        ),
        &recorded,
    );
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(
        &sandbox,
        &selecting("    let a = [1];\n    loop { a = a + a; }", "\"r\""),
        &[],
    );
    let error = limit(&run, "array size", &recorded);
    assert_eq!(error["bound"]["elements"], 1_048_576, "{error}");
}

#[test]
fn try_does_not_catch_a_breach_and_the_host_refuses_one_that_a_closure_call_wrapped() {
    // The control: `try` catches an ordinary error, also one that a closure call
    // wraps, and the policy goes on to select.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(
        &sandbox,
        &selecting(
            "    try { throw \"boom\"; } catch (e) { }\n    \
             try { [1].map(|x| { throw \"boom\"; }); } catch (e) { }",
            "\"r\"",
        ),
        &recorded,
    );
    for (label, body, bound) in [
        (
            "a string that doubles",
            "    try { let s = \"x\"; loop { s += s; } } catch (e) { }",
            "string size",
        ),
        (
            "an endless loop",
            "    try { loop { } } catch (e) { }",
            "operations",
        ),
        (
            "an endless loop in a closure",
            "    try { [1].map(|x| { loop { } }); } catch (e) { }",
            "operations",
        ),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let run = start(&sandbox, &selecting(body, "\"r\""), &[]);
        let error = limit(&run, bound, &recorded);
        assert_eq!(error["bound"]["from"], "fixed", "{label}: {error}");
    }
}

#[test]
fn a_breach_inside_a_function_pointer_call_ends_the_evaluation_although_try_surrounds_it() {
    // Rhai 1.27.0 wraps the error of a function pointer in one that `try`
    // catches, whatever it holds, so the worker replaces the array and map
    // functions that call a pointer (`rhai_worker::pointer_calls`). Each limit
    // has a control with the same shape and a harmless amount, which selects.
    let lets = |count: usize| {
        (0..count)
            .map(|n| format!("let v{n} = 0;"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let under_try = |closure: &str| {
        policy(&format!(
            "fn down(n) {{ if n == 0 {{ 0 }} else {{ down(n - 1) }} }}\n\
             fn up(n) {{ up(n + 1) }}\n\
             fn select(request, context) {{\n    try {{ [1].map({closure}); }} catch (e) {{ }}\n    \
             #{{ status: \"selected\", program: \"recording-agent\", args: [], provider: \"p\", \
             model: \"m\", effort: \"e\", reason: \"r\" }}\n}}"
        ))
    };
    for (control, hostile, bound) in [
        (
            "|x| down(40)".to_owned(),
            "|x| up(0)".to_owned(),
            "call depth",
        ),
        (
            "|x| { let s = \"x\"; for i in 0..10 { s += s; } 0 }".to_owned(),
            "|x| { let s = \"x\"; loop { s += s; } }".to_owned(),
            "string size",
        ),
        (
            "|x| { let a = [1]; for i in 0..10 { a = a + a; } 0 }".to_owned(),
            "|x| { let a = [1]; loop { a = a + a; } }".to_owned(),
            "array size",
        ),
        (
            format!("|x| {{ {} 0 }}", lets(100)),
            format!("|x| {{ {} 0 }}", lets(16_500)),
            "variables",
        ),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        survives(&sandbox, &under_try(&control), &recorded);
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let run = start(&sandbox, &under_try(&hostile), &[]);
        limit(&run, bound, &recorded);
    }
}

#[test]
fn a_breach_inside_a_comparer_ends_the_evaluation_although_no_try_is_involved() {
    // Rhai 1.27.0 discards the error of the comparer in `dedup` and `sort`, so
    // the worker replaces them. The control calls the same comparers with a
    // depth that is under the bound, and the policy selects.
    for call in ["dedup", "sort"] {
        let comparer = |depth: &str| {
            policy(&format!(
                "fn down(n) {{ if n == 0 {{ 0 }} else {{ down(n - 1) }} }}\n\
                 fn up(n) {{ up(n + 1) }}\n\
                 fn select(request, context) {{\n    [1, 2].{call}(|x, y| {{ {depth}; 0 }});\n    \
                 #{{ status: \"selected\", program: \"recording-agent\", args: [], provider: \"p\", \
                 model: \"m\", effort: \"e\", reason: \"r\" }}\n}}"
            ))
        };
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        survives(&sandbox, &comparer("down(40)"), &recorded);
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let run = start(&sandbox, &comparer("up(0)"), &[]);
        let error = limit(&run, "call depth", &recorded);
        assert_eq!(error["bound"]["levels"], 64, "{call}: {error}");
    }
}

#[test]
fn a_policy_that_fills_the_heap_ends_at_the_heap_bound_and_the_cli_reports_it() {
    // Each variable holds a copy of an array that is under the array size, and
    // no single value, and no one array with its nested elements counted, passes
    // a size limit. Only the sum of the copies reaches the heap bound.
    let fill = |copies: usize| {
        let lets = (0..copies)
            .map(|n| format!("    let copy{n} = chunk + [0];"))
            .collect::<Vec<_>>()
            .join("\n");
        selecting(
            &format!("    let chunk = [];\n    chunk.pad(900000, 0);\n{lets}"),
            "\"r\"",
        )
    };
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(&sandbox, &fill(5), &recorded);
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(&sandbox, &fill(400), &[]);
    let error = refused(&run, 3, "memory_limit", &recorded);
    assert_eq!(error["stage"], "evaluation", "{error}");
    assert_eq!(error["bound"]["name"], "heap", "{error}");
    assert_eq!(error["bound"]["bytes"], 1024 * 1024 * 1024_u64, "{error}");
}

/// `--timeout-ms 1000` and the time a refusal may take: the bound, and then the
/// teardown, which the supervision keeps within two seconds.
const DEADLINE_MS: u64 = 1_000;

fn within_the_deadline(started: Instant) {
    let took = started.elapsed();
    assert!(
        took >= Duration::from_millis(DEADLINE_MS)
            && took < Duration::from_millis(DEADLINE_MS + 3_000),
        "took {took:?}"
    );
}

#[test]
fn a_native_call_that_does_not_yield_stops_at_the_deadline() {
    // `sleep` and the `pad` loop of Rhai 1.27.0 never return to the engine, so
    // no stop flag reaches them: only the death of the worker ends them.
    for (label, body, control) in [
        ("sleep", "    sleep(600);", "    sleep(0);"),
        (
            "pad with an empty padding string",
            "    let s = \"a\";\n    s.pad(10, \"\");",
            "    let s = \"a\";\n    s.pad(10, \"-\");",
        ),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let started = Instant::now();
        let run = start(
            &sandbox,
            &selecting(control, "\"r\""),
            &["--timeout-ms", "30000"],
        );
        run.report();
        assert!(
            started.elapsed() < Duration::from_millis(DEADLINE_MS),
            "{label}: the control must return at once"
        );
        assert!(
            recorded.argv().is_some(),
            "{label}: the control did not start the harness"
        );

        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let started = Instant::now();
        let run = start(
            &sandbox,
            &selecting(body, "\"r\""),
            &["--timeout-ms", &DEADLINE_MS.to_string()],
        );
        let error = refused(&run, 124, "selection_timeout", &recorded);
        assert_eq!(error["stage"], "evaluation", "{label}: {error}");
        within_the_deadline(started);
        assert_eq!(
            std::fs::read_dir(&sandbox.tmp).unwrap().count(),
            0,
            "{label}: the private directory of the worker is gone"
        );
    }
}

#[test]
fn a_policy_that_prints_without_end_ends_as_output_limit() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(
        &sandbox,
        &selecting("    for i in 0..10 { print(\"line\"); }", "\"r\""),
        &[],
    );
    let report = run.report();
    assert_eq!(report["diagnostics"]["stdout"], "line\n".repeat(10));
    assert!(recorded.argv().is_some());

    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let flood =
        "    loop { print(\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"); }";
    let run = start(&sandbox, &selecting(flood, "\"r\""), &[]);
    let error = refused(&run, 3, "output_limit", &recorded);
    assert_eq!(error["bound"]["name"], "diagnostics", "{error}");
}

#[test]
fn a_result_over_the_message_bound_ends_as_message_too_large() {
    // 2^13 bytes pass and 2^21 bytes do not.
    let grow = |doublings: u32| {
        selecting(
            &format!("    let s = \"x\";\n    for i in 0..{doublings} {{ s += s; }}"),
            "s",
        )
    };
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    survives(&sandbox, &grow(13), &recorded);
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let run = start(&sandbox, &grow(21), &[]);
    let error = refused(&run, 3, "message_too_large", &recorded);
    assert_eq!(error["bound"]["name"], "message", "{error}");
}

/// The pid of the worker that `front` launched, once it runs.
fn worker_of(front: u32) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let table = Command::new("ps")
            .args(["-A", "-o", "pid=,ppid=,command="])
            .output()
            .expect("ps runs");
        let table = String::from_utf8_lossy(&table.stdout).into_owned();
        let found = table.lines().find_map(|line| {
            let mut fields = line.split_whitespace();
            let pid: i32 = fields.next()?.parse().ok()?;
            let parent: u32 = fields.next()?.parse().ok()?;
            (parent == front && fields.next() == Some(WORKER_ARG0)).then_some(pid)
        });
        if let Some(pid) = found {
            return pid;
        }
        assert!(
            Instant::now() < deadline,
            "no worker of {front} in\n{table}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn gone(pid: i32) -> bool {
    // SAFETY: signal zero only tests whether the process exists.
    unsafe { libc::kill(pid, 0) == -1 }
}

/// A front that is evaluating a policy that sleeps for ten minutes.
fn sleeping_front(sandbox: &Sandbox) -> std::process::Child {
    sandbox.personal_rhai(&selecting("    sleep(600);", "\"r\""));
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
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn a_signal_during_the_evaluation_kills_and_reaps_the_worker_and_launches_nothing() {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let front = sleeping_front(&sandbox);
        let worker = worker_of(front.id());
        std::thread::sleep(Duration::from_millis(200));
        let started = Instant::now();
        // SAFETY: the pid is this test's live front.
        assert_eq!(unsafe { libc::kill(front.id() as i32, signal) }, 0);
        let output = front.wait_with_output().unwrap();
        // The front raises the signal again once it has cleaned up, so the
        // status is 128 plus the number to a shell.
        assert_eq!(output.status.signal(), Some(signal), "signal {signal}");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("selection_cancelled"), "{stderr}");
        assert!(
            gone(worker),
            "signal {signal}: the worker {worker} survived"
        );
        assert_eq!(recorded.argv(), None, "signal {signal}: a harness started");
        assert_eq!(
            std::fs::read_dir(&sandbox.tmp).unwrap().count(),
            0,
            "signal {signal}: the private directory of the worker is gone"
        );
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "signal {signal}"
        );
    }
}

#[test]
fn a_worker_killed_from_outside_refuses_as_worker_failed() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let front = sleeping_front(&sandbox);
    let worker = worker_of(front.id());
    std::thread::sleep(Duration::from_millis(200));
    // SAFETY: the pid is the live worker of this test's front.
    assert_eq!(unsafe { libc::kill(worker, libc::SIGKILL) }, 0);
    let output = front.wait_with_output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(5),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let error: Value = serde_json::from_str(stderr.lines().last().unwrap()).unwrap();
    assert_eq!(error["error"]["code"], "worker_failed", "{error}");
    assert_eq!(error["error"]["stage"], "worker", "{error}");
    assert_eq!(recorded.argv(), None);
    assert!(gone(worker));
}

#[test]
fn nothing_a_script_returns_can_be_called_after_it_ends() {
    // rhaiscript/rhai#1193: Rhai 1.27.0 loses the imports of a closure that a
    // host calls after the script ended. Ravel calls the named function
    // `select` and nothing else, and the converter refuses every function
    // pointer, so a closure that a script returns never reaches a host call.
    for returned in ["|x| x + 1", "Fn(\"select\")"] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let source = policy(&format!(
            "fn select(request, context) {{\n    #{{ status: \"selected\", program: \"recording-agent\", \
             args: [{returned}], provider: \"p\", model: \"m\", effort: \"e\", reason: \"r\" }}\n}}"
        ));
        let run = start(&sandbox, &source, &[]);
        let error = refused(&run, 3, "selection_malformed", &recorded);
        assert_eq!(error["location"], "result.args[0]", "{returned}: {error}");
    }
}
