//! Structured output of `ravel::llm` through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *LLM calls*, and the requirement
//! *Endpoint failures are data*).
//!
//! A `schema` becomes the `output_schema` of Rig on each wire, and Ravel checks
//! the answer itself against the schema of the script. The validator has no
//! seam of its own, so every case here sends its schema and its answer through
//! the fake endpoint, and the response map decides the result. A case that holds
//! for every API type runs once for each wire.

mod support;

use serde_json::{Value, json};
use support::Sandbox;
use support::llm_seam::*;
use support::llm_server::{Reply, Server, Wire};

/// The call with the Rhai expression `schema` and the `more` fields after it.
fn call_on(wire: Wire, schema: &str, more: &str) -> String {
    ask_on(wire, "main", &format!(", schema: {schema}{more}"))
}

/// The settings fields that make an endpoint of `wire` take a schema. The
/// Anthropic and OpenAI types always do, and the owner marks a compatible one.
fn structured(wire: Wire) -> Value {
    match wire {
        Wire::Chat => json!({ "structuredOutput": true }),
        _ => json!({}),
    }
}

/// [`one_call_on`] for an endpoint that takes a schema.
fn one_call(wire: Wire, script: Vec<Reply>, more: &str) -> (Value, Server) {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(script);
    settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
    (
        answered(&sandbox, &recorded, true, &ask_on(wire, "main", more)),
        server,
    )
}

/// A person: `name` is required, `age` is not, and nothing else is allowed.
const PERSON: &str = r#"#{ type: "object",
    properties: #{ name: #{ type: "string" }, age: #{ type: "integer" } },
    required: ["name"], additionalProperties: false }"#;

/// The responses to `cases`, each a schema and the text that the endpoint
/// answers with. One policy makes every call in turn, so a table of edge cases
/// costs one `start-session` for each wire.
fn verdicts(wire: Wire, cases: &[(&str, &str)]) -> Vec<Value> {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(
        cases
            .iter()
            .map(|(_, answer)| wire.answer(answer))
            .collect(),
    );
    settings_with_on(
        wire,
        &sandbox,
        &[("main", &server)],
        &structured(wire),
        &json!({ "llmCalls": 64 }),
    );
    let pushes: String = cases
        .iter()
        .map(|(schema, _)| {
            format!(
                "    out.push({});\n",
                call_on(wire, schema, ", maxTokens: 16")
            )
        })
        .collect();
    sandbox.personal_rhai(&selecting(&format!(
        "    let out = [];\n{pushes}    let args = [ravel::json_encode(out)];"
    )));
    start(&sandbox, true, &[]).report();
    let args = recorded.argv().unwrap();
    let responses: Vec<Value> = serde_json::from_str(&args[0]).unwrap();
    assert_eq!(responses.len(), cases.len(), "{wire:?}");
    assert_eq!(server.requests_started(), cases.len(), "{wire:?}");
    responses
}

/// The message of a failed response with the code `malformed_response`, which
/// is never retryable for a mismatch.
fn mismatch(label: &str, wire: Wire, response: &Value) -> String {
    let context = format!("{label} on {wire:?}: {response}");
    assert_eq!(response["status"], "failed", "{context}");
    assert_eq!(response["error"]["code"], "malformed_response", "{context}");
    assert_eq!(response["error"]["retryable"], false, "{context}");
    assert_eq!(response["json"], Value::Null, "{context}");
    assert_eq!(response["text"], Value::Null, "{context}");
    response["error"]["message"].as_str().unwrap().to_owned()
}

