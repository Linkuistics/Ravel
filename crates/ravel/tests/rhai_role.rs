//! The internal role of the Rhai policy worker, driven as a real process
//! (`docs/specs/policy-evaluation.md`, *The policy worker*).
//!
//! The role is Ravel's own binary started with the `arg0` that marks it. These
//! tests start the real binary that way, hold the front's end of the private
//! socket and speak the frames by hand, so they exercise exactly what
//! `worker::supervise` launches, without `start-session`.

mod support;

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt as _;
use std::process::{Child, Command, Output, Stdio};

use serde_json::{Value, json};
use support::FRONT;

/// The `arg0` that marks the role (`rhai_worker::ARG0`).
const ARG0: &str = "ravel-rhai-policy-worker";

const POLICY: &str = r#"export const SCHEMA_VERSION = 3;
export const VERSION = "role-test";
const TABLE = #{ impl: ["--print"] };
print("printed at the top level");
fn select(request, context) {
    debug("debugged in select");
    #{ status: "selected", program: "fake-agent", args: global::TABLE[request.kind] }
}
"#;

fn bounds() -> Value {
    json!({
        "selectionMs": 30000, "contextBytes": 262144, "sourceBytes": 65536, "sources": 256,
        "messageBytes": 1048576, "diagnosticsBytes": 262144,
        "llmCalls": 8, "llmOutputTokens": 32768, "harnessRuns": 0,
    })
}

fn evaluate(source: &str) -> Value {
    json!({
        "type": "evaluate", "protocol": 1, "entry": "/home/o/policy.rhai", "source": source,
        "request": { "kind": "impl", "prompt": "go", "cwd": "/work", "params": {} },
        "bounds": bounds(),
    })
}

fn write_frame(mut stream: &UnixStream, message: &Value) {
    let body = serde_json::to_vec(message).unwrap();
    stream
        .write_all(&(body.len() as u32).to_be_bytes())
        .unwrap();
    stream.write_all(&body).unwrap();
}

fn read_frame(mut stream: &UnixStream) -> Value {
    let mut header = [0; 4];
    stream.read_exact(&mut header).unwrap();
    let mut body = vec![0; u32::from_be_bytes(header) as usize];
    stream.read_exact(&mut body).unwrap();
    serde_json::from_slice(&body).unwrap()
}

/// The role, with its end of a socket pair at descriptor 3, as the front starts
/// it: null stdin, piped output and an empty environment.
fn start(arguments: &[&str], channel: bool) -> (Child, UnixStream) {
    let (front_end, worker_end) = UnixStream::pair().unwrap();
    let mut command = Command::new(FRONT);
    command
        .arg0(ARG0)
        .args(arguments)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if channel {
        let fd = std::os::fd::AsRawFd::as_raw_fd(&worker_end);
        // SAFETY: only `dup2` runs between fork and exec, on a descriptor the
        // parent holds open until the child has exec'd.
        unsafe {
            command.pre_exec(move || {
                if libc::dup2(fd, 3) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }
    let child = command.spawn().unwrap();
    drop(worker_end);
    // A role that fails to refuse leaves a read waiting; a test must fail then.
    // macOS refuses a timeout on a socket whose peer already exited (`EINVAL`),
    // and a read from such a socket cannot block.
    let _ = front_end.set_read_timeout(Some(std::time::Duration::from_secs(10)));
    (child, front_end)
}

fn finish(child: Child) -> Output {
    child.wait_with_output().unwrap()
}

#[test]
fn the_role_says_hello_evaluates_the_source_and_exits_after_its_result() {
    let (child, channel) = start(&[], true);

    let hello = read_frame(&channel);
    assert_eq!(hello["type"], "hello");
    assert_eq!(hello["protocol"], 1);
    assert_eq!(hello["packageVersion"], env!("CARGO_PKG_VERSION"));
    assert_eq!(hello["rhaiVersion"], "1.27.0");
    let build = hello["buildId"].as_str().unwrap();
    assert!(build.len() == 64 && build.bytes().all(|b| b.is_ascii_hexdigit()));

    write_frame(&channel, &evaluate(POLICY));
    let reply = read_frame(&channel);
    assert_eq!(
        reply,
        json!({
            "type": "selection",
            "result": { "status": "selected", "program": "fake-agent", "args": ["--print"] },
        })
    );

    let output = finish(child);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "printed at the top level\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("debugged in select"), "{stderr}");
}

#[test]
fn a_policy_failure_comes_back_as_a_frame_with_its_location() {
    let (child, channel) = start(&[], true);
    read_frame(&channel);
    write_frame(
        &channel,
        &evaluate("export const SCHEMA_VERSION = 3;\nlet x = ;\n"),
    );
    let reply = read_frame(&channel);
    assert_eq!(reply["type"], "failure");
    assert_eq!(reply["failure"]["code"], "policy_parse_failed");
    assert_eq!(reply["failure"]["stage"], "load");
    assert_eq!(reply["failure"]["file"], "/home/o/policy.rhai");
    assert_eq!(reply["failure"]["line"], 2);
    assert_eq!(reply["failure"]["column"], 9);
    assert_eq!(finish(child).status.code(), Some(0));
}

#[test]
fn input_beyond_arg0_exits_with_status_2_before_any_frame() {
    let (child, channel) = start(&["--help"], true);
    let output = finish(child);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("accepts no command-line input"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut rest = Vec::new();
    (&channel).read_to_end(&mut rest).unwrap();
    assert!(rest.is_empty(), "no hello may precede the refusal");
}

#[test]
fn a_missing_socket_exits_with_status_2() {
    let (child, _channel) = start(&[], false);
    let output = finish(child);
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("not the protocol socket"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_descriptor_3_that_is_no_socket_exits_with_status_2() {
    let mut command = Command::new(FRONT);
    command
        .arg0(ARG0)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let file = std::fs::File::open("/dev/null").unwrap();
    let fd = std::os::fd::AsRawFd::as_raw_fd(&file);
    // SAFETY: as above; the file stays open in the parent until after the spawn.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(fd, 3) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn a_failed_handshake_exits_with_status_2() {
    let bad = [
        json!({ "type": "select", "protocol": 1 }),
        json!({ "type": "evaluate", "protocol": 2, "entry": "e", "source": "", "request": {}, "bounds": bounds() }),
        json!({ "type": "evaluate", "protocol": 1, "entry": "e", "source": "", "request": {}, "bounds": {} }),
        json!(7),
    ];
    for frame in bad {
        let (child, channel) = start(&[], true);
        read_frame(&channel);
        write_frame(&channel, &frame);
        let output = finish(child);
        assert_eq!(output.status.code(), Some(2), "{frame}");
    }
}

#[test]
fn a_front_that_closes_the_socket_before_it_sends_leaves_status_2() {
    let (child, channel) = start(&[], true);
    drop(channel);
    assert_eq!(finish(child).status.code(), Some(2));
}

#[test]
fn only_an_arg0_equal_to_the_role_name_enters_the_role() {
    let output = Command::new(FRONT)
        .arg0(format!("/some/dir/{ARG0}"))
        .env_clear()
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("internal role"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
