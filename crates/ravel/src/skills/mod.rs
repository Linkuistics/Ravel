//! Skill source resolution: the configured local sources or the sources of
//! the default skill directory, their retained snapshots, the directory
//! descriptor and the preamble. The skill discovery
//! specification owns the contract (docs/specs/skill-discovery.md).
//!
//! Ravel reads no `SKILL.md`, frontmatter or skill name here. It copies and
//! checks files, and it shows directories.

mod bounds;
mod cache;
pub mod command;
#[cfg(test)]
mod default_tests;
mod dir;
mod installed;
pub mod launch;
mod preamble;
#[cfg(test)]
mod publication_tests;
pub mod settings;
mod snapshot;
pub mod watch;

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

#[cfg(test)]
use crate::completion::EXIT_FILE_VARIABLE;
use crate::refusal::{EXIT_REFUSED, Refusal, Stage};
use crate::settings::{SETTINGS_FILE, Settings};
use bounds::{Actual, Limits};
use cache::Cache;
#[cfg(not(test))]
use ravel::EXIT_FILE_VARIABLE;
use settings::SkillSettings;
use snapshot::{Budget, Expected, MANIFEST_FILE, TREE_DIR};
use watch::Watch;

/// The cache root, relative to HOME. A later cache format uses a new
/// directory and keeps this one.
pub const CACHE_ROOT: &str = ".cache/ravel/skills/v1";
/// The protected configuration directory, relative to HOME.
const CONFIG_DIR: &str = ".config/ravel";
/// The descriptor file inside its retained directory.
pub const DESCRIPTOR_FILE: &str = "directories.json";

/// A source as refusals name it.
#[derive(Debug)]
pub struct Located {
    pub id: String,
    /// Its settings location, such as `skills.sources[1]`, or the entry path
    /// of a default source.
    pub location: String,
}

/// Where a source of the source set comes from.
#[derive(Clone, Debug)]
enum Place {
    /// The index of a configured source in `skills.sources`.
    Setting(usize),
    /// The entry path of a default source. A refusal names it in place of a
    /// settings location
    /// (docs/specs/skill-discovery.md#entries-and-roots).
    Entry(String),
}

impl Place {
    /// The location of the field `suffix` of this source, such as `.path`.
    /// A default source has no fields, so its location is its entry path.
    fn location(&self, suffix: &str) -> String {
        match self {
            Place::Setting(index) => format!("skills.sources[{index}]{suffix}"),
            Place::Entry(path) => path.clone(),
        }
    }
}

/// A source of the source set, configured or default.
#[derive(Clone, Debug)]
struct Source {
    id: String,
    path: String,
    roots: Vec<String>,
    place: Place,
}

/// A mandatory reference, with the position of its source in the source set.
#[derive(Clone, Debug)]
struct Mandatory {
    source: usize,
    path: String,
}

/// The sources that one resolution captures, and the mandatory references
/// into them.
#[derive(Debug)]
struct SourceSet {
    sources: Vec<Source>,
    mandatory: Vec<Mandatory>,
    /// The default skill directory when it supplied the source set.
    default_directory: Option<String>,
}

impl SourceSet {
    /// Delivery is disabled when the source set and the mandatory references
    /// are both empty.
    fn empty(&self) -> bool {
        self.sources.is_empty() && self.mandatory.is_empty()
    }

    /// The settings location of the sources, or the default skill directory.
    fn location(&self) -> String {
        self.default_directory
            .clone()
            .unwrap_or_else(|| "skills.sources".to_owned())
    }

    /// How a bound refusal names the sources.
    fn origin(&self) -> &'static str {
        match self.default_directory {
            Some(_) => "default",
            None => "configured",
        }
    }
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
    /// The default skill directory when it supplied the source set.
    pub default_directory: Option<String>,
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

/// Resolve the sources of `settings`, read from `home`: the configured ones,
/// or without `sources` the ones of the default skill directory.
pub fn resolve(home: Option<&Path>, settings: &Settings) -> Result<Resolution, Refusal> {
    resolve_protecting(home, settings, &[])
}

