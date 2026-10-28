//! An LLM call is a harness run, and a harness run keeps the credential
//! boundary (`docs/specs/configuration-evaluation.md`, *LLM calls through a harness*,
//! *Credentials*, *Generic LLM interaction* and *Credential boundary*).
//!
//! Everything goes through the real `ravel start-session` against a sandbox
//! home. A fake harness script on the private `PATH` plays the model: it prints
//! a scripted answer, and it records its arguments and its standard input for
//! the test. The real worker and the real runner run in every test. Each case
//! has a firing control in the same test: the same configuration script under a change that
//! makes the opposite result appear, so a check that read clean for every input
//! would fail.

mod support;

use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{Run, Sandbox, run, write};

const SETTINGS: &str = ".config/ravel/settings.json";

/// The credential that no grant names. No other part of the output can hold it
/// by chance: it has no word in it and no run of digits that a time or a pid
/// could make.
const SENTINEL: &str = "sk-ant-SENTINEL-q7Zx9Kv2Lm4Wp8Rt1Yb6Nc3Hd5Jf0Ga";

/// A prompt, and the answers of the fake model, that no other text of a notice
/// can hold either.
const PROMPT: &str = "PROMPT-MARKER-5e1c please refactor the parser";
const HEAVY: &str = "CLASS-heavy-7f3a9d";
const LIGHT: &str = "CLASS-light-2b9d4e";

/// The settings that name `harnesses`, each as its own program, and allow `runs`.
fn settings(sandbox: &Sandbox, harnesses: &[&str], runs: u64) {
    let entries: serde_json::Map<String, Value> = harnesses
        .iter()
        .map(|name| ((*name).to_owned(), json!({ "program": name })))
        .collect();
    let contents = json!({ "harnesses": entries, "budgets": { "harnessRuns": runs } });
    write(&sandbox.home.join(SETTINGS), &contents.to_string());
}

/// A harness script `name` on the private `PATH`.
fn harness(sandbox: &Sandbox, name: &str, body: &str) {
    let script = sandbox.bin.join(name);
    write(&script, &format!("#!/bin/sh\n{body}\n"));
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
}

/// A fake model `name` that records its arguments and its standard input, and
/// then runs `answer`, a shell fragment that prints the answer. The record is
/// `<name>.argv` with one argument per line, and `<name>.stdin`.
fn fake_model(sandbox: &Sandbox, name: &str, answer: &str) -> Model {
    let model = Model {
        argv: sandbox.root.join(format!("{name}.argv")),
        stdin: sandbox.root.join(format!("{name}.stdin")),
    };
    harness(
        sandbox,
        name,
        &format!(
            "printf '%s\\n' \"$@\" > {:?}\ncat > {:?}\n{answer}",
            model.argv, model.stdin
        ),
    );
    model
}

/// What a fake model saw.
struct Model {
    argv: PathBuf,
    stdin: PathBuf,
}

