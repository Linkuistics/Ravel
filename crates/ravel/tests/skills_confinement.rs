//! The read grants of skill delivery under `--confine`, on the real backend
//! of the platform: macOS `sandbox-exec` or Linux bubblewrap. The seam is the
//! public `ravel start-session`, with local sources outside the working
//! directory.
//! docs/specs/skill-discovery.md, "Read access and delegation" and "Test
//! seams and acceptance".
//!
//! These rows run here: "Real confinement", the confined part of "Harness
//! runs" and the confined part of "Nested sessions".
//!
//! The fixture programs read the descriptor with `sed` and `grep`. The
//! descriptor is compact JSON, and the temporary paths hold no quote or
//! bracket.

mod support;

use std::fs;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{FRONT, Run, Sandbox, run, text, write};

const SETTINGS: &str = ".config/ravel/settings.json";
const CACHE: &str = ".cache/ravel/skills/v1";

/// `paths DESCRIPTOR KEY` prints the `path` of each record of the array `KEY`
/// of a descriptor, one on each line.
const PATHS: &str = r#"paths() {
  sed -n "s/.*\"$2\":\[\([^]]*\)\].*/\1/p" "$1" | grep -o '"path":"[^"]*"' | sed 's/^"path":"//; s/"$//'
}"#;

