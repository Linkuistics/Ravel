//! The harness runs of a policy (`docs/specs/policy-evaluation.md`, *Harness
//! runs*, *Evaluation budgets and their composition* and *Re-entry and
//! completion authority*).
//!
//! A run executes the program of a harness entry through the runner of
//! `keyed-launch`. This module holds what the runner needs and what comes back.
//! That is the request of the script, the plan of one launch, and the result
//! map. The reader thread calls [`resolve`] and [`execute`], because both look
//! at the filesystem or wait for a child. [`super::host`] owns the budget, the
//! deadline of one run and the stop.
//!
//! Every outcome of a run is data for the script, except a cleanup failure. A
//! cleanup failure is a process group that survives the run, or a private
//! directory that stays.

use std::ffi::{OsStr, OsString};
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};

use keyed_launch::{
    Argv, Captured, CapturedLaunch, End, EntrySignals, FilesystemGrants, Group, LaunchEvent,
    StopRequest,
};
use serde_json::{Value, json};

#[cfg(test)]
use crate::completion::{EXIT_FILE_VARIABLE, SessionChannel};
use crate::confinement::{Basis, Confined};
use crate::environment::{CONFIG_RUN, LAUNCH_CONTROLS};
use crate::program;
use crate::refusal::{EXIT_WORKER, Refusal, Stage};
use crate::run::ESCALATION;
#[cfg(not(test))]
use ravel::{EXIT_FILE_VARIABLE, SessionChannel};

/// The most that each output stream of a run keeps. The first byte beyond it
/// stops the run.
pub const OUTPUT_BYTES: usize = 1024 * 1024;

/// How the Ravel CLI launches a harness run. It takes the signal state to hand
/// on, and what `--confine` and `--exit-dir` said on the command line.
#[derive(Clone, Debug, Default)]
pub struct RunSetup {
    /// The signal state the CLI inherited, which each run starts with.
    pub entry: EntrySignals,
    /// Present when the caller passed `--confine`.
    pub confine: Option<ConfineSetup>,
    /// The `--exit-dir`, beneath which the channel of a run is allocated.
    pub exit_dir: Option<PathBuf>,
}

/// What a confined run is granted, and what it is kept from.
#[derive(Clone, Debug)]
pub struct ConfineSetup {
    pub basis: Basis,
    pub runtime_read: Vec<PathBuf>,
}

/// A request of the script, as the worker checked and sent it.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub harness: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub timeout_ms: Option<u64>,
}

impl Request {
    /// The request in a `run_harness` host frame. The worker checked every
    /// field, so a frame that fails here broke the protocol.
    pub fn from_frame(frame: &Value) -> Result<Self, String> {
        let harness = frame
            .get("harness")
            .and_then(Value::as_str)
            .ok_or("a run_harness frame has no harness name")?
            .to_owned();
        let args = frame
            .get("args")
            .and_then(Value::as_array)
            .ok_or("a run_harness frame has no args array")?
            .iter()
            .map(|arg| arg.as_str().map(str::to_owned))
            .collect::<Option<Vec<_>>>()
            .ok_or("a run_harness frame has an argument that is no string")?;
        let stdin = match frame.get("stdin") {
            None | Some(Value::Null) => None,
            Some(Value::String(text)) => Some(text.clone()),
            Some(_) => return Err("a run_harness frame has a stdin that is no string".to_owned()),
        };
        let timeout_ms = match frame.get("timeoutMs") {
            None | Some(Value::Null) => None,
            Some(value) => Some(
                value
                    .as_u64()
                    .filter(|&ms| ms >= 1)
                    .ok_or("a run_harness frame has a timeoutMs that is no positive integer")?,
            ),
        };
        Ok(Request {
            harness,
            args,
            stdin,
            timeout_ms,
        })
    }
}

/// One run, as the notice lists it. An LLM call is one of these.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunRecord {
    pub harness: String,
    pub status: &'static str,
    pub exit_code: Option<i32>,
    pub signal: Option<String>,
    pub elapsed_ms: u64,
    pub sent_result: bool,
}

impl RunRecord {
    pub fn to_json(&self) -> Value {
        json!({
            "harness": self.harness,
            "status": self.status,
            "exitCode": self.exit_code,
            "signal": self.signal,
            "elapsedMs": self.elapsed_ms,
            "sentResult": self.sent_result,
        })
    }
}

