//! The default skill directory at the public `ravel skills resolve` and
//! `ravel start-session` seams. A copied executable sits in a Homebrew-shaped
//! keg with an installation record. The record names a fixture directory of
//! relative links into other kegs and of real directories.
//! docs/specs/skill-discovery.md, "Default skill directory" and the rows
//! "Default skill directory" and "Disabled delivery" of "Test seams and
//! acceptance".
//!
//! The development build in `target` has no record, so the other suites see
//! no default skill directory.

mod support;

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use support::rhai::rhai_script;
use support::{Run, Sandbox, mkfifo, run, text, write};

const RECORD: &str = "share/ravel/installation.json";

/// A configuration script that cannot parse: a refusal before it proves that
/// no worker evaluated the entry.
const UNPARSED: &str = "export const SCHEMA_VERSION = 4;\nfn select(request, context) {\n";

/// A sandbox with a Homebrew-shaped installation of the development build.
///
/// - `cellar/ravel/2.0.0/bin/ravel` is a copy, and its keg holds the record.
/// - `prefix/bin/ravel` is a relative link to that copy, as `brew link`
///   makes it.
/// - The record names `prefix/share/ravel/skills`, the default skill
///   directory. The Cellar lies outside the prefix, as it does when Homebrew
///   puts it inside its repository.
struct Fixture {
    sandbox: Sandbox,
    keg: PathBuf,
    linked: PathBuf,
    directory: PathBuf,
}

impl std::ops::Deref for Fixture {
    type Target = Sandbox;
    fn deref(&self) -> &Sandbox {
        &self.sandbox
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Snapshots are read-only.
        let _ = Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.sandbox.root)
            .status();
    }
}

impl Fixture {
    fn new() -> Self {
        let sandbox = Sandbox::new();
        let keg = sandbox.root.join("cellar/ravel/2.0.0");
        fs::create_dir_all(keg.join("bin")).unwrap();
        fs::copy(support::FRONT, keg.join("bin/ravel")).unwrap();
        let prefix = sandbox.root.join("prefix");
        fs::create_dir_all(prefix.join("bin")).unwrap();
        let linked = prefix.join("bin/ravel");
        symlink("../../cellar/ravel/2.0.0/bin/ravel", &linked).unwrap();
        let directory = prefix.join("share/ravel/skills");
        let fixture = Fixture {
            sandbox,
            keg,
            linked,
            directory,
        };
        fixture
            .record(&json!({"schemaVersion": 1, "skillDirectory": fixture.directory}).to_string());
        fixture
    }

    /// Write the installation record of the keg.
    fn record(&self, text: &str) {
        write(&self.keg.join(RECORD), text);
    }

    /// A source in its own keg, `cellar/<formula>/1.0/share/ravel/skills/<id>`,
    /// with one skill in each of `roots`. The default skill directory gets a
    /// relative link to it, as `brew link` makes it.
    fn linked_source(&self, formula: &str, id: &str, roots: &[&str]) -> PathBuf {
        let keg = self
            .root
            .join("cellar")
            .join(formula)
            .join("1.0/share/ravel/skills")
            .join(id);
        for root in roots {
            write(&keg.join(root).join(format!("{id}-{root}/SKILL.md")), id);
        }
        write(&keg.join("tests/fixture/SKILL.md"), "not shown");
        fs::create_dir_all(&self.directory).unwrap();
        symlink(
            format!("../../../../cellar/{formula}/1.0/share/ravel/skills/{id}"),
            self.directory.join(id),
        )
        .unwrap();
        keg
    }

    /// A real directory in the default skill directory, with one skill in
    /// each of `roots`.
    fn real_source(&self, id: &str, roots: &[&str]) -> PathBuf {
        let source = self.directory.join(id);
        for root in roots {
            write(&source.join(root).join(format!("{id}-{root}/SKILL.md")), id);
        }
        source
    }

    fn settings(&self, value: &Value) {
        write(
            &self.home.join(".config/ravel/settings.json"),
            &value.to_string(),
        );
    }

    /// `skills resolve --json` through the linked executable.
    fn resolve(&self) -> Run {
        let mut command = self.command_for(&self.linked);
        command.args(["skills", "resolve", "--json"]);
        run(&mut command)
    }

    fn resolved(&self) -> Value {
        let output = self.resolve();
        assert_eq!(output.code, Some(0), "{}", output.stderr);
        assert!(output.stderr.is_empty(), "{}", output.stderr);
        serde_json::from_str(&output.stdout).unwrap()
    }

    /// The error of a refused `skills resolve --json`.
    fn refused(&self, exit: i32) -> Value {
        let output = self.resolve();
        assert_eq!(output.code, Some(exit), "{}", output.stderr);
        assert!(output.stdout.is_empty(), "{}", output.stdout);
        serde_json::from_str::<Value>(&output.stderr).unwrap()["error"].clone()
    }

