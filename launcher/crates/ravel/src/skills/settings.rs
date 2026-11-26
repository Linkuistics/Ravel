//! The `skills` key of the owner settings file, checked whole before any
//! capture. The skill discovery specification owns each rule here
//! (docs/specs/skill-discovery.md#settings).

use std::collections::BTreeSet;

use serde_json::{Map, Value};

/// The default bound of one complete resolution.
pub const RESOLUTION_DEFAULT_MS: u64 = 120_000;
pub const RESOLUTION_MIN_MS: u64 = 1_000;
pub const RESOLUTION_MAX_MS: u64 = 3_600_000;

/// The checked `skills` object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkillSettings {
    /// The configured sources, or `None` when the settings name no
    /// `sources`. The default skill directory then supplies the source set.
    pub sources: Option<Vec<SourceSetting>>,
    pub mandatory: Vec<MandatorySetting>,
    pub resolution_timeout_ms: u64,
    /// Whether the settings file sets `resolutionTimeoutMs`.
    pub resolution_timeout_set: bool,
}

/// The value of an absent `skills` key: no `sources`, so the default skill
/// directory supplies the source set, and no mandatory file.
impl Default for SkillSettings {
    fn default() -> Self {
        SkillSettings {
            sources: None,
            mandatory: Vec::new(),
            resolution_timeout_ms: RESOLUTION_DEFAULT_MS,
            resolution_timeout_set: false,
        }
    }
}

impl SkillSettings {
    /// Whether delivery is disabled before any directory is read:
    /// `"sources": []` with no mandatory reference.
    pub fn switched_off(&self) -> bool {
        self.sources.as_ref().is_some_and(Vec::is_empty) && self.mandatory.is_empty()
    }
}

/// A `local` source, the only source type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSetting {
    pub id: String,
    /// An absolute directory path, exactly as configured.
    pub path: String,
    /// Normalised search directories, relative to the source directory.
    pub roots: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MandatorySetting {
    /// The ID of a source in the source set. With `sources`, the parser
    /// checked that it names a configured source. Without `sources`, the
    /// resolution looks for it in the default skill directory.
    pub source: String,
    /// A normalised file path, relative to that source.
    pub path: String,
}

/// A malformed value: where it is, what is wrong with it and, where the
/// generic settings remedy does not cover it, its own remedy.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub location: String,
    pub what: String,
    pub remedy: Option<&'static str>,
}

fn problem(location: impl Into<String>, what: impl Into<String>) -> Problem {
    Problem {
        location: location.into(),
        what: what.into(),
        remedy: None,
    }
}

/// The remedy of a source `type` other than `local`. It covers a misspelt
/// value and a former Git record alike.
pub const TYPE_REMEDY: &str = "set type to \"local\", the only source type; make any checkout \
     of a repository with a tool outside Ravel, and name its directory in a local record with \
     path";

