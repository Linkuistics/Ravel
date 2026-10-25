//! The LLM adapter (`docs/specs/policy-evaluation.md`, *LLM calls*, *LLM
//! endpoints and credentials* and *Rig upgrades*).
//!
//! All use of Rig stays in this module. Its interface is one call: an endpoint
//! entry and a checked [`Request`] go in, and a [`Response`] or a classified
//! [`Failure`] comes out. An upgrade of the four Rig pins touches only this
//! module.
//!
//! Ravel builds its own HTTP client (`transport`) and connects the Rig client to
//! it with `connect`. It never uses the shared client or the environment
//! constructors of Rig, so no ambient `*_BASE_URL` or `*_API_KEY` variable takes
//! part. Ravel leaves `on_unsupported` unset, so Rig refuses an option that the
//! wire or the model cannot honour. Ravel sends no tools, no documents and no
//! provider options, and it makes no retry.
//!
//! The Rig API that the adapter uses, all of `rig-core` and `rig-http` 0.45.0,
//! with the file and the line in the pinned source:
//!
//! - `AnthropicConfig::new` (`providers/anthropic/wire.rs:272`) and
//!   `with_base_url` (`:328`), and `connect` (`client/macros.rs:32`): the
//!   Messages wire on the transport of Ravel.
//! - `Anthropic::completion` (`client/anthropic.rs:36`) and `Model::call`
//!   (`driver.rs:353`): the model and its unary call.
//! - `OpenAIConfig::new` (`providers/openai/wire.rs:578`), `with_key` (`:584`),
//!   `with_base_url` (`:695`) and `with_route` (`:740`), and `OpenAI::completion`
//!   (`client/openai.rs:56`): the Responses wire for `openai` with the `OPENAI`
//!   dialect (`providers/openai/wire/dialects.rs:31`), and the Chat Completions
//!   wire for `openai-compatible`.
//! - `Dialect::gateway` (`providers/openai/wire.rs:482`), which gives the
//!   `Quirks::openai()` baseline, and `Auth::OptionalBearer`
//!   (`providers/openai/wire/auth.rs:23`), which sends no `Authorization` header
//!   for an empty key (`:134`).
//! - `CompletionRequest::new` (`completion/request.rs:1480`) and `messages`
//!   (`:1528`), and its public fields `max_tokens`, `temperature` and
//!   `options.reasoning`.
//! - `CompletionResponse` (`completion/request.rs:309`): the fields `choice`,
//!   `usage`, `origin`, `error` and `provider_request_id`, and `finish_reason`
//!   (`:436`).
//! - `ProviderError` (`error.rs:313`) and `is_retryable` (`:478`).
//! - `HttpClientExt` (`rig-http` `http_client/mod.rs:204`), with
//!   `Error::non_success_with_details` (`:99`) and `Error::instance` (`:145`).
//! - `Secret` (`wire/secret.rs:15`), which Rig builds from the credential string.
//!
//! `schema` becomes `CompletionRequest.output_schema` (`completion/request.rs:1230`),
//! a `schemars::Schema` that `rig_core::schemars` re-exports. Each wire sends it
//! through `structured_output_schema` (`providers/openai/mod.rs:71`), which runs
//! the sanitiser (`providers/internal/schema.rs`): every property becomes
//! required, and an object without the keyword gets `additionalProperties:
//! false`. Messages puts the result in `output_config.format`
//! (`providers/anthropic/completion.rs:306`), Responses in `text.format`
//! (`providers/openai/responses_api/mod.rs:511`) and Chat Completions in
//! `response_format.json_schema` (`providers/openai/wire/chat.rs:293`). Chat
//! drops the schema with only a warning when the dialect has no
//! `supports_response_format` (`:293`), so a call with a schema to an endpoint
//! without structured output fails as `unsupported_capability` before Rig sees
//! it, and the dialect of such an endpoint backs that check with
//! `without_response_format` (`providers/openai/wire.rs:445`). The check of the
//! answer is Ravel's own (`schema`), against the schema as the script wrote it.
//!
//! Rig maps `reasoning` for a Chat Completions dialect only for the ones it
//! registers, and it refuses the option for any other (`providers/openai/options.rs:523`,
//! with `refuse_all` at `providers/internal/mod.rs:42`). The `openai-compatible`
//! dialect is not registered, so `reasoning` on that type fails as
//! `unsupported_parameter`. The Responses wire reads a `max_output_tokens`
//! incompletion as the finish `Length` and a `content_filter` one as
//! `ContentFilter` (`providers/openai/responses_api/streaming.rs:276`).
//!
//! Rig reads a Messages refusal as the finish `ContentFilter` plus an error
//! text (`providers/anthropic/streaming.rs:133`). The adapter reads the finish
//! first, so a refusal is `content_filtered`. Rig checks the options of a
//! request against its catalog only when they are not all default
//! (`catalog/spec.rs:509`), so `reasoning` turns the check on.
//!
//! The credential is a Rig `Secret` from the construction of the config on. No
//! text that this module returns holds it: [`redact`] removes it from every
//! string of a response and of a failure, whatever the endpoint echoes.

