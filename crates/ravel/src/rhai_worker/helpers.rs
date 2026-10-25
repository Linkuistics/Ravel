//! The data helpers of the `ravel` namespace (`docs/specs/policy-evaluation.md`,
//! *Data helpers*): JSON, the version 4 UUID and the lexical path functions.
//!
//! They run in the worker and send no frame. The encoder may write any string
//! Rhai can hold, 16 MiB, and one message of the worker holds 1 MiB, so a round
//! trip could not carry them. They register through the same namespace as the
//! reads, so a malformed argument ends the evaluation as `host_request_invalid`
//! and a data failure is a thrown map, as everywhere in `ravel`.

use std::rc::Rc;

use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, NativeCallContext};

use super::bridge::{Bridge, Outcome, thrown};
use super::convert::{check_depth, from_json, to_json};
use crate::schema::Schema;

pub(super) fn register(module: &mut Module, bridge: &Rc<Bridge>) {
    let checked = Rc::clone(bridge);
    FuncRegistration::new("json_parse").set_into_module(
        module,
        move |context: NativeCallContext, text: Dynamic| {
            let text = checked.string(&context, "json_parse", "text", &text)?;
            parse(&context, &text)
        },
    );
    FuncRegistration::new("json_encode")
        .set_into_module(module, |context: NativeCallContext, value: Dynamic| {
            encode(&context, &value)
        });
    FuncRegistration::new("json_check").set_into_module(
        module,
        |context: NativeCallContext, value: Dynamic, schema: Dynamic| {
            check(&context, &value, &schema)
        },
    );
    FuncRegistration::new("uuid_v4")
        .set_into_module(module, |context: NativeCallContext| uuid_v4(&context));
    let checked = Rc::clone(bridge);
    FuncRegistration::new("path_resolve").set_into_module(
        module,
        move |context: NativeCallContext, base: Dynamic, path: Dynamic| {
            let base = checked.string(&context, "path_resolve", "base", &base)?;
            let path = checked.string(&context, "path_resolve", "path", &path)?;
            if !base.starts_with('/') {
                return Err(not_absolute(&context, "path_resolve", "base", &base));
            }
            Ok(resolved(&base, &path))
        },
    );
    let checked = Rc::clone(bridge);
    FuncRegistration::new("path_parent").set_into_module(
        module,
        move |context: NativeCallContext, path: Dynamic| {
            let path = checked.string(&context, "path_parent", "path", &path)?;
            if !path.starts_with('/') {
                return Err(not_absolute(&context, "path_parent", "path", &path));
            }
            Ok(dirname(&path))
        },
    );
}

fn not_absolute(
    context: &NativeCallContext,
    function: &str,
    field: &str,
    given: &str,
) -> Box<EvalAltResult> {
    thrown(
        context,
        "path_not_absolute",
        format!("ravel::{function} needs an absolute {field}, and {given:?} is not"),
        None,
    )
}

/// `ravel::json_parse`: strict JSON, with the nesting bound of the conversion.
fn parse(context: &NativeCallContext, text: &str) -> Outcome<Dynamic> {
    let invalid = |why: String| thrown(context, "json_invalid", why, None);
    let value =
        serde_json::from_str(text).map_err(|error| invalid(format!("not JSON: {error}")))?;
    check_depth(&value, "value").map_err(|refused| invalid(format!("not usable: {refused}")))?;
    Ok(from_json(&value))
}

/// `ravel::json_encode`: the canonical form. `serde_json` writes no whitespace,
/// keeps its object members in ascending order of their UTF-8 bytes, which is
/// the order of their code points, escapes `"`, `\` and the control characters
/// with lowercase `\u00xx`, and writes floats in the shortest form that reads
/// back exactly (ryu).
/// <https://docs.rs/serde_json/1/serde_json/struct.Map.html>, without the
/// `preserve_order` feature
fn encode(context: &NativeCallContext, value: &Dynamic) -> Outcome<String> {
    let json = to_json(value, "value")
        .map_err(|refused| thrown(context, "json_unencodable", refused.to_string(), None))?;
    Ok(json.to_string())
}

