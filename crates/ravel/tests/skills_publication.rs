//! Capture and publication at the public `ravel skills resolve` seam: path
//! identity on real cache filesystems, concurrent and killed publishers,
//! controlled concurrent edits, tampered objects, unsafe cache roots, failed
//! storage and the ownership of staging.
//! docs/specs/skill-discovery.md, "Local freshness", "Publication,
//! concurrency and cleanup" and "Test seams and acceptance".
//!
//! Capture is not atomic against every concurrent writer. The edit fixtures
//! here show that an observed change refuses and that every published object
//! is complete. A successful retry after an edit does not prove atomicity.
use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

const FRONT: &str = env!("CARGO_BIN_EXE_ravel");
const CACHE: &str = ".cache/ravel/skills/v1";

struct Fixture {
    root: PathBuf,
    home: PathBuf,
    #[cfg(target_os = "macos")]
    images: Vec<Image>,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap().keep().canonicalize().unwrap();
        Self::with_home(root.clone(), root.join("home"))
    }

    fn with_home(root: PathBuf, home: PathBuf) -> Self {
        for path in [&home, &root.join("work"), &root.join("tmp")] {
            fs::create_dir_all(path).unwrap();
        }
        Fixture {
            root,
            home,
            #[cfg(target_os = "macos")]
            images: Vec::new(),
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(FRONT);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("TMPDIR", self.root.join("tmp"))
            .env("PATH", "/usr/bin:/bin")
            .current_dir(self.root.join("work"));
        command
    }

    fn skills(&self, source: &Path) {
        let path = self.home.join(".config/ravel/settings.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let skills = json!({"skills": {"schemaVersion": 1, "sources": [
            {"id": "a", "type": "local", "path": source, "roots": ["."]}
        ], "mandatory": []}});
        fs::write(path, skills.to_string()).unwrap();
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
        serde_json::from_slice(&output.stdout).unwrap()
    }

    /// The error object of a refused resolution with `exit`, after checking
    /// that nothing reached stdout.
    fn refused(&self, exit: i32) -> Value {
        let output = self.resolve();
        assert_eq!(output.status.code(), Some(exit), "{}", stderr(&output));
        assert!(output.stdout.is_empty());
        let document: Value = serde_json::from_slice(&output.stderr).unwrap();
        document["error"].clone()
    }

    fn cache(&self) -> PathBuf {
        self.home.join(CACHE)
    }

    fn names(&self, directory: &str) -> Vec<String> {
        names(&self.cache().join(directory))
    }

    #[cfg(target_os = "macos")]
    fn image(&mut self, name: &str, filesystem: &str, size: &str) -> PathBuf {
        let image = Image::new(&self.root, name, filesystem, size);
        let mount = image.mount.clone();
        self.images.push(image);
        mount
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        self.images.clear();
        let _ = Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.root)
            .status();
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// An attached disk image of one filesystem type, detached on drop.
#[cfg(target_os = "macos")]
struct Image {
    mount: PathBuf,
}

#[cfg(target_os = "macos")]
impl Image {
    fn new(dir: &Path, name: &str, filesystem: &str, size: &str) -> Image {
        let dmg = dir.join(format!("{name}.dmg"));
        let mount = dir.join(name);
        let created = Command::new("hdiutil")
            .args([
                "create", "-quiet", "-size", size, "-fs", filesystem, "-volname", name,
            ])
            .arg(&dmg)
            .status()
            .unwrap();
        assert!(created.success(), "hdiutil create {filesystem}");
        let attached = Command::new("hdiutil")
            .args(["attach", "-quiet", "-nobrowse", "-mountpoint"])
            .arg(&mount)
            .arg(&dmg)
            .status()
            .unwrap();
        assert!(attached.success(), "hdiutil attach {filesystem}");
        Image {
            mount: mount.canonicalize().unwrap(),
        }
    }
}

