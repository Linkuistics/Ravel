//! Validate the plain JSON result of Rhai selection and loader refusals.

use serde_json::{Map, Value};

use crate::limits::Limits;
use crate::refusal::{EXIT_REFUSED, Refusal, Stage};

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

const SELECTED_FIELDS: [&str; 7] = [
    "status", "program", "args", "provider", "model", "effort", "reason",
];
const REFUSED_FIELDS: [&str; 4] = ["status", "code", "message", "remedy"];

/// The result shapes `select` may return, as a remedy.
pub fn shape(source: &str) -> String {
    format!(
        "return #{{ status: \"selected\", program, args, provider, model, effort, reason }} or \
         #{{ status: \"refused\", code, message, remedy }} from select in {source}"
    )
}

/// Judge the value `select` returned, already converted to JSON by the worker.
pub fn judge_result(result: Value, kind: &str, source: &str) -> Result<Command, Refusal> {
    let shape = shape(source);
    let refuse = |code: &'static str, message: String, remedy: String, location: &str| {
        Refusal::new(code, Stage::Selection, EXIT_REFUSED, message, remedy)
            .source(source)
            .location(location)
    };
    let malformed = |location: &str, message: String| {
        refuse("selection_malformed", message, shape.clone(), location)
    };

    if result.is_null() {
        return Err(refuse(
            "selection_abstained",
            format!(
                "select in {source} returned no result (the unit value `()`), so it selected nothing"
            ),
            format!(
                "{shape}; ravel never picks a command for a configuration script that abstains"
            ),
            "result",
        ));
    }
    let fields = match result.as_object() {
        Some(fields) => fields,
        _ => {
            return Err(malformed(
                "result",
                format!(
                    "a selection result must be an object, found {}",
                    describe(&result)
                ),
            ));
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
            ));
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
            "config_refused",
            Stage::Selection,
            EXIT_REFUSED,
            format!("the configuration script {source} refused the selection: {message}"),
            remedy,
        )
        .config_code(code)
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
/// `{ status: "refused", code, message, remedy }`, is the configuration script's own
/// refusal, reported as `select`'s is but at the context stage, and `select`
/// is never asked. Anything else with a `status` is neither a context nor a
/// refusal, and is refused where it sits.
pub fn loader_refused(fields: &Map<String, Value>, kind: &str, source: &str) -> Refusal {
    let malformed = |location: &str, message: String| {
        Refusal::new(
            "context_invalid",
            Stage::Context,
            EXIT_REFUSED,
            format!("load_context in {source} returned neither a context nor a refusal: {message}"),
            format!(
                "return a version-1 context, or #{{ status: \"refused\", code, message, remedy }} \
                 to decline, from load_context in {source}"
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
                return malformed(&location, format!("`{field}` must not be blank"));
            }
            Some(value) => {
                return malformed(
                    &location,
                    format!("`{field}` must be a string, found {}", describe(value)),
                );
            }
        }
    }
    let [code, message, remedy]: [String; 3] = said.try_into().expect("three fields were said");
    Refusal::new(
        "config_refused",
        Stage::Context,
        EXIT_REFUSED,
        format!(
            "the configuration script {source} refused the selection in load_context: {message}"
        ),
        remedy,
    )
    .config_code(code)
    .input(format!("--kind {kind}"))
    .source(source)
    .location("config.load_context")
}

/// A result that exceeds the fixed protocol message bound is never truncated.
pub fn message_too_large(actual: Option<u64>, source: &str, limits: &Limits) -> Refusal {
    let bound = limits.message.value;
    let size = actual
        .map(|actual| format!("{actual} bytes"))
        .unwrap_or_else(|| "more".to_owned());
    Refusal::new(
        "message_too_large",
        Stage::Selection,
        EXIT_REFUSED,
        format!("the result of select in {source} encodes to {size} as a protocol message, over the fixed bound of {bound} bytes"),
        "return a smaller result, under 1 MiB of JSON with its arguments and the prompt among them; a harness can read a long prompt from a file an argument names; ravel never truncates a message to fit",
    ).source(source).bound(limits.message)
}

/// A short description of a JSON value for refusal messages.
fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(_) => "a boolean".to_owned(),
        Value::Number(_) => "a number".to_owned(),
        Value::String(_) => "a string".to_owned(),
        Value::Array(_) => "an array".to_owned(),
        Value::Object(_) => "an object".to_owned(),
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
        judge_result(result, "impl", "/p.rhai")
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
                "result.status".to_owned(),
                "missing",
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
                "found an object",
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
            assert_eq!(refusal.source.as_deref(), Some("/p.rhai"));
        }
    }

    #[test]
    fn unit_abstains_and_an_oversized_result_refuses() {
        assert_eq!(judged(Value::Null).unwrap_err().code, "selection_abstained");
        let refusal = message_too_large(Some(2_000_000), "/p.rhai", &limits());
        assert_eq!(refusal.code, "message_too_large");
        assert_eq!(refusal.bound.map(|bound| bound.value), Some(1_048_576));
    }

    #[test]
    fn a_configs_refusal_keeps_its_code_beside_the_stable_one() {
        let refusal = judged(
            json!({"status": "refused", "code": "no_reviewer", "message": "nobody fits", "remedy": "declare one"}),
        )
        .unwrap_err();
        assert_eq!(refusal.code, "config_refused");
        assert_eq!(refusal.config_code.as_deref(), Some("no_reviewer"));
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
