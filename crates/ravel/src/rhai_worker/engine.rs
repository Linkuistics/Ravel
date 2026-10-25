//! The strict Rhai engine and the module contract (`docs/specs/policy-evaluation.md`,
//! *The policy module* and *Request, context and selection*).
//!
//! The worker hands this module one entry source and one request. It compiles
//! the source, runs its top level once as a module, checks the exports and calls
//! `select`. Everything it reports is data in a [`Reply`]: this module never
//! decides what Ravel does with a selection.

use std::io::Write as _;

use rhai::{
    Dynamic, Engine, EvalAltResult, Module, ParseError, Position, Scope,
    module_resolvers::DummyModuleResolver,
};
use serde_json::Value;

use super::convert::{self, from_json};
use super::locate;
use super::{Bounds, Reply, ScriptFailure};

/// The value of `SCHEMA_VERSION` this release reads.
pub const SCHEMA_VERSION: i64 = 3;

/// The module name under which the evaluated entry answers calls. Its functions
/// run against the constants the top level declared, which a call on the AST
/// would not reach.
const ENTRY_MODULE: &str = "policy";

/// What the worker evaluates: one entry, as the front read it.
pub struct Entry<'a> {
    /// The name the failures give the entry, exactly as the front admitted it.
    pub name: &'a str,
    pub source: &'a str,
}

/// Evaluate `entry` for `request`. A request that does not have the shape the
/// front sends is an `Err`: it breaks the protocol, and no policy can cause it.
///
/// `select_context` is the second argument of `select`. The front sends the
/// caller's document with its measurement there, because the front owns the
/// measured values. Without one, `select` receives `request.context`.
pub fn evaluate(
    entry: &Entry<'_>,
    request: &Value,
    select_context: Option<&Value>,
    bounds: &Bounds,
) -> Result<Reply, String> {
    let request = request_map(request, bounds)?;
    let context = match select_context {
        Some(context) if context.is_object() => from_json(context),
        Some(_) => return Err("the selectContext is not an object".to_owned()),
        None => request.get("context").cloned().unwrap_or(Dynamic::UNIT),
    };
    Ok(match run(entry, request, context) {
        Ok((version, result)) => Reply::Selection { version, result },
        Err(failure) => Reply::Failure(*failure),
    })
}

/// The engine every policy runs in.
///
/// The default `Engine` installs a file module resolver, which would read any
/// path a script names; this worker reads no file, so it takes the dummy
/// resolver, which refuses every import. Source: `Engine::new` in
/// <https://docs.rs/rhai/1.27.0/src/rhai/engine.rs.html>, and
/// <https://docs.rs/rhai/1.27.0/rhai/module_resolvers/struct.DummyModuleResolver.html>.
/// The strict options are the ones of
/// <https://docs.rs/rhai/1.27.0/rhai/struct.Engine.html#method.set_strict_variables>
/// and `set_fail_on_invalid_map_property`. Every limit keeps its default; the
/// leaf `rhai-engine-limits` sets them.
fn engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_module_resolver(DummyModuleResolver::new());
    engine.set_strict_variables(true);
    engine.set_fail_on_invalid_map_property(true);
    engine.disable_symbol("eval");
    engine.on_print(|text| {
        let _ = writeln!(std::io::stdout(), "{text}");
    });
    engine.on_debug(|text, source, position| {
        let source = source.map_or_else(String::new, |source| format!("{source} @ "));
        let _ = writeln!(std::io::stderr(), "{source}{position} | {text}");
    });
    engine
}

