//! Read owner authority and inputs, evaluate policy, validate its selection and resolve its program.

use std::path::Path;

use serde_json::{Map, Value};

use crate::authority::{self, Format, PolicyEntry};
use crate::cancellation;
use crate::cli::SelectionArgs;
use crate::context::{self, Delivered, Measured, SourceBreach};
use crate::inputs::Inputs;
use crate::limits::{Bound, Limits, Origin};
use crate::policy::{self, Command, Language, Policy, SCHEMA_VERSION, Validator};
use crate::program::{self, Executable};
use crate::refusal::{Diagnostics, EXIT_REFUSED, EXIT_WORKER, Failure, Refusal, Stage};
use crate::rhai_worker::front::{self as rhai_front, Admitted, Session};
use crate::rhai_worker::harness::{RunRecord, RunSetup};
use crate::rhai_worker::{Limit, Module, Read, Reply, ScriptFailure};
use crate::settings::Settings;
use crate::worker::{self, Assembled, Breach, Halt, Loaded, Outcome};

/// The worker that evaluated the entry, as it reported itself.
#[derive(Debug)]
pub enum WorkerReport {
    /// (typescript)
    Bun(worker::WorkerIdentity),
    /// (rhai)
    Rhai(rhai_front::Identity),
}

#[derive(Debug)]
pub struct Choice {
    pub version: String,
    pub worker: WorkerReport,
    pub elapsed: std::time::Duration,
    pub inputs: Inputs,
    pub entry: PolicyEntry,
    /// The command `select` returned, exactly.
    pub command: Command,
    /// The file its program resolved to.
    pub executable: Executable,
    /// The context selection saw, when there was one: a loader's result, or
    /// the caller's document, measured.
    pub context: Option<Delivered>,
    /// The entry first, then each module it imported, as measured. Empty on the
    /// TypeScript path. (rhai)
    pub modules: Vec<Module>,
    /// Every measured read of the Rhai path, with the phase that made it. (rhai)
    pub reads: Vec<Read>,
    /// Every harness run of the Rhai path, in order. (rhai)
    pub runs: Vec<RunRecord>,
    pub diagnostics: Diagnostics,
    pub operational: Vec<String>,
}

/// Resolve owner authority and inputs without evaluating any policy code.
pub struct Prepared {
    pub inputs: Inputs,
    pub entry: PolicyEntry,
    pub settings_path: Option<std::path::PathBuf>,
}

pub fn prepare(args: &SelectionArgs) -> Result<Prepared, Failure> {
    let home = std::env::var_os("HOME");
    let settings = Settings::read(home.as_deref())?;
    let inputs = Inputs::read(args, &settings)?;
    let entry = authority::resolve(args.config.as_deref(), &inputs.cwd, home.as_deref())?;
    let settings_path = home
        .as_deref()
        .map(Path::new)
        .filter(|path| path.is_absolute())
        .map(|path| path.join(crate::settings::SETTINGS_FILE));
    Ok(Prepared {
        inputs,
        entry,
        settings_path,
    })
}

pub fn choose_prepared(prepared: Prepared, setup: &RunSetup) -> Result<Selected, Failure> {
    let Prepared { inputs, entry, .. } = prepared;
    let source = entry.display();
    let evaluated = match entry.format {
        Format::TypeScript => {
            let worker_path = worker::locate()?;
            evaluate_and_resolve(inputs, entry, &worker_path)
        }
        Format::Rhai { .. } => evaluate_and_resolve_rhai(inputs, entry, setup),
    };

    match evaluated {
        // The program is resolved, and the handlers stay: the caller decides
        // when a signal stops cancelling.
        Ok(choice) => match cancellation::received() {
            None => Ok(Selected { choice, source }),
            Some(signal) => Err(Failure::with_diagnostics(
                cancellation::refusal(signal, &source).report(
                    &choice.operational.join("\n"),
                    "inspect any reported worker operation before retrying",
                ),
                choice.diagnostics,
            )),
        },
        Err(failure) => Err(overruled(failure, &source)),
    }
}