/// Why a run did not start, as the script reads it in `error`.
#[derive(Debug, PartialEq, Eq)]
pub struct Unavailable {
    pub code: &'static str,
    pub message: String,
}

impl Unavailable {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Unavailable {
            code,
            message: message.into(),
        }
    }

    /// The entry of the notice for a run that never started.
    pub fn record(&self, harness: &str) -> RunRecord {
        RunRecord {
            harness: harness.to_owned(),
            status: "unavailable",
            exit_code: None,
            signal: None,
            elapsed_ms: 0,
            sent_result: false,
        }
    }

    /// The result map of a run that never started.
    pub fn value(&self) -> Value {
        result_map(
            "unavailable",
            (None, None),
            ("", ""),
            None,
            0,
            Some(json!({ "code": self.code, "message": self.message })),
        )
    }
}

/// What a finished call left: the map for the script, the entry for the notice,
/// and whether the runner launched the harness.
pub struct Finished {
    pub value: Value,
    pub record: RunRecord,
    pub launched: bool,
}

/// How a call to [`execute`] ended.
pub enum Executed {
    /// The run is over, or it never started.
    Finished(Finished),
    /// The run left something behind: a process group that survived, a child
    /// that was not reaped, or a private directory that stayed. This ends the
    /// evaluation.
    Cleanup {
        launched: bool,
        reports: Vec<String>,
    },
}

/// The program of the entry, as the runner launches it.
pub struct Resolved {
    program: String,
    path: String,
}

/// Find the program of `name` as a selected program is found. The lookup is a
/// filesystem call, so the reader thread calls it.
pub fn resolve(
    name: &str,
    entry_program: &str,
    cwd: &Path,
    path: Option<&OsStr>,
) -> Result<Resolved, Unavailable> {
    match program::resolve(
        entry_program,
        &format!("the harness entry {name}"),
        cwd,
        path,
    ) {
        Ok(found) => Ok(Resolved {
            program: found.program,
            path: found.path,
        }),
        Err(refusal) if refusal.code == "program_not_found" => Err(Unavailable::new(
            "harness_not_found",
            format!("the program {entry_program:?} of the harness {name} was not found"),
        )),
        Err(_) => Err(Unavailable::new(
            "harness_not_executable",
            format!("the program {entry_program:?} of the harness {name} cannot be executed"),
        )),
    }
}

/// Everything one launch needs, owned so that the reader thread can run it.
pub struct Plan {
    pub name: String,
    pub resolved: Resolved,
    pub args: Vec<String>,
    pub stdin: Vec<u8>,
    pub cwd: PathBuf,
    pub setup: RunSetup,
}

