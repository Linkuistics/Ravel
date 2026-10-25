//! The owner settings of a Rhai policy through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *Harness entries*, *Evaluation budgets
//! and their composition* and *Discovery*): the keys `harnesses` and `budgets`,
//! and the functions `ravel::harnesses` and `ravel::remaining`.
//!
//! Everything runs in a sandbox home with fake harness scripts on a private
//! `PATH`. No test makes a network call, and none could: Ravel has no HTTP
//! client. The policy hands what it computed to the recording harness as
//! arguments, so a test sees what the script saw.

mod support;

use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt as _;

use serde_json::{Value, json};
use support::rhai::{self, Recorded, policy};
use support::{Run, Sandbox, run, text};

const SETTINGS: &str = ".config/ravel/settings.json";

fn settings(sandbox: &Sandbox, contents: &str) {
    support::write(&sandbox.home.join(SETTINGS), contents);
}

/// `start-session` with `envs` in the caller's environment.
fn start_with(sandbox: &Sandbox, envs: &[(&str, &OsStr)], args: &[&str]) -> Run {
    let mut command = sandbox.command();
    for (name, value) in envs {
        command.env(name, value);
    }
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .args(args);
    run(&mut command)
}

/// A `select` whose `body` computes `args`, an array of strings, after the
/// global functions in `globals`.
fn selecting(globals: &str, body: &str) -> String {
    policy(&format!(
        r#"{globals}
fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }}
}}"#
    ))
}

/// The one string that a policy with `body` computed, with `envs` in the
/// caller's environment, under the settings `contents`.
fn computed_with(contents: &str, envs: &[(&str, &OsStr)], globals: &str, body: &str) -> String {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    settings(&sandbox, contents);
    sandbox.personal_rhai(&selecting(globals, body));
    start_with(&sandbox, envs, &[]).report();
    let mut args = recorded.argv().unwrap();
    assert_eq!(args.len(), 1, "{args:?}");
    args.remove(0)
}

fn computed_value(contents: &str, envs: &[(&str, &OsStr)], expression: &str) -> Value {
    let body = format!("    let args = [ravel::json_encode({expression})];");
    serde_json::from_str(&computed_with(contents, envs, "", &body)).unwrap()
}

