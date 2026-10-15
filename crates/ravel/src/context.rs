//! Validate caller/loader data and attach measured file sources within the context budget.

use std::fmt::Write as _;
use std::fs;
use std::io::Read as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;

use serde_json::{json, Map, Value};
use sha2::{Digest as _, Sha256};

use crate::limits::{Bound, Limits, Origin, CONTEXT_MAX_BYTES, SOURCE_DEFAULT_BYTES};
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};

/// A version-1 context's fields; the delivered context adds `measured`.
const FIELDS: [&str; 6] = [
    "schemaVersion",
    "summary",
    "acceptanceCriteria",
    "facts",
    "assessments",
    "sources",
];

/// Names that would make a field executable if a context could have one. Each
/// is refused as unknown, with a message saying that a context is data.
const EXECUTABLE: [&str; 11] = [
    "program",
    "args",
    "argv",
    "command",
    "select",
    "loadContext",
    "env",
    "environment",
    "exec",
    "shell",
    "script",
];

const SOURCE_FIELDS: [&str; 4] = ["name", "sha256", "bytes", "version"];
const ASSESSMENT_FIELDS: [&str; 2] = ["by", "value"];

/// How the `--context` document is named as a measured source.
pub const VIA_CALLER: &str = "--context";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Measured {
    /// The canonical path.
    pub name: String,
    /// `--context`, `readText` or `readJson`.
    pub via: String,
    pub bytes: u64,
    pub sha256: String,
}

impl Measured {
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "via": self.via,
            "bytes": self.bytes,
            "sha256": self.sha256,
        })
    }
}

/// The caller's `--context` document, read and validated.
#[derive(Debug)]
pub struct CallerContext {
    pub value: Value,
    pub measured: Measured,
}

/// Read the `--context` document once, within the context budget, and check
/// it against the version-1 shape. The path resolves against the caller's cwd
/// and must name a regular file, which is opened without blocking, so that a
/// FIFO cannot hold the invocation before its policy even starts.
pub fn read_caller(given: &Path, cwd: &Path, limits: &Limits) -> Result<CallerContext, Refusal> {
    let joined = cwd.join(given);
    let shown = joined.to_string_lossy().into_owned();
    let unreadable = |what: String| {
        Refusal::new(
            "context_unreadable",
            Stage::Context,
            EXIT_REFUSED,
            format!("the --context document {shown} {what}"),
            "name a readable version-1 context document with --context, relative to the current \
             directory, or omit --context",
        )
        .input("--context")
        .source(shown.clone())
    };
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(&joined)
        .map_err(|error| unreadable(format!("cannot be opened: {error}")))?;
    let metadata = file
        .metadata()
        .map_err(|error| unreadable(format!("cannot be examined: {error}")))?;
    if !metadata.is_file() {
        return Err(unreadable("is not a regular file".to_owned()));
    }
    let name = fs::canonicalize(&joined)
        .map_err(|error| unreadable(format!("cannot be resolved: {error}")))?
        .into_os_string()
        .into_string()
        .map_err(|_| {
            unreadable(
                "resolves to a path that is not valid UTF-8, which no context can name".to_owned(),
            )
        })?;
    let budget = limits.context.value;
    let mut bytes = Vec::new();
    file.take(budget + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| unreadable(format!("cannot be read: {error}")))?;
    if bytes.len() as u64 > budget {
        let size = metadata.len().max(bytes.len() as u64);
        return Err(Refusal::new(
            "context_too_large",
            Stage::Context,
            EXIT_REFUSED,
            format!(
                "the --context document {name} is {size} bytes, over the context bound of \
                 {budget} bytes"
            ),
            format!(
                "shorten the document, or raise the bound with contextBytes in the owner \
                 settings or --context-bytes, up to {CONTEXT_MAX_BYTES}; ravel never \
                 truncates a context to fit"
            ),
        )
        .input("--context")
        .source(name)
        .bound(limits.context));
    }
    let shape = Shape::caller(&name);
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|error| shape.invalid("context", format!("the document is not JSON: {error}")))?;
    shape.validate(&value, limits)?;
    let measured = Measured {
        via: VIA_CALLER.to_owned(),
        bytes: bytes.len() as u64,
        sha256: hex(&Sha256::digest(&bytes)),
        name,
    };
    Ok(CallerContext { value, measured })
}

