//! `ravel::json_check` through the real `ravel start-session`
//! (`docs/specs/configuration-evaluation.md`, *Data helpers*).
//!
//! The helper runs in the worker and sends no message, and a data failure is a
//! thrown map that the script can catch. Every case therefore runs in a configuration script
//! that catches the throw and hands the code and the message to the recording
//! harness, so a test sees what the script saw. Each refusing case has a
//! matching case beside it that must pass, so a helper that threw for every
//! input would fail the test.

mod support;

use support::rhai::{Recorded, rhai_script};
use support::{Run, Sandbox};

fn start(sandbox: &Sandbox) -> Run {
    sandbox.start(&["--kind", "check", "--json"])
}

/// `verdict(|| call)` is `ok` when `call` returns, and `code|message` when it
/// throws.
const VERDICT: &str = r#"fn verdict(attempt) {
    try { attempt.call(); } catch (error) { return error.code + "|" + error.message; }
    "ok"
}
"#;

/// A `select` whose `body` computes `args`, an array of strings.
fn selecting(body: &str) -> String {
    rhai_script(&format!(
        r#"{VERDICT}
fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }}
}}"#
    ))
}

fn run_config(body: &str) -> (Sandbox, Recorded, Run) {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(body));
    let run = start(&sandbox);
    (sandbox, recorded, run)
}

/// The arguments that a configuration script with `body` handed to the harness.
fn computed(body: &str) -> Vec<String> {
    let (_sandbox, recorded, run) = run_config(body);
    run.report();
    recorded.argv().unwrap()
}

/// The verdict of `ravel::json_check(value, schema)` for each case, a pair of
/// Rhai expressions.
fn verdicts(cases: &[(&str, &str)]) -> Vec<String> {
    let pushes: String = cases
        .iter()
        .map(|(value, schema)| {
            format!("    args.push(verdict(|| ravel::json_check({value}, {schema})));\n")
        })
        .collect();
    let args = computed(&format!("    let args = [];\n{pushes}"));
    assert_eq!(args.len(), cases.len());
    args
}

const MISMATCH: &str = "json_mismatch|the value does not match the schema: ";

/// A person: `name` is required, `age` is not, and nothing else is allowed.
const PERSON: &str = r#"#{ type: "object",
    properties: #{ name: #{ type: "string" }, age: #{ type: "integer" } },
    required: ["name"], additionalProperties: false }"#;