fn refused(recorded: &Recorded, run: &Run) -> Value {
    let document = run.refusal(2);
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

#[test]
fn each_settings_failure_refuses_as_settings_invalid_with_exit_2_before_any_worker() {
    let long = "a".repeat(65);
    for (entries, location) in [
        // The name rule.
        (
            r#""harnesses": { "Bad": { "program": "x" } }"#.to_owned(),
            "harnesses",
        ),
        (
            r#""harnesses": { "": { "program": "x" } }"#.to_owned(),
            "harnesses",
        ),
        (
            r#""harnesses": { "-a": { "program": "x" } }"#.to_owned(),
            "harnesses",
        ),
        (
            format!(r#""harnesses": {{ "{long}": {{ "program": "x" }} }}"#),
            "harnesses",
        ),
        // The fields.
        (
            r#""harnesses": { "a": { "capabilities": [] } }"#.to_owned(),
            "harnesses.a.program",
        ),
        (
            r#""harnesses": { "a": { "program": "x", "env": {} } }"#.to_owned(),
            "harnesses.a.env",
        ),
        // The budgets.
        (
            r#""budgets": { "llmCalls": 8 }"#.to_owned(),
            "budgets.llmCalls",
        ),
        (
            r#""budgets": { "llmOutputTokens": 4096 }"#.to_owned(),
            "budgets.llmOutputTokens",
        ),
        (
            r#""budgets": { "harnessRuns": -1 }"#.to_owned(),
            "budgets.harnessRuns",
        ),
        (
            r#""budgets": { "harnessRuns": 17 }"#.to_owned(),
            "budgets.harnessRuns",
        ),
        (
            r#""budgets": { "harnessRuns": "1" }"#.to_owned(),
            "budgets.harnessRuns",
        ),
        (
            r#""budgets": { "retries": 1 }"#.to_owned(),
            "budgets.retries",
        ),
        (r#""budgets": [1]"#.to_owned(), "budgets"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let contents = format!("{{ {entries} }}");
        settings(&sandbox, &contents);
        sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

        let error = refused(&recorded, &start_with(&sandbox, &[], &[]));

        assert_eq!(error["code"], "settings_invalid", "{contents}");
        assert_eq!(error["location"], location, "{contents}");
        assert!(
            error["message"].as_str().unwrap().contains(SETTINGS),
            "{contents}: {error}"
        );
    }
}

/// The two budgets of `ravel::llm` are gone, so a settings file that still sets
/// one refuses as an unknown budget, and no policy starts.
#[test]
fn the_budgets_of_the_removed_llm_call_are_unknown_budgets() {
    for key in ["llmCalls", "llmOutputTokens"] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        settings(&sandbox, &format!(r#"{{ "budgets": {{ "{key}": 8 }} }}"#));
        sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

        let error = refused(&recorded, &start_with(&sandbox, &[], &[]));

        assert_eq!(error["code"], "settings_invalid", "{key}");
        assert_eq!(error["location"], format!("budgets.{key}"), "{key}");
        let message = error["message"].as_str().unwrap();
        assert!(message.contains("an unknown budget"), "{key}: {message}");
        assert!(message.contains("harnessRuns"), "{key}: {message}");
    }
}

/// `endpoints` held the LLM endpoints of the removed `ravel::llm`. The settings
/// file names no such key now, so a file that still holds one refuses before any
/// worker starts, and the refusal names the key.
#[test]
fn the_endpoints_key_is_an_unknown_key_and_the_refusal_names_it() {
    for contents in [
        r#"{ "endpoints": {} }"#,
        r#"{ "endpoints": { "main": { "type": "anthropic", "apiKeyEnv": "MAIN_KEY" } } }"#,
        r#"{ "harnesses": {}, "endpoints": [] }"#,
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        settings(&sandbox, contents);
        sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

        let error = refused(&recorded, &start_with(&sandbox, &[], &[]));

        assert_eq!(error["code"], "settings_invalid", "{contents}");
        assert_eq!(error["location"], "endpoints", "{contents}");
        let message = error["message"].as_str().unwrap();
        assert!(message.contains(SETTINGS), "{contents}: {message}");
        assert!(
            message.contains(r#"the unknown key "endpoints""#),
            "{contents}: {message}"
        );
    }
}

/// The variable that an endpoint named as its credential is an ordinary name
/// again: `--policy-env` grants it, and `ravel::env` reads it.
#[test]
fn a_variable_that_an_endpoint_once_named_as_its_credential_is_an_ordinary_grant() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(
        "",
        r#"    let args = [ravel::env("MAIN_KEY")];"#,
    ));
    start_with(
        &sandbox,
        &[("MAIN_KEY", OsStr::new("granted-value"))],
        &["--policy-env", "MAIN_KEY"],
    )
    .report();
    assert_eq!(recorded.argv().unwrap(), ["granted-value"]);
}

/// `ravel::endpoints()` is gone, so a policy that calls it fails as any call of
/// a function that the namespace lacks does. The control beside it calls a
/// function that stays, in the same position, and succeeds.
#[test]
fn a_policy_that_calls_ravel_endpoints_fails_with_an_unknown_function() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(
        "",
        r#"    let args = [ravel::json_encode(ravel::endpoints())];"#,
    ));

    let run = start_with(&sandbox, &[], &[]);

    let document = run.refusal(3);
    assert_eq!(recorded.argv(), None, "a harness started after a failure");
    assert_eq!(document["error"]["code"], "selection_threw", "{document}");
    let message = document["error"]["message"].as_str().unwrap();
    assert!(
        message.contains("Function not found: ravel::endpoints"),
        "{document}"
    );

    let control = computed_value("{}", &[], "ravel::harnesses()");
    assert_eq!(control, json!([]));
}

/// A fake program on the private `PATH`, with the mode that it should have.
fn program(sandbox: &Sandbox, relative: &str, mode: u32) -> std::path::PathBuf {
    let path = sandbox.root.join(relative);
    support::write(&path, "#!/bin/sh\ntouch \"$0.ran\"\n");
    fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
    path
}

#[test]
fn harnesses_resolves_each_program_as_a_selected_program_resolves_and_runs_nothing() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let on_path = program(&sandbox, "bin/on-path", 0o755);
    program(&sandbox, "bin/plain-file", 0o644);
    let absolute = program(&sandbox, "elsewhere/absolute-agent", 0o755);
    program(&sandbox, "work/tools/relative-agent", 0o755);
    let directory = sandbox.root.join("elsewhere/a-directory");
    fs::create_dir_all(&directory).unwrap();
    settings(
        &sandbox,
        &format!(
            r#"{{ "harnesses": {{
            "z-on-path": {{ "program": "on-path", "capabilities": ["edit", "shell"] }},
            "a-absolute": {{ "program": {absolute:?} }},
            "m-relative": {{ "program": "./tools/relative-agent", "capabilities": [] }},
            "b-missing": {{ "program": "no-such-agent" }},
            "c-missing-path": {{ "program": "/no/such/dir/agent" }},
            "d-plain": {{ "program": "plain-file" }},
            "e-directory": {{ "program": {directory:?} }}
        }} }}"#
        ),
    );
    sandbox.personal_rhai(&selecting(
        "",
        "    let args = [ravel::json_encode(ravel::harnesses())];",
    ));

    start_with(&sandbox, &[], &[]).report();

    let reported: Value = serde_json::from_str(&recorded.argv().unwrap()[0]).unwrap();
    let path = |p: &std::path::Path| text(p);
    assert_eq!(
        reported,
        json!([
            { "name": "a-absolute", "program": path(&absolute), "available": true,
              "resolved": path(&absolute), "reason": null, "capabilities": [] },
            { "name": "b-missing", "program": "no-such-agent", "available": false,
              "resolved": null, "reason": "not_found", "capabilities": [] },
            { "name": "c-missing-path", "program": "/no/such/dir/agent", "available": false,
              "resolved": null, "reason": "not_found", "capabilities": [] },
            { "name": "d-plain", "program": "plain-file", "available": false,
              "resolved": null, "reason": "not_executable", "capabilities": [] },
            { "name": "e-directory", "program": path(&directory), "available": false,
              "resolved": null, "reason": "not_executable", "capabilities": [] },
            { "name": "m-relative", "program": "./tools/relative-agent", "available": true,
              "resolved": format!("{}/./tools/relative-agent", text(&sandbox.cwd)),
              "reason": null, "capabilities": [] },
            { "name": "z-on-path", "program": "on-path", "available": true,
              "resolved": path(&on_path), "reason": null, "capabilities": ["edit", "shell"] },
        ])
    );
    // Discovery resolves and runs nothing.
    for ran in [
        "bin/on-path.ran",
        "elsewhere/absolute-agent.ran",
        "work/tools/relative-agent.ran",
    ] {
        assert!(!sandbox.root.join(ran).exists(), "{ran} ran");
    }
}