#[derive(Debug)]
pub struct Delivered {
    pub value: Value,
    pub sources: Vec<Measured>,
    pub loader: bool,
    /// The bytes actually read from every measured source.
    pub source_bytes: u64,
    /// The length of the delivered value's compact JSON encoding, with object
    /// keys in sorted order.
    pub encoded_bytes: u64,
    /// The SHA-256 of that encoding.
    pub sha256: String,
}

impl Delivered {
    pub fn to_json(&self, value: bool) -> Value {
        let mut report = Map::new();
        report.insert("loader".into(), self.loader.into());
        report.insert(
            "sources".into(),
            self.sources.iter().map(Measured::to_json).collect(),
        );
        report.insert("sourceBytes".into(), self.source_bytes.into());
        report.insert("encodedBytes".into(), self.encoded_bytes.into());
        report.insert("sha256".into(), self.sha256.clone().into());
        if value {
            report.insert("value".into(), self.value.clone());
        }
        Value::Object(report)
    }
}

/// Where a context came from, for its refusals: the caller's document, or the
/// policy's `loadContext`.
#[derive(Clone, Copy)]
enum Author {
    Caller,
    Loader,
}

pub fn deliver(
    context: Value,
    sources: Vec<Measured>,
    loader: bool,
    limits: &Limits,
    source: &str,
) -> Result<Delivered, Refusal> {
    let shape = if loader {
        Shape::loader(source)
    } else {
        Shape::caller(source)
    };
    if context.is_null() {
        return Err(shape.invalid(
            "context",
            "loadContext returned no context (undefined or null)".to_owned(),
        ));
    }
    shape.validate(&context, limits)?;
    if sources.len() as u64 > limits.sources.value {
        return Err(too_many_sources(limits, sources.len() as u64, source));
    }
    let mut value = context
        .as_object()
        .cloned()
        .expect("validation accepted an object");
    value.insert(
        "measured".into(),
        sources.iter().map(Measured::to_json).collect(),
    );
    let value = Value::Object(value);
    let encoded = serde_json::to_vec(&value).expect("a JSON value always encodes");
    let encoded_bytes = encoded.len() as u64;
    if encoded_bytes > limits.context.value {
        return Err(context_too_large(limits, encoded_bytes, source, loader));
    }
    Ok(Delivered {
        source_bytes: sources.iter().map(|source| source.bytes).sum(),
        sha256: hex(&Sha256::digest(&encoded)),
        encoded_bytes,
        loader,
        sources,
        value,
    })
}

/// The delivered context encodes to more than the budget: its source records
/// count, and nothing is ever cut to fit.
pub fn context_too_large(limits: &Limits, actual: u64, source: &str, loader: bool) -> Refusal {
    let budget = limits.context.value;
    let deliver_less = if loader {
        format!("have loadContext in {source} deliver less")
    } else {
        "shorten the --context document".to_owned()
    };
    let refusal = Refusal::new(
        "context_too_large",
        Stage::Context,
        EXIT_REFUSED,
        format!(
            "the context delivered to selection encodes to {actual} bytes, its source records \
             included, over the context bound of {budget} bytes"
        ),
        format!(
            "{deliver_less}, or raise the bound with contextBytes in the owner \
             settings or --context-bytes, up to {CONTEXT_MAX_BYTES}; \
             ravel never truncates a context to fit"
        ),
    )
    .source(source)
    .bound(limits.context);
    match limits.context.set_by() {
        Some(flag) => refusal.input(flag),
        None => refusal,
    }
}

/// More sources than the fixed bound: measured ones, or records in a
/// context's `sources`.
pub fn too_many_sources(limits: &Limits, count: u64, source: &str) -> Refusal {
    Refusal::new(
        "too_many_sources",
        Stage::Context,
        EXIT_REFUSED,
        format!(
            "the context has {count} measured sources, over the fixed bound of {} (the \
             --context document counts as one)",
            limits.sources.value
        ),
        "read fewer sources, or combine them before reading; nothing is dropped to fit",
    )
    .source(source)
    .bound(limits.sources)
}

/// What the worker reported about an SDK read over its limit. `limit` is the
/// read's own, `from` the origin of that limit, and `requested` a `maxBytes`
/// above the context budget, which no read may exceed.
pub struct SourceBreach {
    pub name: String,
    pub limit: u64,
    pub from: Origin,
    pub actual: Option<u64>,
    pub requested: Option<u64>,
}

