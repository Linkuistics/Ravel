//! The shared parts of the tests of `ravel::llm` through the real `ravel
//! start-session` (`docs/specs/policy-evaluation.md`, *Test seams*).
//!
//! A test names its endpoints in the settings of a sandbox, plays a script on a
//! deterministic local server behind each, and reads what the policy saw from
//! the recording harness. `Wire` picks the API type, and the server speaks its
//! wire.

use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::llm_server::{Body, Reply, Server, Wire};
use super::rhai::{Recorded, policy};
use super::{self as support, Run, Sandbox, run};

pub const SETTINGS: &str = ".config/ravel/settings.json";
pub const KEY_VARIABLE: &str = "SEAM_LLM_KEY";
pub const KEY: &str = "sk-seam-credential-91c4e7";
pub const MODEL: &str = "claude-sonnet-5-5";

/// Settings that name each of `endpoints` as an `anthropic` endpoint at the base
/// URL of its server. `extra` adds fields to every entry.
pub fn settings(sandbox: &Sandbox, endpoints: &[(&str, &Server)], extra: &Value) {
    settings_on(Wire::Messages, sandbox, endpoints, extra);
}

/// [`settings`] for the API type of `wire`. The base URL of an OpenAI type ends
/// in `/v1`, as the services publish it.
pub fn settings_on(wire: Wire, sandbox: &Sandbox, endpoints: &[(&str, &Server)], extra: &Value) {
    settings_with_on(wire, sandbox, endpoints, extra, &Value::Null);
}

/// [`settings`], and the owner's `budgets` unless it is null.
pub fn settings_with(
    sandbox: &Sandbox,
    endpoints: &[(&str, &Server)],
    extra: &Value,
    budgets: &Value,
) {
    settings_with_on(Wire::Messages, sandbox, endpoints, extra, budgets);
}

/// [`settings_on`], and the owner's `budgets` unless it is null.
pub fn settings_with_on(
    wire: Wire,
    sandbox: &Sandbox,
    endpoints: &[(&str, &Server)],
    extra: &Value,
    budgets: &Value,
) {
    let mut entries = serde_json::Map::new();
    for (name, server) in endpoints {
        let mut entry = json!({
            "type": wire.api_type(), "apiKeyEnv": KEY_VARIABLE,
            "baseUrl": format!("{}{}", server.url(), wire.base_path()),
        });
        for (field, value) in extra.as_object().into_iter().flatten() {
            entry[field] = value.clone();
        }
        entries.insert((*name).to_owned(), entry);
    }
    let mut document = json!({ "endpoints": entries });
    if !budgets.is_null() {
        document["budgets"] = budgets.clone();
    }
    support::write(&sandbox.home.join(SETTINGS), &document.to_string());
}

/// `start-session` with the credential in the caller's environment, unless
/// `key` is false.
pub fn start(sandbox: &Sandbox, key: bool, args: &[&str]) -> Run {
    run(&mut start_command(sandbox, key, args))
}

pub fn start_command(sandbox: &Sandbox, key: bool, args: &[&str]) -> Command {
    let mut command = sandbox.command();
    if key {
        command.env(KEY_VARIABLE, KEY);
    }
    command
        .args([
            "start-session",
            "--prompt",
            "classify me",
            "--kind",
            "check",
            "--json",
        ])
        .args(args);
    command
}

/// A `start-session` that the test can signal while it runs.
pub fn spawn(sandbox: &Sandbox, args: &[&str]) -> Child {
    start_command(sandbox, true, args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

/// Wait until `server` has read a request, so that a call is in flight.
pub fn await_request(server: &Server) {
    let deadline = Instant::now() + Duration::from_secs(20);
    while server.requests_started() == 0 {
        assert!(Instant::now() < deadline, "no request reached the server");
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A `select` whose `body` computes `args`, an array of strings.
pub fn selecting(body: &str) -> String {
    policy(&format!(
        r#"fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }}
}}"#
    ))
}

/// The call that most tests make: one user message with the prompt, and `more`
/// fields after the required ones.
pub fn ask(endpoint: &str, more: &str) -> String {
    ask_on(Wire::Messages, endpoint, more)
}

/// [`ask`] with the model of `wire`.
pub fn ask_on(wire: Wire, endpoint: &str, more: &str) -> String {
    let model = wire.model();
    format!(
        r#"ravel::llm(#{{ endpoint: "{endpoint}", model: "{model}",
        messages: [#{{ role: "user", content: request.prompt }}]{more} }})"#
    )
}

/// The response map of the one call that `body` makes, as the script saw it.
pub fn answered(sandbox: &Sandbox, recorded: &Recorded, key: bool, call: &str) -> Value {
    sandbox.personal_rhai(&selecting(&format!(
        "    let response = {call};\n    let args = [ravel::json_encode(response)];"
    )));
    start(sandbox, key, &[]).report();
    let args = recorded.argv().unwrap();
    assert_eq!(args.len(), 1, "{args:?}");
    serde_json::from_str(&args[0]).unwrap()
}

/// A sandbox with one endpoint `main` at a server that plays `script`, and the
/// response of the call `more` adds to the fields of `ask`.
pub fn one_call(script: Vec<Reply>, more: &str) -> (Value, Server) {
    one_call_on(Wire::Messages, script, more)
}

/// [`one_call`] on the API type of `wire`.
pub fn one_call_on(wire: Wire, script: Vec<Reply>, more: &str) -> (Value, Server) {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(script);
    settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
    let response = answered(&sandbox, &recorded, true, &ask_on(wire, "main", more));
    (response, server)
}

pub fn refused(run: &Run, code: &str, recorded: &Recorded) -> Value {
    let document = run.refusal(3);
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

pub fn stalled() -> Reply {
    Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Stall {
            hold: Duration::from_secs(30),
        },
    }
}

/// An endpoint that reads the request and never answers.
pub fn silent() -> Reply {
    Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Silent {
            hold: Duration::from_secs(30),
        },
    }
}
