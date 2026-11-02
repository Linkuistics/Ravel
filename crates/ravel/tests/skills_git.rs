//! Online exact Git acquisition at both public seams, `ravel skills resolve`
//! and `ravel start-session`, with local repositories reached through
//! `file://` and absolute paths. docs/specs/skill-discovery.md, "Exact Git
//! acquisition and offline reuse" without the cache hit, and "Test seams and
//! acceptance".
//!
//! These rows run here: "Exact Git", "Git LFS object bytes", the gitlinks of
//! "Relative resources", the commit, `subdir` and object cases of "Missing or
//! corrupt revision", and "Directory delivery" with mixed local and Git
//! sources. A spy `git` first on the private `PATH` records what every Git
//! child inherits, and then runs the real Git.

mod support;

use std::fs;
use std::io::Write as _;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{FRONT, Run, Sandbox, run, write};

const SETTINGS: &str = ".config/ravel/settings.json";
const CACHE: &str = ".cache/ravel/skills/v1";
/// The real Git that the fixtures and the spy use.
const GIT: &str = "/usr/bin/git";
/// A configuration script that cannot parse: a refusal before it proves that
/// no worker evaluated the entry.
const UNPARSED: &str = "export const SCHEMA_VERSION = 4;\nfn select(request, context) {\n";

struct Fixture {
    sandbox: Sandbox,
    /// The HOME of the fixture repositories, apart from the HOME of Ravel.
    author: PathBuf,
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
        let sandbox = Sandbox::new();
        let author = sandbox.root.join("author");
        fs::create_dir_all(&author).unwrap();
        Fixture { sandbox, author }
    }

    fn repo(&self, name: &str) -> Repo {
        self.repo_in(name, "sha1")
    }

    fn repo_in(&self, name: &str, format: &str) -> Repo {
        let repo = Repo {
            path: self.root.join("repos").join(name),
            author: self.author.clone(),
        };
        fs::create_dir_all(&repo.path).unwrap();
        repo.git(
            &[
                "init",
                "--bare",
                "--quiet",
                "--initial-branch=main",
                &format!("--object-format={format}"),
                ".",
            ],
            None,
        );
        repo
    }

    fn skills(&self, sources: Vec<Value>, mandatory: Vec<Value>) {
        write(
            &self.home.join(SETTINGS),
            &json!({"skills": {"schemaVersion": 1, "sources": sources, "mandatory": mandatory}})
                .to_string(),
        );
    }

    /// A local source directory outside the working directory.
    fn local(&self, name: &str, files: &[(&str, &str)]) -> PathBuf {
        let source = self.root.join("sources").join(name);
        for (path, contents) in files {
            write(&source.join(path), contents);
        }
        source
    }

    fn resolve(&self) -> Output {
        self.command()
            .args(["skills", "resolve", "--json"])
            .output()
            .unwrap()
    }

    fn resolved(&self) -> Value {
        let output = self.resolve();
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        serde_json::from_slice(&output.stdout).unwrap()
    }

    /// The failure document of a refused resolution.
    fn refused(&self, exit: i32) -> Value {
        refused(&self.resolve(), exit)
    }

    fn start(&self, prompt: &str) -> Run {
        let mut command = self.command();
        command.args([
            "start-session",
            "--kind",
            "check",
            "--json",
            "--prompt",
            prompt,
        ]);
        run(&mut command)
    }

    /// A spy `git` first on the private `PATH`. It records one line for each
    /// child and then runs the real Git.
    fn spy(&self) -> PathBuf {
        let log = self.tmp.join("git-spy.log");
        let spy = self.bin.join("git");
        write(
            &spy,
            &format!(
                r#"#!/bin/sh
{{
  env | sed 's/=.*//' | grep -E '^(GIT_|RAVEL_|HARNESS_DISPATCH_|LC_ALL$|SSH_ASKPASS_REQUIRE$)' | sort | tr '\n' ' '
  printf '|'
  if [ /dev/stdin -ef /dev/null ]; then printf null; else printf other; fi
  printf '|'
  if (: < /dev/tty) 2>/dev/null; then printf tty; else printf notty; fi
  printf '|'
  if [ -e /dev/fd/9 ]; then printf fd9; else printf nofd9; fi
  printf '|%s\n' "$*"
}} >> {log:?}
exec {GIT} "$@"
"#
            ),
        );
        fs::set_permissions(&spy, fs::Permissions::from_mode(0o755)).unwrap();
        log
    }

    fn entries(&self, name: &str) -> Vec<String> {
        match fs::read_dir(self.home.join(CACHE).join(name)) {
            Err(_) => Vec::new(),
            Ok(listing) => listing
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect(),
        }
    }

    /// Ravel's own files in the private `TMPDIR`.
    fn left_in_tmp(&self) -> Vec<String> {
        fs::read_dir(&self.tmp)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("ravel-"))
            .collect()
    }
}

