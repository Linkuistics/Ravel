//! The front side: launch the role through `worker::supervise` and have it
//! evaluate one entry.
//!
//! The API is small on purpose. [`evaluate`] takes the entry source, the request
//! and the launch description, and returns what the judge needs: the
//! [`Reply`](super::Reply), the identity of the worker and the supervision
//! reports. The leaf that joins this to `start-session` builds the request and
//! judges the reply.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::engine::Entry;
use super::{ARG0, BUILD_ID, Bounds, PROTOCOL, RHAI_VERSION, Reply};
use crate::frame::write_frame;
use crate::limits::Limits;
use crate::refusal::{Diagnostics, EXIT_WORKER, Failure, Refusal, Stage};
use crate::worker::{Bounded, Conversation, Launch, broken_for, receive_for, supervise};

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The remedy for a worker that cannot run or is not this front's pair. The
/// worker is the front's own binary, so a mismatch means the file changed under
/// the running process.
const REBUILD: &str = "reinstall ravel, and start the session again; the policy worker is \
     the ravel binary itself, so it differs only when that file was replaced while ravel ran";

/// The largest hello: a few short fields.
const HELLO_BYTES: usize = 4096;

/// What a reply frame carries beyond its result.
const REPLY_ENVELOPE: usize = 1024;

/// What the worker reported about itself in its hello.
#[derive(Debug)]
pub struct Identity {
    pub path: PathBuf,
    pub package_version: String,
    pub build_id: String,
    pub rhai_version: String,
}

/// An entry the worker evaluated, and what supervising it left behind.
#[derive(Debug)]
pub struct Evaluated {
    pub worker: Identity,
    /// What the worker reported. Judging it is the caller's.
    pub reply: Reply,
    pub diagnostics: Diagnostics,
    pub operational: Vec<String>,
    /// From starting the worker to its reply.
    pub elapsed: Duration,
}

/// The program that is Ravel's own binary, as the role must be launched.
///
/// On Linux it is `/proc/self/exe`, which names the running image even after
/// the file was replaced. On macOS there is no such name, so it is the
/// canonical path of the executable, which names the file now at that path.
pub fn executable() -> Result<PathBuf, Refusal> {
    #[cfg(target_os = "linux")]
    {
        Ok(PathBuf::from("/proc/self/exe"))
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::env::current_exe()
            .and_then(std::fs::canonicalize)
            .map_err(|error| {
                Refusal::new(
                    "worker_missing",
                    Stage::Worker,
                    EXIT_WORKER,
                    format!(
                        "cannot find the ravel executable to launch the policy worker: {error}"
                    ),
                    REBUILD,
                )
            })
    }
}

/// How to launch the role from `program`, with the whole environment it gets.
pub fn launch(program: &Path, environment: Vec<(OsString, OsString)>) -> Launch<'_> {
    Launch {
        program,
        arg0: Some(OsStr::new(ARG0)),
        environment,
        rebuild: REBUILD,
    }
}

/// Start the role, have it evaluate `entry` for `request`, and tear it down.
///
/// `request` holds `kind`, `prompt`, `cwd` and `params`, and may hold `taskFile`,
/// `taskId` and `context`. The worker adds `schemaVersion` and `limits` itself,
/// from its own copy of the bounds. `select_context` is the second argument of
/// `select`: the caller's document with its measurement attached. Without one,
/// `select` receives `request.context`. The whole evaluation counts from the start
/// of the worker, within the selection bound of `limits`.
pub fn evaluate(
    entry: &Entry<'_>,
    request: &Value,
    select_context: Option<&Value>,
    launch: Launch<'_>,
    limits: &Limits,
) -> Result<Evaluated, Failure> {
    let program = launch.program.to_owned();
    let bounds = Bounds::from_limits(limits);
    let supervised = supervise(launch, entry.name, limits, |channel, started| {
        converse(
            channel,
            &program,
            entry,
            (request, select_context),
            &bounds,
            started,
        )
    })?;
    let (worker, reply, elapsed) = supervised.decided;
    Ok(Evaluated {
        worker,
        reply,
        diagnostics: supervised.diagnostics,
        operational: supervised.operational,
        elapsed,
    })
}