/// Check the value of the `skills` key.
pub fn parse(value: &Value) -> Result<SkillSettings, Problem> {
    let at = "skills";
    let fields = object(value, at)?;
    only(
        fields,
        at,
        &[
            "schemaVersion",
            "sources",
            "mandatory",
            "resolutionTimeoutMs",
        ],
    )?;
    let version = required(fields, at, "schemaVersion")?;
    if version.as_u64() != Some(1) {
        return Err(problem(
            "skills.schemaVersion",
            format!("is {version}, not the integer 1"),
        ));
    }
    let resolution_timeout_ms = match fields.get("resolutionTimeoutMs") {
        None => RESOLUTION_DEFAULT_MS,
        Some(value) => value
            .as_u64()
            .filter(|ms| (RESOLUTION_MIN_MS..=RESOLUTION_MAX_MS).contains(ms))
            .ok_or_else(|| {
                problem(
                    "skills.resolutionTimeoutMs",
                    format!(
                        "is {value}, not a whole number of milliseconds from \
                         {RESOLUTION_MIN_MS} to {RESOLUTION_MAX_MS}"
                    ),
                )
            })?,
    };

    let sources = match fields.get("sources") {
        None => None,
        Some(value) => {
            let records = array(value, "skills.sources")?;
            let mut sources = Vec::with_capacity(records.len());
            for (index, record) in records.iter().enumerate() {
                let source = source(record, &format!("skills.sources[{index}]"))?;
                if sources
                    .iter()
                    .any(|known: &SourceSetting| known.id == source.id)
                {
                    return Err(problem(
                        format!("skills.sources[{index}].id"),
                        format!("repeats the source ID {:?}", source.id),
                    ));
                }
                sources.push(source);
            }
            Some(sources)
        }
    };

    let records = match fields.get("mandatory") {
        None => &[][..],
        Some(value) => array(value, "skills.mandatory")?.as_slice(),
    };
    if !records.is_empty() && sources.as_ref().is_some_and(Vec::is_empty) {
        return Err(problem(
            "skills.mandatory",
            "names mandatory files, but sources is empty",
        ));
    }
    let mut mandatory: Vec<MandatorySetting> = Vec::with_capacity(records.len());
    for (index, record) in records.iter().enumerate() {
        let at = format!("skills.mandatory[{index}]");
        let fields = object(record, &at)?;
        only(fields, &at, &["source", "path"])?;
        let source_at = format!("{at}.source");
        let id = string(required(fields, &at, "source")?, &source_at)?;
        if !source_id(id) {
            return Err(problem(source_at, format!("is {id:?}, {ID_GRAMMAR}")));
        }
        if let Some(sources) = &sources
            && !sources.iter().any(|source| source.id == id)
        {
            return Err(problem(
                source_at,
                format!("names {id:?}, which is not a configured source ID"),
            ));
        }
        let path_at = format!("{at}.path");
        let path = relative(required(fields, &at, "path")?, &path_at)?;
        if path == "." {
            return Err(problem(path_at, "is \".\", which is not a file"));
        }
        if mandatory
            .iter()
            .any(|known| known.source == id && known.path == path)
        {
            return Err(problem(
                at,
                format!("repeats the reference {id:?} {path:?}"),
            ));
        }
        mandatory.push(MandatorySetting {
            source: id.to_owned(),
            path,
        });
    }

    Ok(SkillSettings {
        sources,
        mandatory,
        resolution_timeout_ms,
        resolution_timeout_set: fields.contains_key("resolutionTimeoutMs"),
    })
}

fn source(value: &Value, at: &str) -> Result<SourceSetting, Problem> {
    let fields = object(value, at)?;
    // `type` comes first: no other field of a record with another type is
    // checked, so a former Git record refuses here and nowhere else.
    let type_at = format!("{at}.type");
    match fields.get("type") {
        Some(Value::String(kind)) if kind == "local" => {}
        found => {
            let what = match found {
                None => "is missing".to_owned(),
                Some(value) => format!("is {value}, not \"local\""),
            };
            return Err(Problem {
                remedy: Some(TYPE_REMEDY),
                ..problem(type_at, what)
            });
        }
    }
    only(fields, at, &["id", "type", "path", "roots"])?;

    let id_at = format!("{at}.id");
    let id = string(required(fields, at, "id")?, &id_at)?;
    if !source_id(id) {
        return Err(problem(id_at, format!("is {id:?}, {ID_GRAMMAR}")));
    }

    let path_at = format!("{at}.path");
    let path = string(required(fields, at, "path")?, &path_at)?;
    if !path.starts_with('/') || has_control(path) {
        return Err(problem(
            path_at,
            format!(
                "is {path:?}, not an absolute directory path; Ravel expands no tilde and no \
                 environment variable"
            ),
        ));
    }

    let roots_at = format!("{at}.roots");
    let values = array(required(fields, at, "roots")?, &roots_at)?;
    if values.is_empty() {
        return Err(problem(
            roots_at,
            "is empty; name at least one search directory",
        ));
    }
    let mut roots: Vec<String> = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        let root_at = format!("{roots_at}[{index}]");
        let root = relative(value, &root_at)?;
        if roots.contains(&root) {
            return Err(problem(root_at, format!("repeats the root {root:?}")));
        }
        roots.push(root);
    }

    Ok(SourceSetting {
        id: id.to_owned(),
        path: path.to_owned(),
        roots,
    })
}