    /// `start-session --json` through the linked executable.
    fn start(&self) -> Run {
        let mut command = self.command_for(&self.linked);
        command.args([
            "start-session",
            "--kind",
            "check",
            "--json",
            "--prompt",
            "task",
        ]);
        run(&mut command)
    }

    /// A configuration script that launches `fake-agent` with the prompt.
    fn route(&self) {
        self.personal_rhai(&rhai_script(
            r#"fn select(request, context) {
    #{ status: "selected", program: "fake-agent", args: [request.prompt],
       provider: "fixture", model: "fixture", effort: "high", reason: "fixture" }
}"#,
        ));
    }

    fn no_cache(&self) -> bool {
        !self.home.join(".cache").exists()
    }
}

fn ids(resolved: &Value) -> Vec<String> {
    resolved["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn with_no_settings_each_default_source_is_delivered_with_its_roots_in_byte_order() {
    let fixture = Fixture::new();
    let core = fixture.linked_source("grove", "grove-core", &["craft", "skills"]);
    let zeta = fixture.linked_source("zeta", "zeta-pack", &["skills"]);
    let beta = fixture.real_source("beta", &["craft", "docs"]);
    // Ignored dot entries, of each kind.
    write(&fixture.directory.join(".DS_Store"), "x");
    fs::create_dir(fixture.directory.join(".hidden")).unwrap();
    symlink("nowhere", fixture.directory.join(".dangling")).unwrap();

    let resolved = fixture.resolved();
    assert_eq!(ids(&resolved), ["beta", "grove-core", "zeta-pack"]);
    // Capture canonicalises a link, so the origin is the keg.
    let origins: Vec<&str> = resolved["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source["originPath"].as_str().unwrap())
        .collect();
    assert_eq!(
        origins,
        [
            text(&beta),
            text(&core.canonicalize().unwrap()),
            text(&zeta.canonicalize().unwrap())
        ]
    );
    let roots: Vec<String> = resolved["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source["snapshotRoot"].as_str().unwrap().to_owned())
        .collect();
    let directories: Vec<&str> = resolved["directories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|directory| directory["path"].as_str().unwrap())
        .collect();
    // `skills`, then `craft`, each where it exists, and no other root.
    assert_eq!(
        directories,
        [
            format!("{}/craft", roots[0]),
            format!("{}/skills", roots[1]),
            format!("{}/craft", roots[1]),
            format!("{}/skills", roots[2]),
        ]
    );
    assert_eq!(resolved["mandatory"], json!([]));
    let instructions = resolved["instructions"].as_str().unwrap();
    assert!(!instructions.contains("/tests"), "{instructions}");
    assert!(!instructions.contains("/docs"), "{instructions}");

    // The copy in the keg reads the same record, and a launch delivers the
    // same directories. The routing notice names the directory.
    let mut direct = fixture.command_for(&fixture.keg.join("bin/ravel"));
    let direct = run(direct.args(["skills", "resolve", "--json"]));
    assert_eq!(direct.code, Some(0), "{}", direct.stderr);
    assert_eq!(
        serde_json::from_str::<Value>(&direct.stdout).unwrap(),
        resolved
    );
    fixture.route();
    let notice = fixture.start().report();
    let skills = &notice["skills"];
    assert_eq!(skills["defaultDirectory"], text(&fixture.directory));
    assert_eq!(skills["descriptorPath"], resolved["descriptorPath"]);
    let notified: Vec<&str> = skills["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source["id"].as_str().unwrap())
        .collect();
    assert_eq!(notified, ["beta", "grove-core", "zeta-pack"]);

    // A mandatory reference names a default source by its ID.
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "mandatory": [
        {"source": "grove-core", "path": "skills/grove-core-skills/SKILL.md"}
    ]}}));
    let resolved = fixture.resolved();
    assert_eq!(
        resolved["mandatory"],
        json!([{"source": "grove-core",
                "path": format!("{}/skills/grove-core-skills/SKILL.md", roots[1])}])
    );
}

#[test]
fn configured_sources_replace_the_default_and_their_notice_names_no_directory() {
    let fixture = Fixture::new();
    fixture.linked_source("grove", "grove-core", &["skills"]);
    let own = fixture.root.join("own");
    write(&own.join("s/x/SKILL.md"), "x");
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "sources": [
        {"id": "own", "type": "local", "path": own, "roots": ["s"]}
    ]}}));
    assert_eq!(ids(&fixture.resolved()), ["own"]);
    fixture.route();
    assert_eq!(
        fixture.start().report()["skills"]["defaultDirectory"],
        Value::Null
    );
    // A mandatory reference to a default source is then unknown to the
    // settings.
    fixture.settings(&json!({"skills": {"schemaVersion": 1,
        "sources": [{"id": "own", "type": "local", "path": own, "roots": ["s"]}],
        "mandatory": [{"source": "grove-core", "path": "skills/x.md"}]}}));
    let error = fixture.refused(2);
    assert_eq!(error["code"], "settings_invalid", "{error}");
    assert_eq!(error["location"], "skills.mandatory[0].source", "{error}");
}

