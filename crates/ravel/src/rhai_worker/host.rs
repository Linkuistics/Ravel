//! The host calls the front answers (`docs/specs/policy-evaluation.md`, *Host
//! capabilities* and *Measured reads*): the measured reads and the module
//! imports of the worker.
//!
//! [`Host`] keeps the ledger of one evaluation: the sources it has measured and
//! the modules it has loaded. It never touches the filesystem itself. Each call
//! is a closure that the reader thread runs, so the runtime thread that awaits
//! it keeps watching the deadline and the worker.
//!
//! A call answers in one of two ways. It returns the bytes of a `reply` frame,
//! which may report a data failure that the script can catch. Or it ends the
//! evaluation with a [`ScriptFailure`]. The front returns that failure as the
//! worker's reply and sends the worker nothing more, so a script cannot catch
//! it.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

use super::convert::check_depth;
use super::discovery;
use super::llm_call::{self, Call};
use super::reader::{FileFailure, Reader, canonical, read_file};
use super::{Bounds, LlmCall, MAX_MODULES, Module, Phase, Read, ScriptFailure};
use crate::authority::RHAI_ENTRY_BYTES;
use crate::context::{Measured, hex};
use crate::environment::{Grants, Lookup};
use crate::frame::FRONT_FRAME_BYTES;
use crate::llm::{Adapter, Code, Failure, Response};
use crate::settings::Directory;

/// The `via` of a measured read, as the notice and the delivered context name it.
const VIA_TEXT: &str = "read_text";
const VIA_JSON: &str = "read_json";

/// What the front does with one host call.
pub(super) enum Answer {
    /// The body of the `reply` frame to send.
    Reply(Vec<u8>),
    /// The evaluation ends with this failure, and the worker hears nothing.
    End(Box<ScriptFailure>),
}

/// Where in the policy a host call was made, as the worker reported it.
#[derive(Clone, Debug, Default)]
struct Site {
    file: Option<String>,
    line: Option<u64>,
    column: Option<u64>,
}

impl Site {
    fn of(frame: &Value) -> Self {
        Site {
            file: frame.get("file").and_then(Value::as_str).map(str::to_owned),
            line: frame.get("line").and_then(Value::as_u64),
            column: frame.get("column").and_then(Value::as_u64),
        }
    }

    fn failure(&self, code: &str, phase: Phase, message: String) -> ScriptFailure {
        let mut failure = ScriptFailure::new(code, phase.name(), message);
        failure.file = self.file.clone();
        failure.line = self.line;
        failure.column = self.column;
        failure
    }
}

pub(super) struct Host {
    reader: Reader,
    cwd: PathBuf,
    bounds: Bounds,
    /// The variables of the caller's environment that the policy may read.
    grants: Grants,
    /// The endpoints and harness entries that discovery reports on.
    directory: Directory,
    /// The end of the whole deadline, which `ravel::remaining` counts to.
    deadline: Instant,
    /// The caller's `--context` document, which counts as a source and heads
    /// the measured list of a loader's context.
    caller: Option<Measured>,
    reads: Vec<Read>,
    /// The reads whose bytes arrived and then failed to decode. They count as
    /// sources, because the file was read, and they are no measured read,
    /// because the script got no content.
    undecoded: u64,
    /// The entry first, then each module in the order of its first import.
    modules: Vec<Module>,
    /// The HTTP client of the evaluation, built at the first call that sends.
    adapter: Option<Adapter>,
    /// Every `ravel::llm` call, in order, for the notice.
    llm_calls: Vec<LlmCall>,
    /// The LLM calls the evaluation may still send.
    calls_left: u64,
    /// The output tokens the evaluation may still ask for.
    tokens_left: u64,
}

impl Host {
    pub(super) fn new(
        reader: Reader,
        cwd: PathBuf,
        bounds: Bounds,
        grants: Grants,
        directory: Directory,
        deadline: Instant,
        caller: Option<Measured>,
    ) -> Self {
        Host {
            reader,
            cwd,
            bounds,
            grants,
            directory,
            deadline,
            caller,
            reads: Vec::new(),
            undecoded: 0,
            modules: Vec::new(),
            adapter: None,
            llm_calls: Vec::new(),
            calls_left: bounds.llm_calls,
            tokens_left: bounds.llm_output_tokens,
        }
    }