impl Model {
    fn argv(&self) -> Vec<String> {
        fs::read_to_string(&self.argv)
            .unwrap_or_else(|error| panic!("{}: {error}", self.argv.display()))
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn stdin(&self) -> String {
        fs::read_to_string(&self.stdin).unwrap()
    }

    fn started(&self) -> bool {
        self.argv.exists()
    }
}

/// `start-session` with `prompt`, the extra `env` of the caller and `args`.
fn start(sandbox: &Sandbox, prompt: &str, env: &[(&str, &str)], args: &[&str]) -> Run {
    let mut command = sandbox.command();
    command
        .args([
            "start-session",
            "--prompt",
            prompt,
            "--kind",
            "check",
            "--json",
        ])
        .args(args);
    for (name, value) in env {
        command.env(name, OsStr::new(value));
    }
    run(&mut command)
}

/// The routing notice of a session that ended with the managed process's value.
/// Nothing but the notice reached stderr, so a configuration script that prints nothing leaves
/// no diagnostic behind.
fn notice_of(run: &Run) -> Value {
    let notice = run.report();
    assert_eq!(run.stderr.lines().count(), 1, "{}", run.stderr);
    notice
}

/// The notice lists each run with its harness and its status, and an elapsed
/// time. A run that sent nothing says so.
fn assert_runs(notice: &Value, expected: &[(&str, &str)]) {
    let listed = notice["harnessRuns"].as_array().expect("harnessRuns");
    assert_eq!(listed.len(), expected.len(), "{notice}");
    for (run, (harness, status)) in listed.iter().zip(expected) {
        assert_eq!(run["harness"], *harness, "{notice}");
        assert_eq!(run["status"], *status, "{notice}");
        assert!(run["elapsedMs"].as_u64().is_some(), "{notice}");
        assert_eq!(run["sentResult"], false, "{notice}");
    }
}

/// The notice holds none of `texts`. The control is the same document holding
/// what it does list, so the check reads the document that would carry them.
fn assert_withheld(notice: &Value, texts: &[&str]) {
    let whole = notice.to_string();
    assert!(whole.contains("harnessRuns"), "{whole}");
    for text in texts {
        assert!(!whole.contains(text), "the notice holds {text:?}: {whole}");
    }
}

/// The notice without the report of the delivered context. A configuration script that
/// copies an answer into the context it delivers publishes it there, and the
/// notice has always reported that context.
fn without_context(notice: &Value) -> Value {
    let mut rest = notice.clone();
    rest.as_object_mut().unwrap().remove("context");
    rest
}

/// A route that the recording agent starts, from `name` and the configuration script's `args`.
fn routed(body: &str, args: &str) -> String {
    rhai_script(&format!(
        r#"{body}
fn select(request, context) {{
    let args = {args};
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p",
       model: "m", effort: "e", reason: "r" }}
}}"#
    ))
}

/// The configuration script of "LLM-assisted selection": it asks the model `classifier` to
/// classify the prompt, and maps the answer to a route.
const SELECTION: &str = r#"fn select(request, context) {
    let routes = #{
        "CLASS-heavy-7f3a9d": #{ model: "big", effort: "high", name: "deep-route" },
        "CLASS-light-2b9d4e": #{ model: "small", effort: "low", name: "quick-route" },
    };
    let run = ravel::run_harness(#{
        harness: "classifier",
        args: ["-p", "--model", "tiny", "--output-format", "text"],
        stdin: "Classify the request as heavy or light.\n\n" + request.prompt,
    });
    let answer = run.stdout;
    answer.trim();
    let route = routes[answer];
    #{ status: "selected", program: "recording-agent", args: ["routed", route.name],
       provider: "p", model: route.model, effort: route.effort, reason: "classified" }
}"#;

#[test]
fn an_llm_assisted_selection_launches_the_route_that_the_answer_maps_to() {
    // The classifier reads its standard input, so the answer follows the prompt.
    let classify = |sandbox: &Sandbox, stdin: &str| {
        fake_model(
            sandbox,
            "classifier",
            &format!("if grep -q refactor {stdin:?}; then echo {HEAVY}; else echo {LIGHT}; fi"),
        )
    };
    let attempt = |prompt: &str| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let stdin = sandbox.root.join("classifier.stdin");
        let model = classify(&sandbox, stdin.to_str().unwrap());
        settings(&sandbox, &["classifier"], 1);
        sandbox.personal_rhai(&rhai_script(SELECTION));
        let run = start(&sandbox, prompt, &[], &[]);
        (notice_of(&run), recorded, model, sandbox)
    };

    let (notice, recorded, model, _keep) = attempt(PROMPT);

    // Ravel launched the route that the heavy answer maps to.
    assert_eq!(recorded.argv().unwrap(), ["routed", "deep-route"]);
    assert_eq!(notice["selection"]["model"], "big");
    // The fake model ran once, as one harness run, with exactly the script's
    // arguments and standard input.
    assert_runs(&notice, &[("classifier", "exited")]);
    assert_eq!(
        model.argv(),
        ["-p", "--model", "tiny", "--output-format", "text"]
    );
    assert_eq!(
        model.stdin(),
        format!("Classify the request as heavy or light.\n\n{PROMPT}")
    );
    // Neither the prompt nor the answer is in the notice.
    assert_withheld(&notice, &[PROMPT, HEAVY, LIGHT]);

    // The firing control: a prompt that the classifier answers otherwise takes
    // the other route, so the route came from the answer and not from a default.
    let (notice, recorded, _, _keep) = attempt("PROMPT-MARKER-5e1c please rename the file");
    assert_eq!(recorded.argv().unwrap(), ["routed", "quick-route"]);
    assert_eq!(notice["selection"]["model"], "small");
    assert_runs(&notice, &[("classifier", "exited")]);
}