pub fn source_too_large(limits: &Limits, breach: &SourceBreach, entry: &str) -> Refusal {
    let budget = limits.context.value;
    let name = &breach.name;
    let (message, remedy, bound) = match breach.requested {
        Some(requested) => (
            format!(
                "loadContext in {entry} asked to read {name} with maxBytes {requested}, over the \
                 context budget of {budget} bytes, which no read may exceed"
            ),
            format!(
                "pass a maxBytes of at most {budget}, or raise the budget with contextBytes in \
                 the owner settings or --context-bytes, up to {CONTEXT_MAX_BYTES}"
            ),
            Bound {
                name: "source",
                ..limits.context
            },
        ),
        None => (
            format!(
                "loadContext in {entry} read {name}, which holds more than its limit of {} bytes{}{}",
                breach.limit,
                breach
                    .actual
                    .map(|actual| format!(" ({actual} bytes or more)"))
                    .unwrap_or_default(),
                if breach.limit < budget {
                    ""
                } else {
                    ", the whole context budget"
                }
            ),
            // A read that already takes the whole budget can ask for no more,
            // so only the budget, or a smaller source, can meet it.
            if breach.limit < budget {
                format!(
                    "read it with a larger maxBytes, up to the context budget of {budget} bytes, \
                     or read a smaller source; a read is never truncated"
                )
            } else if budget == CONTEXT_MAX_BYTES {
                format!(
                    "read a smaller source: the read already takes the whole context budget, at \
                     its ceiling of {CONTEXT_MAX_BYTES} bytes, and a read is never truncated"
                )
            } else if breach.from == Origin::Set("maxBytes") {
                format!(
                    "raise the context budget with contextBytes in the owner \
                     settings or --context-bytes, up to {CONTEXT_MAX_BYTES}: a \
                     read whose maxBytes is request.limits.contextBytes takes the new budget, and \
                     one with a fixed maxBytes needs it raised too; or read a smaller source, \
                     since a read is never truncated"
                )
            } else {
                format!(
                    "raise the context budget with contextBytes in the owner \
                     settings or --context-bytes, up to {CONTEXT_MAX_BYTES}: a \
                     read without maxBytes takes the budget, up to {SOURCE_DEFAULT_BYTES} bytes; \
                     or read a smaller source, since a read is never truncated"
                )
            },
            Bound {
                name: "source",
                unit: "bytes",
                value: breach.limit,
                origin: breach.from,
            },
        ),
    };
    Refusal::new(
        "source_too_large",
        Stage::Context,
        EXIT_REFUSED,
        message,
        remedy,
    )
    .source(name.clone())
    .location("host.readText")
    .bound(bound)
}

/// Validates one context against the version-1 shape, naming the file a
/// refusal is about.
struct Shape<'a> {
    source: &'a str,
    author: Author,
}

impl<'a> Shape<'a> {
    fn caller(source: &'a str) -> Self {
        Shape {
            source,
            author: Author::Caller,
        }
    }

    fn loader(source: &'a str) -> Self {
        Shape {
            source,
            author: Author::Loader,
        }
    }

    fn invalid(&self, location: &str, message: String) -> Refusal {
        let (message, remedy) = match self.author {
            Author::Caller => (
                message,
                format!(
                    "correct {location} in the --context document {}; the types in \
                     ravel/sdk describe a version-1 context",
                    self.source
                ),
            ),
            Author::Loader => (
                format!("the context loadContext returned is invalid: {message}"),
                format!(
                    "correct what loadContext in {} returns at {location}; the types in \
                     ravel/sdk describe a version-1 context",
                    self.source
                ),
            ),
        };
        let refusal = Refusal::new(
            "context_invalid",
            Stage::Context,
            EXIT_REFUSED,
            message,
            remedy,
        )
        .source(self.source)
        .location(location);
        match self.author {
            Author::Caller => refusal.input("--context"),
            Author::Loader => refusal,
        }
    }

