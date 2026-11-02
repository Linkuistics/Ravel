//! The `skills` key of the owner settings file, checked whole before any
//! acquisition. The skill discovery specification owns each rule here
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
    pub sources: Vec<SourceSetting>,
    pub mandatory: Vec<MandatorySetting>,
    pub resolution_timeout_ms: u64,
    /// Whether the settings file sets `resolutionTimeoutMs`.
    pub resolution_timeout_set: bool,
}

impl SkillSettings {
    /// Delivery is enabled when either array holds a record.
    pub fn enabled(&self) -> bool {
        !self.sources.is_empty() || !self.mandatory.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSetting {
    pub id: String,
    pub origin: Origin,
    /// Normalised search directories, relative to the source directory.
    pub roots: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// An absolute directory path, exactly as configured.
    Local { path: String },
    Git {
        url: String,
        revision: String,
        subdir: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MandatorySetting {
    /// The index of the configured source in `sources`.
    pub source: usize,
    /// A normalised file path, relative to that source.
    pub path: String,
}

/// A malformed value: where it is, and what is wrong with it.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub location: String,
    pub what: String,
}

fn problem(location: impl Into<String>, what: impl Into<String>) -> Problem {
    Problem {
        location: location.into(),
        what: what.into(),
    }
}

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

    let records = array(required(fields, at, "sources")?, "skills.sources")?;
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

    let records = array(required(fields, at, "mandatory")?, "skills.mandatory")?;
    if !records.is_empty() && sources.is_empty() {
        return Err(problem(
            "skills.mandatory",
            "names mandatory files, but no source is configured",
        ));
    }
    let mut mandatory: Vec<MandatorySetting> = Vec::with_capacity(records.len());
    for (index, record) in records.iter().enumerate() {
        let at = format!("skills.mandatory[{index}]");
        let fields = object(record, &at)?;
        only(fields, &at, &["source", "path"])?;
        let id = string(required(fields, &at, "source")?, &format!("{at}.source"))?;
        let source = sources
            .iter()
            .position(|source| source.id == id)
            .ok_or_else(|| {
                problem(
                    format!("{at}.source"),
                    format!("names {id:?}, which is not a configured source ID"),
                )
            })?;
        let path_at = format!("{at}.path");
        let path = relative(required(fields, &at, "path")?, &path_at)?;
        if path == "." {
            return Err(problem(path_at, "is \".\", which is not a file"));
        }
        if mandatory
            .iter()
            .any(|known| known.source == source && known.path == path)
        {
            return Err(problem(
                at,
                format!("repeats the reference {id:?} {path:?}"),
            ));
        }
        mandatory.push(MandatorySetting { source, path });
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
    let kind = string(required(fields, at, "type")?, &format!("{at}.type"))?;
    let origin_fields: &[&str] = match kind {
        "local" => &["path"],
        "git" => &["url", "revision", "subdir"],
        _ => {
            return Err(problem(
                format!("{at}.type"),
                format!("is {kind:?}, not \"local\" or \"git\""),
            ));
        }
    };
    let mut allowed = vec!["id", "type", "roots"];
    allowed.extend_from_slice(origin_fields);
    only(fields, at, &allowed)?;

    let id_at = format!("{at}.id");
    let id = string(required(fields, at, "id")?, &id_at)?;
    if !source_id(id) {
        return Err(problem(
            id_at,
            format!(
                "is {id:?}, not 1 to 64 lowercase ASCII letters and digits in groups that \
                 single hyphens separate, starting with a letter"
            ),
        ));
    }

    let origin = if kind == "local" {
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
        Origin::Local {
            path: path.to_owned(),
        }
    } else {
        let url_at = format!("{at}.url");
        let url = string(required(fields, at, "url")?, &url_at)?;
        if !git_url(url) {
            return Err(problem(
                url_at,
                format!(
                    "is {url:?}, not an HTTPS, SSH or file:// URL without credentials, or the \
                     absolute path of a local repository"
                ),
            ));
        }
        let revision_at = format!("{at}.revision");
        let revision = string(required(fields, at, "revision")?, &revision_at)?;
        if !matches!(revision.len(), 40 | 64)
            || !revision
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(problem(
                revision_at,
                format!(
                    "is {revision:?}, not a full lowercase commit object ID of 40 or 64 \
                     hexadecimal digits; a ref, a tag or an abbreviated ID refuses"
                ),
            ));
        }
        let subdir = relative(required(fields, at, "subdir")?, &format!("{at}.subdir"))?;
        Origin::Git {
            url: url.to_owned(),
            revision: revision.to_owned(),
            subdir,
        }
    };

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
        origin,
        roots,
    })
}

