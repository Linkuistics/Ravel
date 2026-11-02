//! The public `ravel skills resolve` seam with local sources: temporary homes,
//! working directories and source trees, and no configuration script.
//! docs/specs/skill-discovery.md, "Test seams and acceptance".
mod support;

use std::fs;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};

const FRONT: &str = env!("CARGO_BIN_EXE_ravel");
const CACHE: &str = ".cache/ravel/skills/v1";

struct Fixture {
    root: PathBuf,
    home: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap().keep().canonicalize().unwrap();
        let home = root.join("home");
        for path in [
            &home,
            &root.join("work"),
            &root.join("tmp"),
            &root.join("bin"),
        ] {
            fs::create_dir_all(path).unwrap();
        }
        Fixture { root, home }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(FRONT);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("TMPDIR", self.root.join("tmp"))
            .env(
                "PATH",
                format!("{}:/usr/bin:/bin", self.root.join("bin").display()),
            )
            .current_dir(self.root.join("work"));
        command
    }

    fn settings(&self, text: &str) {
        let path = self.home.join(".config/ravel/settings.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn skills(&self, skills: Value) {
        self.settings(&json!({ "skills": skills }).to_string());
    }

    /// A source directory outside the working directory, from relative paths
    /// and their contents.
    fn source(&self, name: &str, files: &[(&str, &str)]) -> PathBuf {
        let source = self.root.join("sources").join(name);
        fs::create_dir_all(&source).unwrap();
        for (path, text) in files {
            let path = source.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        source
    }

    fn resolve(&self, json: bool) -> Output {
        let mut command = self.command();
        command.args(["skills", "resolve"]);
        if json {
            command.arg("--json");
        }
        command.output().unwrap()
    }

    fn resolved(&self) -> Value {
        let output = self.resolve(true);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        serde_json::from_slice(&output.stdout).unwrap()
    }

    /// The failure document of a refused resolution, after checking that
    /// nothing reached stdout.
    fn refused(&self, exit: i32) -> Value {
        let output = self.resolve(true);
        assert_eq!(output.status.code(), Some(exit), "{}", stderr(&output));
        assert!(
            output.stdout.is_empty(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let document: Value = serde_json::from_slice(&output.stderr).unwrap();
        document["error"].clone()
    }

    fn cache(&self) -> PathBuf {
        self.home.join(CACHE)
    }

    fn entries(&self, name: &str) -> Vec<String> {
        match fs::read_dir(self.cache().join(name)) {
            Err(_) => Vec::new(),
            Ok(listing) => {
                let mut names: Vec<String> = listing
                    .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                    .collect();
                names.sort();
                names
            }
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Snapshots are read-only, so restore write permission first.
        let _ = Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.root)
            .status();
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn local(id: &str, path: &Path, roots: &[&str]) -> Value {
    json!({ "id": id, "type": "local", "path": path, "roots": roots })
}

fn mandatory(source: &str, path: &str) -> Value {
    json!({ "source": source, "path": path })
}

fn skills(sources: Vec<Value>, mandatory: Vec<Value>) -> Value {
    json!({ "schemaVersion": 1, "sources": sources, "mandatory": mandatory })
}

fn paths(records: &Value) -> Vec<String> {
    records
        .as_array()
        .unwrap()
        .iter()
        .map(|record| record["path"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn disabled_delivery_prints_nothing_and_creates_no_cache() {
    let fixture = Fixture::new();
    let source = fixture.source("a", &[("SKILL.md", "x")]);
    let configurations: [Option<String>; 4] = [
        None,
        Some("{}".to_owned()),
        Some(r#"{"timeoutMs": 5000}"#.to_owned()),
        Some(json!({"skills": skills(vec![], vec![])}).to_string()),
    ];
    for settings in configurations {
        if let Some(settings) = &settings {
            fixture.settings(settings);
        }
        let output = fixture.resolve(false);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{settings:?}: {}",
            stderr(&output)
        );
        assert!(
            output.stdout.is_empty() && output.stderr.is_empty(),
            "{settings:?}"
        );
        let output = fixture.resolve(true);
        assert_eq!(output.status.code(), Some(0), "{settings:?}");
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            json!({"schemaVersion": 1, "status": "disabled"}),
            "{settings:?}"
        );
        assert!(!fixture.home.join(".cache").exists(), "{settings:?}");
    }
    // Without an absolute HOME, Ravel reads no settings, so the valid
    // enabled settings below are not read.
    fixture.skills(skills(vec![local("a", &source, &["."])], vec![]));
    for home in [None, Some("relative/home")] {
        let mut command = fixture.command();
        command.env_remove("HOME");
        if let Some(home) = home {
            command.env("HOME", home);
        }
        let output = command
            .args(["skills", "resolve", "--json"])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{home:?}: {}",
            stderr(&output)
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap()["status"],
            "disabled"
        );
    }
    assert!(!fixture.home.join(".cache").exists());
    assert!(!fixture.root.join("work/relative").exists());
}

#[test]
fn directory_delivery_shows_retained_paths_in_configured_order() {
    let fixture = Fixture::new();
    let core = fixture.source(
        "core",
        &[
            ("kinds/impl/SKILL.md", "impl"),
            ("craft/x/SKILL.md", "x"),
            ("spine.md", "spine"),
        ],
    );
    let extra = fixture.source(
        "extra",
        &[("skills/y/SKILL.md", "y"), ("rules.md", "rules")],
    );
    fixture.skills(skills(
        vec![
            local("core", &core, &["kinds", "craft"]),
            local("extra", &extra, &["skills", "."]),
        ],
        vec![
            mandatory("extra", "rules.md"),
            mandatory("core", "spine.md"),
        ],
    ));
    let resolved = fixture.resolved();
    assert_eq!(resolved["schemaVersion"], 1);
    assert_eq!(resolved["status"], "resolved");

    let sources = resolved["sources"].as_array().unwrap();
    let roots: Vec<&str> = sources
        .iter()
        .map(|s| s["snapshotRoot"].as_str().unwrap())
        .collect();
    assert_eq!(sources[0]["id"], "core");
    assert_eq!(sources[1]["id"], "extra");
    assert_eq!(sources[0]["originPath"], core.to_str().unwrap());
    assert_eq!(sources[0]["type"], "local");
    for (source, root) in sources.iter().zip(&roots) {
        let digest = source["resourceDigest"].as_str().unwrap();
        assert_eq!(digest.len(), 64);
        assert!(
            digest
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        );
        assert!(
            root.starts_with(fixture.cache().to_str().unwrap()),
            "{root}"
        );
        assert!(root.contains(digest), "{root}");
        // The source record holds exactly the specified fields.
        let mut fields: Vec<&str> = source
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        fields.sort_unstable();
        assert_eq!(
            fields,
            ["id", "originPath", "resourceDigest", "snapshotRoot", "type"]
        );
    }
    assert_eq!(
        paths(&resolved["directories"]),
        [
            format!("{}/kinds", roots[0]),
            format!("{}/craft", roots[0]),
            format!("{}/skills", roots[1]),
            roots[1].to_owned(),
        ]
    );
    assert_eq!(
        paths(&resolved["mandatory"]),
        [
            format!("{}/rules.md", roots[1]),
            format!("{}/spine.md", roots[0])
        ]
    );
    assert_eq!(resolved["mandatory"][0]["source"], "extra");

    // The descriptor holds the same records and nothing else.
    let descriptor_path = PathBuf::from(resolved["descriptorPath"].as_str().unwrap());
    let bytes = fs::read(&descriptor_path).unwrap();
    let descriptor: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        descriptor,
        json!({
            "schemaVersion": 1,
            "sources": resolved["sources"],
            "directories": resolved["directories"],
            "mandatory": resolved["mandatory"],
        })
    );
    // Its directory has the name of the digest of its bytes.
    let digest = sha256(&bytes);
    assert_eq!(
        descriptor_path.parent().unwrap().file_name().unwrap(),
        digest.as_str()
    );
    assert_eq!(descriptor_path.file_name().unwrap(), "directories.json");

    // The plain output is the same preamble with a final newline.
    let plain = fixture.resolve(false);
    assert_eq!(plain.status.code(), Some(0));
    let instructions = resolved["instructions"].as_str().unwrap();
    assert_eq!(
        String::from_utf8(plain.stdout).unwrap(),
        format!("{instructions}\n")
    );
    // Each path appears in the preamble as a JSON literal, in order.
    let mut at = 0;
    for path in paths(&resolved["directories"])
        .iter()
        .chain(&paths(&resolved["mandatory"]))
    {
        let quoted = serde_json::to_string(path).unwrap();
        let found = instructions[at..]
            .find(&quoted)
            .unwrap_or_else(|| panic!("{quoted}"));
        at += found + quoted.len();
    }
    assert!(instructions.contains(&serde_json::to_string(&descriptor_path).unwrap()));
    assert!(!instructions.contains("CALLER TASK"));
    // Resolution leaves no staging.
    assert!(fixture.entries("staging").is_empty());
}

#[test]
fn a_repeated_directory_path_appears_once_at_its_first_position() {
    let fixture = Fixture::new();
    let a = fixture.source("a", &[("s/SKILL.md", "same")]);
    let b = fixture.source("b", &[("s/SKILL.md", "same")]);
    fixture.skills(skills(
        vec![local("a", &a, &["s", "."]), local("b", &b, &["."])],
        vec![],
    ));
    let resolved = fixture.resolved();
    let root = resolved["sources"][0]["snapshotRoot"].as_str().unwrap();
    // Two sources with the same content share one snapshot, but keep their
    // provenance.
    assert_eq!(resolved["sources"][1]["snapshotRoot"], root);
    assert_eq!(resolved["sources"][1]["originPath"], b.to_str().unwrap());
    assert_eq!(
        paths(&resolved["directories"]),
        [format!("{root}/s"), root.to_owned()]
    );
    assert_eq!(resolved["directories"][1]["source"], "a");
}

#[test]
fn filesystem_contents_resolve_without_skill_semantics() {
    let fixture = Fixture::new();
    let source = fixture.source(
        "odd",
        &[
            ("one/SKILL.md", "---\nname: duplicate-zebra\n---\n"),
            ("two/SKILL.md", "---\nname: duplicate-zebra\n---\n"),
            ("broken/SKILL.md", "---\nname: [unclosed\ndescription: {\n"),
            ("no front matter/SKILL.md", "plain"),
            ("ünïcode \"quoted\" dir/SKILL.md", "u"),
        ],
    );
    fs::create_dir(source.join("empty")).unwrap();
    fixture.skills(skills(vec![local("odd", &source, &[".", "empty"])], vec![]));
    let resolved = fixture.resolved();
    let root = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(fs::read_dir(root.join("empty")).unwrap().count(), 0);
    assert_eq!(
        fs::read_to_string(root.join("ünïcode \"quoted\" dir/SKILL.md")).unwrap(),
        "u"
    );
    let instructions = resolved["instructions"].as_str().unwrap();
    assert!(!instructions.contains("zebra"), "Ravel reads no skill name");
}

#[test]
fn the_snapshot_keeps_the_whole_source_and_its_relative_resources() {
    let fixture = Fixture::new();
    let source = fixture.source(
        "pack",
        &[
            ("skills/a/SKILL.md", "see ../../shared/notes.md and ./ref.md"),
            ("skills/a/ref.md", "sibling"),
            ("shared/notes.md", "parent-relative"),
            ("nested/.git/HEAD", "excluded"),
            (".jj/repo", "excluded"),
            ("lfs/model.bin", "version https://git-lfs.github.com/spec/v1\noid sha256:4d7a214614ab2935c943f9e0ff69d22eadbb8f32b1258daaa5e2ca24d17e2393\nsize 12345\n"),
        ],
    );
    fs::create_dir(source.join(".git")).unwrap();
    fs::write(source.join(".git/config"), "excluded").unwrap();
    // Admitted links: to a file, to a directory, and a chain of two links.
    symlink("../shared/notes.md", source.join("skills/notes.md")).unwrap();
    symlink("../shared", source.join("skills/shared")).unwrap();
    symlink("notes.md", source.join("skills/chained.md")).unwrap();
    symlink("shared/notes.md", source.join("skills/through-dir.md")).unwrap();
    fs::write(source.join("tool.sh"), "#!/bin/sh\n").unwrap();
    fs::set_permissions(source.join("tool.sh"), fs::Permissions::from_mode(0o4755)).unwrap();
    fs::set_permissions(
        source.join("shared/notes.md"),
        fs::Permissions::from_mode(0o664),
    )
    .unwrap();
    fixture.skills(skills(vec![local("pack", &source, &["skills"])], vec![]));
    let resolved = fixture.resolved();
    let root = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    let skill = root.join("skills/a");
    assert_eq!(fs::read_to_string(skill.join("ref.md")).unwrap(), "sibling");
    assert_eq!(
        fs::read_to_string(skill.join("../../shared/notes.md")).unwrap(),
        "parent-relative"
    );
    for link in [
        "notes.md",
        "chained.md",
        "shared/notes.md",
        "through-dir.md",
    ] {
        assert_eq!(
            fs::read_to_string(root.join("skills").join(link)).unwrap(),
            "parent-relative"
        );
    }
    assert!(
        fs::symlink_metadata(root.join("skills/notes.md"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    // A committed LFS pointer is captured as literal bytes.
    assert_eq!(
        fs::read(root.join("lfs/model.bin")).unwrap(),
        fs::read(source.join("lfs/model.bin")).unwrap()
    );
    // The reserved entries are left out at every level.
    assert!(!root.join(".git").exists());
    assert!(!root.join(".jj").exists());
    assert!(!root.join("nested/.git").exists());
    assert!(root.join("nested").is_dir());
    // No write permission and no set-ID bit; the executable bit stays.
    let mode = |path: &Path| fs::metadata(path).unwrap().permissions().mode() & 0o7777;
    assert_eq!(mode(&root.join("tool.sh")), 0o555);
    assert_eq!(mode(&root.join("shared/notes.md")), 0o444);
    assert_eq!(mode(&root.join("shared")), 0o555);
    assert_eq!(mode(&root), 0o555);
    assert_eq!(mode(root.parent().unwrap()), 0o555);
    assert!(fs::write(root.join("shared/notes.md"), "x").is_err());
    assert!(fs::write(root.join("new.md"), "x").is_err());
}

#[test]
fn local_freshness_captures_current_bytes_and_old_readers_keep_old_bytes() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "first")]);
    fixture.skills(skills(vec![local("pack", &source, &["."])], vec![]));
    let first = fixture.resolved();
    let again = fixture.resolved();
    // Unchanged content reuses the published snapshot and descriptor.
    assert_eq!(first["sources"], again["sources"]);
    assert_eq!(first["descriptorPath"], again["descriptorPath"]);

    fs::write(source.join("SKILL.md"), "second").unwrap();
    let second = fixture.resolved();
    let old = PathBuf::from(first["sources"][0]["snapshotRoot"].as_str().unwrap());
    let new = PathBuf::from(second["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_ne!(old, new);
    assert_ne!(first["descriptorPath"], second["descriptorPath"]);
    assert_eq!(fs::read_to_string(old.join("SKILL.md")).unwrap(), "first");
    assert_eq!(fs::read_to_string(new.join("SKILL.md")).unwrap(), "second");
    let first_descriptor = PathBuf::from(first["descriptorPath"].as_str().unwrap());
    assert!(first_descriptor.is_file());

    // A removed source refuses, and its snapshot stays.
    fs::remove_dir_all(&source).unwrap();
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_source_missing");
    assert_eq!(fs::read_to_string(old.join("SKILL.md")).unwrap(), "first");
    assert_eq!(fixture.entries("snapshots").len(), 2);
}

#[test]
fn settings_refusals_come_before_any_acquisition() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "x")]);
    let ok = local("pack", &source, &["."]);
    let path = source.to_str().unwrap();
    for (settings, location) in [
        (
            json!({"skills": skills(vec![ok.clone()], vec![mandatory("other", "SKILL.md")])}).to_string(),
            "skills.mandatory[0].source",
        ),
        (
            json!({"skills": skills(vec![local("pack", &source, &["../x"])], vec![])}).to_string(),
            "skills.sources[0].roots[0]",
        ),
        (
            json!({"skills": skills(vec![json!({"id": "pack", "type": "local", "path": path, "roots": ["."], "extra": 1})], vec![])}).to_string(),
            "skills.sources[0].extra",
        ),
        (
            json!({"skills": {"schemaVersion": 1, "sources": [ok], "mandatory": [], "x": 1}}).to_string(),
            "skills.x",
        ),
        (
            format!(r#"{{"skills": {{"schemaVersion": 1, "sources": [{{"id": "pack", "id": "pack", "type": "local", "path": {path:?}, "roots": ["."]}}], "mandatory": []}}}}"#),
            "skills.sources[0].id",
        ),
        // A repeated member name refuses in every key, not only in skills.
        (r#"{"timeoutMs": 5000, "timeoutMs": 6000}"#.to_owned(), "timeoutMs"),
        (
            r#"{"harnesses": {"a": {"program": "a", "program": "b"}}}"#.to_owned(),
            "harnesses.a.program",
        ),
    ] {
        fixture.settings(&settings);
        let error = fixture.refused(2);
        assert_eq!(error["code"], "settings_invalid", "{settings}");
        assert_eq!(error["stage"], "cli", "{settings}");
        assert_eq!(error["location"], location, "{settings}");
        assert!(!fixture.home.join(".cache").exists(), "{settings}");
    }
}

#[test]
fn missing_sources_roots_and_mandatory_files_refuse_with_their_codes() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "x"), ("latin1.md", "")]);
    fs::write(source.join("latin1.md"), b"caf\xe9").unwrap();
    fs::create_dir(source.join("dir.md")).unwrap();
    let gone = fixture.root.join("sources/gone");
    for (skills_value, code, location) in [
        (
            skills(vec![local("gone", &gone, &["."])], vec![]),
            "skill_source_missing",
            "skills.sources[0].path",
        ),
        (
            skills(
                vec![local("pack", &source.join("SKILL.md"), &["."])],
                vec![],
            ),
            "skill_source_missing",
            "skills.sources[0].path",
        ),
        (
            skills(vec![local("pack", &source, &[".", "absent"])], vec![]),
            "skill_source_missing",
            "skills.sources[0].roots[1]",
        ),
        (
            skills(vec![local("pack", &source, &["SKILL.md"])], vec![]),
            "skill_source_missing",
            "skills.sources[0].roots[0]",
        ),
        (
            skills(
                vec![local("pack", &source, &["."])],
                vec![
                    mandatory("pack", "SKILL.md"),
                    mandatory("pack", "absent.md"),
                ],
            ),
            "skill_mandatory_invalid",
            "skills.mandatory[1]",
        ),
        (
            skills(
                vec![local("pack", &source, &["."])],
                vec![mandatory("pack", "latin1.md")],
            ),
            "skill_mandatory_invalid",
            "skills.mandatory[0]",
        ),
        (
            skills(
                vec![local("pack", &source, &["."])],
                vec![mandatory("pack", "dir.md")],
            ),
            "skill_mandatory_invalid",
            "skills.mandatory[0]",
        ),
    ] {
        fixture.skills(skills_value.clone());
        let error = fixture.refused(3);
        assert_eq!(error["code"], code, "{skills_value}");
        assert_eq!(error["stage"], "skills", "{skills_value}");
        assert_eq!(error["location"], location, "{skills_value}");
        assert!(error["source"].is_string(), "{skills_value}");
        assert!(error["remedy"].is_string(), "{skills_value}");
        // A refusal leaves no staging and no descriptor.
        assert!(fixture.entries("staging").is_empty(), "{skills_value}");
        assert!(fixture.entries("descriptors").is_empty(), "{skills_value}");
    }
}