fn converse(
    mut channel: Bounded<'_>,
    program: &Path,
    entry: &Entry<'_>,
    (request, select_context): (&Value, Option<&Value>),
    bounds: &Bounds,
    started: Instant,
) -> Result<(Identity, Reply, Duration), Conversation> {
    let hello = receive_for(&mut channel, program, REBUILD, false, HELLO_BYTES)?;
    let identity = verify_hello(&hello, program).map_err(Conversation::Broken)?;

    let mut frame = json!({
        "type": "evaluate",
        "protocol": PROTOCOL,
        // Exactly the admitted file: the failures name it.
        "entry": entry.name,
        "source": entry.source,
        "request": request,
        "bounds": bounds.to_json(),
    });
    if let Some(context) = select_context {
        frame["selectContext"] = context.clone();
    }
    write_frame(&mut channel, &frame).map_err(|error| Conversation::failed(error, false))?;

    let max = usize::try_from(bounds.message_bytes)
        .unwrap_or(usize::MAX)
        .saturating_add(REPLY_ENVELOPE);
    let frame = receive_for(&mut channel, program, REBUILD, true, max)?;
    let reply = Reply::from_frame(&frame).ok_or_else(|| {
        broken_for(
            program,
            REBUILD,
            format!(
                "the policy worker sent an unexpected frame: {}",
                shown(&frame)
            ),
        )
    })?;
    Ok((identity, reply, started.elapsed()))
}

fn verify_hello(hello: &Value, program: &Path) -> Result<Identity, Refusal> {
    let text = |field: &str| hello.get(field).and_then(Value::as_str).unwrap_or("absent");
    if hello.get("type").and_then(Value::as_str) != Some("hello") {
        return Err(Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker's first frame is not a hello: {}",
                shown(hello)
            ),
            REBUILD,
        )
        .source(program.to_string_lossy()));
    }
    let protocol = hello.get("protocol").and_then(Value::as_u64);
    if protocol != Some(PROTOCOL)
        || text("packageVersion") != PACKAGE_VERSION
        || text("buildId") != BUILD_ID
        || text("rhaiVersion") != RHAI_VERSION
    {
        let protocol = protocol.map_or_else(|| "absent".to_owned(), |p| p.to_string());
        return Err(Refusal::new(
            "worker_identity_mismatch",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker {} is not this front's pair: it reports protocol {protocol}, \
                 package {}, build {}, rhai {}; this front needs protocol {PROTOCOL}, package \
                 {PACKAGE_VERSION}, build {BUILD_ID}, rhai {RHAI_VERSION}",
                program.display(),
                text("packageVersion"),
                text("buildId"),
                text("rhaiVersion"),
            ),
            REBUILD,
        )
        .source(program.to_string_lossy()));
    }
    Ok(Identity {
        path: program.to_owned(),
        package_version: PACKAGE_VERSION.to_owned(),
        build_id: BUILD_ID.to_owned(),
        rhai_version: RHAI_VERSION.to_owned(),
    })
}

