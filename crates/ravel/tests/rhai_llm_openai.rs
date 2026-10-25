//! `ravel::llm` for the `openai` and `openai-compatible` API types through the
//! real `ravel start-session` (`docs/specs/policy-evaluation.md`, *LLM calls*,
//! *LLM endpoints and credentials*, and the requirements *Endpoint failures are
//! data* and *Credential boundary*).
//!
//! The `openai` type speaks the Responses wire, and `openai-compatible` speaks
//! Chat Completions. A case that holds for both runs once for each wire, and each
//! run has its own sandbox and its own scripted server. The real Rig client and
//! the real worker run in every case, with a dummy key and no network.

mod support;

use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::Sandbox;
use support::llm_seam::*;
use support::llm_server::{Body, Reply, Server, Wire};

/// The call with the `more` fields after the required ones, on `wire`.
fn call_on(wire: Wire, more: &str) -> String {
    ask_on(wire, "main", more)
}

#[test]
fn select_asks_an_endpoint_to_classify_the_prompt_and_routes_on_the_answer() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("review")]);
        settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
        sandbox.personal_rhai(&selecting(&format!(
            r#"    let response = {};
    let args = [response.text, response.status];"#,
            call_on(wire, ", maxTokens: 32, temperature: 0.5")
        )));

        let report = start(&sandbox, true, &[]).report();

        assert_eq!(recorded.argv().unwrap(), ["review", "ok"], "{wire:?}");
        assert_eq!(report["selection"]["reason"], "r", "{wire:?}");
        let exchanges = server.exchanges(1);
        assert_eq!(server.requests_started(), 1, "{wire:?}: one request");
        let seen = &exchanges[0].seen;
        assert_eq!(
            (seen.method.as_str(), seen.path.as_str()),
            ("POST", wire.path()),
            "{wire:?}"
        );
        let (header, value) = wire.credential_header(KEY);
        assert_eq!(seen.header(header), Some(value.as_str()), "{wire:?}");
        let body = seen.json();
        assert_eq!(body["model"], wire.model(), "{wire:?}");
        assert_eq!(body["temperature"], 0.5, "{wire:?}: {body}");
        let limit = match wire {
            Wire::Responses => "max_output_tokens",
            _ => "max_tokens",
        };
        assert_eq!(body[limit], 32, "{wire:?}: {body}");
        assert!(body.to_string().contains("classify me"), "{wire:?}: {body}");
    }
}

#[test]
fn the_default_token_limit_and_every_message_reach_the_wire() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("a"), wire.answer("b")]);
        settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
        let model = wire.model();
        sandbox.personal_rhai(&selecting(&format!(
            r#"    let first = {};
    let second = ravel::llm(#{{ endpoint: "main", model: "{model}", maxTokens: 77,
        timeoutMs: 20000,
        messages: [#{{ role: "system", content: "Answer in one word." }},
                   #{{ role: "user", content: "one" }},
                   #{{ role: "assistant", content: "two" }},
                   #{{ role: "user", content: "three" }}] }});
    let args = [first.text, second.text];"#,
            call_on(wire, "")
        )));

        start(&sandbox, true, &[]).report();

        assert_eq!(recorded.argv().unwrap(), ["a", "b"], "{wire:?}");
        let exchanges = server.exchanges(2);
        let limit = match wire {
            Wire::Responses => "max_output_tokens",
            _ => "max_tokens",
        };
        assert_eq!(exchanges[0].seen.json()[limit], 1024, "{wire:?}");
        let body = exchanges[1].seen.json();
        assert_eq!(body[limit], 77, "{wire:?}");
        let sent = body.to_string();
        let at = |text: &str| {
            sent.find(text)
                .unwrap_or_else(|| panic!("{text} in {sent}"))
        };
        assert!(
            at("Answer in one word.") < at("one")
                && at("one") < at("two")
                && at("two") < at("three"),
            "{wire:?}: {sent}"
        );
    }
}

