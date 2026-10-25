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

use std::os::unix::process::ExitStatusExt as _;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::llm_server::{self, Body, Reply, Server};
use support::rhai::{Recorded, gone, policy, worker_of};
use support::{Run, Sandbox, run};

const SETTINGS: &str = ".config/ravel/settings.json";
const KEY_VARIABLE: &str = "SEAM_LLM_KEY";
const KEY: &str = "sk-seam-credential-91c4e7";
const MODEL: &str = "claude-sonnet-5-5";

/// Settings that name each of `endpoints` as an `anthropic` endpoint at the base
/// URL of its server. `extra` adds fields to every entry.
fn settings(sandbox: &Sandbox, endpoints: &[(&str, &Server)], extra: &Value) {
    settings_with(sandbox, endpoints, extra, &Value::Null);
}

/// [`settings`], and the owner's `budgets` unless it is null.
fn settings_with(sandbox: &Sandbox, endpoints: &[(&str, &Server)], extra: &Value, budgets: &Value) {
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
    let mut document = json!({ "endpoints": entries });
    if !budgets.is_null() {
        document["budgets"] = budgets.clone();
    }
    support::write(&sandbox.home.join(SETTINGS), &document.to_string());
}

/// `start-session` with the credential in the caller's environment, unless
/// `key` is false.
fn start(sandbox: &Sandbox, key: bool, args: &[&str]) -> Run {
    run(&mut start_command(sandbox, key, args))
}

