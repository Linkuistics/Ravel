//! The bounds, the resolution deadline and cancellation of skill resolution
//! at the public `ravel skills resolve` and `start-session` seams.
//! docs/specs/skill-discovery.md, "Bounds", "Failures" and the local parts of
//! the acceptance rows "Bounds and frames", "Slow valid source" and
//! "Cancellation".
//!
//! A controlled delay stops the resolver with SIGSTOP while its capture
//! staging exists and before it writes a manifest, so the delay and a signal
//! act during capture. The fixture confirms the stop before it examines the
//! staging, and it tries again when the stop missed the capture. The descriptor and preamble bounds cannot be reached
//! on macOS within the other bounds, so the unit tests in
//! `skills::publication_tests` cross them with smaller limits.

mod support;

use std::fs::{self, File};
use std::os::unix::fs::symlink;
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{Sandbox, write};

const SETTINGS: &str = ".config/ravel/settings.json";
const CACHE: &str = ".cache/ravel/skills/v1";
const MIB: u64 = 1024 * 1024;

/// A sandbox whose snapshots, which are read-only, are removed at the end.
struct Fixture {
    sandbox: Sandbox,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.sandbox.root)
            .status();
    }
}

impl std::ops::Deref for Fixture {
    type Target = Sandbox;
    fn deref(&self) -> &Sandbox {
        &self.sandbox
    }
}

impl Fixture {
    fn new() -> Self {
        Fixture {
            sandbox: Sandbox::new(),
        }
    }

    fn dir(&self, name: &str) -> PathBuf {
        let path = self.root.join("sources").join(name);
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn skills(&self, skills: Value) {
        write(
            &self.home.join(SETTINGS),
            &json!({ "skills": skills }).to_string(),
        );
    }

    fn resolve_command(&self) -> Command {
        let mut command = self.command();
        command.args(["skills", "resolve", "--json"]);
        command
    }

    fn resolve(&self) -> Output {
        self.resolve_command().output().unwrap()
    }

    fn resolved(&self) -> Value {
        let output = self.resolve();
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        serde_json::from_slice(&output.stdout).unwrap()
    }

    /// The refusal document of a resolution that exits `exit`.
    fn refused(&self, exit: i32) -> Value {
        let output = self.resolve();
        assert_eq!(output.status.code(), Some(exit), "{}", stderr(&output));
        assert!(output.stdout.is_empty());
        refusal(&output)
    }

    fn names(&self, directory: &str) -> Vec<String> {
        match fs::read_dir(self.home.join(CACHE).join(directory)) {
            Err(_) => Vec::new(),
            Ok(entries) => {
                let mut names: Vec<String> = entries
                    .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                    .collect();
                names.sort();
                names
            }
        }
    }

    /// No staging stays, and no descriptor beyond `descriptors` exists.
    fn clean(&self, descriptors: usize) {
        assert!(
            self.names("staging").is_empty(),
            "{:?}",
            self.names("staging")
        );
        assert_eq!(self.names("descriptors").len(), descriptors);
    }

    fn start_command(&self) -> Command {
        self.personal_rhai(&rhai_script(
            r#"fn select(request, context) {
    #{ status: "selected", program: "fake-agent", args: [],
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture" }
}"#,
        ));
        let mut command = self.command();
        command.args([
            "start-session",
            "--kind",
            "check",
            "--json",
            "--prompt",
            "p",
        ]);
        command
    }