/// A bare fixture repository, written with Git plumbing so that a tree can
/// hold any mode and name.
struct Repo {
    path: PathBuf,
    author: PathBuf,
}

impl Repo {
    fn git(&self, args: &[&str], input: Option<&[u8]>) -> String {
        let mut command = Command::new(GIT);
        command
            .env_clear()
            .env("HOME", &self.author)
            .env("PATH", "/usr/bin:/bin")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_AUTHOR_NAME", "Fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
            .env("GIT_AUTHOR_DATE", "2026-01-01T00:00:00Z")
            .env("GIT_COMMITTER_NAME", "Fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
            .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00Z")
            .arg("-C")
            .arg(&self.path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.unwrap_or_default())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "git {args:?}: {}", stderr(&output));
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn blob(&self, bytes: &[u8]) -> String {
        self.git(&["hash-object", "-w", "--stdin"], Some(bytes))
    }

    /// A tree of `(mode, object, name)` entries.
    fn tree(&self, entries: &[(&str, &str, &str)]) -> String {
        let mut input = String::new();
        for (mode, object, name) in entries {
            let kind = match *mode {
                "040000" => "tree",
                "160000" => "commit",
                _ => "blob",
            };
            input.push_str(&format!("{mode} {kind} {object}\t{name}\n"));
        }
        self.git(&["mktree"], Some(input.as_bytes()))
    }

    /// A tree from `(path, mode, contents)` files, with nested directories.
    fn files(&self, files: &[(&str, &str, &[u8])]) -> String {
        let mut here: Vec<(String, String, String)> = Vec::new();
        let mut nested: Vec<Nested> = Vec::new();
        for (path, mode, contents) in files {
            match path.split_once('/') {
                None => {
                    let object = if *mode == "160000" {
                        String::from_utf8(contents.to_vec()).unwrap()
                    } else {
                        self.blob(contents)
                    };
                    here.push((mode.to_string(), object, path.to_string()));
                }
                Some((first, rest)) => {
                    let file = (rest.to_owned(), mode.to_string(), contents.to_vec());
                    match nested.iter_mut().find(|(name, _)| name == first) {
                        Some((_, list)) => list.push(file),
                        None => nested.push((first.to_owned(), vec![file])),
                    }
                }
            }
        }
        for (name, list) in nested {
            let list: Vec<(&str, &str, &[u8])> = list
                .iter()
                .map(|(path, mode, contents)| (path.as_str(), mode.as_str(), contents.as_slice()))
                .collect();
            here.push(("040000".to_owned(), self.files(&list), name));
        }
        let entries: Vec<(&str, &str, &str)> = here
            .iter()
            .map(|(mode, object, name)| (mode.as_str(), object.as_str(), name.as_str()))
            .collect();
        self.tree(&entries)
    }

    fn commit(&self, tree: &str, parents: &[&str]) -> String {
        let mut args = vec!["commit-tree", tree, "-m", "fixture"];
        for parent in parents {
            args.extend(["-p", parent]);
        }
        self.git(&args, None)
    }

    /// A commit of `files`, which the branch `main` then names.
    fn commit_files(&self, files: &[(&str, &str, &[u8])], parents: &[&str]) -> String {
        let commit = self.commit(&self.files(files), parents);
        self.git(&["update-ref", "refs/heads/main", &commit], None);
        commit
    }

    fn url(&self) -> String {
        format!("file://{}", self.path.display())
    }
}

fn git_source(id: &str, url: &str, revision: &str, subdir: &str, roots: &[&str]) -> Value {
    json!({"id": id, "type": "git", "url": url, "revision": revision, "subdir": subdir, "roots": roots})
}

fn local(id: &str, path: &Path, roots: &[&str]) -> Value {
    json!({ "id": id, "type": "local", "path": path, "roots": roots })
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn refused(output: &Output, exit: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit), "{}", stderr(output));
    assert!(output.stdout.is_empty());
    let document: Value = serde_json::from_slice(&output.stderr).unwrap();
    document["error"].clone()
}