pub mod schema;
mod shape;
mod transport;

/// The most bytes of an answer text. A longer text fails the call as
/// `output_limit`.
pub const MAX_TEXT_BYTES: usize = 1024 * 1024;

use std::ffi::OsString;
use std::time::{Duration, Instant};

use rig_core::completion::{
    AssistantContent, CompletionRequest, Effort, FinishReason as RigFinish, Message as RigMessage,
    Reasoning as RigReasoning,
};
use rig_core::error::ProviderError;
use rig_core::http_client::Error as HttpError;
use rig_core::providers::anthropic::AnthropicConfig;
use rig_core::providers::openai::wire::{Auth, Dialect};
use rig_core::providers::openai::{OpenAIConfig, Route};

use crate::endpoints::{ApiType, Endpoint};
use schema::Schema;
use transport::{Fault, Transport};

/// The role of a message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// One message of a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// How much the model reasons. The words are the spec's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reasoning {
    Off,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl Reasoning {
    pub fn parse(word: &str) -> Option<Reasoning> {
        Some(match word {
            "off" => Reasoning::Off,
            "minimal" => Reasoning::Minimal,
            "low" => Reasoning::Low,
            "medium" => Reasoning::Medium,
            "high" => Reasoning::High,
            "xhigh" => Reasoning::XHigh,
            "max" => Reasoning::Max,
            _ => return None,
        })
    }

    fn to_rig(self) -> RigReasoning {
        match self {
            Reasoning::Off => RigReasoning::Off,
            Reasoning::Minimal => Effort::Minimal.into(),
            Reasoning::Low => Effort::Low.into(),
            Reasoning::Medium => Effort::Medium.into(),
            Reasoning::High => Effort::High.into(),
            Reasoning::XHigh => Effort::XHigh.into(),
            Reasoning::Max => Effort::Max.into(),
        }
    }
}

/// A request that the caller already checked: one or more messages whose last
/// role is `user`, and every bound in range.
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u64,
    pub temperature: Option<f64>,
    pub reasoning: Option<Reasoning>,
    /// The shape that the answer must have. The adapter sends it natively and
    /// checks the answer against it.
    pub schema: Option<Schema>,
}

/// Why the model stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finish {
    Stop,
    Length,
    Other,
}

impl Finish {
    pub fn name(self) -> &'static str {
        match self {
            Finish::Stop => "stop",
            Finish::Length => "length",
            Finish::Other => "other",
        }
    }
}

/// The tokens of one call. A counter the endpoint did not report is `None`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