    /// Read the entry, on the reader thread like every other file, and record it
    /// as the first module. The error completes the sentence "the selected policy
    /// entry PATH ...".
    pub(super) async fn read_entry(&mut self, name: &str) -> Result<String, String> {
        let path = PathBuf::from(name);
        let sent = self.reader.run(move || {
            let content = read_file(&path, RHAI_ENTRY_BYTES)
                .map_err(|failure| describe(&failure, RHAI_ENTRY_BYTES, "an entry"))?;
            let source = String::from_utf8(content.bytes)
                .map_err(|error| format!("is not valid UTF-8: {}", error.utf8_error()))?;
            let module = Module {
                name: content.name,
                bytes: source.len() as u64,
                sha256: hex(&Sha256::digest(source.as_bytes())),
            };
            Ok::<_, String>((module, source))
        });
        let (module, source) = sent
            .await
            .map_err(|_| "cannot be read: the reader thread ended first".to_owned())??;
        self.modules.push(module);
        Ok(source)
    }

    pub(super) fn into_record(self) -> (Vec<Module>, Vec<Read>, Vec<LlmCall>) {
        (self.modules, self.reads, self.llm_calls)
    }

    /// The sources a loader's context is measured from: the caller's document,
    /// then the reads that `load_context` made.
    pub(super) fn context_sources(&self) -> Vec<Measured> {
        self.caller
            .iter()
            .cloned()
            .chain(
                self.reads
                    .iter()
                    .filter(|read| read.phase == Phase::Context)
                    .map(|read| read.measured.clone()),
            )
            .collect()
    }

    /// Answer one `host` frame. An `Err` is a frame that no worker of this
    /// build sends, which breaks the protocol.
    pub(super) async fn call(&mut self, frame: &Value) -> Result<Answer, String> {
        let phase = frame
            .get("phase")
            .and_then(Value::as_str)
            .and_then(Phase::from_name)
            .ok_or("a host frame has no known phase")?;
        let site = Site::of(frame);
        let path = || {
            frame
                .get("path")
                .and_then(Value::as_str)
                .filter(|path| !path.is_empty())
                .ok_or("a host frame has no path")
        };
        match frame.get("call").and_then(Value::as_str) {
            Some("env") => {
                let name = frame
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or("an env frame has no name")?;
                Ok(self.env(name))
            }
            Some("endpoints") => Ok(self.endpoints()),
            Some("harnesses") => Ok(self.harnesses(phase, &site).await),
            Some("remaining") => Ok(self.remaining()),
            Some("llm") => {
                let call = llm_call::decode(frame)?;
                Ok(self.llm(call, phase, &site).await)
            }
            Some(call @ ("read_text" | "read_json")) => {
                let max_bytes = match frame.get("maxBytes") {
                    None | Some(Value::Null) => None,
                    Some(value) => Some(
                        value
                            .as_u64()
                            .filter(|&max| max >= 1)
                            .ok_or("a host frame has a maxBytes that is no positive integer")?,
                    ),
                };
                Ok(self
                    .read(call == "read_json", path()?, max_bytes, phase, &site)
                    .await)
            }
            Some("import") => {
                let from = frame
                    .get("from")
                    .and_then(Value::as_str)
                    .ok_or("an import frame has no importing file")?;
                Ok(self.import(path()?, from, phase, &site).await)
            }
            other => Err(format!("a host frame names the unknown call {other:?}")),
        }
    }

    /// `ravel::env`: the caller's value of a readable name. It touches no file,
    /// so the runtime thread answers it.
    fn env(&self, name: &str) -> Answer {
        let failure = |code: &str, message: String| {
            Answer::Reply(encode(&json!({
                "type": "reply", "status": "error", "code": code, "message": message,
                "name": name,
            })))
        };
        match self.grants.lookup(name) {
            Lookup::Set(value) => match value.into_string() {
                Ok(value) => Answer::Reply(encode(
                    &json!({ "type": "reply", "status": "ok", "value": value }),
                )),
                Err(_) => failure(
                    "env_not_utf8",
                    format!("the variable {name} does not hold UTF-8, which a Rhai string needs"),
                ),
            },
            Lookup::Unset => Answer::Reply(encode(
                &json!({ "type": "reply", "status": "ok", "value": null }),
            )),
            Lookup::NotReadable => failure(
                "env_not_granted",
                format!(
                    "the variable {name} is not readable: a policy reads HOME, PATH, TMPDIR, \
                     LANG, LC_* and the names that policyEnv or --policy-env grant"
                ),
            ),
        }
    }