#[test]
fn delivery_is_disabled_without_a_record_a_directory_or_a_source_and_by_empty_sources() {
    let fixture = Fixture::new();
    fixture.route();
    let disabled = |case: &str| {
        let output = fixture.resolve();
        assert_eq!(output.code, Some(0), "{case}: {}", output.stderr);
        assert_eq!(
            serde_json::from_str::<Value>(&output.stdout).unwrap(),
            json!({"schemaVersion": 1, "status": "disabled"}),
            "{case}"
        );
        let notice = fixture.start().report();
        assert_eq!(notice["skills"], Value::Null, "{case}");
        assert!(fixture.no_cache(), "{case}: cache work");
    };
    // The record names a directory that does not exist yet.
    disabled("no default skill directory");
    fs::create_dir_all(&fixture.directory).unwrap();
    write(&fixture.directory.join(".DS_Store"), "x");
    disabled("an empty default skill directory");
    for settings in [json!({}), json!({"skills": {"schemaVersion": 1}})] {
        fixture.settings(&settings);
        disabled(&format!("settings {settings} and an empty directory"));
    }
    fs::remove_file(fixture.keg.join(RECORD)).unwrap();
    fixture.real_source("pack", &["skills"]);
    disabled("no record");
    fixture.record(&json!({"schemaVersion": 1, "skillDirectory": fixture.directory}).to_string());
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "sources": []}}));
    disabled("empty sources beside a populated directory");
    // The control: the same installation delivers without that setting.
    fs::remove_file(fixture.home.join(".config/ravel/settings.json")).unwrap();
    assert_eq!(ids(&fixture.resolved()), ["pack"]);
}

/// Make one entry of each refusing row of the entry table in `directory`.
type Make = fn(&Fixture, &Path);

fn refusing_rows() -> Vec<(&'static str, &'static str, Make)> {
    vec![
        ("an empty directory", "empty", |_, path| {
            fs::create_dir(path).unwrap();
        }),
        ("a directory with neither root", "docs-only", |_, path| {
            write(&path.join("docs/a/SKILL.md"), "a");
        }),
        ("a root that is a file", "file-root", |_, path| {
            write(&path.join("skills"), "not a directory");
        }),
        (
            "a link to a directory with neither root",
            "linked-empty",
            |fixture, path| {
                fs::create_dir_all(fixture.root.join("cellar/empty")).unwrap();
                symlink("../../../../cellar/empty", path).unwrap();
            },
        ),
        (
            "a name outside the source-ID grammar",
            "Bad_Name",
            |_, path| {
                write(&path.join("skills/a/SKILL.md"), "a");
            },
        ),
        ("a name with a leading digit", "1pack", |_, path| {
            write(&path.join("skills/a/SKILL.md"), "a");
        }),
        ("a regular file", "notes", |_, path| write(path, "x")),
        ("a FIFO", "pipe", |_, path| mkfifo(path)),
        (
            "a link to a regular file",
            "linked-file",
            |fixture, path| {
                write(&fixture.root.join("cellar/file"), "x");
                symlink("../../../../cellar/file", path).unwrap();
            },
        ),
        ("a dangling link", "gone", |_, path| {
            symlink("../../../../cellar/nowhere", path).unwrap();
        }),
        ("a cyclic link", "cycle", |_, path| {
            symlink("cycle", path).unwrap();
        }),
    ]
}

#[test]
fn each_refusing_entry_refuses_as_skill_directory_invalid_and_no_worker_launches() {
    for (case, name, make) in refusing_rows() {
        let fixture = Fixture::new();
        fixture.linked_source("grove", "grove-core", &["skills"]);
        fixture.personal_rhai(UNPARSED);
        let entry = fixture.directory.join(name);
        make(&fixture, &entry);

        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_directory_invalid", "{case}: {error}");
        assert_eq!(error["stage"], "skills", "{case}");
        assert_eq!(error["source"], text(&entry), "{case}: {error}");
        assert_eq!(error["location"], text(&entry), "{case}: {error}");
        assert!(
            error["message"].as_str().unwrap().contains(&text(&entry)),
            "{case}: {error}"
        );
        assert!(
            error["remedy"]
                .as_str()
                .unwrap()
                .contains("reinstall the package"),
            "{case}: {error}"
        );
        // A launch refuses at the same stage, before the worker parses the
        // entry and before any cache work.
        let error = fixture.start().refusal(3)["error"].clone();
        assert_eq!(error["code"], "skill_directory_invalid", "{case}: {error}");
        assert_eq!(error["source"], text(&entry), "{case}: {error}");
        assert!(fixture.no_cache(), "{case}: cache work");

        // The control: without that entry, the same directory resolves.
        match fs::symlink_metadata(&entry) {
            Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(&entry).unwrap(),
            _ => fs::remove_file(&entry).unwrap(),
        }
        assert_eq!(ids(&fixture.resolved()), ["grove-core"], "{case}");
    }
}