/// A successful call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub text: String,
    /// The text as a value that the request's schema accepted, when it had one.
    pub json: Option<serde_json::Value>,
    pub finish: Finish,
    pub usage: Usage,
    pub reported_model: Option<String>,
    pub response_id: Option<String>,
    pub request_id: Option<String>,
    pub elapsed: Duration,
}

/// The failure codes of the spec that a call can end in. The adapter produces
/// most of them. The host function produces `UnknownEndpoint` and
/// `ModelNotOffered`, which fail before the adapter is asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    UnknownEndpoint,
    EndpointUnavailable,
    ModelNotOffered,
    UnsupportedCapability,
    UnsupportedParameter,
    Authentication,
    RateLimited,
    ProviderError,
    Network,
    Timeout,
    MalformedResponse,
    ContentFiltered,
    OutputLimit,
}

impl Code {
    /// Whether a call that failed with this code had left the Ravel CLI. A call
    /// that fails before Ravel sends it uses no budget.
    ///
    /// The codes that say no are the ones decided from the settings, from the
    /// environment or from the request alone: Ravel checks each before it opens
    /// a connection. `Network` says yes even for a connect that failed, because
    /// the code cannot tell a request that never left from one cut mid-flight.
    pub fn was_sent(self) -> bool {
        !matches!(
            self,
            Code::UnknownEndpoint
                | Code::EndpointUnavailable
                | Code::ModelNotOffered
                | Code::UnsupportedCapability
                | Code::UnsupportedParameter
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            Code::UnknownEndpoint => "unknown_endpoint",
            Code::EndpointUnavailable => "endpoint_unavailable",
            Code::ModelNotOffered => "model_not_offered",
            Code::UnsupportedCapability => "unsupported_capability",
            Code::UnsupportedParameter => "unsupported_parameter",
            Code::Authentication => "authentication",
            Code::RateLimited => "rate_limited",
            Code::ProviderError => "provider_error",
            Code::Network => "network",
            Code::Timeout => "timeout",
            Code::MalformedResponse => "malformed_response",
            Code::ContentFiltered => "content_filtered",
            Code::OutputLimit => "output_limit",
        }
    }
}

/// A failed call, as the script sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub code: Code,
    pub message: String,
    pub retryable: bool,
    pub http_status: Option<u16>,
    pub retry_after_ms: Option<u64>,
    pub request_id: Option<String>,
    pub elapsed: Duration,
}

impl Failure {
    pub fn new(code: Code, retryable: bool, message: impl Into<String>) -> Self {
        Failure {
            code,
            message: message.into(),
            retryable,
            http_status: None,
            retry_after_ms: None,
            request_id: None,
            elapsed: Duration::ZERO,
        }
    }
}

/// The adapter: one HTTP client for the calls of an evaluation.
#[derive(Clone, Debug)]
pub struct Adapter {
    transport: Transport,
}

impl Adapter {
    /// Build the HTTP client. The error says why the platform gave no TLS
    /// configuration.
    pub fn new() -> Result<Self, String> {
        Ok(Adapter {
            transport: Transport::new()?,
        })
    }

