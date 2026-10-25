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

use keyed_launch::StopRequest;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use tokio::sync::oneshot;

use super::convert::check_depth;
use super::discovery;
use super::harness::{
    self, Executed, Plan, Request, RunRecord, RunSetup, Unavailable, cleanup_message,
    cleanup_refusal,
};
use super::reader::{FileFailure, Reader, canonical, read_file};
use super::{Bounds, MAX_MODULES, Module, Phase, Read, ScriptFailure};
use crate::authority::RHAI_ENTRY_BYTES;
use crate::context::{Measured, hex};
use crate::environment::{Grants, Lookup};
use crate::frame::FRONT_FRAME_BYTES;
use crate::settings::Directory;
use crate::worker::Conversation;

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

/// What the evaluation runs in, beyond its bounds and its deadline.
pub(super) struct Surroundings {
    pub cwd: PathBuf,
    /// The variables of the caller's environment that the policy may read.
    pub grants: Grants,
    /// The harness entries that discovery reports on.
    pub directory: Directory,
    /// The caller's `--context` document, which counts as a source and heads
    /// the measured list of a loader's context.
    pub caller: Option<Measured>,
    /// How the CLI launches a harness run.
    pub setup: RunSetup,
}

pub(super) struct Host {
    reader: Reader,
    cwd: PathBuf,
    bounds: Bounds,
    /// The variables of the caller's environment that the policy may read.
    grants: Grants,
    /// The harness entries that discovery reports on.
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
    /// How the CLI launches a harness run.
    setup: RunSetup,
    /// Every harness run of the evaluation, in order, for the notice.
    runs: Vec<RunRecord>,
    /// The runs that the runner launched, which the budget counts.
    launched: u64,
    /// The run in progress. A stop drops the call that waits for it, so the
    /// record of the run stays here for the teardown to find.
    flight: Option<Flight>,
}

/// A harness run that the reader thread executes.
struct Flight {
    stop: StopRequest,
    outcome: oneshot::Receiver<Executed>,
}

/// How long a stop waits for a run to settle. This is a ceiling for a runner
/// that does not report, and no schedule: a stop that the runner confirms ends
/// within its kill-grace and one second. The ceiling adds the runner's bounds
/// for a reap that fails (two seconds) and for the confirmation of the group
/// (one second), so that it never refuses a stop that the runner would still
/// have reported.
const SETTLE: std::time::Duration =
    std::time::Duration::from_secs(crate::run::ESCALATION.kill_grace.as_secs() + 2 + 1);