#[test]
fn an_unreadable_mandatory_file_is_mandatory_guidance_and_another_file_a_resource() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "x"), ("secret.md", "x")]);
    fs::set_permissions(source.join("secret.md"), fs::Permissions::from_mode(0o000)).unwrap();
    if fs::read(source.join("secret.md")).is_ok() {
        // A privileged account reads any file, so this case cannot occur.
        return;
    }
    fixture.skills(skills(
        vec![local("pack", &source, &["."])],
        vec![mandatory("pack", "secret.md")],
    ));
    assert_eq!(fixture.refused(3)["code"], "skill_mandatory_invalid");
    // A link can lead the mandatory reference to the unreadable file.
    symlink("secret.md", source.join("mandatory.md")).unwrap();
    fixture.skills(skills(
        vec![local("pack", &source, &["."])],
        vec![mandatory("pack", "mandatory.md")],
    ));
    assert_eq!(fixture.refused(3)["code"], "skill_mandatory_invalid");
    fixture.skills(skills(vec![local("pack", &source, &["."])], vec![]));
    assert_eq!(fixture.refused(3)["code"], "skill_resource_invalid");
    fs::set_permissions(source.join("secret.md"), fs::Permissions::from_mode(0o644)).unwrap();
}

#[test]
fn two_mandatory_references_to_one_retained_file_refuse() {
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "x"), ("other.md", "y")]);
    symlink("SKILL.md", source.join("alias.md")).unwrap();
    let twin = fixture.source("twin", &[("SKILL.md", "x"), ("other.md", "y")]);
    symlink("SKILL.md", twin.join("alias.md")).unwrap();
    let pack = |id: &str| local(id, &source, &["."]);
    for (name, skills_value) in [
        (
            "one source path under two IDs",
            skills(
                vec![pack("a"), pack("b")],
                vec![mandatory("a", "SKILL.md"), mandatory("b", "SKILL.md")],
            ),
        ),
        (
            "an admitted link",
            skills(
                vec![pack("a")],
                vec![mandatory("a", "SKILL.md"), mandatory("a", "alias.md")],
            ),
        ),
        // Identical contents share one snapshot, so both references name one
        // retained path.
        (
            "two sources with identical contents",
            skills(
                vec![pack("a"), local("b", &twin, &["."])],
                vec![mandatory("a", "SKILL.md"), mandatory("b", "SKILL.md")],
            ),
        ),
    ] {
        fixture.skills(skills_value);
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_mandatory_invalid", "{name}: {error}");
        assert_eq!(error["location"], "skills.mandatory[1]", "{name}");
        assert!(fixture.entries("staging").is_empty(), "{name}");
        assert!(fixture.entries("descriptors").is_empty(), "{name}");
    }
    // Distinct files of one source remain valid.
    fixture.skills(skills(
        vec![pack("a")],
        vec![mandatory("a", "SKILL.md"), mandatory("a", "other.md")],
    ));
    assert_eq!(paths(&fixture.resolved()["mandatory"]).len(), 2);
}