#[test]
fn the_response_map_has_every_field_of_the_spec() {
    for wire in Wire::OPENAI {
        let (response, _) = one_call_on(wire, vec![wire.answer("an answer")], "");

        assert_eq!(response["status"], "ok", "{wire:?}");
        assert_eq!(response["endpoint"], "main");
        assert_eq!(response["model"], wire.model());
        assert_eq!(response["text"], "an answer");
        assert_eq!(response["json"], Value::Null);
        assert_eq!(response["finishReason"], "stop", "{wire:?}");
        assert_eq!(
            response["usage"],
            json!({ "inputTokens": 11, "outputTokens": 7, "totalTokens": 18 }),
            "{wire:?}"
        );
        assert_eq!(response["reportedModel"], "fixture-reported-model");
        assert_eq!(response["responseId"], wire.body("x")["id"], "{wire:?}");
        assert_eq!(response["requestId"], "req_fixture_1");
        assert!(response["elapsedMs"].is_u64(), "{wire:?}: {response}");
        assert_eq!(response["error"], Value::Null);
    }
}

#[test]
fn an_endpoint_that_reports_no_usage_gives_a_usage_map_of_unset_counters() {
    for wire in Wire::OPENAI {
        let (response, _) = one_call_on(wire, vec![wire.answer_without_usage("fine")], "");

        assert_eq!(response["status"], "ok", "{wire:?}: {response}");
        assert_eq!(
            response["usage"],
            json!({ "inputTokens": null, "outputTokens": null, "totalTokens": null }),
            "{wire:?}"
        );
    }
}

#[test]
fn a_length_ending_is_reported_as_length_and_a_filter_ending_as_a_failure() {
    let (response, _) = one_call_on(
        Wire::Responses,
        vec![Wire::Responses.answer_ending("cut off", "max_output_tokens")],
        "",
    );
    assert_eq!(response["finishReason"], "length", "{response}");
    let (response, _) = one_call_on(
        Wire::Chat,
        vec![Wire::Chat.answer_ending("cut off", "length")],
        "",
    );
    assert_eq!(response["finishReason"], "length", "{response}");

    for (wire, reason) in [
        (Wire::Responses, "content_filter"),
        (Wire::Chat, "content_filter"),
    ] {
        let (response, _) = one_call_on(wire, vec![wire.answer_ending("", reason)], "");
        assert_eq!(response["status"], "failed", "{wire:?}: {response}");
        assert_eq!(response["error"]["code"], "content_filtered", "{wire:?}");
        assert_eq!(response["error"]["retryable"], false);
    }
}