fn snapshot(resolved: &Value, index: usize) -> PathBuf {
    PathBuf::from(resolved["sources"][index]["snapshotRoot"].as_str().unwrap())
}

const F: &str = "100644";

/// One file of a fixture tree: its path, its Git mode and its contents.
type File<'a> = (&'a str, &'a str, &'a [u8]);
/// The files below one directory name, owned while a tree is built.
type Nested = (String, Vec<(String, String, Vec<u8>)>);

#[test]
fn only_the_full_pin_is_extracted_with_two_commits_and_a_moved_tag() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let first = repo.commit_files(&[("pack/skills/a/SKILL.md", F, b"one\n")], &[]);
    repo.git(&["tag", "moving", &first], None);
    let second = repo.commit_files(&[("pack/skills/a/SKILL.md", F, b"two\n")], &[&first]);
    // The tag moves to the newer commit. The pin keeps naming the older one.
    repo.git(&["tag", "-f", "moving", &second], None);
    for (url, revision, expected) in [
        (repo.url(), &first, "one\n"),
        (repo.path.display().to_string(), &second, "two\n"),
    ] {
        fixture.skills(
            vec![git_source("pack", &url, revision, "pack", &["skills"])],
            vec![],
        );
        let resolved = fixture.resolved();
        let root = snapshot(&resolved, 0);
        assert_eq!(
            fs::read_to_string(root.join("skills/a/SKILL.md")).unwrap(),
            expected
        );
        let record = &resolved["sources"][0];
        let mut keys: Vec<&str> = record
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "id",
                "resourceDigest",
                "revision",
                "snapshotRoot",
                "subdir",
                "type",
                "url"
            ],
            "{record}"
        );
        assert_eq!(record["type"], "git");
        assert_eq!(record["url"], url.as_str());
        assert_eq!(record["revision"], revision.as_str());
        assert_eq!(record["subdir"], "pack");
        assert_eq!(
            resolved["directories"][0]["path"],
            root.join("skills").to_str().unwrap()
        );
        // The descriptor holds the same source record.
        let descriptor: Value = serde_json::from_slice(
            &fs::read(resolved["descriptorPath"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(descriptor["sources"], resolved["sources"]);
    }
    // Only the published objects remain; each acquisition removed its staging.
    assert!(fixture.entries("staging").is_empty());
}

#[test]
fn a_ref_an_abbreviation_and_a_malformed_pin_refuse_in_the_settings_before_any_git_child() {
    let fixture = Fixture::new();
    let log = fixture.spy();
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(&[("SKILL.md", F, b"x")], &[]);
    let local_source = fixture.local("core", &[("SKILL.md", "x")]);
    for (revision, field) in [
        ("main", "revision"),
        (&commit[..12], "revision"),
        (&commit.to_uppercase(), "revision"),
    ] {
        fixture.skills(
            vec![
                local("core", &local_source, &["."]),
                git_source("pack", &repo.url(), revision, ".", &["."]),
            ],
            vec![],
        );
        let error = fixture.refused(2);
        assert_eq!(error["code"], "settings_invalid", "{error}");
        assert_eq!(error["stage"], "cli", "{error}");
        assert_eq!(error["location"], format!("skills.sources[1].{field}"));
    }
    assert!(!fixture.home.join(".cache").exists());
    assert!(!log.exists(), "a Git child ran");
}

#[test]
fn a_tag_object_an_absent_commit_and_a_bad_subdir_have_distinct_remedies() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(
        &[
            ("pack/SKILL.md", F, b"x"),
            ("file.md", F, b"not a directory"),
        ],
        &[],
    );
    repo.git(&["tag", "-a", "-m", "release", "release", &commit], None);
    let tag = repo.git(&["rev-parse", "release"], None);
    assert_ne!(tag, commit);
    let absent = "0123456789abcdef0123456789abcdef01234567";
    let cases = [
        (
            tag.as_str(),
            "pack",
            "revision",
            "a tag object",
            "never peels a tag object",
        ),
        (
            absent,
            "pack",
            "revision",
            "does not hold the commit",
            "make the commit",
        ),
        (
            commit.as_str(),
            "nope",
            "subdir",
            "\"nope\" does not exist",
            "change the subdir",
        ),
        (
            commit.as_str(),
            "file.md",
            "subdir",
            "\"file.md\" is not a directory",
            "change the subdir",
        ),
    ];
    for (revision, subdir, field, message, remedy) in cases {
        fixture.skills(
            vec![git_source("pack", &repo.url(), revision, subdir, &["."])],
            vec![],
        );
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_revision_unavailable", "{error}");
        assert_eq!(error["stage"], "skills");
        assert_eq!(error["location"], format!("skills.sources[0].{field}"));
        let text = error["message"].as_str().unwrap();
        assert!(text.contains(message), "{text}");
        assert!(text.contains("\"pack\""), "{text}");
        assert!(text.contains(revision), "{text}");
        let advice = error["remedy"].as_str().unwrap();
        assert!(advice.contains(remedy), "{advice}");
        // No remedy names a revision that the owner did not configure.
        for other in [&commit, &tag] {
            if other != revision {
                assert!(!text.contains(other.as_str()), "{text}");
                assert!(!advice.contains(other.as_str()), "{advice}");
            }
        }
        assert!(fixture.entries("descriptors").is_empty());
        assert!(fixture.entries("staging").is_empty());
    }
}

