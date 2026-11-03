//! The lifetime of retained skill resources, nested sessions and the recovery
//! environment, at the public `ravel skills resolve` and `start-session`
//! seams, with local sources.
//! docs/specs/skill-discovery.md, "`RAVEL_SKILL_DIRECTORIES`", "Publication,
//! concurrency and cleanup", "Read access and delegation" and "Test seams and
//! acceptance".
//!
//! These rows run here: "Lifetime", "Nested sessions" and the fixture part of
//! "Directory recovery". Agent behaviour after a
//! compaction belongs to the actual harness trials.
//!
//! The fixture programs read the descriptor with `sed` and `grep`. The
//! descriptor is compact JSON, and the temporary paths hold no quote or
//! bracket.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{FRONT, Run, Sandbox, run, write};

const SETTINGS: &str = ".config/ravel/settings.json";
const CACHE: &str = ".cache/ravel/skills/v1";

/// A configuration script that cannot parse: a refusal before it proves that
/// no worker evaluated the entry.
const UNPARSED: &str = "export const SCHEMA_VERSION = 4;\nfn select(request, context) {\n";

/// `paths DESCRIPTOR KEY` prints the `path` of each record of the array `KEY`
/// of a descriptor, one on each line.
const PATHS: &str = r#"paths() {
  sed -n "s/.*\"$2\":\[\([^]]*\)\].*/\1/p" "$1" | grep -o '"path":"[^"]*"' | sed 's/^"path":"//; s/"$//'
}"#;

/// A sandbox whose snapshots, which are read-only, are made writable at the
/// end, so that the temporary directory can go.
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

    /// A source directory outside the working directory.
    fn source(&self, name: &str, files: &[(&str, &str)]) -> PathBuf {
        let source = self.root.join("sources").join(name);
        fs::create_dir_all(&source).unwrap();
        for (path, contents) in files {
            write(&source.join(path), contents);
        }
        source
    }

    fn settings(&self, value: &Value) {
        write(&self.home.join(SETTINGS), &value.to_string());
    }

    fn skills(&self, sources: Vec<Value>, mandatory: Vec<Value>) {
        self.settings(&json!({ "skills": {
            "schemaVersion": 1, "sources": sources, "mandatory": mandatory
        }}));
    }

    /// A program on the private `PATH`, with the `paths` function.
    fn program(&self, name: &str, body: &str) -> PathBuf {
        let path = self.bin.join(name);
        write(
            &path,
            &format!("#!/bin/sh\nFRONT={FRONT:?}\n{PATHS}\n{body}\n"),
        );
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn resolve(&self) -> Run {
        let mut command = self.command();
        command.args(["skills", "resolve", "--json"]);
        run(&mut command)
    }

    fn resolve_json(&self) -> Value {
        let resolved = self.resolve();
        assert_eq!(resolved.code, Some(0), "{}", resolved.stderr);
        serde_json::from_str(&resolved.stdout).unwrap()
    }

    fn start_with(&self, kind: &str, prompt: &str, inherited: Option<&str>) -> Run {
        let mut command = self.command();
        command.args([
            "start-session",
            "--kind",
            kind,
            "--json",
            "--prompt",
            prompt,
        ]);
        if let Some(value) = inherited {
            command.env("RAVEL_SKILL_DIRECTORIES", value);
        }
        run(&mut command)
    }

    fn cache(&self) -> PathBuf {
        self.home.join(CACHE)
    }

    fn objects(&self, kind: &str) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(self.cache().join(kind))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }
}

/// The routing notice of a successful launch whose result need not be `done`.
fn notice(launch: &Run) -> Value {
    assert_eq!(launch.code, Some(0), "{}", launch.stderr);
    serde_json::from_str(launch.stderr.lines().next().expect("routing notice")).unwrap()
}

fn local(id: &str, path: &Path, roots: &[&str]) -> Value {
    json!({ "id": id, "type": "local", "path": path, "roots": roots })
}