/// `ravel::json_check`: the value, unchanged, when it matches the schema of the
/// subset. The schema comes first, so a schema outside the subset throws
/// whatever the value is. The mismatch message gives the location and the kinds
/// of value, and no part of the value except the keys in the location.
fn check(context: &NativeCallContext, value: &Dynamic, schema: &Dynamic) -> Outcome<Dynamic> {
    let invalid = |path: &str, reason: &str| {
        thrown(
            context,
            "schema_invalid",
            format!("the schema is outside the subset, at {path}: {reason}"),
            None,
        )
    };
    let schema =
        to_json(schema, "schema").map_err(|refused| invalid(&refused.path, &refused.reason))?;
    let schema = Schema::parse(&schema, "schema").map_err(|why| invalid(&why.path, &why.reason))?;
    let json = to_json(value, "value")
        .map_err(|refused| thrown(context, "json_unencodable", refused.to_string(), None))?;
    schema.check(&json, "value").map_err(|why| {
        thrown(
            context,
            "json_mismatch",
            format!("the value does not match the schema: {why}"),
            None,
        )
    })?;
    Ok(value.clone())
}

/// `ravel::uuid_v4`: 122 random bits from the operating system, with the version
/// and variant bits of RFC 9562 section 5.4.
fn uuid_v4(context: &NativeCallContext) -> Outcome<String> {
    let mut bytes = [0u8; 16];
    // https://docs.rs/getrandom/0.4.2/getrandom/fn.fill.html
    getrandom::fill(&mut bytes).map_err(|error| {
        thrown(
            context,
            "random_unavailable",
            format!("the operating system gave no random bytes: {error}"),
            None,
        )
    })?;
    bytes[6] = bytes[6] & 0x0f | 0x40;
    bytes[8] = bytes[8] & 0x3f | 0x80;
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    Ok(format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    ))
}

/// The `path.resolve(base, path)` of Node on POSIX, for an absolute `base`: an
/// absolute `path` replaces the base, empty and `.` segments vanish, and `..`
/// removes the segment before it and never rises above `/`.
/// <https://github.com/nodejs/node/blob/v22.x/lib/path.js> (`posix.resolve`)
pub(super) fn resolved(base: &str, path: &str) -> String {
    let joined = if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("{base}/{path}")
    };
    let mut kept: Vec<&str> = Vec::new();
    for segment in joined.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                kept.pop();
            }
            segment => kept.push(segment),
        }
    }
    format!("/{}", kept.join("/"))
}

/// The `path.dirname(path)` of Node on POSIX, for an absolute `path`. It
/// normalises nothing: it drops trailing slashes and the last segment, and
/// keeps what is left of the rest as it is, so `/a//b` gives `/a/`, and `//a`
/// gives `//`.
/// <https://github.com/nodejs/node/blob/v22.x/lib/path.js> (`posix.dirname`)
pub(super) fn dirname(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut matched_slash = true;
    let mut end = None;
    for index in (1..bytes.len()).rev() {
        if bytes[index] == b'/' {
            if !matched_slash {
                end = Some(index);
                break;
            }
        } else {
            matched_slash = false;
        }
    }
    match end {
        None => "/".to_owned(),
        Some(1) => "//".to_owned(),
        Some(end) => path[..end].to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_joins_normalises_and_never_rises_above_the_root() {
        for (base, path, expected) in [
            ("/a/b", "c", "/a/b/c"),
            ("/a/b", "../c", "/a/c"),
            ("/a/b", "../../../c", "/c"),
            ("/a/b", "/x/./y/", "/x/y"),
            ("/a/b", "", "/a/b"),
            ("/", "..", "/"),
            ("//a//b/", "./c//d", "/a/b/c/d"),
        ] {
            assert_eq!(resolved(base, path), expected, "{base:?} {path:?}");
        }
    }

    #[test]
    fn dirname_keeps_the_quirks_of_node() {
        for (path, expected) in [
            ("/", "/"),
            ("/a", "/"),
            ("/a/", "/"),
            ("/a/b", "/a"),
            ("/a/b/", "/a"),
            ("/a//b", "/a/"),
            ("//a", "//"),
            ("/a/./b", "/a/."),
        ] {
            assert_eq!(dirname(path), expected, "{path:?}");
        }
    }
}