    /// `ravel::endpoints`: configuration, and whether each credential is set. It
    /// touches no file, so the runtime thread answers it.
    fn endpoints(&self) -> Answer {
        let value = discovery::endpoints(&self.directory.endpoints, |name| std::env::var_os(name));
        Answer::Reply(encode(
            &json!({ "type": "reply", "status": "ok", "value": value }),
        ))
    }

    /// `ravel::harnesses`: each program resolves as a selected program does, on
    /// the reader thread, because the lookup is a filesystem call.
    async fn harnesses(&self, phase: Phase, site: &Site) -> Answer {
        let harnesses = self.directory.harnesses.clone();
        let cwd = self.cwd.clone();
        let path = std::env::var_os("PATH");
        let sent = self
            .reader
            .run(move || discovery::harnesses(&harnesses, &cwd, path.as_deref()));
        match sent.await {
            Ok(value) => Answer::Reply(encode(
                &json!({ "type": "reply", "status": "ok", "value": value }),
            )),
            Err(_) => Answer::End(Box::new(site.failure(
                "worker_failed",
                phase,
                "the reader thread ended before it answered".to_owned(),
            ))),
        }
    }

    /// `ravel::llm`: send one request through the named endpoint. Every failure
    /// is data in the response map, so the answer is always a reply, with one
    /// exception: a call that finds no LLM call or no output token left ends the
    /// evaluation before anything is sent. The worker hears nothing then, so a
    /// script cannot catch it, and `try` around the call changes nothing.
    ///
    /// The adapter call runs on this thread's runtime, which keeps watching the
    /// deadline, so a stop abandons a call in flight. That holds for the whole
    /// deadline, not only for the deadline of the call: the call's deadline is
    /// never later than the whole one, and the watch wins a tie.
    async fn llm(&mut self, mut call: Call, phase: Phase, site: &Site) -> Answer {
        if self.calls_left == 0 || self.tokens_left == 0 {
            return Answer::End(Box::new(self.exhausted(&call, phase, site)));
        }
        // The endpoint may spend only what the budget still holds.
        call.request.max_tokens = call.request.max_tokens.min(self.tokens_left);
        let (outcome, sent) = self.attempt(&call).await;
        if sent {
            self.debit(&call, &outcome);
        }
        let (value, record) = llm_call::answer(&call, &outcome);
        self.llm_calls.push(record);
        Answer::Reply(encode(
            &json!({ "type": "reply", "status": "ok", "value": value }),
        ))
    }

    /// Charge a call that Ravel sent: one LLM call, and the output tokens the
    /// endpoint reported, or the effective `maxTokens` when it reported none.
    /// A failure reports no usage, so a sent call that failed costs its
    /// `maxTokens` too: an endpoint can bill a request that Ravel then fails
    /// or abandons. A count past what remains leaves nothing.
    fn debit(&mut self, call: &Call, outcome: &Result<Response, Failure>) {
        let tokens = match outcome {
            Ok(response) => response.usage.output_tokens,
            Err(_) => None,
        }
        .unwrap_or(call.request.max_tokens);
        self.calls_left = self.calls_left.saturating_sub(1);
        self.tokens_left = self.tokens_left.saturating_sub(tokens);
    }

    /// The failure that ends the evaluation for a call with no budget left. The
    /// refusal names the budget through `bound`.
    fn exhausted(&self, call: &Call, phase: Phase, site: &Site) -> ScriptFailure {
        let (bound, what, all) = if self.calls_left == 0 {
            ("llmCalls", "LLM call", self.bounds.llm_calls)
        } else {
            (
                "llmOutputTokens",
                "output token",
                self.bounds.llm_output_tokens,
            )
        };
        let mut failure = site.failure(
            "llm_budget_exhausted",
            phase,
            format!(
                "ravel::llm to the endpoint {:?} found no {what} left, and the evaluation \
                 may use {all}",
                call.endpoint
            ),
        );
        failure.bound = Some(bound.to_owned());
        failure
    }