#[test]
fn each_failure_of_an_endpoint_is_a_code_that_the_script_can_read() {
    let garbled = || Reply::raw(200, "application/json", b"this is not json");
    for wire in Wire::OPENAI {
        for (label, reply, code, retryable, status) in [
            (
                "401",
                Reply::raw(
                    401,
                    "application/json",
                    br#"{"error":{"message":"bad key"}}"#,
                ),
                "authentication",
                false,
                json!(401),
            ),
            (
                "403",
                Reply::raw(403, "application/json", br#"{"error":{"message":"no"}}"#),
                "authentication",
                false,
                json!(403),
            ),
            (
                "429",
                Reply::raw(429, "application/json", br#"{"error":"slow down"}"#)
                    .header("retry-after", "3"),
                "rate_limited",
                true,
                json!(429),
            ),
            (
                "500",
                Reply::raw(500, "text/plain", b"down for maintenance"),
                "provider_error",
                true,
                json!(500),
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
        ] {
            let (response, _) = one_call_on(wire, vec![reply], "");
            let label = format!("{wire:?} {label}: {response}");
            assert_eq!(response["status"], "failed", "{label}");
            let error = &response["error"];
            assert_eq!(error["code"], code, "{label}");
            assert_eq!(error["retryable"], retryable, "{label}");
            assert_eq!(error["httpStatus"], status, "{label}");
            for unset in [
                "text",
                "json",
                "finishReason",
                "usage",
                "reportedModel",
                "responseId",
            ] {
                assert_eq!(response[unset], Value::Null, "{label}: {unset}");
            }
        }
    }
}

#[test]
fn a_rate_limit_carries_its_wait_and_the_failure_message_holds_the_body() {
    for wire in Wire::OPENAI {
        let (response, _) = one_call_on(
            wire,
            vec![
                Reply::raw(429, "application/json", br#"{"error":"slow down"}"#)
                    .header("retry-after", "3"),
            ],
            "",
        );
        let error = &response["error"];
        assert_eq!(error["retryAfterMs"], 3000, "{wire:?}: {response}");
        assert!(
            error["message"].as_str().unwrap().contains("slow down"),
            "{wire:?}: {error}"
        );

        let (response, _) = one_call_on(wire, vec![Reply::raw(429, "text/plain", b"later")], "");
        assert_eq!(response["error"]["retryAfterMs"], Value::Null, "{wire:?}");
    }
}

#[test]
fn a_rate_limit_on_one_type_falls_back_to_an_endpoint_of_another_type() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let limited = Server::start(vec![
        Reply::raw(429, "application/json", br#"{"error":"slow down"}"#).header("retry-after", "3"),
    ]);
    let backup = Server::start(vec![Wire::Chat.answer("from the backup")]);
    support::write(
        &sandbox.home.join(SETTINGS),
        &json!({ "endpoints": {
            "first": { "type": "openai", "apiKeyEnv": KEY_VARIABLE,
                       "baseUrl": format!("{}/v1", limited.url()) },
            "second": { "type": "openai-compatible", "apiKeyEnv": KEY_VARIABLE,
                        "baseUrl": format!("{}/v1", backup.url()) },
        } })
        .to_string(),
    );
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let first = {};
    let used = first;
    if first.status == "failed" && first.error.retryable {{
        used = {};
    }}
    let args = [first.error.code, first.error.retryAfterMs.to_string(), used.text];"#,
        ask_on(Wire::Responses, "first", ""),
        ask_on(Wire::Chat, "second", "")
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["rate_limited", "3000", "from the backup"]
    );
    assert_eq!(limited.requests_started(), 1);
    assert_eq!(backup.requests_started(), 1);
}

#[test]
fn a_call_that_reaches_its_own_deadline_is_a_retryable_timeout_and_closes_the_connection() {
    for wire in Wire::OPENAI {
        for (label, reply) in [
            ("a stall after the head", stalled()),
            ("no head at all", silent()),
        ] {
            let began = Instant::now();
            let (response, server) = one_call_on(wire, vec![reply], ", timeoutMs: 400");

            let label = format!("{wire:?}, {label}: {response}");
            let error = &response["error"];
            assert_eq!(error["code"], "timeout", "{label}");
            assert_eq!(error["retryable"], true, "{label}");
            assert!(response["elapsedMs"].as_u64().unwrap() >= 400, "{label}");
            assert!(began.elapsed() < Duration::from_secs(20), "{label}");
            assert!(server.exchanges(1)[0].client_closed_early, "{label}");
        }
    }
}

#[test]
fn an_oversized_successful_body_fails_the_call_and_the_connection_closes_early() {
    let total = 9 * 1024 * 1024;
    for wire in Wire::OPENAI {
        let (response, server) = one_call_on(
            wire,
            vec![Reply::json(200, &json!({})).with_body(wire.oversized(total, false))],
            "",
        );

        assert_eq!(
            response["error"]["code"], "output_limit",
            "{wire:?}: {response}"
        );
        assert_eq!(response["error"]["retryable"], false);
        let exchange = &server.exchanges(1)[0];
        assert!(
            exchange.client_closed_early,
            "{wire:?}: the server wrote {} of {total} bytes and saw no close",
            exchange.body_bytes_sent
        );
        assert!(exchange.body_bytes_sent < total, "{wire:?}");
    }
}

#[test]
fn a_declared_length_beyond_the_bound_fails_from_the_head_alone() {
    for wire in Wire::OPENAI {
        let (response, _) = one_call_on(
            wire,
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

        assert_eq!(
            response["error"]["code"], "output_limit",
            "{wire:?}: {response}"
        );
    }
}

#[test]
fn an_answer_text_beyond_one_mib_in_a_body_below_the_bound_fails_the_call() {
    for wire in Wire::OPENAI {
        let (response, _) = one_call_on(wire, vec![wire.answer(&"a".repeat(1024 * 1024 + 1))], "");

        assert_eq!(response["status"], "failed", "{wire:?}: {response}");
        let error = &response["error"];
        assert_eq!(error["code"], "output_limit", "{wire:?}");
        assert!(error["message"].as_str().unwrap().len() < 200, "{wire:?}");
        assert_eq!(response["requestId"], "req_fixture_1", "{wire:?}");
    }
}

#[test]
fn a_model_outside_the_list_of_an_endpoint_fails_and_a_listed_model_works() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("listed")]);
        settings_on(
            wire,
            &sandbox,
            &[("main", &server)],
            &json!({ "models": [wire.model()] }),
        );
        sandbox.personal_rhai(&selecting(&format!(
            r#"    let unlisted = ravel::llm(#{{ endpoint: "main", model: "not-listed",
        messages: [#{{ role: "user", content: "x" }}] }});
    let listed = {};
    let args = [unlisted.error.code, unlisted.error.retryable.to_string(), listed.status];"#,
            call_on(wire, "")
        )));

        // No credential in the environment would fail the listed call first, so
        // the credential is present and only the model decides.
        start(&sandbox, true, &[]).report();

        assert_eq!(
            recorded.argv().unwrap(),
            ["model_not_offered", "false", "ok"],
            "{wire:?}"
        );
        assert_eq!(
            server.requests_started(),
            1,
            "{wire:?}: only the listed call is sent"
        );
    }
}

#[test]
fn a_compatible_endpoint_without_a_credential_variable_works_and_discovery_says_available() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Wire::Chat.answer("from a local server")]);
    support::write(
        &sandbox.home.join(SETTINGS),
        &json!({ "endpoints": {
            "main": { "type": "openai-compatible", "baseUrl": format!("{}/v1", server.url()) },
        } })
        .to_string(),
    );
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let found = ravel::endpoints()[0];
    let response = {};
    let args = [found.name, found.available.to_string(), response.status, response.text];"#,
        call_on(Wire::Chat, "")
    )));

    // The credential variable is absent from the environment and from the settings.
    start(&sandbox, false, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["main", "true", "ok", "from a local server"]
    );
    let seen = &server.exchanges(1)[0].seen;
    assert_eq!(seen.header("authorization"), None);
}