/// A choice made while the evaluation's handlers are still installed: a
/// handled signal from here on is noted, and nothing yet acts on it.
pub struct Selected {
    pub choice: Choice,
    /// The policy entry, as a cancellation names it.
    pub source: String,
}

/// A handled signal received before launch decides the outcome:
/// a refusal reached meanwhile is reported as the cancellation, with whatever
/// the policy printed.
pub fn overruled(failure: Failure, source: &str) -> Failure {
    match cancellation::received() {
        None => failure,
        Some(signal) => failure.cancelled_by(signal, source),
    }
}

fn evaluate_and_resolve(
    inputs: Inputs,
    entry: PolicyEntry,
    worker_path: &Path,
) -> Result<Choice, Failure> {
    let source = entry.display();
    let evaluation = worker::evaluate(
        worker_path,
        &entry.path,
        request(&inputs),
        &inputs.limits,
        inputs.context.as_ref().map(|caller| &caller.measured),
        inputs.grants.worker_environment(),
        |outcome, loaded| judge(outcome, loaded, &inputs, &source),
    )?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| {
        let refusal = if evaluation.operational.is_empty() {
            refusal
        } else {
            refusal.report(
                &evaluation.operational.join("\n"),
                "inspect reported worker operations before retrying",
            )
        };
        Failure::with_diagnostics(refusal, diagnostics.clone())
    };

    let Judged {
        policy,
        command,
        context,
        ..
    } = evaluation.decided;
    // The command is validated.
    cancellation::check(&source).map_err(refuse)?;
    let path = std::env::var_os("PATH");
    let executable = program::resolve(&command.program, &source, &inputs.cwd, path.as_deref())
        .map_err(refuse)?;

    Ok(Choice {
        version: policy.version,
        worker: WorkerReport::Bun(evaluation.worker),
        elapsed: evaluation.elapsed,
        inputs,
        entry,
        command,
        executable,
        context,
        modules: Vec::new(),
        reads: Vec::new(),
        runs: Vec::new(),
        diagnostics,
        operational: evaluation.operational,
    })
}

/// Evaluate an entry of the Rhai path in the policy worker, judge what it
/// reports and resolve the program it selected. (rhai)
///
/// The worker reads no file, so the front reads the entry on its reader thread
/// and the source travels in the evaluate frame exactly as read. The context `select` receives is the caller's document
/// with its measurement attached, which only this side can measure.
fn evaluate_and_resolve_rhai(
    inputs: Inputs,
    entry: PolicyEntry,
    setup: &RunSetup,
) -> Result<Choice, Failure> {
    let source = entry.display();
    let Format::Rhai(admitted) = &entry.format else {
        unreachable!("the caller dispatched on the Rhai format")
    };
    let unreadable = |what: &str| admitted.unreadable(what);

    let context = inputs
        .context
        .as_ref()
        .map(|caller| {
            context::deliver(
                caller.value.clone(),
                vec![caller.measured.clone()],
                None,
                &inputs.limits,
                &caller.measured.name,
            )
        })
        .transpose()?;
    let program = rhai_front::executable()?;
    // The worker has no environment of its own. A grant is for `ravel::env`,
    // which reads the caller's environment.
    let launch = rhai_front::launch(&program, Vec::new());
    let mut judge =
        |value: Value, sources: Vec<Measured>| rhai_context(value, sources, &inputs, &source);
    let evaluation = rhai_front::evaluate(
        &Admitted {
            name: &entry.path,
            unreadable: &unreadable,
        },
        &rhai_request(&inputs),
        context.as_ref().map(|delivered| &delivered.value),
        launch,
        &inputs.limits,
        Session {
            cwd: &inputs.cwd,
            grants: &inputs.grants,
            directory: &inputs.directory,
            caller: inputs.context.as_ref().map(|caller| &caller.measured),
            setup,
            judge: &mut judge,
        },
    )?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| {
        let refusal = if evaluation.operational.is_empty() {
            refusal
        } else {
            refusal.report(
                &evaluation.operational.join("\n"),
                "inspect reported worker operations before retrying",
            )
        };
        Failure::with_diagnostics(refusal, diagnostics.clone())
    };

    let (version, result) = match evaluation.reply {
        Reply::Selection { version, result } => (version, result),
        Reply::Failure(failure) => {
            return Err(refuse(script_refusal(&failure, &source, &inputs.limits)));
        }
    };
    let command =
        policy::judge_result(result, &inputs.kind, &source, Language::Rhai).map_err(refuse)?;
    cancellation::check(&source).map_err(refuse)?;
    let path = std::env::var_os("PATH");
    let executable = program::resolve(&command.program, &source, &inputs.cwd, path.as_deref())
        .map_err(refuse)?;

    Ok(Choice {
        version,
        worker: WorkerReport::Rhai(evaluation.worker),
        elapsed: evaluation.elapsed,
        inputs,
        entry,
        command,
        executable,
        // What `load_context` returned, or else the caller's own document.
        context: evaluation.delivered.or(context),
        modules: evaluation.modules,
        reads: evaluation.reads,
        runs: evaluation.runs,
        diagnostics,
        operational: evaluation.operational,
    })
}