#[test]
fn load_context_runs_a_fake_harness_for_a_summary_and_select_receives_it() {
    let config_source = routed(
        r#"fn load_context(request) {
    let run = ravel::run_harness(#{
        harness: "summariser",
        args: ["-p", "summarise"],
        stdin: "Summarise this for a router.\n" + request.prompt,
    });
    #{ schemaVersion: 1, facts: #{ summary: run.stdout } }
}"#,
        "[context.facts.summary]",
    );
    let attempt = |summary: &str| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let model = fake_model(&sandbox, "summariser", &format!("printf '%s' {summary:?}"));
        settings(&sandbox, &["summariser"], 1);
        sandbox.personal_rhai(&config_source);
        let run = start(&sandbox, PROMPT, &[], &[]);
        (notice_of(&run), recorded, model, sandbox)
    };

    let (notice, recorded, model, _keep) = attempt("SUMMARY-text-a81f");

    // `select` got the summary in the delivered context.
    assert_eq!(recorded.argv().unwrap(), ["SUMMARY-text-a81f"]);
    assert_eq!(model.argv(), ["-p", "summarise"]);
    assert_eq!(
        model.stdin(),
        format!("Summarise this for a router.\n{PROMPT}")
    );
    assert_runs(&notice, &[("summariser", "exited")]);
    assert_withheld(&without_context(&notice), &[PROMPT, "SUMMARY-text-a81f"]);
    // The notice reports the context that the configuration script delivered, and the configuration script
    // put the summary there. That is the one place the summary reaches it.
    assert_eq!(
        notice["context"]["value"]["facts"]["summary"], "SUMMARY-text-a81f",
        "{notice}"
    );

    // The firing control: another summary is another delivered fact.
    let (notice, recorded, _, _keep) = attempt("SUMMARY-text-c02e");
    assert_eq!(recorded.argv().unwrap(), ["SUMMARY-text-c02e"]);
    assert_withheld(&without_context(&notice), &["SUMMARY-text-c02e"]);
    assert_eq!(
        notice["context"]["value"]["facts"]["summary"], "SUMMARY-text-c02e",
        "{notice}"
    );
}

