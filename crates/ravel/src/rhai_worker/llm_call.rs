//! `ravel::llm` on both sides of the bridge (`docs/specs/policy-evaluation.md`,
//! *LLM calls*).
//!
//! The worker checks the request the script built and sends a normalised `host`
//! frame. A malformed request ends the evaluation as `host_request_invalid` with
//! the field and the line, and nothing is sent. The front decodes the frame into
//! an [`llm::Request`](crate::llm::Request), runs the call, and answers with the
//! whole response map, so the worker only converts it. A failed call is data in
//! that map. It is never an `error` reply.
//!
//! The frame carries every field the front needs, with the default applied:
//!
//! | Field | Type |
//! |---|---|
//! | `endpoint`, `model` | String |
//! | `messages` | Array of `{ role, content }` |
//! | `maxTokens` | Integer, 1 or more |
//! | `temperature`, `reasoning`, `timeoutMs` | Number, String, Integer, or null when the script gave none |
//! | `schema` | A map of the schema subset, or null when the script gave none |

use std::time::Duration;

use rhai::{Dynamic, EvalAltResult, Map, NativeCallContext};
use serde_json::{Value, json};

use super::bridge::{Bridge, Outcome};
use super::convert::{from_json, to_json};
use crate::llm::schema::Schema;
use crate::llm::{Failure, Message, Reasoning, Request, Response, Role, Usage};

/// The `maxTokens` of a request that names none.
const DEFAULT_MAX_TOKENS: i64 = 1024;

/// The fields of the request, in the order the spec lists them.
const FIELDS: [&str; 8] = [
    "endpoint",
    "model",
    "messages",
    "maxTokens",
    "temperature",
    "reasoning",
    "schema",
    "timeoutMs",
];

const ROLES: [(&str, Role); 3] = [
    ("system", Role::System),
    ("user", Role::User),
    ("assistant", Role::Assistant),
];

/// The worker's half: check `given` and return the `host` frame to send.
pub(super) fn frame(
    bridge: &Bridge,
    context: &NativeCallContext,
    given: &Dynamic,
) -> Outcome<Value> {
    let invalid = |field: &str, reason: &str| bridge.invalid(context, "llm", field, reason);
    let Some(map) = given.clone().try_cast::<Map>() else {
        return Err(invalid("request", "it must be a map"));
    };
    if let Some(unknown) = map.keys().find(|key| !FIELDS.contains(&key.as_str())) {
        return Err(invalid(unknown, "it is not a field of the request"));
    }
    let required = |field: &str| {
        map.get(field)
            .ok_or_else(|| invalid(field, "it is required"))
    };
    let text = |field: &str, value: &Dynamic| {
        value
            .clone()
            .into_immutable_string()
            .ok()
            .filter(|text| !text.is_empty())
            .ok_or_else(|| invalid(field, "it must be a nonempty string"))
    };
    let whole = |field: &str, value: &Dynamic| {
        value
            .as_int()
            .ok()
            .filter(|&number| number >= 1)
            .ok_or_else(|| invalid(field, "it must be a positive integer"))
    };

    let endpoint = text("endpoint", required("endpoint")?)?;
    let model = text("model", required("model")?)?;
    let messages = messages(required("messages")?, &invalid)?;
    let max_tokens = match map.get("maxTokens") {
        None => DEFAULT_MAX_TOKENS,
        Some(value) => whole("maxTokens", value)?,
    };
    let temperature = match map.get("temperature") {
        None => Value::Null,
        Some(value) => {
            let number = value
                .as_float()
                .ok()
                .filter(|number| (0.0..=2.0).contains(number))
                .ok_or_else(|| invalid("temperature", "it must be a float from 0.0 to 2.0"))?;
            json!(number)
        }
    };
    let reasoning = match map.get("reasoning") {
        None => Value::Null,
        Some(value) => {
            let word = value
                .clone()
                .into_immutable_string()
                .ok()
                .filter(|word| Reasoning::parse(word).is_some())
                .ok_or_else(|| {
                    invalid(
                        "reasoning",
                        "it must be off, minimal, low, medium, high, xhigh or max",
                    )
                })?;
            json!(word.as_str())
        }
    };
    let schema = match map.get("schema") {
        None => Value::Null,
        Some(value) => {
            let json = to_json(value, "schema")
                .map_err(|refused| invalid(&refused.path, &refused.reason))?;
            Schema::parse(&json, "schema").map_err(|why| invalid(&why.path, &why.reason))?;
            json
        }
    };
    let timeout = match map.get("timeoutMs") {
        None => Value::Null,
        Some(value) => json!(whole("timeoutMs", value)?),
    };

    let mut frame = bridge.frame(context, "llm");
    frame["endpoint"] = json!(endpoint.as_str());
    frame["model"] = json!(model.as_str());
    frame["messages"] = Value::Array(messages);
    frame["maxTokens"] = json!(max_tokens);
    frame["temperature"] = temperature;
    frame["reasoning"] = reasoning;
    frame["schema"] = schema;
    frame["timeoutMs"] = timeout;

    // The front reads no host frame past the message bound, and a request the
    // script assembled from large reads can pass it.
    let size = serde_json::to_vec(&frame).map_or(u64::MAX, |bytes| bytes.len() as u64);
    let bound = bridge.bounds().message_bytes;
    if size > bound {
        return Err(invalid(
            "request",
            &format!(
                "it encodes to {size} bytes, over the message bound of {bound} bytes; send \
                 less text in one call"
            ),
        ));
    }
    Ok(frame)
}