    /// The outcome of one call, and whether Ravel sent it. The checks that need
    /// only the settings fail before anything is sent, and so does a client
    /// that cannot start.
    async fn attempt(&mut self, call: &Call) -> (Result<Response, Failure>, bool) {
        let name = &call.endpoint;
        let model = &call.request.model;
        let Some(endpoint) = self.directory.endpoints.get(name) else {
            return (
                Err(Failure::new(
                    Code::UnknownEndpoint,
                    false,
                    format!("settings name no endpoint {name:?}; ravel::endpoints() lists them"),
                )),
                false,
            );
        };
        if !endpoint.models.is_empty() && !endpoint.models.contains(model) {
            return (
                Err(Failure::new(
                    Code::ModelNotOffered,
                    false,
                    format!(
                        "the endpoint {name:?} offers only {}, not {model:?}",
                        endpoint.models.join(", ")
                    ),
                )),
                false,
            );
        }
        if self.adapter.is_none() {
            // Building the client loads the trust store, which on Linux can
            // read files that the caller's environment names. That is a
            // filesystem call, so it runs on the reader thread, and a stop
            // abandons it like any other read.
            let built = match self.reader.run(Adapter::new).await {
                Ok(built) => built,
                Err(_) => Err("the reader thread ended before it answered".to_owned()),
            };
            match built {
                Ok(adapter) => self.adapter = Some(adapter),
                Err(why) => {
                    return (
                        Err(Failure::new(
                            Code::Network,
                            false,
                            format!("the HTTP client could not start: {why}"),
                        )),
                        false,
                    );
                }
            }
        }
        let timeout = llm_call::deadline_of(
            call.timeout,
            self.deadline.saturating_duration_since(Instant::now()),
        );
        let adapter = self.adapter.as_ref().expect("the adapter was just built");
        let outcome = adapter
            .call(
                endpoint,
                |variable| std::env::var_os(variable),
                &call.request,
                timeout,
            )
            .await;
        let sent = outcome
            .as_ref()
            .err()
            .is_none_or(|failure| failure.code.was_sent());
        (outcome, sent)
    }

    /// `ravel::remaining`: what the evaluation still has. The harness runs do not
    /// count yet, so that budget is whole.
    fn remaining(&self) -> Answer {
        let ms = self
            .deadline
            .saturating_duration_since(Instant::now())
            .as_millis();
        let sources = self.bounds.sources.saturating_sub(self.source_count());
        Answer::Reply(encode(&json!({
            "type": "reply", "status": "ok", "value": {
                "ms": u64::try_from(ms).unwrap_or(u64::MAX),
                "llmCalls": self.calls_left,
                "llmOutputTokens": self.tokens_left,
                "harnessRuns": self.bounds.harness_runs,
                "sources": sources,
            },
        })))
    }

    /// The sources the evaluation has used: the measured reads, the reads that
    /// decoded to nothing, and the caller's document.
    fn source_count(&self) -> u64 {
        self.reads.len() as u64 + self.undecoded + u64::from(self.caller.is_some())
    }

