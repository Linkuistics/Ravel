//! The count of the JSON values in a response body
//! (`docs/specs/policy-evaluation.md`, *LLM endpoints and credentials*).
//!
//! A bound on the bytes of a body does not bound the memory of its decode. Rig
//! parses a successful body into a `serde_json::Value`, which it keeps as
//! `CompletionResponse.raw`, and it reads the body again for the answer. A body
//! of 8 MiB that holds `0,0,0,…` has four million values of 32 bytes each, and
//! its decode peaked at 586 MiB in the test that proves this bound. This pass
//! counts the values of the document without building a tree, so a body with too
//! many fails before Rig sees it. Its own memory is the depth of the document,
//! which serde_json limits to 128.

use std::cell::Cell;
use std::fmt;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

/// What the count found.
#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    /// The document has at most the limit of values, or it is not JSON, which
    /// Rig reports in its own words.
    Fits,
    /// The document has more values than the limit.
    TooMany,
}

/// Count the values of the JSON document in `bytes`: every scalar, array and
/// object, and every key of an object. The count stops at the first value past
/// `limit`.
pub fn check(bytes: &[u8], limit: u64) -> Shape {
    let counter = Counter {
        seen: Cell::new(0),
        limit,
        beyond: Cell::new(false),
    };
    let mut parser = serde_json::Deserializer::from_slice(bytes);
    // A parse error is not this pass's to report. The body goes to Rig, which
    // fails on it before it builds anything large.
    let _ = (&counter).deserialize(&mut parser);
    if counter.beyond.get() {
        Shape::TooMany
    } else {
        Shape::Fits
    }
}

struct Counter {
    seen: Cell<u64>,
    limit: u64,
    beyond: Cell<bool>,
}

/// The error that stops the walk at the limit.
struct Beyond;

impl Counter {
    fn count<E: de::Error>(&self) -> Result<(), E> {
        let seen = self.seen.get() + 1;
        self.seen.set(seen);
        if seen > self.limit {
            self.beyond.set(true);
            return Err(E::custom(Beyond));
        }
        Ok(())
    }
}

impl fmt::Display for Beyond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("more values than the limit")
    }
}

impl<'de> DeserializeSeed<'de> for &Counter {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for &Counter {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("any JSON value")
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
        self.count()
    }

    fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
        self.count()
    }

    fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
        self.count()
    }

    fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
        self.count()
    }

    fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
        self.count()
    }

    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        self.count()
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        self.count()?;
        while seq.next_element_seed(self)?.is_some() {}
        Ok(())
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        self.count()?;
        while map.next_key_seed(self)?.is_some() {
            map.next_value_seed(self)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_scalars_containers_and_keys() {
        // The array, the object, its key, its value, and two elements.
        let body = br#"[{"a":1},2,"x"]"#;
        assert_eq!(check(body, 6), Shape::Fits);
        assert_eq!(check(body, 5), Shape::TooMany);
    }

    #[test]
    fn a_document_at_the_limit_fits_and_one_value_more_does_not() {
        let body = b"[0,0,0]";
        assert_eq!(check(body, 4), Shape::Fits);
        assert_eq!(check(body, 3), Shape::TooMany);
    }

    #[test]
    fn a_document_that_is_not_json_is_left_to_rig() {
        assert_eq!(check(b"<html>", 10), Shape::Fits);
        assert_eq!(check(b"[0,0,", 10), Shape::Fits);
        assert_eq!(check(b"", 10), Shape::Fits);
    }

    #[test]
    fn a_count_that_passes_the_limit_stops_before_the_end_of_the_document() {
        // The tail is not JSON. A walk that reached it would fail on the syntax,
        // so `TooMany` shows that the walk stopped at the limit.
        assert_eq!(check(b"[0,0,0,0,0,0,0,0,zzz", 3), Shape::TooMany);
    }

    #[test]
    fn deep_nesting_ends_at_the_recursion_limit_of_serde_json() {
        let body = format!("{}{}", "[".repeat(10_000), "]".repeat(10_000));
        assert_eq!(check(body.as_bytes(), u64::MAX), Shape::Fits);
    }
}
