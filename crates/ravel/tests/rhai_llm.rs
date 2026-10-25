//! `ravel::llm` for the `anthropic` API type through the real `ravel
//! start-session` (`docs/specs/policy-evaluation.md`, *LLM calls*, *LLM
//! endpoints and credentials*, and the requirements *Generic LLM interaction*,
//! *Endpoint failures are data* and *Credential boundary*).
//!
//! The real Rig client and the real worker run in every test. A deterministic
//! local HTTP server stands behind the endpoint of the settings, with a dummy
//! key and no network. The policy hands what it saw to the recording harness as
//! arguments, so a test sees what the script saw.

mod support;

use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::llm_server::{self, Body, Reply, Server};
use support::rhai::{Recorded, policy};
use support::{Run, Sandbox, run};

const SETTINGS: &str = ".config/ravel/settings.json";
const KEY_VARIABLE: &str = "SEAM_LLM_KEY";
const KEY: &str = "sk-seam-credential-91c4e7";
const MODEL: &str = "claude-sonnet-5-5";

/// Settings that name each of `endpoints` as an `anthropic` endpoint at the base
/// URL of its server. `extra` adds fields to every entry.
fn settings(sandbox: &Sandbox, endpoints: &[(&str, &Server)], extra: &Value) {
    let mut entries = serde_json::Map::new();
    for (name, server) in endpoints {
        let mut entry = json!({
            "type": "anthropic", "apiKeyEnv": KEY_VARIABLE, "baseUrl": server.url(),
        });
        for (field, value) in extra.as_object().into_iter().flatten() {
            entry[field] = value.clone();
        }
        entries.insert((*name).to_owned(), entry);
    }
    support::write(
        &sandbox.home.join(SETTINGS),
        &json!({ "endpoints": entries }).to_string(),
    );
}

/// `start-session` with the credential in the caller's environment, unless
/// `key` is false.
fn start(sandbox: &Sandbox, key: bool, args: &[&str]) -> Run {
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
    run(&mut command)
}

/// A `select` whose `body` computes `args`, an array of strings.
fn selecting(body: &str) -> String {
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
fn ask(endpoint: &str, more: &str) -> String {
    format!(
        r#"ravel::llm(#{{ endpoint: "{endpoint}", model: "{MODEL}",
        messages: [#{{ role: "user", content: request.prompt }}]{more} }})"#
    )
}

/// The response map of the one call that `body` makes, as the script saw it.
fn answered(sandbox: &Sandbox, recorded: &Recorded, key: bool, call: &str) -> Value {
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
fn one_call(script: Vec<Reply>, more: &str) -> (Value, Server) {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(script);
    settings(&sandbox, &[("main", &server)], &json!({}));
    let response = answered(&sandbox, &recorded, true, &ask("main", more));
    (response, server)
}

fn refused(run: &Run, code: &str, recorded: &Recorded) -> Value {
    let document = run.refusal(3);
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

fn stalled() -> Reply {
    Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Stall {
            hold: Duration::from_secs(30),
        },
    }
}

#[test]
fn select_asks_an_endpoint_to_classify_the_prompt_and_routes_on_the_answer() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("review")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let response = {};
    let args = [response.text, response.status];"#,
        ask("main", ", maxTokens: 32")
    )));

    let report = start(&sandbox, true, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["review", "ok"]);
    assert_eq!(report["selection"]["reason"], "r");
    let exchanges = server.exchanges(1);
    assert_eq!(server.requests_started(), 1, "one request, and no more");
    let seen = &exchanges[0].seen;
    assert_eq!(
        (seen.method.as_str(), seen.path.as_str()),
        ("POST", "/v1/messages")
    );
    assert_eq!(seen.header("x-api-key"), Some(KEY));
    let body = seen.json();
    assert_eq!(body["model"], MODEL);
    assert_eq!(body["max_tokens"], 32);
    assert_eq!(
        body["messages"],
        json!([{ "role": "user", "content": [{ "type": "text", "text": "classify me" }] }])
    );
}

