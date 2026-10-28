//! The front side: launch the role through `worker::supervise` and have it
//! evaluate one entry.
//!
//! The API is small on purpose. [`evaluate`] takes the entry source, the request
//! and the launch description, and returns what the judge needs: the
//! [`Reply`](super::Reply), the identity of the worker, the modules and reads it
//! measured and the supervision reports. `choice` builds the request and judges
//! the reply.
//!
//! After the handshake the conversation moves to a Tokio current-thread runtime
//! (`serve`), which answers the host calls of the worker. The runtime is built
//! here and retired here, so none exists once the worker is stopped.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::Value;

use super::harness::{RunRecord, RunSetup};
use super::host::{Host, Surroundings};
use super::reader::Reader;
use super::serve::{Judge, Served, Setup, serve};
use super::{ARG0, BUILD_ID, Bounds, Module, PROTOCOL, RHAI_VERSION, Read, Reply};
use crate::context::{Delivered, Measured};
use crate::environment::Grants;
use crate::limits::Limits;
use crate::refusal::{Diagnostics, EXIT_WORKER, Failure, Refusal, Stage};
use crate::settings::Directory;
use crate::worker::{Bounded, Conversation, Launch, broken_for, receive_for, supervise};

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The remedy for a worker that cannot run or is not this front's pair. The
/// worker is the front's own binary, so a mismatch means the file changed under
/// the running process.
const REBUILD: &str = "reinstall ravel, and start the session again; the Rhai worker is \
     the ravel binary itself, so it differs only when that file was replaced while ravel ran";

/// The largest hello: a few short fields.
const HELLO_BYTES: usize = 4096;

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
    /// The entry first, then each module in the order of its first import.
    pub modules: Vec<Module>,
    /// Every measured read that succeeded, in order.
    pub reads: Vec<Read>,
    /// Every harness run, in order, whether or not it started.
    pub runs: Vec<RunRecord>,
    /// The context a loader returned, as it was judged and measured.
    pub delivered: Option<Delivered>,
}

/// What the front knows about the evaluation beyond the entry and the request,
/// and what it needs to answer the host calls of the worker.
pub struct Session<'a> {
    /// The caller's working directory, which a relative read resolves against.
    pub cwd: &'a Path,
    /// The names of the caller's environment that `ravel::env` may read.
    pub grants: &'a Grants,
    /// The owner's harness entries, which discovery reports.
    pub directory: &'a Directory,
    /// The caller's `--context` document, which counts as a source.
    pub caller: Option<&'a Measured>,
    /// How a harness run is launched.
    pub setup: &'a RunSetup,
    /// Judges and measures the context that `load_context` returns.
    pub judge: &'a mut Judge<'a>,
    /// Judge the final reply before teardown can supersede its original refusal.
    /// Successful selection still requires the supervisor's admission receipt.
    pub reply_judge: &'a dyn Fn(&Reply) -> Result<(), Refusal>,
}

/// The program that is Ravel's own binary, as the role must be launched.
///
/// On Linux it is `/proc/self/exe`, which names the running image even after
/// the file was replaced. On macOS there is no such name, so it is the
/// canonical path of the executable, which names the file now at that path.
pub fn executable() -> Result<PathBuf, Refusal> {
    #[cfg(test)]
    if let Some(program) = super::fixtures::executable() {
        return Ok(program);
    }
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
                    format!("cannot find the ravel executable to launch the Rhai worker: {error}"),
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

/// The entry the front admitted, which it reads itself.
pub struct Admitted<'a> {
    /// The canonical path of the entry, exactly as the failures name it.
    pub name: &'a str,
    /// The refusal for an entry that cannot be read, from what is wrong with it.
    pub unreadable: &'a dyn Fn(&str) -> Refusal,
}

/// Start the role, have it evaluate `entry` for `request`, and tear it down.
/// The reader thread reads the entry once the worker has started, so the whole
/// deadline covers it.
///
/// `request` holds `kind`, `prompt`, `cwd` and `params`, and may hold `taskFile`,
/// `taskId` and `context`. The worker adds `schemaVersion` and `limits` itself,
/// from its own copy of the bounds. `select_context` is the second argument of
/// `select`: the caller's document with its measurement attached. Without one,
/// `select` receives `request.context`. The whole evaluation counts from the start
/// of the worker, within the selection bound of `limits`.
pub fn evaluate(
    entry: &Admitted<'_>,
    request: &Value,
    select_context: Option<&Value>,
    launch: Launch<'_>,
    limits: &Limits,
    session: Session<'_>,
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
            session,
        )
    })?;
    let (worker, served, elapsed) = supervised.decided;
    Ok(Evaluated {
        worker,
        reply: served.reply,
        diagnostics: supervised.diagnostics,
        operational: supervised.operational,
        elapsed,
        modules: served.modules,
        reads: served.reads,
        runs: served.runs,
        delivered: served.delivered,
    })
}