    async fn read(
        &mut self,
        json: bool,
        given: &str,
        max_bytes: Option<u64>,
        phase: Phase,
        site: &Site,
    ) -> Answer {
        let joined = self.cwd.join(given);
        let shown = joined.to_string_lossy().into_owned();
        let function = if json { VIA_JSON } else { VIA_TEXT };
        let bound = match max_bytes {
            // No read may take more than the context budget, because nothing
            // it reads could then be delivered.
            Some(max) if max > self.bounds.context_bytes => {
                let mut failure = site.failure(
                    "source_too_large",
                    phase,
                    format!(
                        "ravel::{function} of {shown} asked for max_bytes {max}, over the \
                         context budget of {} bytes, which no read may exceed",
                        self.bounds.context_bytes
                    ),
                );
                failure.source = Some(shown);
                failure.max_bytes = Some(max);
                return Answer::End(Box::new(failure));
            }
            Some(max) => max,
            None => self.bounds.source_bytes,
        };
        let count = self.source_count();
        if count >= self.bounds.sources {
            let mut failure = site.failure(
                "too_many_sources",
                phase,
                format!(
                    "ravel::{function} of {shown} would be source {}, over the bound of {}",
                    count + 1,
                    self.bounds.sources
                ),
            );
            failure.source = Some(shown);
            failure.actual = Some(count + 1);
            return Answer::End(Box::new(failure));
        }

        let sent = self.reader.run(move || fetch(&joined, bound, json));
        let Ok(fetched) = sent.await else {
            return Answer::End(Box::new(site.failure(
                "worker_failed",
                phase,
                "the reader thread ended before it answered".to_owned(),
            )));
        };
        match fetched {
            Fetched::Read { body, measured } => {
                self.reads.push(Read {
                    phase,
                    measured: Measured {
                        via: function.to_owned(),
                        ..measured
                    },
                });
                Answer::Reply(body)
            }
            Fetched::Data(body) => Answer::Reply(body),
            Fetched::Undecoded(body) => {
                self.undecoded += 1;
                Answer::Reply(body)
            }
            Fetched::TooLarge { name, actual } => {
                let mut failure = site.failure(
                    "source_too_large",
                    phase,
                    format!(
                        "ravel::{function} of {name} holds more than its limit of {bound} bytes"
                    ),
                );
                failure.source = Some(name);
                failure.actual = Some(actual);
                failure.max_bytes = max_bytes;
                Answer::End(Box::new(failure))
            }
        }
    }

    async fn import(&mut self, given: &str, from: &str, phase: Phase, site: &Site) -> Answer {
        let refuse = |message: String| {
            Answer::End(Box::new(site.failure(
                "policy_import_failed",
                phase,
                message,
            )))
        };
        let target = module_path(given, Path::new(from));
        let shown = target.to_string_lossy().into_owned();
        let known: HashSet<String> = self
            .modules
            .iter()
            .map(|module| module.name.clone())
            .collect();
        let sent = self.reader.run(move || load_module(&target, &known));
        let Ok(loaded) = sent.await else {
            return refuse("the reader thread ended before it answered".to_owned());
        };
        match loaded {
            Err(why) => refuse(format!("the module \"{given}\" ({shown}) {why}")),
            Ok(Loaded::Known(name)) => Answer::Reply(encode(
                &json!({ "type": "reply", "status": "ok", "name": name, "source": null }),
            )),
            Ok(Loaded::New { module, source }) => {
                if self.modules.len() > MAX_MODULES {
                    return refuse(format!(
                        "the module \"{given}\" ({}) would be module {}, over the bound of \
                         {MAX_MODULES}",
                        module.name,
                        self.modules.len()
                    ));
                }
                let reply = json!({
                    "type": "reply", "status": "ok", "name": module.name, "source": source,
                });
                self.modules.push(module);
                Answer::Reply(encode(&reply))
            }
        }
    }
}

/// The file an import names: relative to the directory of the importing file,
/// with `.rhai` appended when the path has no extension.
fn module_path(given: &str, from: &Path) -> PathBuf {
    let given = Path::new(given);
    let mut target = if given.is_absolute() {
        given.to_path_buf()
    } else {
        from.parent().unwrap_or_else(|| Path::new("/")).join(given)
    };
    if target.extension().is_none() {
        target.set_extension("rhai");
    }
    target
}

enum Loaded {
    /// A module the evaluation already loaded.
    Known(String),
    New {
        module: Module,
        source: String,
    },
}

/// Read a module with the file rules of the entry: a regular UTF-8 file of at
/// most [`RHAI_ENTRY_BYTES`]. A module that is already loaded is not read again.
/// The error is the end of a sentence about the module.
fn load_module(target: &Path, known: &HashSet<String>) -> Result<Loaded, String> {
    let name =
        canonical(target).map_err(|failure| describe(&failure, RHAI_ENTRY_BYTES, "a module"))?;
    if known.contains(&name) {
        return Ok(Loaded::Known(name));
    }
    let content = read_file(Path::new(&name), RHAI_ENTRY_BYTES)
        .map_err(|failure| describe(&failure, RHAI_ENTRY_BYTES, "a module"))?;
    let source = String::from_utf8(content.bytes)
        .map_err(|error| format!("is not valid UTF-8: {}", error.utf8_error()))?;
    let module = Module {
        name: content.name,
        bytes: source.len() as u64,
        sha256: hex(&Sha256::digest(source.as_bytes())),
    };
    Ok(Loaded::New { module, source })
}

