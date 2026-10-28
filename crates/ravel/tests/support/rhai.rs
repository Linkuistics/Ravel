//! The sandbox helper for Rhai policies at the real CLI seam.
//!
//! It adds to [`Sandbox`] the personal `config.rhai`, entries by relative path
//! and a harness that records the argument vector it was started with, so a
//! test can read what a route table selected. The removal increment keeps this
//! file: the Rhai path is the one that stays.
use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use super::{FRONT, Sandbox, write};

/// The header every policy of these tests starts with: the two constants the
/// contract needs.
pub const HEADER: &str =
    "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"rhai-seam-1\";\n";

/// A `select` that routes to the plain fake harness with the prompt as its
/// only argument.
pub const ROUTED: &str = r#"export const SCHEMA_VERSION = 3;
export const VERSION = "rhai-seam-1";
fn select(request, context) {
    #{ status: "selected", program: "fake-agent", args: [request.prompt],
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture route" }
}
"#;

/// A policy that has `body` after the header.
pub fn policy(body: &str) -> String {
    format!("{HEADER}{body}\n")
}

/// A `select` that routes to the recording harness with `args`, a Rhai array
/// expression, as its arguments.
pub fn routing(args: &str) -> String {
    policy(&format!(
        r#"fn select(request, context) {{
    #{{ status: "selected", program: "recording-agent", args: {args},
       provider: "fixture", model: "fixture", effort: "high", reason: "recorded" }}
}}"#
    ))
}

/// What the recording harness saw.
pub struct Recorded {
    file: PathBuf,
}

impl Recorded {
    /// Each argument, one per line, or `None` when the harness never started.
    pub fn argv(&self) -> Option<Vec<String>> {
        fs::read_to_string(&self.file)
            .ok()
            .map(|text| text.lines().map(str::to_owned).collect())
    }
}

impl Sandbox {
    pub fn personal_rhai_path(&self) -> PathBuf {
        self.home.join(".config/ravel/config.rhai")
    }

    pub fn personal_rhai(&self, source: &str) -> PathBuf {
        let path = self.personal_rhai_path();
        write(&path, source);
        path
    }

    /// A harness named `recording-agent` on the private `PATH`. It writes its
    /// arguments, and then reports a result like `fake-agent`.
    pub fn recording_agent(&self) -> Recorded {
        let file = self.tmp.join("recording-agent.argv");
        let script = self.bin.join("recording-agent");
        write(
            &script,
            &format!(
                "#!/bin/sh\nprintf '%s\\n' \"$@\" > {file:?}\nexec {FRONT:?} end-session done\n"
            ),
        );
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        Recorded { file }
    }
}

/// The `arg0` of the worker role, which `ps` shows as its command.
const WORKER_ARG0: &str = "ravel-rhai-policy-worker";

/// The pid of the worker that `front` launched, once it runs.
pub fn worker_of(front: u32) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let table = Command::new("ps")
            .args(["-A", "-o", "pid=,ppid=,command="])
            .output()
            .expect("ps runs");
        let table = String::from_utf8_lossy(&table.stdout).into_owned();
        let found = table.lines().find_map(|line| {
            let mut fields = line.split_whitespace();
            let pid: i32 = fields.next()?.parse().ok()?;
            let parent: u32 = fields.next()?.parse().ok()?;
            (parent == front && fields.next() == Some(WORKER_ARG0)).then_some(pid)
        });
        if let Some(pid) = found {
            return pid;
        }
        assert!(
            Instant::now() < deadline,
            "no worker of {front} in\n{table}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

pub fn gone(pid: i32) -> bool {
    // SAFETY: signal zero only tests whether the process exists.
    unsafe { libc::kill(pid, 0) == -1 }
}
