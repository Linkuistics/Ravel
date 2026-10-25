//! The LLM endpoints and harness entries of the owner settings
//! (`docs/specs/policy-evaluation.md`, *LLM endpoints and credentials* and
//! *Harness entries*).
//!
//! The owner names them outside the script, so a policy can pick an endpoint
//! but never a URL or a credential. This module checks the shape of one entry
//! and holds what the checked entry says. It makes no network call and reads no
//! environment variable: `ravel::endpoints()` reports availability from the
//! front, at the time of the call.

use std::collections::BTreeMap;

use serde_json::{Map, Value};
use url::{Host, Url};

use crate::environment;

/// A name has 1 to 64 lowercase letters, digits and dashes, and its first
/// character is a letter or a digit. Endpoints and harness entries share it.
const NAME_MAX: usize = 64;

/// The owner's endpoints, by name.
pub type Endpoints = BTreeMap<String, Endpoint>;

/// The owner's harness entries, by name.
pub type Harnesses = BTreeMap<String, Harness>;

/// What is wrong with one entry of the settings, and where. The caller turns it
/// into the refusal and names the file.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    /// A dotted path such as `endpoints.main.baseUrl`.
    pub location: String,
    pub what: String,
}

impl Problem {
    fn new(location: impl Into<String>, what: impl Into<String>) -> Self {
        Problem {
            location: location.into(),
            what: what.into(),
        }
    }
}

/// The API types, each with its wire in the spec table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiType {
    Anthropic,
    OpenAi,
    OpenAiCompatible,
}

impl ApiType {
    pub fn name(self) -> &'static str {
        match self {
            ApiType::Anthropic => "anthropic",
            ApiType::OpenAi => "openai",
            ApiType::OpenAiCompatible => "openai-compatible",
        }
    }

    fn parse(name: &str) -> Option<ApiType> {
        [
            ApiType::Anthropic,
            ApiType::OpenAi,
            ApiType::OpenAiCompatible,
        ]
        .into_iter()
        .find(|api| api.name() == name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub api: ApiType,
    /// The variable that holds the credential. Only `openai-compatible` may
    /// have none.
    pub api_key_env: Option<String>,
    pub base_url: Option<String>,
    structured_output: bool,
    /// The models a call may name. Empty means any.
    pub models: Vec<String>,
}

impl Endpoint {
    /// Whether the endpoint takes a `schema`. `anthropic` and `openai` always
    /// do, and `openai-compatible` does when the owner says so.
    pub fn structured_output(&self) -> bool {
        match self.api {
            ApiType::Anthropic | ApiType::OpenAi => true,
            ApiType::OpenAiCompatible => self.structured_output,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Harness {
    pub program: String,
    pub capabilities: Vec<String>,
}

/// The name of the endpoint that uses `variable` as its credential, if one does.
pub fn credential_user<'a>(endpoints: &'a Endpoints, variable: &str) -> Option<&'a str> {
    endpoints
        .iter()
        .find(|(_, endpoint)| endpoint.api_key_env.as_deref() == Some(variable))
        .map(|(name, _)| name.as_str())
}

/// Every endpoint's credential variable.
pub fn credentials(endpoints: &Endpoints) -> Vec<String> {
    endpoints
        .values()
        .filter_map(|endpoint| endpoint.api_key_env.clone())
        .collect()
}

/// The map under `key` of the settings, entry by entry.
pub fn parse_endpoints(value: &Value) -> Result<Endpoints, Problem> {
    let entries = object(value, "endpoints")?;
    entries
        .iter()
        .map(|(name, entry)| Ok((name.clone(), endpoint(name, entry)?)))
        .collect()
}

pub fn parse_harnesses(value: &Value) -> Result<Harnesses, Problem> {
    let entries = object(value, "harnesses")?;
    entries
        .iter()
        .map(|(name, entry)| Ok((name.clone(), harness(name, entry)?)))
        .collect()
}

fn object<'a>(value: &'a Value, location: &str) -> Result<&'a Map<String, Value>, Problem> {
    value
        .as_object()
        .ok_or_else(|| Problem::new(location, format!("holds {value}, which is not an object")))
}

/// Why `name` is no endpoint or harness name, if it is not.
fn name_problem(name: &str, location: &str) -> Option<Problem> {
    let first_ok = name
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase() || first.is_ascii_digit());
    let rest_ok = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    (name.len() > NAME_MAX || !first_ok || !rest_ok).then(|| {
        Problem::new(
            location,
            format!(
                "holds the name {name:?}, which is not 1 to {NAME_MAX} lowercase letters, digits \
                 and dashes that begin with a letter or a digit"
            ),
        )
    })
}

