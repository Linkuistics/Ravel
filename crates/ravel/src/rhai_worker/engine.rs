//! The strict Rhai engine and the module contract (`docs/specs/policy-evaluation.md`,
//! *The policy module* and *Request, context and selection*).
//!
//! The worker hands this module one entry source and one request. It compiles
//! the source, runs its top level once as a module, checks the exports and calls
//! `select`. Everything it reports is data in a [`Reply`]: this module never
//! decides what Ravel does with a selection.

use std::io::Write as _;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering::Relaxed};

use rhai::{Dynamic, Engine, EvalAltResult, Module, ParseError, ParseErrorType, Position, Scope};
use serde_json::{Value, json};

use super::bridge::Bridge;
use super::callbacks;
use super::convert::{self, from_json};
use super::locate;
use super::resolver::HostResolver;
use super::{Bounds, EXPRESSION_DEPTH_IN_FUNCTION, Limit, Phase, Reply, ScriptFailure};

/// The value of `SCHEMA_VERSION` this release reads.
pub const SCHEMA_VERSION: i64 = 3;

/// The name of the namespace that holds the host functions.
const HOST_NAMESPACE: &str = "ravel";

/// The module name under which the evaluated entry answers calls. Its functions
/// run against the constants the top level declared, which a call on the AST
/// would not reach.
const ENTRY_MODULE: &str = "config";

/// What the host knows about the evaluation that Rhai errors cannot tell it.
///
/// Rhai passes an engine limit up through a script function without wrapping
/// it, and `try` does not catch it. Rhai does wrap every error that crosses a
/// call of a function pointer, such as the closure that `map` calls, and that
/// wrapper is catchable. `pointer_calls` replaces the functions that wrap, so a
/// limit reaches the top as it is. The host still does not trust the error
/// alone for the one limit it can see. The progress hook records an operations
/// breach here, at the moment it happens, and [`evaluate_with`] reads the record
/// after the script returns, whatever the script returned.
///
/// The record covers the breaches that the host can see, which are the
/// operations bound and a stop. Rhai 1.27.0 has no hook for the other limits,
/// and they end the evaluation through the error itself.
#[derive(Clone, Default)]
pub struct Latch {
    stop: Arc<AtomicBool>,
    breach: Arc<AtomicU8>,
}

impl Latch {
    /// The flag that asks the evaluation to end at its next operation. The role
    /// hands it to the thread that watches the protocol channel.
    pub fn stop_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop)
    }

    fn record(&self, limit: Limit) {
        let code = Limit::ALL
            .iter()
            .position(|&known| known == limit)
            .unwrap_or(0) as u8
            + 1;
        let _ = self.breach.compare_exchange(0, code, Relaxed, Relaxed);
    }

    fn breached(&self) -> Option<Limit> {
        usize::from(self.breach.load(Relaxed))
            .checked_sub(1)
            .and_then(|index| Limit::ALL.get(index).copied())
    }

    fn stopped(&self) -> bool {
        self.stop.load(Relaxed)
    }
}

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
#[cfg(test)]
pub fn evaluate(
    entry: &Entry<'_>,
    request: &Value,
    select_context: Option<&Value>,
    bounds: &Bounds,
) -> Result<Reply, String> {
    evaluate_with(
        entry,
        request,
        select_context,
        bounds,
        &Latch::default(),
        &Bridge::detached(*bounds),
    )
}

/// [`evaluate`] with a latch the caller holds, so that it can stop the
/// evaluation. A stopped evaluation is an `Err`: nobody waits for its reply.
pub fn evaluate_with(
    entry: &Entry<'_>,
    request: &Value,
    select_context: Option<&Value>,
    bounds: &Bounds,
    latch: &Latch,
    bridge: &Rc<Bridge>,
) -> Result<Reply, String> {
    let request = request_map(request, bounds)?;
    let context = match select_context {
        Some(context) if context.is_object() => from_json(context),
        Some(_) => return Err("the selectContext is not an object".to_owned()),
        None => request.get("context").cloned().unwrap_or(Dynamic::UNIT),
    };
    let outcome = run(entry, request, context, latch, bridge);
    if latch.stopped() {
        return Err("the front asked the evaluation to stop".to_owned());
    }
    // The latch decides before the outcome does: a breach that a script
    // swallowed still refuses.
    if let Some(limit) = latch.breached() {
        return Ok(Reply::Failure(limit_failure(entry.name, limit, None, None)));
    }
    // So does an end that a host function raised, which `try` cannot catch.
    if let Some(failure) = bridge.ended() {
        return Ok(Reply::Failure(failure));
    }
    Ok(match outcome {
        Ok((version, result)) => Reply::Selection { version, result },
        Err(failure) => Reply::Failure(*failure),
    })
}

