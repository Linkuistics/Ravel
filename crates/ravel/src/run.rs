//! Compose the policy choice with its own ephemeral result channel and runner.
use std::ffi::{OsStr, OsString};
use std::io::{self, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};
use std::path::{Path, PathBuf};
use std::time::Duration;

use keyed_launch::{Argv, End, Escalation, Group, Launch, LaunchEvent};
use ravel::{SessionChannel, EXIT_FILE_VARIABLE};
use serde_json::json;

use crate::choice::{self, Selected};
use crate::cli::RunArgs;
use crate::refusal::{Failure, Refusal, Stage, EXIT_MALFORMED, EXIT_WORKER};
use crate::signal_state;

const ESCALATION: Escalation = Escalation {
    grace: Duration::from_secs(2),
    kill_grace: Duration::from_secs(5),
};

fn failed(code: &'static str, message: impl std::fmt::Display) -> Refusal {
    Refusal::new(
        code,
        Stage::Exec,
        EXIT_WORKER,
        message.to_string(),
        "correct the named process or temporary-directory error and retry start-session",
    )
}

pub fn run(args: &RunArgs) -> Result<(), Failure> {
    signal_state::recorded()?;
    let exit_dir = args.exit_dir.as_deref().map(exit_dir).transpose()?;
    let prepared = choice::prepare(&args.selection)?;
    let confined = args
        .confine
        .then(|| {
            crate::confinement::Confined::prepare(
                &prepared,
                exit_dir.as_deref(),
                &args.runtime_read,
            )
        })
        .transpose()?;
    let selected = choice::choose_prepared(prepared);
    let Selected {
        choice,
        handlers,
        source,
    } = match selected {
        Ok(selected) => selected,
        Err(failure) => {
            let cleanup = confined.map_or(Ok(()), |confined| {
                confined
                    .remove()
                    .map_err(|error| failed("session_cleanup_failed", error))
            });
            return settle(
                Err(failure),
                None,
                vec![cleanup],
                crate::cancellation::received(),
                "selection",
                &mut io::stderr(),
            )
            .map(|_| ());
        }
    };
    let mut channel = None;
    let mut output = None;
    let mut ended = None;
    let mut runner_signal = None;
    // Every path through this closure joins explicit cleanup below, including
    // allocation/reservation failures and cancellation before runner startup.
    let result = (|| -> Result<String, Failure> {
        output =
            Some(ResultOutput::reserve().map_err(|error| failed("output_unavailable", error))?);
        let parent = exit_dir.as_deref().or_else(|| {
            confined
                .as_ref()
                .map(crate::confinement::Confined::exit_dir)
        });
        let temporary = std::env::temp_dir();
        let allocated = channel.insert(
            SessionChannel::allocate(parent.unwrap_or(&temporary))
                .map_err(|error| failed("session_channel_unavailable", error))?,
        );
        if args.json {
            eprintln!(
                "{}",
                json!({"schemaVersion": 1, "selection": {
            "provider": choice.command.provider, "model": choice.command.model,
            "effort": choice.command.effort, "reason": choice.command.reason,
        }, "policy": {"path": choice.entry.path, "version": choice.version},
        "worker": {"path": choice.worker.path, "version": choice.worker.package_version, "buildId": choice.worker.build_id, "bunVersion": choice.worker.bun_version},
        "selectionMs": choice.elapsed.as_millis(), "context": choice.context.as_ref().map(|context| context.to_json(true)),
        "bounds": choice.inputs.limits.to_json(), "policyEnvironment": choice.inputs.grants.to_json(),
        "resolution": choice.executable.to_text(),
        "promptSource": match &choice.inputs.prompt.source {
            crate::inputs::PromptSource::Argument => "argument".to_owned(),
            crate::inputs::PromptSource::File(path) => path.display().to_string(),
        }, "diagnostics": choice.diagnostics.to_json()})
            );
        } else {
            eprint!("{}", choice.diagnostics.to_text());
            eprintln!(
                "ravel: starting provider {}, model {}, effort {} for kind {:?}",
                choice.command.provider,
                choice.command.model,
                choice.command.effort,
                choice.inputs.kind
            );
        }

        if let Some(signal) = handlers.block_and_check() {
            return Err(crate::cancellation::refusal(signal, &source).into());
        }
        let argv = Argv::new(
            OsString::from(&choice.executable.path),
            choice.command.args.iter().map(OsString::from).collect(),
        )
        .with_arg0(OsString::from(&choice.executable.program));
        let environment = confined
            .as_ref()
            .map_or_else(Vec::new, crate::confinement::Confined::environment);
        let grant: Vec<_> = environment
            .iter()
            .map(|(name, value)| (name.as_os_str(), value.as_os_str()))
            .collect();
        let scrub: Vec<_> = [
            "HARNESS_DISPATCH_EXIT_FILE",
            "HARNESS_DISPATCH_RUN_ID",
            "HARNESS_DISPATCH_STATE_DIR",
            "RAVEL_EXIT_FILE",
            "RAVEL_RUN_ID",
            "RAVEL_STATE_DIR",
        ]
        .map(OsStr::new)
        .into();
        let entry = signal_state::entry();
        let launch = Launch {
            argv: &argv,
            channel: Some((allocated.channel(), EXIT_FILE_VARIABLE)),
            scrub: &scrub,
            grant: &grant,
            transparent: Some(&entry),
            cwd: confined
                .as_ref()
                .map(|confined| confined.writable[0].as_path()),
            escalation: ESCALATION,
        };
        let mut started = false;
        let mut observe = |event| {
            if event == LaunchEvent::Started {
                started = true;
                let _ = signal_state::restore_mask();
            }
        };
        let supervised = match &confined {
            Some(confined) => keyed_launch::run_confined_observed(
                launch,
                &keyed_launch::FilesystemGrants {
                    writable: &confined.writable,
                    runtime_read: &confined.reads(),
                },
                &mut observe,
            ),
            None => keyed_launch::run_observed(launch, &mut observe),
        };
        let supervision = supervised.map_err(|error| {
            let (failure, observed) = supervision_failure(
                error.to_string(),
                error.raw_os_error(),
                error.consumed_signal(),
                started,
            );
            runner_signal = observed;
            failure
        })?;
        runner_signal = match supervision.end {
            End::Interrupted { signal } => crate::cancellation::Signal::of(signal),
            _ => None,
        };
        let ending = ended.insert(supervision);
        let result = allocated
            .result_after(ending)
            .map_err(|error| failed("session_result_invalid", error))?;
        result.ok_or_else(|| {
            Refusal::new(
                "session_result_missing",
                Stage::Exec,
                1,
                format!(
                    "the managed process ended without a result ({})",
                    ending.status
                ),
                "have the managed process call ravel end-session RESULT before it exits",
            )
            .into()
        })
    })();
    // A failed confined command may not have installed runner handlers.
    // Restore while whichever handler owns the transition is still installed,
    // then inspect both latches after dispositions have returned to entry.
    let mask =
        signal_state::restore_mask().map_err(|error| failed("signal_state_unavailable", error));
    let channel_cleanup = channel.map_or(Ok(()), |channel| {
        channel
            .close()
            .map_err(|error| failed("session_cleanup_failed", error))
    });
    let private_cleanup = confined.map_or(Ok(()), |confined| {
        confined
            .remove()
            .map_err(|error| failed("session_cleanup_failed", error))
    });
    let restore = output.as_ref().map_or(Ok(()), |output| {
        output
            .restore()
            .map_err(|error| failed("output_unavailable", error))
    });
    drop(output);
    drop(handlers);
    let late_signal = keyed_launch::take_interrupt().and_then(crate::cancellation::Signal::of);
    let signal = join_cancellation(crate::cancellation::received(), runner_signal, late_signal);
    let result = settle(
        result,
        ended.as_ref(),
        vec![mask, channel_cleanup, private_cleanup, restore],
        signal,
        &source,
        &mut io::stderr(),
    )?;
    io::stdout()
        .write_all(result.as_bytes())
        .and_then(|()| io::stdout().flush())
        .map_err(|error| failed("result_write_failed", error))?;
    Ok(())
}