fn start_command(sandbox: &Sandbox, key: bool, args: &[&str]) -> Command {
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
fn spawn(sandbox: &Sandbox, args: &[&str]) -> Child {
    start_command(sandbox, true, args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

/// Wait until `server` has read a request, so that a call is in flight.
fn await_request(server: &Server) {
    let deadline = Instant::now() + Duration::from_secs(20);
    while server.requests_started() == 0 {
        assert!(Instant::now() < deadline, "no request reached the server");
        std::thread::sleep(Duration::from_millis(10));
    }
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

/// An endpoint that reads the request and never answers.
fn silent() -> Reply {
    Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Silent {
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
fn an_answer_text_beyond_one_mib_in_a_body_below_the_bound_fails_the_call() {
    // The body is a little over 1 MiB, far below the 8 MiB of the body bound, so
    // only the bound on the text can fail the call.
    // The script returns the length, because the text itself would pass the
    // bound of 1 MiB on a message from the worker.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages(&"a".repeat(1024 * 1024))]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        "    let response = {};\n    let args = [response.status, response.text.len().to_string()];",
        ask("main", "")
    )));
    start(&sandbox, true, &[]).report();
    assert_eq!(
        recorded.argv().unwrap(),
        ["ok", "1048576"],
        "exactly 1 MiB passes"
    );

    let beyond = "a".repeat(1024 * 1024 + 1);
    let (response, _) = one_call(vec![Reply::messages(&beyond)], "");

    assert_eq!(response["status"], "failed", "{response}");
    let error = &response["error"];
    assert_eq!(error["code"], "output_limit", "{response}");
    assert_eq!(error["retryable"], false);
    assert!(
        error["message"].as_str().unwrap().len() < 200,
        "the message holds no text: {response}"
    );
    // The failure keeps the request id of the answer.
    assert_eq!(response["requestId"], "req_fixture_1", "{response}");
}

#[test]
fn a_body_of_millions_of_small_values_fails_the_call_before_it_is_decoded() {
    // About 8 MB, below the bound on bytes, with four million values.
    let values = format!("[{}0]", "0,".repeat(4_000_000));
    let body = format!(
        r#"{{"id":"msg_big","type":"message","role":"assistant","model":"m","content":[{{"type":"text","text":"ok"}}],"stop_reason":"end_turn","usage":{{"input_tokens":1,"output_tokens":1}},"metadata":{values}}}"#
    );
    assert!(body.len() < 8 * 1024 * 1024);
    let (response, _) = one_call(
        vec![Reply::raw(200, "application/json", body.as_bytes())],
        "",
    );

    assert_eq!(response["error"]["code"], "output_limit", "{response}");
    assert_eq!(response["error"]["retryable"], false);
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

/// A policy whose one call asks for a minute, which is longer than any
/// evaluation of these tests, so the whole deadline decides.
fn long_call(sandbox: &Sandbox) {
    sandbox.personal_rhai(&selecting(&format!(
        "    let response = {};\n    let args = [response.status];",
        ask("main", ", timeoutMs: 60000")
    )));
}

/// Hold the teardown of `front` open, so that the test can look at the front
/// while it still runs. Returns the pid of the worker.
///
/// On Linux the worker is stopped. The teardown sends it TERM, which a stopped
/// process takes only when it continues, so the front waits out its grace of one
/// second and then kills it. macOS ends a stopped process on TERM, so there the
/// teardown takes a few milliseconds and the checks that need the hold do not
/// run (`assert_closed_before_exit`).
fn hold_teardown(front: &Child) -> i32 {
    let worker = worker_of(front.id());
    #[cfg(target_os = "linux")]
    // SAFETY: the pid is the live worker of this test's front.
    assert_eq!(unsafe { libc::kill(worker, libc::SIGSTOP) }, 0);
    worker
}

/// The fake endpoint saw its connection close. On Linux it saw the close while
/// the front, held in its teardown, had not yet exited. A close at the exit of
/// the process would prove nothing, because the exit closes every descriptor.
fn assert_closed_before_exit(server: &Server, front: &mut Child, what: &str) {
    assert!(
        server.exchanges(1)[0].client_closed_early,
        "{what}: the fake endpoint saw no close"
    );
    #[cfg(target_os = "linux")]
    assert!(
        front.try_wait().unwrap().is_none(),
        "{what}: the front had exited when the close arrived"
    );
    #[cfg(not(target_os = "linux"))]
    let _ = front;
}

fn assert_no_residue(sandbox: &Sandbox, recorded: &Recorded, worker: i32, what: &str) {
    assert!(gone(worker), "{what}: the worker survived");
    assert_eq!(recorded.argv(), None, "{what}: a harness started");
    assert_eq!(
        std::fs::read_dir(&sandbox.tmp).unwrap().count(),
        0,
        "{what}: a private directory remains"
    );
}

#[test]
fn a_stalled_endpoint_ends_the_evaluation_at_the_deadline_and_the_connection_closes() {
    // The scenario "Stalled endpoint". One endpoint stalls after its head, so
    // the body never arrives. The other never answers at all.
    for (what, reply) in [("a stalled body", stalled()), ("no answer", silent())] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![reply]);
        settings(&sandbox, &[("main", &server)], &json!({}));
        long_call(&sandbox);
        // The call asks for a minute, and the evaluation has one second.
        let mut front = spawn(&sandbox, &["--timeout-ms", "1000"]);
        await_request(&server);
        let worker = hold_teardown(&front);
        let began = Instant::now();

        assert_closed_before_exit(&server, &mut front, what);
        let output = front.wait_with_output().unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(124), "{what}: {stderr}");
        assert!(stderr.contains("selection_timeout"), "{what}: {stderr}");
        assert!(began.elapsed() < Duration::from_secs(20), "{what}");
        assert_no_residue(&sandbox, &recorded, worker, what);
    }
}

#[test]
fn a_signal_during_a_call_in_flight_closes_its_connection_and_launches_nothing() {
    for signal in [libc::SIGTERM, libc::SIGINT, libc::SIGHUP] {
        for (what, reply) in [("a stalled body", stalled()), ("no answer", silent())] {
            let what = &format!("signal {signal}, {what}");
            let sandbox = Sandbox::new();
            let recorded = sandbox.recording_agent();
            let server = Server::start(vec![reply]);
            settings(&sandbox, &[("main", &server)], &json!({}));
            long_call(&sandbox);
            // The evaluation has its default 30 s, so only the signal can stop it.
            let mut front = spawn(&sandbox, &[]);
            await_request(&server);
            let worker = hold_teardown(&front);
            let began = Instant::now();

            // SAFETY: the pid is this test's live front.
            assert_eq!(unsafe { libc::kill(front.id() as i32, signal) }, 0);
            assert_closed_before_exit(&server, &mut front, what);
            let output = front.wait_with_output().unwrap();

            // The front raises the signal again once it has cleaned up, so the
            // status is 128 plus the number to a shell: 143 for TERM.
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert_eq!(output.status.signal(), Some(signal), "{what}: {stderr}");
            assert!(output.stdout.is_empty(), "{what}");
            assert!(stderr.contains("selection_cancelled"), "{what}: {stderr}");
            assert!(began.elapsed() < Duration::from_secs(10), "{what}");
            assert_no_residue(&sandbox, &recorded, worker, what);
        }
    }
}

/// Linux only: the hold on the teardown needs a stopped process to keep its TERM
/// pending (see `hold_teardown`). The same rule holds on macOS, but no seam test
/// can put a signal into the few milliseconds of its teardown without a race.
#[cfg(target_os = "linux")]
#[test]
fn a_signal_that_arrives_after_the_deadline_wins_and_the_report_keeps_the_timeout() {
    // The deadline passes during the call, and TERM arrives while the front
    // tears down, which the stopped worker holds open. The signal came before
    // the end of the teardown, so it wins over the deadline (*Cancellation and
    // deadlines*), and the report keeps the deadline as the original failure.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![stalled()]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    long_call(&sandbox);
    let mut front = spawn(&sandbox, &["--timeout-ms", "1000"]);
    await_request(&server);
    let worker = hold_teardown(&front);

    // The deadline closes the connection, and the teardown begins.
    assert_closed_before_exit(&server, &mut front, "after the deadline");
    // SAFETY: the pid is this test's live front.
    assert_eq!(unsafe { libc::kill(front.id() as i32, libc::SIGTERM) }, 0);
    let output = front.wait_with_output().unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.signal(), Some(libc::SIGTERM), "{stderr}");
    assert!(stderr.contains("selection_cancelled"), "{stderr}");
    assert!(
        stderr.contains("selection_timeout"),
        "the report keeps the deadline as the original failure: {stderr}"
    );
    assert_no_residue(&sandbox, &recorded, worker, "after the deadline");
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

// The evaluation budgets for LLM calls and output tokens (`docs/specs/
// policy-evaluation.md`, *Evaluation budgets and their composition*, and the
// scenario "Caught budget breach").

/// A policy that makes `calls` in order, each as `let rN = ...;`, and then hands
/// the harness `tail`, an array expression, as its arguments.
fn calling(calls: &[String], tail: &str) -> String {
    let lets: String = calls
        .iter()
        .enumerate()
        .map(|(index, call)| format!("    let r{index} = {call};\n"))
        .collect();
    selecting(&format!("{lets}    let args = {tail};"))
}

/// The refusal for a budget, with its bound.
fn exhausted(run: &Run, recorded: &Recorded) -> Value {
    let error = refused(run, "llm_budget_exhausted", recorded);
    assert_eq!(error["stage"], "evaluation", "{error}");
    error
}

#[test]
fn a_ninth_call_in_try_still_ends_the_evaluation_as_llm_budget_exhausted() {
    // The scenario "Caught budget breach". Eight calls use the default budget.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages("a"); 8]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    for turn in 0..8 {{ {}; }}
    let args = ["escaped"];
    try {{ {}; args = ["caught"]; }} catch (error) {{ args = ["swallowed"]; }}"#,
        ask("main", ", maxTokens: 8"),
        ask("main", ", maxTokens: 8"),
    )));

    let run = start(&sandbox, true, &[]);

    let error = exhausted(&run, &recorded);
    assert!(
        error["message"].as_str().unwrap().contains("ravel::llm"),
        "{error}"
    );
    assert_eq!(error["bound"]["name"], "llmCalls", "{error}");
    assert_eq!(error["bound"]["calls"], 8, "{error}");
    assert_eq!(error["bound"]["from"], "default", "{error}");
    assert_eq!(server.requests_started(), 8, "the ninth call was sent");
}