/// A route by kind: each kind of the table selects its own program, with the
/// prepared prompt as its only argument.
fn routes(table: &[(&str, &str)]) -> String {
    let entries: Vec<String> = table
        .iter()
        .map(|(kind, program)| format!("{kind:?}: {program:?}"))
        .collect();
    rhai_script(&format!(
        r#"const TABLE = #{{ {} }};
fn select(request, context) {{
    #{{ status: "selected", program: global::TABLE[request.kind], args: [request.prompt],
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture" }}
}}"#,
        entries.join(", ")
    ))
}

/// Wait until `path` exists, for at most a minute.
fn await_file(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "{} never appeared",
            path.display()
        );
        thread::sleep(Duration::from_millis(20));
    }
}

fn pid(path: &Path) -> i32 {
    fs::read_to_string(path).unwrap().trim().parse().unwrap()
}

fn alive(pid: i32) -> bool {
    // SAFETY: signal zero sends nothing. It checks whether the fixture
    // process exists.
    unsafe { libc::kill(pid, 0) == 0 }
}

fn await_gone(pid: i32) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while alive(pid) {
        assert!(Instant::now() < deadline, "the process {pid} did not end");
        thread::sleep(Duration::from_millis(20));
    }
}

/// Make an object of the cache writable and remove it, as an owner does by
/// hand.
fn remove_by_hand(path: &Path) {
    assert!(
        Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(path)
            .status()
            .unwrap()
            .success()
    );
    fs::remove_dir_all(path).unwrap();
}

/// `reader DESCRIPTOR OUTPUT GATE` takes the descriptor from its argument, or
/// from `RAVEL_SKILL_DIRECTORIES` when the argument is empty. It writes its
/// process ID, waits for the gate and then follows the descriptor to the
/// mandatory file and to two files of a skill.
const READER: &str = r#"descriptor=${1:-$RAVEL_SKILL_DIRECTORIES}
out=$2
gate=$3
echo $$ > "$out.pid"
i=0
while [ ! -e "$gate" ]; do
  i=$((i + 1))
  [ "$i" -lt 3000 ] || exit 90
  sleep 0.02
done
mandatory=$(paths "$descriptor" mandatory)
directory=$(paths "$descriptor" directories)
printf '%s|%s|%s|%s' "$descriptor" "$(cat "$mandatory")" "$(cat "$directory/alpha/SKILL.md")" \
  "$(cat "$directory/alpha/notes.md")" > "$out.part" && mv "$out.part" "$out""#;