/// `probe COMMAND NAME` runs `COMMAND` in a new shell and records in
/// `./observed` whether the confinement allowed it.
const PROBE: &str = r#"probe() {
  if sh -c "$1" >/dev/null 2>&1; then echo "$2=allowed" >> ./observed; else echo "$2=denied" >> ./observed; fi
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
            .arg("u+rwx")
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

    /// The source `pack` outside the working directory: a mandatory file, a
    /// skill and a relative link from the skill to a sibling directory.
    fn pack(&self) -> PathBuf {
        let source = self.root.join("sources/pack");
        write(&source.join("guide/MANDATORY.md"), "mandatory body");
        write(&source.join("skills/alpha/SKILL.md"), "alpha body");
        write(&source.join("shared/notes.md"), "shared notes");
        symlink(
            "../../shared/notes.md",
            source.join("skills/alpha/notes.md"),
        )
        .unwrap();
        source
    }

    fn settings(&self, value: &Value) {
        write(&self.home.join(SETTINGS), &value.to_string());
    }

    /// Delivery of `pack`, with `extra` members at the top of the settings.
    fn deliver(&self, source: &Path, extra: Value) {
        let mut settings = json!({ "skills": {
            "schemaVersion": 1,
            "sources": [{ "id": "pack", "type": "local", "path": source, "roots": ["skills"] }],
            "mandatory": [{ "source": "pack", "path": "guide/MANDATORY.md" }],
        }});
        if let Value::Object(extra) = extra {
            settings.as_object_mut().unwrap().extend(extra);
        }
        self.settings(&settings);
    }

    /// A program on the private `PATH`, with `paths` and `probe`.
    fn program(&self, name: &str, body: &str) -> PathBuf {
        let path = self.bin.join(name);
        write(
            &path,
            &format!("#!/bin/sh\nFRONT={FRONT:?}\n{PATHS}\n{PROBE}\n{body}\n"),
        );
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn resolve_json(&self) -> Value {
        let mut command = self.command();
        command.args(["skills", "resolve", "--json"]);
        let resolved = run(&mut command);
        assert_eq!(resolved.code, Some(0), "{}", resolved.stderr);
        serde_json::from_str(&resolved.stdout).unwrap()
    }

    /// A confined launch of `kind` that grants `programs` as runtime reads.
    fn confined(&self, kind: &str, programs: &[&Path], extra: &[&str]) -> Command {
        let mut command = self.command();
        command.args([
            "start-session",
            "--kind",
            kind,
            "--json",
            "--prompt",
            "confined task",
            "--confine",
        ]);
        for program in programs {
            command.arg("--runtime-read").arg(program);
        }
        command.args(extra);
        command
    }

    fn observed(&self) -> Vec<String> {
        fs::read_to_string(self.cwd.join("observed"))
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect()
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

/// The routing notice of a successful launch.
fn notice(launch: &Run) -> Value {
    assert_eq!(launch.code, Some(0), "{}", launch.stderr);
    serde_json::from_str(
        launch
            .stderr
            .lines()
            .find(|line| line.starts_with('{'))
            .expect("routing notice"),
    )
    .unwrap()
}

/// The reads and the denied operations of a confined agent. It follows the
/// descriptor from its prompt and from `RAVEL_SKILL_DIRECTORIES`. A child
/// and a grandchild read through the relative link of the snapshot.
///
/// The listing probes look for a real entry. Bubblewrap creates the parent
/// directories of each mount in an empty root, so a listing of the home
/// succeeds there and shows only the path to the cache.
fn reader_body(source: &Path, home: &Path) -> String {
    format!(
        r#"d=$RAVEL_SKILL_DIRECTORIES
case $1 in *"Descriptor: \"$d\""*) ;; *) exit 90 ;; esac
: > ./observed
mandatory=$(paths "$d" mandatory)
dir=$(paths "$d" directories)
echo "mandatory=$(cat "$mandatory")" >> ./observed
echo "skill=$(sh -c 'cat "$1/alpha/SKILL.md"' child "$dir")" >> ./observed
echo "link=$(sh -c 'sh -c "cat \"\$1/alpha/notes.md\"" grandchild "$1"' child "$dir")" >> ./observed
echo "listing=$(ls "$dir/alpha" | tr '\n' ,)" >> ./observed
probe "printf x > '$dir/alpha/SKILL.md'" write-skill
probe "chmod u+w '$dir/alpha/SKILL.md'" chmod-skill
probe "printf x > '$dir/alpha/notes.md'" write-through-link
probe ": > '$dir/new'" create-in-snapshot
probe "rm -f '$dir/alpha/SKILL.md' && test ! -e '$dir/alpha/SKILL.md'" remove-skill
probe "printf x > '$d'" write-descriptor
probe ": > '$(dirname "$d")/new'" create-beside-descriptor
probe "cat '{home}/.config/ravel/settings.json'" read-settings
probe "cat '{home}/.config/ravel/config.rhai'" read-config
probe "cat '{source}/skills/alpha/SKILL.md'" read-source
probe "ls '{home}/{CACHE}/staging'" list-staging
probe "ls -a '{home}/{CACHE}' | grep -qx staging" list-cache
probe "ls -a '{home}' | grep -qx .config" list-home
probe "cat \"$(dirname "$(dirname "$mandatory")")/../manifest\"" read-manifest
"#,
        source = source.display(),
        home = home.display(),
    )
}

/// What [`reader_body`] must record when the confinement grants exactly the
/// descriptor directory and the snapshot trees, read-only.
const EXPECTED: &[&str] = &[
    "mandatory=mandatory body",
    "skill=alpha body",
    "link=shared notes",
    "listing=SKILL.md,notes.md,",
    "write-skill=denied",
    "chmod-skill=denied",
    "write-through-link=denied",
    "create-in-snapshot=denied",
    "remove-skill=denied",
    "write-descriptor=denied",
    "create-beside-descriptor=denied",
    "read-settings=denied",
    "read-config=denied",
    "read-source=denied",
    "list-staging=denied",
    "list-cache=denied",
    "list-home=denied",
    "read-manifest=denied",
];