#[test]
fn a_malformed_record_refuses_unless_the_settings_name_the_sources() {
    let fixture = Fixture::new();
    fixture.real_source("pack", &["skills"]);
    let record = fixture.keg.join(RECORD);
    for (case, text_of_record) in [
        ("not JSON", "{".to_owned()),
        (
            "a relative directory",
            json!({"schemaVersion": 1, "skillDirectory": "share/ravel/skills"}).to_string(),
        ),
        (
            "a repeated member",
            r#"{"schemaVersion": 1, "schemaVersion": 1, "skillDirectory": "/a"}"#.to_owned(),
        ),
    ] {
        fixture.record(&text_of_record);
        fixture.personal_rhai(UNPARSED);
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_directory_invalid", "{case}: {error}");
        assert_eq!(error["stage"], "skills", "{case}");
        assert_eq!(error["source"], text(&record), "{case}: {error}");
        let error = fixture.start().refusal(3)["error"].clone();
        assert_eq!(error["code"], "skill_directory_invalid", "{case}: {error}");
        assert!(fixture.no_cache(), "{case}: cache work");
    }
    // A record that is not a regular file refuses too.
    fs::remove_file(&record).unwrap();
    fs::create_dir(&record).unwrap();
    assert_eq!(fixture.refused(3)["code"], "skill_directory_invalid");

    // Settings that name `sources` read no record, so they launch beside it.
    let own = fixture.root.join("own");
    write(&own.join("s/x/SKILL.md"), "x");
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "sources": [
        {"id": "own", "type": "local", "path": own, "roots": ["s"]}
    ]}}));
    assert_eq!(ids(&fixture.resolved()), ["own"]);
    fixture.route();
    assert_eq!(
        fixture.start().report()["skills"]["defaultDirectory"],
        Value::Null
    );
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "sources": []}}));
    assert_eq!(
        fixture.resolved(),
        json!({"schemaVersion": 1, "status": "disabled"})
    );
}

#[test]
fn a_mandatory_reference_to_an_absent_default_source_refuses_as_missing() {
    let fixture = Fixture::new();
    fixture.linked_source("grove", "grove-core", &["skills"]);
    fixture.personal_rhai(UNPARSED);
    fixture.settings(&json!({"skills": {"schemaVersion": 1, "mandatory": [
        {"source": "grove-core", "path": "skills/grove-core-skills/SKILL.md"},
        {"source": "writing", "path": "skills/w/SKILL.md"}
    ]}}));
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_source_missing", "{error}");
    assert_eq!(error["stage"], "skills");
    assert_eq!(error["location"], "skills.mandatory[1].source", "{error}");
    assert!(
        error["remedy"]
            .as_str()
            .unwrap()
            .contains("install the package"),
        "{error}"
    );
    let error = fixture.start().refusal(3)["error"].clone();
    assert_eq!(error["code"], "skill_source_missing", "{error}");
    assert!(fixture.no_cache());
    // The control: once the package installs the source, the reference holds.
    fixture.linked_source("writing", "writing", &["skills"]);
    write(
        &fixture
            .root
            .join("cellar/writing/1.0/share/ravel/skills/writing/skills/w/SKILL.md"),
        "w",
    );
    assert_eq!(ids(&fixture.resolved()), ["grove-core", "writing"]);
    // Without a record, no default directory holds the source.
    fs::remove_file(fixture.keg.join(RECORD)).unwrap();
    assert_eq!(fixture.refused(3)["code"], "skill_source_missing");
}

#[test]
fn the_source_bounds_count_the_default_sources_after_the_listing() {
    let fixture = Fixture::new();
    for index in 0..16 {
        fixture.real_source(&format!("pack-{index:02}"), &["skills"]);
    }
    assert_eq!(ids(&fixture.resolved()).len(), 16);
    fixture.real_source("pack-16", &["skills"]);
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_limit_exceeded", "{error}");
    assert_eq!(error["location"], text(&fixture.directory), "{error}");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("default sources"),
        "{error}"
    );
}
