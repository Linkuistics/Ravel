//! The schema subset of structured output and the check of an answer against it
//! (`docs/specs/policy-evaluation.md`, *LLM calls*).
//!
//! A [`Schema`] keeps the JSON that the script wrote, because Rig rewrites the
//! schema on each wire and the adapter sends it as written. It also keeps a
//! tree of the keywords that Ravel checks. The answer is checked against that
//! tree, never against the rewritten schema of Rig.
//!
//! The subset has `type`, `properties`, `required`, `additionalProperties`,
//! `items`, `enum`, `title` and `description`. A schema checks shape only. The
//! keywords apply to the kinds of value they describe: `required` and
//! `properties` say nothing about an array, and `items` says nothing about an
//! object.

use serde_json::{Map, Value};

use crate::rhai_worker::convert;

/// The deepest nesting of schemas: the root is level 1, and each `properties`
/// member or `items` schema is one level more.
pub const MAX_LEVELS: usize = 32;

/// The longest key that a message shows. A key of an answer is the endpoint's
/// text, so a message cuts it.
const SHOWN_KEY: usize = 64;

/// A schema the subset rejects, with where it is and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invalid {
    /// The location in the style of the converter, such as `schema.properties.a`.
    pub path: String,
    pub reason: String,
}

/// A schema of the subset.
#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    raw: Value,
    root: Node,
}

#[derive(Clone, Debug, PartialEq)]
struct Node {
    kinds: Option<Vec<Kind>>,
    properties: Vec<(String, Node)>,
    required: Vec<String>,
    additional: Option<bool>,
    items: Option<Box<Node>>,
    choices: Option<Vec<Value>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Object,
    Array,
    String,
    Number,
    Integer,
    Boolean,
    Null,
}

const KINDS: [(&str, Kind); 7] = [
    ("object", Kind::Object),
    ("array", Kind::Array),
    ("string", Kind::String),
    ("number", Kind::Number),
    ("integer", Kind::Integer),
    ("boolean", Kind::Boolean),
    ("null", Kind::Null),
];

impl Kind {
    fn name(self) -> &'static str {
        KINDS
            .iter()
            .find(|(_, kind)| *kind == self)
            .map_or("value", |(name, _)| name)
    }

    fn admits(self, value: &Value) -> bool {
        match (self, value) {
            (Kind::Object, Value::Object(_))
            | (Kind::Array, Value::Array(_))
            | (Kind::String, Value::String(_))
            | (Kind::Number, Value::Number(_))
            | (Kind::Boolean, Value::Bool(_))
            | (Kind::Null, Value::Null) => true,
            (Kind::Integer, Value::Number(number)) => {
                number.is_i64()
                    || number.is_u64()
                    || number
                        .as_f64()
                        .is_some_and(|float| float.is_finite() && float.fract() == 0.0)
            }
            _ => false,
        }
    }
}