    /// Send `request` through `endpoint` and wait at most `timeout` for the
    /// whole answer. `environment` looks a variable up in the caller's
    /// environment, and the adapter reads the credential through it at the time
    /// of the call.
    pub async fn call(
        &self,
        endpoint: &Endpoint,
        environment: impl Fn(&str) -> Option<OsString>,
        request: &Request,
        timeout: Duration,
    ) -> Result<Response, Failure> {
        let began = Instant::now();
        let finish = |mut failure: Failure| {
            failure.elapsed = began.elapsed();
            failure
        };
        let credential = credential(endpoint, environment).map_err(finish)?;
        // Rig drops a schema with only a warning on a dialect without
        // structured output, so the request never reaches it.
        if request.schema.is_some() && !endpoint.structured_output() {
            return Err(finish(Failure::new(
                Code::UnsupportedCapability,
                false,
                "the endpoint takes no schema: set structuredOutput to true in its settings                  entry if the server supports structured output",
            )));
        }
        let secret = credential.clone();
        let sent = self.send(endpoint, credential, request);
        let outcome = match tokio::time::timeout(timeout, sent).await {
            Ok(outcome) => outcome,
            Err(_) => Err(Failure::new(
                Code::Timeout,
                true,
                format!(
                    "the call reached its deadline of {} ms",
                    timeout.as_millis()
                ),
            )),
        };
        match outcome {
            Ok(mut response) => {
                // An endpoint chooses every string of its answer, so any of
                // them can echo the credential.
                response.text = redact(&response.text, &secret);
                redact_option(&mut response.reported_model, &secret);
                redact_option(&mut response.response_id, &secret);
                redact_option(&mut response.request_id, &secret);
                response.elapsed = began.elapsed();
                // The script sees the text after the redaction, which can make
                // it longer, so the bound applies to that text.
                if response.text.len() > MAX_TEXT_BYTES {
                    let mut failure = Failure::new(
                        Code::OutputLimit,
                        false,
                        format!(
                            "the answer text holds {} bytes, over the limit of {MAX_TEXT_BYTES} bytes",
                            response.text.len()
                        ),
                    );
                    failure.request_id = response.request_id;
                    failure.elapsed = response.elapsed;
                    return Err(failure);
                }
                // The check reads the redacted text, so the value cannot carry
                // the credential either.
                if let Some(schema) = &request.schema {
                    match schema.check(&response.text) {
                        Ok(value) => response.json = Some(value),
                        Err(why) => {
                            let mut failure = Failure::new(Code::MalformedResponse, false, why);
                            failure.request_id = response.request_id;
                            failure.elapsed = response.elapsed;
                            return Err(failure);
                        }
                    }
                }
                Ok(response)
            }
            Err(mut failure) => {
                failure.message = redact(&failure.message, &secret);
                redact_option(&mut failure.request_id, &secret);
                Err(finish(failure))
            }
        }
    }

    async fn send(
        &self,
        endpoint: &Endpoint,
        credential: String,
        request: &Request,
    ) -> Result<Response, Failure> {
        let outcome = match endpoint.api {
            ApiType::Anthropic => {
                let mut config = AnthropicConfig::new(credential);
                if let Some(base) = &endpoint.base_url {
                    config = config.with_base_url(base);
                }
                let client = config.connect(self.transport.clone());
                client
                    .completion(request.model.clone())
                    .call(rig_request(request))
                    .await
            }
            ApiType::OpenAi | ApiType::OpenAiCompatible => {
                let config = openai_config(endpoint, credential)?;
                let client = config.connect(self.transport.clone());
                client
                    .completion(request.model.clone())
                    .call(rig_request(request))
                    .await
            }
        };
        match outcome {
            Ok(response) => answer(response),
            Err(error) => Err(classify(&error)),
        }
    }
}

/// The dialect of an `openai-compatible` endpoint: the baseline quirks of Rig
/// for a server that speaks Chat Completions, and no more.
///
/// The dialect is not one that Rig registers, so Rig knows no mapping of
/// `reasoning` onto this server, and it refuses the option as unsupported. A
/// server that wants `reasoning_effort` is not one that Ravel can know about.
/// `x-request-id` is the header that OpenAI names, and compatible servers copy
/// it.
const COMPATIBLE: Dialect = Dialect {
    request_id_header: Some("x-request-id"),
    ..Dialect::gateway("openai-compatible", "", "")
};

/// [`COMPATIBLE`] for a server that the owner did not mark as taking structured
/// output: Rig then maps no `output_schema` onto `response_format`. The adapter
/// fails such a call before it reaches Rig, so this only backs that check.
const COMPATIBLE_PLAIN: Dialect =
    COMPATIBLE.with_quirks(COMPATIBLE.quirks.without_response_format());

