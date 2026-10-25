//! The owner settings of a Rhai policy through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *LLM endpoints and credentials*, *Harness
//! entries*, *Evaluation budgets and their composition* and *Discovery*): the
//! keys `endpoints`, `harnesses` and `budgets`, the credential rules, and the
//! functions `ravel::endpoints`, `ravel::harnesses` and `ravel::remaining`.
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

const MAIN: &str = r#""main": { "type": "anthropic", "apiKeyEnv": "MAIN_KEY" }"#;

#[test]
fn each_settings_failure_refuses_as_settings_invalid_with_exit_2_before_any_worker() {
    let long = "a".repeat(65);
    for (endpoints_and_more, location) in [
        // The name rule.
        (r#""endpoints": { "": { "type": "openai", "apiKeyEnv": "K" } }"#.to_owned(), "endpoints"),
        (r#""endpoints": { "-a": { "type": "openai", "apiKeyEnv": "K" } }"#.to_owned(), "endpoints"),
        (r#""endpoints": { "A": { "type": "openai", "apiKeyEnv": "K" } }"#.to_owned(), "endpoints"),
        (r#""endpoints": { "a_b": { "type": "openai", "apiKeyEnv": "K" } }"#.to_owned(), "endpoints"),
        (
            format!(r#""endpoints": {{ "{long}": {{ "type": "openai", "apiKeyEnv": "K" }} }}"#),
            "endpoints",
        ),
        (r#""harnesses": { "Bad": { "program": "x" } }"#.to_owned(), "harnesses"),
        // The fields.
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "key": "x" } }"#.to_owned(), "endpoints.a.key"),
        (r#""endpoints": { "a": { "apiKeyEnv": "K" } }"#.to_owned(), "endpoints.a.type"),
        (r#""endpoints": { "a": { "type": "bedrock", "apiKeyEnv": "K" } }"#.to_owned(), "endpoints.a.type"),
        (r#""endpoints": { "a": { "type": "anthropic" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "openai-compatible" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (
            r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "structuredOutput": true } }"#.to_owned(),
            "endpoints.a.structuredOutput",
        ),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "models": [1] } }"#.to_owned(), "endpoints.a.models"),
        (r#""harnesses": { "a": { "capabilities": [] } }"#.to_owned(), "harnesses.a.program"),
        (r#""harnesses": { "a": { "program": "x", "env": {} } }"#.to_owned(), "harnesses.a.env"),
        // The base URL.
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "http://api.example.com" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "http://10.0.0.1:8080" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "https://u:p@api.example.com" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "https://api.example.com/v1?key=1" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "https://api.example.com/v1#top" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "ftp://api.example.com" } }"#.to_owned(), "endpoints.a.baseUrl"),
        (r#""endpoints": { "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "not a url" } }"#.to_owned(), "endpoints.a.baseUrl"),
        // The budgets.
        (r#""budgets": { "llmCalls": 8 }"#.to_owned(), "budgets.llmCalls"),
        (r#""budgets": { "llmOutputTokens": 4096 }"#.to_owned(), "budgets.llmOutputTokens"),
        (r#""budgets": { "harnessRuns": -1 }"#.to_owned(), "budgets.harnessRuns"),
        (r#""budgets": { "harnessRuns": 17 }"#.to_owned(), "budgets.harnessRuns"),
        (r#""budgets": { "harnessRuns": "1" }"#.to_owned(), "budgets.harnessRuns"),
        (r#""budgets": { "retries": 1 }"#.to_owned(), "budgets.retries"),
        (r#""budgets": [1]"#.to_owned(), "budgets"),
        // A credential cannot hide under a name that `ravel::env` reads for free.
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "HOME" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "PATH" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "TMPDIR" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "LANG" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "LC_ALL" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
        (r#""endpoints": { "a": { "type": "anthropic", "apiKeyEnv": "LC_MODEL_KEY" } }"#.to_owned(), "endpoints.a.apiKeyEnv"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let contents = format!("{{ {endpoints_and_more} }}");
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

#[test]
fn a_settings_grant_of_a_credential_refuses_as_excluded_grant_with_exit_2() {
    for contents in [
        format!(r#"{{ "policyEnv": ["MAIN_KEY"], "endpoints": {{ {MAIN} }} }}"#),
        format!(r#"{{ "endpoints": {{ {MAIN} }}, "policyEnv": ["X_TOKEN", "MAIN_KEY"] }}"#),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        settings(&sandbox, &contents);
        sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

        let error = refused(&recorded, &start_with(&sandbox, &[], &[]));

        assert_eq!(error["code"], "excluded_grant", "{contents}");
        assert_eq!(error["location"], "policyEnv", "{contents}");
    }
}

/// The scenario "A flag grant of the credential".
#[test]
fn a_flag_grant_of_a_credential_refuses_as_excluded_grant_with_exit_2_and_no_worker() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    settings(&sandbox, &format!(r#"{{ "endpoints": {{ {MAIN} }} }}"#));
    sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

    let run = start_with(
        &sandbox,
        &[("MAIN_KEY", OsStr::new("topsecret"))],
        &["--policy-env", "OTHER", "--policy-env", "MAIN_KEY"],
    );
    let error = refused(&recorded, &run);

    assert_eq!(error["code"], "excluded_grant");
    assert_eq!(error["input"], "--policy-env");
    assert!(!run.stderr.contains("topsecret"), "{}", run.stderr);

    // Without the credential's endpoint, the same flag is an ordinary grant.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));
    start_with(&sandbox, &[], &["--policy-env", "MAIN_KEY"]).report();
    assert_eq!(recorded.argv().unwrap(), ["x"]);
}

/// The scenario "A credential under a base name".
#[test]
fn a_credential_under_a_base_name_refuses_as_settings_invalid_with_exit_2_and_no_worker() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    settings(
        &sandbox,
        r#"{ "endpoints": { "main": { "type": "anthropic", "apiKeyEnv": "LC_MODEL_KEY" } } }"#,
    );
    sandbox.personal_rhai(&rhai::routing(r#"["x"]"#));

    let error = refused(&recorded, &start_with(&sandbox, &[], &[]));

    assert_eq!(error["code"], "settings_invalid");
    assert!(error["message"].as_str().unwrap().contains("LC_MODEL_KEY"));
}

#[test]
fn env_still_refuses_the_variable_of_a_credential_that_the_caller_has_set() {
    let args = computed_with(
        &format!(r#"{{ "endpoints": {{ {MAIN} }} }}"#),
        &[("MAIN_KEY", OsStr::new("topsecret"))],
        r#"fn caught(attempt) {
    try { attempt.call(); } catch (error) { return error.code + ":" + error.message; }
    "none"
}"#,
        r#"    let args = [caught(|| ravel::env("MAIN_KEY"))];"#,
    );

    assert!(args.starts_with("env_not_granted:"), "{args}");
    assert!(!args.contains("topsecret"), "{args}");
}

#[test]
fn endpoints_lists_each_endpoint_in_name_order_with_its_availability() {
    // The keys come out of order, and the order of the answer is the names'.
    let contents = r#"{ "endpoints": {
        "local": { "type": "openai-compatible", "baseUrl": "http://localhost:11434/v1",
                   "structuredOutput": true, "models": ["llama", "qwen"] },
        "plain": { "type": "openai-compatible", "baseUrl": "https://gateway.example/v1" },
        "gpt": { "type": "openai", "apiKeyEnv": "GPT_KEY", "models": [] },
        "claude": { "type": "anthropic", "apiKeyEnv": "CLAUDE_KEY",
                    "baseUrl": "https://proxy.example", "models": ["m1"] },
        "9lives": { "type": "anthropic", "apiKeyEnv": "EMPTY_KEY" }
    } }"#;

    let value = computed_value(
        contents,
        &[
            ("CLAUDE_KEY", OsStr::new("sk-claude")),
            ("EMPTY_KEY", OsStr::new("")),
        ],
        "ravel::endpoints()",
    );

    assert_eq!(
        value,
        json!([
            { "name": "9lives", "type": "anthropic", "available": false,
              "reason": "credential_unset", "models": [], "structuredOutput": true },
            { "name": "claude", "type": "anthropic", "available": true,
              "reason": null, "models": ["m1"], "structuredOutput": true },
            { "name": "gpt", "type": "openai", "available": false,
              "reason": "credential_unset", "models": [], "structuredOutput": true },
            { "name": "local", "type": "openai-compatible", "available": true,
              "reason": null, "models": ["llama", "qwen"], "structuredOutput": true },
            { "name": "plain", "type": "openai-compatible", "available": true,
              "reason": null, "models": [], "structuredOutput": false },
        ])
    );
}

#[test]
fn endpoints_is_an_empty_array_without_settings_and_never_shows_a_credential_or_a_url() {
    let value = computed_value("{}", &[], "ravel::endpoints()");
    assert_eq!(value, json!([]));

    let args = computed_with(
        &format!(r#"{{ "endpoints": {{ {MAIN} }} }}"#),
        &[("MAIN_KEY", OsStr::new("sk-topsecret"))],
        "",
        r#"    let args = [ravel::json_encode(ravel::endpoints())];"#,
    );
    assert!(
        !args.contains("sk-topsecret") && !args.contains("MAIN_KEY"),
        "{args}"
    );
    assert!(!args.contains("http"), "{args}");
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
        &format!(
            r#"{{ "endpoints": {{ {MAIN} }}, "harnesses": {{ "agent": {{ "program": "recording-agent" }} }} }}"#
        ),
    );
    sandbox.file(
        "policy-lib.rhai",
        r#"export const AT_TOP = [ravel::endpoints().len(), ravel::harnesses().len(),
                         ravel::remaining().harnessRuns];"#,
    );
    sandbox.personal_rhai(&policy(
        r#"import "policy-lib" as lib;
fn load_context(request) {
    #{ schemaVersion: 1, facts: #{ endpoints: ravel::endpoints().len(),
                                   harnesses: ravel::harnesses().len(),
                                   harnessRuns: ravel::remaining().harnessRuns } }
}
fn select(request, context) {
    #{ status: "selected", program: "recording-agent",
       args: [lib::AT_TOP.map(|n| n.to_string()).reduce(|sum, n| sum + "," + n, ""),
              context.facts.endpoints.to_string(), context.facts.harnesses.to_string(),
              context.facts.harnessRuns.to_string(), ravel::endpoints().len().to_string()],
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

    assert_eq!(recorded.argv().unwrap(), [",1,1,0", "1", "1", "0", "1"]);
}