    fn validate(&self, value: &Value, limits: &Limits) -> Result<(), Refusal> {
        let context = self.object(value, "context")?;

        // The version comes first: a later schema's fields are not unknown to it.
        let schema = context.get("schemaVersion").ok_or_else(|| {
            self.invalid("context.schemaVersion", "`schemaVersion` is missing".into())
        })?;
        match schema.as_u64() {
            Some(1) => {}
            Some(other) => {
                let refusal = Refusal::new(
                    "unsupported_version",
                    Stage::Context,
                    EXIT_REFUSED,
                    format!(
                        "context schemaVersion {other} is not supported; this release reads \
                         schemaVersion 1"
                    ),
                    "write a version-1 context, or upgrade ravel",
                )
                .source(self.source)
                .location("context.schemaVersion");
                return Err(match self.author {
                    Author::Caller => refusal.input("--context"),
                    Author::Loader => refusal,
                });
            }
            None => {
                return Err(self.invalid(
                    "context.schemaVersion",
                    format!(
                        "`schemaVersion` must be the number 1, found {}",
                        self.describe(schema)
                    ),
                ))
            }
        }
        if let Some(unknown) = context.keys().find(|key| !FIELDS.contains(&key.as_str())) {
            let message = if EXECUTABLE.contains(&unknown.as_str()) {
                format!(
                    "`{unknown}` would be an executable field, and a context is data: it supplies \
                     no program, argument, environment or policy"
                )
            } else if unknown == "measured" && matches!(self.author, Author::Loader) {
                "`measured` is what ravel measured, and it attaches that itself; \
                 loadContext cannot supply it"
                    .to_owned()
            } else {
                format!("unknown field `{unknown}`")
            };
            return Err(self.invalid(&format!("context.{unknown}"), message));
        }

        if let Some(summary) = context.get("summary") {
            self.string(summary, "context.summary", false)?;
        }
        if let Some(criteria) = context.get("acceptanceCriteria") {
            for (index, criterion) in self
                .array(criteria, "context.acceptanceCriteria")?
                .iter()
                .enumerate()
            {
                self.string(
                    criterion,
                    &format!("context.acceptanceCriteria[{index}]"),
                    false,
                )?;
            }
        }
        if let Some(facts) = context.get("facts") {
            for (key, fact) in self.object(facts, "context.facts")? {
                self.data(fact, &format!("context.facts[{}]", quoted(key)))?;
            }
        }
        if let Some(assessments) = context.get("assessments") {
            for (key, assessment) in self.object(assessments, "context.assessments")? {
                self.assessment(assessment, &format!("context.assessments[{}]", quoted(key)))?;
            }
        }
        if let Some(sources) = context.get("sources") {
            let sources = self.array(sources, "context.sources")?;
            if sources.len() as u64 > limits.sources.value {
                return Err(Refusal::new(
                    "too_many_sources",
                    Stage::Context,
                    EXIT_REFUSED,
                    format!(
                        "the context's `sources` has {} records, over the fixed bound of {}",
                        sources.len(),
                        limits.sources.value
                    ),
                    "attribute fewer sources, or combine them; nothing is dropped to fit",
                )
                .source(self.source)
                .location("context.sources")
                .bound(limits.sources));
            }
            for (index, record) in sources.iter().enumerate() {
                self.source_record(record, &format!("context.sources[{index}]"))?;
            }
        }
        Ok(())
    }

    fn assessment(&self, value: &Value, at: &str) -> Result<(), Refusal> {
        let fields = self.object(value, at)?;
        self.known(
            fields,
            &ASSESSMENT_FIELDS,
            at,
            "an assessment is { by, value }",
        )?;
        self.string(self.field(fields, "by", at)?, &format!("{at}.by"), true)?;
        self.data(self.field(fields, "value", at)?, &format!("{at}.value"))
    }

