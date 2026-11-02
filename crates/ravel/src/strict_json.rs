//! JSON for the owner settings file: `serde_json::Value` keeps the last of two
//! members with one name, so this reader builds the same value and refuses a
//! repeated member name in any object, with the location of the repeat.

use std::cell::RefCell;
use std::fmt;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};

/// Why a document did not parse.
#[derive(Debug)]
pub enum Error {
    /// The bytes are not one JSON value.
    Syntax(serde_json::Error),
    /// An object repeats a member name. The location names the repeated
    /// member, such as `skills.sources[2].id`.
    Repeated { location: String },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syntax(error) => write!(formatter, "is not JSON: {error}"),
            Error::Repeated { location } => {
                write!(formatter, "repeats the member name at {location}")
            }
        }
    }
}

/// Parse one JSON document and refuse a repeated member name in any object.
pub fn parse(bytes: &[u8]) -> Result<Value, Error> {
    let repeated = RefCell::new(None);
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let parsed = Seed {
        location: String::new(),
        repeated: &repeated,
    }
    .deserialize(&mut deserializer)
    .and_then(|value| deserializer.end().map(|()| value));
    match (parsed, repeated.into_inner()) {
        (_, Some(location)) => Err(Error::Repeated { location }),
        (Ok(value), None) => Ok(value),
        (Err(error), None) => Err(Error::Syntax(error)),
    }
}

/// One value at its location. The first repeat is recorded beside the parse,
/// because a deserializer error carries a message and no structure.
struct Seed<'a> {
    location: String,
    repeated: &'a RefCell<Option<String>>,
}

impl Seed<'_> {
    fn child(&self, location: String) -> Self {
        Seed {
            location,
            repeated: self.repeated,
        }
    }
}

impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("a number that is not finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut items: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) =
            items.next_element_seed(self.child(format!("{}[{}]", self.location, values.len())))?
        {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut members: A) -> Result<Value, A::Error> {
        let mut object = Map::new();
        while let Some(name) = members.next_key::<String>()? {
            let location = if self.location.is_empty() {
                name.clone()
            } else {
                format!("{}.{name}", self.location)
            };
            if object.contains_key(&name) {
                *self.repeated.borrow_mut() = Some(location);
                return Err(de::Error::custom("a repeated member name"));
            }
            let value = members.next_value_seed(self.child(location))?;
            object.insert(name, value);
        }
        Ok(Value::Object(object))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_document_without_repeats_parses_as_serde_json_parses_it() {
        let text = r#"{"a":[1,-2,3.5,"x",null,true,{"b":{}}],"c":{"d":[]}}"#;
        let expected: Value = serde_json::from_str(text).unwrap();
        assert_eq!(parse(text.as_bytes()).unwrap(), expected);
    }

    #[test]
    fn a_repeated_member_name_refuses_at_its_location_in_any_object() {
        for (text, location) in [
            (r#"{"a":1,"a":2}"#, "a"),
            (r#"{"a":{"b":1,"c":2,"b":3}}"#, "a.b"),
            (r#"{"s":[{"x":1},{"x":1},{"y":1,"y":1}]}"#, "s[2].y"),
            (r#"{"a":[[{"z":0,"z":0}]]}"#, "a[0][0].z"),
        ] {
            match parse(text.as_bytes()) {
                Err(Error::Repeated { location: found }) => assert_eq!(found, location, "{text}"),
                other => panic!("{text}: {other:?}"),
            }
        }
    }

    #[test]
    fn malformed_text_and_trailing_bytes_are_syntax_errors() {
        for text in ["", "{", r#"{"a":1} x"#, "[1,]"] {
            assert!(
                matches!(parse(text.as_bytes()), Err(Error::Syntax(_))),
                "{text}"
            );
        }
    }
}