#[test]
fn independent_and_detached_readers_outlive_the_session_and_ordinary_descendants_are_reaped() {
    let fixture = Fixture::new();
    let source = fixture.source(
        "pack",
        &[
            ("MANDATORY.md", "mandatory v1"),
            ("skills/alpha/SKILL.md", "alpha v1"),
            ("skills/alpha/notes.md", "notes v1"),
        ],
    );
    fixture.skills(
        vec![local("pack", &source, &["skills"])],
        vec![json!({"source": "pack", "path": "MANDATORY.md"})],
    );
    let reader = fixture.program("reader", READER);
    let records = fixture.root.join("records");
    fs::create_dir(&records).unwrap();
    let gate = records.join("gate");
    let (detached, independent, ordinary) = (
        records.join("detached"),
        records.join("independent"),
        records.join("ordinary.pid"),
    );
    // The managed program starts one ordinary descendant in its group, and
    // one reader that leaves the group with `setsid`. It ends the session only
    // after the reader has left, and before any reader has read.
    fixture.program(
        "parent",
        &format!(
            "sleep 600 </dev/null >/dev/null 2>&1 &\necho $! > {ordinary:?}\n\
             /usr/bin/perl -MPOSIX -e 'exit 0 if fork; POSIX::setsid() >= 0 or die \"setsid: $!\"; \
             exec @ARGV or die \"exec: $!\"' reader '' {detached:?} {gate:?} \
             </dev/null >/dev/null 2>&1\n\
             i=0\nwhile [ ! -e {detached_pid:?} ]; do i=$((i + 1)); [ \"$i\" -lt 3000 ] || exit 90; \
             sleep 0.02; done\nexec \"$FRONT\" end-session done",
            detached_pid = records.join("detached.pid"),
        ),
    );
    fixture.personal_rhai(&routes(&[("check", "parent")]));

    // An independent reader takes the path from `skills resolve`, as a
    // harness that the owner opens does. It runs outside every session.
    let resolved = fixture.resolve_json();
    let resolved_path = resolved["descriptorPath"].as_str().unwrap().to_owned();
    let mut independent_reader: Child = fixture
        .command_for(&reader)
        .args([resolved_path.as_str()])
        .arg(&independent)
        .arg(&gate)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let notice = fixture.start_with("check", "p", None).report();
    let descriptor = notice["skills"]["descriptorPath"]
        .as_str()
        .unwrap()
        .to_owned();
    // The launch captured the same bytes, so it names the same descriptor.
    assert_eq!(descriptor, resolved_path);
    let descriptor_bytes = fs::read(&descriptor).unwrap();

    // The group is gone, with its ordinary descendant. The detached reader
    // still runs.
    let ordinary = pid(&ordinary);
    assert!(
        !alive(ordinary),
        "the ordinary descendant {ordinary} survived"
    );
    let detached_pid = pid(&records.join("detached.pid"));
    assert!(alive(detached_pid), "the detached reader did not survive");

    // The settings change, a new resolution publishes another descriptor and
    // the source goes.
    let other = fixture.source("other", &[("skills/beta/SKILL.md", "beta")]);
    fixture.skills(vec![local("other", &other, &["skills"])], vec![]);
    let changed = fixture.resolve_json();
    assert_ne!(changed["descriptorPath"].as_str().unwrap(), descriptor);
    fs::remove_dir_all(&source).unwrap();

    write(&gate, "");
    assert!(independent_reader.wait().unwrap().success());
    await_file(&detached);
    await_gone(detached_pid);
    let expected = format!("{descriptor}|mandatory v1|alpha v1|notes v1");
    assert_eq!(fs::read_to_string(&independent).unwrap(), expected);
    assert_eq!(fs::read_to_string(&detached).unwrap(), expected);
    assert_eq!(fs::read(&descriptor).unwrap(), descriptor_bytes);
}