    /// A source record names its evidence and identifies it by digest or
    /// version, so that a context never claims evidence it cannot pin.
    fn source_record(&self, value: &Value, at: &str) -> Result<(), Refusal> {
        let fields = self.object(value, at)?;
        self.known(
            fields,
            &SOURCE_FIELDS,
            at,
            "a source record is { name, sha256?, bytes?, version? }",
        )?;
        self.string(self.field(fields, "name", at)?, &format!("{at}.name"), true)?;
        if let Some(sha256) = fields.get("sha256") {
            let location = format!("{at}.sha256");
            let digest = self.string(sha256, &location, false)?;
            if digest.len() != 64
                || !digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(self.invalid(
                    &location,
                    "`sha256` must be 64 lowercase hexadecimal digits".to_owned(),
                ));
            }
        }
        if let Some(bytes) = fields.get("bytes") {
            if bytes.as_u64().is_none() {
                return Err(self.invalid(
                    &format!("{at}.bytes"),
                    format!(
                        "`bytes` must be a whole number of bytes, found {}",
                        self.describe(bytes)
                    ),
                ));
            }
        }
        if let Some(version) = fields.get("version") {
            self.string(version, &format!("{at}.version"), true)?;
        }
        if !fields.contains_key("sha256") && !fields.contains_key("version") {
            return Err(self.invalid(
                at,
                "a source record identifies its evidence with `sha256`, `version` or both, and \
                 this one has neither"
                    .to_owned(),
            ));
        }
        Ok(())
    }

    /// Any JSON value, as data, except the worker's marker for a value JSON
    /// cannot carry, anywhere inside it.
    fn data(&self, value: &Value, at: &str) -> Result<(), Refusal> {
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    self.data(item, &format!("{at}[{index}]"))?;
                }
                Ok(())
            }
            Value::Object(_) if is_marker(value) => Err(self.reserved(value, at)),
            Value::Object(fields) => {
                for (key, field) in fields {
                    self.data(field, &format!("{at}[{}]", quoted(key)))?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn reserved(&self, marker: &Value, at: &str) -> Refusal {
        let message = match self.author {
            Author::Caller => "an object whose only key is `$ravel` is reserved: the \
                               worker uses it to mark a value that JSON cannot carry"
                .to_owned(),
            Author::Loader => format!(
                "{} cannot be carried in JSON, so no context can deliver it",
                self.describe(marker)
            ),
        };
        self.invalid(at, message)
    }

    fn known(
        &self,
        fields: &Map<String, Value>,
        known: &[&str],
        at: &str,
        shape: &str,
    ) -> Result<(), Refusal> {
        match fields.keys().find(|key| !known.contains(&key.as_str())) {
            Some(unknown) => Err(self.invalid(
                &format!("{at}.{unknown}"),
                format!("unknown field `{unknown}`; {shape}"),
            )),
            None => Ok(()),
        }
    }

    fn field<'v>(
        &self,
        fields: &'v Map<String, Value>,
        field: &str,
        at: &str,
    ) -> Result<&'v Value, Refusal> {
        fields
            .get(field)
            .ok_or_else(|| self.invalid(&format!("{at}.{field}"), format!("`{field}` is missing")))
    }

    fn object<'v>(&self, value: &'v Value, at: &str) -> Result<&'v Map<String, Value>, Refusal> {
        match value.as_object() {
            Some(_) if is_marker(value) => Err(self.reserved(value, at)),
            Some(object) => Ok(object),
            None => Err(self.invalid(
                at,
                format!("expected an object, found {}", self.describe(value)),
            )),
        }
    }

    fn array<'v>(&self, value: &'v Value, at: &str) -> Result<&'v Vec<Value>, Refusal> {
        value.as_array().ok_or_else(|| {
            self.invalid(
                at,
                format!("expected an array, found {}", self.describe(value)),
            )
        })
    }

    fn string<'v>(&self, value: &'v Value, at: &str, nonblank: bool) -> Result<&'v str, Refusal> {
        let text = value.as_str().ok_or_else(|| {
            self.invalid(
                at,
                format!("expected a string, found {}", self.describe(value)),
            )
        })?;
        if nonblank && text.trim().is_empty() {
            return Err(self.invalid(at, "must not be blank".to_owned()));
        }
        Ok(text)
    }

    /// A value's type, for "found …" messages. Only a loader's result can hold
    /// a marker the worker wrote; in a caller's document it is an object.
    fn describe(&self, value: &Value) -> String {
        match value {
            Value::Null => "null".to_owned(),
            Value::Bool(_) => "a boolean".to_owned(),
            Value::Number(_) => "a number".to_owned(),
            Value::String(_) => "a string".to_owned(),
            Value::Array(_) => "an array".to_owned(),
            Value::Object(object) => match (self.author, object.get("$ravel")) {
                (Author::Loader, Some(Value::String(kind))) if object.len() == 1 => {
                    format!("a {kind}")
                }
                _ => "an object".to_owned(),
            },
        }
    }
}