#[cfg(target_os = "macos")]
impl Drop for Image {
    fn drop(&mut self) {
        let _ = Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.mount)
            .status();
        let _ = Command::new("hdiutil")
            .args(["detach", "-quiet", "-force"])
            .arg(&self.mount)
            .status();
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .map(|listing| {
            listing
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// A source of `count` files in nested directories, so that a capture takes
/// long enough for publishers, readers and editors to overlap with it.
fn wide_source(dir: &Path, count: usize) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    for index in 0..count {
        let relative = format!("s{}/d{}/f{index}.md", index % 4, index % 16);
        let text = format!("{index} {}\n", "x".repeat(512));
        write(&dir.join(&relative), &text);
        files.insert(relative, text);
    }
    files
}

/// Whether the published snapshot `object` holds a manifest and exactly
/// `files` in its tree. A reader that sees an object at all must see this.
fn complete_snapshot(object: &Path, files: &BTreeMap<String, String>) -> bool {
    let tree = object.join("tree");
    if names(object) != ["manifest", "tree"] || fs::read(object.join("manifest")).is_err() {
        return false;
    }
    files
        .iter()
        .all(|(relative, text)| fs::read_to_string(tree.join(relative)).ok().as_ref() == Some(text))
}

/// Every published descriptor parses and names only complete snapshots.
fn descriptors_complete(fixture: &Fixture, files: &BTreeMap<String, String>) -> usize {
    let mut count = 0;
    for name in fixture.names("descriptors") {
        let object = fixture.cache().join("descriptors").join(&name);
        assert_eq!(names(&object), ["directories.json"], "{name}");
        let bytes = fs::read(object.join("directories.json")).unwrap();
        let descriptor: Value = serde_json::from_slice(&bytes).unwrap();
        for source in descriptor["sources"].as_array().unwrap() {
            let root = PathBuf::from(source["snapshotRoot"].as_str().unwrap());
            assert!(complete_snapshot(root.parent().unwrap(), files), "{name}");
        }
        count += 1;
    }
    count
}

#[test]
fn publishers_behind_a_barrier_and_an_existing_reader_see_no_partial_object() {
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    let files = wide_source(&source, 200);
    fixture.skills(&source);
    let barrier = fixture.root.join("barrier");
    assert!(
        Command::new("mkfifo")
            .arg(&barrier)
            .status()
            .unwrap()
            .success()
    );

    // The reader is already looking when the publishers start.
    let stop = Arc::new(AtomicBool::new(false));
    let seen = Arc::new(AtomicUsize::new(0));
    let reader = {
        let (stop, seen, cache, files) =
            (stop.clone(), seen.clone(), fixture.cache(), files.clone());
        thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                for name in names(&cache.join("snapshots")) {
                    let object = cache.join("snapshots").join(name);
                    assert!(complete_snapshot(&object, &files), "a partial snapshot");
                    seen.fetch_add(1, Ordering::SeqCst);
                }
                for name in names(&cache.join("descriptors")) {
                    let file = cache
                        .join("descriptors")
                        .join(name)
                        .join("directories.json");
                    let descriptor: Value =
                        serde_json::from_slice(&fs::read(file).unwrap()).unwrap();
                    assert_eq!(descriptor["schemaVersion"], 1, "a partial descriptor");
                }
            }
        })
    };
    let publishers: Vec<Child> = (0..6)
        .map(|_| {
            Command::new("/bin/sh")
                .args([
                    "-c",
                    r#"read _ < "$1"; exec "$2" skills resolve --json"#,
                    "sh",
                ])
                .arg(&barrier)
                .arg(FRONT)
                .env_clear()
                .env("HOME", &fixture.home)
                .env("TMPDIR", fixture.root.join("tmp"))
                .env("PATH", "/usr/bin:/bin")
                .current_dir(fixture.root.join("work"))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
        })
        .collect();
    // One line for each publisher releases the publishers that wait. The write
    // end stays open until every publisher ends. A publisher that opens the
    // barrier late then reads its line and does not block. A shell reads a
    // pipe one byte at a time, so each publisher takes exactly one line. The
    // sleep only makes it probable that the publishers start together.
    thread::sleep(Duration::from_millis(200));
    let mut release = fs::OpenOptions::new().write(true).open(&barrier).unwrap();
    release.write_all(&[b'\n'; 6]).unwrap();
    let outputs: Vec<Output> = publishers
        .into_iter()
        .map(|child| child.wait_with_output().unwrap())
        .collect();
    drop(release);
    stop.store(true, Ordering::SeqCst);
    reader.join().unwrap();

    let mut paths = Vec::new();
    for output in &outputs {
        assert_eq!(output.status.code(), Some(0), "{}", stderr(output));
        let resolved: Value = serde_json::from_slice(&output.stdout).unwrap();
        paths.push(resolved["descriptorPath"].as_str().unwrap().to_owned());
    }
    paths.dedup();
    assert_eq!(paths.len(), 1, "{paths:?}");
    // One snapshot and one descriptor, and every loser removed its staging.
    assert_eq!(fixture.names("snapshots").len(), 1);
    assert_eq!(descriptors_complete(&fixture, &files), 1);
    assert!(
        fixture.names("staging").is_empty(),
        "{:?}",
        fixture.names("staging")
    );
    assert!(
        seen.load(Ordering::SeqCst) > 0,
        "the reader never saw the object"
    );
}