#[test]
fn links_and_objects_that_a_snapshot_cannot_keep_refuse() {
    type Setup = fn(&Path);
    let cases: [(&str, Setup); 9] = [
        ("absolute", |s| {
            symlink("/etc/hosts", s.join("bad")).unwrap()
        }),
        ("escaping", |s| {
            symlink("../../outside", s.join("bad")).unwrap()
        }),
        ("escaping through a link", |s| {
            symlink("..", s.join("up")).unwrap();
            symlink("up/pack/SKILL.md", s.join("bad")).unwrap();
        }),
        // The target exists outside, and the same relative path exists
        // inside, so only the boundary check refuses it.
        ("escaping to an existing file", |s| {
            let outside = s.parent().unwrap().join("other");
            fs::create_dir_all(&outside).unwrap();
            fs::write(outside.join("file.md"), "outside").unwrap();
            fs::create_dir(s.join("other")).unwrap();
            fs::write(s.join("other/file.md"), "inside").unwrap();
            symlink("../other/file.md", s.join("bad")).unwrap();
        }),
        ("dangling", |s| symlink("absent.md", s.join("bad")).unwrap()),
        ("cyclic", |s| {
            symlink("b", s.join("a")).unwrap();
            symlink("a", s.join("b")).unwrap();
        }),
        ("into a reserved entry", |s| {
            fs::create_dir(s.join(".git")).unwrap();
            fs::write(s.join(".git/HEAD"), "x").unwrap();
            symlink(".git/HEAD", s.join("bad")).unwrap();
        }),
        ("a FIFO", |s| {
            let path = std::ffi::CString::new(s.join("fifo").to_str().unwrap()).unwrap();
            // SAFETY: a valid NUL-terminated path.
            assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        }),
        ("a control character", |s| {
            fs::write(s.join("a\u{1}b"), "x").unwrap()
        }),
    ];
    for (name, setup) in cases {
        let fixture = Fixture::new();
        let source = fixture.source("pack", &[("SKILL.md", "x")]);
        setup(&source);
        fixture.skills(skills(vec![local("pack", &source, &["."])], vec![]));
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_resource_invalid", "{name}: {error}");
        assert_eq!(error["location"], "skills.sources[0]", "{name}");
        assert!(fixture.entries("staging").is_empty(), "{name}");
        assert!(fixture.entries("snapshots").is_empty(), "{name}");
        assert!(fixture.entries("descriptors").is_empty(), "{name}");
    }
}