    /// Ravel's own files in the private `TMPDIR`: result channels and
    /// confined run directories.
    fn left_in_tmp(&self) -> Vec<String> {
        fs::read_dir(&self.tmp)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("ravel-"))
            .collect()
    }

    /// Start the command that `command` builds, and stop it with SIGSTOP
    /// while its capture staging exists and holds no manifest. Capture
    /// writes its manifest only at its end.
    ///
    /// The staging is examined only after `waitpid` reports the stop, so the
    /// resolver cannot write or rename while the fixture looks. A stop that
    /// misses the capture, or a resolver that ends before the fixture sees
    /// it, kills that resolver and resets the cache. Then the fixture tries
    /// again, within a bound of attempts.
    fn stopped_in_capture(&self, command: impl Fn() -> Command) -> Stopped {
        const ATTEMPTS: usize = 5;
        let staging = self.home.join(CACHE).join("staging");
        let started = Instant::now();
        let mut missed = Vec::new();
        for _ in 0..ATTEMPTS {
            let mut child = Stopped::spawn(command());
            let capture = loop {
                let capturing = fs::read_dir(&staging).ok().and_then(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .find(|entry| entry.file_name().to_string_lossy().starts_with("capture-"))
                });
                if let Some(entry) = capturing {
                    child.signal(libc::SIGSTOP);
                    if child.confirm_stop() {
                        break Some(entry.path());
                    }
                    missed.push("the resolver ended before its stop".to_owned());
                    break None;
                }
                if let Some(status) = child.ended() {
                    missed.push(format!("the resolver ended first: {status}"));
                    break None;
                }
                assert!(
                    started.elapsed() < Duration::from_secs(60),
                    "no capture began; missed: {missed:?}"
                );
                thread::sleep(Duration::from_micros(200));
            };
            match capture {
                Some(capture)
                    if capture.is_dir()
                        && !capture.join("manifest").exists()
                        && self.names("snapshots").is_empty() =>
                {
                    return child;
                }
                Some(_) => missed.push("the stop came after the capture".to_owned()),
                None => {}
            }
            drop(child);
            self.reset_cache();
        }
        panic!("no stop came during a capture in {ATTEMPTS} attempts: {missed:?}");
    }

    /// Remove the cache, whose snapshots are read-only.
    fn reset_cache(&self) {
        let cache = self.home.join(".cache");
        if cache.exists() {
            let _ = Command::new("chmod")
                .arg("-R")
                .arg("u+w")
                .arg(&cache)
                .status();
            fs::remove_dir_all(&cache).unwrap();
        }
    }
}

/// A resolver process that a fixture controls with signals. It is killed and
/// reaped on every path that does not collect it, also when a test fails.
struct Stopped {
    child: Child,
    /// The process was reaped, so its process ID can name another process.
    reaped: bool,
}

impl Stopped {
    fn spawn(mut command: Command) -> Self {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        Stopped {
            child,
            reaped: false,
        }
    }

    fn pid(&self) -> libc::pid_t {
        assert!(!self.reaped, "the resolver was reaped");
        libc::pid_t::try_from(self.child.id()).unwrap()
    }

    fn signal(&self, number: libc::c_int) {
        // SAFETY: the child is not reaped yet, so its process ID is still its own.
        assert_eq!(unsafe { libc::kill(self.pid(), number) }, 0);
    }

    /// The status of a resolver that ended, which reaps it.
    fn ended(&mut self) -> Option<std::process::ExitStatus> {
        let status = self.child.try_wait().unwrap();
        self.reaped |= status.is_some();
        status
    }

    /// Wait until the process is stopped. False when it ended instead, which
    /// reaps it.
    fn confirm_stop(&mut self) -> bool {
        let pid = self.pid();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let mut status = 0;
            // SAFETY: `pid` is the unreaped child, and `status` is a valid
            // destination.
            let waited =
                unsafe { libc::waitpid(pid, &mut status, libc::WUNTRACED | libc::WNOHANG) };
            if waited == pid {
                if libc::WIFSTOPPED(status) {
                    return true;
                }
                self.reaped = true;
                return false;
            }
            if waited < 0 {
                let error = std::io::Error::last_os_error();
                assert_eq!(error.kind(), std::io::ErrorKind::Interrupted, "{error}");
            }
            assert!(Instant::now() < deadline, "the resolver did not stop");
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// Continue the process, and collect it and its output within 60 seconds.
    fn finish(mut self) -> Output {
        self.signal(libc::SIGCONT);
        let mut stdout = self.child.stdout.take().unwrap();
        let mut stderr = self.child.stderr.take().unwrap();
        let out = thread::spawn(move || {
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut stdout, &mut bytes).map(|_| bytes)
        });
        let err = thread::spawn(move || {
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut stderr, &mut bytes).map(|_| bytes)
        });
        let deadline = Instant::now() + Duration::from_secs(60);
        let status = loop {
            if let Some(status) = self.ended() {
                break status;
            }
            assert!(Instant::now() < deadline, "the resolver did not end");
            thread::sleep(Duration::from_millis(5));
        };
        Output {
            status,
            stdout: out.join().unwrap().unwrap(),
            stderr: err.join().unwrap().unwrap(),
        }
    }
}

