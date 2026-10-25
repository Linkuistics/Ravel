//! The adapter against the deterministic local server of `tests/support`. Each
//! case plays one scripted reply and checks what Ravel sent and how the adapter
//! classified what came back.

use std::ffi::OsString;
use std::time::Duration;

use serde_json::json;

use super::*;

#[allow(dead_code, reason = "each user of the fixture needs a subset")]
#[path = "../../tests/support/llm_server.rs"]
mod server;

use server::{Body, Reply, Server, messages_body, oversized};

const KEY: &str = "sk-fixture-credential-7f3a";

fn endpoint(base_url: &str) -> Endpoint {
    crate::endpoints::parse_endpoints(&json!({
        "main": { "type": "anthropic", "apiKeyEnv": "FIXTURE_KEY", "baseUrl": base_url }
    }))
    .unwrap()
    .remove("main")
    .unwrap()
}

fn environment(name: &str) -> Option<OsString> {
    (name == "FIXTURE_KEY").then(|| OsString::from(KEY))
}

fn request() -> Request {
    Request {
        model: "claude-sonnet-5-5".to_owned(),
        messages: vec![Message {
            role: Role::User,
            content: "Classify this prompt.".to_owned(),
        }],
        max_tokens: 1024,
        temperature: None,
        reasoning: None,
    }
}

fn run_with(
    endpoint: &Endpoint,
    environment: impl Fn(&str) -> Option<OsString>,
    request: &Request,
    timeout: Duration,
) -> Result<Response, Failure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let adapter = Adapter::new().unwrap();
    runtime.block_on(adapter.call(endpoint, environment, request, timeout))
}

/// Play `script` and make one call.
fn call(script: Vec<Reply>, request: &Request) -> (Result<Response, Failure>, Server) {
    let server = Server::start(script);
    let result = run_with(
        &endpoint(&server.url()),
        environment,
        request,
        Duration::from_secs(30),
    );
    (result, server)
}

fn failed(script: Vec<Reply>) -> (Failure, Server) {
    let (result, server) = call(script, &request());
    (result.expect_err("the call fails"), server)
}

#[test]
fn a_call_sends_the_messages_wire_and_returns_the_answer() {
    let (result, server) = call(vec![Reply::messages("route: impl")], &request());
    let response = result.unwrap();
    assert_eq!(response.text, "route: impl");
    assert_eq!(response.finish, Finish::Stop);
    assert_eq!(
        response.usage,
        Usage {
            input_tokens: Some(11),
            output_tokens: Some(7),
            total_tokens: Some(18)
        }
    );
    assert_eq!(
        response.reported_model.as_deref(),
        Some("fixture-reported-model")
    );
    assert_eq!(response.response_id.as_deref(), Some("msg_fixture_1"));
    assert_eq!(response.request_id.as_deref(), Some("req_fixture_1"));

    let seen = &server.exchanges(1)[0].seen;
    assert_eq!(
        (seen.method.as_str(), seen.path.as_str()),
        ("POST", "/v1/messages")
    );
    assert_eq!(seen.header("x-api-key"), Some(KEY));
    assert!(seen.header("anthropic-version").is_some());
    assert_eq!(
        seen.header("accept-encoding"),
        None,
        "Ravel decodes nothing"
    );
    let body = seen.json();
    assert_eq!(body["model"], "claude-sonnet-5-5");
    assert_eq!(body["max_tokens"], 1024);
    assert_eq!(
        body["messages"],
        json!([{ "role": "user", "content": [{ "type": "text", "text": "Classify this prompt." }] }])
    );
    // Rig adds adaptive thinking by itself for a model whose catalog entry says
    // so. A script turns it off with `reasoning: "off"`.
    for absent in ["tools", "tool_choice", "temperature", "stream"] {
        assert!(
            body.get(absent).is_none_or(|value| value == &json!(false)),
            "{absent} is in {body}"
        );
    }
}

#[test]
fn the_credential_reaches_the_server_only_in_its_header() {
    let (_, server) = call(vec![Reply::messages("x")], &request());
    let seen = &server.exchanges(1)[0].seen;
    for (name, value) in &seen.headers {
        assert_eq!(value.contains(KEY), name == "x-api-key", "{name}");
    }
    assert!(!String::from_utf8_lossy(&seen.body).contains(KEY));
    assert!(!seen.path.contains(KEY));
}

