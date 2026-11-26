//! What a signalled launcher tells the process that started *it*.
//!
//! **Only a wait status can say "was signalled"**, and the only way to produce
//! one is to die of the signal — so this drives a real process death rather
//! than asserting about a return value. The test re-runs *itself* as a child:
//! the branch below fires only when the child's environment variable is set,
//! and the parent half reads the wait status the child leaves behind.
//!
//! Its own integration-test binary, because `reraise` ends the process it is
//! called in and would take every other test in the binary with it.

mod support;

use std::os::unix::process::ExitStatusExt as _;

/// The variable the parent half sets and the child half answers to. Nothing
/// outside this file reads it.
const REEXEC: &str = "KEYED_LAUNCH_RERAISE_TEST_CHILD";

const SIGTERM: i32 = 15;

#[test]
fn a_reraised_signal_reaches_the_parent_as_a_wait_status() {
    if std::env::var_os(REEXEC).is_some() {
        // The child half. Never returns; the parent below is the assertion.
        keyed_launch::reraise(SIGTERM);
    }

    let status = support::command(std::env::current_exe().unwrap())
        .arg("a_reraised_signal_reaches_the_parent_as_a_wait_status")
        .arg("--exact")
        .env(REEXEC, "1")
        .status()
        .unwrap();

    assert_eq!(
        status.signal(),
        Some(SIGTERM),
        "a re-raised signal must reach the parent as a signal death: {status:?}"
    );
    // The half a launcher that merely `exit`s gets wrong, and the reason the
    // whole thing exists: an exit code — any exit code, 0 included — says the
    // process ran to a decision of its own. There is no code that means
    // "killed".
    assert_eq!(
        status.code(),
        None,
        "an exit code cannot express a signal death, so there must not be one"
    );
}