impl Drop for Stopped {
    fn drop(&mut self) {
        if !self.reaped {
            // SAFETY: the child is not reaped yet, so its process ID is still
            // its own. KILL also ends a stopped process.
            unsafe { libc::kill(self.pid(), libc::SIGKILL) };
            let _ = self.child.wait();
        }
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The refusal document, the last line of standard error.
fn refusal(output: &Output) -> Value {
    let text = stderr(output);
    let document: Value = serde_json::from_str(text.lines().last().expect("a refusal"))
        .unwrap_or_else(|error| {
            panic!("{error}: {text}");
        });
    document["error"].clone()
}

fn local(id: &str, path: &Path, roots: &[&str]) -> Value {
    json!({ "id": id, "type": "local", "path": path, "roots": roots })
}

fn skills(sources: Vec<Value>, mandatory: Vec<Value>) -> Value {
    json!({ "schemaVersion": 1, "sources": sources, "mandatory": mandatory })
}

fn file(path: &Path, bytes: u64) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    File::create(path).unwrap().set_len(bytes).unwrap();
}

/// A source of 2,000 small files in 20 directories, wide enough that a
/// fixture can stop its capture.
fn wide(source: &Path) {
    for directory in 0..20 {
        for index in 0..100 {
            write(
                &source.join(format!("d{directory}/f{index}.md")),
                &format!("{directory} {index}\n"),
            );
        }
    }
}

/// A refusal of the bound `name` at `bound`, which reports `actual`.
fn limited(error: &Value, name: &str, bound: u64, actual: &str) {
    assert_eq!(error["code"], "skill_limit_exceeded", "{error}");
    assert_eq!(error["stage"], "skills", "{error}");
    assert_eq!(error["exit"], 3, "{error}");
    assert_eq!(error["bound"]["name"], name, "{error}");
    assert_eq!(error["bound"]["from"], "fixed", "{error}");
    let unit = error["bound"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(key, _)| !["name", "from"].contains(&key.as_str()))
        .unwrap();
    assert_eq!(unit.1.as_u64(), Some(bound), "{error}");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains(actual), "{message}");
    assert!(
        message.contains(&format!("over the bound of {bound} ")),
        "{message}"
    );
}