/// Wait until the staging directory of the cache holds a name that `before`
/// does not, or until `child` exits. Whether the name appeared.
fn staging_appeared(fixture: &Fixture, before: &[String], child: &mut Child) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        if fixture
            .names("staging")
            .iter()
            .any(|name| !before.contains(name))
        {
            return true;
        }
        if child.try_wait().unwrap().is_some() || std::time::Instant::now() > deadline {
            return false;
        }
        thread::sleep(Duration::from_micros(200));
    }
}

#[test]
fn a_killed_publisher_never_publishes_a_partial_descriptor() {
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    let files = wide_source(&source, 300);
    fixture.skills(&source);
    // A child counts as interrupted only when it was killed after its
    // staging appeared, so it had begun to capture. The delays after that
    // milestone sample capture and publication. No fixture can aim at the
    // moment between the rename of an object and its seal.
    let mut interrupted = 0;
    for delay in [0, 5, 10, 20, 40, 60, 90, 130, 180, 250] {
        let before = fixture.names("staging");
        let mut child = fixture
            .command()
            .args(["skills", "resolve", "--json"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let reached = staging_appeared(&fixture, &before, &mut child);
        thread::sleep(Duration::from_millis(delay));
        let _ = child.kill();
        let status = child.wait().unwrap();
        if reached && status.code().is_none() {
            interrupted += 1;
        }
        descriptors_complete(&fixture, &files);
        for name in fixture.names("snapshots") {
            let object = fixture.cache().join("snapshots").join(name);
            assert!(complete_snapshot(&object, &files));
        }
    }
    assert!(
        interrupted > 0,
        "no publisher was killed after its capture began"
    );
    // A killed publisher leaves its staging. The next resolution succeeds and
    // removes only its own staging.
    let left = fixture.names("staging");
    let resolved = fixture.resolved();
    assert_eq!(fixture.names("staging"), left);
    let root = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert!(complete_snapshot(root.parent().unwrap(), &files));
}

#[test]
fn ordinary_cleanup_removes_only_the_staging_of_its_own_resolver() {
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    write(&source.join("s/SKILL.md"), "s");
    fixture.skills(&source);
    fixture.resolved();
    let foreign = fixture.cache().join("staging/capture-another-resolver");
    write(&foreign.join("tree/x.md"), "x");
    fixture.resolved();
    assert_eq!(fixture.names("staging"), ["capture-another-resolver"]);
    fs::remove_dir_all(source.join("s")).unwrap();
    write(&source.join("other.md"), "o");
    fixture.resolved();
    assert_eq!(fixture.names("staging"), ["capture-another-resolver"]);
}

#[test]
fn controlled_concurrent_edits_refuse_as_changed_and_every_published_object_is_complete() {
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    let mut files = wide_source(&source, 200);
    let churn = "s0/d0/churn.md";
    let states = ["first\n", "second, longer\n"];
    write(&source.join(churn), states[0]);
    fixture.skills(&source);

    // The editor writes each state beside the source and renames it into
    // place, so the file always holds one complete state. A truncating write
    // could let a quiet capture keep an empty file, which the non-atomic
    // contract allows.
    let stop = Arc::new(AtomicBool::new(false));
    let editor = {
        let (stop, path, spare) = (
            stop.clone(),
            source.join(churn),
            fixture.root.join("tmp/churn"),
        );
        thread::spawn(move || {
            let mut index = 0;
            while !stop.load(Ordering::SeqCst) {
                index = 1 - index;
                fs::write(&spare, states[index]).unwrap();
                fs::rename(&spare, &path).unwrap();
                thread::sleep(Duration::from_micros(300));
            }
        })
    };
    // The scheduler decides whether an edit falls inside a capture, so the
    // fixture resolves until it observes a refusal, within a bound.
    let (mut refused, mut succeeded) = (0, 0);
    for attempt in 0..40 {
        if attempt >= 6 && refused > 0 {
            break;
        }
        let output = fixture.resolve();
        match output.status.code() {
            Some(0) => {
                succeeded += 1;
                let resolved: Value = serde_json::from_slice(&output.stdout).unwrap();
                let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
                let copy = fs::read_to_string(tree.join(churn)).unwrap();
                assert!(states.contains(&copy.as_str()), "{copy:?}");
            }
            Some(3) => {
                refused += 1;
                assert!(output.stdout.is_empty());
                let document: Value = serde_json::from_slice(&output.stderr).unwrap();
                assert_eq!(
                    document["error"]["code"], "skill_source_changed",
                    "{document}"
                );
                assert!(
                    document["error"]["remedy"]
                        .as_str()
                        .unwrap()
                        .contains("try again"),
                    "{document}"
                );
            }
            code => panic!("{code:?}: {}", stderr(&output)),
        }
    }
    stop.store(true, Ordering::SeqCst);
    editor.join().unwrap();
    assert!(
        refused > 0,
        "no edit was observed ({succeeded} resolutions succeeded)"
    );
    assert!(
        fixture.names("staging").is_empty(),
        "{:?}",
        fixture.names("staging")
    );
    // Every published object is complete for the state that it copied.
    for name in fixture.names("snapshots") {
        let object = fixture.cache().join("snapshots").join(name);
        let copy = fs::read_to_string(object.join("tree").join(churn)).unwrap();
        files.insert(churn.to_owned(), copy);
        assert!(complete_snapshot(&object, &files));
    }

    // With the source quiet, the digests before and after a capture agree,
    // and a retry succeeds.
    let before = fixture.resolved();
    let after = fixture.resolved();
    assert_eq!(before["sources"], after["sources"]);
    let tree = PathBuf::from(after["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(
        fs::read_to_string(tree.join(churn)).unwrap(),
        fs::read_to_string(source.join(churn)).unwrap()
    );
}

/// Make `path` writable, so that a fixture can tamper with a sealed object.
fn unseal(path: &Path) {
    let mode = if path.is_dir() { 0o700 } else { 0o600 };
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
}

#[test]
fn a_tampered_snapshot_or_descriptor_refuses_as_corrupt_and_stays_unrepaired() {
    type Tamper = fn(&Path);
    let snapshot_cases: [(&str, Tamper); 4] = [
        ("content", |object| {
            let file = object.join("tree/s/SKILL.md");
            unseal(&file);
            fs::write(&file, "tampered").unwrap();
        }),
        ("an added file", |object| {
            unseal(&object.join("tree/s"));
            fs::write(object.join("tree/s/extra.md"), "x").unwrap();
        }),
        ("a removed file", |object| {
            unseal(&object.join("tree/s"));
            fs::remove_file(object.join("tree/s/notes.md")).unwrap();
        }),
        ("the manifest", |object| {
            let file = object.join("manifest");
            unseal(&file);
            let mut bytes = fs::read(&file).unwrap();
            bytes.push(b'\n');
            fs::write(&file, bytes).unwrap();
        }),
    ];
    for (case, tamper) in snapshot_cases {
        let fixture = Fixture::new();
        let source = fixture.root.join("source");
        write(&source.join("s/SKILL.md"), "s");
        write(&source.join("s/notes.md"), "notes");
        fixture.skills(&source);
        let resolved = fixture.resolved();
        let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
        let object = tree.parent().unwrap().to_owned();
        tamper(&object);
        let listing = fs::read_dir(&object)
            .map(|_| names(&object.join("tree/s")))
            .unwrap();
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_cache_corrupt", "{case}: {error}");
        assert_eq!(error["source"], object.to_str().unwrap(), "{case}");
        assert!(
            error["remedy"].as_str().unwrap().contains("quarantine"),
            "{case}: {error}"
        );
        // Ravel did not repair the object under its readers.
        assert_eq!(names(&object.join("tree/s")), listing, "{case}");
        assert!(fixture.names("staging").is_empty(), "{case}");
    }

    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    write(&source.join("s/SKILL.md"), "s");
    fixture.skills(&source);
    let resolved = fixture.resolved();
    let path = PathBuf::from(resolved["descriptorPath"].as_str().unwrap());
    unseal(&path);
    fs::write(&path, "{}").unwrap();
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_cache_corrupt", "{error}");
    assert_eq!(error["source"], path.parent().unwrap().to_str().unwrap());
    assert_eq!(fs::read_to_string(&path).unwrap(), "{}");
}

#[test]
fn an_unsafe_cache_path_a_permission_failure_and_a_safe_link_have_their_outcomes() {
    // A link to a cache in a directory that every account can write.
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    write(&source.join("s/SKILL.md"), "s");
    fixture.skills(&source);
    let shared = fixture.root.join("shared");
    fs::create_dir_all(shared.join("cache")).unwrap();
    fs::set_permissions(&shared, fs::Permissions::from_mode(0o777)).unwrap();
    symlink(shared.join("cache"), fixture.home.join(".cache")).unwrap();
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_cache_unavailable", "{error}");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("writable by other accounts"),
        "{error}"
    );
    assert!(!shared.join("cache/ravel").exists());

    // A sticky shared directory protects the account's own entry in it.
    fs::set_permissions(&shared, fs::Permissions::from_mode(0o1777)).unwrap();
    let resolved = fixture.resolved();
    let descriptor = resolved["descriptorPath"].as_str().unwrap();
    assert!(
        descriptor.starts_with(shared.join("cache").to_str().unwrap()),
        "{descriptor}"
    );

    // A cache root that other accounts can open.
    let root = shared.join("cache/ravel/skills/v1");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o750)).unwrap();
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_cache_unavailable", "{error}");
    assert!(
        error["remedy"].as_str().unwrap().contains("chmod 700"),
        "{error}"
    );
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();

    // A permission failure in the staging directory.
    fs::set_permissions(root.join("staging"), fs::Permissions::from_mode(0o500)).unwrap();
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_cache_unavailable", "{error}");
    assert!(
        error["remedy"]
            .as_str()
            .unwrap()
            .contains("write permission"),
        "{error}"
    );
    fs::set_permissions(root.join("staging"), fs::Permissions::from_mode(0o700)).unwrap();
    fixture.resolved();
}