#[test]
fn system_and_assistant_messages_and_the_options_map_onto_the_request() {
    let mut multi = request();
    multi.messages = vec![
        Message {
            role: Role::System,
            content: "Answer in one word.".into(),
        },
        Message {
            role: Role::User,
            content: "first".into(),
        },
        Message {
            role: Role::Assistant,
            content: "second".into(),
        },
        Message {
            role: Role::User,
            content: "third".into(),
        },
    ];
    multi.temperature = Some(0.25);
    multi.max_tokens = 77;
    let (result, server) = call(vec![Reply::messages("ok")], &multi);
    result.unwrap();
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(body["max_tokens"], 77);
    assert_eq!(body["temperature"], 0.25);
    let system = body["system"].to_string();
    assert!(system.contains("Answer in one word."), "{system}");
    let roles: Vec<&str> = body["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|message| message["role"].as_str().unwrap())
        .collect();
    assert_eq!(roles, ["user", "assistant", "user"]);
}

#[test]
fn reasoning_maps_onto_the_portable_option_of_rig() {
    // Rig decides the wire form, which differs by model. The test shows that
    // each word reaches it: the levels differ, and a level appears on the wire.
    let mut bodies = Vec::new();
    for reasoning in [None, Some(Reasoning::Off), Some(Reasoning::High)] {
        let mut asked = request();
        asked.reasoning = reasoning;
        let (result, server) = call(vec![Reply::messages("ok")], &asked);
        result.unwrap();
        bodies.push(server.exchanges(1)[0].seen.json());
    }
    assert_ne!(bodies[1], bodies[2], "off and high send the same request");
    assert!(bodies[2].to_string().contains("high"), "{}", bodies[2]);
}

#[test]
fn an_option_the_model_cannot_honour_fails_before_anything_is_sent() {
    let mut asked = request();
    // The catalog of Rig lists the efforts low, medium and high for this model.
    asked.model = "claude-opus-4-5".to_owned();
    asked.reasoning = Some(Reasoning::XHigh);
    let (result, server) = call(vec![Reply::messages("never sent")], &asked);
    let failure = result.expect_err("an unsupported option fails");
    assert_eq!(failure.code, Code::UnsupportedParameter, "{failure:?}");
    assert!(!failure.retryable);
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn an_unset_or_empty_credential_fails_before_anything_is_sent() {
    for value in [None, Some("")] {
        let server = Server::start(vec![Reply::messages("never sent")]);
        let lookup = |name: &str| {
            (name == "FIXTURE_KEY")
                .then(|| value.map(OsString::from))
                .flatten()
        };
        let failure = run_with(
            &endpoint(&server.url()),
            lookup,
            &request(),
            Duration::from_secs(5),
        )
        .expect_err("no credential");
        assert_eq!(failure.code, Code::EndpointUnavailable);
        assert!(!failure.retryable);
        assert!(failure.message.contains("FIXTURE_KEY"));
        assert_eq!(server.requests_started(), 0);
    }
}

#[test]
fn a_401_and_a_403_are_authentication_failures() {
    for status in [401, 403] {
        let (failure, _) = failed(vec![Reply::json(
            status,
            &json!({ "type": "error", "error": { "type": "authentication_error", "message": "bad key" } }),
        )]);
        assert_eq!(failure.code, Code::Authentication);
        assert_eq!(failure.http_status, Some(status));
        assert!(!failure.retryable);
        assert!(failure.message.contains("bad key"), "{}", failure.message);
    }
}

#[test]
fn a_429_is_rate_limited_and_carries_its_wait() {
    let (failure, _) = failed(vec![
        Reply::raw(
            429,
            "application/json",
            br#"{"type":"error","error":{"type":"rate_limit_error","message":"slow down"}}"#,
        )
        .header("retry-after", "7")
        .header("request-id", "req_429"),
    ]);
    assert_eq!(failure.code, Code::RateLimited);
    assert!(failure.retryable);
    assert_eq!(failure.http_status, Some(429));
    assert_eq!(failure.retry_after_ms, Some(7000));
    assert_eq!(failure.request_id.as_deref(), Some("req_429"));
}

#[test]
fn a_429_without_a_wait_has_none() {
    let (failure, _) = failed(vec![Reply::raw(429, "text/plain", b"later")]);
    assert_eq!(
        (failure.code, failure.retry_after_ms),
        (Code::RateLimited, None)
    );
}

#[test]
fn a_large_failure_body_keeps_the_class_of_its_status_and_is_read_to_a_bound() {
    let big = vec![b'Z'; 1024 * 1024];
    let (failure, server) = failed(vec![Reply::raw(429, "text/html", &big)]);
    assert_eq!(failure.code, Code::RateLimited);
    assert!(failure.retryable);
    let kept = failure.message.matches('Z').count();
    assert!(
        kept <= transport::MAX_FAILURE_BODY_BYTES,
        "{kept} bytes kept"
    );
    assert!(
        kept > transport::MAX_FAILURE_BODY_BYTES - 1024,
        "{kept} bytes kept: the message holds that part of the body"
    );
    drop(server);
}

#[test]
fn a_500_and_an_overloaded_529_are_retryable_provider_errors() {
    for status in [500, 529] {
        let (failure, _) = failed(vec![Reply::raw(status, "text/plain", b"upstream broke")]);
        assert_eq!(failure.code, Code::ProviderError);
        assert!(failure.retryable, "{status}");
        assert_eq!(failure.http_status, Some(status));
        assert!(failure.message.contains("upstream broke"));
    }
}

#[test]
fn a_400_is_a_provider_error_that_is_not_retryable() {
    let (failure, _) = failed(vec![Reply::raw(
        400,
        "application/json",
        br#"{"error":"bad"}"#,
    )]);
    assert_eq!(
        (failure.code, failure.retryable),
        (Code::ProviderError, false)
    );
}

#[test]
fn an_error_envelope_with_a_success_status_is_a_provider_error() {
    let (failure, _) = failed(vec![Reply::json(
        200,
        &json!({ "type": "error", "error": { "type": "overloaded_error", "message": "busy" } }),
    )]);
    assert_eq!(failure.code, Code::ProviderError, "{failure:?}");
    assert_eq!(failure.http_status, Some(200));
}

#[test]
fn a_body_that_does_not_decode_is_a_malformed_response() {
    for body in [&b"this is not JSON"[..], &b"{\"unexpected\":true}"[..]] {
        let (failure, _) = failed(vec![Reply::raw(200, "application/json", body)]);
        assert_eq!(failure.code, Code::MalformedResponse, "{failure:?}");
        assert!(!failure.retryable, "{body:?} {failure:?}");
    }
}

#[test]
fn an_answer_without_text_is_a_malformed_response() {
    let mut empty = messages_body("x");
    empty["content"] = json!([]);
    let (failure, _) = failed(vec![Reply::json(200, &empty)]);
    assert_eq!(failure.code, Code::MalformedResponse, "{failure:?}");
    assert!(!failure.retryable);
}

#[test]
fn a_body_that_stops_short_is_the_retryable_malformed_response() {
    let (failure, _) = failed(vec![Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Short {
            declared: 5000,
            sent: br#"{"id":"msg"#.to_vec(),
        },
    }]);
    assert_eq!(failure.code, Code::MalformedResponse, "{failure:?}");
    assert!(failure.retryable);
}

#[test]
fn a_length_finish_is_reported_as_length() {
    let mut cut = messages_body("partial");
    cut["stop_reason"] = json!("max_tokens");
    let (result, _) = call(vec![Reply::json(200, &cut)], &request());
    assert_eq!(result.unwrap().finish, Finish::Length);
}

#[test]
fn a_refusal_is_content_filtered_or_a_non_text_ending() {
    let mut refused = messages_body("I will not.");
    refused["stop_reason"] = json!("refusal");
    let (result, _) = call(vec![Reply::json(200, &refused)], &request());
    match result {
        Err(failure) => assert_eq!(failure.code, Code::ContentFiltered, "{failure:?}"),
        Ok(response) => panic!("a refusal returned {response:?}"),
    }
}

#[test]
fn a_successful_body_beyond_the_bound_without_a_declared_length_stops_at_the_bound() {
    let total = 9 * 1024 * 1024;
    let (failure, server) = failed(vec![
        Reply::json(200, &json!({})).with_body(oversized(total, false)),
    ]);
    assert_eq!(failure.code, Code::OutputLimit, "{failure:?}");
    assert!(!failure.retryable);
    let exchange = &server.exchanges(1)[0];
    assert!(
        exchange.client_closed_early,
        "the server wrote {} bytes of {total} and saw no close",
        exchange.body_bytes_sent
    );
    assert!(exchange.body_bytes_sent < total);
}

#[test]
fn a_declared_length_beyond_the_bound_fails_before_the_body_is_read() {
    // The server announces 9 MiB and sends ten bytes. A client that read the
    // body would fail as a short body, so `output_limit` shows that the head
    // alone decided.
    let (failure, _) = failed(vec![Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Short {
            declared: 9 * 1024 * 1024,
            sent: b"0123456789".to_vec(),
        },
    }]);
    assert_eq!(failure.code, Code::OutputLimit, "{failure:?}");
}