/// The configuration of Rig for an endpoint of an OpenAI API type. `openai` is
/// the OpenAI dialect on the Responses wire. `openai-compatible` is [`COMPATIBLE`]
/// on the Chat Completions wire, and it sends no `Authorization` header when it
/// has no credential.
fn openai_config(endpoint: &Endpoint, credential: String) -> Result<OpenAIConfig, Failure> {
    let config = match endpoint.api {
        ApiType::OpenAiCompatible => {
            let Some(base) = &endpoint.base_url else {
                return Err(Failure::new(
                    Code::ProviderError,
                    false,
                    "an openai-compatible endpoint has no base URL",
                ));
            };
            let dialect = if endpoint.structured_output() {
                &COMPATIBLE
            } else {
                &COMPATIBLE_PLAIN
            };
            let mut config = OpenAIConfig::with_key(dialect, credential)
                .with_base_url(base)
                .with_route(Route::Chat);
            config.auth = Auth::OptionalBearer;
            config
        }
        _ => {
            let config = OpenAIConfig::new(credential).with_route(Route::Responses);
            match &endpoint.base_url {
                Some(base) => config.with_base_url(base),
                None => config,
            }
        }
    };
    Ok(config)
}

/// The credential of `endpoint`, read at the time of the call.
fn credential(
    endpoint: &Endpoint,
    environment: impl Fn(&str) -> Option<OsString>,
) -> Result<String, Failure> {
    let Some(variable) = endpoint.api_key_env.as_deref() else {
        return Ok(String::new());
    };
    match environment(variable).map(OsString::into_string) {
        Some(Ok(value)) if !value.is_empty() => Ok(value),
        Some(Err(_)) => Err(Failure::new(
            Code::EndpointUnavailable,
            false,
            format!("the credential variable {variable} does not hold UTF-8"),
        )),
        _ => Err(Failure::new(
            Code::EndpointUnavailable,
            false,
            format!("the credential variable {variable} is unset or empty"),
        )),
    }
}

/// `request` as the `CompletionRequest` of Rig: the messages become its chat
/// history, the last one as the prompt.
fn rig_request(request: &Request) -> CompletionRequest {
    let rig = |message: &Message| match message.role {
        Role::System => RigMessage::system(message.content.clone()),
        Role::User => RigMessage::user(message.content.clone()),
        Role::Assistant => RigMessage::assistant(message.content.clone()),
    };
    let (prompt, earlier) = request
        .messages
        .split_last()
        .expect("a checked request has a message");
    let mut built = CompletionRequest::new(rig(prompt)).messages(earlier.iter().map(rig));
    built.max_tokens = Some(request.max_tokens);
    built.temperature = request.temperature;
    built.options.reasoning = request.reasoning.map(Reasoning::to_rig);
    built.output_schema = request.schema.as_ref().map(Schema::to_rig);
    built
}

/// The successful answer, or the failure of an answer that is no answer.
fn answer(response: rig_core::completion::CompletionResponse) -> Result<Response, Failure> {
    let request_id = response.provider_request_id.clone();
    let tagged = |mut failure: Failure| {
        failure.request_id.clone_from(&request_id);
        failure
    };
    let finish = response.finish_reason();
    // Anthropic reports a refusal as a content-filter finish and an explanation,
    // so the filter is read before the error.
    if finish == Some(RigFinish::ContentFilter) {
        let why = response
            .error
            .as_deref()
            .map_or_else(String::new, |why| format!(": {why}"));
        return Err(tagged(Failure::new(
            Code::ContentFiltered,
            false,
            format!("the provider ended the turn with its content filter{why}"),
        )));
    }
    if let Some(reason) = &response.error {
        return Err(tagged(Failure::new(
            Code::ProviderError,
            false,
            format!("the provider ended the turn with an error: {reason}"),
        )));
    }
    let text: String = response
        .choice
        .iter()
        .filter_map(|block| match block {
            AssistantContent::Text(text) => Some(text.text.as_str()),
            _ => None,
        })
        .collect();
    if text.is_empty() {
        return Err(tagged(Failure::new(
            Code::MalformedResponse,
            false,
            "the response has no text",
        )));
    }
    Ok(Response {
        text,
        json: None,
        finish: match finish {
            Some(RigFinish::Length) => Finish::Length,
            Some(RigFinish::Stop) | None => Finish::Stop,
            Some(_) => Finish::Other,
        },
        usage: Usage {
            input_tokens: response.usage.input_tokens,
            output_tokens: response.usage.output_tokens,
            total_tokens: response.usage.total_tokens,
        },
        reported_model: response.origin.response_model.clone(),
        response_id: response.origin.response_id.clone(),
        request_id,
        elapsed: Duration::ZERO,
    })
}