#[test]
fn a_confined_child_and_its_descendants_read_the_resources_and_cannot_write_them() {
    let fixture = Fixture::new();
    let source = fixture.pack();
    fixture.deliver(&source, json!({}));
    let agent = fixture.program(
        "reading-agent",
        &format!(
            "{}exec \"$FRONT\" end-session done",
            reader_body(&source, &fixture.home)
        ),
    );
    fixture.personal_rhai(&routes(&[("read", "reading-agent")]));

    let launch = run(&mut fixture.confined("read", &[&agent], &[]));
    let notice = notice(&launch);
    assert_eq!(launch.stdout, "done");
    assert_eq!(fixture.observed(), EXPECTED);
    // The snapshot and the descriptor are unchanged.
    let descriptor = notice["skills"]["descriptorPath"].as_str().unwrap();
    let delivered: Value = serde_json::from_slice(&fs::read(descriptor).unwrap()).unwrap();
    let tree = Path::new(delivered["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(
        fs::read_to_string(tree.join("skills/alpha/SKILL.md")).unwrap(),
        "alpha body"
    );
    assert!(!tree.join("skills/new").exists());
    assert_eq!(
        fs::read_dir(Path::new(descriptor).parent().unwrap())
            .unwrap()
            .count(),
        1
    );
    assert!(fixture.left_in_tmp().is_empty());
}

#[test]
fn a_confined_harness_run_reads_the_resources_and_cannot_write_them() {
    let fixture = Fixture::new();
    let source = fixture.pack();
    fixture.deliver(
        &source,
        json!({
            "harnesses": { "reading-harness": { "program": "reading-harness" } },
            "budgets": { "harnessRuns": 1 },
        }),
    );
    // The harness has no prompt. It finds the descriptor through the variable.
    let harness = fixture.program(
        "reading-harness",
        &format!(
            "set -- \"Descriptor: \\\"$RAVEL_SKILL_DIRECTORIES\\\"\"\n{}\
             exec \"$FRONT\" end-session harness-read",
            reader_body(&source, &fixture.home)
        ),
    );
    fixture.personal_rhai(&rhai_script(
        r#"fn select(request, context) {
    let run = ravel::run_harness(#{ harness: "reading-harness", args: [] });
    if run.status != "exited" || run.result != "harness-read" {
        throw "the confined run did not finish: " + ravel::json_encode(run);
    }
    #{ status: "selected", program: "fake-agent", args: [], provider: "p", model: "m",
       effort: "e", reason: "r" }
}"#,
    ));
    let fake = fixture.bin.join("fake-agent");
    let launch = run(&mut fixture.confined("read", &[&harness, &fake], &[]));
    let notice = notice(&launch);
    assert_eq!(launch.stdout, "done");
    assert_eq!(notice["harnessRuns"][0]["status"], "exited", "{notice}");
    assert_eq!(fixture.observed(), EXPECTED);
    assert!(fixture.left_in_tmp().is_empty());
}

/// The last line of a nested launch's standard error, as JSON.
fn last_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap();
    serde_json::from_str(text.lines().last().expect("a JSON line")).unwrap()
}

