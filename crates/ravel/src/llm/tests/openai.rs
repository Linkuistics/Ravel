//! The adapter on the two OpenAI wires: `openai` on Responses and
//! `openai-compatible` on Chat Completions. Each case plays one scripted reply
//! and checks what Ravel sent and how the adapter classified what came back.

use super::*;

/// Play `script` on `wire` and make one call.
fn call_on(
    wire: Wire,
    script: Vec<Reply>,
    request: &Request,
) -> (Result<Response, Failure>, Server) {
    let server = Server::start(script);
    let result = run_with(
        &endpoint_on(wire, &server.url()),
        environment,
        request,
        Duration::from_secs(30),
    );
    (result, server)
}

fn failed_on(wire: Wire, script: Vec<Reply>) -> (Failure, Server) {
    let (result, server) = call_on(wire, script, &request_on(wire));
    (result.expect_err("the call fails"), server)
}

fn answered_on(wire: Wire, script: Vec<Reply>) -> Response {
    call_on(wire, script, &request_on(wire))
        .0
        .expect("the call succeeds")
}

#[test]
fn a_call_sends_its_wire_and_returns_the_answer() {
    for wire in Wire::OPENAI {
        let (result, server) = call_on(wire, vec![wire.answer("route: impl")], &request_on(wire));
        let response = result.unwrap();
        assert_eq!(response.text, "route: impl", "{wire:?}");
        assert_eq!(response.finish, Finish::Stop, "{wire:?}");
        assert_eq!(
            response.usage,
            Usage {
                input_tokens: Some(11),
                output_tokens: Some(7),
                total_tokens: Some(18)
            },
            "{wire:?}"
        );
        assert_eq!(
            response.reported_model.as_deref(),
            Some("fixture-reported-model")
        );
        assert!(response.response_id.is_some(), "{wire:?}");
        assert_eq!(response.request_id.as_deref(), Some("req_fixture_1"));

        let seen = &server.exchanges(1)[0].seen;
        assert_eq!(
            (seen.method.as_str(), seen.path.as_str()),
            ("POST", wire.path()),
            "{wire:?}"
        );
        let (name, value) = wire.credential_header(KEY);
        assert_eq!(seen.header(name), Some(value.as_str()), "{wire:?}");
        assert_eq!(
            seen.header("accept-encoding"),
            None,
            "Ravel decodes nothing"
        );
        let body = seen.json();
        assert_eq!(body["model"], wire.model(), "{wire:?}");
        for absent in ["tools", "tool_choice", "temperature", "stream"] {
            assert!(
                body.get(absent).is_none_or(|value| value == &json!(false)),
                "{wire:?}: {absent} is in {body}"
            );
        }
    }
}

#[test]
fn the_text_and_the_token_limit_reach_each_wire_in_its_own_field() {
    let (_, server) = call_on(
        Wire::Responses,
        vec![Wire::Responses.answer("x")],
        &request_on(Wire::Responses),
    );
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(body["max_output_tokens"], 1024, "{body}");
    assert!(
        body["input"].to_string().contains("Classify this prompt."),
        "{body}"
    );

    let (_, server) = call_on(
        Wire::Chat,
        vec![Wire::Chat.answer("x")],
        &request_on(Wire::Chat),
    );
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(body["max_tokens"], 1024, "{body}");
    assert_eq!(
        body["messages"],
        json!([{ "role": "user", "content": "Classify this prompt." }]),
    );
}

#[test]
fn system_and_assistant_messages_and_the_options_map_onto_each_wire() {
    for wire in Wire::OPENAI {
        let mut multi = request_on(wire);
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
        let (result, server) = call_on(wire, vec![wire.answer("ok")], &multi);
        result.unwrap();
        let body = server.exchanges(1)[0].seen.json();
        assert_eq!(body["temperature"], 0.25, "{wire:?}: {body}");
        let limit = if wire == Wire::Responses {
            "max_output_tokens"
        } else {
            "max_tokens"
        };
        assert_eq!(body[limit], 77, "{wire:?}: {body}");
        let sent = body.to_string();
        for text in ["Answer in one word.", "first", "second", "third"] {
            assert!(sent.contains(text), "{wire:?}: {text} is not in {sent}");
        }
        assert!(
            sent.find("first") < sent.find("second") && sent.find("second") < sent.find("third"),
            "{wire:?}: the messages are out of order in {sent}"
        );
    }
}