/// Judge and measure the context that `load_context` returned. A result with a
/// `status` is no context: it is the policy's refusal, or it is malformed. (rhai)
fn rhai_context(
    value: Value,
    sources: Vec<Measured>,
    inputs: &Inputs,
    source: &str,
) -> Result<Delivered, Refusal> {
    if let Some(fields) = value
        .as_object()
        .filter(|fields| fields.contains_key("status"))
    {
        return Err(policy::loader_refused(
            fields,
            &inputs.kind,
            source,
            Language::Rhai,
        ));
    }
    context::deliver(value, sources, Some(Language::Rhai), &inputs.limits, source)
}

/// The refusal for a failure the Rhai worker reported. The code and stage are
/// the worker's, and a position in the entry becomes `LINE:COLUMN`. (rhai)
fn script_refusal(failure: &ScriptFailure, source: &str, limits: &Limits) -> Refusal {
    let file = failure.file.as_deref().unwrap_or(source);
    let position = match (failure.line, failure.column) {
        (Some(line), Some(column)) => Some(format!("{line}:{column}")),
        (Some(line), None) => Some(line.to_string()),
        _ => None,
    };
    let (code, stage, remedy): (&'static str, Stage, String) = match failure.code.as_str() {
        "policy_parse_failed" => (
            "policy_parse_failed",
            Stage::Load,
            format!("fix the syntax error in {file}, at the line and column the location names"),
        ),
        "policy_import_failed" => (
            "policy_import_failed",
            Stage::Load,
            format!(
                "fix {file}, or the module it imports: a module is a regular UTF-8 file of at \
                 most 1 MiB, at most 64 modules load, none may import itself, and the top level \
                 of each must run to its end"
            ),
        ),
        "context_loader_failed" => (
            "context_loader_failed",
            Stage::Context,
            format!(
                "fix load_context in {file}, or supply what it requires; a loader that fails \
                 fails the whole selection, and nothing is selected without its context"
            ),
        ),
        "context_invalid" => (
            "context_invalid",
            Stage::Context,
            format!(
                "return plain data from load_context in {file}: maps, arrays, strings, finite \
                 numbers, booleans and `()`, nested at most 100 levels"
            ),
        ),
        "host_request_invalid" => {
            let stage = match failure.stage.as_str() {
                "load" => Stage::Load,
                "context" => Stage::Context,
                _ => Stage::Selection,
            };
            let refusal = Refusal::new(
                "host_request_invalid",
                stage,
                EXIT_REFUSED,
                failure.message.clone(),
                format!("correct the call in {file}, at the line and column the location names"),
            )
            .source(file);
            // The location names the function, the field and the line.
            return match (&failure.path, &position) {
                (Some(call), Some(position)) => refusal.location(format!("{call} at {position}")),
                (Some(call), None) => refusal.location(call),
                (None, Some(position)) => refusal.location(position),
                (None, None) => refusal,
            };
        }
        "source_too_large" => {
            let budget = limits.context.value;
            let name = failure.source.clone().unwrap_or_else(|| source.to_owned());
            let (limit, from, requested) = match failure.max_bytes {
                Some(max) if max > budget => (budget, limits.context.origin, Some(max)),
                Some(max) => (max, Origin::Set("max_bytes"), None),
                None => (limits.source.value, limits.source.origin, None),
            };
            let refusal = context::source_too_large(
                limits,
                &SourceBreach {
                    name,
                    limit,
                    from,
                    actual: failure.actual,
                    requested,
                },
                file,
                Language::Rhai,
            );
            return match &position {
                Some(position) => refusal.location(format!("ravel::read_text at {position}")),
                None => refusal,
            };
        }
        "too_many_sources" => {
            return context::too_many_sources(limits, failure.actual.unwrap_or(0), file);
        }
        "context_too_large" => {
            return context::context_too_large(
                limits,
                failure.actual.unwrap_or(0),
                file,
                Some(Language::Rhai),
            );
        }
        "harness_budget_exhausted" => {
            let refusal = Refusal::new(
                "harness_budget_exhausted",
                Stage::Evaluation,
                EXIT_REFUSED,
                failure.message.clone(),
                "raise budgets.harnessRuns in the owner settings, up to 16, or make the policy \
                 run fewer harnesses; a script reads what remains with ravel::remaining() and \
                 cannot raise the budget or catch this refusal",
            )
            .source(file)
            .bound(limits.budgets.bounds()[0]);
            return match (&failure.path, &position) {
                (Some(call), Some(position)) => refusal.location(format!("{call} at {position}")),
                (None, Some(position)) => refusal.location(position),
                _ => refusal,
            };
        }
        "harness_cleanup_failed" => {
            return Refusal::new(
                "harness_cleanup_failed",
                Stage::Evaluation,
                EXIT_WORKER,
                failure.message.clone(),
                "stop the named process and remove the named directory by hand, then retry; do \
                 not signal a process ID that may have been reused",
            )
            .source(file);
        }
        "worker_failed" => {
            return Refusal::new(
                "worker_failed",
                Stage::Worker,
                EXIT_WORKER,
                failure.message.clone(),
                "reinstall ravel, and start the session again",
            )
            .source(file);
        }
        "policy_invalid" => (
            "policy_invalid",
            Stage::Validation,
            format!(
                "define `export const SCHEMA_VERSION = 3;`, a nonblank `export const VERSION` \
                 and `fn select(request, context)` in {file}"
            ),
        ),
        "unsupported_version" => (
            "unsupported_version",
            Stage::Validation,
            format!("set `export const SCHEMA_VERSION = 3;` in {file}; this release reads only 3"),
        ),
        "selection_threw" => (
            "selection_threw",
            Stage::Selection,
            format!(
                "fix select in {file} so that it returns a result on every path; to decline, \
                 return #{{ status: \"refused\", code, message, remedy }} rather than raising"
            ),
        ),
        "selection_malformed" => (
            "selection_malformed",
            Stage::Selection,
            Language::Rhai.shape(source),
        ),
        "script_limit" | "memory_limit" => return limit_refusal(failure, file),
        "message_too_large" => {
            let breach = Breach {
                bound: "message".to_owned(),
                actual: failure.actual,
                source: None,
                max_bytes: None,
            };
            return policy::message_too_large(Stage::Selection, &breach, source, limits);
        }
        other => {
            return Refusal::new(
                "worker_failed",
                Stage::Worker,
                EXIT_WORKER,
                format!(
                    "the policy worker reported the code {other}, which this front does not \
                     know: {}",
                    failure.message
                ),
                "reinstall ravel, and start the session again",
            )
            .source(file);
        }
    };
    let refusal =
        Refusal::new(code, stage, EXIT_REFUSED, failure.message.clone(), remedy).source(file);
    match (&position, &failure.path) {
        (Some(position), _) => refusal.location(position),
        (None, Some(path)) => refusal.location(path),
        (None, None) => refusal,
    }
}