#[test]
fn a_compatible_endpoint_with_a_credential_variable_that_is_unset_is_unavailable() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Wire::Chat.answer("never sent")]);
    settings_on(Wire::Chat, &sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let found = ravel::endpoints()[0];
    let response = {};
    let args = [found.available.to_string(), found.reason, response.error.code];"#,
        call_on(Wire::Chat, "")
    )));

    start(&sandbox, false, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["false", "credential_unset", "endpoint_unavailable"]
    );
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn reasoning_reaches_the_openai_wire_and_an_option_the_wire_cannot_honour_fails_unsent() {
    // The `openai` type sends the effort in the field of the Responses wire.
    let (response, server) = one_call_on(
        Wire::Responses,
        vec![Wire::Responses.answer("thought")],
        ", reasoning: \"high\"",
    );
    assert_eq!(response["status"], "ok", "{response}");
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(body["reasoning"]["effort"], "high", "{body}");

    // Rig knows no mapping of `reasoning` onto a Chat Completions server that it
    // does not register, and the catalog of Rig says that GPT-4o does not reason.
    // Both fail before a connection opens.
    for (wire, model, option) in [
        (Wire::Chat, Wire::Chat.model(), "off"),
        (Wire::Chat, Wire::Chat.model(), "high"),
        (Wire::Responses, "gpt-4o", "high"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("never sent")]);
        settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
        let call = format!(
            r#"ravel::llm(#{{ endpoint: "main", model: "{model}", reasoning: "{option}",
            messages: [#{{ role: "user", content: "x" }}] }})"#
        );

        let response = answered(&sandbox, &recorded, true, &call);

        let label = format!("{wire:?} {model} {option}: {response}");
        assert_eq!(
            response["error"]["code"], "unsupported_parameter",
            "{label}"
        );
        assert_eq!(response["error"]["retryable"], false, "{label}");
        assert_eq!(server.requests_started(), 0, "{label}");
    }
}

#[test]
fn a_schema_is_still_a_malformed_request_on_each_type() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("never sent")]);
        settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
        sandbox.personal_rhai(&selecting(&format!(
            "    {};\n    let args = [];",
            call_on(wire, ", schema: #{ type: \"object\" }")
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
                .starts_with("ravel::llm(schema)"),
            "{wire:?}: {error}"
        );
        assert_eq!(server.requests_started(), 0, "{wire:?}");
    }
}