#[test]
fn a_value_that_matches_comes_back_unchanged() {
    let args = computed(&format!(
        r#"    let cases = [
        [#{{ name: "Ada", age: 36 }}, {PERSON}],
        [#{{ name: "Ada" }}, {PERSON}],
        [[1, 2, 3], #{{ type: "array", items: #{{ type: "integer" }} }}],
        [[], #{{ type: "array", items: #{{ type: "integer" }} }}],
        ["red", #{{ enum: ["red", "green", 1, true] }}],
        [(), #{{ type: ["string", "null"] }}],
        [3.0, #{{ type: "integer" }}],
        [#{{ anything: [1, "two", ()] }}, #{{}}],
    ];
    let args = [];
    for row in cases {{
        let result = ravel::json_check(row[0], row[1]);
        args.push((result == row[0]).to_string() + " " + type_of(result) + " " + type_of(row[0]));
    }}"#
    ));

    assert_eq!(
        args,
        [
            "true map map",
            "true map map",
            "true array array",
            "true array array",
            "true string string",
            "true () ()",
            "true f64 f64",
            "true map map",
        ]
    );
}

#[test]
fn a_value_that_does_not_match_throws_json_mismatch_with_its_location() {
    let nested = r#"#{ type: "object", properties: #{ items: #{ type: "array",
        items: #{ type: "object", properties: #{ name: #{ type: "string" } } } } } }"#;
    let cases = [
        (
            r#"#{ name: "Ada", age: 3.5 }"#,
            PERSON,
            "value.age: expected integer, found number",
        ),
        (
            r#"#{ name: 3 }"#,
            PERSON,
            "value.name: expected string, found integer",
        ),
        (
            r#"#{ age: 3 }"#,
            PERSON,
            "value: the required property name is missing",
        ),
        (
            r#"#{ name: "Ada", nick: "A" }"#,
            PERSON,
            "value.nick: the schema allows no additional property",
        ),
        (
            r#"#{ color: "blue" }"#,
            r#"#{ type: "object", properties: #{ color: #{ enum: ["red", "green"] } } }"#,
            "value.color: it is none of the 2 values that enum allows",
        ),
        (
            r#"["a", 2]"#,
            r#"#{ type: "array", items: #{ type: "string" } }"#,
            "value[1]: expected string, found integer",
        ),
        (
            r#"#{ items: [#{ name: "a" }, #{ name: 3 }] }"#,
            nested,
            "value.items[1].name: expected string, found integer",
        ),
        (
            "3",
            r#"#{ type: ["string", "null"] }"#,
            "value: expected string or null, found integer",
        ),
        (
            "[]",
            r#"#{ type: "object" }"#,
            "value: expected object, found array",
        ),
        (
            "()",
            r#"#{ type: "boolean" }"#,
            "value: expected boolean, found null",
        ),
    ];
    let table: Vec<(&str, &str)> = cases
        .iter()
        .map(|(value, schema, _)| (*value, *schema))
        .collect();

    let args = verdicts(&table);

    for ((value, _, location), verdict) in cases.iter().zip(&args) {
        assert_eq!(verdict, &format!("{MISMATCH}{location}"), "{value}");
    }
}

#[test]
fn a_mismatch_message_holds_no_part_of_the_value_and_cuts_a_long_key() {
    let long = "k".repeat(200);
    let args = verdicts(&[
        // The firing control: the same value matches a schema that accepts it.
        (r#"#{ name: "SECRET-TEXT-1234" }"#, r#"#{ type: "object" }"#),
        (
            r#"#{ name: "SECRET-TEXT-1234" }"#,
            r#"#{ properties: #{ name: #{ type: "integer" } } }"#,
        ),
        (
            r#"#{ name: "SECRET-TEXT-1234" }"#,
            r#"#{ properties: #{ name: #{ enum: ["a", "b"] } } }"#,
        ),
        (
            r#"["SECRET-TEXT-1234"]"#,
            r#"#{ type: "array", items: #{ type: "boolean" } }"#,
        ),
        (
            &format!(r#"#{{ {long}: 1 }}"#),
            r#"#{ additionalProperties: false }"#,
        ),
    ]);

    assert_eq!(args[0], "ok");
    for verdict in &args[1..4] {
        assert!(verdict.starts_with(MISMATCH), "{verdict}");
        assert!(!verdict.contains("SECRET-TEXT-1234"), "{verdict}");
    }
    let cut = format!("{}…", "k".repeat(64));
    assert!(args[4].contains(&format!("value.{cut}:")), "{}", args[4]);
    assert!(!args[4].contains(&"k".repeat(65)), "{}", args[4]);
}

#[test]
fn a_schema_outside_the_subset_throws_schema_invalid_and_names_its_field() {
    let cases = [
        (r#"#{ type: "object", pattern: "^a" }"#, "schema.pattern"),
        (
            r#"#{ type: "object", minProperties: 1 }"#,
            "schema.minProperties",
        ),
        (r#"#{ oneOf: [#{ type: "string" }] }"#, "schema.oneOf"),
        (r##"#{ "$ref": "#/defs/a" }"##, "schema.$ref"),
        (
            r#"#{ type: "object", properties: #{ a: #{ format: "date" } } }"#,
            "schema.properties.a.format",
        ),
        (
            r#"#{ type: "array", items: #{ minLength: 1 } }"#,
            "schema.items.minLength",
        ),
        (r#"#{ type: "text" }"#, "schema.type"),
        (r#"#{ type: ["string", "text"] }"#, "schema.type[1]"),
        (r#"#{ type: [] }"#, "schema.type"),
        (r#"#{ properties: [] }"#, "schema.properties"),
        (
            r#"#{ properties: #{ a: "string" } }"#,
            "schema.properties.a",
        ),
        (r#"#{ required: "a" }"#, "schema.required"),
        (r#"#{ required: ["a", 3] }"#, "schema.required[1]"),
        (
            r#"#{ additionalProperties: "no" }"#,
            "schema.additionalProperties",
        ),
        (
            r#"#{ additionalProperties: #{ type: "string" } }"#,
            "schema.additionalProperties",
        ),
        (r#"#{ items: [#{ type: "string" }] }"#, "schema.items"),
        (r#"#{ enum: "a" }"#, "schema.enum"),
        (r#"#{ title: 3 }"#, "schema.title"),
        (r#"#{ description: [] }"#, "schema.description"),
        (r#""object""#, "schema"),
        ("()", "schema"),
        (r#"#{ f: || 1 }"#, "schema.f"),
    ];
    let mut table: Vec<(&str, &str)> = cases.iter().map(|(schema, _)| ("1", *schema)).collect();
    // The firing control: the same value passes a schema inside the subset.
    table.push(("1", r#"#{ type: "integer", title: "t", description: "d" }"#));
    // The schema is checked before the value, so a bad schema throws whatever
    // the value is.
    table.push(("'c'", r#"#{ pattern: "^a" }"#));

    let args = verdicts(&table);

    for ((schema, path), verdict) in cases.iter().zip(&args) {
        assert!(
            verdict.starts_with(&format!(
                "schema_invalid|the schema is outside the subset, at {path}: "
            )),
            "{schema}: {verdict}"
        );
    }
    assert_eq!(args[cases.len()], "ok");
    assert!(
        args[cases.len() + 1].starts_with("schema_invalid|"),
        "{}",
        args[cases.len() + 1]
    );
}

#[test]
fn a_schema_nested_32_levels_passes_and_33_levels_throws_at_the_33rd() {
    let by_property = r#"schema = #{ type: "object", properties: #{ a: schema } };"#;
    let by_items = r#"schema = #{ type: "array", items: schema };"#;
    let mut body = String::from("    let args = [];\n");
    for (step, wrap) in [(by_property, "#{ a: value }"), (by_items, "[value]")] {
        for levels in [32, 33] {
            // The value nests as deep as the schema, so a pass proves that the
            // deepest schema was applied.
            body.push_str(&format!(
                "    {{\n        let schema = #{{ type: \"string\" }};\n        let value = \"x\";\n        \
                 for step in 1..{levels} {{ {step} value = {wrap}; }}\n        \
                 args.push(verdict(|| ravel::json_check(value, schema)));\n    }}\n"
            ));
        }
    }

    let args = computed(&body);

    assert_eq!(args[0], "ok");
    assert_eq!(args[2], "ok");
    for (index, path) in [(1, ".properties.a".repeat(32)), (3, ".items".repeat(32))] {
        assert!(
            args[index].starts_with(&format!(
                "schema_invalid|the schema is outside the subset, at schema{path}: "
            )),
            "{}",
            args[index]
        );
        assert!(args[index].contains("32 levels"), "{}", args[index]);
    }
}

#[test]
fn a_value_with_no_json_form_throws_json_unencodable_and_names_the_location() {
    let args = computed(
        r#"    let args = [];
    let schema = #{};
    for value in [0.0 / 0.0, 1.0 / 0.0, 'c', blob(2), |x| x, timestamp(), 1..3] {
        args.push(verdict(|| ravel::json_check(value, schema)));
    }
    args.push(verdict(|| ravel::json_check(#{ args: ["a", "b", 'c'] }, schema)));
    args.push(verdict(|| ravel::json_check(#{ args: ["a", "b", "c"] }, schema)));"#,
    );

    for verdict in &args[..7] {
        assert!(verdict.starts_with("json_unencodable|value"), "{verdict}");
    }
    assert_eq!(
        args[7],
        "json_unencodable|value.args[2]: a value of type char has no JSON form"
    );
    assert_eq!(args[8], "ok", "an encodable value passes the same schema");
}

#[test]
fn an_integer_matches_a_float_with_no_fraction_and_a_float_with_one_does_not() {
    let args = verdicts(&[
        ("3.0", r#"#{ type: "integer" }"#),
        ("-0.0", r#"#{ type: "integer" }"#),
        ("3.5", r#"#{ type: "integer" }"#),
        ("3", r#"#{ type: "integer" }"#),
        ("3", r#"#{ type: "number" }"#),
        ("3.0", r#"#{ enum: [3] }"#),
        ("3", r#"#{ enum: [3.0] }"#),
        ("3.5", r#"#{ enum: [3] }"#),
    ]);

    assert_eq!(args[..2], ["ok", "ok"]);
    assert_eq!(
        args[2],
        format!("{MISMATCH}value: expected integer, found number")
    );
    assert_eq!(args[3..7], ["ok", "ok", "ok", "ok"]);
    assert_eq!(
        args[7],
        format!("{MISMATCH}value: it is none of the 1 values that enum allows")
    );
}

#[test]
fn a_keyword_applies_to_one_kind_of_value() {
    let args = verdicts(&[
        // `required`, `properties` and `additionalProperties` say nothing about
        // an array, and `items` says nothing about an object.
        ("[]", r#"#{ required: ["x"] }"#),
        ("[1]", r#"#{ properties: #{ a: #{ type: "string" } } }"#),
        ("[1]", r#"#{ additionalProperties: false }"#),
        (r#"#{ x: 1 }"#, r#"#{ items: #{ type: "string" } }"#),
        // `items` says nothing about a string either.
        (
            r#""text""#,
            r#"#{ items: #{ type: "integer" }, required: ["x"] }"#,
        ),
        // The firing controls: the same keywords bind the kind they describe.
        ("#{}", r#"#{ required: ["x"] }"#),
        (
            r#"#{ a: 1 }"#,
            r#"#{ properties: #{ a: #{ type: "string" } } }"#,
        ),
        (r#"#{ a: 1 }"#, r#"#{ additionalProperties: false }"#),
        (r#"["a"]"#, r#"#{ items: #{ type: "integer" } }"#),
    ]);

    assert_eq!(args[..5], ["ok", "ok", "ok", "ok", "ok"]);
    for verdict in &args[5..] {
        assert!(verdict.starts_with(MISMATCH), "{verdict}");
    }
}

#[test]
fn enum_accepts_scalars_only() {
    let args = verdicts(&[
        (r#""a""#, r#"#{ enum: ["a", 1, 2.5, true, ()] }"#),
        ("()", r#"#{ enum: ["a", ()] }"#),
        ("2.5", r#"#{ enum: ["a", 2.5] }"#),
        (r#""a""#, r#"#{ enum: [["a"]] }"#),
        (r#""a""#, r#"#{ enum: [#{ k: "a" }] }"#),
        (r#""a""#, r#"#{ enum: ["b", ["a"]] }"#),
    ]);

    assert_eq!(args[..3], ["ok", "ok", "ok"]);
    for (verdict, path) in
        args[3..]
            .iter()
            .zip(["schema.enum[0]", "schema.enum[0]", "schema.enum[1]"])
    {
        assert!(
            verdict.starts_with(&format!(
                "schema_invalid|the schema is outside the subset, at {path}: "
            )),
            "{verdict}"
        );
    }
}

#[test]
fn a_value_over_the_size_of_a_message_is_checked_because_the_helper_sends_none() {
    // One message of the worker holds 1 MiB. A helper that sent the value to the
    // CLI could not carry a 2 MiB string.
    let args = computed(
        r#"    let text = "x";
    for step in 0..21 { text += text; }
    let args = [text.len().to_string(),
                verdict(|| ravel::json_check(text, #{ type: "string" })),
                verdict(|| ravel::json_check(text, #{ type: "integer" })).sub_string(0, 13)];"#,
    );

    assert_eq!(args, ["2097152", "ok", "json_mismatch"]);
}

#[test]
fn an_uncaught_throw_ends_the_evaluation_and_starts_no_harness() {
    // The firing control: the same call with a matching value starts the harness.
    assert_eq!(
        computed("    ravel::json_check(1, #{ type: \"integer\" });\n    let args = [\"routed\"];"),
        ["routed"]
    );

    let (_sandbox, recorded, run) = run_config(
        r#"    ravel::json_check(1, #{ type: "string" });
    let args = [];"#,
    );

    let document = run.refusal(3);
    assert_eq!(
        recorded.argv(),
        None,
        "a harness started after a refusal: {document}"
    );
}
