//! Private launch seam for process-isolated unit fixtures. Never compiled into
//! the installed executable; the native-main fixture normally uses its real role.
use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

thread_local! {
    static EXECUTABLE: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

pub(crate) fn executable() -> Option<PathBuf> {
    EXECUTABLE.with(|value| value.borrow().clone())
}

pub(crate) fn use_executable(program: PathBuf) {
    EXECUTABLE.with(|value| *value.borrow_mut() = Some(program));
}

/// Libtest owns main and cannot dispatch argv[0]. Run the same role implementation
/// through its ignored test, with exactly the environment the supervisor supplies.
pub(crate) fn install_role(program: &Path, discard_diagnostics: bool) {
    // Native reader-fault rows deliberately close an uncreated pipe. Libtest's
    // startup banner would panic before the role even sees its socket. Those
    // rows observe reader/worker ownership, not script output.
    let redirect = if discard_diagnostics {
        " >/dev/null 2>&1"
    } else {
        ""
    };
    fs::write(
        program,
        format!(
            "#!/bin/sh\nexec '{}' --exact rhai_worker::role::tests::role_fixture --ignored --nocapture{redirect}\n",
            std::env::current_exe().unwrap().display()
        ),
    )
    .unwrap();
    fs::set_permissions(program, fs::Permissions::from_mode(0o755)).unwrap();
}

/// Require the payload on its own diagnostic stream. Rhai debug adds the source
/// position and quotes strings; fake protocol workers write raw stderr.
pub(crate) fn assert_owner_diagnostics(text: &str) {
    assert!(
        text.lines()
            .any(|line| line == "config stdout: owner stdout"),
        "owner stdout lost: {text}"
    );
    assert!(
        text.lines().any(|line| line.starts_with("config stderr: ")
            && (line == "config stderr: owner stderr" || line.ends_with("| \"owner stderr\""))),
        "owner stderr lost: {text}"
    );
}