/// Launch the run and wait for its end. `stop` asks the runner to stop it, and
/// `timed_out` says that the stop is the deadline of the run.
///
/// The run receives the environment of a managed process, or the confined
/// environment, and the marker. It receives a result channel of its own, whose
/// path replaces `RAVEL_EXIT_FILE`.
pub fn execute(plan: Plan, stop: &StopRequest, timed_out: &StopRequest) -> Executed {
    let Plan {
        name,
        resolved,
        args,
        stdin,
        cwd,
        setup,
    } = plan;
    let refuse = |unavailable: Unavailable| {
        Executed::Finished(Finished {
            record: unavailable.record(&name),
            value: unavailable.value(),
            launched: false,
        })
    };
    let unlaunched = |message: String| {
        refuse(Unavailable::new(
            "harness_launch_failed",
            format!("the harness {name} could not be launched: {message}"),
        ))
    };

    // The private run directory comes first, so that the channel lives in it.
    let confined = match &setup.confine {
        Some(confine) => {
            match Confined::prepare_for(
                &confine.basis,
                setup.exit_dir.as_deref(),
                &confine.runtime_read,
                false,
            ) {
                Ok(confined) => Some(confined),
                Err(refusal) => return unlaunched(refusal.message.clone()),
            }
        }
        None => None,
    };
    let remove = |confined: Option<Confined>, reports: &mut Vec<String>| {
        if let Some(Err(error)) = confined.map(Confined::remove) {
            reports.push(error);
        }
    };
    let temporary = std::env::temp_dir();
    let parent = setup
        .exit_dir
        .as_deref()
        .or_else(|| confined.as_ref().map(Confined::exit_dir))
        .unwrap_or(&temporary);
    let channel = match SessionChannel::allocate(parent) {
        Ok(channel) => channel,
        Err(error) => {
            let mut reports = Vec::new();
            remove(confined, &mut reports);
            return if reports.is_empty() {
                unlaunched(format!("cannot allocate its result channel: {error}"))
            } else {
                Executed::Cleanup {
                    launched: false,
                    reports,
                }
            };
        }
    };

    let argv = Argv::new(
        OsString::from(&resolved.path),
        args.iter().map(OsString::from).collect(),
    )
    .with_arg0(OsString::from(&resolved.program));
    let scrub: Vec<&OsStr> = LAUNCH_CONTROLS.map(OsStr::new).into();
    let mut granted: Vec<(OsString, OsString)> =
        vec![(OsString::from(CONFIG_RUN), OsString::from("1"))];
    if let Some(confined) = &confined {
        granted.extend(confined.environment());
    }
    let grant: Vec<(&OsStr, &OsStr)> = granted
        .iter()
        .map(|(name, value)| (name.as_os_str(), value.as_os_str()))
        .collect();
    let reads = confined.as_ref().map(Confined::reads);
    let grants = confined
        .as_ref()
        .zip(reads.as_ref())
        .map(|(confined, reads)| (confined.writable.as_slice(), reads.as_slice()));
    let filesystem = grants.map(|(writable, runtime_read)| FilesystemGrants {
        writable,
        runtime_read,
    });
    let working = confined
        .as_ref()
        .map_or(cwd.as_path(), |confined| confined.writable[0].as_path());

    let mut launched = false;
    let ran = keyed_launch::run_captured(
        CapturedLaunch {
            argv: &argv,
            channel: (channel.channel(), EXIT_FILE_VARIABLE),
            scrub: &scrub,
            grant: &grant,
            transparent: Some(&setup.entry),
            cwd: Some(working),
            escalation: ESCALATION,
            stdin: &stdin,
            limit: OUTPUT_BYTES,
            stop,
        },
        filesystem.as_ref(),
        &mut |event| {
            if event == LaunchEvent::Started {
                launched = true;
            }
        },
    );

    let mut reports = Vec::new();
    let captured = match ran {
        Ok(captured) => captured,
        Err(error) => {
            let message = error.to_string();
            if launched {
                // The job existed, and the runner could not settle it.
                reports.push(message.clone());
            }
            if let Err(error) = channel.close() {
                reports.push(format!("cannot remove the result channel: {error}"));
            }
            remove(confined, &mut reports);
            return if reports.is_empty() {
                unlaunched(message)
            } else {
                Executed::Cleanup { launched, reports }
            };
        }
    };
    reports.extend(captured.ended.operation_reports.iter().cloned());
    let group_survived = match captured.ended.group {
        Group::Gone => false,
        Group::Present { pgid } => {
            reports.push(format!("process group {pgid} survived cleanup"));
            true
        }
    };
    // A group that is still there never yields a result.
    let result = if group_survived {
        None
    } else {
        channel.result_after(&captured.ended).ok().flatten()
    };
    let mut cleanup = Vec::new();
    if let Err(error) = channel.close() {
        cleanup.push(format!("cannot remove the result channel: {error}"));
    }
    remove(confined, &mut cleanup);
    if group_survived || !cleanup.is_empty() {
        reports.extend(cleanup);
        return Executed::Cleanup { launched, reports };
    }
    Executed::Finished(finish(&name, &captured, result, timed_out.requested()))
}

/// The map and the notice entry of a run that the runner settled.
fn finish(name: &str, captured: &Captured, result: Option<String>, timed_out: bool) -> Finished {
    let status = &captured.ended.status;
    let stopped = captured.ended.end == End::Escalated;
    let label = if captured.overflow {
        "output_limit"
    } else if stopped && timed_out {
        "timed_out"
    } else if status.signal().is_some() {
        "signalled"
    } else {
        "exited"
    };
    let exit_code = status.code();
    let signal = status.signal().map(signal_name);
    let elapsed_ms = u64::try_from(captured.ended.elapsed.as_millis()).unwrap_or(u64::MAX);
    let stdout = String::from_utf8_lossy(&captured.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&captured.stderr).into_owned();
    let value = result_map(
        label,
        (exit_code, signal.as_deref()),
        (&stdout, &stderr),
        result.as_deref(),
        elapsed_ms,
        None,
    );
    Finished {
        value,
        record: RunRecord {
            harness: name.to_owned(),
            status: label,
            exit_code,
            signal,
            elapsed_ms,
            sent_result: result.is_some(),
        },
        launched: true,
    }
}