#[test]
fn a_missing_git_executable_and_a_failed_transport_refuse_as_revision_unavailable() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(&[("SKILL.md", F, b"x")], &[]);
    fixture.skills(
        vec![git_source("pack", &repo.url(), &commit, ".", &["."])],
        vec![],
    );
    let empty = fixture.root.join("empty-bin");
    fs::create_dir_all(&empty).unwrap();
    let output = fixture
        .command()
        .env("PATH", &empty)
        .args(["skills", "resolve", "--json"])
        .output()
        .unwrap();
    let error = refused(&output, 3);
    assert_eq!(error["code"], "skill_revision_unavailable", "{error}");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains("no git executable"), "{message}");
    assert!(message.contains(&commit), "{message}");
    assert!(error["remedy"].as_str().unwrap().contains("PATH"));
    // The control: the same settings resolve with Git on the PATH.
    fixture.resolved();

    let gone = fixture.root.join("repos/gone");
    fixture.skills(
        vec![git_source(
            "pack",
            gone.to_str().unwrap(),
            &commit,
            ".",
            &["."],
        )],
        vec![],
    );
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_revision_unavailable", "{error}");
    assert_eq!(error["location"], "skills.sources[0]");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("Git cannot fetch from")
    );
    assert!(error["remedy"].as_str().unwrap().contains("restore access"));
}

#[test]
fn a_pin_of_the_other_object_format_refuses_and_a_sha256_pin_resolves() {
    let fixture = Fixture::new();
    let repo = fixture.repo_in("wide", "sha256");
    let commit = repo.commit_files(&[("SKILL.md", F, b"wide")], &[]);
    assert_eq!(commit.len(), 64);
    fixture.skills(
        vec![git_source("wide", &repo.url(), &commit, ".", &["."])],
        vec![],
    );
    let resolved = fixture.resolved();
    assert_eq!(
        fs::read_to_string(snapshot(&resolved, 0).join("SKILL.md")).unwrap(),
        "wide"
    );
    fixture.skills(
        vec![git_source(
            "wide",
            &repo.url(),
            "0123456789abcdef0123456789abcdef01234567",
            ".",
            &["."],
        )],
        vec![],
    );
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_revision_unavailable", "{error}");
    assert_eq!(error["location"], "skills.sources[0].revision");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("the sha1 object format"),
        "{error}"
    );
}

#[test]
fn a_commit_that_the_remote_does_not_advertise_arrives_through_the_advertised_refs() {
    let fixture = Fixture::new();
    let log = fixture.spy();
    let repo = fixture.repo("pack");
    let older = repo.commit_files(&[("SKILL.md", F, b"older")], &[]);
    repo.commit_files(&[("SKILL.md", F, b"newer")], &[&older]);
    // Protocol 0 refuses a want that no ref tip names, so the direct fetch
    // of the older commit fails and the fetch of the advertised refs follows.
    write(
        &fixture.home.join(".gitconfig"),
        "[protocol]\n\tversion = 0\n",
    );
    fixture.skills(
        vec![git_source("pack", &repo.url(), &older, ".", &["."])],
        vec![],
    );
    let resolved = fixture.resolved();
    assert_eq!(
        fs::read_to_string(snapshot(&resolved, 0).join("SKILL.md")).unwrap(),
        "older"
    );
    let calls = fs::read_to_string(&log).unwrap();
    let fetches: Vec<&str> = calls
        .lines()
        .filter(|line| line.contains(" fetch "))
        .collect();
    assert_eq!(fetches.len(), 2, "{calls}");
    assert!(fetches[0].contains("--depth=1"), "{calls}");
    assert!(fetches[1].contains("+refs/*:refs/fetched/*"), "{calls}");
}