/// The messages of the request: one or more `#{ role, content }` maps, the last
/// of them a `user` message.
fn messages(
    value: &Dynamic,
    invalid: &impl Fn(&str, &str) -> Box<EvalAltResult>,
) -> Outcome<Vec<Value>> {
    let Ok(items) = value.clone().into_array() else {
        return Err(invalid("messages", "it must be an array"));
    };
    if items.is_empty() {
        return Err(invalid("messages", "it must hold at least one message"));
    }
    let mut checked = Vec::with_capacity(items.len());
    let mut last = None;
    for (index, item) in items.iter().enumerate() {
        let at = |field: &str| format!("messages[{index}]{field}");
        let Some(map) = item.clone().try_cast::<Map>() else {
            return Err(invalid(&at(""), "it must be a map of role and content"));
        };
        if let Some(unknown) = map
            .keys()
            .find(|key| !matches!(key.as_str(), "role" | "content"))
        {
            return Err(invalid(
                &at(&format!(".{unknown}")),
                "it is not a field of a message",
            ));
        }
        let role = map
            .get("role")
            .and_then(|role| role.clone().into_immutable_string().ok())
            .and_then(|role| ROLES.iter().find(|(name, _)| *name == role.as_str()))
            .ok_or_else(|| {
                invalid(
                    &at(".role"),
                    "it is required, and it must be system, user or assistant",
                )
            })?;
        let content = map
            .get("content")
            .and_then(|content| content.clone().into_immutable_string().ok())
            .ok_or_else(|| invalid(&at(".content"), "it is required, and it must be a string"))?;
        last = Some((index, role.1));
        checked.push(json!({ "role": role.0, "content": content.as_str() }));
    }
    match last {
        Some((_, Role::User)) => Ok(checked),
        Some((index, _)) => Err(invalid(
            &format!("messages[{index}].role"),
            "the last message must have the role user",
        )),
        None => unreachable!("the messages are not empty"),
    }
}

/// The worker's half: the response map the script receives, from the `value` of
/// the front's reply.
pub(super) fn response(value: &Value) -> Dynamic {
    from_json(value)
}

/// What the front decoded from an `llm` frame.
pub(super) struct Call {
    pub endpoint: String,
    pub request: Request,
    /// The call's own deadline, before it is lowered to the time that remains.
    pub timeout: Option<Duration>,
}