#[test]
fn the_credential_reaches_the_server_only_in_its_header_and_an_echo_leaks_nothing() {
    for wire in Wire::OPENAI {
        let (response, server) =
            one_call_on(wire, vec![wire.answer(&format!("the key is {KEY}"))], "");

        assert_eq!(response["text"], "the key is [redacted]", "{wire:?}");
        let seen = &server.exchanges(1)[0].seen;
        assert!(
            !String::from_utf8_lossy(&seen.body).contains(KEY),
            "{wire:?}: the body"
        );
        assert!(!seen.path.contains(KEY), "{wire:?}: the path");
        for (name, value) in &seen.headers {
            let (header, _) = wire.credential_header(KEY);
            assert!(
                !value.contains(KEY) || *name == header,
                "{wire:?}: {name} holds the credential"
            );
        }
    }
}

#[test]
fn a_redirect_is_not_followed_and_the_credential_goes_nowhere_else() {
    for wire in Wire::OPENAI {
        let elsewhere = Server::start(vec![wire.answer("must not be reached")]);
        let target = format!("{}{}", elsewhere.url(), wire.path());
        let (response, _) = one_call_on(
            wire,
            vec![Reply::raw(302, "text/plain", b"moved").header("Location", &target)],
            "",
        );

        assert_eq!(
            response["error"]["code"], "provider_error",
            "{wire:?}: {response}"
        );
        assert_eq!(response["error"]["httpStatus"], 302);
        assert_eq!(
            elsewhere.requests_started(),
            0,
            "{wire:?}: the credential went on"
        );
    }
}

#[test]
fn the_budgets_count_a_call_of_each_type_by_the_tokens_it_reports() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![
            wire.answer_reporting("a", 300),
            wire.answer_reporting("b", 42),
            wire.answer_without_usage("c"),
        ]);
        settings_on(wire, &sandbox, &[("main", &server)], &json!({}));
        sandbox.personal_rhai(&selecting(&format!(
            r#"    let before = ravel::remaining();
    {};
    let middle = ravel::remaining();
    {};
    let after = ravel::remaining();
    {};
    let last = ravel::remaining();
    let args = [ravel::json_encode(#{{ before: before, middle: middle, after: after,
                                      last: last }})];"#,
            call_on(wire, ", maxTokens: 500"),
            call_on(wire, ", maxTokens: 500"),
            call_on(wire, ", maxTokens: 100"),
        )));

        start(&sandbox, true, &[]).report();

        let seen: Value = serde_json::from_str(&recorded.argv().unwrap()[0]).unwrap();
        let live = |at: &str| {
            (
                seen[at]["llmCalls"].clone(),
                seen[at]["llmOutputTokens"].clone(),
            )
        };
        assert_eq!(live("before"), (json!(8), json!(32_768)), "{wire:?}");
        assert_eq!(live("middle"), (json!(7), json!(32_768 - 300)), "{wire:?}");
        assert_eq!(live("after"), (json!(6), json!(32_768 - 342)), "{wire:?}");
        // A call that reports no usage costs the tokens that it was allowed.
        assert_eq!(
            live("last"),
            (json!(5), json!(32_768 - 342 - 100)),
            "{wire:?}"
        );
    }
}

#[test]
fn the_tokens_that_remain_lower_max_tokens_on_each_wire() {
    for wire in Wire::OPENAI {
        let sandbox = Sandbox::new();
        // The agent must exist for the launch that follows, but the test reads
        // the requests of the server and not the arguments of the harness.
        let _recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer_reporting("a", 30), wire.answer("b")]);
        settings_with_on(
            wire,
            &sandbox,
            &[("main", &server)],
            &json!({}),
            &json!({ "llmOutputTokens": 100 }),
        );
        sandbox.personal_rhai(&selecting(&format!(
            "    {};\n    {};\n    let args = [];",
            call_on(wire, ", maxTokens: 90"),
            call_on(wire, ", maxTokens: 500"),
        )));

        start(&sandbox, true, &[]).report();

        let exchanges = server.exchanges(2);
        let limit = match wire {
            Wire::Responses => "max_output_tokens",
            _ => "max_tokens",
        };
        assert_eq!(exchanges[0].seen.json()[limit], 90, "{wire:?}");
        assert_eq!(exchanges[1].seen.json()[limit], 70, "{wire:?}");
    }
}