#[test]
fn a_quiet_manual_removal_loses_reuse_and_never_substitutes_a_neighbour() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("skills/alpha/SKILL.md", "v1")]);
    fixture.skills(vec![local("pack", &source, &["skills"])], vec![]);
    let first = fixture.resolve_json();
    let first_root = first["sources"][0]["snapshotRoot"]
        .as_str()
        .unwrap()
        .to_owned();
    let first_snapshot = Path::new(&first_root).parent().unwrap().to_owned();
    let first_descriptor = Path::new(first["descriptorPath"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_owned();

    // The removal of the whole quiet cache keeps nothing to reuse. The next
    // resolution captures the present source again and publishes complete
    // objects at the same content paths.
    remove_by_hand(&fixture.cache());
    assert!(!fixture.cache().exists());
    let again = fixture.resolve_json();
    assert_eq!(again["descriptorPath"], first["descriptorPath"]);
    assert_eq!(
        fs::read_to_string(Path::new(&first_root).join("skills/alpha/SKILL.md")).unwrap(),
        "v1"
    );

    // A second capture gives a neighbour object of the same source ID.
    fs::write(source.join("skills/alpha/SKILL.md"), "v2").unwrap();
    let second = fixture.resolve_json();
    let second_root = second["sources"][0]["snapshotRoot"].as_str().unwrap();
    assert_ne!(second_root, first_root);
    let snapshots = fixture.objects("snapshots");
    let descriptors = fixture.objects("descriptors");
    assert_eq!((snapshots.len(), descriptors.len()), (2, 2));

    // The owner removes the first objects by hand while no reader runs, and
    // the source goes. The next resolution and the next launch refuse
    // clearly. Neither uses the neighbour, and neither publishes an object.
    remove_by_hand(&first_snapshot);
    remove_by_hand(&first_descriptor);
    fs::remove_dir_all(&source).unwrap();
    let kept = (fixture.objects("snapshots"), fixture.objects("descriptors"));
    let refused = fixture.resolve();
    assert_eq!(refused.code, Some(3), "{}", refused.stderr);
    assert!(refused.stdout.is_empty());
    let document: Value = serde_json::from_str(&refused.stderr).unwrap();
    assert_eq!(
        document["error"]["code"], "skill_source_missing",
        "{document}"
    );
    assert_eq!(
        document["error"]["source"],
        source.to_str().unwrap(),
        "{document}"
    );
    fixture.personal_rhai(UNPARSED);
    let launch = fixture.start_with("check", "p", None);
    assert_eq!(
        launch.refusal(3)["error"]["code"],
        "skill_source_missing",
        "{}",
        launch.stderr
    );
    assert_eq!(
        (fixture.objects("snapshots"), fixture.objects("descriptors")),
        kept
    );
    assert!(fixture.objects("staging").is_empty());
    assert_eq!(
        fs::read_to_string(Path::new(second_root).join("skills/alpha/SKILL.md")).unwrap(),
        "v2"
    );

    // Only a new capture brings the first objects back. The restored source
    // gives the first digest, not the neighbour.
    write(&source.join("skills/alpha/SKILL.md"), "v1");
    let restored = fixture.resolve_json();
    assert_eq!(restored["descriptorPath"], first["descriptorPath"]);
    assert_eq!(restored["sources"][0]["snapshotRoot"], first_root.as_str());
}

/// `snapshot-reader DESCRIPTOR OUTPUT STOP` reads every file of the one search
/// directory again and again, and appends one checksum for each pass, until
/// the stop file exists.
const SNAPSHOT_READER: &str = r#"directory=$(paths "$1" directories)
i=0
while [ ! -e "$3" ]; do
  i=$((i + 1))
  [ "$i" -lt 100000 ] || exit 90
  cat "$directory"/files/* | cksum >> "$2"
done"#;

fn lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect()
}

/// Wait until `path` holds more than `count` lines.
fn await_lines(path: &Path, count: usize) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while lines(path).len() <= count {
        assert!(
            Instant::now() < deadline,
            "{} stopped at {count} lines",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn concurrent_readers_of_one_snapshot_read_the_same_bytes_while_a_changed_source_publishes() {
    let fixture = Fixture::new();
    let names: Vec<String> = (0..40)
        .map(|index| format!("files/{index:02}.md"))
        .collect();
    let source = fixture.root.join("sources/pack");
    for name in &names {
        write(&source.join(name), &format!("{name} original\n"));
    }
    fixture.skills(vec![local("pack", &source, &["."])], vec![]);
    let reader = fixture.program("snapshot-reader", SNAPSHOT_READER);
    let first = fixture.resolve_json();
    let descriptor = first["descriptorPath"].as_str().unwrap().to_owned();
    let checksum = |root: &str| -> String {
        let output = Command::new("/bin/sh")
            .args(["-c", r#"cat "$1"/files/* | cksum"#, "sh", root])
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    };
    let first_root = first["sources"][0]["snapshotRoot"].as_str().unwrap();
    let expected = checksum(first_root);

    let stop = fixture.root.join("stop");
    let outputs: Vec<PathBuf> = (0..4)
        .map(|index| fixture.root.join(format!("reader-{index}")))
        .collect();
    let readers: Vec<Child> = outputs
        .iter()
        .map(|output| {
            fixture
                .command_for(&reader)
                .arg(&descriptor)
                .arg(output)
                .arg(&stop)
                .spawn()
                .unwrap()
        })
        .collect();
    for output in &outputs {
        await_lines(output, 0);
    }

    // Each round changes every file and publishes a new snapshot while the
    // readers read the first one.
    let mut roots = vec![first_root.to_owned()];
    for round in 0..3 {
        for name in &names {
            fs::write(source.join(name), format!("{name} round {round}\n")).unwrap();
        }
        let changed = fixture.resolve_json();
        let root = changed["sources"][0]["snapshotRoot"]
            .as_str()
            .unwrap()
            .to_owned();
        assert!(!roots.contains(&root), "round {round} reused {root}");
        assert_eq!(
            fs::read_to_string(Path::new(&root).join(&names[0])).unwrap(),
            format!("{} round {round}\n", names[0])
        );
        roots.push(root);
    }
    let counts: Vec<usize> = outputs.iter().map(|output| lines(output).len()).collect();
    for (output, count) in outputs.iter().zip(counts) {
        await_lines(output, count);
    }
    write(&stop, "");
    for mut reader in readers {
        assert!(reader.wait().unwrap().success());
    }
    for output in &outputs {
        let seen = lines(output);
        assert!(seen.len() >= 2, "{seen:?}");
        assert!(
            seen.iter().all(|line| *line == expected),
            "{}: {seen:?}",
            output.display()
        );
    }
    assert_eq!(checksum(first_root), expected);
}

#[test]
fn a_nested_start_session_resolves_current_sources_and_returns_only_its_own_result() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("skills/alpha/SKILL.md", "v1")]);
    fixture.skills(vec![local("pack", &source, &["skills"])], vec![]);
    let records = fixture.root.join("records");
    fs::create_dir(&records).unwrap();
    let skill = source.join("skills/alpha/SKILL.md");
    // The outer agent reads its skill, changes the source and starts a nested
    // session. Then it checks its own variables and reads its skill again.
    fixture.program(
        "outer-agent",
        &format!(
            r#"outer=$RAVEL_SKILL_DIRECTORIES
channel=$RAVEL_EXIT_FILE
case $1 in *"Descriptor: \"$outer\""*) ;; *) exit 90 ;; esac
before=$(cat "$(paths "$outer" directories)/alpha/SKILL.md")
printf v2 > {skill:?}
inner=$("$FRONT" start-session --kind inner --prompt nested) || exit 91
[ "$outer" = "$RAVEL_SKILL_DIRECTORIES" ] || exit 92
[ "$channel" = "$RAVEL_EXIT_FILE" ] || exit 93
after=$(cat "$(paths "$outer" directories)/alpha/SKILL.md")
exec "$FRONT" end-session "$inner|outer $before $after|$outer|$channel""#
        ),
    );
    fixture.program(
        "inner-agent",
        &format!(
            r#"case $1 in *"Descriptor: \"$RAVEL_SKILL_DIRECTORIES\""*) ;; *) exit 90 ;; esac
printf '%s' "$RAVEL_SKILL_DIRECTORIES" > {descriptor:?}
printf '%s' "$RAVEL_EXIT_FILE" > {channel:?}
"$FRONT" end-session "inner $(cat "$(paths "$RAVEL_SKILL_DIRECTORIES" directories)/alpha/SKILL.md")"
exec "$FRONT" end-session ignored"#,
            descriptor = records.join("inner-descriptor"),
            channel = records.join("inner-channel"),
        ),
    );
    fixture.personal_rhai(&routes(&[
        ("outer", "outer-agent"),
        ("inner", "inner-agent"),
    ]));

    let outer = fixture.start_with("outer", "outer task", Some("/stale/directories.json"));
    assert_eq!(outer.code, Some(0), "{}", outer.stderr);
    // The text notice of the nested session comes first, because the outer
    // session writes its JSON notice when its job has ended.
    let notice: Value = serde_json::from_str(
        outer
            .stderr
            .lines()
            .find(|line| line.starts_with('{'))
            .expect("the JSON notice of the outer session"),
    )
    .unwrap();
    let outer_descriptor = notice["skills"]["descriptorPath"].as_str().unwrap();
    let parts: Vec<&str> = outer.stdout.split('|').collect();
    assert_eq!(parts.len(), 4, "{}", outer.stdout);
    // The nested session saw the current source, and the outer session got
    // only the nested result.
    assert_eq!(parts[0], "inner v2");
    // The parent kept its descriptor and its resources.
    assert_eq!(parts[1], "outer v1 v1");
    assert_eq!(parts[2], outer_descriptor);
    let inner_descriptor = fs::read_to_string(records.join("inner-descriptor")).unwrap();
    let inner_channel = fs::read_to_string(records.join("inner-channel")).unwrap();
    // Only the nested managed process received the new descriptor and the new
    // channel.
    assert_ne!(inner_descriptor, outer_descriptor);
    assert_ne!(inner_channel, parts[3]);
    assert!(!parts[3].is_empty());
    assert_eq!(fixture.objects("descriptors").len(), 2);
    let inner: Value = serde_json::from_slice(&fs::read(&inner_descriptor).unwrap()).unwrap();
    let inner_root = inner["sources"][0]["snapshotRoot"].as_str().unwrap();
    assert_eq!(
        fs::read_to_string(Path::new(inner_root).join("skills/alpha/SKILL.md")).unwrap(),
        "v2"
    );
}

/// An agent that has lost the descriptor path from its prompt: it never reads
/// its argument. It recovers the path from `RAVEL_SKILL_DIRECTORIES` when the
/// name is set. Otherwise it reports missing context. Each report counts the
/// environment values that name the cache or a descriptor, so a recovered
/// report also shows that the count finds a path.
const FORGETFUL: &str = r#"guessed=$(env | grep -c -e '\.cache/ravel' -e 'directories\.json' || true)
if [ -n "${RAVEL_SKILL_DIRECTORIES-}" ]; then
  descriptor=$RAVEL_SKILL_DIRECTORIES
  exec "$FRONT" end-session "recovered $descriptor $(cat "$(paths "$descriptor" mandatory)") guessed $guessed"
fi
exec "$FRONT" end-session "missing discovery context, guessed $guessed""#;

#[test]
fn the_variable_recovers_a_lost_prompt_path_and_without_both_channels_nothing_supplies_one() {
    let fixture = Fixture::new();
    let source = fixture.source(
        "pack",
        &[
            ("MANDATORY.md", "read me first"),
            ("skills/a/SKILL.md", "a"),
        ],
    );
    let enabled = || {
        fixture.skills(
            vec![local("pack", &source, &["skills"])],
            vec![json!({"source": "pack", "path": "MANDATORY.md"})],
        );
    };
    enabled();
    fixture.program("forgetful", FORGETFUL);
    fixture.personal_rhai(&rhai_script(
        r#"fn select(request, context) {
    let args = if request.kind == "scrubbed" {
        ["-u", "RAVEL_SKILL_DIRECTORIES", "forgetful", request.prompt]
    } else {
        ["forgetful", request.prompt]
    };
    #{ status: "selected", program: "env", args: args,
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture" }
}"#,
    ));
    let stale = Some("/stale/.cache/ravel/skills/v1/descriptors/stale/directories.json");

    // The prompt path is lost, and the variable gives the same descriptor.
    // The stale inherited value is gone.
    let recovered = fixture.start_with("check", "p", stale);
    let recovered_notice = notice(&recovered);
    let descriptor = recovered_notice["skills"]["descriptorPath"]
        .as_str()
        .unwrap();
    assert_eq!(
        recovered.stdout,
        format!("recovered {descriptor} read me first guessed 1")
    );

    // A child that has lost both path channels: the prompt, and the variable
    // that its parent removed. Nothing else names a path.
    let scrubbed = fixture.start_with("scrubbed", "p", stale);
    notice(&scrubbed);
    assert_eq!(scrubbed.stdout, "missing discovery context, guessed 0");

    // Disabled delivery removes the stale inherited name.
    fixture.settings(&json!({}));
    let disabled = fixture.start_with("check", "p", stale);
    assert_eq!(notice(&disabled)["skills"], Value::Null);
    assert_eq!(disabled.stdout, "missing discovery context, guessed 0");

    // The cache holds no pointer to a latest descriptor: only the three
    // directories, and descriptors named by their digests.
    assert_eq!(fixture.objects(""), ["descriptors", "snapshots", "staging"]);
    for name in fixture.objects("descriptors") {
        assert!(
            name.len() == 64 && name.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "{name}"
        );
        let path = fixture.cache().join("descriptors").join(&name);
        assert!(fs::symlink_metadata(&path).unwrap().is_dir(), "{name}");
    }
}
