//! The one converter between Rhai values and JSON (`docs/specs/configuration-evaluation.md`,
//! *Conversion at the boundary*).
//!
//! Every value that crosses between the script and the front is JSON data, and
//! both directions apply the same table. A value the table refuses is named by
//! its location, such as `result.args[2]`.

use rhai::{Array, Dynamic, Map};
use serde_json::{Number, Value};

/// How deep arrays and maps may nest. `serde_json` refuses to read a frame
/// nested past 128 levels, so a deeper value would break the protocol instead
/// of refusing at its location.
pub const MAX_DEPTH: usize = 100;

/// A value the table refuses, with where it is and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    /// The location, such as `result.args[2]`.
    pub path: String,
    pub reason: String,
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.reason)
    }
}

/// The JSON form of `value`, whose location is `root` in a refusal.
pub fn to_json(value: &Dynamic, root: &str) -> Result<Value, Refused> {
    convert(value, &mut Path::new(root))
}

/// The Rhai form of `value`. An integer outside the signed 64-bit range becomes
/// a float.
pub fn from_json(value: &Value) -> Dynamic {
    match value {
        Value::Null => Dynamic::UNIT,
        Value::Bool(flag) => Dynamic::from(*flag),
        Value::Number(number) => match number.as_i64() {
            Some(integer) => Dynamic::from(integer),
            // `as_f64` of a number `serde_json` read cannot be `None`, because
            // the `arbitrary_precision` feature is off.
            None => Dynamic::from(number.as_f64().unwrap_or(f64::NAN)),
        },
        Value::String(text) => Dynamic::from(text.clone()),
        Value::Array(items) => Dynamic::from_array(items.iter().map(from_json).collect()),
        Value::Object(fields) => {
            let mut map = Map::new();
            for (key, value) in fields {
                map.insert(key.as_str().into(), from_json(value));
            }
            Dynamic::from_map(map)
        }
    }
}

/// Refuse a JSON value that nests deeper than [`MAX_DEPTH`], at the location of
/// the first container past it. This is the bound of `to_json` for a value that
/// arrives as JSON, such as a file that `read_json` parsed.
pub fn check_depth(value: &Value, root: &str) -> Result<(), Refused> {
    fn walk(value: &Value, path: &mut Path) -> Result<(), Refused> {
        match value {
            Value::Array(items) => {
                too_deep(path)?;
                for (index, item) in items.iter().enumerate() {
                    path.enter(format_args!("[{index}]"));
                    walk(item, path)?;
                    path.leave();
                }
            }
            Value::Object(fields) => {
                too_deep(path)?;
                for (key, item) in fields {
                    path.enter(format_args!(".{key}"));
                    walk(item, path)?;
                    path.leave();
                }
            }
            _ => {}
        }
        Ok(())
    }
    walk(value, &mut Path::new(root))
}

/// The location of the value being converted, built up as the walk descends.
struct Path {
    text: String,
    marks: Vec<usize>,
}

impl Path {
    fn new(root: &str) -> Self {
        Path {
            text: root.to_owned(),
            marks: Vec::new(),
        }
    }

    fn enter(&mut self, step: std::fmt::Arguments<'_>) {
        self.marks.push(self.text.len());
        std::fmt::Write::write_fmt(&mut self.text, step).expect("a String does not fail to write");
    }

    fn leave(&mut self) {
        if let Some(mark) = self.marks.pop() {
            self.text.truncate(mark);
        }
    }

    fn refuse(&self, reason: String) -> Refused {
        Refused {
            path: self.text.clone(),
            reason,
        }
    }
}

fn convert(value: &Dynamic, path: &mut Path) -> Result<Value, Refused> {
    // The `as_*_ref` readers look through a shared value, so a value that a
    // closure captured converts as its current content.
    if value.is_unit() {
        return Ok(Value::Null);
    }
    if let Ok(flag) = value.as_bool() {
        return Ok(Value::Bool(flag));
    }
    if let Ok(integer) = value.as_int() {
        return Ok(Value::from(integer));
    }
    if let Ok(float) = value.as_float() {
        return Number::from_f64(float)
            .map(Value::Number)
            .ok_or_else(|| path.refuse(format!("the float {float} has no JSON form")));
    }
    if let Ok(text) = value.as_immutable_string_ref() {
        return Ok(Value::String(text.to_string()));
    }
    if let Ok(items) = value.as_array_ref() {
        return array(&items, path);
    }
    if let Ok(fields) = value.as_map_ref() {
        return map(&fields, path);
    }
    Err(path.refuse(format!(
        "a value of type {} has no JSON form",
        value.type_name()
    )))
}

fn array(items: &Array, path: &mut Path) -> Result<Value, Refused> {
    too_deep(path)?;
    let mut converted = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        path.enter(format_args!("[{index}]"));
        converted.push(convert(item, path)?);
        path.leave();
    }
    Ok(Value::Array(converted))
}

fn map(fields: &Map, path: &mut Path) -> Result<Value, Refused> {
    too_deep(path)?;
    let mut converted = serde_json::Map::new();
    for (key, item) in fields {
        path.enter(format_args!(".{key}"));
        converted.insert(key.to_string(), convert(item, path)?);
        path.leave();
    }
    Ok(Value::Object(converted))
}