#[test]
fn the_owners_call_budget_replaces_the_default_and_a_zero_budget_forbids_every_call() {
    for (budget, sent) in [(2, 2), (0, 0)] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![Reply::messages("a"); 3]);
        settings_with(
            &sandbox,
            &[("main", &server)],
            &json!({}),
            &json!({ "llmCalls": budget }),
        );
        let call = ask("main", ", maxTokens: 8");
        sandbox.personal_rhai(&calling(&[call.clone(), call.clone(), call], "[]"));

        let error = exhausted(&start(&sandbox, true, &[]), &recorded);

        assert_eq!(error["bound"]["calls"], budget, "{error}");
        assert_eq!(error["bound"]["from"], "settings.json", "{error}");
        assert_eq!(server.requests_started(), sent, "budget {budget}");
    }
}

#[test]
fn a_call_uses_one_call_and_the_output_tokens_the_endpoint_reports() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![
        Reply::messages_reporting("a", 300),
        Reply::messages_reporting("b", 42),
    ]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let before = ravel::remaining();
    {};
    let middle = ravel::remaining();
    {};
    let after = ravel::remaining();
    let args = [ravel::json_encode(#{{ before: before, middle: middle, after: after,
                                      limits: request.limits }})];"#,
        ask("main", ", maxTokens: 500"),
        ask("main", ", maxTokens: 500"),
    )));

    start(&sandbox, true, &[]).report();

    let seen: Value = serde_json::from_str(&recorded.argv().unwrap()[0]).unwrap();
    let live = |at: &str| {
        (
            seen[at]["llmCalls"].clone(),
            seen[at]["llmOutputTokens"].clone(),
        )
    };
    assert_eq!(live("before"), (json!(8), json!(32_768)));
    assert_eq!(live("middle"), (json!(7), json!(32_768 - 300)));
    assert_eq!(live("after"), (json!(6), json!(32_768 - 342)));
    // The map of limits holds the budgets of the evaluation, not what is left.
    assert_eq!(seen["limits"]["llmCalls"], 8);
    assert_eq!(seen["limits"]["llmOutputTokens"], 32_768);
}