/// Resolve as [`resolve`] does, and also protect `also`. A launch protects the
/// directory of its configuration entry, which `--config` can move away from
/// `$HOME/.config/ravel`.
pub fn resolve_protecting(
    home: Option<&Path>,
    settings: &Settings,
    also: &[&Path],
) -> Result<Resolution, Refusal> {
    let Some(home) = home.filter(|home| home.is_absolute()) else {
        return Ok(Resolution::Disabled);
    };
    let skills = effective(settings);
    // `"sources": []` reads no record and no directory.
    if skills.switched_off() {
        return Ok(Resolution::Disabled);
    }
    let watch = Watch::start(&skills);
    let resolved = source_set(&skills, &watch)
        .and_then(|set| {
            if set.empty() {
                return Ok(None);
            }
            watched(home, &set, also, Limits::current(), &watch).map(Some)
        })
        .and_then(|resolved| {
            watch.enter("the end of the resolution", None);
            watch.check().map(|()| resolved)
        })
        .map_err(|refusal| watch.overrule(refusal))?;
    Ok(resolved.map_or(Resolution::Disabled, Resolution::Resolved))
}

/// The `skills` object of `settings`, or the value of an absent key.
pub(crate) fn effective(settings: &Settings) -> Cow<'_, SkillSettings> {
    settings
        .skills
        .as_ref()
        .map_or_else(|| Cow::Owned(SkillSettings::default()), Cow::Borrowed)
}

/// The source set of `skills`. With `sources`, the array names it exactly.
/// Without `sources`, the default skill directory supplies it, and each
/// mandatory reference must name one of its sources.
fn source_set(skills: &SkillSettings, watch: &Watch) -> Result<SourceSet, Refusal> {
    let (sources, default_directory) = match &skills.sources {
        Some(configured) => {
            let sources = configured
                .iter()
                .enumerate()
                .map(|(index, source)| Source {
                    id: source.id.clone(),
                    path: source.path.clone(),
                    roots: source.roots.clone(),
                    place: Place::Setting(index),
                })
                .collect();
            (sources, None)
        }
        None => {
            watch.enter("the read of the installation record", None);
            watch.check()?;
            match installed::directory()? {
                None => (Vec::new(), None),
                Some(directory) => {
                    watch.enter("the listing of the default skill directory", None);
                    let sources = installed::list(&directory, watch)?
                        .into_iter()
                        .map(|entry| Source {
                            place: Place::Entry(entry.path.clone()),
                            id: entry.id,
                            path: entry.path,
                            roots: entry.roots,
                        })
                        .collect();
                    #[cfg(test)]
                    faults::listed_default(Path::new(&directory));
                    (sources, Some(directory))
                }
            }
        }
    };
    let mut mandatory = Vec::with_capacity(skills.mandatory.len());
    for (position, reference) in skills.mandatory.iter().enumerate() {
        let Some(source) = sources
            .iter()
            .position(|source: &Source| source.id == reference.source)
        else {
            // The parser already refused an unknown ID of configured sources.
            return Err(absent_default(
                position,
                &reference.source,
                default_directory.as_deref(),
            ));
        };
        mandatory.push(Mandatory {
            source,
            path: reference.path.clone(),
        });
    }
    Ok(SourceSet {
        sources,
        mandatory,
        default_directory,
    })
}

/// The refusal of a mandatory reference to a source that the default skill
/// directory does not hold.
fn absent_default(position: usize, id: &str, directory: Option<&str>) -> Refusal {
    let (holder, source) = match directory {
        Some(directory) => (
            format!("the default skill directory {directory} does not hold it"),
            format!("{directory}/{id}"),
        ),
        None => (
            "the installation has no installation record, so it has no default skill directory"
                .to_owned(),
            id.to_owned(),
        ),
    };
    refuse(
        "skill_source_missing",
        format!("the mandatory reference skills.mandatory[{position}] names the source {id:?}, but {holder}"),
        "install the package that supplies the source, or name the source in skills.sources",
    )
    .source(source)
    .location(format!("skills.mandatory[{position}].source"))
}

