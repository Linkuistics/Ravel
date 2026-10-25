//! The worker's side of the bridge to the Ravel CLI
//! (`docs/specs/policy-evaluation.md`, *The bridge between the script and the
//! CLI* and *Host capabilities*).
//!
//! A host function sends one frame over the private socket and blocks until the
//! answer arrives. The script sees a synchronous call. The thread that reads the
//! socket (`role`) hands each answer to the evaluation thread through a channel,
//! and ends the wait when the front closes its end.
//!
//! Two kinds of end exist, and neither can be caught with `try`. The front ends
//! the evaluation when a bound breaks, and the worker then hears nothing more.
//! The worker ends it when its own check fails, such as a malformed request or an
//! import cycle. [`Bridge::end`] records that failure and raises Rhai's
//! `ErrorTerminated`, which `try` does not catch, and the engine reads the
//! record after the script returns, whatever the script returned.

use std::cell::{Cell, RefCell};
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;

use rhai::{
    Dynamic, EvalAltResult, FuncRegistration, ImmutableString, Map, Module, NativeCallContext,
    Position,
};
use serde_json::{Value, json};

use super::convert::from_json;
use super::helpers;
use super::{Bounds, Phase, ScriptFailure};
use crate::frame::write_frame;

pub(super) type Outcome<T> = Result<T, Box<EvalAltResult>>;

/// The channel to the front.
pub struct Link {
    pub channel: UnixStream,
    /// What the reading thread got from the front: a frame, or `None` once the
    /// channel closed or broke.
    pub frames: Receiver<Option<Value>>,
    pub stop: std::sync::Arc<AtomicBool>,
}

pub struct Bridge {
    link: Option<Link>,
    bounds: Bounds,
    phase: Cell<Phase>,
    ended: RefCell<Option<ScriptFailure>>,
}

impl Bridge {
    pub fn linked(link: Link, bounds: Bounds) -> Rc<Self> {
        Rc::new(Bridge {
            link: Some(link),
            bounds,
            phase: Cell::new(Phase::Load),
            ended: RefCell::new(None),
        })
    }