/// The front's half: decode a frame that the worker checked. An error is a frame
/// that no worker of this build sends.
pub(super) fn decode(frame: &Value) -> Result<Call, String> {
    let text = |field: &str| {
        frame
            .get(field)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| format!("an llm frame has no {field}"))
    };
    let number = |field: &str| match frame.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .filter(|&number| number >= 1)
            .map(Some)
            .ok_or_else(|| format!("an llm frame has a {field} that is no positive integer")),
    };
    let messages = frame
        .get("messages")
        .and_then(Value::as_array)
        .filter(|messages| !messages.is_empty())
        .ok_or("an llm frame has no messages")?
        .iter()
        .map(|message| {
            let role = message
                .get("role")
                .and_then(Value::as_str)
                .and_then(|word| ROLES.iter().find(|(name, _)| *name == word))
                .ok_or("an llm frame has a message with no known role")?;
            let content = message
                .get("content")
                .and_then(Value::as_str)
                .ok_or("an llm frame has a message with no content")?;
            Ok::<_, String>(Message {
                role: role.1,
                content: content.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if messages.last().map(|message| message.role) != Some(Role::User) {
        return Err("an llm frame does not end with a user message".to_owned());
    }
    let temperature = match frame.get("temperature") {
        None | Some(Value::Null) => None,
        Some(value) => Some(
            value
                .as_f64()
                .filter(|number| (0.0..=2.0).contains(number))
                .ok_or("an llm frame has a temperature outside 0.0 to 2.0")?,
        ),
    };
    let reasoning = match frame.get("reasoning") {
        None | Some(Value::Null) => None,
        Some(value) => Some(
            value
                .as_str()
                .and_then(Reasoning::parse)
                .ok_or("an llm frame has an unknown reasoning")?,
        ),
    };
    let schema = match frame.get("schema") {
        None | Some(Value::Null) => None,
        Some(value) => Some(Schema::parse(value, "schema").map_err(|why| {
            format!(
                "an llm frame has a schema outside the subset: {}: {}",
                why.path, why.reason
            )
        })?),
    };
    Ok(Call {
        endpoint: text("endpoint")?,
        request: Request {
            model: text("model")?,
            messages,
            max_tokens: number("maxTokens")?.ok_or("an llm frame has no maxTokens")?,
            temperature,
            reasoning,
            schema,
        },
        timeout: number("timeoutMs")?.map(Duration::from_millis),
    })
}

/// The time one call may take: its own `timeoutMs`, lowered to the time that
/// remains of the whole evaluation.
pub(super) fn deadline_of(own: Option<Duration>, remaining: Duration) -> Duration {
    own.map_or(remaining, |own| own.min(remaining))
}

fn usage(usage: Usage) -> Value {
    json!({
        "inputTokens": usage.input_tokens,
        "outputTokens": usage.output_tokens,
        "totalTokens": usage.total_tokens,
    })
}

fn millis(elapsed: Duration) -> u64 {
    u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
}

/// The response map of the spec for `outcome`, and the entry the notice keeps.
pub(super) fn answer(call: &Call, outcome: &Result<Response, Failure>) -> (Value, LlmCall) {
    let (endpoint, model) = (&call.endpoint, &call.request.model);
    match outcome {
        Ok(done) => (
            json!({
                "status": "ok", "endpoint": endpoint, "model": model,
                "text": done.text, "json": done.json, "finishReason": done.finish.name(),
                "usage": usage(done.usage), "reportedModel": done.reported_model,
                "responseId": done.response_id, "requestId": done.request_id,
                "elapsedMs": millis(done.elapsed), "error": null,
            }),
            LlmCall {
                endpoint: endpoint.clone(),
                model: model.clone(),
                status: "ok",
                error_code: None,
                usage: Some(done.usage),
                elapsed_ms: millis(done.elapsed),
            },
        ),
        Err(failed) => (
            json!({
                "status": "failed", "endpoint": endpoint, "model": model,
                "text": null, "json": null, "finishReason": null, "usage": null,
                "reportedModel": null, "responseId": null, "requestId": failed.request_id,
                "elapsedMs": millis(failed.elapsed),
                "error": {
                    "code": failed.code.name(), "message": failed.message,
                    "retryable": failed.retryable, "httpStatus": failed.http_status,
                    "retryAfterMs": failed.retry_after_ms,
                },
            }),
            LlmCall {
                endpoint: endpoint.clone(),
                model: model.clone(),
                status: "failed",
                error_code: Some(failed.code.name()),
                usage: None,
                elapsed_ms: millis(failed.elapsed),
            },
        ),
    }
}

/// One call of the evaluation as the notice lists it. It holds no message and
/// no answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmCall {
    pub endpoint: String,
    pub model: String,
    /// `ok` or `failed`.
    pub status: &'static str,
    pub error_code: Option<&'static str>,
    pub usage: Option<Usage>,
    pub elapsed_ms: u64,
}

impl LlmCall {
    /// The entry of the notice: no message and no answer.
    pub fn to_json(&self) -> Value {
        json!({
            "endpoint": self.endpoint,
            "model": self.model,
            "status": self.status,
            "errorCode": self.error_code,
            "usage": self.usage.map(usage),
            "elapsedMs": self.elapsed_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::{Code, Finish};

    fn frame() -> Value {
        json!({
            "type": "host", "call": "llm", "endpoint": "main", "model": "m",
            "messages": [{ "role": "system", "content": "s" }, { "role": "user", "content": "u" }],
            "maxTokens": 64, "temperature": 0.5, "reasoning": "low", "timeoutMs": 250,
        })
    }

    #[test]
    fn a_frame_decodes_into_the_request_of_the_adapter() {
        let call = decode(&frame()).unwrap();
        assert_eq!(call.endpoint, "main");
        assert_eq!(call.timeout, Some(Duration::from_millis(250)));
        assert_eq!(call.request.max_tokens, 64);
        assert_eq!(call.request.temperature, Some(0.5));
        assert_eq!(call.request.reasoning, Some(Reasoning::Low));
        assert_eq!(call.request.messages.len(), 2);
        assert_eq!(call.request.messages[0].role, Role::System);
    }

    #[test]
    fn a_frame_that_no_worker_sends_does_not_decode() {
        for (field, value) in [
            ("endpoint", json!(3)),
            ("model", json!(null)),
            ("messages", json!([])),
            (
                "messages",
                json!([{ "role": "assistant", "content": "last" }]),
            ),
            ("messages", json!([{ "role": "tool", "content": "x" }])),
            ("maxTokens", json!(0)),
            ("maxTokens", json!(null)),
            ("temperature", json!(2.5)),
            ("reasoning", json!("deep")),
            ("timeoutMs", json!("soon")),
        ] {
            let mut broken = frame();
            broken[field] = value.clone();
            assert!(decode(&broken).is_err(), "{field} = {value}");
        }
    }

    #[test]
    fn an_own_deadline_is_lowered_to_the_time_that_remains() {
        let second = Duration::from_secs(1);
        assert_eq!(deadline_of(None, second), second);
        assert_eq!(deadline_of(Some(second * 5), second), second);
        assert_eq!(deadline_of(Some(second / 2), second), second / 2);
        assert_eq!(deadline_of(Some(second), Duration::ZERO), Duration::ZERO);
    }

    #[test]
    fn a_failure_is_a_failed_map_with_its_error_and_a_notice_entry_without_text() {
        let call = decode(&frame()).unwrap();
        let mut failure = Failure::new(Code::RateLimited, true, "the endpoint answered 429: slow");
        failure.http_status = Some(429);
        failure.retry_after_ms = Some(2000);
        failure.elapsed = Duration::from_millis(12);
        let (map, record) = answer(&call, &Err(failure));
        assert_eq!(map["status"], "failed");
        assert_eq!(map["text"], Value::Null);
        assert_eq!(map["error"]["code"], "rate_limited");
        assert_eq!(map["error"]["retryAfterMs"], 2000);
        assert_eq!(map["elapsedMs"], 12);
        assert_eq!(
            record.to_json(),
            json!({
                "endpoint": "main", "model": "m", "status": "failed",
                "errorCode": "rate_limited", "usage": null, "elapsedMs": 12,
            })
        );
    }

    #[test]
    fn an_answer_is_an_ok_map_and_a_notice_entry_with_usage_but_no_text() {
        let call = decode(&frame()).unwrap();
        let done = Response {
            text: "secret answer".to_owned(),
            json: None,
            finish: Finish::Stop,
            usage: Usage {
                input_tokens: Some(3),
                output_tokens: None,
                total_tokens: None,
            },
            reported_model: Some("reported".to_owned()),
            response_id: None,
            request_id: Some("req".to_owned()),
            elapsed: Duration::from_millis(5),
        };
        let (map, record) = answer(&call, &Ok(done));
        assert_eq!(map["text"], "secret answer");
        assert_eq!(map["finishReason"], "stop");
        assert_eq!(map["usage"]["inputTokens"], 3);
        assert_eq!(map["usage"]["outputTokens"], Value::Null);
        assert_eq!(map["error"], Value::Null);
        let notice = record.to_json().to_string();
        assert!(!notice.contains("secret answer"), "{notice}");
    }
}
