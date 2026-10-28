//! The module resolver of the worker (`docs/specs/policy-evaluation.md`,
//! *Policy entry and discovery*).
//!
//! Rhai calls its resolver on the evaluation thread, so a resolution is one more
//! host call: the worker sends the path and the importing file, and the front
//! resolves the path, reads the file on its reader thread, and answers with the
//! canonical name and the source. The worker compiles the source and evaluates
//! its top level as a module. Each module loads once, and a module that imports
//! itself, directly or through others, ends the evaluation.
//!
//! Every failure here is the worker's own end of the evaluation, recorded in the
//! [`Bridge`] and raised so that `try` cannot catch it. A failure of the front
//! to read or count a module never reaches the worker: the front ends the
//! evaluation itself.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rhai::{Engine, EvalAltResult, Module, ModuleResolver, Position, Scope, Shared};
use serde_json::{Value, json};

use super::ScriptFailure;
use super::bridge::Bridge;
use super::engine::{import_failure, parse_failure};

type SharedModule = Shared<Module>;
type Resolved<T> = Result<T, Box<EvalAltResult>>;

pub struct HostResolver {
    bridge: Rc<Bridge>,
    /// The canonical names of the files being loaded, the entry first. A name
    /// that is already here and is imported again is a cycle.
    loading: RefCell<Vec<String>>,
    loaded: RefCell<HashMap<String, SharedModule>>,
}

impl HostResolver {
    pub fn new(bridge: Rc<Bridge>, entry: &str) -> Self {
        HostResolver {
            bridge,
            loading: RefCell::new(vec![entry.to_owned()]),
            loaded: RefCell::new(HashMap::new()),
        }
    }

    fn failure(&self, from: &str, position: Position, message: String) -> ScriptFailure {
        let mut failure = ScriptFailure::new("config_import_failed", "load", message);
        failure.file = Some(from.to_owned());
        failure.line = position.line().map(|line| line as u64);
        failure.column = position.position().map(|column| column as u64);
        failure
    }
}

impl ModuleResolver for HostResolver {
    fn resolve(
        &self,
        engine: &Engine,
        source: Option<&str>,
        path: &str,
        position: Position,
    ) -> Resolved<SharedModule> {
        // A script that Rhai ran without a source name is the entry's own.
        let from = source
            .filter(|source| !source.is_empty())
            .map_or_else(|| self.loading.borrow()[0].clone(), str::to_owned);
        let request = json!({
            "type": "host",
            "call": "import",
            "phase": self.bridge.phase().name(),
            "path": path,
            "from": from,
            "file": from,
            "line": position.line(),
            "column": position.position(),
        });
        let answer = self.bridge.ask(&request)?;
        let (Some("ok"), Some(name)) = (
            answer.get("status").and_then(Value::as_str),
            answer.get("name").and_then(Value::as_str),
        ) else {
            return Err(self.bridge.protocol("an import", "no module name"));
        };

        if self.loading.borrow().iter().any(|loading| loading == name) {
            let chain = self
                .loading
                .borrow()
                .iter()
                .skip_while(|loading| loading.as_str() != name)
                .cloned()
                .chain([name.to_owned()])
                .collect::<Vec<_>>()
                .join(" -> ");
            return Err(self.bridge.end(self.failure(
                &from,
                position,
                format!("the import \"{path}\" closes a cycle: {chain}"),
            )));
        }
        if let Some(module) = self.loaded.borrow().get(name) {
            return Ok(module.clone());
        }
        let Some(text) = answer.get("source").and_then(Value::as_str) else {
            return Err(self
                .bridge
                .protocol("an import", "no source for a new module"));
        };

        let mut ast = engine
            .compile(text)
            .map_err(|error| self.bridge.end(parse_failure(name, &error)))?;
        ast.set_source(name);
        self.loading.borrow_mut().push(name.to_owned());
        let evaluated = Module::eval_ast_as_new(Scope::new(), &ast, engine);
        self.loading.borrow_mut().pop();
        let module: SharedModule = match evaluated {
            Ok(module) => module.into(),
            // A stop, or an end that a deeper import already recorded.
            Err(error) if matches!(*error, EvalAltResult::ErrorTerminated(..)) => {
                return Err(error);
            }
            Err(error) => return Err(self.bridge.end(import_failure(name, &error))),
        };
        self.loaded
            .borrow_mut()
            .insert(name.to_owned(), module.clone());
        Ok(module)
    }
}