/// The engine every policy runs in.
///
/// The default `Engine` installs a file module resolver, which would read any
/// path a script names; this worker reads no file, so it takes a resolver that
/// asks the front. Source: `Engine::new` in
/// <https://docs.rs/rhai/1.27.0/src/rhai/engine.rs.html>, and
/// <https://docs.rs/rhai/1.27.0/rhai/trait.ModuleResolver.html>.
/// The strict options are the ones of
/// <https://docs.rs/rhai/1.27.0/rhai/struct.Engine.html#method.set_strict_variables>
/// and `set_fail_on_invalid_map_property`.
///
/// Every limit is set here, because Rhai's defaults differ between a debug and
/// a release build (<https://docs.rs/rhai/1.27.0/rhai/struct.Engine.html#method.set_max_call_levels>
/// and the siblings on the same page). The values are the table of the spec's
/// *The Rhai worker*, in [`Limit`]. The operations bound is not set on the
/// engine: the progress hook counts and enforces it, across the top level and
/// `select` together, and records the breach in the latch.
fn engine(latch: &Latch, bridge: &Rc<Bridge>, entry: &Entry<'_>) -> Engine {
    let mut engine = Engine::new();
    // The default `Engine` installs a file resolver. This one asks the front,
    // which reads the module on its reader thread.
    engine.set_module_resolver(HostResolver::new(Rc::clone(bridge), entry.name));
    engine.register_static_module(HOST_NAMESPACE, bridge.namespace().into());
    engine.set_strict_variables(true);
    engine.set_fail_on_invalid_map_property(true);
    engine.disable_symbol("eval");
    callbacks::install(&mut engine);
    let size = |limit: Limit| usize::try_from(limit.value()).unwrap_or(usize::MAX);
    engine.set_max_operations(0);
    engine.set_max_call_levels(size(Limit::CallDepth));
    engine.set_max_expr_depths(
        size(Limit::ExpressionDepth),
        usize::try_from(EXPRESSION_DEPTH_IN_FUNCTION).unwrap_or(usize::MAX),
    );
    engine.set_max_string_size(size(Limit::StringSize));
    engine.set_max_array_size(size(Limit::ArraySize));
    engine.set_max_map_size(size(Limit::MapSize));
    engine.set_max_variables(size(Limit::Variables));
    engine.set_max_functions(size(Limit::ScriptFunctions));
    // The bound of modules is the front's: it counts distinct files, and the
    // spec's 65th module is a 65th file. Rhai's own counter counts import
    // statements, cached ones too, so it would refuse a script that imports one
    // module 65 times.
    engine.set_max_modules(usize::MAX);
    let (stop, latch, operations) = (latch.stop_flag(), latch.clone(), AtomicU64::new(0));
    engine.on_progress(move |_| {
        if stop.load(Relaxed) {
            return Some(Dynamic::from("stopped"));
        }
        if operations.fetch_add(1, Relaxed) >= Limit::Operations.value() {
            latch.record(Limit::Operations);
            return Some(Dynamic::from("operations"));
        }
        None
    });
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
    latch: &Latch,
    bridge: &Rc<Bridge>,
) -> Result<(String, Value), Box<ScriptFailure>> {
    let mut engine = engine(latch, bridge, entry);
    let mut ast = engine
        .compile(entry.source)
        .map_err(|error| Box::new(parse_failure(entry.name, &error)))?;
    ast.set_source(entry.name);

    bridge.enter(Phase::Load);
    let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine)
        .map_err(|error| Box::new(import_failure(entry.name, &error)))?;
    let (version, loads_context) = check_contract(entry, &ast, &module)?;
    engine.register_static_module(ENTRY_MODULE, module.into());

    let context = if loads_context {
        load_context(&engine, entry, &request, bridge)?
    } else {
        context
    };
    bridge.enter(Phase::Selection);
    let mut scope = Scope::new();
    scope.push("request", Dynamic::from_map(request));
    scope.push("context", context);
    let returned = engine
        .eval_with_scope::<Dynamic>(
            &mut scope,
            &format!("{ENTRY_MODULE}::select(request, context)"),
        )
        .map_err(|error| Box::new(selection_failure(entry.name, &error)))?;
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

/// Run `load_context`, hand what it returned to the front, and return the
/// context the front delivers: the value with its measured sources.
fn load_context(
    engine: &Engine,
    entry: &Entry<'_>,
    request: &rhai::Map,
    bridge: &Rc<Bridge>,
) -> Result<Dynamic, Box<ScriptFailure>> {
    bridge.enter(Phase::Context);
    let mut scope = Scope::new();
    scope.push("request", Dynamic::from_map(request.clone()));
    let returned = engine
        .eval_with_scope::<Dynamic>(
            &mut scope,
            &format!("{ENTRY_MODULE}::load_context(request)"),
        )
        .map_err(|error| Box::new(context_failure(entry.name, &error)))?;
    let value = convert::to_json(&returned, "context").map_err(|refused| {
        Box::new(
            ScriptFailure::new(
                "context_invalid",
                "context",
                format!(
                    "load_context returned a value that cannot cross to Ravel as JSON: {}",
                    refused.reason
                ),
            )
            .at(entry.name, None, None)
            .path(refused.path),
        )
    })?;
    // The front reads a context frame up to the context budget, so a larger one
    // is reported by its size and never sent.
    let size = serde_json::to_vec(&value).map_or(u64::MAX, |bytes| bytes.len() as u64);
    let frame = if size > bridge.bounds().context_bytes {
        json!({ "type": "context", "oversize": size })
    } else {
        json!({ "type": "context", "value": value })
    };
    let answer = bridge.ask(&frame).map_err(|_| {
        Box::new(ScriptFailure::new(
            "stopped",
            "context",
            "the front stopped",
        ))
    })?;
    match (
        answer.get("type").and_then(Value::as_str),
        answer.get("context"),
    ) {
        (Some("delivered"), Some(context)) => Ok(from_json(context)),
        _ => Err(Box::new(ScriptFailure::new(
            "worker_failed",
            "context",
            "the answer to the context broke the protocol",
        ))),
    }
}