#[test]
fn a_nested_start_session_in_a_confined_parent_cannot_read_the_settings_and_the_parent_keeps_its_paths()
 {
    let fixture = Fixture::new();
    let source = fixture.pack();
    fixture.deliver(&source, json!({}));
    // The confined parent cannot read its settings or its configuration
    // script, and it cannot write the cache. It starts one nested session
    // with the default entry and one with an entry in its working directory.
    // Then it checks its variables and reads its resources again.
    let agent = fixture.program(
        "outer-agent",
        r#"outer=$RAVEL_SKILL_DIRECTORIES
channel=$RAVEL_EXIT_FILE
"$FRONT" start-session --json --kind inner --prompt nested > ./default.out 2> ./default.err
default=$?
"$FRONT" start-session --json --config ./nested.rhai --kind inner --prompt nested \
  > ./named.out 2> ./named.err
named=$?
[ "$outer" = "$RAVEL_SKILL_DIRECTORIES" ] || exit 92
[ "$channel" = "$RAVEL_EXIT_FILE" ] || exit 93
exec "$FRONT" end-session "$default $named|$(cat "$(paths "$outer" mandatory)")|$outer""#,
    );
    let route = routes(&[("outer", "outer-agent"), ("inner", "fake-agent")]);
    fixture.personal_rhai(&route);
    write(&fixture.cwd.join("nested.rhai"), &route);

    // The nested route's program must be readable in the parent's sandbox.
    let fake = fixture.bin.join("fake-agent");
    let launch = run(&mut fixture.confined("outer", &[&agent, &fake], &[]));
    let notice = notice(&launch);
    let descriptor = notice["skills"]["descriptorPath"].as_str().unwrap();
    let parts: Vec<&str> = launch.stdout.split('|').collect();
    assert_eq!(parts.len(), 3, "{}", launch.stdout);
    // The parent kept its descriptor, its channel and its resources.
    assert_eq!(parts[1], "mandatory body");
    assert_eq!(parts[2], descriptor);
    // Neither nested launch published anything or opened the settings.
    assert_eq!(
        fs::read_dir(fixture.home.join(CACHE).join("descriptors"))
            .unwrap()
            .count(),
        1
    );
    let default = last_json(&fixture.cwd.join("default.err"));
    let named = fixture.cwd.join("named.err");
    assert_eq!(
        fs::read_to_string(fixture.cwd.join("default.out")).unwrap(),
        ""
    );
    // Seatbelt shows the metadata of the settings file and refuses its read,
    // so each nested launch refuses its settings. Bubblewrap has no settings
    // file in its namespace, so the nested launch sees no settings: delivery
    // is disabled, the default entry is missing and a readable entry runs.
    if cfg!(target_os = "macos") {
        assert_eq!(parts[0], "2 2", "{default}");
        assert_eq!(default["error"]["code"], "settings_invalid", "{default}");
        assert_eq!(last_json(&named)["error"]["code"], "settings_invalid");
    } else {
        assert_eq!(parts[0], "3 0", "{default}");
        assert_eq!(default["error"]["code"], "config_missing", "{default}");
        assert_eq!(
            fs::read_to_string(fixture.cwd.join("named.out")).unwrap(),
            "done"
        );
        let nested: Value =
            serde_json::from_str(fs::read_to_string(&named).unwrap().lines().next().unwrap())
                .unwrap();
        assert_eq!(nested["skills"], Value::Null, "{nested}");
    }
}

/// The refusal of a confined launch: `skill_access_unavailable`, exit 3, no
/// worker, no managed process and no run directory.
fn refused(fixture: &Fixture, launch: &Run, marker: &Path, mention: &str) -> Value {
    let refusal = launch.refusal(3);
    let error = &refusal["error"];
    assert_eq!(error["code"], "skill_access_unavailable", "{error}");
    assert_eq!(error["stage"], "skills", "{error}");
    assert!(
        error["message"].as_str().unwrap().contains(mention),
        "{mention}: {error}"
    );
    assert!(
        error["remedy"].as_str().unwrap().contains("unconfined"),
        "{error}"
    );
    assert!(!marker.exists(), "the managed process ran");
    assert!(
        fixture.left_in_tmp().is_empty(),
        "{:?}",
        fixture.left_in_tmp()
    );
    refusal
}

/// The agent of the refusal fixtures leaves a marker, so that a refusal
/// shows that no managed process ran.
fn marking_agent(fixture: &Fixture) -> (PathBuf, PathBuf) {
    let marker = fixture.cwd.join("marker");
    let agent = fixture.program(
        "marking-agent",
        &format!(": > {marker:?} 2>/dev/null\nexec \"$FRONT\" end-session done"),
    );
    fixture.personal_rhai(&routes(&[("mark", "marking-agent")]));
    (agent, marker)
}