#[test]
fn a_schema_reaches_the_wire_as_the_output_schema_and_a_matching_answer_arrives_in_json() {
    let schema = r#"#{ title: "Person", description: "Someone", type: "object",
        properties: #{ name: #{ type: "string" },
                       pet: #{ type: "object", properties: #{ kind: #{ type: "string" } } } },
        required: ["name"] }"#;
    let answer = "  {\"name\": \"Ada\", \"pet\": {\"kind\": \"cat\"}}\n";
    for wire in Wire::ALL {
        let (response, server) = one_call(
            wire,
            vec![wire.answer(answer)],
            &format!(", schema: {schema}"),
        );

        assert_eq!(response["status"], "ok", "{wire:?}: {response}");
        assert_eq!(
            response["json"],
            json!({ "name": "Ada", "pet": { "kind": "cat" } }),
            "{wire:?}"
        );
        assert_eq!(response["text"], answer, "{wire:?}: the text is untouched");
        assert_eq!(response["error"], Value::Null, "{wire:?}");

        let body = server.exchanges(1)[0].seen.json();
        let sent = wire
            .schema_in(&body)
            .unwrap_or_else(|| panic!("{wire:?}: no schema in {body}"));
        // Rig rewrites the schema on each wire: every property is required, and
        // each object without the keyword gets `additionalProperties: false`.
        assert_eq!(sent["type"], "object", "{wire:?}: {sent}");
        assert_eq!(sent["required"], json!(["name", "pet"]), "{wire:?}: {sent}");
        assert_eq!(sent["additionalProperties"], false, "{wire:?}: {sent}");
        assert_eq!(sent["properties"]["name"]["type"], "string", "{wire:?}");
        assert_eq!(
            sent["properties"]["pet"]["additionalProperties"], false,
            "{wire:?}: {sent}"
        );
        assert_eq!(
            sent["properties"]["pet"]["required"],
            json!(["kind"]),
            "{wire:?}"
        );
        // The keywords that the check ignores still travel.
        assert_eq!(sent["title"], "Person", "{wire:?}");
        assert_eq!(sent["description"], "Someone", "{wire:?}");
        // The kind of the format, and the name that the OpenAI wires want.
        let member = wire.schema_member(&body).unwrap();
        match wire {
            Wire::Messages => assert_eq!(member["type"], "json_schema"),
            Wire::Responses => {
                assert_eq!(member["type"], "json_schema");
                assert_eq!(member["name"], "Person");
                assert_eq!(member["strict"], true);
            }
            Wire::Chat => {
                assert_eq!(body["response_format"]["type"], "json_schema");
                assert_eq!(member["name"], "Person");
                assert_eq!(member["strict"], true);
            }
        }
    }
}

#[test]
fn a_call_without_a_schema_sends_none_and_returns_no_json() {
    for wire in Wire::ALL {
        let (response, server) = one_call(wire, vec![wire.answer("{\"a\":1}")], "");

        assert_eq!(response["status"], "ok", "{wire:?}");
        assert_eq!(response["json"], Value::Null, "{wire:?}");
        let body = server.exchanges(1)[0].seen.json();
        assert_eq!(wire.schema_member(&body), None, "{wire:?}: {body}");
    }
}