/// The refusal for an engine limit or the worker heap: `script_limit` or
/// `memory_limit`, naming the bound. (rhai)
fn limit_refusal(failure: &ScriptFailure, file: &str) -> Refusal {
    let limit = failure.bound.as_deref().and_then(Limit::from_name);
    let (code, advice): (&'static str, &str) = if failure.code == "memory_limit" {
        (
            "memory_limit",
            "make the policy hold less at one time; the worker ends at 1 GiB of heap",
        )
    } else {
        (
            "script_limit",
            "make the policy do less: the engine limits are fixed, and a script cannot raise one",
        )
    };
    let Some(limit) = limit else {
        return Refusal::new(
            "worker_failed",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker reported {} without a bound this front knows: {}",
                failure.code, failure.message
            ),
            "reinstall ravel, and start the session again",
        )
        .source(file);
    };
    let refusal = Refusal::new(
        code,
        Stage::Evaluation,
        EXIT_REFUSED,
        failure.message.clone(),
        advice,
    )
    .source(file)
    .bound(Bound {
        name: limit.name(),
        unit: limit.unit(),
        value: limit.value(),
        origin: Origin::Fixed,
    });
    match (failure.line, failure.column) {
        (Some(line), Some(column)) => refusal.location(format!("{line}:{column}")),
        (Some(line), None) => refusal.location(line.to_string()),
        _ => refusal,
    }
}