#[cfg(target_os = "macos")]
#[test]
fn full_storage_refuses_as_cache_unavailable_and_leaves_no_staging() {
    let mut fixture = Fixture::new();
    let volume = fixture.image("full", "APFS", "8m");
    fixture.home = volume.join("home");
    fs::create_dir_all(&fixture.home).unwrap();
    let source = fixture.root.join("source");
    write(&source.join("s/SKILL.md"), "s");
    fs::write(source.join("s/large.bin"), vec![7u8; 16 * 1024 * 1024]).unwrap();
    fixture.skills(&source);
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_cache_unavailable", "{error}");
    assert!(
        error["remedy"]
            .as_str()
            .unwrap()
            .contains("free storage on the filesystem of the cache"),
        "{error}"
    );
    assert!(fixture.names("staging").is_empty());
    assert!(fixture.names("descriptors").is_empty());
}

/// Two names that differ only in case, as files and as directory segments.
fn case_pair(source: &Path) {
    write(&source.join("s/x/A.md"), "upper");
    write(&source.join("s/x/a.md"), "lower");
}

fn case_segments(source: &Path) {
    write(&source.join("s/D/one.md"), "one");
    write(&source.join("s/d/two.md"), "two");
}

#[cfg(target_os = "macos")]
#[test]
fn paths_that_a_case_insensitive_cache_cannot_keep_apart_refuse_and_name_both() {
    for (case, make, first, second) in [
        ("files", case_pair as fn(&Path), "s/x/A.md", "s/x/a.md"),
        ("directory segments", case_segments, "s/D", "s/d"),
    ] {
        let mut fixture = Fixture::new();
        // The source keeps both names; the cache on the default volume of the
        // release platform does not.
        let source = fixture
            .image("source", "Case-sensitive APFS", "16m")
            .join("src");
        make(&source);
        fixture.skills(&source);
        let error = fixture.refused(3);
        assert_eq!(error["code"], "skill_resource_invalid", "{case}: {error}");
        let message = error["message"].as_str().unwrap();
        for path in [first, second] {
            assert!(
                message.contains(source.join(path).to_str().unwrap()),
                "{case}: {message}"
            );
        }
        assert!(fixture.names("descriptors").is_empty(), "{case}");
        assert!(fixture.names("snapshots").is_empty(), "{case}");
        assert!(fixture.names("staging").is_empty(), "{case}");
    }
}