#[test]
fn the_response_map_has_every_field_of_the_spec() {
    let (response, _) = one_call(vec![Reply::messages("an answer")], "");

    assert_eq!(response["status"], "ok");
    assert_eq!(response["endpoint"], "main");
    assert_eq!(response["model"], MODEL);
    assert_eq!(response["text"], "an answer");
    assert_eq!(response["json"], Value::Null);
    assert_eq!(response["finishReason"], "stop");
    assert_eq!(
        response["usage"],
        json!({ "inputTokens": 11, "outputTokens": 7, "totalTokens": 18 })
    );
    assert_eq!(response["reportedModel"], "fixture-reported-model");
    assert_eq!(response["responseId"], "msg_fixture_1");
    assert_eq!(response["requestId"], "req_fixture_1");
    assert!(response["elapsedMs"].is_u64(), "{response}");
    assert_eq!(response["error"], Value::Null);
}

#[test]
fn the_default_token_limit_and_every_option_reach_the_wire() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("a"), Reply::messages("b")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let second = ravel::llm(#{{ endpoint: "main", model: "claude-opus-4-5", maxTokens: 77,
        temperature: 1.0, reasoning: "off", timeoutMs: 20000,
        messages: [#{{ role: "system", content: "Answer in one word." }},
                   #{{ role: "user", content: "one" }},
                   #{{ role: "assistant", content: "two" }},
                   #{{ role: "user", content: "three" }}] }});
    let args = [first.text, second.text];"#,
        ask("main", "")
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["a", "b"]);
    let exchanges = server.exchanges(2);
    assert_eq!(exchanges[0].seen.json()["max_tokens"], 1024);
    let body = exchanges[1].seen.json();
    assert_eq!(body["max_tokens"], 77);
    assert_eq!(body["temperature"], 1.0);
    assert!(body["system"].to_string().contains("Answer in one word."));
    let roles: Vec<&str> = body["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|message| message["role"].as_str().unwrap())
        .collect();
    assert_eq!(roles, ["user", "assistant", "user"]);
}

#[test]
fn load_context_asks_for_a_summary_that_select_receives() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("a short summary")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&policy(&format!(
        r#"fn load_context(request) {{
    let response = {};
    #{{ schemaVersion: 1, summary: "caller", facts: #{{ summary: response.text }} }}
}}
fn select(request, context) {{
    #{{ status: "selected", program: "recording-agent", args: [context.facts.summary],
       provider: "p", model: "m", effort: "e", reason: "r" }}
}}"#,
        ask("main", "")
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["a short summary"]);
    assert_eq!(server.requests_started(), 1);
}

#[test]
fn a_call_at_the_top_level_of_the_entry_runs_too() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("from load")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&policy(&format!(
        r#"const WORD = ravel::llm(#{{ endpoint: "main", model: "{MODEL}",
    messages: [#{{ role: "user", content: "at load" }}] }}).text;
fn select(request, context) {{
    #{{ status: "selected", program: "recording-agent", args: [global::WORD],
       provider: "p", model: "m", effort: "e", reason: "r" }}
}}"#
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["from load"]);
}

#[test]
fn a_rate_limit_is_data_and_the_policy_falls_back_to_a_second_endpoint() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let limited = Server::start(vec![
        Reply::raw(429, "application/json", br#"{"error":"slow down"}"#).header("retry-after", "3"),
    ]);
    let backup = Server::start(vec![Reply::messages("from the backup")]);
    settings(
        &sandbox,
        &[("first", &limited), ("second", &backup)],
        &json!({}),
    );
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let used = first;
    if first.status == "failed" && first.error.retryable {{
        used = {};
    }}
    let args = [first.error.code, first.error.retryable.to_string(),
                first.error.httpStatus.to_string(), first.error.retryAfterMs.to_string(),
                used.text];"#,
        ask("first", ""),
        ask("second", "")
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["rate_limited", "true", "429", "3000", "from the backup"]
    );
    assert_eq!(limited.requests_started(), 1);
    assert_eq!(backup.requests_started(), 1);
}

#[test]
fn a_failed_response_has_the_shape_of_the_spec() {
    let (response, _) = one_call(
        vec![Reply::raw(500, "text/plain", b"down for maintenance")],
        "",
    );

    assert_eq!(response["status"], "failed");
    assert_eq!(response["endpoint"], "main");
    assert_eq!(response["model"], MODEL);
    for unset in [
        "text",
        "json",
        "finishReason",
        "usage",
        "reportedModel",
        "responseId",
    ] {
        assert_eq!(response[unset], Value::Null, "{unset}");
    }
    assert!(response["elapsedMs"].is_u64());
    let error = &response["error"];
    assert_eq!(error["code"], "provider_error");
    assert_eq!(error["retryable"], true);
    assert_eq!(error["httpStatus"], 500);
    assert_eq!(error["retryAfterMs"], Value::Null);
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("down for maintenance"),
        "{error}"
    );
}