/// What the judge decided: the valid policy, the command it selected, the
/// context it was made with. (typescript)
struct Judged {
    policy: Policy,
    command: Command,
    context: Option<Delivered>,
}

fn judge(
    outcome: Outcome,
    mut loaded: Loaded<'_>,
    inputs: &Inputs,
    source: &str,
) -> Result<Judged, Halt> {
    let import_failed = |detail: String| {
        Refusal::new(
            "policy_import_failed",
            Stage::Load,
            EXIT_REFUSED,
            format!("the policy entry {source} failed to load: {detail}"),
            format!(
                "fix {source} or the module it imports; relative imports resolve from the \
                 importing file, bare ones through node_modules beside it, and nothing is \
                 installed automatically"
            ),
        )
        .source(source)
    };
    let snapshot = match outcome {
        Outcome::Policy(snapshot) => snapshot,
        Outcome::LoadFailed { name, message } => {
            return Err(import_failed(format!("{name}: {message}")).into());
        }
        Outcome::LoadUnsettled => {
            return Err(import_failed(
                "an await in it, or in a module it imports, never settled, and nothing was left \
                 running that could settle it"
                    .to_owned(),
            )
            .into());
        }
        Outcome::Invalid { location, message } => {
            return Err(Refusal::new(
                "policy_invalid",
                Stage::Validation,
                EXIT_REFUSED,
                message,
                format!("export a plain object as `export const policy = {{ ... }}` from {source}"),
            )
            .source(source)
            .location(location)
            .into());
        }
        Outcome::Breach(breach) => {
            return Err(
                policy::message_too_large(Stage::Load, &breach, source, &inputs.limits).into(),
            );
        }
    };
    let policy = Validator::new(source).validate(&snapshot)?;
    let context = if policy.loader || inputs.context.is_some() {
        Some(deliver(loaded.context()?, &policy, inputs, source)?)
    } else {
        None
    };
    let command = policy::selected(loaded.select()?, &inputs.kind, source, &inputs.limits)?;
    Ok(Judged {
        policy,
        command,
        context,
    })
}