#[cfg(target_os = "macos")]
#[test]
fn a_case_sensitive_cache_keeps_both_paths() {
    let mut fixture = Fixture::new();
    let volume = fixture.image("both", "Case-sensitive APFS", "16m");
    fixture.home = volume.join("home");
    fs::create_dir_all(&fixture.home).unwrap();
    let source = volume.join("src");
    case_pair(&source);
    case_segments(&source);
    fixture.skills(&source);
    let resolved = fixture.resolved();
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(fs::read_to_string(tree.join("s/x/A.md")).unwrap(), "upper");
    assert_eq!(fs::read_to_string(tree.join("s/x/a.md")).unwrap(), "lower");
    assert_eq!(names(&tree.join("s")), ["D", "d", "x"]);
}

#[cfg(target_os = "macos")]
#[test]
fn a_cache_filesystem_that_normalises_a_name_refuses_and_names_the_path() {
    let mut fixture = Fixture::new();
    // HFS+ stores names in its decomposed form, so an NFC directory segment
    // of the source does not keep its spelling there.
    let volume = fixture.image("hfs", "HFS+", "16m");
    fixture.home = volume.join("home");
    fs::create_dir_all(&fixture.home).unwrap();
    let source = fixture.root.join("source");
    let nfc = "caf\u{e9}";
    write(&source.join(format!("s/{nfc}/SKILL.md")), "s");
    assert_eq!(names(&source.join("s")), [nfc]);
    fixture.skills(&source);
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_resource_invalid", "{error}");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains(&format!("s/{nfc}")), "{message}");
    // The message escapes the stored spelling, so the combining mark shows.
    assert!(message.contains(r"cafe\u{301}"), "{message}");
    assert!(fixture.names("descriptors").is_empty());
}

