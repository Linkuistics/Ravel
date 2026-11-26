//! Internal fault seams of the default skill directory: a source that a
//! controlled hook removes after the listing and before its capture. The
//! public fixtures in `tests/skills_default.rs` cover what a copied
//! executable can show without a seam
//! (docs/specs/skill-discovery.md, "Changes under running sessions").

use std::fs;
use std::path::PathBuf;

use serde_json::json;

use super::faults::{Hooks, PathHook, install};
use super::installed::RECORD;
use super::{Resolution, resolve};
use crate::refusal::Refusal;
use crate::settings::Settings;

/// A HOME without settings, and an installation prefix whose record names a
/// default skill directory with the sources `alpha` and `beta`.
struct Installation {
    root: PathBuf,
    home: PathBuf,
    prefix: PathBuf,
    directory: PathBuf,
}

impl Installation {
    fn new() -> Installation {
        let root = tempfile::tempdir().unwrap().keep().canonicalize().unwrap();
        let home = root.join("home");
        let prefix = root.join("Cellar/ravel/2.0.0");
        let directory = root.join("share/ravel/skills");
        for id in ["alpha", "beta"] {
            let skill = directory.join(id).join("skills/one/SKILL.md");
            fs::create_dir_all(skill.parent().unwrap()).unwrap();
            fs::write(skill, id).unwrap();
        }
        fs::create_dir_all(&home).unwrap();
        let record = prefix.join(RECORD);
        fs::create_dir_all(record.parent().unwrap()).unwrap();
        fs::write(
            record,
            json!({"schemaVersion": 1, "skillDirectory": directory}).to_string(),
        )
        .unwrap();
        Installation {
            root,
            home,
            prefix,
            directory,
        }
    }

    fn resolve(&self, listed: Option<PathHook>) -> Result<Resolution, Refusal> {
        install(Hooks {
            prefix: Some(self.prefix.clone()),
            listed_default: listed,
            ..Hooks::default()
        });
        let settings = Settings::read(Some(self.home.as_os_str())).unwrap();
        let resolution = resolve(Some(&self.home), &settings);
        install(Hooks::default());
        resolution
    }

    fn descriptors(&self) -> usize {
        fs::read_dir(self.home.join(super::CACHE_ROOT).join("descriptors"))
            .map_or(0, Iterator::count)
    }
}

impl Drop for Installation {
    fn drop(&mut self) {
        let _ = std::process::Command::new("chmod")
            .arg("-R")
            .arg("u+w")
            .arg(&self.root)
            .status();
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn without_settings_the_record_supplies_the_default_sources() {
    let installation = Installation::new();
    let Resolution::Resolved(resolved) = installation.resolve(None).unwrap() else {
        panic!("the default sources enable delivery");
    };
    assert_eq!(
        resolved.default_directory.as_deref(),
        installation.directory.to_str()
    );
    let ids: Vec<&str> = resolved.descriptor["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["alpha", "beta"]);
}

#[test]
fn a_source_removed_after_the_listing_refuses_as_missing() {
    let installation = Installation::new();
    let removed = installation.directory.join("beta");
    let target = removed.clone();
    let refusal = installation
        .resolve(Some(Box::new(move |_| {
            fs::remove_dir_all(&target).unwrap()
        })))
        .unwrap_err();
    assert_eq!(refusal.code, "skill_source_missing", "{}", refusal.message);
    assert_eq!(refusal.source.as_deref(), removed.to_str());
    // A default source names its entry path in place of a settings location.
    assert_eq!(refusal.location.as_deref(), removed.to_str());
    assert!(
        refusal.remedy.contains("package manager"),
        "{}",
        refusal.remedy
    );
    assert_eq!(installation.descriptors(), 0, "a descriptor was published");
    // The rerun after the upgrade lists only what remains, and refuses nothing.
    assert!(matches!(
        installation.resolve(None).unwrap(),
        Resolution::Resolved(_)
    ));
}