/// Judge what the worker assembled as the context: validate and measure it,
/// or refuse for the reason it could not be assembled, or with the refusal the
/// loader returned in its place. Without a loader it is the caller's own
/// document, and a refusal about it names that document.
fn deliver(
    assembled: Assembled,
    policy: &Policy,
    inputs: &Inputs,
    source: &str,
) -> Result<Delivered, Refusal> {
    let limits = &inputs.limits;
    let refuse = |code: &'static str, message: String, remedy: String| {
        Refusal::new(code, Stage::Context, EXIT_REFUSED, message, remedy)
            .source(source)
            .location("policy.loadContext")
    };
    match assembled {
        Assembled::Context { context, measured } => {
            // A version-1 context has no `status`, so a loader's result with
            // one is its refusal, or neither.
            if let Some(fields) = context
                .as_object()
                .filter(|fields| policy.loader && fields.contains_key("status"))
            {
                return Err(policy::loader_refused(
                    fields,
                    &inputs.kind,
                    source,
                    Language::TypeScript,
                ));
            }
            let about = match (&inputs.context, policy.loader) {
                (Some(caller), false) => caller.measured.name.as_str(),
                _ => source,
            };
            context::deliver(
                context,
                measured,
                policy.loader.then_some(Language::TypeScript),
                limits,
                about,
            )
        }
        Assembled::Threw { name, message } => Err(refuse(
            "context_loader_failed",
            format!("loadContext in {source} threw, or its promise rejected: {name}: {message}"),
            format!(
                "fix loadContext in {source}, or supply what it requires; a loader that fails \
                 fails the whole selection, and nothing is selected without its context"
            ),
        )),
        Assembled::SourceUnreadable {
            source: path,
            message,
        } => Err(Refusal::new(
            "context_source_unreadable",
            Stage::Context,
            EXIT_REFUSED,
            format!("loadContext in {source} failed because it could not read {path}: {message}"),
            format!(
                "supply {path}, or change loadContext in {source} so that it does not require \
                 it; a context the policy requires is never assumed"
            ),
        )
        .source(path)
        .location("policy.loadContext")),
        Assembled::Unsettled => Err(refuse(
            "context_loader_unsettled",
            format!(
                "loadContext in {source} returned a promise that never settled: it was still \
                 pending when nothing was left running that could settle it"
            ),
            "make every path through loadContext resolve or reject its promise; an await on \
             something that will never complete leaves it pending"
                .to_owned(),
        )),
        Assembled::Unserializable { name, message } => Err(refuse(
            "context_invalid",
            format!("the context loadContext returned cannot be serialized: {name}: {message}"),
            format!(
                "return plain JSON data from loadContext in {source}: objects, arrays, strings, \
                 finite numbers, booleans and null, with no cycles"
            ),
        )
        .location("context")),
        Assembled::Breach(breach) => Err(context_breach(&breach, policy, inputs, source)),
    }
}

/// A bound the worker saw exceeded while it assembled the context, refused by
/// name with the front's own limits.
fn context_breach(breach: &Breach, policy: &Policy, inputs: &Inputs, source: &str) -> Refusal {
    let limits: &Limits = &inputs.limits;
    let actual = breach.actual.unwrap_or(0);
    match breach.bound.as_str() {
        "source" => {
            let budget = limits.context.value;
            let (limit, from, requested) = match breach.max_bytes {
                Some(max) if max > budget => (budget, limits.context.origin, Some(max)),
                Some(max) => (max, Origin::Set("maxBytes"), None),
                None => (limits.source.value, limits.source.origin, None),
            };
            context::source_too_large(
                limits,
                &SourceBreach {
                    name: breach.source.clone().unwrap_or_else(|| source.to_owned()),
                    limit,
                    from,
                    actual: breach.actual,
                    requested,
                },
                source,
                Language::TypeScript,
            )
        }
        "sources" => context::too_many_sources(limits, actual, source),
        "message" => policy::message_too_large(Stage::Context, breach, source, limits),
        _ => {
            let about = match (&inputs.context, policy.loader) {
                (Some(caller), false) => caller.measured.name.as_str(),
                _ => source,
            };
            context::context_too_large(
                limits,
                actual,
                about,
                policy.loader.then_some(Language::TypeScript),
            )
        }
    }
}

