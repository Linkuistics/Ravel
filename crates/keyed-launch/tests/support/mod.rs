//! Authority controls are test-harness concerns, never runner semantics.
#![allow(dead_code)] // Each integration binary uses only the helpers it needs.

use std::ffi::OsStr;
use std::process::Command;
use std::sync::LazyLock;

pub const CONTROL_ENV: &[&str] = &[
    "GROVE_LAUNCH_DIR",
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "RAVEL_RUN_ID",
    "RAVEL_STATE_DIR",
];

pub fn controls() -> &'static [&'static OsStr] {
    static NAMES: LazyLock<Vec<&'static OsStr>> =
        LazyLock::new(|| CONTROL_ENV.iter().map(OsStr::new).collect());
    NAMES.as_slice()
}

pub fn scrub(command: &mut Command) {
    for name in CONTROL_ENV {
        command.env_remove(name);
    }
}

pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    scrub(&mut command);
    command
}