/// The exports a configuration script must have. The first export that fails refuses. It
/// returns the `VERSION` the configuration script declared.
fn check_contract(
    entry: &Entry<'_>,
    ast: &rhai::AST,
    module: &Module,
) -> Result<(String, bool), Box<ScriptFailure>> {
    let invalid = |message: String, at: Option<locate::Position>| {
        Box::new(located(
            ScriptFailure::new("config_invalid", "validation", message),
            entry,
            at,
        ))
    };
    let constant = |name: &str| locate::declaration(entry.source, "const", name);

    match module.get_var(SCHEMA_VERSION_NAME) {
        None => {
            return Err(invalid(
                format!(
                    "the configuration script does not export {SCHEMA_VERSION_NAME}; add \
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
                    "the configuration script does not export {VERSION_NAME}; add \
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
                "the configuration script does not define `fn select(request, context)`".to_owned(),
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
    let loaders = defined("load_context");
    if !loaders.is_empty()
        && !loaders
            .iter()
            .any(|&(arity, access)| arity == 1 && access == rhai::FnAccess::Public)
    {
        return Err(match loaders.first() {
            Some(&(1, _)) => invalid(
                "`load_context` is private; remove `private`, so that Ravel can call it".to_owned(),
                function("load_context"),
            ),
            Some(&(arity, _)) => invalid(
                format!("`load_context` must take 1 parameter, request, and takes {arity}"),
                function("load_context"),
            ),
            None => unreachable!("the list is not empty"),
        });
    }
    Ok((version, !loaders.is_empty()))
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

fn at(failure: ScriptFailure, file: &str, position: Position) -> ScriptFailure {
    failure.at(
        file,
        position.line().map(|line| line as u64),
        position.position().map(|column| column as u64),
    )
}

/// The limit a Rhai error reports, if it is a limit. Whether an operations
/// breach or a stop ended the run is the latch's to say.
fn limit_of(error: &EvalAltResult) -> Option<Limit> {
    match innermost(error).0 {
        EvalAltResult::ErrorStackOverflow(..) => Some(Limit::CallDepth),
        EvalAltResult::ErrorTooManyVariables(..) => Some(Limit::Variables),
        EvalAltResult::ErrorTooManyOperations(..) => Some(Limit::Operations),
        EvalAltResult::ErrorDataTooLarge(what, ..) => match what.as_str() {
            "Length of string" => Some(Limit::StringSize),
            "Size of array/BLOB" | "Size of BLOB" => Some(Limit::ArraySize),
            "Size of object map" => Some(Limit::MapSize),
            _ => None,
        },
        _ => None,
    }
}

/// The failure for a breach of `limit` in `file`, with the position Rhai gave it.
pub(super) fn limit_failure(
    file: &str,
    limit: Limit,
    position: Option<Position>,
    detail: Option<&str>,
) -> ScriptFailure {
    let value = limit.value();
    let allowed = match limit {
        Limit::ExpressionDepth => {
            format!(
                "{value} levels at the top level and {EXPRESSION_DEPTH_IN_FUNCTION} in a function"
            )
        }
        _ => format!("{value} {}", limit.unit()),
    };
    let detail = detail.map_or_else(String::new, |detail| format!(": {detail}"));
    let failure = ScriptFailure::new(
        "script_limit",
        "evaluation",
        format!(
            "the configuration script exceeded the engine limit {} of {allowed}{detail}",
            limit.name()
        ),
    )
    .bound(limit);
    match position {
        Some(position) => at(failure, file, position),
        None => failure.at(file, None, None),
    }
}

/// The failure for a source that does not parse. `file` is the entry or the
/// module that holds the error.
pub(super) fn parse_failure(file: &str, error: &ParseError) -> ScriptFailure {
    let limit = match error.err_type() {
        ParseErrorType::ExprTooDeep => Some(Limit::ExpressionDepth),
        ParseErrorType::TooManyFunctions => Some(Limit::ScriptFunctions),
        ParseErrorType::LiteralTooLarge(what, _) => match what.as_str() {
            "Length of string" => Some(Limit::StringSize),
            "Size of array" => Some(Limit::ArraySize),
            "Size of object map" => Some(Limit::MapSize),
            _ => None,
        },
        _ => None,
    };
    if let Some(limit) = limit {
        return limit_failure(file, limit, Some(error.position()), None);
    }
    at(
        ScriptFailure::new(
            "config_parse_failed",
            "load",
            without_position(&error.to_string()),
        ),
        file,
        error.position(),
    )
}

/// The failure for a top level that raised an error. `file` is the entry or
/// the module whose top level it was, unless the error came from a function of
/// another file.
pub(super) fn import_failure(file: &str, error: &EvalAltResult) -> ScriptFailure {
    let (cause, position, origin) = innermost(error);
    let file = origin.as_deref().unwrap_or(file);
    if let Some(limit) = limit_of(error) {
        return limit_failure(
            file,
            limit,
            Some(position),
            Some(&without_position(&cause.to_string())),
        );
    }
    let what = without_position(&cause.to_string());
    at(
        ScriptFailure::new(
            "config_import_failed",
            "load",
            format!("the top level of the configuration script failed: {what}"),
        ),
        file,
        position,
    )
}

fn selection_failure(file: &str, error: &EvalAltResult) -> ScriptFailure {
    let (cause, position, origin) = innermost(error);
    if let Some(limit) = limit_of(error) {
        // Rhai moves the position of a limit error to the call that entered the
        // function, and the call that entered `select` is the worker's own
        // expression. That position names nothing in the configuration script, so it stays out.
        return limit_failure(
            file,
            limit,
            None,
            Some(&without_position(&cause.to_string())),
        );
    }
    at(
        ScriptFailure::new(
            "selection_threw",
            "selection",
            format!(
                "select raised an error: {}",
                without_position(&cause.to_string())
            ),
        ),
        origin.as_deref().unwrap_or(file),
        position,
    )
}

/// The failure for a `load_context` that raised an error.
fn context_failure(file: &str, error: &EvalAltResult) -> ScriptFailure {
    let (cause, position, origin) = innermost(error);
    if let Some(limit) = limit_of(error) {
        return limit_failure(
            file,
            limit,
            None,
            Some(&without_position(&cause.to_string())),
        );
    }
    at(
        ScriptFailure::new(
            "context_loader_failed",
            "context",
            format!(
                "load_context raised an error: {}",
                without_position(&cause.to_string())
            ),
        ),
        origin.as_deref().unwrap_or(file),
        position,
    )
}

/// The error that started a chain of calls, with its position and the file of
/// the function it was raised in when that is not the entry: the call frames
/// around it only say where each function was called from.
fn innermost(error: &EvalAltResult) -> (&EvalAltResult, Position, Option<String>) {
    fn descend(
        error: &EvalAltResult,
        origin: Option<String>,
    ) -> (&EvalAltResult, Position, Option<String>) {
        match error {
            EvalAltResult::ErrorInFunctionCall(_, source, inner, _) => {
                let origin = if source.is_empty() {
                    origin
                } else {
                    Some(source.clone())
                };
                descend(inner, origin)
            }
            EvalAltResult::ErrorInModule(_, inner, _) => descend(inner, origin),
            other => (other, other.position(), origin),
        }
    }
    descend(error, None)
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
    use crate::limits::HARNESS_RUNS_DEFAULT;

    fn bounds() -> Bounds {
        Bounds {
            selection_ms: 30_000,
            context_bytes: 262_144,
            source_bytes: 65_536,
            sources: 256,
            message_bytes: 1_048_576,
            diagnostics_bytes: 262_144,
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
        assert_eq!(failure.code, "config_parse_failed");
        assert_eq!(failure.stage, "load");
        assert_eq!(failure.file.as_deref(), Some("/home/o/policy.rhai"));
        assert_eq!((failure.line, failure.column), (Some(4), Some(5)));
        assert!(failure.message.contains("nope"), "{}", failure.message);
    }

    #[test]
    fn a_syntax_error_reports_its_line_and_column() {
        let failure = failure("export const SCHEMA_VERSION = 3;\nlet x = ;\n");
        assert_eq!(failure.code, "config_parse_failed");
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
        assert_eq!(failure.code, "config_parse_failed");
        assert!(failure.message.contains("eval"), "{}", failure.message);
    }

    #[test]
    fn an_import_asks_the_front_and_a_worker_without_one_cannot_resolve_it() {
        let failure = failure(&policy(
            "import \"/etc/hosts\" as hosts;\nfn select(request, context) { () }",
        ));
        assert_eq!(failure.code, "config_import_failed");
        assert!(
            failure.message.contains("no Ravel CLI to ask"),
            "{}",
            failure.message
        );
    }

    #[test]
    fn a_failing_top_level_is_an_import_failure_with_its_location() {
        let failure = failure(
            "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"1\";\nthrow \"early\";\nfn select(a, b) {}\n",
        );
        assert_eq!(failure.code, "config_import_failed");
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
                "config_invalid",
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
                "config_invalid",
                Some((1, 14)),
            ),
            (
                "no VERSION",
                format!("export const SCHEMA_VERSION = 3;\n{select}"),
                "config_invalid",
                None,
            ),
            (
                "blank VERSION",
                format!(
                    "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"  \";\n{select}"
                ),
                "config_invalid",
                Some((2, 14)),
            ),
            (
                "VERSION a number",
                format!("export const SCHEMA_VERSION = 3;\nexport const VERSION = 1;\n{select}"),
                "config_invalid",
                Some((2, 14)),
            ),
            (
                "no select",
                policy("fn helper(a, b) { () }"),
                "config_invalid",
                None,
            ),
            (
                "select of one parameter",
                policy("fn select(request) { () }"),
                "config_invalid",
                Some((3, 4)),
            ),
            (
                "private select",
                policy("private fn select(request, context) { () }"),
                "config_invalid",
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
    fn a_load_context_must_be_public_and_take_the_request_alone() {
        let arity = failure(&policy(
            "fn select(request, context) { () }\nfn load_context(request, more) { #{} }",
        ));
        assert_eq!(arity.code, "config_invalid");
        assert_eq!(arity.stage, "validation");
        assert!(
            arity.message.contains("must take 1 parameter"),
            "{}",
            arity.message
        );
        assert_eq!((arity.line, arity.column), (Some(4), Some(4)));

        let hidden = failure(&policy(
            "fn select(request, context) { () }\nprivate fn load_context(request) { #{} }",
        ));
        assert_eq!(hidden.code, "config_invalid");
        assert!(hidden.message.contains("is private"), "{}", hidden.message);
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

    /// The failure of a configuration script that must hit `limit`, with the fields every
    /// `script_limit` has.
    fn breach(source: &str, limit: Limit) -> ScriptFailure {
        let failure = failure(source);
        assert_eq!(failure.code, "script_limit", "{}", failure.message);
        assert_eq!(failure.stage, "evaluation");
        assert_eq!(failure.bound.as_deref(), Some(limit.name()));
        assert_eq!(failure.file.as_deref(), Some("/home/o/policy.rhai"));
        assert!(
            failure.message.contains(limit.name()),
            "{}",
            failure.message
        );
        failure
    }

    /// A policy whose `select` runs `body` and then selects, so that a body
    /// that survives its hostile step is seen to survive.
    fn selecting(body: &str) -> String {
        policy(&format!(
            "fn select(request, context) {{\n{body}\n    #{{ status: \"selected\" }}\n}}"
        ))
    }

    #[test]
    fn every_limit_of_the_spec_table_is_set_on_the_engine_and_not_left_to_the_build_profile() {
        let entry = Entry {
            name: "/home/o/policy.rhai",
            source: "",
        };
        let engine = engine(&Latch::default(), &Bridge::detached(bounds()), &entry);
        assert_eq!(engine.max_call_levels(), 64);
        assert_eq!(engine.max_expr_depth(), 64);
        assert_eq!(engine.max_function_expr_depth(), 32);
        assert_eq!(engine.max_string_size(), 16 * 1024 * 1024);
        assert_eq!(engine.max_array_size(), 1_048_576);
        assert_eq!(engine.max_map_size(), 1_048_576);
        assert_eq!(engine.max_variables(), 16_384);
        assert_eq!(engine.max_functions(), 4_096);
        // The front counts the modules, as distinct files, so the engine's own
        // count of import statements must not refuse first.
        assert_eq!(engine.max_modules(), usize::MAX);
        // The progress hook counts operations, so the engine's own bound is off.
        assert_eq!(engine.max_operations(), 0);
        // Rhai's defaults for the three depths are 8, 32 and 16 in a debug build
        // and 64, 64 and 32 in a release build, so each of these asserts a value
        // that no build default needs to agree with.
        assert_eq!(Limit::ALL.len(), 9);
    }

    #[test]
    fn an_endless_loop_ends_at_the_operations_bound_and_a_short_loop_does_not() {
        let survived = selecting("    let i = 0;\n    while i < 1000 { i += 1; }");
        assert_eq!(
            reply(&survived),
            selection(json!({ "status": "selected" })),
            "the control loop must finish"
        );
        let failure = breach(
            &policy("fn select(request, context) { loop {} }"),
            Limit::Operations,
        );
        assert!(failure.message.contains("100000000"), "{}", failure.message);
    }

    #[test]
    fn the_operations_bound_counts_the_top_level_and_select_together() {
        // One `while` iteration costs 6 operations in Rhai 1.27.0: the condition,
        // the assignment and their operands. So 10 million iterations are about
        // 60 million operations, under the bound of 100 million. Any cost from 5
        // to 10 operations per iteration keeps this shape: each phase alone
        // stays under the bound, and both together pass it.
        const ITERATIONS: u64 = 10_000_000;
        let loops = |top: bool, within: bool| {
            let spin = |name: &str| {
                format!("let {name} = 0;\nwhile {name} < {ITERATIONS} {{ {name} += 1; }}")
            };
            policy(&format!(
                "{}\nfn select(request, context) {{\n{}\n    #{{ status: \"selected\" }}\n}}",
                if top { spin("i") } else { String::new() },
                if within { spin("j") } else { String::new() },
            ))
        };
        let selected = selection(json!({ "status": "selected" }));
        assert_eq!(
            reply(&loops(true, false)),
            selected,
            "the top level alone must select"
        );
        assert_eq!(
            reply(&loops(false, true)),
            selected,
            "select alone must select"
        );
        // A count that restarts between the two phases, such as the one Rhai
        // passes to the hook for each evaluation, would let this select too.
        let failure = failure(&loops(true, true));
        assert_eq!(failure.code, "script_limit", "{}", failure.message);
        assert_eq!(failure.bound.as_deref(), Some("operations"));
    }

    #[test]
    fn unbounded_recursion_ends_at_the_call_depth_and_a_shallow_one_does_not() {
        let shallow = policy(
            "fn down(n) { if n == 0 { 0 } else { down(n - 1) } }\n\
             fn select(request, context) { down(40); #{ status: \"selected\" } }",
        );
        assert_eq!(reply(&shallow), selection(json!({ "status": "selected" })));
        breach(
            &policy("fn up(n) { up(n + 1) }\nfn select(request, context) { up(0) }"),
            Limit::CallDepth,
        );
    }

    #[test]
    fn a_string_that_doubles_ends_at_the_string_size_and_a_small_one_does_not() {
        let small = selecting("    let s = \"x\";\n    for i in 0..10 { s += s; }");
        assert_eq!(reply(&small), selection(json!({ "status": "selected" })));
        breach(
            &policy("fn select(request, context) { let s = \"x\"; loop { s += s; } }"),
            Limit::StringSize,
        );
        breach(
            &selecting("    let s = \"x\";\n    s.pad(17000000, 'a');"),
            Limit::StringSize,
        );
    }

    #[test]
    fn an_array_that_doubles_ends_at_the_array_size_and_a_small_one_does_not() {
        let small = selecting("    let a = [1];\n    for i in 0..10 { a = a + a; }");
        assert_eq!(reply(&small), selection(json!({ "status": "selected" })));
        breach(
            &policy("fn select(request, context) { let a = [1]; loop { a = a + a; } }"),
            Limit::ArraySize,
        );
        breach(
            &selecting("    let a = [];\n    a.pad(2000000, 0);"),
            Limit::ArraySize,
        );
    }

    #[test]
    fn a_map_that_grows_ends_at_the_map_size_and_a_small_one_does_not() {
        // Rhai checks a size when a call returns a value, not when `push` or an
        // index assignment grows a container in place. So the hostile step joins
        // two maps that are each under the bound.
        let joined = |count: u32| {
            selecting(&format!(
                "    let a = #{{}};\n    let b = #{{}};\n    \
                 for i in 0..{count} {{ a[`a${{i}}`] = i; b[`b${{i}}`] = i; }}\n    let both = a + b;"
            ))
        };
        assert_eq!(
            reply(&joined(1000)),
            selection(json!({ "status": "selected" }))
        );
        breach(&joined(600_000), Limit::MapSize);
    }

    #[test]
    fn too_many_variables_end_at_the_variable_bound() {
        let lets = |count: usize| {
            (0..count)
                .map(|n| format!("    let v{n} = 0;"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(
            reply(&selecting(&lets(100))),
            selection(json!({ "status": "selected" }))
        );
        breach(&selecting(&lets(16_500)), Limit::Variables);
    }

    #[test]
    fn too_many_script_functions_end_at_the_function_bound() {
        let functions = |count: usize| {
            (0..count)
                .map(|n| format!("fn f{n}() {{ }}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let source = |count| policy(&format!("{}\nfn select(a, b) {{ () }}", functions(count)));
        assert_eq!(reply(&source(50)), selection(Value::Null));
        breach(&source(4_200), Limit::ScriptFunctions);
    }

    #[test]
    fn expressions_nest_to_the_depth_of_their_place() {
        // Rhai counts about two levels for each pair of parentheses. The top
        // level allows 64 levels and a function body 32.
        let nested = |depth: usize| format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
        let top = |depth| {
            policy(&format!(
                "const X = {};\nfn select(a, b) {{ () }}",
                nested(depth)
            ))
        };
        let inside = |depth| policy(&format!("fn select(a, b) {{ {} }}", nested(depth)));
        assert_eq!(reply(&top(10)), selection(Value::Null));
        assert_eq!(reply(&inside(8)), selection(json!(1)));
        breach(&top(40), Limit::ExpressionDepth);
        breach(&inside(20), Limit::ExpressionDepth);
        // 20 levels of parentheses pass at the top level and fail in a function:
        // the two places have their own bound.
        assert_eq!(reply(&top(20)), selection(Value::Null));
    }

    #[test]
    fn try_does_not_catch_a_limit_and_catches_an_ordinary_error() {
        let caught = selecting("    try { throw \"boom\"; } catch (e) { }");
        assert_eq!(
            reply(&caught),
            selection(json!({ "status": "selected" })),
            "the control: try catches an ordinary error"
        );
        breach(
            &selecting("    try { let s = \"x\"; loop { s += s; } } catch (e) { }"),
            Limit::StringSize,
        );
        breach(
            &policy("fn up(n) { up(n + 1) }\nfn select(a, b) { try { up(0) } catch (e) { } }"),
            Limit::CallDepth,
        );
        breach(
            &selecting("    try { loop { } } catch (e) { }"),
            Limit::Operations,
        );
    }

    #[test]
    fn a_limit_that_a_closure_call_wraps_is_still_refused_by_the_latch() {
        // `map` calls the closure through a function pointer, and Rhai wraps
        // what comes back in an error that `try` catches. The control shows that
        // the wrapper swallows an ordinary error. The hostile step is an endless
        // loop in the closure, and the latch refuses although `try` caught it.
        let swallowed = selecting("    try { [1].map(|x| { throw \"boom\"; }); } catch (e) { }");
        assert_eq!(
            reply(&swallowed),
            selection(json!({ "status": "selected" }))
        );
        breach(
            &selecting("    try { [1].map(|x| { loop { } }); } catch (e) { }"),
            Limit::Operations,
        );
    }

    #[test]
    fn a_limit_inside_a_function_pointer_call_ends_the_evaluation_although_try_surrounds_it() {
        // Rhai 1.27.0 wraps what a function pointer fails with in an error that
        // `try` catches, whatever the wrapped error is. `pointer_calls` returns
        // a limit unwrapped. Each hostile case runs a closure under `try` with
        // a harmless amount of the same step as its control, which selects.
        let lets = |count: usize| {
            (0..count)
                .map(|n| format!("let v{n} = 0;"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let under_try = |closure: &str| {
            policy(&format!(
                "{RECURSE}fn down(n) {{ if n == 0 {{ 0 }} else {{ down(n - 1) }} }}\n\
                 fn select(request, context) {{\n    try {{ [1].map({closure}); }} catch (e) {{ }}\n    \
                 #{{ status: \"selected\" }}\n}}"
            ))
        };
        let selected = selection(json!({ "status": "selected" }));
        for (label, control, hostile, limit) in [
            (
                "call depth",
                "|x| down(40)".to_owned(),
                "|x| up(0)".to_owned(),
                Limit::CallDepth,
            ),
            (
                "string size",
                "|x| { let s = \"x\"; for i in 0..10 { s += s; } 0 }".to_owned(),
                "|x| { let s = \"x\"; loop { s += s; } }".to_owned(),
                Limit::StringSize,
            ),
            (
                "array size",
                "|x| { let a = [1]; for i in 0..10 { a = a + a; } 0 }".to_owned(),
                "|x| { let a = [1]; loop { a = a + a; } }".to_owned(),
                Limit::ArraySize,
            ),
            (
                "variable count",
                format!("|x| {{ {} 0 }}", lets(100)),
                format!("|x| {{ {} 0 }}", lets(16_500)),
                Limit::Variables,
            ),
        ] {
            assert_eq!(
                reply(&under_try(&control)),
                selected,
                "{label}: the control must select"
            );
            breach(&under_try(&hostile), limit);
        }
    }

    #[test]
    fn try_still_catches_what_a_function_pointer_call_fails_with_when_it_is_no_limit() {
        // The replacements wrap an error that `try` can catch, as Rhai does, so
        // the catch block sees the error of the closure and the configuration script selects.
        let caught = selecting(
            "    try { [1].map(|x| { throw \"boom\"; }); } catch (e) { }\n    \
             try { [1, 2].sort(|a, b| { throw \"boom\"; }); } catch (e) { }\n    \
             try { #{ a: 1 }.filter(|v| { throw \"boom\"; }); } catch (e) { }",
        );
        assert_eq!(reply(&caught), selection(json!({ "status": "selected" })));
        let seen = selecting(
            "    let seen = \"\";\n    \
             try { [1].map(|x| { throw \"boom\"; }); } catch (e) { seen = e; }\n    \
             if !seen.contains(\"boom\") { throw \"the catch block lost the thrown value: \" + seen; }",
        );
        assert_eq!(reply(&seen), selection(json!({ "status": "selected" })));
    }

    #[test]
    fn a_limit_inside_a_function_named_by_drain_or_retain_ends_the_evaluation_although_try_surrounds_it()
     {
        // The deprecated forms `drain("f")` and `retain("f")` call the Rust
        // originals, which wrap the error of `f` for `try`. The control names a
        // function that stays under the bound, and the configuration script selects.
        let doubles = "fn grow(x) { let s = \"x\"; for i in 0..ROUNDS { s += s; } true }\n";
        let under_try = |call: &str, functions: &str| {
            policy(&format!(
                "{RECURSE}fn down(n) {{ if n == 0 {{ true }} else {{ down(n - 1) }} }}\n{functions}\
                 fn select(request, context) {{\n    let a = [1];\n    \
                 try {{ a.{call}; }} catch (e) {{ }}\n    #{{ status: \"selected\" }}\n}}"
            ))
        };
        let selected = selection(json!({ "status": "selected" }));
        for call in ["drain", "retain"] {
            let named = |name: &str| format!("{call}(\"{name}\")");
            let (control, hostile) = ("fn small(x) { down(40) }\n", "fn small(x) { up(0) }\n");
            assert_eq!(
                reply(&under_try(&named("small"), control)),
                selected,
                "{call}: the control must select"
            );
            breach(&under_try(&named("small"), hostile), Limit::CallDepth);
            let (control, hostile) = (
                doubles.replace("ROUNDS", "10"),
                doubles.replace("ROUNDS", "100"),
            );
            assert_eq!(
                reply(&under_try(&named("grow"), &control)),
                selected,
                "{call}: the control must select"
            );
            breach(&under_try(&named("grow"), &hostile), Limit::StringSize);
        }
    }

    /// A function that recurses past the call depth when it is called.
    const RECURSE: &str = "fn up(n) { up(n + 1) }\n";

    #[test]
    fn dedup_and_sort_still_work_with_a_comparer_that_succeeds() {
        // Rhai 1.27.0 discards a comparer's error in these two, so the worker
        // replaces them (`callbacks.rs`). The controls show the replacements
        // keep the results and the argument order of the originals.
        let got = |value: Value| selection(json!({ "status": "selected", "got": value }));
        let changed = |setup: &str, call: &str| {
            reply(&selecting(&format!(
                "    let a = {setup};\n    a.{call};\n    return #{{ status: \"selected\", got: a }};"
            )))
        };
        assert_eq!(
            changed(
                "[1, 1, 2, 2, 3, 1]",
                "dedup(|earlier, later| earlier == later)"
            ),
            got(json!([1, 2, 3, 1]))
        );
        assert_eq!(
            changed("[3, 1, 2]", "sort(|left, right| left - right)"),
            got(json!([1, 2, 3]))
        );
        assert_eq!(
            changed("[3, 1, 2]", "sort_by(|left, right| left > right)"),
            got(json!([3, 2, 1]))
        );
        assert_eq!(changed("[]", "sort(|a, b| a - b)"), got(json!([])));
    }

    #[test]
    fn an_ordinary_error_in_a_dedup_sort_or_order_comparer_now_ends_the_selection() {
        for call in [
            "dedup(|a, b| throw \"boom\")",
            "sort(|a, b| throw \"boom\")",
            "order(|a, b| throw \"boom\")",
            "order_by(|a, b| throw \"boom\")",
        ] {
            let failure = failure(&selecting(&format!("    [1, 2, 3].{call};")));
            assert_eq!(failure.code, "selection_threw", "{call}");
            assert!(failure.message.contains("boom"), "{}", failure.message);
        }
    }

    #[test]
    fn a_limit_inside_a_dedup_sort_or_order_comparer_ends_the_evaluation_without_try() {
        // The functions that Rhai gives a comparer discard its error, so
        // the breach in the comparer used to leave the selection standing. Each
        // hostile case has a control with the same shape that selects.
        let recurses = |call: &str, depth: &str| {
            policy(&format!(
                "{RECURSE}fn select(request, context) {{\n    let a = [1, 2];\n    \
                 a.{call}(|x, y| {{ {depth}; 0 }});\n    #{{ status: \"selected\" }}\n}}"
            ))
        };
        let doubles = |call: &str, rounds: &str| {
            policy(&format!(
                "fn select(request, context) {{\n    let a = [1, 2];\n    \
                 a.{call}(|x, y| {{ let s = \"x\"; for i in 0..{rounds} {{ s += s; }} 0 }});\n    \
                 #{{ status: \"selected\" }}\n}}"
            ))
        };
        for call in ["dedup", "sort", "order", "order_by"] {
            assert_eq!(
                reply(&recurses(call, "1")),
                selection(json!({ "status": "selected" })),
                "{call}: the control must select"
            );
            breach(&recurses(call, "up(0)"), Limit::CallDepth);
            assert_eq!(
                reply(&doubles(call, "10")),
                selection(json!({ "status": "selected" })),
                "{call}: the control must select"
            );
            breach(&doubles(call, "100"), Limit::StringSize);
        }
    }

    #[test]
    fn a_stop_ends_a_cooperative_script_at_its_next_operation() {
        let latch = Latch::default();
        let stop = latch.stop_flag();
        let source = policy("fn select(request, context) { loop { } }");
        let halted = std::thread::spawn(move || {
            let entry = Entry {
                name: "/home/o/policy.rhai",
                source: &source,
            };
            evaluate_with(
                &entry,
                &request(),
                None,
                &bounds(),
                &latch,
                &Bridge::detached(bounds()),
            )
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!halted.is_finished(), "the endless loop must still run");
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        let result = halted.join().unwrap();
        assert!(result.is_err(), "{result:?}");
    }

    #[test]
    fn a_closure_has_no_json_form_so_no_host_call_of_one_can_follow() {
        // rhaiscript/rhai#1193: Rhai 1.27.0 loses the imports of a closure that
        // a host calls after the script ended. The engine here only ever calls
        // `policy::select`, a named function, and the converter refuses every
        // function pointer, so nothing a script returns can be called later.
        for returned in ["|x| x + 1", "Fn(\"select\")", "curry(Fn(\"select\"), 1)"] {
            let failure = failure(&policy(&format!(
                "fn select(request, context) {{ #{{ args: [{returned}] }} }}"
            )));
            assert_eq!(failure.code, "selection_malformed", "{returned}");
            assert_eq!(
                failure.path.as_deref(),
                Some("result.args[0]"),
                "{returned}"
            );
        }
        let kept = failure(&policy(
            "const KEEP = |x| x + 1;\nfn select(request, context) { global::KEEP }",
        ));
        assert_eq!(kept.code, "selection_malformed");
    }
}