/// The request of the Rhai path: the fields of the caller, with the 3 that
/// the worker adds itself, `schemaVersion` and `limits`, left out. (rhai)
fn rhai_request(inputs: &Inputs) -> Value {
    let mut request = request(inputs);
    let fields = request.as_object_mut().expect("a request is an object");
    fields.remove("schemaVersion");
    fields.remove("limits");
    request
}

/// (typescript)
fn request(inputs: &Inputs) -> Value {
    let mut request = Map::new();
    request.insert("schemaVersion".into(), SCHEMA_VERSION.into());
    request.insert("kind".into(), inputs.kind.clone().into());
    request.insert("prompt".into(), inputs.prompt.text.clone().into());
    request.insert("cwd".into(), inputs.cwd.to_string_lossy().into());
    let params: Map<String, Value> = inputs
        .params
        .iter()
        .map(|(name, value)| (name.clone(), value.clone().into()))
        .collect();
    request.insert("params".into(), params.into());
    if let Some(task_file) = &inputs.task_file {
        request.insert("taskFile".into(), task_file.clone().into());
    }
    if let Some(task_id) = &inputs.task_id {
        request.insert("taskId".into(), task_id.clone().into());
    }
    if let Some(context) = &inputs.context {
        request.insert("context".into(), context.value.clone());
    }
    request.insert("limits".into(), inputs.limits.to_request());
    Value::Object(request)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::context::{CallerContext, Measured};
    use crate::inputs::{Prompt, PromptSource};

    #[test]
    fn the_request_carries_the_prompt_and_every_parameter_beside_the_caller_data() {
        let inputs = Inputs {
            kind: "impl".to_owned(),
            cwd: PathBuf::from("/work"),
            task_file: Some("/work/task.md".to_owned()),
            task_id: Some("T-1".to_owned()),
            params: [("repo", "/work"), ("session_name", "a b\n")]
                .map(|(name, value)| (name.to_owned(), value.to_owned()))
                .into(),
            prompt: Prompt {
                text: "the prompt text\n".to_owned(),
                source: PromptSource::File(PathBuf::from("/work/prompt.md")),
            },
            context: Some(CallerContext {
                value: serde_json::json!({ "schemaVersion": 1, "facts": {} }),
                measured: Measured {
                    name: "/work/context.json".to_owned(),
                    via: "--context".to_owned(),
                    bytes: 32,
                    sha256: "0".repeat(64),
                },
            }),
            limits: Limits::read(None, None, &crate::settings::Settings::default()).unwrap(),
            grants: crate::environment::Grants::default(),
            directory: crate::settings::Directory::default(),
        };
        let request = rhai_request(&inputs);
        assert_eq!(
            request,
            serde_json::json!({
                "kind": "impl", "prompt": "the prompt text\n", "cwd": "/work",
                "params": { "repo": "/work", "session_name": "a b\n" },
                "taskFile": "/work/task.md", "taskId": "T-1",
                "context": { "schemaVersion": 1, "facts": {} },
            })
        );
        let absent = Inputs {
            task_file: None,
            task_id: None,
            params: Default::default(),
            context: None,
            ..inputs
        };
        let request = super::rhai_request(&absent);
        // The order of keys is the order of insertion when a crate turns on
        // serde_json's `preserve_order`, so the keys are compared as a set.
        let mut keys = request.as_object().unwrap().keys().collect::<Vec<_>>();
        keys.sort();
        assert_eq!(keys, ["cwd", "kind", "params", "prompt"]);
        assert_eq!(request["params"], serde_json::json!({}));
        assert_eq!(request["prompt"], "the prompt text\n");
    }
}