#[test]
fn a_link_to_another_spelling_of_a_reserved_entry_refuses() {
    for reserved in [".git", ".jj"] {
        let fixture = Fixture::new();
        let source = fixture.source("pack", &[("SKILL.md", "x")]);
        fs::create_dir(source.join(reserved)).unwrap();
        fs::write(source.join(reserved).join("HEAD"), "x").unwrap();
        let alias = reserved.to_uppercase();
        if fs::metadata(source.join(&alias).join("HEAD")).is_err() {
            // A case-sensitive filesystem has no such alias.
            return;
        }
        symlink(format!("{alias}/HEAD"), source.join("bad")).unwrap();
        fixture.skills(skills(vec![local("pack", &source, &["."])], vec![]));
        let error = fixture.refused(3);
        assert_eq!(
            error["code"], "skill_resource_invalid",
            "{reserved}: {error}"
        );
        assert!(fixture.entries("snapshots").is_empty(), "{reserved}");
    }
}

#[test]
fn a_link_to_a_hard_link_of_a_reserved_file_resolves() {
    // A worktree keeps `.git` as a file. Its hard link under another name is
    // an entry that capture keeps, so a link to that name is not dangling.
    let fixture = Fixture::new();
    let source = fixture.source("pack", &[("SKILL.md", "x"), (".git", "gitdir: x")]);
    fs::hard_link(source.join(".git"), source.join("gitfile.md")).unwrap();
    symlink("gitfile.md", source.join("good")).unwrap();
    fixture.skills(skills(vec![local("pack", &source, &["."])], vec![]));
    let root = PathBuf::from(
        fixture.resolved()["sources"][0]["snapshotRoot"]
            .as_str()
            .unwrap(),
    );
    assert_eq!(fs::read_to_string(root.join("good")).unwrap(), "gitdir: x");
    assert!(!root.join(".git").exists());
}