fn is_marker(value: &Value) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.len() == 1 && object.contains_key("$ravel"))
}

/// A key as a JSON string, for locations such as `context.facts["risk"]`.
fn quoted(key: &str) -> String {
    Value::String(key.to_owned()).to_string()
}

pub fn hex(bytes: &[u8]) -> String {
    let mut rendered = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Infallible: `String`'s `Write` never errors, and the format is fixed.
        let _ = write!(&mut rendered, "{byte:02x}");
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> Limits {
        Limits::read(None, None, &crate::settings::Settings::default()).unwrap()
    }

    /// The limits under a `--context-bytes` of `bytes`.
    fn budget(bytes: usize) -> Limits {
        let bytes = bytes.to_string();
        let settings = crate::settings::Settings::default();
        Limits::read(None, Some(std::ffi::OsStr::new(&bytes)), &settings).unwrap()
    }

    fn refused(author: Author, value: Value) -> Refusal {
        let shape = Shape {
            source: "/work/context.json",
            author,
        };
        shape.validate(&value, &limits()).unwrap_err()
    }

    #[test]
    fn every_field_of_a_version_1_context_is_accepted_and_nothing_is_required_but_the_version() {
        let shape = Shape::caller("/work/context.json");
        let full = json!({
            "schemaVersion": 1,
            "summary": "Rename the flag",
            "acceptanceCriteria": ["the old name still parses"],
            "facts": { "risk": null, "files": [1, "two", { "program": "data" }] },
            "assessments": { "risk": { "by": "owner", "value": { "level": "high" } } },
            "sources": [
                { "name": "issue 12", "version": "2026-09-30" },
                { "name": "/work/README.md", "sha256": "a".repeat(64), "bytes": 10 },
            ],
        });
        shape.validate(&full, &limits()).unwrap();
        shape
            .validate(&json!({ "schemaVersion": 1 }), &limits())
            .unwrap();
        shape
            .validate(
                &json!({ "schemaVersion": 1, "facts": {}, "acceptanceCriteria": [], "sources": [] }),
                &limits(),
            )
            .unwrap();
    }

    #[test]
    fn each_malformed_shape_is_refused_at_its_location() {
        for (value, location) in [
            (json!([]), "context"),
            (json!({}), "context.schemaVersion"),
            (json!({ "schemaVersion": "1" }), "context.schemaVersion"),
            (json!({ "schemaVersion": 1, "extra": 1 }), "context.extra"),
            (
                json!({ "schemaVersion": 1, "program": "sh" }),
                "context.program",
            ),
            (
                json!({ "schemaVersion": 1, "measured": [] }),
                "context.measured",
            ),
            (
                json!({ "schemaVersion": 1, "summary": 3 }),
                "context.summary",
            ),
            (
                json!({ "schemaVersion": 1, "summary": null }),
                "context.summary",
            ),
            (
                json!({ "schemaVersion": 1, "acceptanceCriteria": ["a", 2] }),
                "context.acceptanceCriteria[1]",
            ),
            (json!({ "schemaVersion": 1, "facts": [] }), "context.facts"),
            (
                json!({ "schemaVersion": 1, "facts": { "a": { "b": [{ "$ravel": "function" }] } } }),
                "context.facts[\"a\"][\"b\"][0]",
            ),
            (
                json!({ "schemaVersion": 1, "assessments": { "risk": "high" } }),
                "context.assessments[\"risk\"]",
            ),
            (
                json!({ "schemaVersion": 1, "assessments": { "risk": { "value": 1 } } }),
                "context.assessments[\"risk\"].by",
            ),
            (
                json!({ "schemaVersion": 1, "assessments": { "risk": { "by": " ", "value": 1 } } }),
                "context.assessments[\"risk\"].by",
            ),
            (
                json!({ "schemaVersion": 1, "assessments": { "risk": { "by": "me" } } }),
                "context.assessments[\"risk\"].value",
            ),
            (
                json!({ "schemaVersion": 1, "assessments": { "risk": { "by": "me", "value": 1, "why": "" } } }),
                "context.assessments[\"risk\"].why",
            ),
            (
                json!({ "schemaVersion": 1, "sources": [{ "name": "x" }] }),
                "context.sources[0]",
            ),
            (
                json!({ "schemaVersion": 1, "sources": [{ "name": "x", "sha256": "ABC" }] }),
                "context.sources[0].sha256",
            ),
            (
                json!({ "schemaVersion": 1, "sources": [{ "name": "x", "version": "1", "bytes": -1 }] }),
                "context.sources[0].bytes",
            ),
            (
                json!({ "schemaVersion": 1, "sources": [{ "version": "1" }] }),
                "context.sources[0].name",
            ),
        ] {
            let refusal = refused(Author::Caller, value.clone());
            assert_eq!(
                refusal.code, "context_invalid",
                "{value}: {}",
                refusal.message
            );
            assert_eq!(
                refusal.location.as_deref(),
                Some(location),
                "{value}: {}",
                refusal.message
            );
            assert_eq!(refusal.input.as_deref(), Some("--context"), "{value}");
        }
    }

    #[test]
    fn a_later_version_is_unsupported_and_an_executable_field_says_why() {
        let refusal = refused(
            Author::Caller,
            json!({ "schemaVersion": 2, "program": "sh" }),
        );
        assert_eq!(refusal.code, "unsupported_version");
        assert_eq!(refusal.location.as_deref(), Some("context.schemaVersion"));

        let refusal = refused(
            Author::Loader,
            json!({ "schemaVersion": 1, "args": ["-c"] }),
        );
        assert!(
            refusal.message.contains("a context is data"),
            "{}",
            refusal.message
        );
        assert_eq!(refusal.input, None);
    }

    #[test]
    fn a_marker_is_named_for_what_it_marks_only_in_a_loaders_result() {
        let marker = json!({ "$ravel": "function" });
        let typed = json!({ "schemaVersion": 1, "summary": marker });
        let loader = refused(Author::Loader, typed.clone());
        assert!(
            loader.message.contains("found a function"),
            "{}",
            loader.message
        );
        let caller = refused(Author::Caller, typed);
        assert!(
            caller.message.contains("found an object"),
            "{}",
            caller.message
        );

        let free = json!({ "schemaVersion": 1, "facts": { "f": marker } });
        let loader = refused(Author::Loader, free.clone());
        assert!(
            loader.message.contains("a function cannot be carried"),
            "{}",
            loader.message
        );
        let caller = refused(Author::Caller, free);
        assert!(caller.message.contains("is reserved"), "{}", caller.message);
    }

    #[test]
    fn more_source_records_than_the_bound_refuse_by_name() {
        let record = json!({ "name": "x", "version": "1" });
        let at_bound = json!({ "schemaVersion": 1, "sources": vec![record.clone(); 256] });
        Shape::caller("c").validate(&at_bound, &limits()).unwrap();
        let over = json!({ "schemaVersion": 1, "sources": vec![record; 257] });
        let refusal = refused(Author::Caller, over);
        assert_eq!(refusal.code, "too_many_sources");
        assert_eq!(refusal.bound.map(|bound| bound.name), Some("sources"));
    }

    #[test]
    fn delivery_attaches_the_measured_sources_and_measures_the_sorted_encoding() {
        let source = Measured {
            name: "/work/context.json".into(),
            via: VIA_CALLER.into(),
            bytes: 30,
            sha256: "b".repeat(64),
        };
        let delivered = deliver(
            json!({ "summary": "s", "schemaVersion": 1 }),
            vec![source],
            false,
            &limits(),
            "/work/context.json",
        )
        .unwrap();
        let encoded = serde_json::to_string(&delivered.value).unwrap();
        assert!(encoded.starts_with(r#"{"measured":[{"#), "{encoded}");
        assert_eq!(delivered.encoded_bytes, encoded.len() as u64);
        assert_eq!(delivered.sha256, hex(&Sha256::digest(encoded.as_bytes())));
        assert_eq!(delivered.source_bytes, 30);

        let exact = budget(encoded.len());
        let context = json!({ "summary": "s", "schemaVersion": 1 });
        let at_bound = deliver(
            context.clone(),
            delivered.sources.clone(),
            false,
            &exact,
            "c",
        );
        assert!(at_bound.is_ok());
        let over = budget(encoded.len() - 1);
        let refusal = deliver(context, delivered.sources, false, &over, "c").unwrap_err();
        assert_eq!(refusal.code, "context_too_large");
        assert_eq!(refusal.input.as_deref(), Some("--context-bytes"));
    }
}