/// What a value outside the source-ID grammar fails to be.
pub const ID_GRAMMAR: &str = "not 1 to 64 lowercase ASCII letters and digits in groups that \
     single hyphens separate, starting with a letter";

/// Whether `id` has the source-ID grammar. A default source takes its ID from
/// its entry name, so the listing applies the same grammar.
pub fn source_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.starts_with(|first: char| first.is_ascii_lowercase())
        && id.split('-').all(|group| {
            !group.is_empty()
                && group
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

/// A normalised relative path: `.`, or segments with no empty segment, no
/// `.` or `..`, no backslash and no control character.
fn relative(value: &Value, at: &str) -> Result<String, Problem> {
    let path = string(value, at)?;
    let normalised = path == "."
        || (!path.is_empty()
            && !has_control(path)
            && !path.contains('\\')
            && path
                .split('/')
                .all(|segment| !matches!(segment, "" | "." | "..")));
    if normalised {
        Ok(path.to_owned())
    } else {
        Err(problem(
            at,
            format!(
                "is {path:?}, not a normalised relative path: use \".\" or segments without an \
                 absolute start, an empty segment, \".\", \"..\", a backslash or a trailing \
                 separator"
            ),
        ))
    }
}

fn has_control(text: &str) -> bool {
    text.chars().any(char::is_control)
}

fn object<'a>(value: &'a Value, at: &str) -> Result<&'a Map<String, Value>, Problem> {
    value
        .as_object()
        .ok_or_else(|| problem(at, format!("is {value}, not an object")))
}

fn array<'a>(value: &'a Value, at: &str) -> Result<&'a Vec<Value>, Problem> {
    value
        .as_array()
        .ok_or_else(|| problem(at, format!("is {value}, not an array")))
}

fn string<'a>(value: &'a Value, at: &str) -> Result<&'a str, Problem> {
    value
        .as_str()
        .ok_or_else(|| problem(at, format!("is {value}, not a string")))
}

fn required<'a>(fields: &'a Map<String, Value>, at: &str, key: &str) -> Result<&'a Value, Problem> {
    fields
        .get(key)
        .ok_or_else(|| problem(format!("{at}.{key}"), "is missing"))
}

