//! Command fixtures use private resources and an empty spawn environment.
#![allow(dead_code)] // each integration binary uses its own subset
pub mod rhai;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
pub const FRONT: &str = env!("CARGO_BIN_EXE_ravel");
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
        // The fixture publishes through the original development binary even when the
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

/// A plain Git checkout at `dir` with one commit, then uncommitted changes: a
/// changed tracked file and an untracked file. The caller's own `git` makes
/// it, with no global or system configuration and `scratch` as its HOME. It
/// returns the committed and the working bytes of `SKILL.md`.
pub fn git_checkout(dir: &Path, scratch: &Path) -> (&'static str, &'static str) {
    const COMMITTED: &str = "committed\n";
    const WORKING: &str = "uncommitted change\n";
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .current_dir(dir)
            .env("HOME", scratch)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .args([
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
            ])
            .args([
                "-c",
                "commit.gpgsign=false",
                "-c",
                "init.defaultBranch=main",
            ])
            .args(args)
            .output()
            .expect("git starts");
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    write(&dir.join("s/x/SKILL.md"), COMMITTED);
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "fixture"]);
    write(&dir.join("s/x/SKILL.md"), WORKING);
    write(&dir.join("s/x/untracked.md"), "untracked\n");
    (COMMITTED, WORKING)
}

/// A spy `git` at `bin/git` that records each call in `calls` and fails.
pub fn spy_git(bin: &Path, calls: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    let path = bin.join("git");
    write(
        &path,
        &format!("#!/bin/sh\nprintf '%s\\n' \"$*\" >> {calls:?}\nexit 1\n"),
    );
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
}