#[test]
fn an_answer_that_matches_the_schema_of_the_script_passes_whatever_rig_sent() {
    let named = r#"#{ type: "object", properties: #{ a: #{ type: "string" } } }"#;
    for wire in Wire::ALL {
        let responses = verdicts(
            wire,
            &[
                // Rig marks every property required on the wire. The schema of the
                // script does not, so a missing `age` is a match.
                (PERSON, r#"{"name":"Ada"}"#),
                // Rig adds `additionalProperties: false` to an object that lacks
                // the keyword. The schema of the script is silent, so an extra
                // property is a match.
                (named, r#"{"a":"x","b":1}"#),
                (
                    r#"#{ type: "object", properties: #{ a: #{ type: "string" } }, additionalProperties: true }"#,
                    r#"{"a":"x","b":1}"#,
                ),
                (r#"#{ type: "integer" }"#, "3.0"),
                (r#"#{ type: ["string", "null"] }"#, "null"),
                (r#"#{ type: "number" }"#, "3"),
                (r#"#{ enum: ["red", "green", 1, true] }"#, r#""green""#),
                (r#"#{ enum: [1] }"#, "1.0"),
                (
                    r#"#{ type: "array", items: #{ type: "integer" } }"#,
                    "[1, 2, 3]",
                ),
                (r#"#{ type: "array", items: #{ type: "integer" } }"#, "[]"),
                (r#"#{ type: "string" }"#, r#""hello""#),
                // A keyword says nothing about the kinds of value it does not
                // describe.
                (r#"#{ required: ["x"] }"#, "[]"),
                (r#"#{ items: #{ type: "string" } }"#, r#"{"x": 1}"#),
                (r#"#{}"#, r#"{"anything": [1, "two", null]}"#),
            ],
        );

        for (index, response) in responses.iter().enumerate() {
            assert_eq!(
                response["status"], "ok",
                "case {index} on {wire:?}: {response}"
            );
        }
        // An answer of `null` is a match, and `json` is then `()` as well.
        assert_eq!(responses[4]["json"], Value::Null, "{wire:?}");
        assert_eq!(responses[0]["json"], json!({ "name": "Ada" }), "{wire:?}");
        assert_eq!(
            responses[1]["json"],
            json!({ "a": "x", "b": 1 }),
            "{wire:?}"
        );
        assert_eq!(responses[8]["json"], json!([1, 2, 3]), "{wire:?}");
        assert_eq!(responses[10]["json"], json!("hello"), "{wire:?}");
    }
}

#[test]
fn an_answer_that_does_not_match_fails_the_call_with_the_location() {
    let nested = r#"#{ type: "object", properties: #{ items: #{ type: "array",
        items: #{ type: "object", properties: #{ name: #{ type: "string" } } } } } }"#;
    for wire in Wire::ALL {
        let cases = [
            (
                PERSON,
                r#"{"name":"Ada","age":3.5}"#,
                "json.age: expected integer, found number",
            ),
            (
                PERSON,
                r#"{"name":3}"#,
                "json.name: expected string, found integer",
            ),
            (
                PERSON,
                r#"{"age":3}"#,
                "json: the required property name is missing",
            ),
            (
                PERSON,
                r#"{"name":"Ada","nick":"A"}"#,
                "json.nick: the schema allows no additional property",
            ),
            (
                r#"#{ type: "object", properties: #{ color: #{ enum: ["red", "green"] } } }"#,
                r#"{"color":"blue"}"#,
                "json.color: it is none of the 2 values that enum allows",
            ),
            (
                r#"#{ type: "array", items: #{ type: "string" } }"#,
                r#"["a", 2]"#,
                "json[1]: expected string, found integer",
            ),
            (
                nested,
                r#"{"items":[{"name":"a"},{"name":3}]}"#,
                "json.items[1].name: expected string, found integer",
            ),
            (
                r#"#{ type: ["string", "null"] }"#,
                "3",
                "json: expected string or null, found integer",
            ),
            (
                r#"#{ type: "object" }"#,
                "[]",
                "json: expected object, found array",
            ),
            (
                r#"#{ type: "boolean" }"#,
                "null",
                "json: expected boolean, found null",
            ),
            (
                r#"#{ enum: [1, 2] }"#,
                "1.5",
                "json: it is none of the 2 values that enum allows",
            ),
        ];
        let table: Vec<_> = cases
            .iter()
            .map(|(schema, answer, _)| (*schema, *answer))
            .collect();

        let responses = verdicts(wire, &table);

        for ((_, answer, location), response) in cases.iter().zip(&responses) {
            let message = mismatch(answer, wire, response);
            assert_eq!(
                message,
                format!("the answer does not match the schema: {location}"),
                "{answer} on {wire:?}"
            );
        }
    }
}

#[test]
fn a_long_key_in_a_message_is_cut() {
    let long = "k".repeat(200);
    for wire in Wire::ALL {
        let responses = verdicts(
            wire,
            &[(
                r#"#{ additionalProperties: false }"#,
                &format!(r#"{{"{long}": 1}}"#),
            )],
        );

        let message = mismatch("a long key", wire, &responses[0]);

        let cut = format!("{}…", "k".repeat(64));
        assert!(
            message.contains(&format!("json.{cut}:")),
            "{wire:?}: {message}"
        );
        assert!(!message.contains(&"k".repeat(65)), "{wire:?}: {message}");
    }
}

#[test]
fn an_answer_that_is_not_json_fails_the_call() {
    for wire in Wire::ALL {
        let cases = [
            "Sure! {\"a\": 1}",
            "```json\n{\"a\": 1}\n```",
            "{\"a\": 1} and more",
            "{\"a\": ",
            "not json at all",
        ];
        let table: Vec<_> = cases
            .iter()
            .map(|answer| (r#"#{ type: "object" }"#, *answer))
            .collect();

        let responses = verdicts(wire, &table);

        for (answer, response) in cases.iter().zip(&responses) {
            let message = mismatch(answer, wire, response);
            assert!(
                message.starts_with("the answer is not JSON: "),
                "{answer:?}: {message}"
            );
            assert!(
                !message.contains("more"),
                "{answer:?}: no text of the answer: {message}"
            );
        }
    }
}

#[test]
fn an_answer_nested_past_the_converter_fails_the_call_and_one_at_the_limit_passes() {
    let nested = |levels: usize| format!("{}{}", "[".repeat(levels), "]".repeat(levels));
    for wire in Wire::ALL {
        let responses = verdicts(wire, &[(r#"#{}"#, &nested(101)), (r#"#{}"#, &nested(127))]);
        for deeper in &responses {
            let message = mismatch("a deep answer", wire, deeper);
            assert!(
                message.starts_with("the answer nests deeper than 100 levels, at json"),
                "{wire:?}: {message}"
            );
        }

        // The script reads the value of the limit without encoding it again,
        // because a map around it would pass the limit of the encoder.
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer(&nested(100))]);
        settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
        sandbox.personal_rhai(&selecting(&format!(
            "    let response = {};\n    let args = [response.status, response.json.len().to_string()];",
            call_on(wire, r#"#{}"#, "")
        )));

        start(&sandbox, true, &[]).report();

        assert_eq!(recorded.argv().unwrap(), ["ok", "1"], "{wire:?}");
    }
}

#[test]
fn a_large_answer_that_matches_reaches_the_script_whole() {
    // About 0.9 MiB of text: the largest answers stay below the 1 MiB bound of
    // the text, and the value then crosses the protocol with the text beside it.
    let count = 400_000;
    let answer = format!("[{}0]", "0,".repeat(count));
    for wire in Wire::ALL {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer(&answer)]);
        settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
        let call = call_on(
            wire,
            r#"#{ type: "array", items: #{ type: "integer" } }"#,
            "",
        );
        sandbox.personal_rhai(&selecting(&format!(
            "    let response = {call};\n    let args = [response.status, response.json.len().to_string()];"
        )));

        start(&sandbox, true, &[]).report();

        assert_eq!(
            recorded.argv().unwrap(),
            ["ok".to_owned(), (count + 1).to_string()],
            "{wire:?}"
        );
    }
}

#[test]
fn the_credential_in_a_json_answer_is_redacted_before_the_value_is_read() {
    for wire in Wire::ALL {
        let answer = format!(r#"{{"{KEY}": "the key is {KEY}"}}"#);
        let (response, _) = one_call(
            wire,
            vec![wire.answer(&answer)],
            ", schema: #{ type: \"object\" }",
        );

        assert_eq!(response["status"], "ok", "{wire:?}: {response}");
        assert_eq!(
            response["json"],
            json!({ "[redacted]": "the key is [redacted]" }),
            "{wire:?}"
        );
        assert!(!response.to_string().contains(KEY), "{wire:?}: {response}");
    }
}

#[test]
fn a_mismatch_was_sent_so_it_uses_a_call_and_its_effective_max_tokens() {
    // The failure carries no usage, so the call costs `maxTokens`, as any sent
    // call that failed does.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(vec![Wire::Messages.answer("[]")]);
    settings(&sandbox, &[("main", &server)], &json!({}));
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let response = {};
    let left = ravel::remaining();
    let args = [response.error.code, left.llmCalls.to_string(),
                left.llmOutputTokens.to_string()];"#,
        call_on(Wire::Messages, r#"#{ type: "object" }"#, ", maxTokens: 50"),
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["malformed_response", "7", &(32_768 - 50).to_string()]
    );
}

/// A sandbox with the endpoint `main` of `openai-compatible` type at a server
/// that plays `script`, and the `extra` fields of the settings entry.
fn compatible(
    script: Vec<Reply>,
    extra: &Value,
    budgets: &Value,
) -> (Sandbox, support::rhai::Recorded, Server) {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let server = Server::start(script);
    settings_with_on(Wire::Chat, &sandbox, &[("main", &server)], extra, budgets);
    (sandbox, recorded, server)
}

#[test]
fn a_schema_for_a_compatible_endpoint_without_structured_output_fails_and_sends_nothing() {
    let (sandbox, recorded, server) = compatible(
        vec![Wire::Chat.answer("{}")],
        &json!({ "structuredOutput": false }),
        &Value::Null,
    );

    let response = answered(
        &sandbox,
        &recorded,
        true,
        &call_on(Wire::Chat, r#"#{ type: "object" }"#, ""),
    );

    assert_eq!(response["status"], "failed", "{response}");
    assert_eq!(
        response["error"]["code"], "unsupported_capability",
        "{response}"
    );
    assert_eq!(response["error"]["retryable"], false, "{response}");
    assert_eq!(response["error"]["httpStatus"], Value::Null, "{response}");
    assert!(
        response["error"]["message"]
            .as_str()
            .unwrap()
            .contains("structuredOutput"),
        "{response}"
    );
    assert_eq!(
        server.requests_started(),
        0,
        "the fake endpoint receives nothing"
    );
}

#[test]
fn an_entry_that_does_not_name_structured_output_is_without_it() {
    let (sandbox, recorded, server) =
        compatible(vec![Wire::Chat.answer("{}")], &json!({}), &Value::Null);

    let response = answered(
        &sandbox,
        &recorded,
        true,
        &call_on(Wire::Chat, r#"#{ type: "object" }"#, ""),
    );

    assert_eq!(
        response["error"]["code"], "unsupported_capability",
        "{response}"
    );
    assert_eq!(server.requests_started(), 0);
}

#[test]
fn a_compatible_endpoint_with_structured_output_takes_the_schema_and_one_without_takes_plain_calls()
{
    let (response, server) = {
        let (sandbox, recorded, server) = compatible(
            vec![Wire::Chat.answer(r#"{"a":"x"}"#)],
            &json!({ "structuredOutput": true }),
            &Value::Null,
        );
        let call = call_on(
            Wire::Chat,
            r#"#{ type: "object", properties: #{ a: #{ type: "string" } } }"#,
            "",
        );
        (answered(&sandbox, &recorded, true, &call), server)
    };
    assert_eq!(response["status"], "ok", "{response}");
    assert_eq!(response["json"], json!({ "a": "x" }));
    let body = server.exchanges(1)[0].seen.json();
    assert!(Wire::Chat.schema_in(&body).is_some(), "{body}");

    // The same endpoint without the setting still answers a call with no schema,
    // and the request carries no `response_format`.
    let (sandbox, recorded, server) = compatible(
        vec![Wire::Chat.answer("plain")],
        &json!({ "structuredOutput": false }),
        &Value::Null,
    );
    let plain = answered(&sandbox, &recorded, true, &ask_on(Wire::Chat, "main", ""));
    assert_eq!(plain["status"], "ok", "{plain}");
    assert_eq!(plain["text"], "plain");
    let body = server.exchanges(1)[0].seen.json();
    assert_eq!(Wire::Chat.schema_member(&body), None, "{body}");
    assert!(body.get("response_format").is_none(), "{body}");
}

#[test]
fn an_unsupported_capability_uses_no_budget() {
    // The budget holds one call and one token. The refused call must leave it
    // whole, so the second call is the one that spends it.
    let (sandbox, recorded, server) = compatible(
        vec![Wire::Chat.answer_reporting("fine", 1)],
        &json!({ "structuredOutput": false }),
        &json!({ "llmCalls": 1, "llmOutputTokens": 1 }),
    );
    sandbox.personal_rhai(&selecting(&format!(
        r#"    let refused = {};
    let left = ravel::remaining();
    let sent = {};
    let args = [refused.error.code, left.llmCalls.to_string(),
                left.llmOutputTokens.to_string(), sent.status,
                ravel::remaining().llmCalls.to_string()];"#,
        call_on(Wire::Chat, r#"#{ type: "object" }"#, ", maxTokens: 1"),
        ask_on(Wire::Chat, "main", ", maxTokens: 1"),
    )));

    start(&sandbox, true, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["unsupported_capability", "1", "1", "ok", "0"]
    );
    assert_eq!(server.requests_started(), 1);
}

#[test]
fn an_endpoint_can_refuse_a_schema_of_the_subset_and_the_call_selects_no_fallback() {
    let schema = r#"#{ type: "object", properties: #{ a: #{ type: "string" } },
        additionalProperties: true }"#;
    for wire in Wire::ALL {
        let refusal = Reply::json(
            400,
            &json!({ "error": { "type": "invalid_request_error",
                "message": "additionalProperties must be false" } }),
        );
        let (response, server) = one_call(wire, vec![refusal], &format!(", schema: {schema}"));

        assert_eq!(response["status"], "failed", "{wire:?}: {response}");
        let error = &response["error"];
        assert_eq!(error["code"], "provider_error", "{wire:?}: {response}");
        assert_eq!(error["httpStatus"], 400, "{wire:?}: {response}");
        assert_eq!(
            error["retryable"], false,
            "{wire:?}: a policy that follows retryable selects no fallback"
        );
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .contains("additionalProperties must be false"),
            "{wire:?}: {response}"
        );
        // Rig keeps the keyword that the script set.
        let body = server.exchanges(1)[0].seen.json();
        assert_eq!(
            wire.schema_in(&body).unwrap()["additionalProperties"],
            true,
            "{wire:?}: {body}"
        );
    }
}

/// A `select` that builds a schema of `levels` nested schemas through `step`,
/// a Rhai statement that wraps `schema`, and makes the call.
fn nested_call(wire: Wire, levels: usize, step: &str) -> String {
    selecting(&format!(
        "    let schema = #{{ type: \"string\" }};\n    for step in 1..{levels} {{ {step} }}\n    {};\n    let args = [];",
        ask_on(wire, "main", ", schema: schema")
    ))
}

const BY_PROPERTY: &str = r#"schema = #{ type: "object", properties: #{ a: schema } };"#;
const BY_ITEMS: &str = r#"schema = #{ type: "array", items: schema };"#;

#[test]
fn a_schema_of_32_levels_passes_on_every_wire() {
    let by_property = format!("{}\"x\"{}", "{\"a\":".repeat(31), "}".repeat(31));
    let by_items = format!("{}\"x\"{}", "[".repeat(31), "]".repeat(31));
    for wire in Wire::ALL {
        for (step, answer) in [(BY_PROPERTY, &by_property), (BY_ITEMS, &by_items)] {
            let sandbox = Sandbox::new();
            let recorded = sandbox.recording_agent();
            let server = Server::start(vec![wire.answer(answer)]);
            settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
            sandbox.personal_rhai(&selecting(&format!(
                "    let schema = #{{ type: \"string\" }};\n    for step in 1..32 {{ {step} }}\n    let response = {};\n    let args = [response.status];",
                ask_on(wire, "main", ", schema: schema")
            )));

            start(&sandbox, true, &[]).report();

            assert_eq!(recorded.argv().unwrap(), ["ok"], "{wire:?} {step}");
            let body = server.exchanges(1)[0].seen.json();
            assert!(
                wire.schema_in(&body).is_some(),
                "{wire:?}: the deepest schema was sent"
            );
        }
    }
}

#[test]
fn a_schema_of_33_levels_ends_the_evaluation_at_the_location_of_the_33rd() {
    for wire in Wire::ALL {
        for (step, path) in [
            (BY_PROPERTY, ".properties.a".repeat(32)),
            (BY_ITEMS, ".items".repeat(32)),
        ] {
            let sandbox = Sandbox::new();
            let recorded = sandbox.recording_agent();
            let server = Server::start(vec![wire.answer("never sent")]);
            settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
            sandbox.personal_rhai(&nested_call(wire, 33, step));

            let error = refused(
                &start(&sandbox, true, &[]),
                "host_request_invalid",
                &recorded,
            );

            let location = error["location"].as_str().unwrap();
            assert!(
                location.starts_with(&format!("ravel::llm(schema{path}) at ")),
                "{wire:?}: {error}"
            );
            assert!(
                error["message"].as_str().unwrap().contains("32 levels"),
                "{wire:?}: {error}"
            );
            assert_eq!(server.requests_started(), 0, "{wire:?}");
        }
    }
}

#[test]
fn a_keyword_outside_the_subset_ends_the_evaluation_whatever_the_wire() {
    for wire in Wire::ALL {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![wire.answer("never sent")]);
        settings_on(wire, &sandbox, &[("main", &server)], &structured(wire));
        sandbox.personal_rhai(&selecting(&format!(
            "    try {{ {}; }} catch (error) {{ }}\n    let args = [];",
            call_on(wire, r#"#{ type: "object", pattern: "^a" }"#, "")
        )));

        let error = refused(
            &start(&sandbox, true, &[]),
            "host_request_invalid",
            &recorded,
        );

        let location = error["location"].as_str().unwrap();
        assert!(
            location.starts_with("ravel::llm(schema.pattern) at "),
            "{wire:?}: {error}"
        );
        assert!(
            error["message"].as_str().unwrap().contains("pattern"),
            "{wire:?}: {error}"
        );
        assert_eq!(server.requests_started(), 0, "{wire:?}");
    }
}

#[test]
fn each_schema_outside_the_subset_names_its_field() {
    // The check runs in the worker, before the wire is known, so one API type
    // stands for all of them.
    for (schema, field) in [
        (
            r#"#{ type: "object", minProperties: 1 }"#,
            "schema.minProperties",
        ),
        (r#"#{ oneOf: [#{ type: "string" }] }"#, "schema.oneOf"),
        (r##"#{ "$ref": "#/defs/a" }"##, "schema.$ref"),
        (
            r#"#{ type: "object", properties: #{ a: #{ format: "date" } } }"#,
            "schema.properties.a.format",
        ),
        (
            r#"#{ type: "array", items: #{ minLength: 1 } }"#,
            "schema.items.minLength",
        ),
        (r#"#{ type: "text" }"#, "schema.type"),
        (r#"#{ type: ["string", "text"] }"#, "schema.type[1]"),
        (r#"#{ type: [] }"#, "schema.type"),
        (r#"#{ properties: [] }"#, "schema.properties"),
        (
            r#"#{ properties: #{ a: "string" } }"#,
            "schema.properties.a",
        ),
        (r#"#{ required: "a" }"#, "schema.required"),
        (r#"#{ required: ["a", 3] }"#, "schema.required[1]"),
        (
            r#"#{ additionalProperties: "no" }"#,
            "schema.additionalProperties",
        ),
        (
            r#"#{ additionalProperties: #{ type: "string" } }"#,
            "schema.additionalProperties",
        ),
        (r#"#{ items: [#{ type: "string" }] }"#, "schema.items"),
        (r#"#{ enum: "a" }"#, "schema.enum"),
        (r#"#{ enum: [["a"]] }"#, "schema.enum[0]"),
        (r#"#{ title: 3 }"#, "schema.title"),
        (r#"#{ description: [] }"#, "schema.description"),
        (r#""object""#, "schema"),
        ("()", "schema"),
        (r#"#{ f: || 1 }"#, "schema.f"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let server = Server::start(vec![Wire::Messages.answer("never sent")]);
        settings(&sandbox, &[("main", &server)], &json!({}));
        sandbox.personal_rhai(&selecting(&format!(
            "    {};\n    let args = [];",
            call_on(Wire::Messages, schema, "")
        )));

        let error = refused(
            &start(&sandbox, true, &[]),
            "host_request_invalid",
            &recorded,
        );

        let location = error["location"].as_str().unwrap();
        assert!(
            location.starts_with(&format!("ravel::llm({field}) at ")),
            "{schema}: {error}"
        );
        assert_eq!(server.requests_started(), 0, "{schema}");
    }
}