fn run(
    entry: &Entry<'_>,
    request: rhai::Map,
    context: Dynamic,
) -> Result<(String, Value), Box<ScriptFailure>> {
    let mut engine = engine();
    let mut ast = engine
        .compile(entry.source)
        .map_err(|error| Box::new(parse_failure(entry, &error)))?;
    ast.set_source(entry.name);

    let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine)
        .map_err(|error| Box::new(import_failure(entry, &error)))?;
    let version = check_contract(entry, &ast, &module)?;
    engine.register_static_module(ENTRY_MODULE, module.into());

    let mut scope = Scope::new();
    scope.push("request", Dynamic::from_map(request));
    scope.push("context", context);
    let returned = engine
        .eval_with_scope::<Dynamic>(
            &mut scope,
            &format!("{ENTRY_MODULE}::select(request, context)"),
        )
        .map_err(|error| Box::new(selection_failure(entry, &error)))?;
    let result = convert::to_json(&returned, "result").map_err(|refused| {
        Box::new(
            ScriptFailure::new(
                "selection_malformed",
                "selection",
                format!(
                    "select returned a value that cannot cross to Ravel as JSON: {}",
                    refused.reason
                ),
            )
            .at(entry.name, None, None)
            .path(refused.path),
        )
    })?;
    Ok((version, result))
}

/// The exports a policy must have. The first export that fails refuses. It
/// returns the `VERSION` the policy declared.
fn check_contract(
    entry: &Entry<'_>,
    ast: &rhai::AST,
    module: &Module,
) -> Result<String, Box<ScriptFailure>> {
    let invalid = |message: String, at: Option<locate::Position>| {
        Box::new(located(
            ScriptFailure::new("policy_invalid", "validation", message),
            entry,
            at,
        ))
    };
    let constant = |name: &str| locate::declaration(entry.source, "const", name);

    match module.get_var(SCHEMA_VERSION_NAME) {
        None => {
            return Err(invalid(
                format!(
                    "the policy does not export {SCHEMA_VERSION_NAME}; add \
                     `export const {SCHEMA_VERSION_NAME} = {SCHEMA_VERSION};`"
                ),
                None,
            ));
        }
        Some(declared) => match declared.as_int() {
            Ok(SCHEMA_VERSION) => {}
            Ok(other) => {
                return Err(Box::new(located(
                    ScriptFailure::new(
                        "unsupported_version",
                        "validation",
                        format!(
                            "{SCHEMA_VERSION_NAME} is {other}, and this release reads \
                             {SCHEMA_VERSION}"
                        ),
                    ),
                    entry,
                    constant(SCHEMA_VERSION_NAME),
                )));
            }
            Err(found) => {
                return Err(invalid(
                    format!(
                        "{SCHEMA_VERSION_NAME} must be the integer {SCHEMA_VERSION}, found a \
                         value of type {found}"
                    ),
                    constant(SCHEMA_VERSION_NAME),
                ));
            }
        },
    }

    let version = match module.get_var(VERSION_NAME) {
        None => {
            return Err(invalid(
                format!(
                    "the policy does not export {VERSION_NAME}; add \
                     `export const {VERSION_NAME} = \"...\";`"
                ),
                None,
            ));
        }
        Some(declared) => match declared.into_string() {
            Ok(text) if !text.trim().is_empty() => text.to_string(),
            Ok(_) => {
                return Err(invalid(
                    format!("{VERSION_NAME} must not be blank"),
                    constant(VERSION_NAME),
                ));
            }
            Err(found) => {
                return Err(invalid(
                    format!("{VERSION_NAME} must be a string, found a value of type {found}"),
                    constant(VERSION_NAME),
                ));
            }
        },
    };

    let function = |name: &str| locate::declaration(entry.source, "fn", name);
    let defined = |name: &str| {
        ast.iter_functions()
            .filter(|function| function.name == name)
            .map(|function| (function.params.len(), function.access))
            .collect::<Vec<_>>()
    };
    let selects = defined("select");
    if !selects
        .iter()
        .any(|&(arity, access)| arity == 2 && access == rhai::FnAccess::Public)
    {
        return Err(match selects.first() {
            None => invalid(
                "the policy does not define `fn select(request, context)`".to_owned(),
                None,
            ),
            Some(&(2, _)) => invalid(
                "`select` is private; remove `private`, so that Ravel can call it".to_owned(),
                function("select"),
            ),
            Some(&(arity, _)) => invalid(
                format!("`select` must take 2 parameters, request and context, and takes {arity}"),
                function("select"),
            ),
        });
    }
    if !defined("load_context").is_empty() {
        return Err(invalid(
            "this build does not support `load_context` yet; remove it, and the selection \
             receives the caller's context"
                .to_owned(),
            function("load_context"),
        ));
    }
    Ok(version)
}

