//! Validate a policy snapshot, then judge what its `select` returned
//! (`docs/specs/session-execution.md`, *Policy and the selected
//! command*).
//!
//! The worker hands over the entry's `policy` export as JSON, with any value
//! JSON cannot carry replaced by a `{"$ravel": "<type>"}` marker, and
//! later the value `select` produced, marked the same way. This module is the
//! one validator for both: every refusal names the location it found, in the
//! form `policy.select` or `result.args[2]`, and nothing invalid is ever
//! repaired into something that passes.
//!
//! Only the shape is judged. A selected result is the command to run, and
//! what it holds is the owner's function's: nothing here checks that the
//! prompt is among the arguments, that the program is one the owner listed
//! anywhere, or that a kind has a route.

use serde_json::{Map, Value};

use crate::limits::Limits;
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};
use crate::worker::{Breach, Produced};

/// The policy contract this release evaluates.
pub const SCHEMA_VERSION: u64 = 2;

const TOP_LEVEL: [&str; 4] = ["schemaVersion", "version", "select", "loadContext"];

#[derive(Debug)]
pub struct Policy {
    pub version: String,
    /// Whether it has a `loadContext`, which only the worker can call.
    pub loader: bool,
}

/// The selected command: what `select` returned when it did not refuse. The
/// program and arguments are run as they are, each argument one whole word,
/// and the labels are the owner's for what they run.
#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub provider: String,
    pub model: String,
    pub effort: String,
    pub reason: String,
}

/// Validates against one source file, so each refusal can name it.
pub struct Validator<'a> {
    source: &'a str,
}

impl<'a> Validator<'a> {
    pub fn new(source: &'a str) -> Self {
        Validator { source }
    }

    fn invalid(&self, location: &str, message: impl Into<String>) -> Refusal {
        Refusal::new(
            "policy_invalid",
            Stage::Validation,
            EXIT_REFUSED,
            message,
            format!(
                "correct {location} in {}; the types in ravel/sdk describe a \
                 version-{SCHEMA_VERSION} policy",
                self.source
            ),
        )
        .source(self.source)
        .location(location)
    }

    pub fn validate(&self, snapshot: &Value) -> Result<Policy, Refusal> {
        let policy = match snapshot.as_object() {
            Some(object) if !is_marker(snapshot) => object,
            _ => {
                return Err(self.invalid(
                    "policy",
                    format!("expected an object, found {}", describe(snapshot)),
                ))
            }
        };

        // The version comes first: another schema's fields are not unknown to it.
        let schema = policy
            .get("schemaVersion")
            .ok_or_else(|| self.invalid("policy.schemaVersion", "`schemaVersion` is missing"))?;
        match schema.as_u64() {
            Some(SCHEMA_VERSION) => {}
            Some(other) => {
                let unsupported = |message: String, remedy: String| {
                    Refusal::new(
                        "unsupported_version",
                        Stage::Validation,
                        EXIT_REFUSED,
                        message,
                        remedy,
                    )
                    .source(self.source)
                    .location("policy.schemaVersion")
                };
                return Err(if other == 1 {
                    unsupported(
                        "schemaVersion 1 is the catalog contract, which this release no longer \
                         reads: a policy is now one `select` that returns the command to run"
                            .to_owned(),
                        format!(
                            "rewrite {} to schemaVersion {SCHEMA_VERSION}: drop `catalog` and \
                             `routes`, and have `select` return {{ status: \"selected\", program, \
                             args, provider, model, effort, reason }}, building `args` from \
                             `request.prompt` and its other fields; nothing converts a version-1 \
                             policy, and the types in ravel/sdk describe the contract",
                            self.source
                        ),
                    )
                } else {
                    unsupported(
                        format!(
                            "schemaVersion {other} is not supported; this release reads \
                             schemaVersion {SCHEMA_VERSION}"
                        ),
                        format!(
                            "write schemaVersion {SCHEMA_VERSION} in {}, or upgrade \
                             ravel",
                            self.source
                        ),
                    )
                });
            }
            None => {
                return Err(self.invalid(
                    "policy.schemaVersion",
                    format!(
                        "`schemaVersion` must be the number {SCHEMA_VERSION}, found {}",
                        describe(schema)
                    ),
                ))
            }
        }
        if let Some(unknown) = policy.keys().find(|key| !TOP_LEVEL.contains(&key.as_str())) {
            return Err(self.invalid(
                &format!("policy.{unknown}"),
                format!(
                    "unknown field `{unknown}`: a policy has only {}",
                    TOP_LEVEL.join(", ")
                ),
            ));
        }

        let function = |field: &str| match policy.get(field) {
            Some(value) if is_function(value) => Ok(()),
            Some(value) => Err(self.invalid(
                &format!("policy.{field}"),
                format!("`{field}` must be a function, found {}", describe(value)),
            )),
            None => Err(self.invalid(&format!("policy.{field}"), format!("`{field}` is missing"))),
        };
        let loader = policy
            .get("loadContext")
            .is_some_and(|value| !value.is_null());
        if loader {
            function("loadContext")?;
        }
        let version = match policy.get("version") {
            Some(Value::String(version)) if !version.trim().is_empty() => version.clone(),
            Some(Value::String(_)) => {
                return Err(self.invalid("policy.version", "`version` must not be blank"))
            }
            Some(other) => {
                return Err(self.invalid(
                    "policy.version",
                    format!("`version` must be a string, found {}", describe(other)),
                ))
            }
            None => return Err(self.invalid("policy.version", "`version` is missing")),
        };
        function("select")?;
        Ok(Policy { version, loader })
    }
}