#[test]
fn reasoning_reaches_the_openai_wire_and_the_compatible_wire_refuses_it() {
    let mut asked = request_on(Wire::Responses);
    asked.reasoning = Some(Reasoning::High);
    let (result, server) = call_on(Wire::Responses, vec![Wire::Responses.answer("ok")], &asked);
    result.unwrap();
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(body["reasoning"]["effort"], "high", "{body}");

    // Rig registers no mapping of `reasoning` for a Chat Completions server that
    // Ravel cannot name, and it refuses the option before it sends anything.
    for reasoning in [Reasoning::Off, Reasoning::High] {
        let mut asked = request_on(Wire::Chat);
        asked.reasoning = Some(reasoning);
        let (result, server) = call_on(Wire::Chat, vec![Wire::Chat.answer("never")], &asked);
        let failure = result.expect_err("refused");
        assert_eq!(failure.code, Code::UnsupportedParameter, "{reasoning:?}");
        assert!(!failure.retryable);
        assert_eq!(
            server.requests_started(),
            0,
            "{reasoning:?}: a request was sent"
        );
    }
}

#[test]
fn an_option_the_model_cannot_honour_fails_before_anything_is_sent_on_the_openai_wire() {
    // The catalog of Rig says that GPT-4o does not reason, so it refuses any effort.
    let mut asked = request_on(Wire::Responses);
    asked.model = "gpt-4o".to_owned();
    asked.reasoning = Some(Reasoning::High);
    let (result, server) = call_on(
        Wire::Responses,
        vec![Wire::Responses.answer("never")],
        &asked,
    );
    let failure = result.expect_err("a model that does not reason refuses an effort");
    assert_eq!(failure.code, Code::UnsupportedParameter);
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn a_keyless_compatible_endpoint_sends_no_authorization_header() {
    let server = Server::start(vec![Wire::Chat.answer("fine")]);
    let keyless = crate::endpoints::parse_endpoints(&json!({
        "main": { "type": "openai-compatible", "baseUrl": format!("{}/v1", server.url()) }
    }))
    .unwrap()
    .remove("main")
    .unwrap();
    let response = run_with(
        &keyless,
        |_| None,
        &request_on(Wire::Chat),
        Duration::from_secs(30),
    )
    .expect("a call without a credential");
    assert_eq!(response.text, "fine");
    let seen = &server.exchanges(1)[0].seen;
    assert_eq!(seen.header("authorization"), None);
    assert_eq!(seen.path, "/v1/chat/completions");
}

#[test]
fn an_unset_or_empty_credential_fails_before_anything_is_sent_on_each_wire() {
    for wire in Wire::OPENAI {
        let server = Server::start(vec![wire.answer("never")]);
        for environment in [(|_: &str| None) as fn(&str) -> Option<OsString>, |_| {
            Some(OsString::new())
        }] {
            let failure = run_with(
                &endpoint_on(wire, &server.url()),
                environment,
                &request_on(wire),
                Duration::from_secs(5),
            )
            .expect_err("no credential");
            assert_eq!(failure.code, Code::EndpointUnavailable, "{wire:?}");
        }
        assert_eq!(server.requests_started(), 0, "{wire:?}");
    }
}

#[test]
fn a_401_and_a_403_are_authentication_failures_on_each_wire() {
    for wire in Wire::OPENAI {
        for status in [401, 403] {
            let (failure, _) = failed_on(
                wire,
                vec![Reply::raw(
                    status,
                    "application/json",
                    br#"{"error":{"message":"bad key"}}"#,
                )],
            );
            assert_eq!(failure.code, Code::Authentication, "{wire:?} {status}");
            assert!(!failure.retryable);
            assert_eq!(failure.http_status, Some(status));
        }
    }
}

#[test]
fn a_429_is_rate_limited_and_carries_its_wait_on_each_wire() {
    for wire in Wire::OPENAI {
        let (failure, _) = failed_on(
            wire,
            vec![
                Reply::raw(429, "application/json", br#"{"error":"slow"}"#)
                    .header("retry-after", "3"),
            ],
        );
        assert_eq!(failure.code, Code::RateLimited, "{wire:?}");
        assert!(failure.retryable);
        assert_eq!(failure.retry_after_ms, Some(3000), "{wire:?}");
        let (failure, _) = failed_on(wire, vec![Reply::raw(429, "application/json", b"{}")]);
        assert_eq!(failure.retry_after_ms, None, "{wire:?}");
    }
}

#[test]
fn a_500_is_a_retryable_provider_error_and_a_400_is_not_on_each_wire() {
    for wire in Wire::OPENAI {
        let (failure, _) = failed_on(wire, vec![Reply::raw(500, "text/plain", b"down")]);
        assert_eq!(failure.code, Code::ProviderError, "{wire:?}");
        assert!(failure.retryable, "{wire:?}");
        assert_eq!(failure.http_status, Some(500));
        assert!(failure.message.contains("down"));
        let (failure, _) = failed_on(
            wire,
            vec![Reply::raw(
                400,
                "application/json",
                br#"{"error":"bad request"}"#,
            )],
        );
        assert_eq!(failure.code, Code::ProviderError, "{wire:?}");
        assert!(!failure.retryable, "{wire:?}");
    }
}

#[test]
fn a_body_that_does_not_decode_is_a_malformed_response_on_each_wire() {
    for wire in Wire::OPENAI {
        let (failure, _) = failed_on(wire, vec![Reply::raw(200, "application/json", b"not json")]);
        assert_eq!(failure.code, Code::MalformedResponse, "{wire:?}");
        assert!(!failure.retryable, "{wire:?}");
    }
}

#[test]
fn a_body_that_stops_short_is_the_retryable_malformed_response_on_each_wire() {
    for wire in Wire::OPENAI {
        let (failure, _) = failed_on(
            wire,
            vec![Reply {
                status: 200,
                headers: Vec::new(),
                body: Body::Short {
                    declared: 4000,
                    sent: br#"{"id":"x"#.to_vec(),
                },
            }],
        );
        assert_eq!(failure.code, Code::MalformedResponse, "{wire:?}");
        assert!(failure.retryable, "{wire:?}");
    }
}

#[test]
fn an_answer_without_text_is_a_malformed_response_on_each_wire() {
    for wire in Wire::OPENAI {
        let (failure, _) = failed_on(wire, vec![wire.answer("")]);
        assert_eq!(
            failure.code,
            Code::MalformedResponse,
            "{wire:?}: {failure:?}"
        );
    }
}

#[test]
fn the_finish_of_each_wire_is_stop_length_or_other() {
    // The Responses wire ends short of the limit as an incomplete response. The
    // Chat wire says `length` or `content_filter` in the `finish_reason`.
    assert_eq!(
        answered_on(
            Wire::Responses,
            vec![Wire::Responses.answer_ending("cut", "max_output_tokens")]
        )
        .finish,
        Finish::Length
    );
    assert_eq!(
        answered_on(Wire::Chat, vec![Wire::Chat.answer_ending("cut", "length")]).finish,
        Finish::Length
    );
    for wire in Wire::OPENAI {
        assert_eq!(
            answered_on(wire, vec![wire.answer("whole")]).finish,
            Finish::Stop
        );
    }
    // A finish that is neither: Chat reports the tool calls that Ravel never asks for.
    assert_eq!(
        answered_on(
            Wire::Chat,
            vec![Wire::Chat.answer_ending("x", "tool_calls")]
        )
        .finish,
        Finish::Other
    );
}

#[test]
fn a_content_filter_ending_is_content_filtered_on_each_wire() {
    let (failure, _) = failed_on(
        Wire::Responses,
        vec![Wire::Responses.answer_ending("", "content_filter")],
    );
    assert_eq!(failure.code, Code::ContentFiltered, "{failure:?}");
    assert!(!failure.retryable);
    let (failure, _) = failed_on(
        Wire::Chat,
        vec![Wire::Chat.answer_ending("", "content_filter")],
    );
    assert_eq!(failure.code, Code::ContentFiltered, "{failure:?}");
    assert!(!failure.retryable);
}

#[test]
fn a_failed_response_with_a_success_status_is_a_provider_error_on_each_wire() {
    let mut responses = Wire::Responses.body("");
    responses["status"] = "failed".into();
    responses["error"] = json!({ "code": "server_error", "message": "the model fell over" });
    let (failure, _) = failed_on(Wire::Responses, vec![Reply::json(200, &responses)]);
    assert_eq!(failure.code, Code::ProviderError, "{failure:?}");
    assert!(
        failure.message.contains("the model fell over"),
        "{failure:?}"
    );

    let envelope = json!({ "error": { "message": "overloaded", "type": "server_error" } });
    let (failure, _) = failed_on(Wire::Chat, vec![Reply::json(200, &envelope)]);
    assert_eq!(failure.code, Code::ProviderError, "{failure:?}");
}

#[test]
fn an_answer_that_reports_no_usage_has_no_counters() {
    for wire in Wire::OPENAI {
        let response = answered_on(wire, vec![wire.answer_without_usage("fine")]);
        assert_eq!(response.usage, Usage::default(), "{wire:?}");
    }
}

#[test]
fn a_successful_body_beyond_the_bound_stops_at_the_bound_on_each_wire() {
    let total = 9 * 1024 * 1024;
    for wire in Wire::OPENAI {
        let (failure, server) = failed_on(
            wire,
            vec![Reply::json(200, &json!({})).with_body(wire.oversized(total, false))],
        );
        assert_eq!(failure.code, Code::OutputLimit, "{wire:?}");
        let exchange = &server.exchanges(1)[0];
        assert!(exchange.client_closed_early, "{wire:?}");
        assert!(exchange.body_bytes_sent < total, "{wire:?}");
    }
}

#[test]
fn an_answer_text_beyond_one_mib_fails_the_call_on_each_wire() {
    for wire in Wire::OPENAI {
        let exactly = answered_on(wire, vec![wire.answer(&"a".repeat(MAX_TEXT_BYTES))]);
        assert_eq!(exactly.text.len(), MAX_TEXT_BYTES, "{wire:?}");
        let (failure, _) = failed_on(wire, vec![wire.answer(&"a".repeat(MAX_TEXT_BYTES + 1))]);
        assert_eq!(failure.code, Code::OutputLimit, "{wire:?}");
        assert!(failure.message.len() < 200, "{wire:?}");
    }
}

#[test]
fn a_call_that_reaches_its_deadline_is_a_retryable_timeout_on_each_wire() {
    for wire in Wire::OPENAI {
        let server = Server::start(vec![Reply {
            status: 200,
            headers: Vec::new(),
            body: Body::Silent {
                hold: Duration::from_secs(30),
            },
        }]);
        let failure = run_with(
            &endpoint_on(wire, &server.url()),
            environment,
            &request_on(wire),
            Duration::from_millis(300),
        )
        .expect_err("a deadline");
        assert_eq!(failure.code, Code::Timeout, "{wire:?}");
        assert!(failure.retryable);
    }
}

#[test]
fn a_redirect_is_not_followed_on_each_wire() {
    for wire in Wire::OPENAI {
        let elsewhere = Server::start(vec![wire.answer("must not be reached")]);
        let target = format!("{}{}", elsewhere.url(), wire.path());
        let (failure, _) = failed_on(
            wire,
            vec![Reply::raw(302, "text/plain", b"moved").header("Location", &target)],
        );
        assert_eq!(failure.code, Code::ProviderError, "{wire:?}");
        assert_eq!(failure.http_status, Some(302));
        assert_eq!(
            elsewhere.requests_started(),
            0,
            "{wire:?}: the credential went on"
        );
    }
}

#[test]
fn a_credential_that_the_endpoint_echoes_is_redacted_on_each_wire() {
    for wire in Wire::OPENAI {
        let response = answered_on(wire, vec![wire.answer(&format!("your key is {KEY}"))]);
        assert!(!response.text.contains(KEY), "{wire:?}");
        assert!(response.text.contains("[redacted]"), "{wire:?}");
        let (failure, _) = failed_on(
            wire,
            vec![Reply::raw(
                500,
                "text/plain",
                format!("bad key {KEY}").as_bytes(),
            )],
        );
        assert!(!failure.message.contains(KEY), "{wire:?}");
    }
}