#[test]
fn every_git_child_is_noninteractive_and_inherits_no_redirection_or_completion_authority() {
    let fixture = Fixture::new();
    let log = fixture.spy();
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(&[("SKILL.md", F, b"x")], &[]);
    fixture.skills(
        vec![git_source("pack", &repo.url(), &commit, ".", &["."])],
        vec![],
    );
    let channel = fixture.tmp.join("channel");
    write(&channel, "");
    let held = fs::File::open(&channel).unwrap();
    let mut command = fixture.command();
    command
        .args(["skills", "resolve", "--json"])
        .env("GIT_DIR", fixture.root.join("elsewhere"))
        .env("GIT_WORK_TREE", fixture.root.join("elsewhere"))
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "core.hooksPath")
        .env("GIT_CONFIG_VALUE_0", fixture.root.join("hooks"))
        .env("GIT_SSH_COMMAND", "false")
        .env("RAVEL_EXIT_FILE", &channel)
        .env("RAVEL_RUN_ID", "parent")
        .env("HARNESS_DISPATCH_EXIT_FILE", &channel)
        .env("LC_ALL", "fr_FR.UTF-8")
        // A regular file, so that a child that inherited it would show it.
        .stdin(fs::File::open(&channel).unwrap());
    let fd = std::os::fd::AsRawFd::as_raw_fd(&held);
    // SAFETY: `dup2` is async-signal-safe and both descriptors are open. The
    // copy at 9 has no close-on-exec flag, like a caller's completion
    // descriptor.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(fd, 9) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let calls = fs::read_to_string(&log).unwrap();
    assert!(calls.lines().count() >= 4, "{calls}");
    for line in calls.lines() {
        let fields: Vec<&str> = line.splitn(5, '|').collect();
        assert_eq!(
            fields[0],
            "GIT_ASKPASS GIT_NO_REPLACE_OBJECTS GIT_TERMINAL_PROMPT LC_ALL SSH_ASKPASS_REQUIRE ",
            "{line}"
        );
        assert_eq!(&fields[1..4], ["null", "notty", "nofd9"], "{line}");
    }

    // The controls: the spy sees each property when a caller gives it.
    let control = Command::new(fixture.bin.join("git"))
        .arg("--version")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_DIR", "x")
        .stdin(fs::File::open(&channel).unwrap())
        .output()
        .unwrap();
    assert!(control.status.success());
    let last = fs::read_to_string(&log).unwrap();
    let last = last.lines().last().unwrap();
    assert!(last.starts_with("GIT_DIR "), "{last}");
    assert!(last.contains("|other|"), "{last}");
    let mut inherited = Command::new(fixture.bin.join("git"));
    inherited
        .arg("--version")
        .env_clear()
        .env("PATH", "/usr/bin:/bin");
    // SAFETY: as above.
    unsafe {
        inherited.pre_exec(move || {
            if libc::dup2(fd, 9) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    assert!(inherited.status().unwrap().success());
    let last = fs::read_to_string(&log).unwrap();
    assert!(last.lines().last().unwrap().contains("|fd9|"), "{last}");
}

#[test]
fn no_hook_filter_or_lfs_transfer_runs_and_a_pointer_is_captured_byte_for_byte() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let pointer = b"version https://git-lfs.github.com/spec/v1\noid sha256:\
        4d7a214614ab2935c943f9e0ff69d22eadbb8f32b1258daaa5e2ca24d17e2393\nsize 12\n";
    let older = repo.commit_files(&[("SKILL.md", F, b"older")], &[]);
    let commit = repo.commit_files(
        &[
            (
                ".gitattributes",
                F,
                b"*.bin filter=lfs diff=lfs merge=lfs -text\n* ident\n",
            ),
            ("model.bin", F, pointer),
            ("SKILL.md", F, b"$Id$\r\n"),
        ],
        &[&older],
    );
    // The owner's global configuration names hooks, a template with hooks and
    // a smudge filter, each of which leaves a marker when Git runs it.
    let markers = fixture.root.join("markers");
    fs::create_dir_all(&markers).unwrap();
    let hook = |path: &Path, name: &str| {
        write(
            path,
            &format!(
                "#!/bin/sh\ntouch {:?}\ncat >/dev/null\nexit 0\n",
                markers.join(name)
            ),
        );
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    };
    let hooks = fixture.root.join("hooks");
    let template = fixture.root.join("template");
    for name in ["reference-transaction", "post-checkout"] {
        hook(&hooks.join(name), &format!("hooks-{name}"));
        hook(
            &template.join("hooks").join(name),
            &format!("template-{name}"),
        );
    }
    let smudge = fixture.root.join("smudge");
    write(
        &smudge,
        &format!("#!/bin/sh\ntouch {:?}\ncat\n", markers.join("smudge")),
    );
    fs::set_permissions(&smudge, fs::Permissions::from_mode(0o755)).unwrap();
    write(
        &fixture.home.join(".gitconfig"),
        &format!(
            "[core]\n\thooksPath = {}\n[init]\n\ttemplateDir = {}\n[filter \"lfs\"]\n\t\
             smudge = {}\n\trequired = true\n[protocol]\n\tversion = 0\n",
            hooks.display(),
            template.display(),
            smudge.display()
        ),
    );
    // Protocol 0 forces the fetch of the advertised refs, which updates refs,
    // so a reference-transaction hook would run if Ravel allowed it.
    fixture.skills(
        vec![git_source("pack", &repo.url(), &older, ".", &["."])],
        vec![],
    );
    fixture.resolved();
    fixture.skills(
        vec![git_source("pack", &repo.url(), &commit, ".", &["."])],
        vec![],
    );
    let resolved = fixture.resolved();
    let root = snapshot(&resolved, 0);
    assert_eq!(fs::read(root.join("model.bin")).unwrap(), pointer);
    // No ident expansion and no line-ending conversion.
    assert_eq!(fs::read(root.join("SKILL.md")).unwrap(), b"$Id$\r\n");
    let seen: Vec<String> = fs::read_dir(&markers)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    assert!(seen.is_empty(), "{seen:?}");

    // The control: an ordinary clone and fetch with the same HOME run each
    // hook and the filter.
    let work = fixture.root.join("control");
    let ordinary = |args: &[&std::ffi::OsStr]| {
        let status = Command::new(GIT)
            .env_clear()
            .env("HOME", &fixture.home)
            .env("PATH", "/usr/bin:/bin")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "{args:?}");
    };
    ordinary(&[
        "clone".as_ref(),
        "-q".as_ref(),
        repo.path.as_os_str(),
        work.as_os_str(),
    ]);
    let mut seen: Vec<String> = fs::read_dir(&markers)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    seen.sort();
    assert_eq!(
        seen,
        [
            "hooks-post-checkout",
            "hooks-reference-transaction",
            "smudge"
        ]
    );
    // With the hooks path of the repository itself, the hooks of the
    // template run.
    let bare = fixture.root.join("control.git");
    let own = format!("core.hooksPath={}", bare.join("hooks").display());
    ordinary(&[
        "init".as_ref(),
        "-q".as_ref(),
        "--bare".as_ref(),
        bare.as_os_str(),
    ]);
    ordinary(&[
        "-c".as_ref(),
        own.as_ref(),
        format!("--git-dir={}", bare.display()).as_ref(),
        "fetch".as_ref(),
        "-q".as_ref(),
        repo.path.as_os_str(),
        "+refs/heads/*:refs/heads/*".as_ref(),
    ]);
    assert!(markers.join("template-reference-transaction").exists());
}