    /// A bridge with no front, for tests of the engine alone. A host call fails.
    #[cfg(test)]
    pub fn detached(bounds: Bounds) -> Rc<Self> {
        Rc::new(Bridge {
            link: None,
            bounds,
            phase: Cell::new(Phase::Load),
            ended: RefCell::new(None),
        })
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn phase(&self) -> Phase {
        self.phase.get()
    }

    pub fn enter(&self, phase: Phase) {
        self.phase.set(phase);
    }

    /// The failure that ended the evaluation from inside a host call, if one did.
    pub fn ended(&self) -> Option<ScriptFailure> {
        self.ended.borrow().clone()
    }

    /// End the evaluation with `failure`. The first failure stays.
    pub fn end(&self, failure: ScriptFailure) -> Box<EvalAltResult> {
        self.ended.borrow_mut().get_or_insert(failure);
        terminated("ended")
    }

    /// Send `request` and wait for the answer. An error is the evaluation
    /// stopping: the front closed the channel, or it broke.
    pub fn ask(&self, request: &Value) -> Outcome<Value> {
        let Some(link) = &self.link else {
            return Err(EvalAltResult::ErrorRuntime(
                "this worker has no Ravel CLI to ask".into(),
                Position::NONE,
            )
            .into());
        };
        let stopped = || {
            link.stop.store(true, Ordering::Relaxed);
            terminated("stopped")
        };
        write_frame(&link.channel, request).map_err(|_| stopped())?;
        match link.frames.recv() {
            Ok(Some(answer)) => Ok(answer),
            Ok(None) | Err(_) => Err(stopped()),
        }
    }

    /// A malformed request to the host function `function`: the field and the
    /// reason are named, and the evaluation ends.
    fn invalid(
        &self,
        context: &NativeCallContext,
        function: &str,
        field: &str,
        reason: &str,
    ) -> Box<EvalAltResult> {
        let position = context.call_position();
        let mut failure = ScriptFailure::new(
            "host_request_invalid",
            self.phase().name(),
            format!("ravel::{function} was called with a malformed {field}: {reason}"),
        )
        .path(format!("ravel::{function}({field})"));
        failure.file = context.call_source().map(str::to_owned);
        failure.line = position.line().map(|line| line as u64);
        failure.column = position.position().map(|column| column as u64);
        self.end(failure)
    }

    /// The common fields of a `host` frame for a call made at `context`.
    fn frame(&self, context: &NativeCallContext, call: &str) -> Value {
        let position = context.call_position();
        json!({
            "type": "host",
            "call": call,
            "phase": self.phase().name(),
            "file": context.call_source(),
            "line": position.line(),
            "column": position.position(),
        })
    }

    /// `ravel::read_text` or `ravel::read_json` with `path` and `max_bytes`.
    fn read(
        &self,
        context: &NativeCallContext,
        function: &'static str,
        path: &Dynamic,
        max_bytes: Option<&Dynamic>,
    ) -> Outcome<Dynamic> {
        let path = match path.clone().into_immutable_string() {
            Ok(path) if !path.is_empty() => path,
            _ => {
                return Err(self.invalid(
                    context,
                    function,
                    "path",
                    "it must be a nonempty string",
                ));
            }
        };
        let max = match max_bytes {
            None => Value::Null,
            Some(max) => match max.as_int() {
                Ok(max) if max >= 1 => json!(max),
                _ => {
                    return Err(self.invalid(
                        context,
                        function,
                        "max_bytes",
                        "it must be a positive integer",
                    ));
                }
            },
        };
        let mut request = self.frame(context, function);
        request["path"] = json!(path.as_str());
        request["maxBytes"] = max;
        let answer = self.ask(&request)?;
        match answer.get("status").and_then(Value::as_str) {
            Some("ok") => {
                let mut map = Map::new();
                let (field, value) = if function == "read_json" {
                    ("value", answer.get("value"))
                } else {
                    ("text", answer.get("text"))
                };
                let (Some(value), Some(source)) = (value, answer.get("source")) else {
                    return Err(self.protocol(function, "a read reply lacks its value or source"));
                };
                map.insert(field.into(), from_json(value));
                map.insert("source".into(), from_json(source));
                Ok(Dynamic::from_map(map))
            }
            Some("error") => Err(thrown_reply(context, &answer)),
            _ => Err(self.protocol(function, "a read reply has no known status")),
        }
    }

    /// The string in `argument`, or the end of the evaluation as a malformed
    /// `field` of `function`.
    pub(super) fn string(
        &self,
        context: &NativeCallContext,
        function: &str,
        field: &str,
        argument: &Dynamic,
    ) -> Outcome<ImmutableString> {
        argument
            .clone()
            .into_immutable_string()
            .map_err(|_| self.invalid(context, function, field, "it must be a string"))
    }

    /// `ravel::env`: the front holds the caller's environment, and the worker
    /// has none.
    fn env(&self, context: &NativeCallContext, name: &Dynamic) -> Outcome<Dynamic> {
        let name = self.string(context, "env", "name", name)?;
        let mut request = self.frame(context, "env");
        request["name"] = json!(name.as_str());
        let answer = self.ask(&request)?;
        match answer.get("status").and_then(Value::as_str) {
            Some("ok") => match answer.get("value") {
                Some(value @ (Value::Null | Value::String(_))) => Ok(from_json(value)),
                _ => Err(self.protocol("env", "an env reply has no string or null value")),
            },
            Some("error") => Err(thrown_reply(context, &answer)),
            _ => Err(self.protocol("env", "an env reply has no known status")),
        }
    }

    /// The front answered in a way no front of this build does.
    pub fn protocol(&self, what: &str, why: &str) -> Box<EvalAltResult> {
        self.end(ScriptFailure::new(
            "worker_failed",
            self.phase().name(),
            format!("the answer to {what} broke the protocol: {why}"),
        ))
    }

    /// The `ravel` namespace.
    pub fn namespace(self: &Rc<Self>) -> Module {
        let mut module = Module::new();
        for function in ["read_text", "read_json"] {
            let bridge = Rc::clone(self);
            FuncRegistration::new(function).set_into_module(
                &mut module,
                move |context: NativeCallContext, path: Dynamic| {
                    bridge.read(&context, function, &path, None)
                },
            );
            let bridge = Rc::clone(self);
            FuncRegistration::new(function).set_into_module(
                &mut module,
                move |context: NativeCallContext, path: Dynamic, max_bytes: Dynamic| {
                    bridge.read(&context, function, &path, Some(&max_bytes))
                },
            );
        }
        let bridge = Rc::clone(self);
        FuncRegistration::new("env").set_into_module(
            &mut module,
            move |context: NativeCallContext, name: Dynamic| bridge.env(&context, &name),
        );
        helpers::register(&mut module, self);
        module
    }
}

/// A data failure that the script can catch: a map with `code`, `message` and,
/// where it applies, `name`.
pub(super) fn thrown(
    context: &NativeCallContext,
    code: &str,
    message: String,
    name: Option<&str>,
) -> Box<EvalAltResult> {
    let mut map = Map::new();
    map.insert("code".into(), code.into());
    map.insert("message".into(), message.into());
    if let Some(name) = name {
        map.insert("name".into(), name.into());
    }
    EvalAltResult::ErrorRuntime(Dynamic::from_map(map), context.call_position()).into()
}

/// The failure that an `error` reply carries, thrown as it came.
fn thrown_reply(context: &NativeCallContext, answer: &Value) -> Box<EvalAltResult> {
    let text = |field: &str| answer.get(field).map_or(Value::Null, Clone::clone);
    let mut map = Map::new();
    for field in ["code", "message", "name"] {
        map.insert(field.into(), from_json(&text(field)));
    }
    EvalAltResult::ErrorRuntime(Dynamic::from_map(map), context.call_position()).into()
}

/// An error that `try` does not catch. The engine's progress hook ends a run
/// the same way.
fn terminated(token: &str) -> Box<EvalAltResult> {
    EvalAltResult::ErrorTerminated(Dynamic::from(ImmutableString::from(token)), Position::NONE)
        .into()
}