const SCHEMA_VERSION_NAME: &str = "SCHEMA_VERSION";
const VERSION_NAME: &str = "VERSION";

/// The request map of the spec: every documented field is present, and an
/// optional field the caller did not give holds `()`. `limits` is the worker's
/// own copy of the bounds.
fn request_map(request: &Value, bounds: &Bounds) -> Result<rhai::Map, String> {
    let object = request.as_object().ok_or("the request is not an object")?;
    let mut map = rhai::Map::new();
    map.insert("schemaVersion".into(), Dynamic::from(SCHEMA_VERSION));
    for field in ["kind", "prompt", "cwd"] {
        let text = object
            .get(field)
            .filter(|value| value.is_string())
            .ok_or_else(|| format!("the request field {field} is not a string"))?;
        map.insert(field.into(), from_json(text));
    }
    let params = object
        .get("params")
        .and_then(Value::as_object)
        .filter(|params| params.values().all(Value::is_string))
        .ok_or("the request field params is not a map of strings")?;
    map.insert("params".into(), from_json(&Value::Object(params.clone())));
    for field in ["taskFile", "taskId"] {
        let value = object.get(field).unwrap_or(&Value::Null);
        if !value.is_null() && !value.is_string() {
            return Err(format!("the request field {field} is not a string or null"));
        }
        map.insert(field.into(), from_json(value));
    }
    let context = object.get("context").unwrap_or(&Value::Null);
    if !context.is_null() && !context.is_object() {
        return Err("the request field context is not an object or null".to_owned());
    }
    map.insert("context".into(), from_json(context));
    map.insert("limits".into(), from_json(&bounds.to_json()));
    Ok(map)
}

fn located(
    failure: ScriptFailure,
    entry: &Entry<'_>,
    at: Option<locate::Position>,
) -> ScriptFailure {
    failure.at(
        entry.name,
        at.map(|position| position.line),
        at.map(|position| position.column),
    )
}

fn at(failure: ScriptFailure, entry: &Entry<'_>, position: Position) -> ScriptFailure {
    failure.at(
        entry.name,
        position.line().map(|line| line as u64),
        position.position().map(|column| column as u64),
    )
}

fn parse_failure(entry: &Entry<'_>, error: &ParseError) -> ScriptFailure {
    at(
        ScriptFailure::new(
            "policy_parse_failed",
            "load",
            without_position(&error.to_string()),
        ),
        entry,
        error.position(),
    )
}

fn import_failure(entry: &Entry<'_>, error: &EvalAltResult) -> ScriptFailure {
    let (cause, position) = innermost(error);
    at(
        ScriptFailure::new(
            "policy_import_failed",
            "load",
            format!(
                "the top level of the policy failed: {}",
                without_position(&cause.to_string())
            ),
        ),
        entry,
        position,
    )
}

fn selection_failure(entry: &Entry<'_>, error: &EvalAltResult) -> ScriptFailure {
    let (cause, position) = innermost(error);
    at(
        ScriptFailure::new(
            "selection_threw",
            "selection",
            format!(
                "select raised an error: {}",
                without_position(&cause.to_string())
            ),
        ),
        entry,
        position,
    )
}

/// The error that started a chain of calls, with its position in the entry:
/// the call frames around it only say where each function was called from.
fn innermost(error: &EvalAltResult) -> (&EvalAltResult, Position) {
    match error {
        EvalAltResult::ErrorInFunctionCall(_, _, inner, _)
        | EvalAltResult::ErrorInModule(_, inner, _) => innermost(inner),
        other => (other, other.position()),
    }
}