#[test]
fn a_body_of_exactly_the_bound_is_read() {
    // A valid body of exactly 8 MiB: the bound is inclusive.
    let (result, _) = call(
        vec![Reply::json(200, &json!({})).with_body(oversized(8 * 1024 * 1024, true))],
        &request(),
    );
    assert_eq!(result.unwrap().text, "ok");
}

#[test]
fn a_redirect_is_not_followed() {
    let elsewhere = Server::start(vec![Reply::messages("must not be reached")]);
    let target = format!("{}/v1/messages", elsewhere.url());
    let (failure, _) = failed(vec![
        Reply::raw(302, "text/plain", b"moved").header("Location", &target),
    ]);
    assert_eq!(failure.code, Code::ProviderError, "{failure:?}");
    assert_eq!(failure.http_status, Some(302));
    assert!(!failure.retryable);
    assert_eq!(elsewhere.requests_started(), 0, "the credential went on");
}

#[test]
fn an_ambient_proxy_variable_is_ignored() {
    let proxy = Server::start(vec![Reply::messages("from the proxy")]);
    // SAFETY: nextest runs each test in its own process, and no other thread of
    // this process reads the environment before the client is built.
    unsafe {
        std::env::set_var("HTTP_PROXY", proxy.url());
        std::env::set_var("http_proxy", proxy.url());
        std::env::set_var("ALL_PROXY", proxy.url());
    }
    let (result, server) = call(vec![Reply::messages("direct")], &request());
    assert_eq!(result.unwrap().text, "direct");
    assert_eq!(server.requests_started(), 1);
    assert_eq!(proxy.requests_started(), 0);
}