#[test]
fn modes_links_and_gitlinks_follow_the_rules_of_a_local_source() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let submodule = repo.commit_files(&[("x", F, b"x")], &[]);
    let base: Vec<(&str, &str, &[u8])> = vec![
        ("skills/alpha/SKILL.md", F, b"alpha"),
        ("skills/alpha/run.sh", "100755", b"#!/bin/sh\n"),
        (
            "skills/alpha/linked.md",
            "120000",
            b"../../shared/common.md",
        ),
        ("shared/common.md", F, b"parent-relative"),
        (".jj/working_copy", F, b"left out"),
    ];
    let commit = repo.commit_files(&base, &[]);
    fixture.skills(
        vec![git_source("pack", &repo.url(), &commit, ".", &["skills"])],
        vec![json!({"source": "pack", "path": "skills/alpha/linked.md"})],
    );
    let resolved = fixture.resolved();
    let root = snapshot(&resolved, 0);
    assert_eq!(
        fs::read_to_string(root.join("skills/alpha/linked.md")).unwrap(),
        "parent-relative"
    );
    let mode = |path: &str| fs::metadata(root.join(path)).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode("skills/alpha/run.sh"), 0o555);
    assert_eq!(mode("skills/alpha/SKILL.md"), 0o444);
    assert!(!root.join(".jj").exists());

    let cases: Vec<(Vec<File<'_>>, &str, &str)> = vec![
        (
            vec![("skills/escape.md", "120000", b"../../outside")],
            "skills/escape.md",
            "leaves the source",
        ),
        (
            vec![("skills/dangling.md", "120000", b"nowhere")],
            "skills/dangling.md",
            "is dangling",
        ),
        (
            vec![("skills/absolute.md", "120000", b"/etc/passwd")],
            "skills/absolute.md",
            "is absolute",
        ),
        (
            vec![("vendor/sub", "160000", submodule.as_bytes())],
            "vendor/sub",
            "gitlink",
        ),
    ];
    for (extra, path, why) in cases {
        let mut files = base.clone();
        files.extend(extra);
        let commit = repo.commit_files(&files, &[]);
        fixture.skills(
            vec![git_source("pack", &repo.url(), &commit, ".", &["skills"])],
            vec![],
        );
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_resource_invalid", "{error}");
        let message = error["message"].as_str().unwrap();
        assert!(message.contains(why), "{message}");
        // The refusal names the path in the pin, not in private staging.
        let shown = format!("{}@{commit}/{path}", repo.url());
        assert!(message.contains(&shown), "{message}");
        assert_eq!(error["source"], shown.as_str());
        assert!(fixture.entries("staging").is_empty());
    }
}