/// The class of a failure that Rig reported.
fn classify(error: &ProviderError) -> Failure {
    let mut failure = match error {
        ProviderError::Http(http) => match &**http {
            HttpError::Instance(inner) => match inner.downcast_ref::<Fault>() {
                Some(Fault::Network(cause)) => Failure::new(Code::Network, true, cause.clone()),
                Some(Fault::ShortBody(cause)) => Failure::new(
                    Code::MalformedResponse,
                    true,
                    format!("the response {cause}"),
                ),
                Some(Fault::OutputLimit { .. } | Fault::TooManyValues) => {
                    Failure::new(Code::OutputLimit, false, inner.to_string())
                }
                Some(Fault::Unsupported(_)) | None => {
                    Failure::new(Code::Network, error.is_retryable(), inner.to_string())
                }
            },
            other => Failure::new(Code::Network, error.is_retryable(), other.to_string()),
        },
        ProviderError::ProviderResponse(reply) | ProviderError::InvalidAuthentication(reply) => {
            from_reply(error, reply)
        }
        ProviderError::UnsupportedOption(option) => {
            Failure::new(Code::UnsupportedParameter, false, option.to_string())
        }
        ProviderError::Json(_) | ProviderError::Response(_) => {
            Failure::new(Code::MalformedResponse, false, error.to_string())
        }
        // The transport reports a body that stops short as its own fault, so a
        // body that reaches Rig is whole. Rig reports `Truncated` when that whole
        // body holds no answer of the wire, and a new attempt would get the same.
        ProviderError::Truncated => Failure::new(
            Code::MalformedResponse,
            false,
            "the response body holds no answer",
        ),
        other => Failure::new(Code::ProviderError, false, other.to_string()),
    };
    failure.request_id = error.provider_request_id().map(str::to_owned);
    failure
}

/// The failure of a reply with a status and a body.
fn from_reply(error: &ProviderError, reply: &rig_core::ProviderResponseError) -> Failure {
    let status = reply.status.map(|status| status.as_u16());
    let (code, retryable) = match status {
        Some(401 | 403) => (Code::Authentication, false),
        Some(429) => (Code::RateLimited, true),
        _ => (Code::ProviderError, error.is_retryable()),
    };
    let shown = match status {
        Some(status) => format!("the endpoint answered {status}: {}", reply.body),
        None => format!("the endpoint reported a failure: {}", reply.body),
    };
    let mut failure = Failure::new(code, retryable, shown);
    failure.http_status = status;
    failure.retry_after_ms = reply
        .headers
        .as_ref()
        .and_then(|headers| headers.get("retry-after"))
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|seconds| seconds.saturating_mul(1000));
    failure
}

/// `text` with every occurrence of `secret` removed.
fn redact(text: &str, secret: &str) -> String {
    if secret.is_empty() {
        text.to_owned()
    } else {
        text.replace(secret, "[redacted]")
    }
}

/// `field`, if it holds text, with every occurrence of `secret` removed.
fn redact_option(field: &mut Option<String>, secret: &str) {
    if let Some(text) = field {
        *text = redact(text, secret);
    }
}

#[cfg(test)]
mod tests;