#[test]
fn an_endpoint_that_reports_no_usage_costs_the_effective_max_tokens() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![
        Reply::messages_without_usage("a"),
        Reply::messages_without_usage("b"),
    ]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let middle = ravel::remaining().llmOutputTokens;
    let second = {};
    let after = ravel::remaining().llmOutputTokens;
    let args = [first.status, ravel::json_encode(first.usage), middle.to_string(),
                after.to_string()];"#,
        ask("main", ", maxTokens: 100"),
        // No maxTokens: the default of 1,024 applies.
        ask("main", ""),
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "ok",
            r#"{"inputTokens":null,"outputTokens":null,"totalTokens":null}"#,
            &(32_768 - 100).to_string(),
            &(32_768 - 100 - 1024).to_string(),
        ]
    );
    assert_eq!(server.requests_started(), 2);
}

#[test]
fn the_tokens_that_remain_lower_max_tokens_and_a_spent_budget_ends_the_evaluation() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![
        Reply::messages_without_usage("a"),
        Reply::messages_reporting("b", 15),
        Reply::messages_without_usage("c"),
        Reply::messages("never sent"),
    ]);
    settings_with(
        &sandbox,
        &[("main", &server)],
        &json!({}),
        &json!({ "llmOutputTokens": 100 }),
    );
    sandbox.personal_rhai(&calling(
        &[
            ask("main", ", maxTokens: 60"),
            // The default of 1,024 is lowered to the 40 that remain, and the
            // endpoint reports 15 of them.
            ask("main", ""),
            // 25 remain, and the call asks for 5,000.
            ask("main", ", maxTokens: 5000"),
            ask("main", ""),
        ],
        "[]",
    ));

    let error = exhausted(&start(&sandbox, true, &[]), &recorded);

    assert_eq!(error["bound"]["name"], "llmOutputTokens", "{error}");
    assert_eq!(error["bound"]["tokens"], 100, "{error}");
    assert_eq!(error["bound"]["from"], "settings.json", "{error}");
    let sent: Vec<Value> = server
        .exchanges(3)
        .iter()
        .map(|exchange| exchange.seen.json()["max_tokens"].clone())
        .collect();
    assert_eq!(sent, [json!(60), json!(40), json!(25)]);
    assert_eq!(server.requests_started(), 3, "a fourth call was sent");
}

