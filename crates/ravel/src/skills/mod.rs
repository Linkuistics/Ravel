//! Skill source resolution: the configured local sources, their retained
//! snapshots, the directory descriptor and the preamble. The skill discovery
//! specification owns the contract (docs/specs/skill-discovery.md).
//!
//! Ravel reads no `SKILL.md`, frontmatter or skill name here. It copies and
//! checks files, and it shows directories.

pub mod command;
mod preamble;
pub mod settings;
mod snapshot;

use std::collections::BTreeSet;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::DirBuilderExt as _;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

use crate::refusal::{EXIT_REFUSED, Refusal, Stage};
use crate::settings::{SETTINGS_FILE, Settings};
use settings::{Origin, SkillSettings};
use snapshot::{MANIFEST_FILE, TREE_DIR};

/// The cache root, relative to HOME. A later cache format uses a new
/// directory and keeps this one.
pub const CACHE_ROOT: &str = ".cache/ravel/skills/v1";
/// The protected configuration directory, relative to HOME.
const CONFIG_DIR: &str = ".config/ravel";
/// The descriptor file inside its retained directory.
pub const DESCRIPTOR_FILE: &str = "directories.json";

/// A configured source as refusals name it.
#[derive(Debug)]
pub struct Located {
    pub id: String,
    /// Its settings location, such as `skills.sources[1]`.
    pub location: String,
}

/// A refusal of the `skills` stage, exit 3.
pub fn refuse(
    code: &'static str,
    message: impl Into<String>,
    remedy: impl Into<String>,
) -> Refusal {
    Refusal::new(code, Stage::Skills, EXIT_REFUSED, message, remedy)
}

/// What one resolution produced.
#[derive(Debug)]
pub enum Resolution {
    Disabled,
    Resolved(Resolved),
}

/// A complete published directory set.
#[derive(Debug)]
pub struct Resolved {
    /// The absolute path of the retained `directories.json`.
    pub descriptor_path: String,
    /// The descriptor: `schemaVersion`, `sources`, `directories` and
    /// `mandatory`.
    pub descriptor: Value,
    /// The preamble, without a final newline.
    pub instructions: String,
}

impl Resolved {
    /// The `skills resolve --json` object.
    pub fn to_json(&self) -> Value {
        json!({
            "schemaVersion": 1,
            "status": "resolved",
            "descriptorPath": self.descriptor_path,
            "sources": self.descriptor["sources"],
            "directories": self.descriptor["directories"],
            "mandatory": self.descriptor["mandatory"],
            "instructions": self.instructions,
        })
    }
}

/// Resolve the configured sources of `settings`, read from `home`.
pub fn resolve(home: Option<&Path>, settings: &Settings) -> Result<Resolution, Refusal> {
    let (Some(home), Some(skills)) = (
        home.filter(|home| home.is_absolute()),
        settings.skills.as_ref().filter(|skills| skills.enabled()),
    ) else {
        return Ok(Resolution::Disabled);
    };
    refuse_git(skills)?;
    let protected = Protected::new(home);
    let origins = skills
        .sources
        .iter()
        .enumerate()
        .map(|(index, source)| origin(index, source, &protected))
        .collect::<Result<Vec<_>, _>>()?;
    let cache = Cache::open(home)?;

    let mut records = Vec::with_capacity(origins.len());
    for (index, (source, origin)) in skills.sources.iter().zip(&origins).enumerate() {
        let located = Located {
            id: source.id.clone(),
            location: format!("skills.sources[{index}]"),
        };
        let mandatory: BTreeSet<String> = skills
            .mandatory
            .iter()
            .filter(|reference| reference.source == index)
            .map(|reference| reference.path.clone())
            .collect();
        let mut capture =
            snapshot::capture(&origin.canonical, &cache.staging, &located, &mandatory)?;
        let checked = check_roots(&capture.tree(), source, origin, &located).and_then(|()| {
            skills
                .mandatory
                .iter()
                .enumerate()
                .filter(|(_, reference)| reference.source == index)
                .try_for_each(|(position, reference)| {
                    check_mandatory(&capture.tree(), &reference.path, origin, &located, position)
                })
        });
        if let Err(refusal) = checked {
            return Err(snapshot::after_cleanup(refusal, &mut capture.staging));
        }
        let snapshot::Capture {
            mut staging,
            manifest,
            digest,
        } = capture;
        if let Err(error) = fs::write(staging.path().join(MANIFEST_FILE), &manifest) {
            let refusal = snapshot::unavailable(staging.path(), &error);
            return Err(snapshot::after_cleanup(refusal, &mut staging));
        }
        let (object, _) =
            snapshot::publish(staging, &cache.snapshots, &digest, MANIFEST_FILE, &manifest)?;
        records.push(Record {
            id: source.id.clone(),
            digest,
            root: utf8(&object.join(TREE_DIR))?,
            origin: origin.text.clone(),
        });
    }

    let descriptor = descriptor(skills, &records);
    let bytes = serde_json::to_vec(&descriptor).expect("a JSON value encodes");
    let digest = snapshot::hex(&Sha256::digest(&bytes));
    let mut staging = snapshot::Staging::new(&cache.staging, "descriptor-")
        .map_err(|error| snapshot::unavailable(&cache.staging, &error))?;
    if let Err(error) = fs::write(staging.path().join(DESCRIPTOR_FILE), &bytes) {
        let refusal = snapshot::unavailable(staging.path(), &error);
        return Err(snapshot::after_cleanup(refusal, &mut staging));
    }
    let (directory, _) = snapshot::publish(
        staging,
        &cache.descriptors,
        &digest,
        DESCRIPTOR_FILE,
        &bytes,
    )?;
    let descriptor_path = utf8(&directory.join(DESCRIPTOR_FILE))?;
    let instructions = preamble::render(&descriptor, &descriptor_path);
    Ok(Resolution::Resolved(Resolved {
        descriptor_path,
        descriptor,
        instructions,
    }))
}