#[test]
fn each_count_bound_passes_at_its_value_and_refuses_one_more() {
    let fixture = Fixture::new();
    let tiny = |name: &str| {
        let source = fixture.dir(name);
        write(&source.join("SKILL.md"), name);
        source
    };
    let sources: Vec<Value> = (0..17)
        .map(|index| local(&format!("s{index}"), &tiny(&format!("s{index}")), &["."]))
        .collect();
    fixture.skills(skills(sources[..16].to_vec(), vec![]));
    assert_eq!(fixture.resolved()["sources"].as_array().unwrap().len(), 16);
    fixture.skills(skills(sources.clone(), vec![]));
    let error = fixture.refused(3);
    limited(&error, "skillSources", 16, "17 sources");
    assert_eq!(error["location"], "skills.sources");
    fixture.clean(1);

    // Roots count across sources: 128 and 129 are 257. Each source has
    // its own content, so that no two roots name one snapshot directory.
    let (left, right) = (fixture.dir("left"), fixture.dir("right"));
    for index in 0..129 {
        fs::create_dir(left.join(format!("r{index}"))).unwrap();
        fs::create_dir(right.join(format!("r{index}"))).unwrap();
    }
    write(&right.join("SKILL.md"), "right");
    let roots: Vec<String> = (0..129).map(|index| format!("r{index}")).collect();
    let roots: Vec<&str> = roots.iter().map(String::as_str).collect();
    fixture.skills(skills(
        vec![
            local("a", &left, &roots[..128]),
            local("b", &right, &roots[..128]),
        ],
        vec![],
    ));
    assert_eq!(
        fixture.resolved()["directories"].as_array().unwrap().len(),
        256
    );
    fixture.skills(skills(
        vec![local("a", &left, &roots[..128]), local("b", &right, &roots)],
        vec![],
    ));
    let error = fixture.refused(3);
    limited(&error, "skillSearchDirectories", 256, "257 directories");
    assert_eq!(error["location"], "skills.sources");
    fixture.clean(2);

    let guidance = fixture.dir("guidance");
    for index in 0..257 {
        write(&guidance.join(format!("m{index}.md")), "m");
    }
    let mandatory: Vec<Value> = (0..257)
        .map(|index| json!({"source": "g", "path": format!("m{index}.md")}))
        .collect();
    fixture.skills(skills(
        vec![local("g", &guidance, &["."])],
        mandatory[..256].to_vec(),
    ));
    assert_eq!(
        fixture.resolved()["mandatory"].as_array().unwrap().len(),
        256
    );
    fixture.skills(skills(vec![local("g", &guidance, &["."])], mandatory));
    let error = fixture.refused(3);
    limited(&error, "skillMandatoryFiles", 256, "257 files");
    assert_eq!(error["location"], "skills.mandatory");
    fixture.clean(3);
}

#[test]
fn a_mandatory_file_passes_at_65536_bytes_and_refuses_one_byte_more() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    let mandatory = |path: &str| vec![json!({"source": "a", "path": path})];
    file(&source.join("m.md"), 65_536);
    fixture.skills(skills(vec![local("a", &source, &["."])], mandatory("m.md")));
    fixture.resolved();
    file(&source.join("m.md"), 65_537);
    let error = fixture.refused(3);
    limited(&error, "mandatoryFileBytes", 65_536, "65537 bytes");
    assert_eq!(error["source"], json!(source.join("m.md")));
    assert_eq!(error["location"], "skills.mandatory[0]");
    fixture.clean(1);

    // A link inside the source can reach the mandatory file.
    file(&source.join("m.md"), 65_536);
    symlink("m.md", source.join("via.md")).unwrap();
    fixture.skills(skills(
        vec![local("a", &source, &["."])],
        mandatory("via.md"),
    ));
    fixture.resolved();
    // The copy of m.md admits it, because m.md itself is not mandatory. The
    // check of the mandatory reference refuses it.
    file(&source.join("m.md"), 65_537);
    let error = fixture.refused(3);
    limited(&error, "mandatoryFileBytes", 65_536, "65537 bytes");
    assert_eq!(error["location"], "skills.mandatory[0]");
    fixture.clean(2);
}

#[test]
fn one_resource_passes_at_16_mib_and_refuses_one_byte_more() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    file(&source.join("big.bin"), 16 * MIB);
    fixture.skills(skills(vec![local("a", &source, &["."])], vec![]));
    fixture.resolved();
    file(&source.join("big.bin"), 16 * MIB + 1);
    let error = fixture.refused(3);
    limited(
        &error,
        "resourceBytes",
        16 * MIB,
        &format!("{} bytes", 16 * MIB + 1),
    );
    assert_eq!(error["source"], json!(source.join("big.bin")));
    assert_eq!(error["location"], "skills.sources[0]");
    fixture.clean(1);
}