/// The resolution of enabled delivery under `watch`, within `limits`.
fn watched(
    home: &Path,
    set: &SourceSet,
    also: &[&Path],
    limits: Limits,
    watch: &Watch,
) -> Result<Resolved, Refusal> {
    // The bounds count the default sources after the listing.
    bounds::count(
        bounds::SOURCES,
        limits.sources,
        set.sources.len(),
        &format!("the {} sources", set.origin()),
    )
    .map_err(|refusal| refusal.location(set.location()))?;
    bounds::count(
        bounds::SEARCH_DIRECTORIES,
        limits.search_directories,
        set.sources.iter().map(|source| source.roots.len()).sum(),
        &format!("the {} search directories", set.origin()),
    )
    .map_err(|refusal| refusal.location(set.location()))?;
    bounds::count(
        bounds::MANDATORY_FILES,
        limits.mandatory_files,
        set.mandatory.len(),
        "the configured mandatory files",
    )
    .map_err(|refusal| refusal.location("skills.mandatory"))?;
    let protected = Protected::new(home, also);
    let origins = set
        .sources
        .iter()
        .map(|source| origin(source, &protected))
        .collect::<Result<Vec<_>, _>>()?;
    watch.enter("the opening of the cache", None);
    watch.check()?;
    let cache = Cache::open(home)?;
    utf8(&cache.path)?;

    let mut budget = Budget {
        limits,
        total_bytes: 0,
    };
    let mut records = Vec::with_capacity(origins.len());
    // The retained file of each mandatory reference so far, by its snapshot
    // and its real path in the tree, with the position of that reference.
    let mut reached = BTreeMap::new();
    for (index, (source, origin)) in set.sources.iter().zip(&origins).enumerate() {
        let located = Located {
            id: source.id.clone(),
            location: source.place.location(""),
        };
        // A link can lead a reference to its file, so capture also knows the
        // real path of each one. An unreadable target then reports as
        // mandatory guidance.
        let mandatory: BTreeSet<String> = set
            .mandatory
            .iter()
            .filter(|reference| reference.source == index)
            .flat_map(|reference| {
                let real = real_inside(&origin.canonical, &reference.path);
                [Some(reference.path.clone()), real].into_iter().flatten()
            })
            .collect();
        watch.enter("capture", Some(&located));
        let mut capture = snapshot::capture(
            &origin.canonical,
            &cache,
            &located,
            &mandatory,
            watch,
            &mut budget,
        )?;
        watch.enter(
            "the check of the roots and the mandatory files",
            Some(&located),
        );
        let checked = watch
            .check()
            .and_then(|()| check_roots(&capture.tree(), source, origin, &located))
            .and_then(|()| {
                set.mandatory
                    .iter()
                    .enumerate()
                    .filter(|(_, reference)| reference.source == index)
                    .try_for_each(|(position, reference)| {
                        check_mandatory(
                            &capture.tree(),
                            &reference.path,
                            origin,
                            &located,
                            position,
                            limits.mandatory_file_bytes,
                        )?;
                        check_distinct(
                            &mut reached,
                            &capture.digest,
                            &capture.tree(),
                            &reference.path,
                            origin,
                            position,
                        )
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
        watch.enter("the publication of the snapshot", Some(&located));
        if let Err(error) = snapshot::write_new(&staging, MANIFEST_FILE, &manifest) {
            let refusal = cache::unavailable(staging.path(), &error);
            return Err(snapshot::after_cleanup(refusal, &mut staging));
        }
        let (object, _) = snapshot::publish(
            staging,
            &cache,
            &cache.snapshots,
            &digest,
            &Expected::Snapshot {
                manifest: &manifest,
            },
            watch,
        )?;
        records.push(Record {
            id: source.id.clone(),
            digest,
            root: utf8(&object.join(TREE_DIR))?,
            origin: origin.text.clone(),
        });
    }

    watch.enter("the publication of the descriptor", None);
    watch.check()?;
    let descriptor = descriptor(set, &records);
    let bytes = serde_json::to_vec(&descriptor).expect("a JSON value encodes");
    bounded(
        bounds::DESCRIPTOR_BYTES,
        limits.descriptor_bytes,
        bytes.len(),
        "the descriptor",
    )?;
    let digest = snapshot::hex(&Sha256::digest(&bytes));
    // The preamble names the path that publication gives the descriptor, so
    // both bounds refuse before anything is published.
    let descriptor_path = utf8(&cache.descriptors.path().join(&digest).join(DESCRIPTOR_FILE))?;
    let instructions = preamble::render(&descriptor, &descriptor_path);
    bounded(
        bounds::PREAMBLE_BYTES,
        limits.preamble_bytes,
        instructions.len(),
        "the preamble",
    )?;
    let mut staging = snapshot::Staging::new(&cache, "descriptor-")?;
    if let Err(error) = snapshot::write_new(&staging, DESCRIPTOR_FILE, &bytes) {
        let refusal = cache::unavailable(staging.path(), &error);
        return Err(snapshot::after_cleanup(refusal, &mut staging));
    }
    let (directory, _) = snapshot::publish(
        staging,
        &cache,
        &cache.descriptors,
        &digest,
        &Expected::Descriptor {
            name: DESCRIPTOR_FILE,
            bytes: &bytes,
        },
        watch,
    )?;
    // The reported paths must still name the held objects.
    cache.check()?;
    debug_assert_eq!(utf8(&directory.join(DESCRIPTOR_FILE))?, descriptor_path);
    Ok(Resolved {
        descriptor_path,
        descriptor,
        instructions,
        default_directory: set.default_directory.clone(),
    })
}

/// Refuse the bytes of the descriptor or the preamble over their bound.
fn bounded(subject: bounds::Subject, bound: u64, bytes: usize, what: &str) -> Result<(), Refusal> {
    let bytes = bytes as u64;
    if bytes > bound {
        return Err(bounds::exceeded(subject, bound, Actual::Exact(bytes), what));
    }
    Ok(())
}

/// The protected paths: the configuration directory, the settings file, the
/// cache and the inherited result channel, each also in its canonical form
/// when it exists.
struct Protected {
    paths: Vec<PathBuf>,
}

impl Protected {
    fn new(home: &Path, also: &[&Path]) -> Self {
        let mut paths = Vec::new();
        // The cache is a writable overlap: a source that holds it would
        // capture its own staging. The cache can still be absent below an
        // aliased or dangling segment, so protect the real path that opening
        // it would reach.
        let cache = cache::projected(&home.join(CACHE_ROOT));
        for path in [
            home.join(CONFIG_DIR),
            home.join(SETTINGS_FILE),
            home.join(CACHE_ROOT),
            cache,
        ]
        .into_iter()
        .chain(also.iter().map(|path| path.to_path_buf()))
        {
            if let Ok(canonical) = path.canonicalize() {
                paths.push(canonical);
            }
            paths.push(path);
        }
        // A source that holds the result channel of an enclosing session
        // could capture what that channel holds.
        if let Some(channel) = inherited_channel() {
            paths.extend(prospective(&channel).ok());
            paths.push(channel);
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

/// The result channel of an enclosing session, from an absolute non-empty
/// `RAVEL_EXIT_FILE`. Source acquisition keeps clear of it
/// (docs/specs/skill-discovery.md, "Read access and delegation").
fn inherited_channel() -> Option<PathBuf> {
    std::env::var_os(EXIT_FILE_VARIABLE)
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

/// Canonicalize every existing component, retaining a normalized absent tail.
/// A channel can be absent before its session ends, and symlinked ancestors
/// and `..` still refer to their actual filesystem locations. A dangling
/// symlink is an error rather than a missing ordinary component.
fn prospective(path: &Path) -> io::Result<PathBuf> {
    let mut resolved = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir => resolved.push("/"),
            Component::CurDir => {}
            Component::ParentDir => {
                resolved.pop();
            }
            Component::Normal(name) => {
                resolved.push(name);
                match fs::symlink_metadata(&resolved) {
                    Ok(_) => resolved = resolved.canonicalize()?,
                    Err(error) if error.kind() == ErrorKind::NotFound => {}
                    Err(error) => return Err(error),
                }
            }
            Component::Prefix(_) => {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "expected a POSIX path",
                ));
            }
        }
    }
    Ok(resolved)
}

struct LocalOrigin {
    canonical: PathBuf,
    text: String,
}

fn origin(source: &Source, protected: &Protected) -> Result<LocalOrigin, Refusal> {
    let path = &source.path;
    let at = source.place.location("");
    let remedy = match source.place {
        Place::Setting(_) => "restore the source directory, or change the settings deliberately",
        // A package manager can remove the entry after the listing, during
        // an upgrade (docs/specs/skill-discovery.md, "Changes under running
        // sessions").
        Place::Entry(_) => {
            "run the command again after the package manager has finished, or reinstall the \
             package that installed the source"
        }
    };
    let missing = |what: String| {
        refuse(
            "skill_source_missing",
            format!("the source {:?} at {path} {what}", source.id),
            remedy,
        )
        .source(path.clone())
        .location(source.place.location(".path"))
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
            "move the skill source outside ~/.config/ravel, the directory of the selected \
             configuration entry, the skill cache and the inherited RAVEL_EXIT_FILE channel, and \
             keep the settings file out of every source directory",
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
    source: &Source,
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
        .location(source.place.location(&format!(".roots[{index}]"))));
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
    bound: u64,
) -> Result<(), Refusal> {
    let path = origin.canonical.join(relative);
    let staged = tree.join(relative);
    let at = format!("skills.mandatory[{position}]");
    // A link inside the source can reach the file, so its size counts here
    // as well as at the copy.
    if let Ok(metadata) = fs::metadata(&staged)
        && metadata.is_file()
        && metadata.len() > bound
    {
        return Err(bounds::exceeded(
            bounds::MANDATORY_FILE_BYTES,
            bound,
            Actual::Exact(metadata.len()),
            &format!("the mandatory file {}", path.display()),
        )
        .source(path.display().to_string())
        .location(at));
    }
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
        .location(at)),
    }
}

/// The real path of `relative` inside `root`, as text with `/` separators.
/// None when it is absent, outside `root` or not exact UTF-8.
fn real_inside(root: &Path, relative: &str) -> Option<String> {
    let root = root.canonicalize().ok()?;
    let real = root.join(relative).canonicalize().ok()?;
    let inside = real.strip_prefix(&root).ok()?;
    let text = inside.to_str()?;
    Some(if text.is_empty() {
        ".".to_owned()
    } else {
        text.to_owned()
    })
}

/// Two mandatory references must not reach one retained file: the same file
/// through two sources with one path, through an admitted link, or through
/// two sources with identical contents, which share one snapshot.
fn check_distinct(
    reached: &mut BTreeMap<(String, String), usize>,
    digest: &str,
    tree: &Path,
    relative: &str,
    origin: &LocalOrigin,
    position: usize,
) -> Result<(), Refusal> {
    let real = real_inside(tree, relative).unwrap_or_else(|| relative.to_owned());
    let Some(&first) = reached.get(&(digest.to_owned(), real.clone())) else {
        reached.insert((digest.to_owned(), real), position);
        return Ok(());
    };
    let path = origin.canonical.join(relative);
    Err(refuse(
        "skill_mandatory_invalid",
        format!(
            "the mandatory references skills.mandatory[{first}] and skills.mandatory[{position}] \
             reach the same retained file {real:?}, so the agent would read it twice"
        ),
        "remove one of the two mandatory references, or point it at a different file",
    )
    .source(path.display().to_string())
    .location(format!("skills.mandatory[{position}]")))
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
fn descriptor(set: &SourceSet, records: &[Record]) -> Value {
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
    for (source, record) in set.sources.iter().zip(records) {
        for root in &source.roots {
            let path = joined(&record.root, root);
            if seen.insert(path.clone()) {
                directories.push(json!({"source": record.id, "path": path}));
            }
        }
    }
    let mandatory: Vec<Value> = set
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

/// Internal fault seams for the unit tests: each hook runs in the thread that
/// installed it, at one named point of a resolution.
#[cfg(test)]
pub(crate) mod faults {
    use std::cell::RefCell;
    use std::io;
    use std::path::Path;

    /// A hook that observes a path.
    pub type PathHook = Box<dyn FnMut(&Path)>;
    /// A hook whose error stands for a failed operation.
    pub type FailHook = Box<dyn FnMut(&Path) -> io::Result<()>>;

    /// One installed hook for each point.
    #[derive(Default)]
    pub struct Hooks {
        /// After capture copies the bytes of a file, before its second look.
        pub copied: Option<PathHook>,
        /// After capture lists and copies a directory, before its second look.
        pub listed: Option<PathHook>,
        /// Before each check of the held cache root, with the cache path.
        pub publishing: Option<PathHook>,
        /// After the check of a staging name, just before its rename, with
        /// the staging path.
        pub renaming: Option<PathHook>,
        /// In place of the rename of a staging name, with the staging path.
        /// An error stands for a failed rename, which then does not happen.
        pub renamed: Option<FailHook>,
        /// Before the removal of a staging tree, with its path. An error
        /// stands for a failed removal.
        pub removing: Option<FailHook>,
        /// Smaller bounds, so that a test can cross one that no fixture can
        /// reach.
        pub limits: Option<super::bounds::Limits>,
        /// The signal that the watch of this thread sees as consumed.
        pub signal: Option<crate::cancellation::Signal>,
        /// The installation prefix, in place of the one two levels above the
        /// test executable.
        pub prefix: Option<std::path::PathBuf>,
        /// After the listing of the default skill directory, before capture,
        /// with the directory path.
        pub listed_default: Option<PathHook>,
    }

    thread_local! { pub static HOOKS: RefCell<Hooks> = RefCell::new(Hooks::default()); }

    pub fn install(hooks: Hooks) {
        HOOKS.with(|cell| *cell.borrow_mut() = hooks);
    }

    /// Take a hook out while it runs, so that it can resolve again.
    fn fire<T: ?Sized, R>(
        slot: fn(&mut Hooks) -> &mut Option<Box<T>>,
        call: impl FnOnce(&mut Box<T>) -> R,
    ) -> Option<R> {
        let taken = HOOKS.with(|cell| slot(&mut cell.borrow_mut()).take());
        let mut hook = taken?;
        let result = call(&mut hook);
        HOOKS.with(|cell| {
            let mut hooks = cell.borrow_mut();
            let place = slot(&mut hooks);
            if place.is_none() {
                *place = Some(hook);
            }
        });
        Some(result)
    }

    pub fn copied(path: &Path) {
        fire(|hooks| &mut hooks.copied, |hook| hook(path));
    }

    pub fn listed(path: &Path) {
        fire(|hooks| &mut hooks.listed, |hook| hook(path));
    }

    pub fn publishing(path: &Path) {
        fire(|hooks| &mut hooks.publishing, |hook| hook(path));
    }

    pub fn renaming(path: &Path) {
        fire(|hooks| &mut hooks.renaming, |hook| hook(path));
    }

    pub fn renamed(path: &Path) -> io::Result<()> {
        fire(|hooks| &mut hooks.renamed, |hook| hook(path)).unwrap_or(Ok(()))
    }

    pub fn limits() -> Option<super::bounds::Limits> {
        HOOKS.with(|cell| cell.borrow().limits)
    }

    pub fn signal() -> Option<crate::cancellation::Signal> {
        HOOKS.with(|cell| cell.borrow().signal)
    }

    pub fn prefix() -> Option<std::path::PathBuf> {
        HOOKS.with(|cell| cell.borrow().prefix.clone())
    }

    pub fn listed_default(path: &Path) {
        fire(|hooks| &mut hooks.listed_default, |hook| hook(path));
    }

    pub fn removing(path: &Path) -> io::Result<()> {
        fire(|hooks| &mut hooks.removing, |hook| hook(path)).unwrap_or(Ok(()))
    }
}