fn source_id(id: &str) -> bool {
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

/// An HTTPS, SSH or `file://` URL with no password, an scp-style SSH locator,
/// or an absolute path. The pinned stage acquires them; this stage only
/// checks their shape.
fn git_url(url: &str) -> bool {
    if url.is_empty() || has_control(url) || url.contains(char::is_whitespace) {
        return false;
    }
    if url.starts_with('/') {
        return true;
    }
    for scheme in ["https://", "ssh://", "file://"] {
        if let Some(rest) = url.strip_prefix(scheme) {
            let authority = rest.split('/').next().unwrap_or_default();
            let credentials = authority.rsplit_once('@').map(|(user, _)| user);
            // A user name is ordinary for SSH. A password is a credential.
            let password = credentials.is_some_and(|user| user.contains(':'));
            let https_user = scheme == "https://" && credentials.is_some();
            return !rest.is_empty() && !password && !https_user;
        }
    }
    // `user@host:path`, where no slash comes before the first colon.
    !url.contains("://")
        && url
            .split_once(':')
            .is_some_and(|(host, path)| !host.is_empty() && !host.contains('/') && !path.is_empty())
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
        let git = json!({
            "id": "research", "type": "git", "url": "https://example.com/r.git",
            "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": "packs/r",
            "roots": ["kinds", "craft"]
        });
        let value = json!({
            "schemaVersion": 1, "resolutionTimeoutMs": 1000,
            "sources": [local("core", json!([".", "skills"])), git],
            "mandatory": [{"source": "research", "path": "kinds/a/SKILL.md"},
                          {"source": "core", "path": "spine.md"}]
        });
        let settings = parse(&value).unwrap();
        assert!(settings.enabled());
        assert_eq!(settings.resolution_timeout_ms, 1000);
        assert_eq!(settings.sources[0].roots, [".", "skills"]);
        assert_eq!(
            settings.mandatory,
            [
                MandatorySetting {
                    source: 1,
                    path: "kinds/a/SKILL.md".into()
                },
                MandatorySetting {
                    source: 0,
                    path: "spine.md".into()
                },
            ]
        );
        assert!(matches!(settings.sources[1].origin, Origin::Git { .. }));
    }

    #[test]
    fn empty_arrays_disable_delivery_and_the_deadline_defaults() {
        let settings = parse(&skills(json!([]), json!([]))).unwrap();
        assert!(!settings.enabled());
        assert_eq!(settings.resolution_timeout_ms, RESOLUTION_DEFAULT_MS);
    }

    #[test]
    fn each_malformed_value_refuses_at_its_location() {
        let ok = local("a", json!(["."]));
        for (value, location) in [
            (json!([]), "skills"),
            (
                json!({"schemaVersion": 1, "sources": []}),
                "skills.mandatory",
            ),
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
                    json!([{"id": "a", "type": "svn", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].type",
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
                skills(
                    json!([{"id": "a", "type": "git", "url": "/r", "revision": "HEAD", "subdir": ".", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].revision",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "/r", "revision": "0123456789ABCDEF0123456789ABCDEF01234567", "subdir": ".", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].revision",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "/r", "revision": "0123456", "subdir": ".", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].revision",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "http://h/r", "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": ".", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].url",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "https://u:p@h/r", "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": ".", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].url",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "/r", "revision": "0123456789abcdef0123456789abcdef01234567", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].subdir",
            ),
            (
                skills(
                    json!([{"id": "a", "type": "git", "url": "/r", "revision": "0123456789abcdef0123456789abcdef01234567", "subdir": "/x", "roots": ["."]}]),
                    json!([]),
                ),
                "skills.sources[0].subdir",
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
    fn git_locators_take_the_documented_shapes() {
        for url in [
            "https://example.com/r.git",
            "ssh://git@example.com/r.git",
            "git@example.com:org/r.git",
            "file:///srv/r.git",
            "/srv/r.git",
        ] {
            assert!(git_url(url), "{url}");
        }
        for url in [
            "",
            "r.git",
            "http://h/r",
            "git://h/r",
            "https://",
            "https://u@h/r",
            "ssh://u:p@h/r",
            "h :r",
            "./r",
        ] {
            assert!(!git_url(url), "{url}");
        }
    }
}