#[cfg(target_os = "linux")]
#[test]
fn a_filesystem_that_keeps_case_and_normalisation_forms_keeps_both() {
    let fixture = Fixture::new();
    let source = fixture.root.join("source");
    case_pair(&source);
    case_segments(&source);
    write(&source.join("s/caf\u{e9}.md"), "nfc");
    write(&source.join("s/cafe\u{301}.md"), "nfd");
    fixture.skills(&source);
    let resolved = fixture.resolved();
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(fs::read_to_string(tree.join("s/x/A.md")).unwrap(), "upper");
    assert_eq!(
        fs::read_to_string(tree.join("s/caf\u{e9}.md")).unwrap(),
        "nfc"
    );
    assert_eq!(
        fs::read_to_string(tree.join("s/cafe\u{301}.md")).unwrap(),
        "nfd"
    );
}

#[cfg(target_os = "macos")]
#[test]
fn a_link_that_only_the_source_filesystem_resolves_refuses_on_a_case_sensitive_cache() {
    let mut fixture = Fixture::new();
    // The source is on the case-insensitive volume of the temporary directory,
    // so the link reaches `Guide.md` through another spelling.
    let source = fixture.root.join("source");
    write(&source.join("s/Guide.md"), "guide");
    symlink("guide.md", source.join("s/alias.md")).unwrap();
    assert_eq!(
        fs::read_to_string(source.join("s/alias.md")).unwrap(),
        "guide"
    );
    fixture.skills(&source);

    // On a case-insensitive cache the retained link works, so it resolves.
    let resolved = fixture.resolved();
    let tree = PathBuf::from(resolved["sources"][0]["snapshotRoot"].as_str().unwrap());
    assert_eq!(
        fs::read_to_string(tree.join("s/alias.md")).unwrap(),
        "guide"
    );

    // On a case-sensitive cache the same link would be dangling.
    let volume = fixture.image("sensitive", "Case-sensitive APFS", "16m");
    fixture.home = volume.join("home");
    fs::create_dir_all(&fixture.home).unwrap();
    fixture.skills(&source);
    let error = fixture.refused(3);
    assert_eq!(error["code"], "skill_resource_invalid", "{error}");
    let message = error["message"].as_str().unwrap();
    assert!(message.contains("s/alias.md"), "{message}");
    assert!(message.contains("cache filesystem"), "{message}");
    assert!(fixture.names("descriptors").is_empty());
    assert!(fixture.names("snapshots").is_empty());
    assert!(fixture.names("staging").is_empty());
}