fn result_map(
    status: &str,
    (exit_code, signal): (Option<i32>, Option<&str>),
    (stdout, stderr): (&str, &str),
    result: Option<&str>,
    elapsed_ms: u64,
    error: Option<Value>,
) -> Value {
    json!({
        "status": status,
        "exitCode": exit_code,
        "signal": signal,
        "stdout": stdout,
        "stderr": stderr,
        "result": result,
        "elapsedMs": elapsed_ms,
        "error": error,
    })
}

/// The message of the failure that a left-over ends the evaluation with.
pub fn cleanup_message(reports: &[String]) -> String {
    format!(
        "a harness run left something behind after its stop: {}",
        reports.join("; ")
    )
}

/// The refusal for a run that left something behind, when a stop of the
/// evaluation found it. A call that is still in progress cannot return it as an
/// answer.
pub fn cleanup_refusal(reports: &[String]) -> Refusal {
    Refusal::new(
        "harness_cleanup_failed",
        Stage::Evaluation,
        EXIT_WORKER,
        cleanup_message(reports),
        "stop the named process and remove the named directory by hand, then retry; do not \
         signal a process ID that may have been reused",
    )
}

/// The name of a signal, such as `SIGTERM`.
pub fn signal_name(number: i32) -> String {
    let name = match number {
        libc::SIGHUP => "SIGHUP",
        libc::SIGINT => "SIGINT",
        libc::SIGQUIT => "SIGQUIT",
        libc::SIGILL => "SIGILL",
        libc::SIGTRAP => "SIGTRAP",
        libc::SIGABRT => "SIGABRT",
        libc::SIGBUS => "SIGBUS",
        libc::SIGFPE => "SIGFPE",
        libc::SIGKILL => "SIGKILL",
        libc::SIGUSR1 => "SIGUSR1",
        libc::SIGSEGV => "SIGSEGV",
        libc::SIGUSR2 => "SIGUSR2",
        libc::SIGPIPE => "SIGPIPE",
        libc::SIGALRM => "SIGALRM",
        libc::SIGTERM => "SIGTERM",
        libc::SIGXCPU => "SIGXCPU",
        libc::SIGXFSZ => "SIGXFSZ",
        libc::SIGSYS => "SIGSYS",
        _ => return format!("SIG{number}"),
    };
    name.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_becomes_a_request() {
        let request = Request::from_frame(&json!({
            "harness": "claude", "args": ["-p", "hi"], "stdin": "text", "timeoutMs": 5,
        }))
        .unwrap();
        assert_eq!(
            request,
            Request {
                harness: "claude".to_owned(),
                args: vec!["-p".to_owned(), "hi".to_owned()],
                stdin: Some("text".to_owned()),
                timeout_ms: Some(5),
            }
        );
        let bare = Request::from_frame(&json!({ "harness": "h", "args": [] })).unwrap();
        assert_eq!((bare.stdin, bare.timeout_ms), (None, None));
    }

    #[test]
    fn a_frame_the_worker_would_not_send_is_refused() {
        for frame in [
            json!({ "args": [] }),
            json!({ "harness": "h" }),
            json!({ "harness": "h", "args": [1] }),
            json!({ "harness": "h", "args": [], "stdin": 3 }),
            json!({ "harness": "h", "args": [], "timeoutMs": 0 }),
            json!({ "harness": "h", "args": [], "timeoutMs": "5" }),
        ] {
            assert!(Request::from_frame(&frame).is_err(), "{frame}");
        }
    }

    #[test]
    fn the_signals_of_the_table_have_their_names() {
        assert_eq!(signal_name(libc::SIGTERM), "SIGTERM");
        assert_eq!(signal_name(libc::SIGKILL), "SIGKILL");
        assert_eq!(signal_name(200), "SIG200");
    }

    #[test]
    fn an_unavailable_run_has_the_map_of_the_spec() {
        let value = Unavailable::new("harness_not_found", "no").value();
        assert_eq!(value["status"], "unavailable");
        assert_eq!(value["error"]["code"], "harness_not_found");
        for field in ["exitCode", "signal", "result"] {
            assert!(value[field].is_null(), "{field}");
        }
        assert_eq!(value["stdout"], "");
        assert_eq!(value["elapsedMs"], 0);
    }
}
