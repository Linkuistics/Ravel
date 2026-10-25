//! The sandbox helper for Rhai policies at the real CLI seam.
//!
//! It adds to [`Sandbox`] the personal `policy.rhai`, entries by relative path
//! and a harness that records the argument vector it was started with, so a
//! test can read what a route table selected. The removal increment keeps this
//! file: the Rhai path is the one that stays.
use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;

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
        self.home.join(".config/ravel/policy.rhai")
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