const SELECTED_FIELDS: [&str; 7] = [
    "status", "program", "args", "provider", "model", "effort", "reason",
];
const REFUSED_FIELDS: [&str; 4] = ["status", "code", "message", "remedy"];

/// Judge what a valid policy's `select` produced. A selected result is the
/// command to run: a program, an array of string arguments, and nonblank
/// labels and reason. A refusal it returns must say what and why. Every other
/// value refuses, each kind with its own code, and nothing is ever run in its
/// place.
pub fn selected(
    produced: Produced,
    kind: &str,
    source: &str,
    limits: &Limits,
) -> Result<Command, Refusal> {
    let shape = format!(
        "return {{ status: \"selected\", program, args, provider, model, effort, reason }} or \
         {{ status: \"refused\", code, message, remedy }} from select in {source}; the types in \
         ravel/sdk describe both"
    );
    let refuse = |code: &'static str, message: String, remedy: String, location: &str| {
        Refusal::new(code, Stage::Selection, EXIT_REFUSED, message, remedy)
            .source(source)
            .location(location)
    };
    let malformed = |location: &str, message: String| {
        refuse("selection_malformed", message, shape.clone(), location)
    };

    let result = match produced {
        Produced::Result(result) => result,
        Produced::Threw { name, message } => {
            return Err(refuse(
                "selection_threw",
                format!("select in {source} threw, or its promise rejected: {name}: {message}"),
                format!(
                    "fix select in {source} so that it returns a result on every path; to \
                     decline, return {{ status: \"refused\", code, message, remedy }} rather \
                     than throwing"
                ),
                "policy.select",
            ))
        }
        Produced::Unsettled => {
            return Err(refuse(
                "selection_unsettled",
                format!(
                    "select in {source} returned a promise that never settled: it was still \
                     pending when nothing was left running that could settle it"
                ),
                "make every path through select resolve or reject its promise; an await on \
                 something that will never complete leaves it pending"
                    .to_owned(),
                "policy.select",
            ))
        }
        Produced::Unserializable { name, message } => {
            return Err(malformed(
                "result",
                format!("the result of select cannot be serialized: {name}: {message}"),
            ))
        }
        Produced::Breach(breach) => {
            return Err(message_too_large(Stage::Selection, &breach, source, limits))
        }
    };

    if result.is_null() {
        return Err(refuse(
            "selection_abstained",
            format!(
                "select in {source} returned no result (undefined or null), so it selected \
                 nothing"
            ),
            format!("{shape}; ravel never picks a command for a policy that abstains"),
            "result",
        ));
    }
    let fields = match result.as_object() {
        Some(fields) if !is_marker(&result) => fields,
        _ => {
            return Err(malformed(
                "result",
                format!(
                    "a selection result must be an object, found {}",
                    describe(&result)
                ),
            ))
        }
    };
    let status = fields
        .get("status")
        .ok_or_else(|| malformed("result.status", "`status` is missing".to_owned()))?;
    let allowed: &[&str] = match status.as_str() {
        Some("selected") => &SELECTED_FIELDS,
        Some("refused") => &REFUSED_FIELDS,
        _ => {
            return Err(malformed(
                "result.status",
                format!(
                    "`status` must be \"selected\" or \"refused\", found {}",
                    describe_value(status)
                ),
            ))
        }
    };
    if let Some(unknown) = fields.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(malformed(
            &format!("result.{unknown}"),
            format!(
                "unknown field `{unknown}`: a {} result has only {}",
                status.as_str().unwrap_or_default(),
                allowed.join(", ")
            ),
        ));
    }
    let nonblank = |field: &str| {
        let location = format!("result.{field}");
        let value = fields
            .get(field)
            .ok_or_else(|| malformed(&location, format!("`{field}` is missing")))?;
        let text = value.as_str().ok_or_else(|| {
            malformed(
                &location,
                format!("`{field}` must be a string, found {}", describe(value)),
            )
        })?;
        if text.trim().is_empty() {
            return Err(malformed(&location, format!("`{field}` must not be blank")));
        }
        Ok(text.to_owned())
    };

    if status == "refused" {
        let (code, message, remedy) =
            (nonblank("code")?, nonblank("message")?, nonblank("remedy")?);
        return Err(Refusal::new(
            "policy_refused",
            Stage::Selection,
            EXIT_REFUSED,
            format!("the policy {source} refused the selection: {message}"),
            remedy,
        )
        .policy_code(code)
        .input(format!("--kind {kind}"))
        .source(source));
    }

    // Exec carries no NUL, so one is refused here, before a handoff is
    // launched, rather than after.
    let no_nul = |text: &str, location: &str| match text.find('\0') {
        Some(offset) => Err(malformed(
            location,
            format!("a NUL character at byte {offset}, which no argument can carry"),
        )),
        None => Ok(()),
    };
    let program = nonblank("program")?;
    no_nul(&program, "result.program")?;
    let args = fields
        .get("args")
        .ok_or_else(|| malformed("result.args", "`args` is missing".to_owned()))?;
    let args = args.as_array().ok_or_else(|| {
        malformed(
            "result.args",
            format!(
                "`args` must be an array of strings, found {}",
                describe(args)
            ),
        )
    })?;
    let args = args
        .iter()
        .enumerate()
        .map(|(index, arg)| {
            let location = format!("result.args[{index}]");
            let text = arg.as_str().ok_or_else(|| {
                malformed(
                    &location,
                    format!(
                        "an argument must be a string, one whole word, found {}",
                        describe(arg)
                    ),
                )
            })?;
            no_nul(text, &location)?;
            Ok(text.to_owned())
        })
        .collect::<Result<Vec<_>, Refusal>>()?;
    Ok(Command {
        program,
        args,
        provider: nonblank("provider")?,
        model: nonblank("model")?,
        effort: nonblank("effort")?,
        reason: nonblank("reason")?,
    })
}