fn converse(
    mut channel: Bounded<'_>,
    program: &Path,
    entry: &Admitted<'_>,
    (request, select_context): (&Value, Option<&Value>),
    bounds: &Bounds,
    started: Instant,
    session: Session<'_>,
) -> Result<(Identity, Served, Duration), Conversation> {
    let hello = receive_for(&mut channel, program, REBUILD, false, HELLO_BYTES)?;
    let identity = verify_hello(&hello, program).map_err(Conversation::Broken)?;

    let (stream, deadline, overflow) = channel.parts();
    let failed = |what: &str, error: &dyn std::fmt::Display| {
        broken_for(
            program,
            REBUILD,
            format!("cannot start serving the Rhai worker: {what}: {error}"),
        )
    };
    // The runtime takes the descriptor over in nonblocking mode, from a copy, so
    // that the supervision still holds the end it closes.
    let stream = stream
        .try_clone()
        .and_then(|copy| copy.set_nonblocking(true).map(|()| copy))
        .map_err(|error| failed("the channel", &error))?;
    let reader = Reader::start().map_err(|error| failed("the reader thread", &error))?;
    let Session {
        cwd,
        grants,
        directory,
        caller,
        setup,
        judge,
        reply_judge,
    } = session;
    let host = Host::new(
        reader,
        *bounds,
        deadline,
        Surroundings {
            cwd: cwd.to_owned(),
            grants: grants.clone(),
            directory: directory.clone(),
            caller: caller.cloned(),
            setup: setup.clone(),
        },
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(|error| failed("the runtime", &error))?;
    let served = runtime.block_on(serve(
        stream,
        (deadline, overflow),
        Setup {
            program,
            rebuild: REBUILD,
            entry: entry.name,
            unreadable: entry.unreadable,
            request,
            select_context,
            bounds,
        },
        host,
        judge,
    ));
    // Every task is finished here, and the copy of the channel is closed.
    retire(runtime);
    let served = served?;
    reply_judge(&served.reply).map_err(Conversation::Refused)?;
    Ok((identity, served, started.elapsed()))
}

/// Drop `runtime` without waiting for a blocking task. Dropping it waits for
/// every one, with no limit, and the resolver of the HTTP client looks a host up
/// with `getaddrinfo` in a blocking task
/// (hyper-util 0.1.21, `client/legacy/connect/dns.rs`, `GaiResolver::call`). A
/// lookup that stalls would hold the Ravel CLI after a stop, when the call that
/// asked for it is long gone. The task cannot be stopped, so the thread is left
/// to the exit of the process.
///
/// <https://docs.rs/tokio/1/tokio/runtime/struct.Runtime.html#method.shutdown_background>
///
/// A task of the runtime itself is dropped at once either way. On the path to a
/// launch no blocking task exists, because no host call is in progress.
fn retire(runtime: tokio::runtime::Runtime) {
    runtime.shutdown_background();
}

fn verify_hello(hello: &Value, program: &Path) -> Result<Identity, Refusal> {
    let text = |field: &str| hello.get(field).and_then(Value::as_str).unwrap_or("absent");
    if hello.get("type").and_then(Value::as_str) != Some("hello") {
        return Err(Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the Rhai worker's first frame is not a hello: {}",
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
                "the Rhai worker {} is not this front's pair: it reports protocol {protocol}, \
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
pub(super) fn shown(frame: &Value) -> String {
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

    use serde_json::json;

    use super::*;
    use crate::settings::Settings;

    /// A runtime whose blocking pool is busy for `hold`, as `getaddrinfo` can be.
    fn busy_runtime(hold: Duration) -> tokio::runtime::Runtime {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap();
        let started = std::sync::Arc::new(std::sync::Barrier::new(2));
        let signal = std::sync::Arc::clone(&started);
        runtime.spawn_blocking(move || {
            signal.wait();
            std::thread::sleep(hold);
        });
        started.wait();
        runtime
    }

    #[test]
    fn retiring_the_runtime_does_not_wait_for_a_blocking_task() {
        let hold = Duration::from_millis(1500);

        // The control: dropping the runtime waits for the task, so this test
        // shows the wait that `retire` avoids.
        let runtime = busy_runtime(hold);
        let began = Instant::now();
        drop(runtime);
        assert!(
            began.elapsed() >= Duration::from_millis(1000),
            "a plain drop took {:?}",
            began.elapsed()
        );

        let runtime = busy_runtime(hold);
        let began = Instant::now();
        retire(runtime);
        assert!(
            began.elapsed() < Duration::from_millis(500),
            "retire took {:?}",
            began.elapsed()
        );
    }

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

    fn unreadable(what: &str) -> Refusal {
        Refusal::new(
            "config_unreadable",
            Stage::Authority,
            3,
            format!("the entry {what}"),
            "read it",
        )
    }

    /// A judge for a test that never loads a context.
    fn no_loader(_: Value, _: Vec<Measured>) -> Result<Delivered, Refusal> {
        panic!("this test has no load_context");
    }

    static NO_GRANTS: std::sync::LazyLock<Grants> = std::sync::LazyLock::new(Grants::default);
    static NO_DIRECTORY: std::sync::LazyLock<Directory> =
        std::sync::LazyLock::new(Directory::default);
    static NO_SETUP: std::sync::LazyLock<RunSetup> = std::sync::LazyLock::new(RunSetup::default);

    fn session<'a>(cwd: &'a Path, judge: &'a mut Judge<'a>) -> Session<'a> {
        Session {
            cwd,
            grants: &NO_GRANTS,
            directory: &NO_DIRECTORY,
            caller: None,
            setup: &NO_SETUP,
            judge,
            reply_judge: &|_| Ok(()),
        }
    }

    fn run(source: &str, request: &Value) -> Result<Evaluated, Failure> {
        let dir = tempfile::tempdir().unwrap();
        let program = role(dir.path());
        let path = dir.path().join("policy.rhai");
        fs::write(&path, source).unwrap();
        let name = fs::canonicalize(&path).unwrap();
        let unreadable = |what: &str| unreadable(what);
        let entry = Admitted {
            name: name.to_str().unwrap(),
            unreadable: &unreadable,
        };
        let mut judge = no_loader;
        evaluate(
            &entry,
            request,
            None,
            launch(&program, Vec::new()),
            &limits(),
            session(dir.path(), &mut judge),
        )
    }

    #[test]
    fn one_conversation_runs_a_route_table_through_supervise() {
        let evaluated = run(
            r#"export const SCHEMA_VERSION = 3;
export const VERSION = "1";
const TABLE = #{ impl: ["--print", "--fast"] };
print("from the configuration script");
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
        assert!(
            stdout.contains("from the configuration script\n"),
            "{stdout}"
        );
        assert!(
            stderr.contains("selecting") && !stdout.contains("selecting"),
            "{stderr}"
        );
    }

    #[test]
    fn a_real_worker_of_another_build_refuses_as_an_identity_mismatch() {
        let dir = tempfile::tempdir().unwrap();
        let program = role(dir.path());
        let entry = Admitted {
            name: "p.rhai",
            unreadable: &unreadable,
        };
        let environment = vec![(
            OsString::from("RAVEL_ROLE_FIXTURE_BUILD"),
            OsString::from("0".repeat(64)),
        )];
        let mut judge = no_loader;
        let failure = evaluate(
            &entry,
            &request(),
            None,
            launch(&program, environment),
            &limits(),
            session(dir.path(), &mut judge),
        )
        .unwrap_err();
        assert_eq!(failure.refusal.code, "worker_identity_mismatch");
        assert_eq!(failure.refusal.exit, EXIT_WORKER);
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
        let entry = Admitted {
            name: "p.rhai",
            unreadable: &unreadable,
        };
        let program = Path::new("/nonexistent/ravel");
        let mut judge = no_loader;
        let failure = evaluate(
            &entry,
            &request(),
            None,
            launch(program, Vec::new()),
            &limits(),
            session(Path::new("/"), &mut judge),
        )
        .unwrap_err();
        assert_eq!(failure.refusal.code, "worker_missing");
    }
}