#[test]
fn a_call_that_reaches_its_deadline_is_a_retryable_timeout() {
    let server = Server::start(vec![Reply {
        status: 200,
        headers: Vec::new(),
        body: Body::Stall {
            hold: Duration::from_secs(10),
        },
    }]);
    let began = std::time::Instant::now();
    let failure = run_with(
        &endpoint(&server.url()),
        environment,
        &request(),
        Duration::from_millis(300),
    )
    .expect_err("a stalled call times out");
    assert_eq!(failure.code, Code::Timeout);
    assert!(failure.retryable);
    assert!(began.elapsed() < Duration::from_secs(5));
    assert!(failure.elapsed >= Duration::from_millis(300));
    assert!(server.exchanges(1)[0].client_closed_early);
}

#[test]
fn a_refused_connection_is_a_retryable_network_failure() {
    let closed = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        format!("http://{}", listener.local_addr().unwrap())
    };
    let failure = run_with(
        &endpoint(&closed),
        environment,
        &request(),
        Duration::from_secs(5),
    )
    .expect_err("nothing listens");
    assert_eq!(failure.code, Code::Network, "{failure:?}");
    assert!(failure.retryable);
    assert!(!failure.message.contains(KEY));
}

#[test]
fn a_credential_that_the_endpoint_echoes_is_redacted() {
    let echo = format!(r#"{{"error":"invalid x-api-key {KEY}"}}"#);
    let (failure, _) = failed(vec![Reply::raw(401, "application/json", echo.as_bytes())]);
    assert_eq!(failure.code, Code::Authentication);
    assert!(!failure.message.contains(KEY), "{}", failure.message);
    assert!(failure.message.contains("[redacted]"));
    assert!(!format!("{failure:?}").contains(KEY));
}

#[test]
fn a_credential_in_the_request_id_of_a_failure_is_redacted() {
    let (failure, _) = failed(vec![
        Reply::raw(401, "application/json", br#"{"error":"bad key"}"#).header("request-id", KEY),
    ]);
    assert_eq!(failure.code, Code::Authentication);
    assert_eq!(failure.request_id.as_deref(), Some("[redacted]"));
    assert!(!format!("{failure:?}").contains(KEY));
}

#[test]
fn a_credential_anywhere_in_a_successful_answer_is_redacted() {
    let mut body = messages_body(&format!("the key is {KEY}"));
    body["id"] = json!(KEY);
    body["model"] = json!(format!("model-{KEY}"));
    let (result, _) = call(
        vec![Reply::json(200, &body).header("request-id", KEY)],
        &request(),
    );
    let response = result.unwrap();
    assert_eq!(response.text, "the key is [redacted]");
    assert_eq!(response.response_id.as_deref(), Some("[redacted]"));
    assert_eq!(response.reported_model.as_deref(), Some("model-[redacted]"));
    assert_eq!(response.request_id.as_deref(), Some("[redacted]"));
    assert!(!format!("{response:?}").contains(KEY));
}

#[test]
fn the_other_api_types_are_not_available_yet() {
    let other = crate::endpoints::parse_endpoints(&json!({
        "gpt": { "type": "openai", "apiKeyEnv": "FIXTURE_KEY", "baseUrl": "http://127.0.0.1:1" }
    }))
    .unwrap()
    .remove("gpt")
    .unwrap();
    let failure = run_with(&other, environment, &request(), Duration::from_secs(5))
        .expect_err("not built yet");
    assert_eq!(failure.code, Code::ProviderError);
}

#[test]
fn only_https_or_loopback_http_is_a_base_url() {
    // The settings check the grammar; the adapter sends to what they accepted.
    let refused = crate::endpoints::parse_endpoints(&json!({
        "a": { "type": "anthropic", "apiKeyEnv": "K", "baseUrl": "http://example.com" }
    }));
    assert!(refused.is_err());
}