// Keep runner outcome data before converting its opaque error. The numeric
// fields are the runner's accessors; tests exercise this same conversion.
fn supervision_failure(
    message: String,
    errno: Option<i32>,
    consumed_signal: Option<i32>,
    started: bool,
) -> (Refusal, Option<crate::cancellation::Signal>) {
    let exit = if started {
        EXIT_WORKER
    } else if errno == Some(libc::ENOENT) {
        127
    } else {
        126
    };
    (
        Refusal::new(
            "supervision_failed",
            Stage::Exec,
            exit,
            message,
            "check the selected program and its execution environment",
        ),
        consumed_signal.and_then(crate::cancellation::Signal::of),
    )
}

fn join_cancellation(
    front: Option<crate::cancellation::Signal>,
    outcome: Option<crate::cancellation::Signal>,
    late: Option<crate::cancellation::Signal>,
) -> Option<crate::cancellation::Signal> {
    front.or(outcome).or(late)
}

/// Report all cleanup/group failures before selecting cancellation or an error.
fn settle(
    result: Result<String, Failure>,
    ended: Option<&keyed_launch::Ended>,
    cleanups: Vec<Result<(), Refusal>>,
    signal: Option<crate::cancellation::Signal>,
    source: &str,
    diagnostics: &mut dyn io::Write,
) -> Result<String, Failure> {
    if signal.is_some() {
        if let Err(error) = &result {
            let _ = diagnostics.write_all(error.to_text().as_bytes());
        }
    }
    for error in cleanups.iter().filter_map(|cleanup| cleanup.as_ref().err()) {
        let _ = writeln!(diagnostics, "ravel: {}", error.message);
    }
    let group = ended.and_then(|ended| match ended.group {
        Group::Present { pgid } => Some(pgid),
        Group::Gone => None,
    });
    if let Some(pgid) = group {
        let _ = writeln!(diagnostics, "ravel: process group {pgid} survived cleanup");
    }
    if let Some(signal) = signal {
        return Err(Refusal::new(
            "session_cancelled",
            Stage::Exec,
            signal.exit(),
            format!(
                "session selected by {source} was cancelled by {}",
                signal.name()
            ),
            "retry the same command if cancellation was unintended",
        )
        .signal(signal)
        .into());
    }
    let result = result?;
    if let Some(error) = cleanups.into_iter().find_map(Result::err) {
        return Err(error.into());
    }
    if let Some(pgid) = group {
        return Err(failed(
            "process_group_survived",
            format!("process group {pgid} survived cleanup"),
        )
        .into());
    }
    Ok(result)
}