#[test]
fn names_that_differ_only_in_case_are_kept_or_refused_by_the_cache_filesystem() {
    let fixture = Fixture::new();
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(
        &[("s/x/SKILL.md", F, b"upper"), ("s/x/skill.md", F, b"lower")],
        &[],
    );
    fixture.skills(
        vec![git_source("pack", &repo.url(), &commit, ".", &["s"])],
        vec![],
    );
    let probe = fixture.home.join("probe");
    write(&probe.join("a"), "");
    let insensitive = probe.join("A").exists();
    if insensitive {
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_resource_invalid", "{error}");
        let message = error["message"].as_str().unwrap();
        for path in ["s/x/SKILL.md", "s/x/skill.md"] {
            assert!(
                message.contains(&format!("{}@{commit}/{path}", repo.url())),
                "{message}"
            );
        }
        assert!(fixture.entries("staging").is_empty());
    } else {
        let root = snapshot(&fixture.resolved(), 0);
        assert_eq!(
            fs::read_to_string(root.join("s/x/SKILL.md")).unwrap(),
            "upper"
        );
        assert_eq!(
            fs::read_to_string(root.join("s/x/skill.md")).unwrap(),
            "lower"
        );
    }
}

#[test]
fn a_local_repository_path_obeys_the_protected_overlap_rules() {
    let fixture = Fixture::new();
    let config = fixture.home.join(".config/ravel");
    fs::create_dir_all(&config).unwrap();
    let inside = Repo {
        path: config.join("repo"),
        author: fixture.author.clone(),
    };
    fs::create_dir_all(&inside.path).unwrap();
    inside.git(&["init", "--bare", "--quiet", "."], None);
    let commit = inside.commit_files(&[("SKILL.md", F, b"x")], &[]);
    let alias = fixture.root.join("alias");
    symlink(&inside.path, &alias).unwrap();
    for url in [
        inside.path.display().to_string(),
        inside.url(),
        format!("file://{}", alias.display()),
    ] {
        fixture.skills(vec![git_source("pack", &url, &commit, ".", &["."])], vec![]);
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_access_unavailable", "{url}: {error}");
        assert_eq!(error["location"], "skills.sources[0].url");
        assert!(!fixture.home.join(".cache").exists(), "{url}");
    }
}