#[test]
fn a_structured_answer_that_lacks_a_property_is_json_mismatch_without_the_answer() {
    // The configuration script checks the answer with `json_check` under `try`, and goes on
    // with the verdict when the check throws.
    let source = rhai_script(
        r#"fn judged(request) {
    let run = ravel::run_harness(#{ harness: "judge", args: ["--json"], stdin: request.prompt });
    let schema = #{ type: "object",
        properties: #{ label: #{ type: "string" }, confidence: #{ type: "number" } },
        required: ["label", "confidence"], additionalProperties: false };
    try {
        let answer = ravel::json_check(ravel::json_parse(run.stdout), schema);
        return ["accepted", answer.label];
    } catch (error) {
        return ["caught", error.code, error.message];
    }
}
fn select(request, context) {
    let verdict = judged(request);
    verdict.push("went-on");
    #{ status: "selected", program: "recording-agent", args: verdict, provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    );
    let attempt = |answer: &str| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        fake_model(&sandbox, "judge", &format!("printf '%s' '{answer}'"));
        settings(&sandbox, &["judge"], 1);
        sandbox.personal_rhai(&source);
        let run = start(&sandbox, PROMPT, &[], &[]);
        (notice_of(&run), recorded, sandbox)
    };

    // The answer lacks `confidence`, and its label is text that nothing else holds.
    let (notice, recorded, _keep) = attempt(r#"{"label":"ANSWER-label-d4b7e1"}"#);

    let seen = recorded.argv().unwrap();
    assert_eq!(
        seen,
        [
            "caught",
            "json_mismatch",
            "the value does not match the schema: value: the required property confidence is missing",
            "went-on",
        ]
    );
    assert!(
        seen.iter()
            .all(|part| !part.contains("ANSWER-label-d4b7e1")),
        "the catch block got part of the answer: {seen:?}"
    );
    assert_runs(&notice, &[("judge", "exited")]);
    assert_withheld(&notice, &[PROMPT, "ANSWER-label-d4b7e1"]);

    // The firing control: the same answer with the property passes the check.
    let (notice, recorded, _keep) = attempt(r#"{"label":"ANSWER-label-d4b7e1","confidence":0.5}"#);
    assert_eq!(
        recorded.argv().unwrap(),
        ["accepted", "ANSWER-label-d4b7e1", "went-on"]
    );
    assert_withheld(&notice, &["ANSWER-label-d4b7e1"]);
}

#[test]
fn a_failed_call_is_data_and_the_config_falls_back_to_a_second_harness() {
    let source = rhai_script(
        r#"fn ask(name, prompt) {
    ravel::run_harness(#{ harness: name, args: ["-p"], stdin: prompt })
}
fn select(request, context) {
    let first = ask("primary", request.prompt);
    let seen = [first.status, first.exitCode.to_string()];
    let answer = first.stdout;
    if first.status != "exited" || first.exitCode != 0 {
        let second = ask("fallback", request.prompt);
        seen.push(second.status);
        answer = second.stdout;
    }
    answer.trim();
    seen.push(answer);
    #{ status: "selected", program: "recording-agent", args: seen, provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    );
    let attempt = |primary: &str| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let first = fake_model(&sandbox, "primary", primary);
        let second = fake_model(&sandbox, "fallback", "echo ANSWER-second-93ac");
        settings(&sandbox, &["primary", "fallback"], 2);
        sandbox.personal_rhai(&source);
        let run = start(&sandbox, PROMPT, &[], &[]);
        (notice_of(&run), recorded, first, second, sandbox)
    };

    // The first call exits 1 with a rate-limit message.
    let (notice, recorded, first, second, _keep) =
        attempt("echo 'rate limit reached, retry in 60s' >&2\nexit 1");

    // The script saw `exited` with exit code 1, and the session used the answer
    // of the second harness.
    assert_eq!(
        recorded.argv().unwrap(),
        ["exited", "1", "exited", "ANSWER-second-93ac"]
    );
    assert_eq!(first.stdin(), PROMPT);
    assert_eq!(second.stdin(), PROMPT);
    assert_runs(&notice, &[("primary", "exited"), ("fallback", "exited")]);
    assert_eq!(notice["harnessRuns"][0]["exitCode"], 1, "{notice}");
    assert_eq!(notice["harnessRuns"][1]["exitCode"], 0, "{notice}");
    assert_withheld(
        &notice,
        &[PROMPT, "rate limit reached", "ANSWER-second-93ac"],
    );

    // The firing control: a first call that succeeds leaves the second one
    // unrun, so the fallback came from the failure and not from the configuration script's order.
    let (notice, recorded, _, second, _keep) = attempt("echo ANSWER-first-5d20");
    assert_eq!(
        recorded.argv().unwrap(),
        ["exited", "0", "ANSWER-first-5d20"]
    );
    assert!(!second.started(), "the fallback ran after a success");
    assert_runs(&notice, &[("primary", "exited")]);
}

/// A configuration script that makes no measured read and runs no harness. It asks for the
/// credential by name, prints every other value that it can reach, and prints
/// the credential too when `ravel::env` returns it.
const PROBE: &str = r#"fn select(request, context) {
    let outcome = "";
    try {
        let value = ravel::env("ANTHROPIC_API_KEY");
        print("probe got " + value);
        outcome = "returned:" + value;
    } catch (error) {
        print("probe failed " + error.message);
        debug("probe code " + error.code + " name " + error.name);
        outcome = error.code;
    }
    print(ravel::json_encode(request));
    print(ravel::json_encode(context));
    print(ravel::json_encode(ravel::harnesses()));
    print(ravel::json_encode(ravel::remaining()));
    #{ status: "selected", program: "recording-agent", args: [outcome], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#;

#[test]
fn an_ungranted_credential_in_the_caller_environment_reaches_no_script_value_or_stream() {
    let probe = |grants: &[&str]| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        sandbox.personal_rhai(&rhai_script(PROBE));
        let run = start(&sandbox, PROMPT, &[("ANTHROPIC_API_KEY", SENTINEL)], grants);
        // The route started, and the session ended with the managed process's
        // value, so nothing but the probe's own prints is a diagnostic.
        assert_eq!(run.code, Some(0), "{}", run.stderr);
        (run, recorded, sandbox)
    };

    let (run, recorded, _keep) = probe(&[]);

    // `env` threw `env_not_granted`, and the sentinel is in no stream: not the
    // notice, not a diagnostic and not the final value.
    assert_eq!(recorded.argv().unwrap(), ["env_not_granted"]);
    let notice = run.report();
    assert_eq!(notice["reads"], json!([]), "{notice}");
    assert_eq!(notice["harnessRuns"], json!([]), "{notice}");
    assert!(run.stderr.contains("probe failed"), "{}", run.stderr);
    assert!(!run.stderr.contains(SENTINEL), "{}", run.stderr);
    assert!(!run.stdout.contains(SENTINEL), "{}", run.stdout);

    // The firing control: with a grant that names the sentinel's variable, the
    // same configuration script reads the sentinel, and the streams that were clean now hold
    // it. The checks above therefore read the streams that would carry a leak.
    let (run, recorded, _keep) = probe(&["--config-env", "ANTHROPIC_API_KEY"]);
    assert_eq!(recorded.argv().unwrap(), [format!("returned:{SENTINEL}")]);
    assert!(run.stderr.contains(SENTINEL), "{}", run.stderr);
}

#[test]
fn an_unconfined_run_gets_the_caller_home_and_the_ungranted_credential_unchanged() {
    let source = rhai_script(
        r#"fn select(request, context) {
    let probe = "";
    try { ravel::env("ANTHROPIC_API_KEY"); probe = "returned"; }
    catch (error) { probe = error.code; }
    let run = ravel::run_harness(#{ harness: "sign-in-finder", args: [] });
    let args = [probe, run.status];
    for line in run.stdout.split("\n") { args.push(line); }
    #{ status: "selected", program: "recording-agent", args: args, provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    );
    let attempt = |env: &[(&str, &str)]| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        harness(
            &sandbox,
            "sign-in-finder",
            "printf '%s\\n' \"$HOME\"\nprintf '%s' \"${ANTHROPIC_API_KEY-unset}\"",
        );
        settings(&sandbox, &["sign-in-finder"], 1);
        sandbox.personal_rhai(&source);
        let run = start(&sandbox, PROMPT, env, &[]);
        let notice = notice_of(&run);
        (sandbox, recorded, notice)
    };

    let (sandbox, recorded, notice) = attempt(&[("ANTHROPIC_API_KEY", SENTINEL)]);

    // The script cannot read the credential, yet the harness run finds it,
    // with the caller's `HOME`, both unchanged.
    let home = sandbox.home.to_str().unwrap();
    assert_eq!(
        recorded.argv().unwrap(),
        ["env_not_granted", "exited", home, SENTINEL]
    );
    // The run is listed with its harness, status and time, and the notice holds
    // no output of it.
    assert_runs(&notice, &[("sign-in-finder", "exited")]);
    assert_withheld(&notice, &[SENTINEL, PROMPT]);

    // The firing control: without the variable in the caller's environment the
    // run reports it unset, so the value above came from the caller.
    let (sandbox, recorded, _) = attempt(&[]);
    assert_eq!(
        recorded.argv().unwrap(),
        [
            "env_not_granted",
            "exited",
            sandbox.home.to_str().unwrap(),
            "unset"
        ]
    );
}