#[test]
fn the_bytes_of_one_source_and_of_all_sources_pass_at_their_bounds_and_refuse_one_more() {
    let fixture = Fixture::new();
    let full = fixture.dir("full");
    for index in 0..8 {
        file(&full.join(format!("f{index}.bin")), 16 * MIB);
    }
    fixture.skills(skills(vec![local("a", &full, &["."])], vec![]));
    fixture.resolved();
    file(&full.join("one.bin"), 1);
    let error = fixture.refused(3);
    limited(
        &error,
        "sourceBytes",
        128 * MIB,
        &format!("at least {} bytes", 128 * MIB + 1),
    );
    assert_eq!(error["location"], "skills.sources[0]");
    fixture.clean(1);
    fs::remove_file(full.join("one.bin")).unwrap();

    // Four sources of 128 MiB are 512 MiB. Each capture counts, also of one
    // directory.
    let four: Vec<Value> = (0..4)
        .map(|index| local(&format!("s{index}"), &full, &["."]))
        .collect();
    fixture.skills(skills(four.clone(), vec![]));
    fixture.resolved();
    let one = fixture.dir("one");
    file(&one.join("one.bin"), 1);
    let mut five = four;
    five.push(local("s4", &one, &["."]));
    fixture.skills(skills(five, vec![]));
    let error = fixture.refused(3);
    limited(
        &error,
        "totalSourceBytes",
        512 * MIB,
        &format!("at least {} bytes", 512 * MIB + 1),
    );
    assert_eq!(error["location"], "skills.sources[4]");
    fixture.clean(2);
}

#[test]
fn the_entries_of_one_source_pass_at_20000_and_refuse_one_more() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    write(&source.join("t.md"), "t");
    // A directory, a file and links: each one counts.
    fs::create_dir(source.join("d")).unwrap();
    for index in 0..19_998 {
        symlink("t.md", source.join(format!("l{index}"))).unwrap();
    }
    fixture.skills(skills(vec![local("a", &source, &["."])], vec![]));
    fixture.resolved();
    symlink("../t.md", source.join("d/extra")).unwrap();
    let error = fixture.refused(3);
    limited(&error, "sourceEntries", 20_000, "at least 20001 entries");
    assert_eq!(error["location"], "skills.sources[0]");
    fixture.clean(1);
}

#[test]
fn a_malformed_deadline_refuses_as_settings_invalid() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    write(&source.join("SKILL.md"), "a");
    for value in [
        json!(999),
        json!(3_600_001),
        json!(-1),
        json!(1500.5),
        json!("120000"),
        json!(null),
    ] {
        let mut settings = skills(vec![local("a", &source, &["."])], vec![]);
        settings["resolutionTimeoutMs"] = value.clone();
        fixture.skills(settings);
        let error = fixture.refused(2);
        assert_eq!(error["code"], "settings_invalid", "{value}: {error}");
        assert_eq!(error["location"], "skills.resolutionTimeoutMs", "{value}");
        assert!(!fixture.home.join(".cache").exists(), "{value}");
    }
    for value in [1_000, 3_600_000] {
        let mut settings = skills(vec![local("a", &source, &["."])], vec![]);
        settings["resolutionTimeoutMs"] = json!(value);
        fixture.skills(settings);
        fixture.resolved();
    }
}

#[test]
fn a_controlled_delay_times_out_and_a_larger_deadline_succeeds_with_every_check() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    wide(&source);
    let settings = |ms: u64| {
        let mut settings = skills(
            vec![local("a", &source, &["d0"])],
            vec![json!({"source": "a", "path": "d1/f1.md"})],
        );
        settings["resolutionTimeoutMs"] = json!(ms);
        settings
    };

    fixture.skills(settings(1_000));
    let child = fixture.stopped_in_capture(|| fixture.resolve_command());
    thread::sleep(Duration::from_millis(1_300));
    let output = child.finish();
    assert_eq!(output.status.code(), Some(124), "{}", stderr(&output));
    assert!(output.stdout.is_empty());
    let error = refusal(&output);
    assert_eq!(error["code"], "skill_resolution_timeout", "{error}");
    assert_eq!(error["stage"], "skills");
    assert_eq!(error["exit"], 124);
    assert_eq!(
        error["bound"],
        json!({"name": "resolutionTimeoutMs", "ms": 1000, "from": "settings.json"})
    );
    assert_eq!(error["location"], "skills.sources[0]");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains("1000 ms"), "{message}");
    assert!(
        message.contains("during capture of the source \"a\""),
        "{message}"
    );
    assert!(
        error["remedy"]
            .as_str()
            .unwrap()
            .contains("skills.resolutionTimeoutMs"),
        "{error}"
    );
    fixture.clean(0);
    assert!(fixture.names("snapshots").is_empty());

    // The same delay within a larger deadline resolves, and the snapshot
    // holds every file.
    fixture.skills(settings(60_000));
    let child = fixture.stopped_in_capture(|| fixture.resolve_command());
    thread::sleep(Duration::from_millis(1_300));
    let output = child.finish();
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let resolved: Value = serde_json::from_slice(&output.stdout).unwrap();
    let root = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    for directory in 0..20 {
        for index in 0..100 {
            let relative = format!("d{directory}/f{index}.md");
            assert_eq!(
                fs::read(root.join(&relative)).unwrap(),
                fs::read(source.join(&relative)).unwrap()
            );
        }
    }
    assert_eq!(
        resolved["mandatory"][0]["path"],
        json!(root.join("d1/f1.md"))
    );
    fixture.clean(1);
}