/// A frame, abbreviated for an error message.
fn shown(frame: &Value) -> String {
    let text = frame.to_string();
    match text.char_indices().nth(400) {
        Some((at, _)) => format!("{}… ({} bytes)", &text[..at], text.len()),
        None => text,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt as _;

    use super::*;
    use crate::settings::Settings;

    fn limits() -> Limits {
        Limits::read(None, None, &Settings::default()).unwrap()
    }

    /// A script that runs this test binary as the role. The fixture is the
    /// `role_fixture` test of `role`, which calls the same `serve` as the real
    /// binary. The environment is the supervision's own, empty but for `PATH`.
    fn role(dir: &Path) -> PathBuf {
        let script = dir.join("role.sh");
        fs::write(
            &script,
            format!(
                "#!/bin/sh\nexec '{}' --exact rhai_worker::role::tests::role_fixture --ignored --nocapture\n",
                std::env::current_exe().unwrap().display()
            ),
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        script
    }

    fn request() -> Value {
        json!({ "kind": "impl", "prompt": "go", "cwd": "/work", "params": { "k": "v" } })
    }

    fn run(source: &str, request: &Value) -> Result<Evaluated, Failure> {
        let dir = tempfile::tempdir().unwrap();
        let program = role(dir.path());
        let entry = Entry {
            name: "/home/o/policy.rhai",
            source,
        };
        evaluate(
            &entry,
            request,
            None,
            launch(&program, Vec::new()),
            &limits(),
        )
    }

    #[test]
    fn one_conversation_runs_a_route_table_through_supervise() {
        let evaluated = run(
            r#"export const SCHEMA_VERSION = 3;
export const VERSION = "1";
const TABLE = #{ impl: ["--print", "--fast"] };
print("from the policy");
fn select(request, context) {
    debug("selecting");
    #{ status: "selected", program: "claude", args: global::TABLE[request.kind],
       reason: request.params.k }
}"#,
            &request(),
        )
        .unwrap();
        assert_eq!(
            evaluated.reply,
            Reply::Selection {
                version: "1".to_owned(),
                result: json!({
                    "status": "selected", "program": "claude",
                    "args": ["--print", "--fast"], "reason": "v",
                }),
            }
        );
        assert_eq!(evaluated.worker.build_id, BUILD_ID);
        assert_eq!(evaluated.worker.rhai_version, "1.27.0");
        let stdout = String::from_utf8_lossy(&evaluated.diagnostics.stdout);
        let stderr = String::from_utf8_lossy(&evaluated.diagnostics.stderr);
        assert!(stdout.contains("from the policy\n"), "{stdout}");
        assert!(
            stderr.contains("selecting") && !stdout.contains("selecting"),
            "{stderr}"
        );
    }

    #[test]
    fn a_policy_failure_is_a_reply_and_the_worker_is_reaped_either_way() {
        let evaluated = run(
            "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"1\";\nfn select(request, context) { throw \"no\"; }",
            &request(),
        )
        .unwrap();
        match evaluated.reply {
            Reply::Failure(failure) => {
                assert_eq!(failure.code, "selection_threw");
                assert_eq!((failure.line, failure.column), (Some(3), Some(31)));
            }
            other => panic!("{other:?}"),
        }
        assert!(
            evaluated.operational.is_empty(),
            "{:?}",
            evaluated.operational
        );
    }

    #[test]
    fn a_worker_that_is_not_this_builds_pair_refuses_as_an_identity_mismatch() {
        let wrong = json!({
            "type": "hello", "protocol": PROTOCOL, "packageVersion": PACKAGE_VERSION,
            "buildId": "0".repeat(64), "rhaiVersion": RHAI_VERSION,
        });
        let refusal = verify_hello(&wrong, Path::new("/x")).unwrap_err();
        assert_eq!(refusal.code, "worker_identity_mismatch");
        assert_eq!(refusal.exit, EXIT_WORKER);
        for other in [json!({ "type": "evaluate" }), json!(3)] {
            assert_eq!(
                verify_hello(&other, Path::new("/x")).unwrap_err().code,
                "protocol_error"
            );
        }
    }

    /// What the launched image reports about itself, for the test below.
    #[test]
    #[ignore = "runs as a child of a supervising test"]
    fn image_probe() {
        println!(
            "argv0={} exe={}",
            std::env::args_os().next().unwrap().to_string_lossy(),
            std::fs::canonicalize(std::env::current_exe().unwrap())
                .unwrap()
                .display()
        );
    }

    /// `Command::arg0` with the program `executable` names runs this very
    /// image under the role's name. On Linux the program is `/proc/self/exe`
    /// and on macOS the canonical path, so this is the check for both.
    #[test]
    fn the_launch_runs_the_same_image_under_the_role_name() {
        use std::os::unix::process::CommandExt as _;
        use std::process::Command;
        let program = executable().unwrap();
        let output = Command::new(&program)
            .arg0(ARG0)
            .args(["--exact", "rhai_worker::front::tests::image_probe"])
            .args(["--ignored", "--nocapture"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let ours = fs::canonicalize(std::env::current_exe().unwrap()).unwrap();
        assert!(
            stdout.contains(&format!("argv0={ARG0} exe={}", ours.display())),
            "{stdout}"
        );
    }

    #[test]
    fn a_program_that_is_not_there_refuses_as_worker_missing() {
        let entry = Entry {
            name: "p.rhai",
            source: "",
        };
        let program = Path::new("/nonexistent/ravel");
        let failure = evaluate(
            &entry,
            &request(),
            None,
            launch(program, Vec::new()),
            &limits(),
        )
        .unwrap_err();
        assert_eq!(failure.refusal.code, "worker_missing");
    }
}