#[test]
fn a_call_that_fails_before_it_is_sent_uses_no_budget() {
    // The budget holds one call and one token. Every call below fails before
    // Ravel sends it, so the budget must be whole at the end, and the fifth call
    // is the one that spends it.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Reply::messages_reporting("fine", 1)]);
    settings_with(
        &sandbox,
        &[("main", &server)],
        &json!({ "models": ["offered-model", "claude-opus-4-5"] }),
        &json!({ "llmCalls": 1, "llmOutputTokens": 1 }),
    );
    // The catalog of Rig lists low, medium and high for this model.
    let unsupported = r#"ravel::llm(#{ endpoint: "main", model: "claude-opus-4-5",
        reasoning: "xhigh", maxTokens: 1, messages: [#{ role: "user", content: "x" }] })"#;
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let codes = [];
    codes.push({}.error.code);
    codes.push({}.error.code);
    codes.push({unsupported}.error.code);
    let left = ravel::remaining();
    let sent = ravel::llm(#{{ endpoint: "main", model: "offered-model", maxTokens: 1,
        messages: [#{{ role: "user", content: "x" }}] }});
    codes.push(sent.status);
    codes.push(ravel::remaining().llmCalls.to_string());
    let args = [ravel::json_encode(codes), left.llmCalls.to_string(),
                left.llmOutputTokens.to_string()];"#,
        ask("missing", ", maxTokens: 1"),
        ask("main", ", maxTokens: 1"),
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            r#"["unknown_endpoint","model_not_offered","unsupported_parameter","ok","0"]"#,
            "1",
            "1"
        ]
    );
    assert_eq!(server.requests_started(), 1);
}

#[test]
fn a_credential_that_is_unset_uses_no_budget() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![]);
    settings_with(
        &sandbox,
        &[("main", &server)],
        &json!({}),
        &json!({ "llmCalls": 1 }),
    );
    let call = ask("main", ", maxTokens: 8");
    sandbox.personal_rhai(&calling(
        &[call.clone(), call.clone(), call],
        "[r0.error.code, r1.error.code, r2.error.code, ravel::remaining().llmCalls.to_string()]",
    ));

    start(&sandbox, false, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "endpoint_unavailable",
            "endpoint_unavailable",
            "endpoint_unavailable",
            "1"
        ]
    );
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn a_call_that_was_sent_and_failed_uses_a_call_and_its_effective_max_tokens() {
    // A 429 and an answer text past 1 MiB are both sent. The second one reached
    // Rig with its usage, but the failure carries none, so it costs max_tokens.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![
        Reply::raw(429, "text/plain", b"slow"),
        Reply::messages_reporting(&"a".repeat(1024 * 1024 + 1), 3),
    ]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let after_first = ravel::remaining();
    let second = {};
    let after_second = ravel::remaining();
    let args = [first.error.code, second.error.code,
                after_first.llmCalls.to_string(), after_first.llmOutputTokens.to_string(),
                after_second.llmCalls.to_string(), after_second.llmOutputTokens.to_string()];"#,
        ask("main", ", maxTokens: 200"),
        ask("main", ", maxTokens: 50"),
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "rate_limited",
            "output_limit",
            "7",
            &(32_768 - 200).to_string(),
            "6",
            &(32_768 - 250).to_string(),
        ]
    );
}

#[test]
fn the_notice_lists_the_effective_budgets_among_its_bounds() {
    let sandbox = Sandbox::new();
    sandbox.recording_agent();
    let server = Server::start(vec![]);
    sandbox.personal_rhai(&selecting("    let args = [];"));
    settings_with(
        &sandbox,
        &[("main", &server)],
        &json!({}),
        &json!({ "llmCalls": 3, "harnessRuns": 2 }),
    );

    let bounds = start(&sandbox, false, &[]).report()["bounds"].clone();

    assert_eq!(
        bounds["llmCalls"],
        json!({ "calls": 3, "from": "settings.json" })
    );
    assert_eq!(
        bounds["llmOutputTokens"],
        json!({ "tokens": 32_768, "from": "default" })
    );
    assert_eq!(
        bounds["harnessRuns"],
        json!({ "runs": 2, "from": "settings.json" })
    );
    assert!(bounds["selection"].is_object(), "{bounds}");
}

#[test]
fn a_budget_breach_reports_the_line_of_the_call() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![]);
    settings_with(
        &sandbox,
        &[("main", &server)],
        &json!({}),
        &json!({ "llmCalls": 0 }),
    );
    sandbox.personal_rhai(&selecting(&format!(
        "    let r = {};\n    let args = [];",
        ask("main", "")
    )));

    let error = exhausted(&start(&sandbox, true, &[]), &recorded);

    assert!(
        error["location"]
            .as_str()
            .is_some_and(|at| at.contains(':')),
        "{error}"
    );
    assert_eq!(server.requests_started(), 0);
}