#[test]
fn a_cold_cache_below_an_aliased_segment_cannot_lie_inside_a_source() {
    // An aliased `.cache`, and a dangling link at `.cache/ravel`: the cache
    // walk would create the absent cache inside the source in both cases.
    type Setup = fn(&Path, &Path);
    let cases: [(&str, Setup); 2] = [
        ("an aliased ancestor", |home, outside| {
            symlink(outside, home.join(".cache")).unwrap();
        }),
        ("a dangling segment", |home, outside| {
            fs::create_dir(home.join(".cache")).unwrap();
            symlink(outside.join("held"), home.join(".cache/ravel")).unwrap();
        }),
    ];
    for (name, setup) in cases {
        let fixture = Fixture::new();
        let outside = fixture.source("outside", &[("SKILL.md", "x")]);
        setup(&fixture.home, &outside);
        fixture.skills(skills(vec![local("pack", &outside, &["."])], vec![]));
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_access_unavailable", "{name}: {error}");
        let mut listed: Vec<String> = fs::read_dir(&outside)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        listed.sort();
        assert_eq!(listed, ["SKILL.md"], "{name}: the refusal created no cache");
    }
}

#[test]
fn a_source_that_overlaps_the_protected_configuration_or_the_cache_refuses() {
    let fixture = Fixture::new();
    fixture.skills(skills(vec![], vec![]));
    let config = fixture.home.join(".config/ravel");
    let alias = fixture.root.join("alias");
    symlink(&config, &alias).unwrap();
    fs::create_dir_all(fixture.home.join(".cache")).unwrap();
    for (path, name) in [
        (config.clone(), "the configuration directory"),
        (alias.clone(), "an alias of it"),
        (fixture.home.join(".config"), "a parent of it"),
        (fixture.home.clone(), "HOME"),
        (fixture.home.join(".cache"), "a parent of the cache"),
    ] {
        fixture.skills(skills(vec![local("pack", &path, &["."])], vec![]));
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_access_unavailable", "{name}: {error}");
        assert!(fixture.entries("snapshots").is_empty(), "{name}");
    }
}