fn too_deep(path: &Path) -> Result<(), Refused> {
    if path.marks.len() >= MAX_DEPTH {
        return Err(path.refuse(format!("the value nests deeper than {MAX_DEPTH} levels")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use rhai::{Engine, Scope};
    use serde_json::json;

    use super::*;

    fn eval(script: &str) -> Dynamic {
        Engine::new().eval::<Dynamic>(script).unwrap()
    }

    fn refused(script: &str) -> Refused {
        to_json(&eval(script), "result").unwrap_err()
    }

    #[test]
    fn the_table_converts_each_supported_value() {
        let value = eval(
            r#"#{ none: (), yes: true, int: 7, float: 1.5, text: "a", list: [1, [2]], map: #{ k: "v" } }"#,
        );
        assert_eq!(
            to_json(&value, "result"),
            Ok(json!({
                "none": null, "yes": true, "int": 7, "float": 1.5, "text": "a",
                "list": [1, [2]], "map": { "k": "v" },
            }))
        );
    }

    #[test]
    fn json_converts_back_to_the_same_value() {
        let original = json!({
            "none": null, "yes": false, "int": -3, "float": 2.5, "text": "é",
            "list": [1, "x", [null]], "map": { "k": { "n": 1 } },
        });
        assert_eq!(to_json(&from_json(&original), "x"), Ok(original));
    }

    #[test]
    fn an_integer_outside_the_signed_range_becomes_a_float() {
        let big = from_json(&json!(u64::MAX));
        assert!(big.is_float());
        assert_eq!(big.as_float().unwrap(), u64::MAX as f64);
        assert!(from_json(&json!(i64::MIN)).is_int());
    }

    #[test]
    fn a_refused_value_is_named_by_its_location() {
        assert_eq!(
            refused(r#"#{ args: ["a", "b", 'c'] }"#).path,
            "result.args[2]"
        );
        assert_eq!(
            refused(r#"#{ a: #{ b: [[(), || 1]] } }"#).path,
            "result.a.b[0][1]"
        );
        assert_eq!(refused("'c'").path, "result");
    }

    #[test]
    fn each_refused_class_names_its_type() {
        for script in [
            "'c'",
            "blob(2)",
            "|x| x",
            "Fn(\"f\")",
            "timestamp()",
            "1..3",
            "1..=3",
        ] {
            let refusal = refused(script);
            assert!(
                refusal.reason.contains("has no JSON form"),
                "{script}: {refusal}"
            );
        }
        assert!(refused("blob(2)").reason.contains("blob"));
    }

    #[test]
    fn nan_and_the_infinities_are_refused() {
        for script in ["0.0 / 0.0", "1.0 / 0.0", "-1.0 / 0.0"] {
            assert!(refused(script).reason.contains("has no JSON form"));
        }
    }

    #[test]
    fn a_shared_value_converts_as_its_current_content() {
        let engine = Engine::new();
        let value = engine
            .eval_with_scope::<Dynamic>(
                &mut Scope::new(),
                r#"
                let counter = 1;
                let bump = || counter += 1;
                bump.call();
                bump.call();
                #{ n: counter, list: [counter] }
                "#,
            )
            .unwrap();
        assert_eq!(
            to_json(&value, "result"),
            Ok(json!({ "n": 3, "list": [3] }))
        );
    }

    #[test]
    fn nesting_past_the_bound_is_refused_at_its_location() {
        let mut value = Dynamic::from_array(vec![]);
        for _ in 0..MAX_DEPTH {
            value = Dynamic::from_array(vec![value]);
        }
        let refusal = to_json(&value, "result").unwrap_err();
        assert!(refusal.reason.contains("nests deeper"));
        assert_eq!(refusal.path, format!("result{}", "[0]".repeat(MAX_DEPTH)));
        let mut within = Dynamic::from_array(vec![]);
        for _ in 1..MAX_DEPTH {
            within = Dynamic::from_array(vec![within]);
        }
        let json = to_json(&within, "result").unwrap();
        // The front reads it back, whatever the depth.
        serde_json::from_slice::<Value>(&serde_json::to_vec(&json).unwrap()).unwrap();
    }

    #[test]
    fn a_json_value_has_the_same_depth_bound_as_a_rhai_value() {
        let nested = |levels: usize| (0..levels).fold(json!(0), |inner, _| json!({ "k": [inner] }));
        // Each step wraps a map and an array, so 50 steps are 100 levels.
        assert_eq!(check_depth(&nested(MAX_DEPTH / 2), "value"), Ok(()));
        let refusal = check_depth(&nested(MAX_DEPTH / 2 + 1), "value").unwrap_err();
        assert!(refusal.reason.contains("nests deeper"), "{refusal}");
        assert_eq!(
            refusal.path,
            format!("value{}", ".k[0]".repeat(MAX_DEPTH / 2))
        );
        // The same bound as the other direction: 100 arrays pass, 101 refuse.
        let arrays = |levels: usize| (0..levels).fold(json!(0), |inner, _| json!([inner]));
        assert_eq!(check_depth(&arrays(MAX_DEPTH), "value"), Ok(()));
        assert!(check_depth(&arrays(MAX_DEPTH + 1), "value").is_err());
    }
}
