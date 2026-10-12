//! A directly executed test binary must not pass its session's authority on.
mod support;

#[test]
fn direct_helpers_remove_old_and_new_authority_but_keep_fixture_inputs() {
    // Dummy values isolate this test from the enclosing session. A missing
    // scrub would expose each name to the real subprocess and fail this test.
    let controls = [
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
    let mut command = std::process::Command::new("env");
    for name in controls {
        command.env(name, "isolated-test-control");
    }
    command.env("RUNNER_FIXTURE_INPUT", "keep");
    support::scrub(&mut command);
    let output = command.output().unwrap();
    assert!(output.status.success());
    let environment = String::from_utf8(output.stdout).unwrap();
    for name in controls {
        assert!(
            !environment
                .lines()
                .any(|line| line.starts_with(&format!("{name}="))),
            "direct helper inherited {name}"
        );
    }
    assert!(environment
        .lines()
        .any(|line| line == "RUNNER_FIXTURE_INPUT=keep"));
}