#[test]
fn a_grant_that_overlaps_the_configuration_or_a_result_channel_refuses_also_through_an_alias() {
    let fixture = Fixture::new();
    let source = fixture.pack();
    // The source holds a copy of a configuration script and of the settings.
    // The snapshot tree then holds them too.
    let route = routes(&[("mark", "marking-agent")]);
    write(&source.join("conf/config.rhai"), &route);
    fixture.deliver(&source, json!({}));
    let settings = fs::read_to_string(fixture.home.join(SETTINGS)).unwrap();
    write(&source.join("conf/settings.json"), &settings);
    let (agent, marker) = marking_agent(&fixture);
    let resolved = fixture.resolve_json();
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    // An alias outside the cache that reaches the snapshot tree.
    let alias = fixture.root.join("alias");
    symlink(&tree, &alias).unwrap();

    // The configuration entry, through the alias.
    let entry = format!("{}/conf/config.rhai", alias.display());
    let launch = run(&mut fixture.confined("mark", &[&agent], &["--config", &entry]));
    refused(&fixture, &launch, &marker, &text(&tree));

    // The settings file, as a link to its copy in the snapshot.
    fs::remove_file(fixture.home.join(SETTINGS)).unwrap();
    symlink(
        alias.join("conf/settings.json"),
        fixture.home.join(SETTINGS),
    )
    .unwrap();
    let launch = run(&mut fixture.confined("mark", &[&agent], &[]));
    refused(&fixture, &launch, &marker, "settings.json");
    fs::remove_file(fixture.home.join(SETTINGS)).unwrap();
    write(&fixture.home.join(SETTINGS), &settings);

    // The result channel of an enclosing session, through the alias.
    let mut command = fixture.confined("mark", &[&agent], &[]);
    command.env("RAVEL_EXIT_FILE", alias.join("channel/result"));
    let launch = run(&mut command);
    refused(&fixture, &launch, &marker, &text(&tree));

    // The same settings and entry launch confined, with the reads granted.
    let launch = run(&mut fixture.confined("mark", &[&agent], &[]));
    assert_eq!(notice(&launch)["skills"]["sources"][0]["id"], "pack");
    assert!(marker.exists());
}

#[test]
fn a_writable_grant_that_overlaps_the_cache_refuses() {
    let fixture = Fixture::new();
    let source = fixture.pack();
    fixture.deliver(&source, json!({}));
    let (agent, marker) = marking_agent(&fixture);
    let resolved = fixture.resolve_json();
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    let cache = fixture.home.join(CACHE);

    // A working directory inside the cache, outside every snapshot.
    let inside = cache.join("staging");
    let mut command = fixture.confined("mark", &[&agent], &[]);
    command.current_dir(&inside);
    let launch = run(&mut command);
    refused(&fixture, &launch, &marker, &text(&inside));

    // A working directory that holds the cache, through an alias.
    let alias = fixture.root.join("home-alias");
    symlink(&fixture.home, &alias).unwrap();
    let mut command = fixture.confined("mark", &[&agent], &[]);
    command.current_dir(alias.join(".cache"));
    let launch = run(&mut command);
    refused(&fixture, &launch, &marker, "overlaps the skill cache");

    // An `--exit-dir` inside a snapshot tree.
    let exit_dir = text(&tree.join("skills"));
    let launch = run(&mut fixture.confined("mark", &[&agent], &["--exit-dir", &exit_dir]));
    refused(&fixture, &launch, &marker, &exit_dir);

    // A private run directory inside the cache, from `TMPDIR`.
    let mut command = fixture.confined("mark", &[&agent], &[]);
    command.env("TMPDIR", cache.join("descriptors"));
    let launch = run(&mut command);
    refused(&fixture, &launch, &marker, "overlaps the skill cache");
    let left: Vec<_> = fs::read_dir(cache.join("descriptors"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("ravel-"))
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[test]
fn a_confined_launch_with_delivery_disabled_does_not_change() {
    let fixture = Fixture::new();
    fixture.settings(&json!({ "skills": { "schemaVersion": 1, "sources": [], "mandatory": [] } }));
    let (agent, marker) = marking_agent(&fixture);
    let launch = run(&mut fixture.confined("mark", &[&agent], &[]));
    assert_eq!(notice(&launch)["skills"], Value::Null);
    assert!(marker.exists());
    assert!(!fixture.home.join(".cache").exists());
}