/// The end of a sentence about a file that could not be read. `kind` is what
/// the file is, for the bound it exceeded.
fn describe(failure: &FileFailure, bound: u64, kind: &str) -> String {
    match failure {
        FileFailure::Missing(why) => format!("cannot be found: {why}"),
        FileFailure::NotRegular => "is not a regular file".to_owned(),
        FileFailure::Unreadable(why) => why.clone(),
        FileFailure::TooLarge { .. } => {
            format!("holds more than {bound} bytes, the most {kind} may hold")
        }
    }
}

/// What the reader thread made of a read.
enum Fetched {
    Read {
        body: Vec<u8>,
        measured: Measured,
    },
    /// A data failure before any byte arrived, as the body of an error reply.
    Data(Vec<u8>),
    /// A data failure after the bytes arrived, as the body of an error reply.
    /// It counts against the bound of sources.
    Undecoded(Vec<u8>),
    TooLarge {
        name: String,
        actual: u64,
    },
}

/// The work of `read_text` and `read_json` on the reader thread: the file, the
/// text, the value and the reply, so that the runtime thread only awaits.
fn fetch(joined: &Path, bound: u64, json: bool) -> Fetched {
    let shown = joined.to_string_lossy().into_owned();
    let data = |code: &str, message: String, name: &str| {
        Fetched::Data(encode(&json!({
            "type": "reply", "status": "error", "code": code, "message": message, "name": name,
        })))
    };
    let content = match read_file(joined, bound) {
        Ok(content) => content,
        Err(FileFailure::Missing(why)) => {
            return data(
                "source_missing",
                format!("{shown} cannot be opened: {why}"),
                &shown,
            );
        }
        Err(FileFailure::NotRegular) => {
            return data(
                "source_not_regular",
                format!("{shown} is not a regular file"),
                &shown,
            );
        }
        Err(FileFailure::Unreadable(why)) => {
            return data("source_unreadable", format!("{shown} {why}"), &shown);
        }
        Err(FileFailure::TooLarge { name, actual }) => return Fetched::TooLarge { name, actual },
    };
    let name = content.name;
    let sha256 = hex(&Sha256::digest(&content.bytes));
    let bytes = content.bytes.len() as u64;
    let undecoded = |code: &str, message: String| {
        Fetched::Undecoded(encode(&json!({
            "type": "reply", "status": "error", "code": code, "message": message, "name": name,
        })))
    };
    let Ok(text) = String::from_utf8(content.bytes) else {
        return undecoded("source_not_utf8", format!("{name} is not valid UTF-8"));
    };
    let source = json!({ "name": name, "bytes": bytes, "sha256": sha256 });
    let reply = if json {
        match serde_json::from_str::<Value>(&text) {
            Ok(value) => {
                // The conversion bound applies on this side of the socket, so
                // that the reply never nests past what the worker's parser
                // reads, and a deeper file is a failure the script can catch.
                if let Err(refused) = check_depth(&value, "value") {
                    return undecoded(
                        "source_invalid_json",
                        format!("{name} is not usable as JSON: {refused}"),
                    );
                }
                json!({ "type": "reply", "status": "ok", "value": value, "source": source })
            }
            Err(error) => {
                return undecoded(
                    "source_invalid_json",
                    format!("{name} is not JSON: {error}"),
                );
            }
        }
    } else {
        json!({ "type": "reply", "status": "ok", "text": text, "source": source })
    };
    let body = encode(&reply);
    // The escapes of a JSON string can make the reply longer than the file.
    if body.len() > FRONT_FRAME_BYTES {
        return undecoded(
            "source_unreadable",
            format!(
                "{name} encodes to {} bytes, over the {FRONT_FRAME_BYTES} bytes of one reply",
                body.len()
            ),
        );
    }
    Fetched::Read {
        body,
        measured: Measured {
            name,
            via: String::new(),
            bytes,
            sha256,
        },
    }
}

fn encode(reply: &Value) -> Vec<u8> {
    serde_json::to_vec(reply).expect("a JSON value always encodes")
}