impl Host {
    pub(super) fn new(
        reader: Reader,
        bounds: Bounds,
        deadline: Instant,
        around: Surroundings,
    ) -> Self {
        let Surroundings {
            cwd,
            grants,
            directory,
            caller,
            setup,
        } = around;
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
            setup,
            runs: Vec::new(),
            launched: 0,
            flight: None,
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

    pub(super) fn into_record(self) -> (Vec<Module>, Vec<Read>, Vec<RunRecord>) {
        (self.modules, self.reads, self.runs)
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
            Some("harnesses") => Ok(self.harnesses(phase, &site).await),
            Some("remaining") => Ok(self.remaining()),
            Some("run_harness") => {
                let request = Request::from_frame(frame)?;
                Ok(self.run_harness(request, phase, &site).await)
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

    /// `ravel::remaining`: what the evaluation still has.
    fn remaining(&self) -> Answer {
        let ms = self
            .deadline
            .saturating_duration_since(Instant::now())
            .as_millis();
        let sources = self.bounds.sources.saturating_sub(self.source_count());
        Answer::Reply(encode(&json!({
            "type": "reply", "status": "ok", "value": {
                "ms": u64::try_from(ms).unwrap_or(u64::MAX),
                "harnessRuns": self.bounds.harness_runs.saturating_sub(self.launched),
                "sources": sources,
            },
        })))
    }

    /// The sources the evaluation has used: the measured reads, the reads that
    /// decoded to nothing, and the caller's document.
    fn source_count(&self) -> u64 {
        self.reads.len() as u64 + self.undecoded + u64::from(self.caller.is_some())
    }

    /// `ravel::run_harness`: a harness that the owner named, run through the
    /// runner. A run that cannot start is data, and so are the deadline and the
    /// output bound of one run. The budget and a cleanup failure end the
    /// evaluation.
    async fn run_harness(&mut self, request: Request, phase: Phase, site: &Site) -> Answer {
        let reader_gone = || {
            Answer::End(Box::new(site.failure(
                "worker_failed",
                phase,
                "the reader thread ended before it answered".to_owned(),
            )))
        };
        let Some(entry) = self.directory.harnesses.get(&request.harness) else {
            let names: Vec<&str> = self
                .directory
                .harnesses
                .keys()
                .map(String::as_str)
                .collect();
            let known = if names.is_empty() {
                "the owner settings name no harness".to_owned()
            } else {
                format!("the owner settings name {}", names.join(", "))
            };
            return self.unavailable(
                &request.harness,
                Unavailable::new(
                    "unknown_harness",
                    format!("there is no harness entry {:?}: {known}", request.harness),
                ),
            );
        };

        let (name, program, cwd) = (
            request.harness.clone(),
            entry.program.clone(),
            self.cwd.clone(),
        );
        let path = std::env::var_os("PATH");
        let found = self
            .reader
            .run(move || harness::resolve(&name, &program, &cwd, path.as_deref()))
            .await;
        let resolved = match found {
            Ok(Ok(resolved)) => resolved,
            Ok(Err(unavailable)) => return self.unavailable(&request.harness, unavailable),
            Err(_) => return reader_gone(),
        };

        // The run uses one run of the budget when the runner launches it, and
        // a call with none left ends the evaluation before that.
        if self.launched >= self.bounds.harness_runs {
            let mut failure = site.failure(
                "harness_budget_exhausted",
                phase,
                format!(
                    "ravel::run_harness of {} would be harness run {}, over the budget of {}",
                    request.harness,
                    self.launched + 1,
                    self.bounds.harness_runs
                ),
            );
            failure.actual = Some(self.launched + 1);
            return Answer::End(Box::new(failure));
        }

        let remaining = self.deadline.saturating_duration_since(Instant::now());
        let limit = request.timeout_ms.map_or(remaining, |ms| {
            std::time::Duration::from_millis(ms).min(remaining)
        });
        let stop = StopRequest::new();
        let timed_out = StopRequest::new();
        let plan = Plan {
            name: request.harness.clone(),
            resolved,
            args: request.args,
            stdin: request.stdin.unwrap_or_default().into_bytes(),
            cwd: self.cwd.clone(),
            setup: self.setup.clone(),
        };
        let outcome = self.reader.run({
            let (stop, timed_out) = (stop.clone(), timed_out.clone());
            move || harness::execute(plan, &stop, &timed_out)
        });
        let flight = self.flight.insert(Flight { stop, outcome });
        let deadline = tokio::time::sleep(limit);
        tokio::pin!(deadline);
        let mut expired = false;
        let executed = loop {
            tokio::select! {
                done = &mut flight.outcome => break done,
                () = &mut deadline, if !expired => {
                    expired = true;
                    timed_out.request();
                    flight.stop.request();
                }
            }
        };
        self.flight = None;
        match executed {
            Err(_) => reader_gone(),
            Ok(Executed::Finished(done)) => {
                self.launched += u64::from(done.launched);
                self.runs.push(done.record);
                Answer::Reply(encode(
                    &json!({ "type": "reply", "status": "ok", "value": done.value }),
                ))
            }
            Ok(Executed::Cleanup { launched, reports }) => {
                self.launched += u64::from(launched);
                Answer::End(Box::new(site.failure(
                    "harness_cleanup_failed",
                    phase,
                    cleanup_message(&reports),
                )))
            }
        }
    }

    /// The answer for a run that did not start, and its entry in the notice.
    fn unavailable(&mut self, harness: &str, why: Unavailable) -> Answer {
        self.runs.push(why.record(harness));
        Answer::Reply(encode(
            &json!({ "type": "reply", "status": "ok", "value": why.value() }),
        ))
    }

    /// Stop a run that is still in progress, and wait for the runner to clear its
    /// group. A stop of the evaluation drops the call that waits for the run, and
    /// the run goes on until this asks it to stop. The refusal of a left-over
    /// replaces `conversation`, because a process that survives is worse than
    /// the deadline or the signal that found it.
    pub(super) async fn settle(&mut self, conversation: Conversation) -> Conversation {
        let Some(flight) = self.flight.take() else {
            return conversation;
        };
        flight.stop.request();
        match tokio::time::timeout(SETTLE, flight.outcome).await {
            Ok(Ok(Executed::Cleanup { reports, .. })) => {
                Conversation::Refused(cleanup_refusal(&reports))
            }
            Ok(_) => conversation,
            Err(_) => Conversation::Refused(cleanup_refusal(&[format!(
                "the runner did not settle the run within {} seconds of the stop",
                SETTLE.as_secs()
            )])),
        }
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
