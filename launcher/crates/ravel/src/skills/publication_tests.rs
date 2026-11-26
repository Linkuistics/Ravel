//! Internal fault seams of capture and publication: controlled source edits
//! at fixed points of the scan, a cache link or cache directory replaced
//! before publication, a staging directory replaced before or during its
//! rename, a planted winner, a rename that the platform does not support and a
//! failed removal of staging. The public fixtures in
//! `tests/skills_publication.rs` cover what the CLI can show without a seam.
//!
//! These edits are controlled, so each refusal here is certain. They do not
//! prove that capture is atomic against every concurrent writer, which it is
//! not (docs/specs/skill-discovery.md#local-freshness).

use std::cell::Cell;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde_json::json;
use sha2::{Digest as _, Sha256};

use super::faults::{Hooks, install};
use super::{CACHE_ROOT, Resolution, resolve};
use crate::refusal::Refusal;
use crate::settings::Settings;

struct Home {
    root: PathBuf,
    home: PathBuf,
    source: PathBuf,
}

impl Home {
    fn new() -> Home {
        let root = tempfile::tempdir().unwrap().keep().canonicalize().unwrap();
        let home = root.join("home");
        let source = root.join("source");
        for (path, text) in [
            ("s/a/SKILL.md", "a"),
            ("s/a/notes.md", "notes"),
            ("s/b/SKILL.md", "b"),
            ("top.md", "top"),
        ] {
            let path = source.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        fs::create_dir_all(&home).unwrap();
        let home = Home { root, home, source };
        home.settings(&home.source.clone());
        home
    }

    fn settings(&self, source: &Path) {
        let path = self.home.join(".config/ravel/settings.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let skills = json!({"skills": {"schemaVersion": 1, "sources": [
            {"id": "a", "type": "local", "path": source, "roots": ["s"]}
        ], "mandatory": []}});
        fs::write(path, skills.to_string()).unwrap();
    }

    fn resolve(&self) -> Result<String, Refusal> {
        let settings = Settings::read(Some(self.home.as_os_str())).unwrap();
        match resolve(Some(&self.home), &settings)? {
            Resolution::Resolved(resolved) => Ok(resolved.descriptor_path),
            Resolution::Disabled => panic!("delivery is enabled"),
        }
    }

    fn cache(&self) -> PathBuf {
        self.home.join(CACHE_ROOT)
    }

    fn names(&self, directory: &str) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(self.cache().join(directory))
            .map(|listing| {
                listing
                    .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        names
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        install(Hooks::default());
        let _ = std::process::Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.root)
            .status();
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// A resolution that refuses with `code` leaves no new snapshot, no
/// descriptor and no staging.
fn refused_cleanly(home: &Home, code: &str) -> Refusal {
    let snapshots = home.names("snapshots");
    let descriptors = home.names("descriptors");
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(refusal.code, code, "{}", refusal.message);
    assert_eq!(home.names("snapshots"), snapshots);
    assert_eq!(home.names("descriptors"), descriptors);
    assert!(
        home.names("staging").is_empty(),
        "{:?}",
        home.names("staging")
    );
    refusal
}

/// An edit at a hook point of one file, run once.
fn on_copy(target: &'static str, edit: impl FnOnce(&Path) + 'static) -> Hooks {
    let mut edit = Some(edit);
    Hooks {
        copied: Some(Box::new(move |path: &Path| {
            if path.ends_with(target)
                && let Some(edit) = edit.take()
            {
                edit(path);
            }
        })),
        ..Hooks::default()
    }
}

fn on_list(target: &'static str, edit: impl FnOnce(&Path) + 'static) -> Hooks {
    let mut edit = Some(edit);
    Hooks {
        listed: Some(Box::new(move |path: &Path| {
            if path.ends_with(target)
                && let Some(edit) = edit.take()
            {
                edit(path);
            }
        })),
        ..Hooks::default()
    }
}

#[test]
fn a_controlled_edit_during_capture_refuses_as_changed_with_no_partial_descriptor() {
    // Each case names the layer that must see it first: the look after the
    // copy, the second listing or the second look at each entry.
    let copied = "changed while Ravel copied it";
    let listing = "gained or lost an entry";
    let second = "changed during capture";
    let cases: Vec<(&str, &str, Hooks)> = vec![
        (
            "same-size content rewrite",
            copied,
            on_copy("s/a/notes.md", |path| fs::write(path, "NOTES").unwrap()),
        ),
        (
            "growth",
            copied,
            on_copy("s/a/notes.md", |path| {
                fs::write(path, "notes, longer").unwrap()
            }),
        ),
        (
            "mode change",
            copied,
            on_copy("s/a/notes.md", |path| {
                fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
            }),
        ),
        (
            "replacement by a new file",
            copied,
            on_copy("s/a/notes.md", |path| {
                fs::remove_file(path).unwrap();
                fs::write(path, "notes").unwrap();
            }),
        ),
        (
            "an added entry",
            listing,
            on_list("s/a", |path| fs::write(path.join("new.md"), "new").unwrap()),
        ),
        (
            "a removed entry",
            listing,
            on_list("s/a", |path| {
                fs::remove_file(path.join("notes.md")).unwrap()
            }),
        ),
        (
            "an in-place rewrite after the listing",
            second,
            on_list("s/a", |path| {
                fs::write(path.join("notes.md"), "NOTES").unwrap()
            }),
        ),
        (
            "a directory replaced by a link with its name",
            second,
            on_list("source/s", |path| {
                let outside = path.parent().unwrap().parent().unwrap().join("b-outside");
                fs::rename(path.join("b"), outside).unwrap();
                symlink("a", path.join("b")).unwrap();
            }),
        ),
    ];
    for (case, layer, hooks) in cases {
        let home = Home::new();
        // The digest of the quiet source before the edit.
        let before = home.resolve().unwrap();
        let before_snapshots = home.names("snapshots");
        install(hooks);
        let refusal = refused_cleanly(&home, "skill_source_changed");
        assert_eq!(refusal.exit, 3, "{case}");
        assert!(
            refusal.remedy.contains("try again"),
            "{case}: {}",
            refusal.remedy
        );
        assert!(
            refusal.message.contains(layer),
            "{case}: {}",
            refusal.message
        );
        if case == "an in-place rewrite after the listing" {
            // Only the second look at the entry sees this edit: the directory
            // itself did not change.
            assert!(
                refusal.source.as_deref().unwrap().ends_with("s/a/notes.md"),
                "{:?}",
                refusal.source
            );
        }
        assert_eq!(home.names("snapshots"), before_snapshots, "{case}");
        // A quiet retry captures the edited source, with a new digest, and
        // leaves the earlier snapshot unchanged.
        // Replacement restores the same bytes, so only that case keeps the
        // digest.
        let after = home.resolve().unwrap();
        assert_eq!(
            after == before,
            case == "replacement by a new file",
            "{case}"
        );
    }
}

#[test]
fn a_quiet_hook_changes_nothing() {
    let home = Home::new();
    let quiet = home.resolve().unwrap();
    install(on_copy("s/a/notes.md", |_| {}));
    assert_eq!(home.resolve().unwrap(), quiet);
}

#[test]
fn a_cache_link_replaced_before_publication_refuses_and_the_substitute_receives_nothing() {
    let home = Home::new();
    let (first, second) = (home.root.join("first"), home.root.join("second"));
    fs::create_dir_all(&first).unwrap();
    fs::create_dir_all(&second).unwrap();
    symlink(&first, home.home.join(".cache")).unwrap();
    let link = home.home.join(".cache");
    let target = second.clone();
    let mut swapped = false;
    install(Hooks {
        publishing: Some(Box::new(move |_| {
            if !swapped {
                swapped = true;
                fs::remove_file(&link).unwrap();
                symlink(&target, &link).unwrap();
            }
        })),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(
        refusal.code, "skill_cache_unavailable",
        "{}",
        refusal.message
    );
    assert!(
        refusal.message.contains("no longer reaches"),
        "{}",
        refusal.message
    );
    // The substitute tree holds nothing, and the held tree no staging and
    // no descriptor.
    assert_eq!(fs::read_dir(&second).unwrap().count(), 0);
    let held = first.join("ravel/skills/v1");
    assert_eq!(fs::read_dir(held.join("staging")).unwrap().count(), 0);
    assert_eq!(fs::read_dir(held.join("descriptors")).unwrap().count(), 0);
}

/// Plant a winner beside the staged snapshot just before it publishes: a copy
/// of the staging with `alter` applied to its tree.
fn plant_winner(cache: PathBuf, alter: impl Fn(&Path) + 'static) -> Hooks {
    let mut planted = false;
    Hooks {
        publishing: Some(Box::new(move |_| {
            if planted {
                return;
            }
            let staging = cache.join("staging");
            let Some(capture) = fs::read_dir(&staging)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .find(|path| {
                    path.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with("capture-")
                })
            else {
                return;
            };
            planted = true;
            let manifest = fs::read(capture.join("manifest")).unwrap();
            let digest = super::snapshot::hex(&Sha256::digest(&manifest));
            let winner = cache.join("snapshots").join(digest);
            let copied = std::process::Command::new("cp")
                .arg("-R")
                .arg(&capture)
                .arg(&winner)
                .status()
                .unwrap();
            assert!(copied.success());
            let _ = std::process::Command::new("chmod")
                .arg("-R")
                .arg("u+w")
                .arg(&winner)
                .status();
            alter(&winner.join("tree"));
        })),
        ..Hooks::default()
    }
}

#[test]
fn the_losing_publisher_reads_the_winner_back_and_reuses_only_an_identical_one() {
    let home = Home::new();
    install(plant_winner(home.cache(), |_| {}));
    home.resolve().unwrap();
    install(Hooks::default());
    assert!(home.names("staging").is_empty());
    assert_eq!(home.names("snapshots").len(), 1);

    type Alter = Box<dyn Fn(&Path)>;
    let alterations: Vec<(&str, Alter)> = vec![
        (
            "content",
            Box::new(|tree: &Path| fs::write(tree.join("top.md"), "TOP").unwrap()),
        ),
        (
            "an added file",
            Box::new(|tree: &Path| fs::write(tree.join("extra.md"), "x").unwrap()),
        ),
        (
            "a removed file",
            Box::new(|tree: &Path| fs::remove_file(tree.join("top.md")).unwrap()),
        ),
        (
            "an executable bit",
            Box::new(|tree: &Path| {
                fs::set_permissions(tree.join("top.md"), fs::Permissions::from_mode(0o755))
                    .unwrap();
            }),
        ),
    ];
    for (case, alter) in alterations {
        let home = Home::new();
        install(plant_winner(home.cache(), alter));
        let refusal = home.resolve().unwrap_err();
        install(Hooks::default());
        assert_eq!(
            refusal.code, "skill_cache_corrupt",
            "{case}: {}",
            refusal.message
        );
        assert!(
            refusal.remedy.contains("quarantine"),
            "{case}: {}",
            refusal.remedy
        );
        assert!(home.names("descriptors").is_empty(), "{case}");
        assert!(home.names("staging").is_empty(), "{case}");
        // Ravel did not repair the planted winner.
        let winner = home
            .cache()
            .join("snapshots")
            .join(&home.names("snapshots")[0]);
        assert!(winner.join("tree").exists(), "{case}");
    }
}

#[test]
fn a_replaced_cache_directory_refuses_and_its_substitute_receives_nothing() {
    for child in ["staging", "snapshots", "descriptors"] {
        let home = Home::new();
        let held = home.cache().join(child);
        let aside = home.cache().join(format!("{child}-held"));
        let (from, to) = (held.clone(), aside.clone());
        let mut swapped = false;
        install(Hooks {
            publishing: Some(Box::new(move |_| {
                if !swapped {
                    swapped = true;
                    fs::rename(&from, &to).unwrap();
                    fs::create_dir(&from).unwrap();
                    fs::set_permissions(&from, fs::Permissions::from_mode(0o700)).unwrap();
                }
            })),
            ..Hooks::default()
        });
        let refusal = home.resolve().unwrap_err();
        install(Hooks::default());
        assert_eq!(
            refusal.code, "skill_cache_unavailable",
            "{child}: {}",
            refusal.message
        );
        assert!(
            refusal.message.contains("no longer reaches"),
            "{child}: {}",
            refusal.message
        );
        assert!(
            refusal.message.contains(&format!("v1/{child}")),
            "{child}: {}",
            refusal.message
        );
        // The substitute holds nothing, and the held directories no staging
        // and no descriptor.
        assert_eq!(fs::read_dir(&held).unwrap().count(), 0, "{child}");
        for name in ["staging", "descriptors"] {
            let dir = if name == child {
                aside.clone()
            } else {
                home.cache().join(name)
            };
            assert_eq!(fs::read_dir(&dir).unwrap().count(), 0, "{child}: {name}");
        }
    }
}

/// Move the first capture staging aside at a hook point and put a copy with
/// a changed resource at its name.
fn substitute_staging(cache: PathBuf) -> impl FnMut(&Path) {
    let mut done = false;
    move |_| {
        if done {
            return;
        }
        let staging = cache.join("staging");
        let Some(capture) = fs::read_dir(&staging)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("capture-")
            })
        else {
            return;
        };
        done = true;
        let aside = staging.join("aside");
        fs::rename(&capture, &aside).unwrap();
        let copied = std::process::Command::new("cp")
            .arg("-R")
            .arg(&aside)
            .arg(&capture)
            .status()
            .unwrap();
        assert!(copied.success());
        let _ = std::process::Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&capture)
            .status();
        fs::write(capture.join("tree/top.md"), "SUBSTITUTE").unwrap();
    }
}