#[test]
fn each_failure_of_an_endpoint_is_a_code_that_the_script_can_read() {
    let garbled = || Reply::raw(200, "application/json", b"this is not json");
    let short = || Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Short {
            declared: 4000,
            sent: br#"{"id":"msg"#.to_vec(),
        },
    };
    for (label, reply, code, retryable, status) in [
        (
            "401",
            Reply::raw(401, "application/json", br#"{"error":"bad key"}"#),
            "authentication",
            false,
            json!(401),
        ),
        (
            "403",
            Reply::raw(403, "application/json", br#"{"error":"no"}"#),
            "authentication",
            false,
            json!(403),
        ),
        (
            "400",
            Reply::raw(400, "application/json", br#"{"error":"bad request"}"#),
            "provider_error",
            false,
            json!(400),
        ),
        (
            "an undecodable body",
            garbled(),
            "malformed_response",
            false,
            Value::Null,
        ),
        (
            "a body that stops short",
            short(),
            "malformed_response",
            true,
            Value::Null,
        ),
    ] {
        let (response, _) = one_call(vec![reply], "");
        assert_eq!(response["status"], "failed", "{label}: {response}");
        let error = &response["error"];
        assert_eq!(error["code"], code, "{label}: {response}");
        assert_eq!(error["retryable"], retryable, "{label}: {response}");
        assert_eq!(error["httpStatus"], status, "{label}: {response}");
    }
}

#[test]
fn a_large_failure_body_keeps_its_class_and_its_message_holds_at_most_64_kib_of_it() {
    let big = vec![b'Z'; 1024 * 1024];
    let (response, _) = one_call(vec![Reply::raw(429, "text/html", &big)], "");

    let error = &response["error"];
    assert_eq!(error["code"], "rate_limited", "{response}");
    assert_eq!(error["retryable"], true);
    let kept = error["message"].as_str().unwrap().matches('Z').count();
    assert!(kept <= 64 * 1024, "{kept} bytes of the body kept");
    assert!(
        kept > 63 * 1024,
        "{kept} bytes kept: the message holds the start of the body"
    );
}

#[test]
fn an_oversized_successful_body_fails_the_call_and_the_connection_closes_early() {
    let total = 9 * 1024 * 1024;
    let (response, server) = one_call(
        vec![Reply::json(200, &json!({})).with_body(llm_server::oversized(total, false))],
        "",
    );

    assert_eq!(response["error"]["code"], "output_limit", "{response}");
    assert_eq!(response["error"]["retryable"], false);
    let exchange = &server.exchanges(1)[0];
    assert!(
        exchange.client_closed_early,
        "the server wrote {} of {total} bytes and saw no close",
        exchange.body_bytes_sent
    );
    assert!(exchange.body_bytes_sent < total);
}

#[test]
fn a_declared_length_beyond_the_bound_fails_from_the_head_alone() {
    // The server announces 9 MiB and sends ten bytes. A client that read the body
    // would fail as a short body, so `output_limit` shows that the head decided.
    let (response, _) = one_call(
        vec![Reply {
            status: 200,
            headers: Vec::new(),
            body: Body::Short {
                declared: 9 * 1024 * 1024,
                sent: b"0123456789".to_vec(),
            },
        }],
        "",
    );

    assert_eq!(response["error"]["code"], "output_limit", "{response}");
}

#[test]
fn a_call_that_reaches_its_own_deadline_is_a_retryable_timeout_and_closes_the_connection() {
    let began = Instant::now();
    let (response, server) = one_call(vec![stalled()], ", timeoutMs: 400");

    let error = &response["error"];
    assert_eq!(error["code"], "timeout", "{response}");
    assert_eq!(error["retryable"], true);
    assert!(response["elapsedMs"].as_u64().unwrap() >= 400, "{response}");
    assert!(began.elapsed() < Duration::from_secs(20));
    assert!(server.exchanges(1)[0].client_closed_early);
}

#[test]
fn a_call_that_outlasts_the_whole_deadline_ends_the_evaluation_not_only_the_call() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![stalled()]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    // The call asks for a minute, and the evaluation has one second.
    sandbox.personal_rhai(&selecting(&format!(
        "    let response = {};\n    let args = [response.status];",
        ask("main", ", timeoutMs: 60000")
    )));
    let began = Instant::now();

    let run = start(&sandbox, true, &["--timeout-ms", "1000"]);

    assert_eq!(run.code, Some(124), "{}", run.stderr);
    assert_eq!(recorded.argv(), None);
    assert!(began.elapsed() < Duration::from_secs(20));
}

#[test]
fn a_redirect_is_not_followed_and_the_credential_goes_nowhere_else() {
    let elsewhere = Server::start(vec![Reply::messages("must not be reached")]);
    let target = format!("{}/v1/messages", elsewhere.url());
    let (response, _) = one_call(
        vec![Reply::raw(302, "text/plain", b"moved").header("Location", &target)],
        "",
    );

    assert_eq!(response["error"]["code"], "provider_error", "{response}");
    assert_eq!(response["error"]["httpStatus"], 302);
    assert_eq!(elsewhere.requests_started(), 0, "the credential went on");
}

#[test]
fn an_option_the_model_cannot_honour_fails_before_anything_is_sent() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("never sent")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    // The catalog of Rig lists low, medium and high for this model.
    let call = r#"ravel::llm(#{ endpoint: "main", model: "claude-opus-4-5", reasoning: "xhigh",
        messages: [#{ role: "user", content: "x" }] })"#;

    let response = answered(&sandbox, &recorded, true, call);

    assert_eq!(
        response["error"]["code"], "unsupported_parameter",
        "{response}"
    );
    assert_eq!(response["error"]["retryable"], false);
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn an_unknown_endpoint_an_unavailable_credential_and_an_unoffered_model_send_nothing() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("never sent")]);
    settings(
        &sandbox,
        &[("main", &server)],
        &json!({ "models": ["offered-model"] }),
    );
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let unknown = {};
    let unoffered = {};
    let offered = ravel::llm(#{{ endpoint: "main", model: "offered-model",
        messages: [#{{ role: "user", content: "x" }}] }});
    let args = [unknown.error.code, unknown.error.retryable.to_string(),
                unoffered.error.code, unoffered.error.retryable.to_string(),
                offered.error.code, offered.error.retryable.to_string()];"#,
        ask("missing", ""),
        ask("main", "")
    )));

    // No credential in the environment: the offered model still fails first on it.
    start(&sandbox, false, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "unknown_endpoint",
            "false",
            "model_not_offered",
            "false",
            "endpoint_unavailable",
            "false"
        ]
    );
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn each_malformed_request_ends_the_evaluation_with_the_field_and_the_line() {
    let user = r#"[#{ role: "user", content: "x" }]"#;
    for (request, field) in [
        // The wrong role, and the roles around it.
        (
            r#"#{ endpoint: "main", model: "m", messages: [#{ role: "tool", content: "x" }] }"#.to_owned(),
            "messages[0].role",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: [#{ role: "user", content: "x" }, #{ role: "assistant", content: "y" }] }"#.to_owned(),
            "messages[1].role",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: [#{ content: "x" }] }"#.to_owned(),
            "messages[0].role",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: [#{ role: "user", content: 3 }] }"#.to_owned(),
            "messages[0].content",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: [#{ role: "user", content: "x", name: "n" }] }"#.to_owned(),
            "messages[0].name",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: ["x"] }"#.to_owned(),
            "messages[0]",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: [] }"#.to_owned(),
            "messages",
        ),
        (
            r#"#{ endpoint: "main", model: "m", messages: "x" }"#.to_owned(),
            "messages",
        ),
        // The fields of the request.
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, tools: [] }}"#), "tools"),
        (
            format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, schema: #{{ type: "object" }} }}"#),
            "schema",
        ),
        (format!(r#"#{{ model: "m", messages: {user} }}"#), "endpoint"),
        (format!(r#"#{{ endpoint: 3, model: "m", messages: {user} }}"#), "endpoint"),
        (format!(r#"#{{ endpoint: "main", messages: {user} }}"#), "model"),
        (format!(r#"#{{ endpoint: "main", model: "", messages: {user} }}"#), "model"),
        (r#"#{ endpoint: "main", model: "m" }"#.to_owned(), "messages"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, maxTokens: 0 }}"#), "maxTokens"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, maxTokens: 1.5 }}"#), "maxTokens"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, temperature: 2.5 }}"#), "temperature"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, temperature: -0.1 }}"#), "temperature"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, temperature: "hot" }}"#), "temperature"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, temperature: 1 }}"#), "temperature"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, reasoning: "deep" }}"#), "reasoning"),
        (format!(r#"#{{ endpoint: "main", model: "m", messages: {user}, timeoutMs: 0 }}"#), "timeoutMs"),
        ("\"main\"".to_owned(), "request"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![Reply::messages("never sent")]);
        settings(&sandbox, &[("main", &server)], &json!({}));
        // `try` does not catch the end.
        sandbox.personal_rhai(&selecting(&format!(
            "    let args = [];\n    try {{\n        ravel::llm({request});\n    }} catch (error) {{ args.push(\"caught\"); }}"
        )));

        let error = refused(&start(&sandbox, true, &[]), "host_request_invalid", &recorded);

        let location = error["location"].as_str().unwrap();
        assert!(
            location.starts_with(&format!("ravel::llm({field}) at 6:")),
            "{request}: {error}"
        );
        assert_eq!(error["stage"], "selection", "{request}: {error}");
        assert_eq!(server.requests_started(), 0, "{request}");
    }
}

#[test]
fn a_request_larger_than_a_protocol_message_is_a_malformed_request() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("never sent")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let text = "x";
    for step in 0..21 {{ text += text; }}
    ravel::llm(#{{ endpoint: "main", model: "{MODEL}",
        messages: [#{{ role: "user", content: text }}] }});
    let args = [];"#
    )));

    let error = refused(
        &start(&sandbox, true, &[]),
        "host_request_invalid",
        &recorded,
    );

    assert!(
        error["location"]
            .as_str()
            .unwrap()
            .starts_with("ravel::llm(request) at"),
        "{error}"
    );
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn a_message_with_the_wrong_role_names_the_line_of_the_script() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    ravel::llm(#{
        endpoint: "main", model: "m",
        messages: [#{ role: "tool", content: "x" }],
    });"#,
    ));

    let error = refused(
        &start(&sandbox, true, &[]),
        "host_request_invalid",
        &recorded,
    );

    let message = error["message"].as_str().unwrap();
    assert!(message.contains("messages[0].role"), "{error}");
    assert!(
        error["location"]
            .as_str()
            .unwrap()
            .starts_with("ravel::llm(messages[0].role) at 5:"),
        "{error}"
    );
}

#[test]
fn the_credential_reaches_the_server_only_in_its_header_and_no_probe_finds_it() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let echoing = format!(r#"{{"error":"invalid x-api-key {KEY}"}}"#);
    let server = Server::start(vec![Reply::raw(
        401,
        "application/json",
        echoing.as_bytes(),
    )]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let probe = "none";
    try {{ ravel::env("{KEY_VARIABLE}"); }} catch (error) {{ probe = error.code; }}
    print(probe);
    print(ravel::json_encode(ravel::endpoints()));
    print(ravel::json_encode(request));
    let response = {};
    print(ravel::json_encode(response));
    debug(response.error.message);
    let args = [probe, response.error.code, response.error.message];"#,
        ask("main", "")
    )));

    let run = start(&sandbox, true, &[]);
    let report = run.report();

    let args = recorded.argv().unwrap();
    assert_eq!(args[0], "env_not_granted");
    assert_eq!(args[1], "authentication");
    assert!(
        !args[2].contains(KEY),
        "the error message holds the key: {}",
        args[2]
    );
    assert!(args[2].contains("[redacted]"), "{}", args[2]);
    assert!(!run.stderr.contains(KEY), "{}", run.stderr);
    assert!(!report.to_string().contains(KEY), "{report}");
    let seen = &server.exchanges(1)[0].seen;
    for (name, value) in &seen.headers {
        assert_eq!(value.contains(KEY), name == "x-api-key", "{name}");
    }
    assert!(!String::from_utf8_lossy(&seen.body).contains(KEY));
    assert!(!seen.path.contains(KEY));
}

#[test]
fn an_endpoint_that_echoes_the_credential_in_any_field_of_an_answer_leaks_nothing() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let mut body = llm_server::messages_body(&format!("the key is {KEY}"));
    body["id"] = json!(KEY);
    body["model"] = json!(format!("model-{KEY}"));
    let server = Server::start(vec![
        Reply::json(200, &body).header("request-id", KEY),
        Reply::raw(401, "application/json", br#"{"error":"bad key"}"#).header("request-id", KEY),
    ]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let answered = {};
    let failed = {};
    print(ravel::json_encode(answered));
    print(ravel::json_encode(failed));
    debug(answered);
    let args = [answered.text, answered.responseId, answered.reportedModel, answered.requestId,
                failed.error.code, failed.requestId];"#,
        ask("main", ""),
        ask("main", "")
    )));

    let run = start(&sandbox, true, &[]);
    let report = run.report();

    let args = recorded.argv().unwrap();
    assert_eq!(args[0], "the key is [redacted]");
    assert_eq!(args[1], "[redacted]");
    assert_eq!(args[2], "model-[redacted]");
    assert_eq!(args[3], "[redacted]");
    assert_eq!(args[4], "authentication");
    assert_eq!(args[5], "[redacted]");
    assert!(!run.stderr.contains(KEY), "{}", run.stderr);
    assert!(!report.to_string().contains(KEY), "{report}");
}

/// On Linux the TLS client reads the certificate file that the caller's
/// environment names, when it is built. A FIFO that no process writes holds that
/// read for good, so the whole deadline must still end the evaluation. The
/// other platforms read no such file.
#[cfg(target_os = "linux")]
#[test]
fn a_trust_store_that_blocks_cannot_hold_the_whole_deadline() {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt as _;

    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("never asked")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        "    let response = {};\n    let args = [response.status];",
        ask("main", "")
    )));
    let fifo = sandbox.home.join("blocking-roots.pem");
    let path = CString::new(fifo.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
    let began = Instant::now();

    let mut command = sandbox.command();
    command
        .env(KEY_VARIABLE, KEY)
        .env("SSL_CERT_FILE", &fifo)
        .env_remove("SSL_CERT_DIR")
        .args([
            "start-session",
            "--prompt",
            "classify me",
            "--kind",
            "check",
            "--json",
            "--timeout-ms",
            "1000",
        ]);
    let run = run(&mut command);

    assert_eq!(run.code, Some(124), "{}", run.stderr);
    assert_eq!(recorded.argv(), None);
    assert!(began.elapsed() < Duration::from_secs(20));
}

#[test]
fn a_flag_grant_of_the_credential_refuses_before_any_worker() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting("    let args = [];"));

    let run = start(&sandbox, true, &["--policy-env", KEY_VARIABLE]);

    let document = run.refusal(2);
    assert_eq!(document["error"]["code"], "excluded_grant", "{document}");
    assert_eq!(recorded.argv(), None);
}