/// Rhai appends ` (line N, position M)` to a message. The failure carries the
/// location as fields, so the text drops it.
fn without_position(message: &str) -> String {
    let message = message.trim_end();
    match message.rfind(" (line ") {
        Some(at) if message.ends_with(')') => message[..at].to_owned(),
        _ => message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::rhai_worker::{HARNESS_RUNS_DEFAULT, LLM_CALLS_DEFAULT, LLM_OUTPUT_TOKENS_DEFAULT};

    fn bounds() -> Bounds {
        Bounds {
            selection_ms: 30_000,
            context_bytes: 262_144,
            source_bytes: 65_536,
            sources: 256,
            message_bytes: 1_048_576,
            diagnostics_bytes: 262_144,
            llm_calls: LLM_CALLS_DEFAULT,
            llm_output_tokens: LLM_OUTPUT_TOKENS_DEFAULT,
            harness_runs: HARNESS_RUNS_DEFAULT,
        }
    }

    fn request() -> Value {
        json!({
            "kind": "impl", "prompt": "do it", "cwd": "/work",
            "params": { "mode": "fast" },
        })
    }

    fn run_with(source: &str, request: &Value) -> Reply {
        let entry = Entry {
            name: "/home/o/policy.rhai",
            source,
        };
        evaluate(&entry, request, None, &bounds()).unwrap()
    }

    /// What a successful evaluation of the `policy` helper's source reports.
    fn selection(result: Value) -> Reply {
        Reply::Selection {
            version: "1.0".to_owned(),
            result,
        }
    }

    fn reply(source: &str) -> Reply {
        run_with(source, &request())
    }

    fn failure(source: &str) -> ScriptFailure {
        match reply(source) {
            Reply::Failure(failure) => failure,
            other => panic!("expected a failure, got {other:?}"),
        }
    }

    fn policy(body: &str) -> String {
        format!("export const SCHEMA_VERSION = 3;\nexport const VERSION = \"1.0\";\n{body}\n")
    }

    #[test]
    fn a_route_table_selects() {
        let source = policy(
            r#"const TABLE = #{ impl: #{ program: "claude", args: ["--print"] } };
fn select(request, context) {
    let route = global::TABLE[request.kind];
    #{ status: "selected", program: route.program, args: route.args,
       provider: "p", model: "m", effort: "e", reason: request.params.mode }
}"#,
        );
        assert_eq!(
            reply(&source),
            selection(json!({
                "status": "selected", "program": "claude", "args": ["--print"],
                "provider": "p", "model": "m", "effort": "e", "reason": "fast",
            }))
        );
    }

    #[test]
    fn the_top_level_runs_once_before_select_and_select_runs_against_its_constants() {
        let source = policy(
            r#"const SEEN = [1 + 1];
fn select(request, context) { global::SEEN }"#,
        );
        assert_eq!(reply(&source), selection(json!([2])));
    }

    #[test]
    fn the_request_map_has_every_documented_field() {
        let source = policy(
            r#"fn select(request, context) {
    let names = request.keys();
    names.sort();
    #{ names: names, task_file: request.taskFile, task_id: request.taskId,
       context: context, schema: request.schemaVersion }
}"#,
        );
        assert_eq!(
            reply(&source),
            selection(json!({
                "names": ["context", "cwd", "kind", "limits", "params", "prompt",
                          "schemaVersion", "taskFile", "taskId"],
                "task_file": null, "task_id": null, "context": null, "schema": 3,
            }))
        );
    }

    #[test]
    fn the_given_optional_fields_reach_the_request_and_the_context_argument() {
        let source = policy(
            r#"fn select(request, context) {
    #{ file: request.taskFile, id: request.taskId, same: request.context == context,
       facts: context.facts }
}"#,
        );
        let request = json!({
            "kind": "impl", "prompt": "p", "cwd": "/w", "params": {},
            "taskFile": "/w/t.md", "taskId": "T-1",
            "context": { "schemaVersion": 1, "facts": { "a": 1 } },
        });
        assert_eq!(
            run_with(&source, &request),
            selection(json!({
                "file": "/w/t.md", "id": "T-1", "same": true, "facts": { "a": 1 },
            }))
        );
    }

    #[test]
    fn limits_holds_the_effective_bounds() {
        let source = policy("fn select(request, context) { request.limits }");
        assert_eq!(reply(&source), selection(bounds().to_json()));
    }

    #[test]
    fn a_malformed_request_breaks_the_protocol_and_is_no_policy_failure() {
        let entry = Entry {
            name: "p.rhai",
            source: "",
        };
        for request in [
            json!([]),
            json!({ "kind": 1, "prompt": "", "cwd": "", "params": {} }),
            json!({ "kind": "k", "prompt": "", "cwd": "", "params": { "a": 1 } }),
            json!({ "kind": "k", "prompt": "", "cwd": "", "params": {}, "taskId": 4 }),
            json!({ "kind": "k", "prompt": "", "cwd": "", "params": {}, "context": [] }),
        ] {
            assert!(
                evaluate(&entry, &request, None, &bounds()).is_err(),
                "{request}"
            );
        }
    }

    #[test]
    fn an_undefined_variable_fails_when_the_script_compiles() {
        let failure = failure(&policy("fn select(request, context) {\n    nope + 1\n}"));
        assert_eq!(failure.code, "policy_parse_failed");
        assert_eq!(failure.stage, "load");
        assert_eq!(failure.file.as_deref(), Some("/home/o/policy.rhai"));
        assert_eq!((failure.line, failure.column), (Some(4), Some(5)));
        assert!(failure.message.contains("nope"), "{}", failure.message);
    }

    #[test]
    fn a_syntax_error_reports_its_line_and_column() {
        let failure = failure("export const SCHEMA_VERSION = 3;\nlet x = ;\n");
        assert_eq!(failure.code, "policy_parse_failed");
        assert_eq!((failure.line, failure.column), (Some(2), Some(9)));
        assert!(!failure.message.contains("line"), "{}", failure.message);
    }

    #[test]
    fn an_absent_map_property_fails_with_a_location_and_a_unit_property_is_readable() {
        let failure = failure(&policy(
            "fn select(request, context) {\n    request.params.nope\n}",
        ));
        assert_eq!(failure.code, "selection_threw");
        assert_eq!((failure.line, failure.column), (Some(4), Some(20)));
        assert!(failure.message.contains("nope"), "{}", failure.message);

        let readable = policy("fn select(request, context) { request.taskFile }");
        assert_eq!(reply(&readable), selection(Value::Null));
    }

    #[test]
    fn eval_is_off() {
        let failure = failure(&policy(r#"fn select(request, context) { eval("1 + 1") }"#));
        assert_eq!(failure.code, "policy_parse_failed");
        assert!(failure.message.contains("eval"), "{}", failure.message);
    }

    #[test]
    fn an_import_has_no_resolver_to_read_a_file() {
        let failure = failure(&policy(
            "import \"/etc/hosts\" as hosts;\nfn select(request, context) { () }",
        ));
        assert_eq!(failure.code, "policy_import_failed");
        assert!(
            failure.message.contains("/etc/hosts"),
            "{}",
            failure.message
        );
    }

    #[test]
    fn a_failing_top_level_is_an_import_failure_with_its_location() {
        let failure = failure(
            "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"1\";\nthrow \"early\";\nfn select(a, b) {}\n",
        );
        assert_eq!(failure.code, "policy_import_failed");
        assert_eq!((failure.line, failure.column), (Some(3), Some(1)));
        assert!(failure.message.contains("early"), "{}", failure.message);
    }

    #[test]
    fn the_contract_refuses_each_missing_or_wrong_export() {
        let select = "fn select(request, context) { () }";
        type Case<'a> = (&'a str, String, &'a str, Option<(u64, u64)>);
        let cases: [Case<'_>; 9] = [
            (
                "no SCHEMA_VERSION",
                format!("export const VERSION = \"1\";\n{select}"),
                "policy_invalid",
                None,
            ),
            (
                "SCHEMA_VERSION 2",
                format!(
                    "export const SCHEMA_VERSION = 2;\nexport const VERSION = \"1\";\n{select}"
                ),
                "unsupported_version",
                Some((1, 14)),
            ),
            (
                "SCHEMA_VERSION a string",
                format!(
                    "export const SCHEMA_VERSION = \"3\";\nexport const VERSION = \"1\";\n{select}"
                ),
                "policy_invalid",
                Some((1, 14)),
            ),
            (
                "no VERSION",
                format!("export const SCHEMA_VERSION = 3;\n{select}"),
                "policy_invalid",
                None,
            ),
            (
                "blank VERSION",
                format!(
                    "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"  \";\n{select}"
                ),
                "policy_invalid",
                Some((2, 14)),
            ),
            (
                "VERSION a number",
                format!("export const SCHEMA_VERSION = 3;\nexport const VERSION = 1;\n{select}"),
                "policy_invalid",
                Some((2, 14)),
            ),
            (
                "no select",
                policy("fn helper(a, b) { () }"),
                "policy_invalid",
                None,
            ),
            (
                "select of one parameter",
                policy("fn select(request) { () }"),
                "policy_invalid",
                Some((3, 4)),
            ),
            (
                "private select",
                policy("private fn select(request, context) { () }"),
                "policy_invalid",
                Some((3, 12)),
            ),
        ];
        for (label, source, code, at) in cases {
            let failure = failure(&source);
            assert_eq!(failure.code, code, "{label}: {}", failure.message);
            assert_eq!(failure.stage, "validation", "{label}");
            assert_eq!(
                failure.file.as_deref(),
                Some("/home/o/policy.rhai"),
                "{label}"
            );
            assert_eq!(
                failure.line.zip(failure.column),
                at,
                "{label}: {}",
                failure.message
            );
        }
    }

    #[test]
    fn a_wrong_version_message_names_both_versions() {
        let failure = failure("export const SCHEMA_VERSION = 2;\n");
        assert_eq!(failure.code, "unsupported_version");
        assert!(failure.message.contains("2") && failure.message.contains("3"));
    }

    #[test]
    fn a_defined_load_context_refuses_as_this_build_does_not_support_it() {
        let failure = failure(&policy(
            "fn select(request, context) { () }\nfn load_context(request) { #{} }",
        ));
        assert_eq!(failure.code, "policy_invalid");
        assert_eq!(failure.stage, "validation");
        assert!(
            failure
                .message
                .contains("does not support `load_context` yet"),
            "{}",
            failure.message
        );
        assert_eq!((failure.line, failure.column), (Some(4), Some(4)));
    }

    #[test]
    fn a_return_of_unit_is_data_for_the_judge() {
        let source = policy("fn select(request, context) { () }");
        assert_eq!(reply(&source), selection(Value::Null));
    }

    #[test]
    fn an_uncaught_error_in_a_nested_call_reports_the_line_that_threw() {
        let failure = failure(&policy(
            "fn inner(x) {\n    throw `bad ${x}`;\n}\nfn select(request, context) {\n    inner(request.kind)\n}",
        ));
        assert_eq!(failure.code, "selection_threw");
        assert_eq!(failure.stage, "selection");
        assert_eq!(failure.file.as_deref(), Some("/home/o/policy.rhai"));
        assert_eq!((failure.line, failure.column), (Some(4), Some(5)));
        assert!(failure.message.contains("bad impl"), "{}", failure.message);
    }

    #[test]
    fn a_result_that_does_not_convert_is_named_by_its_location() {
        let failure = failure(&policy(
            "fn select(request, context) { #{ args: [\"a\", 'c'] } }",
        ));
        assert_eq!(failure.code, "selection_malformed");
        assert_eq!(failure.stage, "selection");
        assert_eq!(failure.path.as_deref(), Some("result.args[1]"));
    }
}