#[test]
fn a_replaced_staging_is_never_published_and_cleanup_spares_it() {
    let home = Home::new();
    install(Hooks {
        publishing: Some(Box::new(substitute_staging(home.cache()))),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(
        refusal.code, "skill_cache_unavailable",
        "{}",
        refusal.message
    );
    assert!(
        refusal.message.contains("was replaced"),
        "{}",
        refusal.message
    );
    // Cleanup removed the contents of its own staging, which is now aside,
    // and left the substitute at the staging name. It reports the name.
    assert!(
        refusal.message.contains("was not removed"),
        "{}",
        refusal.message
    );
    assert!(home.names("snapshots").is_empty());
    assert!(home.names("descriptors").is_empty());
    let staging = home.names("staging");
    assert_eq!(staging.len(), 2, "{staging:?}");
    let aside = home.cache().join("staging/aside");
    assert_eq!(fs::read_dir(aside).unwrap().count(), 0);
    let substitute = staging.iter().find(|name| *name != "aside").unwrap();
    let substitute = home.cache().join("staging").join(substitute);
    assert_eq!(
        fs::read_to_string(substitute.join("tree/top.md")).unwrap(),
        "SUBSTITUTE"
    );
}

#[test]
fn a_staging_replaced_just_before_its_rename_refuses_after_the_rename() {
    let home = Home::new();
    install(Hooks {
        renaming: Some(Box::new(substitute_staging(home.cache()))),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(
        refusal.code, "skill_cache_unavailable",
        "{}",
        refusal.message
    );
    assert!(
        refusal.message.contains("was replaced"),
        "{}",
        refusal.message
    );
    assert!(refusal.remedy.contains("quarantine"), "{}", refusal.remedy);
    // The substitute took the snapshot name, so no descriptor names it.
    assert!(home.names("descriptors").is_empty());
    let aside = home.cache().join("staging/aside");
    assert_eq!(fs::read_dir(aside).unwrap().count(), 0);
}

/// The errors with which this platform reports that it has no rename without
/// replacement.
#[cfg(target_os = "linux")]
const NOREPLACE_UNSUPPORTED: [i32; 2] = [libc::ENOSYS, libc::EINVAL];
#[cfg(target_os = "macos")]
const NOREPLACE_UNSUPPORTED: [i32; 1] = [libc::ENOTSUP];

#[test]
fn a_cache_without_a_rename_that_refuses_an_existing_name_refuses_and_publishes_nothing() {
    for errno in NOREPLACE_UNSUPPORTED {
        let home = Home::new();
        install(Hooks {
            renamed: Some(Box::new(move |_| {
                Err(std::io::Error::from_raw_os_error(errno))
            })),
            ..Hooks::default()
        });
        let refusal = refused_cleanly(&home, "skill_cache_unavailable");
        assert!(
            refusal
                .message
                .contains("no rename that refuses an existing name"),
            "{errno}: {}",
            refusal.message
        );
        assert!(
            refusal.remedy.contains("rename without replacement"),
            "{errno}: {}",
            refusal.remedy
        );
        assert!(home.names("snapshots").is_empty());
        // Nothing stays from the refusal, so the next resolution publishes.
        home.resolve().unwrap();
        assert_eq!(home.names("snapshots").len(), 1);
    }

    // Another failed rename keeps the general remedy of the cache.
    let home = Home::new();
    install(Hooks {
        renamed: Some(Box::new(|_| {
            Err(std::io::Error::from_raw_os_error(libc::EIO))
        })),
        ..Hooks::default()
    });
    let refusal = refused_cleanly(&home, "skill_cache_unavailable");
    assert!(
        !refusal.remedy.contains("rename without replacement"),
        "{}",
        refusal.remedy
    );
}

#[test]
fn a_winner_that_differs_only_in_bytes_that_are_not_utf8_refuses_as_corrupt() {
    // U+FFFD is valid UTF-8, so capture keeps it in a name and a link target.
    // A lossy readback would spell the byte FF the same way.
    let replacement = "\u{fffd}.md";
    type Alter = Box<dyn Fn(&Path) -> bool>;
    let alterations: Vec<(&str, Alter)> = vec![
        (
            "a link target",
            Box::new(|tree: &Path| {
                let link = tree.join("s/a/l.md");
                fs::remove_file(&link).unwrap();
                symlink(OsStr::from_bytes(b"\xff.md"), &link).unwrap();
                true
            }),
        ),
        (
            "a file name",
            Box::new(move |tree: &Path| {
                // APFS refuses a name that is not UTF-8; Linux keeps it.
                fs::rename(
                    tree.join("s/a").join(replacement),
                    tree.join("s/a").join(OsStr::from_bytes(b"\xff.md")),
                )
                .is_ok()
            }),
        ),
    ];
    for (case, alter) in alterations {
        let home = Home::new();
        fs::write(home.source.join("s/a").join(replacement), "r").unwrap();
        symlink(replacement, home.source.join("s/a/l.md")).unwrap();
        let altered = Rc::new(Cell::new(false));
        let seen = altered.clone();
        install(plant_winner(home.cache(), move |tree| {
            seen.set(alter(tree));
        }));
        let outcome = home.resolve();
        install(Hooks::default());
        if !altered.get() {
            eprintln!("{case}: this filesystem cannot hold the altered name");
            continue;
        }
        let refusal = outcome.unwrap_err();
        assert_eq!(
            refusal.code, "skill_cache_corrupt",
            "{case}: {}",
            refusal.message
        );
        assert!(home.names("descriptors").is_empty(), "{case}");
    }
}

#[test]
fn a_failed_removal_of_staging_refuses_as_cleanup_failed_or_joins_the_original_refusal() {
    let home = Home::new();
    home.resolve().unwrap();
    // The second resolution loses to the first snapshot and removes its copy.
    install(Hooks {
        removing: Some(Box::new(|_| Err(std::io::Error::other("held by a fault")))),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(refusal.code, "skill_cleanup_failed");
    assert_eq!(refusal.exit, 5);
    assert!(
        refusal.message.contains("held by a fault"),
        "{}",
        refusal.message
    );

    // A refused resolution keeps its own code and reports the failed removal.
    let home = Home::new();
    fs::write(home.source.join("s/a/fifo-stand-in"), "x").unwrap();
    install(Hooks {
        copied: Some(Box::new(|path: &Path| {
            if path.ends_with("s/a/fifo-stand-in") {
                fs::write(path, "changed").unwrap();
            }
        })),
        removing: Some(Box::new(|_| Err(std::io::Error::other("held by a fault")))),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(refusal.code, "skill_source_changed");
    assert!(
        refusal.message.contains("was not removed"),
        "{}",
        refusal.message
    );
}

/// The bounds of the descriptor and the preamble cannot be reached on macOS
/// within the other bounds, so these tests set each limit to the measured
/// size and to one byte less.
#[test]
fn the_descriptor_and_the_preamble_pass_at_their_bounds_and_refuse_one_byte_more() {
    use super::bounds::Limits;
    let home = Home::new();
    let settings = Settings::read(Some(home.home.as_os_str())).unwrap();
    let Resolution::Resolved(first) = resolve(Some(&home.home), &settings).unwrap() else {
        panic!("delivery is enabled");
    };
    let descriptor = fs::read(&first.descriptor_path).unwrap().len() as u64;
    let preamble = first.instructions.len() as u64;
    for (name, limits) in [
        (
            "descriptorBytes",
            (
                Limits {
                    descriptor_bytes: descriptor - 1,
                    ..Limits::SPECIFIED
                },
                Limits {
                    descriptor_bytes: descriptor,
                    ..Limits::SPECIFIED
                },
            ),
        ),
        (
            "preambleBytes",
            (
                Limits {
                    preamble_bytes: preamble - 1,
                    ..Limits::SPECIFIED
                },
                Limits {
                    preamble_bytes: preamble,
                    ..Limits::SPECIFIED
                },
            ),
        ),
    ] {
        // New content gives a new snapshot and a new descriptor of the same
        // size, because each digest has a fixed length.
        fs::write(home.source.join("s/a/notes.md"), name).unwrap();
        let descriptors = home.names("descriptors");
        install(Hooks {
            limits: Some(limits.0),
            ..Hooks::default()
        });
        let refusal = home.resolve().unwrap_err();
        assert_eq!(refusal.code, "skill_limit_exceeded", "{}", refusal.message);
        assert_eq!(refusal.bound.unwrap().name, name);
        assert!(
            refusal.message.contains("over the bound"),
            "{}",
            refusal.message
        );
        assert_eq!(home.names("descriptors"), descriptors);
        assert!(home.names("staging").is_empty());
        install(Hooks {
            limits: Some(limits.1),
            ..Hooks::default()
        });
        home.resolve().unwrap();
        install(Hooks::default());
        assert_eq!(home.names("descriptors").len(), descriptors.len() + 1);
    }
}

/// A refusal that a resolution reaches after a signal was consumed reports
/// the cancellation, and keeps the refusal as the original failure.
#[test]
fn a_consumed_signal_wins_over_a_resolution_refusal() {
    use super::bounds::Limits;
    let home = Home::new();
    install(Hooks {
        limits: Some(Limits {
            sources: 0,
            ..Limits::SPECIFIED
        }),
        signal: crate::cancellation::Signal::of(libc::SIGTERM),
        ..Hooks::default()
    });
    let refusal = home.resolve().unwrap_err();
    install(Hooks::default());
    assert_eq!(refusal.code, "skill_resolution_cancelled");
    assert_eq!(refusal.exit, 143);
    assert!(
        refusal.message.contains("original failure:"),
        "{}",
        refusal.message
    );
    assert!(
        refusal.message.contains("skill_limit_exceeded"),
        "{}",
        refusal.message
    );
    assert!(!home.root.join("home").join(CACHE_ROOT).exists());
}

/// A deadline that passes during the copy of a file refuses and removes the
/// staging of the capture.
#[test]
fn a_deadline_that_passes_during_a_copy_removes_the_staging() {
    let home = Home::new();
    let path = home.home.join(".config/ravel/settings.json");
    let skills = json!({"skills": {"schemaVersion": 1, "resolutionTimeoutMs": 1000, "sources": [
        {"id": "a", "type": "local", "path": home.source, "roots": ["s"]}
    ], "mandatory": []}});
    fs::write(path, skills.to_string()).unwrap();
    install(Hooks {
        copied: Some(Box::new(|_| {
            std::thread::sleep(std::time::Duration::from_millis(1_050))
        })),
        ..Hooks::default()
    });
    let refusal = refused_cleanly(&home, "skill_resolution_timeout");
    assert_eq!(refusal.exit, 124);
    assert!(
        refusal.message.contains("during capture"),
        "{}",
        refusal.message
    );
}