#[test]
fn int_term_and_hup_during_capture_cancel_remove_the_staging_and_are_raised_again() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    wide(&source);
    fixture.skills(skills(vec![local("a", &source, &["."])], vec![]));
    for (number, name) in [
        (libc::SIGINT, "SIGINT"),
        (libc::SIGTERM, "SIGTERM"),
        (libc::SIGHUP, "SIGHUP"),
    ] {
        let child = fixture.stopped_in_capture(|| fixture.resolve_command());
        // The signal waits while the resolver is stopped, and arrives during
        // its capture when it continues.
        child.signal(number);
        let output = child.finish();
        assert_eq!(output.status.signal(), Some(number), "{}", stderr(&output));
        assert!(output.stdout.is_empty());
        let error = refusal(&output);
        assert_eq!(error["code"], "skill_resolution_cancelled", "{error}");
        assert_eq!(error["stage"], "skills");
        assert_eq!(error["exit"], 128 + number);
        assert_eq!(error["signal"], name);
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .contains("during capture of the source \"a\""),
            "{error}"
        );
        fixture.clean(0);
        assert!(fixture.names("snapshots").is_empty());
    }
}

#[test]
fn a_launch_cancels_or_times_out_during_capture_before_the_worker_and_the_channel() {
    let fixture = Fixture::new();
    let source = fixture.dir("a");
    wide(&source);
    let mut settings = skills(vec![local("a", &source, &["."])], vec![]);
    settings["resolutionTimeoutMs"] = json!(1_000);
    fixture.skills(settings);

    let child = fixture.stopped_in_capture(|| fixture.start_command());
    child.signal(libc::SIGTERM);
    let output = child.finish();
    assert_eq!(
        output.status.signal(),
        Some(libc::SIGTERM),
        "{}",
        stderr(&output)
    );
    assert!(output.stdout.is_empty(), "the managed program ran");
    let error = refusal(&output);
    assert_eq!(error["code"], "skill_resolution_cancelled", "{error}");
    assert_eq!(error["exit"], 143);
    assert_eq!(error["signal"], "SIGTERM");
    // One refusal and no routing notice: no worker evaluated the entry.
    assert_eq!(stderr(&output).lines().count(), 1, "{}", stderr(&output));
    assert!(fixture.left_in_tmp().is_empty());
    fixture.clean(0);

    let child = fixture.stopped_in_capture(|| fixture.start_command());
    thread::sleep(Duration::from_millis(1_300));
    let output = child.finish();
    assert_eq!(output.status.code(), Some(124), "{}", stderr(&output));
    assert!(output.stdout.is_empty());
    let error = refusal(&output);
    assert_eq!(error["code"], "skill_resolution_timeout", "{error}");
    assert_eq!(error["bound"]["ms"], 1000);
    assert_eq!(stderr(&output).lines().count(), 1, "{}", stderr(&output));
    assert!(fixture.left_in_tmp().is_empty());
    fixture.clean(0);
}