/// A managed program that reads one file from each of two snapshot roots,
/// which the configuration script passes as arguments.
fn reader(fixture: &Fixture) {
    let path = fixture.bin.join("reader");
    write(
        &path,
        &format!("#!/bin/sh\nexec {FRONT:?} end-session \"$(cat \"$1\")|$(cat \"$2\")\"\n"),
    );
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    fixture.personal_rhai(&rhai_script(
        r#"fn select(request, context) {
    let local = request.skills.sources[0].snapshotRoot;
    let pinned = request.skills.sources[1].snapshotRoot;
    #{ status: "selected", program: "reader",
       args: [`${local}/skills/l/SKILL.md`, `${pinned}/skills/g/linked.md`],
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture" }
}"#,
    ));
}

#[test]
fn a_launch_with_mixed_local_and_git_sources_reads_both_and_reports_each_disposition() {
    let fixture = Fixture::new();
    let core = fixture.local("core", &[("skills/l/SKILL.md", "local skill")]);
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(
        &[
            ("pack/skills/g/SKILL.md", F, b"git skill"),
            ("pack/skills/g/linked.md", "120000", b"../../shared.md"),
            ("pack/shared.md", F, b"git resource"),
            ("outside.md", F, b"outside the subdir"),
        ],
        &[],
    );
    fixture.skills(
        vec![
            local("core", &core, &["skills"]),
            git_source(
                "pack",
                repo.path.to_str().unwrap(),
                &commit,
                "pack",
                &["skills"],
            ),
        ],
        vec![json!({"source": "pack", "path": "skills/g/SKILL.md"})],
    );
    reader(&fixture);
    let launched = fixture.start("read both");
    assert_eq!(launched.code, Some(0), "{}", launched.stderr);
    assert_eq!(launched.stdout, "local skill|git resource");
    let notice: Value = serde_json::from_str(launched.stderr.lines().next().unwrap()).unwrap();
    let resolved = fixture.resolved();
    assert!(!snapshot(&resolved, 1).join("outside.md").exists());
    let sources = notice["skills"]["sources"].as_array().unwrap();
    assert_eq!(sources[0]["disposition"], "captured", "{notice}");
    assert_eq!(sources[1]["disposition"], "acquired", "{notice}");
    assert_eq!(sources[1]["revision"], commit.as_str(), "{notice}");
    assert_eq!(
        notice["skills"]["descriptorPath"],
        resolved["descriptorPath"]
    );
    assert_eq!(
        resolved["mandatory"][0]["path"],
        snapshot(&resolved, 1)
            .join("skills/g/SKILL.md")
            .to_str()
            .unwrap()
    );
    assert!(fixture.entries("staging").is_empty());
}

#[test]
fn a_failed_git_source_in_a_mixed_set_leaves_no_descriptor_and_launches_nothing() {
    let fixture = Fixture::new();
    let core = fixture.local("core", &[("skills/l/SKILL.md", "local skill")]);
    let repo = fixture.repo("pack");
    let commit = repo.commit_files(&[("pack/skills/g/SKILL.md", F, b"g")], &[]);
    fixture.personal_rhai(UNPARSED);
    for (revision, subdir) in [
        ("0123456789abcdef0123456789abcdef01234567", "pack"),
        (commit.as_str(), "missing"),
    ] {
        fixture.skills(
            vec![
                local("core", &core, &["skills"]),
                git_source("pack", &repo.url(), revision, subdir, &["skills"]),
            ],
            vec![],
        );
        let refused = fixture.start("p");
        let document = refused.refusal(3);
        assert_eq!(
            document["error"]["code"], "skill_revision_unavailable",
            "{document}"
        );
        assert_eq!(refused.stderr.lines().count(), 1, "{}", refused.stderr);
        assert!(fixture.left_in_tmp().is_empty());
        // The local source published its snapshot. No descriptor names it.
        assert!(fixture.entries("descriptors").is_empty());
        assert_eq!(fixture.entries("snapshots").len(), 1);
        assert!(fixture.entries("staging").is_empty());
    }
    // The control: with the pin repaired, the worker reports the parse error.
    fixture.skills(
        vec![
            local("core", &core, &["skills"]),
            git_source("pack", &repo.url(), &commit, "pack", &["skills"]),
        ],
        vec![],
    );
    let refused = fixture.start("p");
    assert_eq!(refused.refusal(3)["error"]["code"], "config_parse_failed");
}