/// What kind of value `value` is, for a message.
fn found(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(number) if number.is_i64() || number.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

impl Schema {
    /// Check `value` against the subset. `root` names the schema in a location.
    pub fn parse(value: &Value, root: &str) -> Result<Schema, Invalid> {
        let node = Node::parse(value, root, 1)?;
        Ok(Schema {
            raw: value.clone(),
            root: node,
        })
    }

    /// The schema as the `output_schema` of a Rig request.
    pub fn to_rig(&self) -> rig_core::schemars::Schema {
        rig_core::schemars::Schema::try_from(self.raw.clone())
            .expect("a checked schema is a JSON object")
    }

    /// Parse the whole answer, trimmed, as JSON and check it against the
    /// schema. The error says where the answer departs, in the style of the
    /// converter, such as `json.items[2].name`.
    pub fn check(&self, answer: &str) -> Result<Value, String> {
        let value: Value = serde_json::from_str(answer.trim())
            .map_err(|error| format!("the answer is not JSON: {error}"))?;
        // The reply to the worker nests the value three levels down, and the
        // worker reads no frame past the depth of its converter.
        convert::check_depth(&value, "json").map_err(|refused| {
            format!(
                "the answer nests deeper than {} levels, at {}",
                convert::MAX_DEPTH,
                refused.path
            )
        })?;
        let mut path = String::from("json");
        match self.root.check(&value, &mut path) {
            Ok(()) => Ok(value),
            Err(why) => Err(format!("the answer does not match the schema: {why}")),
        }
    }
}

impl Node {
    fn parse(value: &Value, path: &str, level: usize) -> Result<Node, Invalid> {
        let invalid = |at: &str, reason: &str| Invalid {
            path: at.to_owned(),
            reason: reason.to_owned(),
        };
        let Some(map) = value.as_object() else {
            return Err(invalid(path, "it must be a map"));
        };
        if level > MAX_LEVELS {
            return Err(invalid(
                path,
                &format!("the schema nests deeper than {MAX_LEVELS} levels"),
            ));
        }
        let mut node = Node {
            kinds: None,
            properties: Vec::new(),
            required: Vec::new(),
            additional: None,
            items: None,
            choices: None,
        };
        for (keyword, given) in map {
            let at = format!("{path}.{keyword}");
            match keyword.as_str() {
                "type" => node.kinds = Some(parse_kinds(given, &at)?),
                "properties" => node.properties = parse_properties(given, &at, level)?,
                "required" => node.required = parse_names(given, &at)?,
                "additionalProperties" => {
                    node.additional = Some(
                        given
                            .as_bool()
                            .ok_or_else(|| invalid(&at, "it must be a boolean"))?,
                    );
                }
                "items" => node.items = Some(Box::new(Node::parse(given, &at, level + 1)?)),
                "enum" => node.choices = Some(parse_choices(given, &at)?),
                "title" | "description" => {
                    if !given.is_string() {
                        return Err(invalid(&at, "it must be a string"));
                    }
                }
                _ => {
                    return Err(invalid(
                        &at,
                        "it is not a keyword of the schema subset: use type, properties, \
                         required, additionalProperties, items, enum, title or description",
                    ));
                }
            }
        }
        Ok(node)
    }

    /// Check `value`, whose location is `path`, and restore `path` before it
    /// returns.
    fn check(&self, value: &Value, path: &mut String) -> Result<(), String> {
        let here = |what: String| format!("{path}: {what}");
        if let Some(kinds) = &self.kinds
            && !kinds.iter().any(|kind| kind.admits(value))
        {
            let names: Vec<_> = kinds.iter().map(|kind| kind.name()).collect();
            return Err(here(format!(
                "expected {}, found {}",
                names.join(" or "),
                found(value)
            )));
        }
        if let Some(choices) = &self.choices
            && !choices.iter().any(|choice| same(choice, value))
        {
            return Err(here(format!(
                "it is none of the {} values that enum allows",
                choices.len()
            )));
        }
        match value {
            Value::Object(fields) => self.check_object(fields, path),
            Value::Array(elements) => {
                let Some(items) = &self.items else {
                    return Ok(());
                };
                for (index, element) in elements.iter().enumerate() {
                    let mark = path.len();
                    path.push_str(&format!("[{index}]"));
                    items.check(element, path)?;
                    path.truncate(mark);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_object(&self, fields: &Map<String, Value>, path: &mut String) -> Result<(), String> {
        if let Some(missing) = self
            .required
            .iter()
            .find(|name| !fields.contains_key(*name))
        {
            return Err(format!(
                "{path}: the required property {} is missing",
                shown(missing)
            ));
        }
        for (name, node) in &self.properties {
            if let Some(field) = fields.get(name) {
                let mark = path.len();
                path.push_str(&format!(".{}", shown(name)));
                node.check(field, path)?;
                path.truncate(mark);
            }
        }
        if self.additional == Some(false)
            && let Some(extra) = fields
                .keys()
                .find(|key| !self.properties.iter().any(|(name, _)| name == *key))
        {
            return Err(format!(
                "{path}.{}: the schema allows no additional property",
                shown(extra)
            ));
        }
        Ok(())
    }
}

/// `name`, cut at [`SHOWN_KEY`] characters.
fn shown(name: &str) -> String {
    if name.chars().count() <= SHOWN_KEY {
        name.to_owned()
    } else {
        let cut: String = name.chars().take(SHOWN_KEY).collect();
        format!("{cut}…")
    }
}

/// Whether two `enum` values are equal. A number equals another when their
/// values do, so `1` equals `1.0`.
fn same(allowed: &Value, answer: &Value) -> bool {
    match (allowed, answer) {
        (Value::Number(left), Value::Number(right)) => {
            match (left.as_i64(), right.as_i64(), left.as_u64(), right.as_u64()) {
                (Some(left), Some(right), _, _) => left == right,
                (_, _, Some(left), Some(right)) => left == right,
                _ => left.as_f64() == right.as_f64(),
            }
        }
        _ => allowed == answer,
    }
}

fn parse_kinds(given: &Value, at: &str) -> Result<Vec<Kind>, Invalid> {
    let invalid = |reason: &str| Invalid {
        path: at.to_owned(),
        reason: reason.to_owned(),
    };
    let one = |value: &Value, at: &str| {
        value
            .as_str()
            .and_then(|word| KINDS.iter().find(|(name, _)| *name == word))
            .map(|(_, kind)| *kind)
            .ok_or_else(|| Invalid {
                path: at.to_owned(),
                reason: "it must be object, array, string, number, integer, boolean or null"
                    .to_owned(),
            })
    };
    match given {
        Value::Array(words) if !words.is_empty() => words
            .iter()
            .enumerate()
            .map(|(index, word)| one(word, &format!("{at}[{index}]")))
            .collect(),
        Value::Array(_) => Err(invalid("it must name at least one type")),
        word => Ok(vec![one(word, at)?]),
    }
}

fn parse_properties(given: &Value, at: &str, level: usize) -> Result<Vec<(String, Node)>, Invalid> {
    let Some(map) = given.as_object() else {
        return Err(Invalid {
            path: at.to_owned(),
            reason: "it must be a map from a name to a schema".to_owned(),
        });
    };
    map.iter()
        .map(|(name, schema)| {
            Node::parse(schema, &format!("{at}.{name}"), level + 1).map(|node| (name.clone(), node))
        })
        .collect()
}

fn parse_names(given: &Value, at: &str) -> Result<Vec<String>, Invalid> {
    let Some(names) = given.as_array() else {
        return Err(Invalid {
            path: at.to_owned(),
            reason: "it must be an array of property names".to_owned(),
        });
    };
    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            name.as_str().map(str::to_owned).ok_or_else(|| Invalid {
                path: format!("{at}[{index}]"),
                reason: "it must be a string".to_owned(),
            })
        })
        .collect()
}

fn parse_choices(given: &Value, at: &str) -> Result<Vec<Value>, Invalid> {
    let Some(choices) = given.as_array() else {
        return Err(Invalid {
            path: at.to_owned(),
            reason: "it must be an array".to_owned(),
        });
    };
    for (index, choice) in choices.iter().enumerate() {
        if choice.is_array() || choice.is_object() {
            return Err(Invalid {
                path: format!("{at}[{index}]"),
                reason: "it must be a string, number, boolean or null".to_owned(),
            });
        }
    }
    Ok(choices.clone())
}