/// "Local sources only": a Git record refuses at its `type` as a settings
/// problem, and a Git checkout with uncommitted changes is an ordinary local
/// source. A spy `git` first on `PATH` sees no call.
#[test]
fn a_git_record_refuses_at_its_type_and_a_checkout_resolves_without_git() {
    let fixture = Fixture::new();
    let calls = fixture.root.join("git-calls");
    support::spy_git(&fixture.root.join("bin"), &calls);
    let source = fixture.source("pack", &[("SKILL.md", "x")]);
    fixture.skills(skills(
        vec![
            local("pack", &source, &["."]),
            json!({"id": "former", "type": "git", "url": "https://example.com/p.git",
                   "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": ".", "roots": ["."]}),
        ],
        vec![],
    ));
    let error = fixture.refused(2);
    assert_eq!(error["code"], "settings_invalid", "{error}");
    assert_eq!(error["stage"], "cli", "{error}");
    assert_eq!(error["location"], "skills.sources[1].type", "{error}");
    let remedy = error["remedy"].as_str().unwrap();
    for text in ["\"local\"", "checkout", "outside Ravel"] {
        assert!(remedy.contains(text), "{text}: {remedy}");
    }
    assert!(!fixture.home.join(".cache").exists());

    let checkout = fixture.root.join("sources/checkout");
    fs::create_dir_all(&checkout).unwrap();
    let (_, working) = support::git_checkout(&checkout, &fixture.root.join("tmp"));
    fixture.skills(skills(vec![local("checkout", &checkout, &["s"])], vec![]));
    let resolved = fixture.resolved();
    assert_eq!(resolved["sources"][0]["type"], "local");
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(
        fs::read_to_string(tree.join("s/x/SKILL.md")).unwrap(),
        working
    );
    assert_eq!(
        fs::read_to_string(tree.join("s/x/untracked.md")).unwrap(),
        "untracked\n"
    );
    assert!(!tree.join(".git").exists());
    assert!(!calls.exists(), "{}", fs::read_to_string(&calls).unwrap());
}

#[test]
fn a_failure_prints_text_without_json_and_nothing_on_stdout() {
    let fixture = Fixture::new();
    fixture.skills(skills(
        vec![local("gone", &fixture.root.join("gone"), &["."])],
        vec![],
    ));
    let output = fixture.resolve(false);
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    let text = stderr(&output);
    assert!(
        text.starts_with("ravel: refused (skill_source_missing, stage skills)"),
        "{text}"
    );
    assert!(text.contains("remedy:"), "{text}");
}

#[test]
fn the_help_shows_both_forms_the_disabled_result_and_the_cache_side_effect() {
    let fixture = Fixture::new();
    let output = fixture
        .command()
        .args(["skills", "resolve", "--help"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8(output.stdout).unwrap();
    for text in [
        "ravel skills resolve\n",
        "ravel skills resolve --json",
        "\"disabled\"",
        "~/.cache/ravel/skills/v1",
        "never removes them automatically",
    ] {
        assert!(help.contains(text), "{text}: {help}");
    }
}

fn sha256(bytes: &[u8]) -> String {
    use sha2::Digest as _;
    sha2::Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