/// Judge what a loader returned in place of a context: an object with a
/// `status`, which a version-1 context never has. `select`'s refusal shape,
/// `{ status: "refused", code, message, remedy }`, is the policy's own
/// refusal, reported as `select`'s is but at the context stage, and `select`
/// is never asked. Anything else with a `status` is neither a context nor a
/// refusal, and is refused where it sits.
pub fn loader_refused(fields: &Map<String, Value>, kind: &str, source: &str) -> Refusal {
    let malformed = |location: &str, message: String| {
        Refusal::new(
            "context_invalid",
            Stage::Context,
            EXIT_REFUSED,
            format!("loadContext in {source} returned neither a context nor a refusal: {message}"),
            format!(
                "return a version-1 context, or {{ status: \"refused\", code, message, remedy }} \
                 to decline, from loadContext in {source}; the types in ravel/sdk \
                 describe both"
            ),
        )
        .source(source)
        .location(location)
    };
    let status = &fields["status"];
    if status.as_str() != Some("refused") {
        return malformed(
            "context.status",
            format!(
                "`status` must be \"refused\", found {}: a context has no status, and a \
                 selection is select's to make",
                describe_value(status)
            ),
        );
    }
    if let Some(unknown) = fields
        .keys()
        .find(|key| !REFUSED_FIELDS.contains(&key.as_str()))
    {
        return malformed(
            &format!("context.{unknown}"),
            format!(
                "unknown field `{unknown}`: a refusal has only {}",
                REFUSED_FIELDS.join(", ")
            ),
        );
    }
    let mut said = Vec::with_capacity(3);
    for field in ["code", "message", "remedy"] {
        let location = format!("context.{field}");
        match fields.get(field) {
            None => return malformed(&location, format!("`{field}` is missing")),
            Some(Value::String(text)) if !text.trim().is_empty() => said.push(text.clone()),
            Some(Value::String(_)) => {
                return malformed(&location, format!("`{field}` must not be blank"))
            }
            Some(value) => {
                return malformed(
                    &location,
                    format!("`{field}` must be a string, found {}", describe(value)),
                )
            }
        }
    }
    let [code, message, remedy]: [String; 3] = said.try_into().expect("three fields were said");
    Refusal::new(
        "policy_refused",
        Stage::Context,
        EXIT_REFUSED,
        format!("the policy {source} refused the selection in loadContext: {message}"),
        remedy,
    )
    .policy_code(code)
    .input(format!("--kind {kind}"))
    .source(source)
    .location("policy.loadContext")
}