#[test]
fn harnesses_is_an_empty_array_without_entries() {
    assert_eq!(computed_value("{}", &[], "ravel::harnesses()"), json!([]));
}

#[test]
fn remaining_reports_the_defaults_and_the_time_that_is_left() {
    let value = computed_value("{}", &[], "ravel::remaining()");

    let object = value.as_object().unwrap();
    let mut names: Vec<&str> = object.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(names, ["harnessRuns", "ms", "sources"]);
    assert_eq!(value["harnessRuns"], 0);
    assert_eq!(value["sources"], 256);
    let ms = value["ms"].as_u64().unwrap();
    assert!((1..=30_000).contains(&ms), "{ms}");
}

#[test]
fn remaining_counts_down_the_time_and_the_sources_and_reports_the_owners_budgets() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    settings(&sandbox, r#"{ "budgets": { "harnessRuns": 2 } }"#);
    sandbox.file("facts.txt", "x");
    sandbox.file("context.json", r#"{"schemaVersion": 1, "facts": {}}"#);
    sandbox.personal_rhai(&selecting(
        "",
        r#"    let before = ravel::remaining();
    ravel::read_text("facts.txt");
    let broken = "none";
    try { ravel::read_text("missing.txt"); } catch (error) { broken = error.code; }
    let after = ravel::remaining();
    let args = [ravel::json_encode(#{ before: before, after: after, broken: broken,
                                       limits: request.limits })];"#,
    ));

    start_with(
        &sandbox,
        &[],
        &["--timeout-ms", "20000", "--context", "context.json"],
    )
    .report();

    let seen: Value = serde_json::from_str(&recorded.argv().unwrap()[0]).unwrap();
    // The caller's document and one read use two sources. A missing file reads
    // nothing, so it uses none.
    assert_eq!(seen["before"]["sources"], 255);
    assert_eq!(seen["after"]["sources"], 254);
    assert_eq!(seen["broken"], "source_missing");
    assert_eq!(seen["before"]["harnessRuns"], 2);
    assert_eq!(seen["after"]["harnessRuns"], 2);
    assert_eq!(seen["limits"]["harnessRuns"], 2);
    let (before, after) = (
        seen["before"]["ms"].as_u64().unwrap(),
        seen["after"]["ms"].as_u64().unwrap(),
    );
    assert!(after <= before && before <= 20_000, "{before} {after}");
    assert_eq!(seen["limits"]["selectionMs"], 20_000);
}