/// The local build acquires no Git source. It refuses a valid one before any
/// acquisition, with no stable public code, until the pinned stage lands.
fn refuse_git(skills: &SkillSettings) -> Result<(), Refusal> {
    let Some((index, source)) = skills
        .sources
        .iter()
        .enumerate()
        .find(|(_, source)| matches!(source.origin, Origin::Git { .. }))
    else {
        return Ok(());
    };
    Err(refuse(
        "skill_git_unsupported",
        format!(
            "the source {:?} is a valid Git source, but this build of Ravel acquires local \
             sources only",
            source.id
        ),
        "Git acquisition arrives with the unfinished pinned-delivery stage; use a local source \
         for now, or remove the Git source from the settings",
    )
    .location(format!("skills.sources[{index}]")))
}

/// The protected configuration: the configuration directory and the settings
/// file, each in its canonical form when it exists.
struct Protected {
    paths: Vec<PathBuf>,
}

impl Protected {
    fn new(home: &Path) -> Self {
        let mut paths = Vec::new();
        // The cache is a writable overlap: a source that holds it would
        // capture its own staging.
        let cache = home
            .canonicalize()
            .unwrap_or_else(|_| home.to_owned())
            .join(CACHE_ROOT);
        for path in [
            home.join(CONFIG_DIR),
            home.join(SETTINGS_FILE),
            home.join(CACHE_ROOT),
            cache,
        ] {
            if let Ok(canonical) = path.canonicalize() {
                paths.push(canonical);
            }
            paths.push(path);
        }
        Protected { paths }
    }

    /// The protected path that `source` contains or lies inside.
    fn overlap(&self, source: &Path) -> Option<&Path> {
        self.paths
            .iter()
            .find(|protected| source.starts_with(protected) || protected.starts_with(source))
            .map(PathBuf::as_path)
    }
}

struct LocalOrigin {
    canonical: PathBuf,
    text: String,
}

fn origin(
    index: usize,
    source: &settings::SourceSetting,
    protected: &Protected,
) -> Result<LocalOrigin, Refusal> {
    let Origin::Local { path } = &source.origin else {
        unreachable!("Git sources refuse before this point");
    };
    let at = format!("skills.sources[{index}]");
    let missing = |what: String| {
        refuse(
            "skill_source_missing",
            format!("the source {:?} at {path} {what}", source.id),
            "restore the source directory, or change the settings deliberately",
        )
        .source(path.clone())
        .location(format!("{at}.path"))
    };
    let canonical = match Path::new(path).canonicalize() {
        Ok(canonical) => canonical,
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {
            return Err(missing("does not exist".to_owned()));
        }
        Err(error) => return Err(missing(format!("cannot be reached: {error}"))),
    };
    if !canonical.is_dir() {
        return Err(missing("is not a directory".to_owned()));
    }
    if let Some(protected) = protected.overlap(&canonical) {
        return Err(refuse(
            "skill_access_unavailable",
            format!(
                "the source {:?} at {} overlaps the protected path {}",
                source.id,
                canonical.display(),
                protected.display()
            ),
            "move the skill source outside ~/.config/ravel and the skill cache, and keep the \
             settings file out of every source directory",
        )
        .source(canonical.display().to_string())
        .location(at));
    }
    let text = canonical.to_str().map(str::to_owned).ok_or_else(|| {
        refuse(
            "skill_resource_invalid",
            format!(
                "the canonical path {} of the source {:?} is not exact UTF-8",
                canonical.display(),
                source.id
            ),
            "move the source to a directory whose canonical path is UTF-8",
        )
        .source(canonical.display().to_string())
        .location(at.clone())
    })?;
    Ok(LocalOrigin { canonical, text })
}