fn only(fields: &Map<String, Value>, at: &str, allowed: &[&str]) -> Result<(), Problem> {
    let known: BTreeSet<&str> = allowed.iter().copied().collect();
    match fields.keys().find(|key| !known.contains(key.as_str())) {
        None => Ok(()),
        Some(key) => Err(problem(
            format!("{at}.{key}"),
            format!(
                "is an unknown field; the fields here are {}",
                allowed.join(", ")
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn local(id: &str, roots: Value) -> Value {
        json!({"id": id, "type": "local", "path": "/packs/a", "roots": roots})
    }

    fn skills(sources: Value, mandatory: Value) -> Value {
        json!({"schemaVersion": 1, "sources": sources, "mandatory": mandatory})
    }

    #[test]
    fn a_complete_object_parses_in_configured_order() {
        let research = json!({
            "id": "research", "type": "local", "path": "/packs/r", "roots": ["kinds", "craft"]
        });
        let value = json!({
            "schemaVersion": 1, "resolutionTimeoutMs": 1000,
            "sources": [local("core", json!([".", "skills"])), research],
            "mandatory": [{"source": "research", "path": "kinds/a/SKILL.md"},
                          {"source": "core", "path": "spine.md"}]
        });
        let settings = parse(&value).unwrap();
        assert!(!settings.switched_off());
        assert_eq!(settings.resolution_timeout_ms, 1000);
        let sources = settings.sources.as_ref().unwrap();
        assert_eq!(sources[0].roots, [".", "skills"]);
        assert_eq!(
            settings.mandatory,
            [
                MandatorySetting {
                    source: "research".into(),
                    path: "kinds/a/SKILL.md".into()
                },
                MandatorySetting {
                    source: "core".into(),
                    path: "spine.md".into()
                },
            ]
        );
        assert_eq!(sources[1].path, "/packs/r");
    }

    #[test]
    fn empty_arrays_disable_delivery_and_the_deadline_defaults() {
        let settings = parse(&skills(json!([]), json!([]))).unwrap();
        assert!(settings.switched_off());
        assert_eq!(settings.resolution_timeout_ms, RESOLUTION_DEFAULT_MS);
        let settings = parse(&json!({"schemaVersion": 1, "sources": []})).unwrap();
        assert!(settings.switched_off());
        assert_eq!(settings.mandatory, []);
    }

    #[test]
    fn without_sources_the_default_directory_supplies_the_source_set() {
        let settings = parse(&json!({"schemaVersion": 1})).unwrap();
        assert_eq!(settings, SkillSettings::default());
        assert_eq!(settings.sources, None);
        assert!(!settings.switched_off());
        // A mandatory reference keeps its ID for the listing to find.
        let settings = parse(&json!({
            "schemaVersion": 1,
            "mandatory": [{"source": "grove-core", "path": "spine/SKILL.md"}]
        }))
        .unwrap();
        assert_eq!(settings.sources, None);
        assert_eq!(
            settings.mandatory,
            [MandatorySetting {
                source: "grove-core".into(),
                path: "spine/SKILL.md".into()
            }]
        );
        assert!(!settings.switched_off());
    }

    #[test]
    fn without_sources_a_mandatory_reference_still_refuses_when_malformed() {
        for (value, location) in [
            (
                json!({"schemaVersion": 1, "mandatory": [{"source": "Core", "path": "x"}]}),
                "skills.mandatory[0].source",
            ),
            (
                json!({"schemaVersion": 1, "mandatory": [{"source": "a", "path": "x"},
                                                         {"source": "a", "path": "x"}]}),
                "skills.mandatory[1]",
            ),
            (
                json!({"schemaVersion": 1, "mandatory": [{"source": "a", "path": "../x"}]}),
                "skills.mandatory[0].path",
            ),
            (
                json!({"schemaVersion": 1, "mandatory": {}}),
                "skills.mandatory",
            ),
        ] {
            let found = parse(&value).unwrap_err();
            assert_eq!(found.location, location, "{value}: {}", found.what);
        }
    }

    #[test]
    fn each_malformed_value_refuses_at_its_location() {
        let ok = local("a", json!(["."]));
        for (value, location) in [
            (json!([]), "skills"),
            (
                json!({"schemaVersion": 2, "sources": [], "mandatory": []}),
                "skills.schemaVersion",
            ),
            (
                json!({"schemaVersion": 1.0, "sources": [], "mandatory": []}),
                "skills.schemaVersion",
            ),
            (
                json!({"schemaVersion": 1, "sources": [], "mandatory": [], "x": 1}),
                "skills.x",
            ),
            (
                json!({"schemaVersion": 1, "sources": [], "mandatory": [], "resolutionTimeoutMs": 999}),
                "skills.resolutionTimeoutMs",
            ),
            (
                json!({"schemaVersion": 1, "sources": [], "mandatory": [], "resolutionTimeoutMs": 3_600_001}),
                "skills.resolutionTimeoutMs",
            ),
            (skills(json!({}), json!([])), "skills.sources"),
            (
                skills(json!([ok, local("b", json!(["a/"]))]), json!([])),
                "skills.sources[1].roots[0]",
            ),
            (skills(json!([ok, ok]), json!([])), "skills.sources[1].id"),
            (
                skills(json!([local("A", json!(["."]))]), json!([])),
                "skills.sources[0].id",
            ),
            (
                skills(json!([local("a--b", json!(["."]))]), json!([])),
                "skills.sources[0].id",
            ),
            (
                skills(json!([local("1a", json!(["."]))]), json!([])),
                "skills.sources[0].id",
            ),
            (
                skills(json!([local(&"a".repeat(65), json!(["."]))]), json!([])),
                "skills.sources[0].id",
            ),
            (
                skills(json!([local("a", json!([]))]), json!([])),
                "skills.sources[0].roots",
            ),
            (
                skills(json!([local("a", json!([".", "."]))]), json!([])),
                "skills.sources[0].roots[1]",
            ),
            (
                skills(json!([local("a", json!(["../x"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!(["/x"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!(["a//b"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!(["a/./b"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!(["a\\b"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!([""]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(json!([local("a", json!(["a\nb"]))]), json!([])),
                "skills.sources[0].roots[0]",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "local", "path": "~/p", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].path",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "local", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].path",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "local", "path": "/p", "url": "/r", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].url",
            ),
            (
                skills(json!([]), json!([{"source": "a", "path": "x"}])),
                "skills.mandatory",
            ),
            (
                skills(json!([ok]), json!([{"source": "b", "path": "x"}])),
                "skills.mandatory[0].source",
            ),
            (
                skills(json!([ok]), json!([{"source": "a", "path": "."}])),
                "skills.mandatory[0].path",
            ),
            (
                skills(json!([ok]), json!([{"source": "a", "path": "../x"}])),
                "skills.mandatory[0].path",
            ),
            (
                skills(json!([ok]), json!([{"source": "a"}])),
                "skills.mandatory[0].path",
            ),
            (
                skills(json!([ok]), json!([{"source": "a", "path": "x", "why": 1}])),
                "skills.mandatory[0].why",
            ),
            (
                skills(
                    json!([ok]),
                    json!([{"source": "a", "path": "x"}, {"source": "a", "path": "x"}]),
                ),
                "skills.mandatory[1]",
            ),
        ] {
            let found = parse(&value).unwrap_err();
            assert_eq!(found.location, location, "{value}: {}", found.what);
        }
    }

    #[test]
    fn a_type_other_than_local_refuses_first_with_the_checkout_remedy() {
        let ok = local("a", json!(["."]));
        let git = json!({
            "id": "research", "type": "git", "url": "https://example.com/r.git",
            "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": ".",
            "roots": ["."]
        });
        for record in [
            git,
            json!({"id": "a", "path": "/p", "roots": ["."]}),
            json!({"id": "a", "type": "Local", "path": "/p", "roots": ["."]}),
            json!({"id": "a", "type": "svn", "path": "/p", "roots": ["."]}),
            json!({"id": "a", "type": 1, "path": "/p", "roots": ["."]}),
            json!({"id": "a", "type": null, "path": "/p", "roots": ["."]}),
            // No other field of the record is checked: neither the bad ID,
            // nor the unknown field, nor the missing roots.
            json!({"id": "A", "type": "git", "x": 1}),
        ] {
            let found = parse(&skills(json!([ok, record]), json!([]))).unwrap_err();
            assert_eq!(found.location, "skills.sources[1].type", "{record}");
            assert_eq!(found.remedy, Some(TYPE_REMEDY), "{record}");
        }
        let found = parse(&skills(json!([ok, {"type": "git"}]), json!([]))).unwrap_err();
        assert_eq!(found.what, "is \"git\", not \"local\"");
        for text in ["\"local\"", "checkout", "outside Ravel", "path"] {
            assert!(TYPE_REMEDY.contains(text), "{text}");
        }
    }

    #[test]
    fn only_a_type_problem_carries_its_own_remedy() {
        for value in [
            skills(json!([local("A", json!(["."]))]), json!([])),
            skills(
                json!([{"id": "a", "type": "local", "url": "/r", "path": "/p", "roots": ["."]}]),
                json!([]),
            ),
            skills(
                json!([{"id": "a", "type": "local", "roots": ["."]}]),
                json!([]),
            ),
        ] {
            assert_eq!(parse(&value).unwrap_err().remedy, None, "{value}");
        }
    }
}