#[test]
fn the_notice_lists_each_call_without_a_message_or_an_answer() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![
        Reply::messages("the confidential answer"),
        Reply::raw(429, "text/plain", b"slow"),
    ]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let second = {};
    let missing = {};
    let args = [first.status];"#,
        ask("main", ""),
        ask("main", ""),
        ask("absent", "")
    )));

    let report = start(&sandbox, true, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["ok"]);
    let calls = report["llmCalls"].as_array().unwrap();
    assert_eq!(calls.len(), 3, "{report}");
    let elapsed = |index: usize| calls[index]["elapsedMs"].clone();
    assert_eq!(
        calls[0],
        json!({
            "endpoint": "main", "model": MODEL, "status": "ok", "errorCode": null,
            "usage": { "inputTokens": 11, "outputTokens": 7, "totalTokens": 18 },
            "elapsedMs": elapsed(0),
        })
    );
    assert_eq!(
        calls[1],
        json!({
            "endpoint": "main", "model": MODEL, "status": "failed", "errorCode": "rate_limited",
            "usage": null, "elapsedMs": elapsed(1),
        })
    );
    assert_eq!(
        calls[2],
        json!({
            "endpoint": "absent", "model": MODEL, "status": "failed",
            "errorCode": "unknown_endpoint", "usage": null, "elapsedMs": 0,
        })
    );
    let notice = report.to_string();
    for private in ["the confidential answer", "classify me", KEY] {
        assert!(!notice.contains(private), "the notice holds {private:?}");
    }
}

#[test]
fn a_policy_without_calls_has_an_empty_list_in_its_notice() {
    let sandbox = Sandbox::new();
    sandbox.recording_agent();
    sandbox.personal_rhai(&selecting("    let args = [];"));

    let report = start(&sandbox, false, &[]).report();

    assert_eq!(report["llmCalls"], json!([]));
}