/// An entry's fields, refusing a field that is not in `known`.
fn fields<'a>(
    entry: &'a Value,
    location: &str,
    known: &[&str],
) -> Result<&'a Map<String, Value>, Problem> {
    let map = object(entry, location)?;
    match map.keys().find(|key| !known.contains(&key.as_str())) {
        Some(key) => Err(Problem::new(
            format!("{location}.{key}"),
            format!(
                "is an unknown field, and {location} holds only {}",
                known.join(", ")
            ),
        )),
        None => Ok(map),
    }
}

fn string<'a>(
    map: &'a Map<String, Value>,
    location: &str,
    key: &str,
) -> Result<Option<&'a str>, Problem> {
    match map.get(key) {
        None => Ok(None),
        Some(Value::String(text)) if !text.is_empty() && !text.contains('\0') => {
            Ok(Some(text.as_str()))
        }
        Some(other) => Err(Problem::new(
            format!("{location}.{key}"),
            format!("holds {other}, which is not a nonempty string"),
        )),
    }
}

/// An array of strings, each nonempty. Absent is empty.
fn strings(map: &Map<String, Value>, location: &str, key: &str) -> Result<Vec<String>, Problem> {
    let here = format!("{location}.{key}");
    let Some(value) = map.get(key) else {
        return Ok(Vec::new());
    };
    let items = value
        .as_array()
        .ok_or_else(|| Problem::new(&here, format!("holds {value}, which is not an array")))?;
    items
        .iter()
        .map(|item| match item {
            Value::String(text) if !text.is_empty() => Ok(text.clone()),
            other => Err(Problem::new(
                &here,
                format!("holds {other}, which is not a nonempty string"),
            )),
        })
        .collect()
}

fn endpoint(name: &str, entry: &Value) -> Result<Endpoint, Problem> {
    let location = format!("endpoints.{name}");
    if let Some(problem) = name_problem(name, "endpoints") {
        return Err(problem);
    }
    let map = fields(
        entry,
        &location,
        &["type", "apiKeyEnv", "baseUrl", "structuredOutput", "models"],
    )?;
    let api = string(map, &location, "type")?
        .and_then(ApiType::parse)
        .ok_or_else(|| {
            Problem::new(
                format!("{location}.type"),
                "must be \"anthropic\", \"openai\" or \"openai-compatible\"",
            )
        })?;
    let api_key_env = string(map, &location, "apiKeyEnv")?.map(str::to_owned);
    match (&api_key_env, api) {
        (None, ApiType::Anthropic | ApiType::OpenAi) => {
            return Err(Problem::new(
                format!("{location}.apiKeyEnv"),
                format!("is required for the type {}", api.name()),
            ));
        }
        (Some(variable), _) => {
            if let Some(problem) = credential_problem(variable, &format!("{location}.apiKeyEnv")) {
                return Err(problem);
            }
        }
        (None, ApiType::OpenAiCompatible) => {}
    }
    let base_url = string(map, &location, "baseUrl")?.map(str::to_owned);
    match (&base_url, api) {
        (None, ApiType::OpenAiCompatible) => {
            return Err(Problem::new(
                format!("{location}.baseUrl"),
                "is required for the type openai-compatible",
            ));
        }
        (Some(url), _) => {
            if let Some(why) = url_problem(url) {
                return Err(Problem::new(format!("{location}.baseUrl"), why));
            }
        }
        (None, _) => {}
    }
    let structured_output = match map.get("structuredOutput") {
        None => false,
        Some(Value::Bool(flag)) if api == ApiType::OpenAiCompatible => *flag,
        Some(Value::Bool(_)) => {
            return Err(Problem::new(
                format!("{location}.structuredOutput"),
                format!(
                    "applies only to the type openai-compatible, and this is {}",
                    api.name()
                ),
            ));
        }
        Some(other) => {
            return Err(Problem::new(
                format!("{location}.structuredOutput"),
                format!("holds {other}, which is not a boolean"),
            ));
        }
    };
    Ok(Endpoint {
        api,
        api_key_env,
        base_url,
        structured_output,
        models: strings(map, &location, "models")?,
    })
}

/// Why `variable` cannot be the credential variable of an endpoint, if it
/// cannot. The credential is never also a variable that `ravel::env` reads
/// without a grant.
fn credential_problem(variable: &str, location: &str) -> Option<Problem> {
    if variable.contains('=') {
        return Some(Problem::new(
            location,
            format!("holds {variable:?}, which contains `=`: it takes a variable's name"),
        ));
    }
    environment::is_base(variable.as_bytes()).then(|| {
        Problem::new(
            location,
            format!(
                "names {variable}, which a policy reads with ravel::env without a grant: HOME, \
                 PATH, TMPDIR, LANG and LC_* cannot hold a credential"
            ),
        )
    })
}