/// A snapshot or result the worker would not send, because it encodes to more
/// than the fixed protocol message bound. Nothing is cut to fit.
pub fn message_too_large(stage: Stage, breach: &Breach, source: &str, limits: &Limits) -> Refusal {
    let bound = limits.message.value;
    let size = breach
        .actual
        .map(|actual| format!("{actual} bytes"))
        .unwrap_or_else(|| "more".to_owned());
    let (what, remedy) = if stage == Stage::Load {
        (
            format!("the policy {source}"),
            "keep the exported policy object under 1 MiB of JSON",
        )
    } else if stage == Stage::Context {
        (
            format!("the failure loadContext in {source} reported"),
            "throw a shorter error from loadContext, under 1 MiB of JSON",
        )
    } else {
        (
            format!("the result of select in {source}"),
            "return a smaller result, under 1 MiB of JSON with its arguments and the prompt \
             among them; a harness can read a long prompt from a file an argument names",
        )
    };
    Refusal::new(
        "message_too_large",
        stage,
        EXIT_REFUSED,
        format!(
            "{what} encodes to {size} as a protocol message, over the fixed bound of {bound} bytes"
        ),
        format!("{remedy}; ravel never truncates a message to fit"),
    )
    .source(source)
    .bound(limits.message)
}

/// Whether a snapshot value is the marker for a JavaScript function.
fn is_function(value: &Value) -> bool {
    is_marker(value) && value["$ravel"] == "function"
}

fn is_marker(value: &Value) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.len() == 1 && object.contains_key("$ravel"))
}

/// A short description of a JSON value's type, for "found …" messages. A marker
/// describes the JavaScript value the worker could not serialize.
fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(_) => "a boolean".to_owned(),
        Value::Number(_) => "a number".to_owned(),
        Value::String(_) => "a string".to_owned(),
        Value::Array(_) => "an array".to_owned(),
        Value::Object(object) => match object.get("$ravel").and_then(Value::as_str) {
            Some(kind) if object.len() == 1 => format!("a {kind}"),
            _ => "an object".to_owned(),
        },
    }
}