fn exit_dir(path: &Path) -> Result<PathBuf, Refusal> {
    let refuse = |error: String| {
        Refusal::new(
            "exit_dir_unusable",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("--exit-dir {}: {error}", path.display()),
            "name an existing writable directory or omit --exit-dir",
        )
    };
    let canonical = path
        .canonicalize()
        .map_err(|error| refuse(error.to_string()))?;
    if !canonical.is_dir() {
        return Err(refuse("not a directory".to_owned()));
    }
    Ok(canonical)
}

struct ResultOutput(OwnedFd);
impl ResultOutput {
    fn reserve() -> io::Result<Self> {
        io::stdout().flush()?;
        // SAFETY: inspect only the descriptor's access mode before opening
        // resources that could occupy a missing standard descriptor.
        let flags = unsafe { libc::fcntl(1, libc::F_GETFL) };
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }
        if flags & libc::O_ACCMODE == libc::O_RDONLY {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "result stdout is not writable",
            ));
        }
        // SAFETY: duplicate valid stdio descriptors; the saved fd is owned
        // once, CLOEXEC, and never exposed to the managed process.
        let saved = unsafe { libc::fcntl(1, libc::F_DUPFD_CLOEXEC, 10) };
        if saved < 0 {
            return Err(io::Error::last_os_error());
        }
        let output = Self(unsafe { OwnedFd::from_raw_fd(saved) });
        if unsafe { libc::dup2(2, 1) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(output)
    }
    fn restore(&self) -> io::Result<()> {
        // SAFETY: this owned descriptor remains live until Drop.
        if unsafe { libc::dup2(self.0.as_raw_fd(), 1) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
impl Drop for ResultOutput {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cancellation::Signal;
    use std::os::unix::process::ExitStatusExt as _;

    #[test]
    fn cancellation_overrides_prelaunch_failure_and_reports_every_cleanup_error() {
        let signal = Signal::of(libc::SIGTERM).unwrap();
        let mut diagnostics = Vec::new();
        let result = settle(
            Err(Failure::with_diagnostics(
                failed("output_unavailable", "stdout is closed"),
                crate::refusal::Diagnostics {
                    stdout: b"owner note".to_vec(),
                    stderr: b"owner warning".to_vec(),
                },
            )),
            None,
            vec![
                Err(failed("cleanup", "channel cannot be removed")),
                Err(failed("cleanup", "sandbox cannot be removed")),
            ],
            Some(signal),
            "policy.ts",
            &mut diagnostics,
        )
        .unwrap_err();
        assert_eq!(result.refusal.signal, Some(signal));
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("channel cannot be removed"));
        assert!(diagnostics.contains("sandbox cannot be removed"));
        assert!(diagnostics.contains("stdout is closed"));
        assert!(diagnostics.contains("policy stdout: owner note"));
        assert!(diagnostics.contains("policy stderr: owner warning"));
    }

    #[test]
    fn cancellation_reports_a_surviving_group_and_suppresses_the_result() {
        let mut diagnostics = Vec::new();
        let ended = keyed_launch::Ended {
            end: End::Interrupted {
                signal: libc::SIGTERM,
            },
            status: std::process::ExitStatus::from_raw(libc::SIGTERM),
            elapsed: Duration::ZERO,
            signalled: true,
            group: Group::Present { pgid: 12345 },
        };
        let result = settle(
            Ok("must not escape".to_owned()),
            Some(&ended),
            vec![],
            crate::cancellation::Signal::of(libc::SIGTERM),
            "policy.ts",
            &mut diagnostics,
        )
        .unwrap_err();
        assert_eq!(result.refusal.signal.unwrap().name(), "SIGTERM");
        assert!(String::from_utf8(diagnostics).unwrap().contains("12345"));
    }

    // Erasing the consumed signal during conversion, changing join order, or
    // dropping the primary error when cancellation wins must fail these cases.
    #[test]
    fn runner_error_conversion_joins_cancellation_before_no_ending_settlement() {
        for (front, runner, late, want) in [
            (
                Some(libc::SIGINT),
                Some(libc::SIGHUP),
                Some(libc::SIGTERM),
                Some(libc::SIGINT),
            ),
            (
                None,
                Some(libc::SIGHUP),
                Some(libc::SIGTERM),
                Some(libc::SIGHUP),
            ),
            (None, None, Some(libc::SIGTERM), Some(libc::SIGTERM)),
            (None, None, None, None),
        ] {
            let (failure, observed) = supervision_failure(
                "original observation failure; distinct emergency wait failure; reported recovery failure".into(),
                None, runner, true,
            );
            assert_eq!(observed, runner.and_then(Signal::of));
            let joined = join_cancellation(
                front.and_then(Signal::of),
                observed,
                late.and_then(Signal::of),
            );
            let mut diagnostics = Vec::new();
            let result = settle(
                Err(failure.into()),
                None,
                vec![
                    Err(failed("cleanup", "mask restore failure")),
                    Err(failed("cleanup", "channel close failure")),
                    Err(failed("cleanup", "private directory removal failure")),
                    Err(failed("cleanup", "output restore failure")),
                ],
                joined,
                "policy.ts",
                &mut diagnostics,
            )
            .unwrap_err();
            assert_eq!(result.refusal.signal, want.and_then(Signal::of));
            assert_eq!(result.refusal.stage, Stage::Exec);
            let text = String::from_utf8(diagnostics).unwrap();
            for report in [
                "mask restore failure",
                "channel close failure",
                "private directory removal failure",
                "output restore failure",
            ] {
                assert!(text.contains(report));
            }
            if want.is_some() {
                for report in [
                    "original observation failure",
                    "distinct emergency wait failure",
                    "reported recovery failure",
                ] {
                    assert!(text.contains(report));
                }
                assert_eq!(result.refusal.code, "session_cancelled");
                assert_eq!(
                    result.to_json()["error"]["signal"],
                    want.and_then(Signal::of).unwrap().name()
                );
            } else {
                assert_eq!(result.refusal.code, "supervision_failed");
                assert_eq!(result.refusal.exit, 5);
                assert!(result
                    .refusal
                    .message
                    .contains("original observation failure"));
            }
            eprintln!("checked front={front:?} runner={runner:?} late={late:?} selected={want:?}; no ending/result, original diagnostics and four cleanup reports asserted");
        }
    }

    #[test]
    fn runner_error_conversion_preserves_start_classifications_and_success_settlement() {
        for (started, errno, exit) in [
            (false, Some(libc::ENOENT), 127),
            (false, Some(libc::EACCES), 126),
            (true, None, 5),
        ] {
            let (failure, signal) =
                supervision_failure("original failure".into(), errno, None, started);
            assert_eq!(signal, None);
            let failure = settle(
                Err(failure.into()),
                None,
                vec![],
                signal,
                "policy.ts",
                &mut Vec::new(),
            )
            .unwrap_err();
            assert_eq!(failure.refusal.exit, exit);
            assert_eq!(failure.refusal.code, "supervision_failed");
            assert_eq!(failure.refusal.stage, Stage::Exec);
            assert_eq!(failure.refusal.message, "original failure");
        }
        let ended = keyed_launch::Ended {
            end: End::Exited,
            status: std::process::ExitStatus::from_raw(0),
            elapsed: Duration::ZERO,
            signalled: true,
            group: Group::Gone,
        };
        for value in ["", "snowman ☃", "first\nsecond\n"] {
            assert_eq!(
                settle(
                    Ok(value.into()),
                    Some(&ended),
                    vec![Ok(())],
                    None,
                    "policy.ts",
                    &mut Vec::new()
                )
                .unwrap(),
                value
            );
        }
    }
}