/// Why `text` is no base URL, if it is not: `https`, or `http` to a loopback
/// address, with no user information, no query and no fragment.
fn url_problem(text: &str) -> Option<String> {
    let url = match Url::parse(text) {
        Ok(url) => url,
        Err(error) => return Some(format!("holds {text:?}, which is not a URL: {error}")),
    };
    let loopback = match url.host() {
        Some(Host::Domain(domain)) => domain == "localhost",
        Some(Host::Ipv4(address)) => address.is_loopback(),
        Some(Host::Ipv6(address)) => address.is_loopback(),
        None => false,
    };
    if !url.username().is_empty() || url.password().is_some() {
        Some(format!("holds {text:?}, which carries user information"))
    } else if url.query().is_some() || url.fragment().is_some() {
        Some(format!("holds {text:?}, which has a query or a fragment"))
    } else if (url.scheme() == "https" && url.host().is_some())
        || (url.scheme() == "http" && loopback)
    {
        None
    } else {
        Some(format!(
            "holds {text:?}, which must use https, or http to localhost, 127.0.0.0/8 or ::1"
        ))
    }
}

fn harness(name: &str, entry: &Value) -> Result<Harness, Problem> {
    let location = format!("harnesses.{name}");
    if let Some(problem) = name_problem(name, "harnesses") {
        return Err(problem);
    }
    let map = fields(entry, &location, &["program", "capabilities"])?;
    let program = string(map, &location, "program")?
        .ok_or_else(|| Problem::new(format!("{location}.program"), "is required"))?
        .to_owned();
    Ok(Harness {
        program,
        capabilities: strings(map, &location, "capabilities")?,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn endpoints(value: Value) -> Result<Endpoints, Problem> {
        parse_endpoints(&value)
    }

    fn located(value: Value) -> String {
        endpoints(value).unwrap_err().location
    }

    #[test]
    fn a_base_url_is_https_or_http_to_loopback_without_user_information_query_or_fragment() {
        for good in [
            "https://api.example.com",
            "https://api.example.com/v1/",
            "https://api.example.com:8443/v1",
            "http://localhost:11434/v1",
            "http://127.0.0.1:8080",
            "http://127.200.0.9/v1",
            "http://[::1]:8080",
            // The parser normalises these to the address that a client connects to.
            "http://LOCALHOST:8080",
            "http://127.1:8080",
            "http://0x7f.1/v1",
            "https://API.Example.COM",
        ] {
            assert_eq!(url_problem(good), None, "{good}");
        }
        for bad in [
            "http://api.example.com",
            "http://10.0.0.1",
            "http://128.0.0.1",
            "http://[::2]",
            "http://[::ffff:127.0.0.1]",
            "http://localhost.example.com",
            "http://localhost.",
            "http://127.0.0.1.example.com",
            "http://localhost@example.com",
            "http://localhost:80@example.com",
            "http://example.com\\@localhost",
            "http://[::127.0.0.1]",
            "http://0.0.0.0",
            "http://2130706433.example.com",
            "ftp://localhost",
            "https://user@api.example.com",
            "https://user:pass@api.example.com",
            "http://user@localhost",
            "https://api.example.com?x=1",
            "https://api.example.com/v1?",
            "https://api.example.com/v1#frag",
            "https://api.example.com/#",
            "api.example.com",
            "https://",
            "/v1",
            "",
        ] {
            assert!(url_problem(bad).is_some(), "{bad:?} is accepted");
        }
    }

    #[test]
    fn a_name_has_one_to_sixty_four_lowercase_letters_digits_and_dashes_and_does_not_begin_with_a_dash()
     {
        let entry = json!({ "type": "anthropic", "apiKeyEnv": "K" });
        let long = "a".repeat(64);
        for good in ["a", "0", "main-1", "9-lives", long.as_str()] {
            let map = json!({ good: entry.clone() });
            assert!(endpoints(map).is_ok(), "{good}");
        }
        let too_long = "a".repeat(65);
        for bad in ["", "-a", "A", "a_b", "a b", "é", "a.b", too_long.as_str()] {
            let map = json!({ bad: entry.clone() });
            assert_eq!(located(map), "endpoints", "{bad:?}");
        }
    }

    #[test]
    fn each_field_follows_the_spec_table() {
        let parsed = endpoints(json!({
            "claude": { "type": "anthropic", "apiKeyEnv": "A_KEY", "models": ["m1", "m2"] },
            "gpt": { "type": "openai", "apiKeyEnv": "O_KEY", "baseUrl": "https://proxy.example/v1" },
            "local": { "type": "openai-compatible", "baseUrl": "http://localhost:1234/v1",
                       "structuredOutput": true },
        }))
        .unwrap();
        assert_eq!(parsed["claude"].models, ["m1", "m2"]);
        assert!(parsed["claude"].structured_output());
        assert!(parsed["gpt"].structured_output());
        assert_eq!(
            parsed["gpt"].base_url.as_deref(),
            Some("https://proxy.example/v1")
        );
        assert_eq!(parsed["local"].api_key_env, None);
        assert!(parsed["local"].structured_output());
        assert!(parsed["local"].models.is_empty());
        let plain = endpoints(json!({
            "local": { "type": "openai-compatible", "baseUrl": "http://localhost:1/v1" }
        }))
        .unwrap();
        assert!(!plain["local"].structured_output());

        for (value, location) in [
            (json!([]), "endpoints"),
            (json!({ "a": 7 }), "endpoints.a"),
            (json!({ "a": {} }), "endpoints.a.type"),
            (
                json!({ "a": { "type": "gemini", "apiKeyEnv": "K" } }),
                "endpoints.a.type",
            ),
            (json!({ "a": { "type": 7 } }), "endpoints.a.type"),
            (
                json!({ "a": { "type": "anthropic" } }),
                "endpoints.a.apiKeyEnv",
            ),
            (
                json!({ "a": { "type": "openai" } }),
                "endpoints.a.apiKeyEnv",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "" } }),
                "endpoints.a.apiKeyEnv",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "A=B" } }),
                "endpoints.a.apiKeyEnv",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": 7 } }),
                "endpoints.a.apiKeyEnv",
            ),
            (
                json!({ "a": { "type": "openai-compatible" } }),
                "endpoints.a.baseUrl",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "baseUrl": "http://x.example" } }),
                "endpoints.a.baseUrl",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "structuredOutput": true } }),
                "endpoints.a.structuredOutput",
            ),
            (
                json!({ "a": { "type": "openai-compatible", "baseUrl": "https://x.example",
                               "structuredOutput": "yes" } }),
                "endpoints.a.structuredOutput",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "models": "m" } }),
                "endpoints.a.models",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "models": [7] } }),
                "endpoints.a.models",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "models": [""] } }),
                "endpoints.a.models",
            ),
            (
                json!({ "a": { "type": "openai", "apiKeyEnv": "K", "region": "x" } }),
                "endpoints.a.region",
            ),
        ] {
            assert_eq!(located(value.clone()), location, "{value}");
        }
    }

    #[test]
    fn a_credential_variable_cannot_be_a_name_that_env_reads_without_a_grant() {
        for base in [
            "HOME",
            "PATH",
            "TMPDIR",
            "LANG",
            "LC_ALL",
            "LC_MODEL_KEY",
            "LC_",
        ] {
            let map = json!({ "a": { "type": "anthropic", "apiKeyEnv": base } });
            assert_eq!(located(map), "endpoints.a.apiKeyEnv", "{base}");
        }
        for near in ["HOMEDIR", "LC", "LANGUAGE", "XPATH", "MODEL_KEY_LC_"] {
            let map = json!({ "a": { "type": "anthropic", "apiKeyEnv": near } });
            assert!(endpoints(map).is_ok(), "{near}");
        }
    }

    #[test]
    fn a_harness_entry_has_a_program_and_optional_capability_labels() {
        let parsed = parse_harnesses(&json!({
            "claude": { "program": "claude", "capabilities": ["edit", "shell"] },
            "bare": { "program": "/opt/bin/agent" },
        }))
        .unwrap();
        assert_eq!(parsed["claude"].program, "claude");
        assert_eq!(parsed["claude"].capabilities, ["edit", "shell"]);
        assert!(parsed["bare"].capabilities.is_empty());
        for (value, location) in [
            (json!(3), "harnesses"),
            (json!({ "A": { "program": "x" } }), "harnesses"),
            (json!({ "a": "x" }), "harnesses.a"),
            (json!({ "a": {} }), "harnesses.a.program"),
            (json!({ "a": { "program": "" } }), "harnesses.a.program"),
            (json!({ "a": { "program": 4 } }), "harnesses.a.program"),
            (
                json!({ "a": { "program": "x", "capabilities": "edit" } }),
                "harnesses.a.capabilities",
            ),
            (
                json!({ "a": { "program": "x", "capabilities": [1] } }),
                "harnesses.a.capabilities",
            ),
            (
                json!({ "a": { "program": "x", "args": [] } }),
                "harnesses.a.args",
            ),
        ] {
            assert_eq!(
                parse_harnesses(&value).unwrap_err().location,
                location,
                "{value}"
            );
        }
    }
}