#[test]
fn a_policy_cannot_raise_a_budget_it_reads() {
    let value = computed_value(
        r#"{ "budgets": { "harnessRuns": 2 } }"#,
        &[],
        r#"{
        let mine = request.limits;
        mine.harnessRuns = 16;
        request.limits.harnessRuns = 16;
        #{ remaining: ravel::remaining(), limits: request.limits, mine: mine }
    }"#,
    );

    assert_eq!(value["remaining"]["harnessRuns"], 2);
    assert_eq!(value["mine"]["harnessRuns"], 16);
}

#[test]
fn the_discovery_functions_are_available_at_the_top_of_a_module_and_in_load_context_and_select() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    settings(
        &sandbox,
        r#"{ "harnesses": { "agent": { "program": "recording-agent" } } }"#,
    );
    sandbox.file(
        "policy-lib.rhai",
        r#"export const AT_TOP = [ravel::harnesses().len(), ravel::remaining().harnessRuns];"#,
    );
    sandbox.personal_rhai(&policy(
        r#"import "policy-lib" as lib;
fn load_context(request) {
    #{ schemaVersion: 1, facts: #{ harnesses: ravel::harnesses().len(),
                                   harnessRuns: ravel::remaining().harnessRuns } }
}
fn select(request, context) {
    #{ status: "selected", program: "recording-agent",
       args: [lib::AT_TOP.map(|n| n.to_string()).reduce(|sum, n| sum + "," + n, ""),
              context.facts.harnesses.to_string(), context.facts.harnessRuns.to_string(),
              ravel::harnesses().len().to_string()],
       provider: "p", model: "m", effort: "e", reason: "r" }
}"#,
    ));
    // The module sits beside the entry, so the import finds it by its name.
    fs::rename(
        sandbox.cwd.join("policy-lib.rhai"),
        sandbox.home.join(".config/ravel/policy-lib.rhai"),
    )
    .unwrap();

    start_with(&sandbox, &[], &[]).report();

    assert_eq!(recorded.argv().unwrap(), [",1,0", "1", "0", "1"]);
}
