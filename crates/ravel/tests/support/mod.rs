//! Command/probe fixtures use private resources and an empty spawn environment.
#![allow(dead_code)] // each integration binary uses its own subset
pub mod direct;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
pub const FRONT: &str = env!("CARGO_BIN_EXE_ravel");
pub const DEEP: &str = r#"{ status: "selected", program: "fake-agent", args: [request.prompt], provider: "fixture", model: "fixture", effort: "high", reason: "the fixture agent" }"#;
pub const ROUTED: &str = r#"export const policy = {
 schemaVersion: 2, version: "seam-1",
 select(request) { return { status: "selected", program: "fake-agent", args: [request.prompt], provider: "fixture", model: "fixture", effort: "high", reason: "fixture route" }; }
};"#;
pub struct Sandbox {
    _dir: tempfile::TempDir,
    pub root: PathBuf,
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub tmp: PathBuf,
    pub bin: PathBuf,
}
impl Sandbox {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let (home, cwd, tmp, bin) = (
            root.join("home"),
            root.join("work"),
            root.join("tmp"),
            root.join("bin"),
        );
        for dir in [&home, &cwd, &tmp, &bin] {
            fs::create_dir(dir).unwrap();
        }
        // The fixture publishes through the original development pair even when the
        // tested front has been copied into a deliberately mismatched installation.
        write(
            &bin.join("fake-agent"),
            &format!("#!/bin/sh\nexec {FRONT:?} end-session done\n"),
        );
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(bin.join("fake-agent"), fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            _dir: dir,
            root,
            home,
            cwd,
            tmp,
            bin,
        }
    }
    pub fn personal_path(&self) -> PathBuf {
        self.home.join(".config/ravel/policy.ts")
    }
    pub fn personal_policy(&self, source: &str) -> PathBuf {
        let path = self.personal_path();
        write(&path, source);
        path
    }
    pub fn file(&self, relative: &str, source: &str) -> PathBuf {
        let path = self.cwd.join(relative);
        write(&path, source);
        path
    }
    pub fn command(&self) -> Command {
        self.command_for(Path::new(FRONT))
    }
    pub fn command_for(&self, front: &Path) -> Command {
        let mut command = Command::new(front);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("TMPDIR", &self.tmp)
            .env("PATH", format!("{}:/usr/bin:/bin", self.bin.display()))
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        command
    }
    pub fn start(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.args(["start-session", "--prompt", "p"]).args(args);
        run(&mut command)
    }
    pub fn run(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("start-session").args(args);
        run(&mut command)
    }
}
pub struct Run {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}
pub fn run(command: &mut Command) -> Run {
    let output = command.output().unwrap();
    Run {
        code: output.status.code(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}
impl Run {
    pub fn report(&self) -> Value {
        assert_eq!(self.code, Some(0), "{}", self.stderr);
        assert_eq!(self.stdout, "done", "{}", self.stderr);
        serde_json::from_str(self.stderr.lines().next().expect("routing notice")).unwrap()
    }
    pub fn refusal(&self, code: i32) -> Value {
        assert_eq!(self.code, Some(code), "{}", self.stderr);
        assert!(self.stdout.is_empty());
        serde_json::from_str(self.stderr.lines().last().expect("refusal")).unwrap()
    }
}
pub fn text(path: &Path) -> String {
    path.to_str().unwrap().to_owned()
}
pub fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
pub fn mkfifo(path: &Path) {
    use std::os::unix::ffi::OsStrExt as _;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
    // SAFETY: a valid NUL-terminated temporary fixture path.
    assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
}