/// `describe`, but quoting a string so a wrong status is visible.
fn describe_value(value: &Value) -> String {
    match value {
        Value::String(text) => format!("{text:?}"),
        other => describe(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid() -> Value {
        json!({
            "schemaVersion": 2,
            "version": "v1",
            "select": {"$ravel": "function"},
        })
    }

    fn limits() -> Limits {
        Limits::read(None, None, &crate::settings::Settings::default()).unwrap()
    }

    fn command() -> Value {
        json!({
            "status": "selected", "program": "harness", "args": ["--model", "m", "the prompt"],
            "provider": "p", "model": "m", "effort": "e", "reason": "r",
        })
    }

    fn judged(result: Value) -> Result<Command, Refusal> {
        selected(Produced::Result(result), "impl", "/p.ts", &limits())
    }

    #[test]
    fn a_policy_is_a_version_a_select_and_an_optional_loader() {
        let policy = Validator::new("/p.ts").validate(&valid()).unwrap();
        assert_eq!(policy.version, "v1");
        assert!(!policy.loader);

        let mut loading = valid();
        loading["loadContext"] = json!({"$ravel": "function"});
        assert!(Validator::new("/p.ts").validate(&loading).unwrap().loader);
        loading["loadContext"] = Value::Null;
        assert!(!Validator::new("/p.ts").validate(&loading).unwrap().loader);
        loading["loadContext"] = json!("load");
        let refusal = Validator::new("/p.ts").validate(&loading).unwrap_err();
        assert_eq!(refusal.code, "policy_invalid");
        assert_eq!(refusal.location.as_deref(), Some("policy.loadContext"));
    }

    #[test]
    fn every_invalid_policy_names_where_it_is_wrong() {
        let with = |field: &str, value: Value| {
            let mut policy = valid();
            if value.is_null() {
                policy.as_object_mut().unwrap().remove(field);
            } else {
                policy[field] = value;
            }
            policy
        };
        let cases = [
            (json!("policy"), "policy", "found a string"),
            (json!({"$ravel": "function"}), "policy", "found a function"),
            (
                with("schemaVersion", Value::Null),
                "policy.schemaVersion",
                "missing",
            ),
            (
                with("schemaVersion", json!("2")),
                "policy.schemaVersion",
                "found a string",
            ),
            (with("version", Value::Null), "policy.version", "missing"),
            (with("version", json!(" ")), "policy.version", "blank"),
            (
                with("version", json!(3)),
                "policy.version",
                "found a number",
            ),
            (with("select", Value::Null), "policy.select", "missing"),
            (
                with("select", json!("choose")),
                "policy.select",
                "found a string",
            ),
            (
                with("select", json!({"$ravel": "symbol"})),
                "policy.select",
                "found a symbol",
            ),
            (
                with("select", json!({})),
                "policy.select",
                "found an object",
            ),
            (
                with("catalog", json!([])),
                "policy.catalog",
                "unknown field `catalog`",
            ),
            (
                with("routes", json!({})),
                "policy.routes",
                "unknown field `routes`",
            ),
        ];
        for (policy, location, found) in cases {
            let refusal = Validator::new("/p.ts").validate(&policy).unwrap_err();
            assert_eq!(refusal.code, "policy_invalid", "{policy}");
            assert_eq!(refusal.location.as_deref(), Some(location), "{policy}");
            assert!(
                refusal.message.contains(found),
                "{policy}: {}",
                refusal.message
            );
        }
    }

    #[test]
    fn a_version_1_policy_refuses_with_the_rewrite_remedy_and_another_as_unsupported() {
        let mut old = json!({
            "schemaVersion": 1,
            "version": "v1",
            "catalog": [],
            "routes": {},
        });
        let refusal = Validator::new("/p.ts").validate(&old).unwrap_err();
        assert_eq!(refusal.code, "unsupported_version");
        assert_eq!(refusal.location.as_deref(), Some("policy.schemaVersion"));
        assert!(
            refusal.message.contains("catalog contract"),
            "{}",
            refusal.message
        );
        assert!(
            refusal.remedy.contains("rewrite /p.ts to schemaVersion 2")
                && refusal.remedy.contains("nothing converts"),
            "{}",
            refusal.remedy
        );

        old["schemaVersion"] = 3.into();
        let refusal = Validator::new("/p.ts").validate(&old).unwrap_err();
        assert_eq!(refusal.code, "unsupported_version");
        assert!(
            refusal.remedy.contains("upgrade ravel"),
            "{}",
            refusal.remedy
        );
    }

    #[test]
    fn a_selected_result_is_the_command_as_returned() {
        let command = judged(command()).unwrap();
        assert_eq!(command.program, "harness");
        assert_eq!(command.args, ["--model", "m", "the prompt"]);
        assert_eq!(
            (
                command.provider.as_str(),
                command.model.as_str(),
                command.effort.as_str()
            ),
            ("p", "m", "e")
        );
        assert_eq!(command.reason, "r");

        // No argument is required, and an argument may be empty or hold
        // anything but a NUL: nothing checks that the prompt is among them.
        let mut bare = self::command();
        bare["args"] = json!([]);
        assert!(judged(bare).unwrap().args.is_empty());
        let mut odd = self::command();
        odd["args"] = json!(["", " ", "two words\n$HOME; `x`"]);
        assert_eq!(
            judged(odd).unwrap().args,
            ["", " ", "two words\n$HOME; `x`"]
        );
    }

    #[test]
    fn every_malformed_result_names_where_it_is_wrong() {
        let selected = |extra: Value| {
            let mut result = command();
            for (key, value) in extra.as_object().unwrap() {
                if value.is_null() {
                    result.as_object_mut().unwrap().remove(key);
                } else {
                    result[key] = value.clone();
                }
            }
            result
        };
        let mut cases = vec![
            (json!("a"), "result".to_owned(), "found a string"),
            (json!(["a"]), "result".to_owned(), "found an array"),
            (
                json!({"$ravel": "function"}),
                "result".to_owned(),
                "found a function",
            ),
            (
                selected(json!({"status": null})),
                "result.status".to_owned(),
                "missing",
            ),
            (
                selected(json!({"status": "chosen"})),
                "result.status".to_owned(),
                "found \"chosen\"",
            ),
            (
                selected(json!({"status": 1})),
                "result.status".to_owned(),
                "found a number",
            ),
            (
                selected(json!({"args": null})),
                "result.args".to_owned(),
                "missing",
            ),
            (
                selected(json!({"args": "--yolo"})),
                "result.args".to_owned(),
                "found a string",
            ),
            (
                selected(json!({"args": ["a", 7]})),
                "result.args[1]".to_owned(),
                "found a number",
            ),
            (
                selected(json!({"args": ["a", {"slot": "prompt"}]})),
                "result.args[1]".to_owned(),
                "found an object",
            ),
            (
                selected(json!({"args": ["a", "b\u{0}c"]})),
                "result.args[1]".to_owned(),
                "NUL character at byte 1",
            ),
            (
                selected(json!({"program": "sh\u{0}"})),
                "result.program".to_owned(),
                "NUL character at byte 2",
            ),
            (
                selected(json!({"candidateId": "a"})),
                "result.candidateId".to_owned(),
                "unknown field",
            ),
            (
                selected(json!({"code": "c"})),
                "result.code".to_owned(),
                "unknown field",
            ),
            (
                json!({"status": "refused", "code": "c", "message": "m"}),
                "result.remedy".to_owned(),
                "missing",
            ),
            (
                json!({"status": "refused", "code": "", "message": "m", "remedy": "r"}),
                "result.code".to_owned(),
                "blank",
            ),
            (
                json!({"status": "refused", "code": "c", "message": "m", "remedy": "r", "program": "sh"}),
                "result.program".to_owned(),
                "unknown field",
            ),
        ];
        for field in ["program", "provider", "model", "effort", "reason"] {
            let location = format!("result.{field}");
            let with = |value: Value| {
                let mut extra = Map::new();
                extra.insert(field.to_owned(), value);
                selected(Value::Object(extra))
            };
            cases.push((with(Value::Null), location.clone(), "missing"));
            cases.push((with(json!(" \n")), location.clone(), "blank"));
            cases.push((with(json!(7)), location.clone(), "found a number"));
            cases.push((
                with(json!({"$ravel": "function"})),
                location,
                "found a function",
            ));
        }
        for (result, location, found) in cases {
            let refusal = judged(result.clone()).unwrap_err();
            assert_eq!(refusal.code, "selection_malformed", "{result}");
            assert_eq!(
                refusal.location.as_deref(),
                Some(location.as_str()),
                "{result}"
            );
            assert!(
                refusal.message.contains(found),
                "{result}: {}",
                refusal.message
            );
            assert_eq!(refusal.source.as_deref(), Some("/p.ts"));
        }
    }

    #[test]
    fn each_other_failure_of_select_has_its_own_code() {
        let refused = |produced| selected(produced, "impl", "/p.ts", &limits()).unwrap_err();
        let threw = refused(Produced::Threw {
            name: "TypeError".into(),
            message: "x is undefined".into(),
        });
        assert_eq!(threw.code, "selection_threw");
        assert!(threw.message.contains("TypeError: x is undefined"));
        assert_eq!(refused(Produced::Unsettled).code, "selection_unsettled");
        let unserializable = refused(Produced::Unserializable {
            name: "TypeError".into(),
            message: "cyclic".into(),
        });
        assert_eq!(unserializable.code, "selection_malformed");
        assert_eq!(
            refused(Produced::Result(Value::Null)).code,
            "selection_abstained"
        );
    }

    #[test]
    fn a_result_over_the_message_bound_refuses_by_name() {
        let breach = Breach {
            bound: "message".into(),
            actual: Some(2_000_000),
            source: None,
            max_bytes: None,
        };
        let refusal = selected(Produced::Breach(breach), "impl", "/p.ts", &limits()).unwrap_err();
        assert_eq!(refusal.code, "message_too_large");
        assert_eq!(refusal.bound.map(|bound| bound.value), Some(1_048_576));
    }

    #[test]
    fn a_policys_refusal_keeps_its_code_beside_the_stable_one() {
        let refusal = judged(
            json!({"status": "refused", "code": "no_reviewer", "message": "nobody fits", "remedy": "declare one"}),
        )
        .unwrap_err();
        assert_eq!(refusal.code, "policy_refused");
        assert_eq!(refusal.policy_code.as_deref(), Some("no_reviewer"));
        assert_eq!(refusal.stage, Stage::Selection);
        assert!(
            refusal.message.ends_with("nobody fits"),
            "{}",
            refusal.message
        );
        assert_eq!(refusal.remedy, "declare one");
        assert_eq!(refusal.input.as_deref(), Some("--kind impl"));
    }
}