/// Each root must be a directory in the captured tree. An empty one is valid.
fn check_roots(
    tree: &Path,
    source: &settings::SourceSetting,
    origin: &LocalOrigin,
    located: &Located,
) -> Result<(), Refusal> {
    for (index, root) in source.roots.iter().enumerate() {
        if fs::metadata(tree.join(root)).is_ok_and(|metadata| metadata.is_dir()) {
            continue;
        }
        let path = origin.canonical.join(root);
        return Err(refuse(
            "skill_source_missing",
            format!(
                "the root {root:?} of the source {:?} is not a directory in the captured source \
                 at {}",
                located.id,
                path.display()
            ),
            "restore the root directory in the source, or change the settings deliberately",
        )
        .source(path.display().to_string())
        .location(format!("{}.roots[{index}]", located.location)));
    }
    Ok(())
}

/// A mandatory reference must reach a UTF-8 regular file in the captured tree.
fn check_mandatory(
    tree: &Path,
    relative: &str,
    origin: &LocalOrigin,
    located: &Located,
    position: usize,
) -> Result<(), Refusal> {
    let path = origin.canonical.join(relative);
    let staged = tree.join(relative);
    let problem = match fs::metadata(&staged) {
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {
            Some("does not exist in the captured source".to_owned())
        }
        Err(error) => Some(format!("cannot be read: {error}")),
        Ok(metadata) if !metadata.is_file() => Some("is not a regular file".to_owned()),
        Ok(_) => match fs::read(&staged) {
            Err(error) => Some(format!("cannot be read: {error}")),
            Ok(bytes) if std::str::from_utf8(&bytes).is_err() => {
                Some("is not UTF-8 text".to_owned())
            }
            Ok(_) => None,
        },
    };
    match problem {
        None => Ok(()),
        Some(problem) => Err(refuse(
            "skill_mandatory_invalid",
            format!(
                "the mandatory file {relative:?} of the source {:?} at {} {problem}",
                located.id,
                path.display()
            ),
            "restore the mandatory file as UTF-8 text, or change the mandatory reference deliberately",
        )
        .source(path.display().to_string())
        .location(format!("skills.mandatory[{position}]"))),
    }
}

/// The private cache directories, created when absent.
struct Cache {
    staging: PathBuf,
    snapshots: PathBuf,
    descriptors: PathBuf,
}

impl Cache {
    fn open(home: &Path) -> Result<Cache, Refusal> {
        let root = home.join(CACHE_ROOT);
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true).mode(0o700);
        for name in ["staging", "snapshots", "descriptors"] {
            let path = root.join(name);
            builder
                .create(&path)
                .map_err(|error| snapshot::unavailable(&path, &error))?;
        }
        let root = root
            .canonicalize()
            .map_err(|error| snapshot::unavailable(&root, &error))?;
        utf8(&root)?;
        Ok(Cache {
            staging: root.join("staging"),
            snapshots: root.join("snapshots"),
            descriptors: root.join("descriptors"),
        })
    }
}

fn utf8(path: &Path) -> Result<String, Refusal> {
    path.to_str().map(str::to_owned).ok_or_else(|| {
        refuse(
            "skill_cache_unavailable",
            format!("the cache path {} is not exact UTF-8", path.display()),
            "use a HOME whose path is UTF-8",
        )
        .source(path.display().to_string())
    })
}

struct Record {
    id: String,
    digest: String,
    root: String,
    origin: String,
}

/// The descriptor value. Its member order is canonical, because a
/// `serde_json` object without `preserve_order` sorts its keys.
fn descriptor(skills: &SkillSettings, records: &[Record]) -> Value {
    let sources: Vec<Value> = records
        .iter()
        .map(|record| {
            json!({
                "id": record.id,
                "type": "local",
                "resourceDigest": record.digest,
                "snapshotRoot": record.root,
                "originPath": record.origin,
            })
        })
        .collect();
    let mut seen = BTreeSet::new();
    let mut directories = Vec::new();
    for (source, record) in skills.sources.iter().zip(records) {
        for root in &source.roots {
            let path = joined(&record.root, root);
            if seen.insert(path.clone()) {
                directories.push(json!({"source": record.id, "path": path}));
            }
        }
    }
    let mandatory: Vec<Value> = skills
        .mandatory
        .iter()
        .map(|reference| {
            let record = &records[reference.source];
            json!({"source": record.id, "path": joined(&record.root, &reference.path)})
        })
        .collect();
    json!({
        "schemaVersion": 1,
        "sources": sources,
        "directories": directories,
        "mandatory": mandatory,
    })
}

fn joined(root: &str, relative: &str) -> String {
    if relative == "." {
        root.to_owned()
    } else {
        format!("{root}/{relative}")
    }
}
