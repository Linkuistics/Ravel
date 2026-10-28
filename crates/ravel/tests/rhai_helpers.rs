//! The data helpers of a Rhai configuration script through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *Data helpers*, and the capability list of
//! *Migration*): JSON, the UUID, the path functions and the environment read.
//!
//! The last part proves each capability that Grove's helpers need, with no Grove
//! code. Everything runs in a sandbox home with fake harness scripts on a
//! private `PATH`, and the configuration script hands what it computed to the recording
//! harness as arguments, so a test sees what the script saw.

mod support;

use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt as _;

use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use support::rhai::{self, Recorded, policy};
use support::{Run, Sandbox, run, text};

fn refused(run: &Run, code: &str, recorded: &Recorded) -> Value {
    let document = run.refusal(3);
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    document["error"].clone()
}

fn start(sandbox: &Sandbox, args: &[&str]) -> Run {
    let mut all = vec!["--kind", "check", "--json"];
    all.extend_from_slice(args);
    sandbox.start(&all)
}

/// `caught(|| call)` is the code of the error that `call` throws, or `none`.
const CAUGHT: &str = r#"fn caught(attempt) {
    try { attempt.call(); } catch (error) { return error.code; }
    "none"
}
"#;

/// A `select` whose `body` computes `args`, an array of strings.
fn selecting(body: &str) -> String {
    selecting_with("", body)
}

/// `selecting`, after the global functions in `globals`.
fn selecting_with(globals: &str, body: &str) -> String {
    policy(&format!(
        r#"{CAUGHT}{globals}
fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }}
}}"#
    ))
}

/// The arguments that a configuration script with `body` handed to the harness.
fn computed(body: &str) -> Vec<String> {
    computed_with("", body)
}

fn computed_with(globals: &str, body: &str) -> Vec<String> {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting_with(globals, body));
    start(&sandbox, &[]).report();
    recorded.argv().unwrap()
}

/// `computed`, for a table of `rows` in the Rhai variable `rows`.
fn computed_over(rows: &Value, body: &str) -> Vec<String> {
    computed(&format!("    let rows = {rows};\n{body}"))
}

#[test]
fn json_parse_reads_strict_json_into_rhai_values() {
    let args = computed(
        r#"    let v = ravel::json_parse(`{"s":"é\n","i":-3,"big":9223372036854775808,"low":-9223372036854775809,"max":9223372036854775807,"f":1.5,"e":1e2,"t":true,"n":null,"a":[1,[2]],"o":{"k":"v"}}`);
    let args = [v.s.len().to_string(), type_of(v.i), v.i.to_string(), type_of(v.big), type_of(v.low),
                type_of(v.max), type_of(v.f), type_of(v.e), v.t.to_string(), type_of(v.n),
                v.a[1][0].to_string(), v.o.k, (v.big == 9223372036854775808.0).to_string()];"#,
    );

    assert_eq!(
        args,
        [
            "2", "i64", "-3", "f64", "f64", "i64", "f64", "f64", "true", "()", "2", "v", "true"
        ]
    );
}

#[test]
fn json_parse_throws_json_invalid_for_what_is_not_strict_json_and_for_nesting_past_100_levels() {
    let args = computed_with(
        r#"fn nested(levels) {
    let text = "";
    for step in 0..levels { text += "["; }
    for step in 0..levels { text += "]"; }
    text
}"#,
        r##"    let args = [];
    for text in ["", "  ", "{'a':1}", "[1,]", "{\"a\":1,}", "NaN", "Infinity", "+1", "01",
                 "[1] x", "// c\n1", "#{a: 1}", "\"\\x41\"", "[1e999]", "\ufeff1", "[",
                 "\"\t\"", "[1, 2", "{\"a\" 1}", "tru"] {
        args.push(caught(|| ravel::json_parse(text)));
    }
    args.push(caught(|| ravel::json_parse(nested(100))));
    args.push(caught(|| ravel::json_parse(nested(101))));
    args.push(caught(|| ravel::json_parse(nested(200))));
    args.push(caught(|| ravel::json_parse(nested(5000))));
    try { ravel::json_parse("[1,"); } catch (error) {
        args.push(if error.message.len() > 0 { "said" } else { "silent" });
    }"##,
    );

    let mut expected = vec!["json_invalid"; 20];
    expected.extend([
        "none",
        "json_invalid",
        "json_invalid",
        "json_invalid",
        "said",
    ]);
    assert_eq!(args, expected);
}

#[test]
fn json_parse_and_json_encode_replace_the_to_json_and_parse_json_of_rhai() {
    let args = computed(
        r##"    let args = [ravel::json_encode("\u0001"), ravel::json_encode(#{ a: "\u0001" })];
    args.push(#{ a: "\u0001" }.to_json());
    // Rhai reads a trailing comma and a single-quoted string, which JSON does not have.
    args.push(parse_json("{\"a\": 1,}").a.to_string());
    args.push(caught(|| ravel::json_parse("{\"a\": 1,}")));"##,
    );

    assert_eq!(
        args,
        [
            r#""\u0001""#,
            r#"{"a":"\u0001"}"#,
            // Rhai writes the Rust debug escape, which is not JSON.
            "{\"a\":\"\\u{1}\"}",
            "1",
            "json_invalid"
        ]
    );
}

#[test]
fn json_encode_writes_no_whitespace_and_orders_members_by_code_point() {
    let args = computed(
        r#"    let cases = [
        #{ b: 1, a: [true, (), "x"], "é": 2, Z: 3 },
        #{ "\U00010000": 1, "\uff5e": 2 },
        [],
        #{},
        #{ nested: #{ y: [#{ b: 1, a: 2 }] } },
        (),
        "",
        [[], #{}, [[]]],
    ];
    let args = [];
    for item in cases { args.push(ravel::json_encode(item)); }"#,
    );

    assert_eq!(
        args,
        [
            r#"{"Z":3,"a":[true,null,"x"],"b":1,"é":2}"#,
            // U+FF5E before U+10000: code point order, not UTF-16 order.
            "{\"\u{ff5e}\":2,\"\u{10000}\":1}",
            "[]",
            "{}",
            r#"{"nested":{"y":[{"a":2,"b":1}]}}"#,
            "null",
            r#""""#,
            "[[],{},[[]]]",
        ]
    );
}

#[test]
fn json_encode_of_a_string_writes_the_bytes_that_json_stringify_writes() {
    // Node v26.12.1: JSON.stringify("a\"b\\c\b\f\n\r\t\u0001\u001f\u007f\u2028é😀") keeps
    // the DEL and U+2028 as they are.
    let args = computed(
        r#"    let text = "a\"b\\c\u0008\u000c\n\r\t\u0001\u001f\u007f\u2028é\U0001F600";
    let args = [ravel::json_encode(text)];"#,
    );

    assert_eq!(
        args,
        ["\"a\\\"b\\\\c\\b\\f\\n\\r\\t\\u0001\\u001f\u{7f}\u{2028}é😀\""]
    );
}

#[test]
fn json_encode_writes_each_other_control_character_as_u00xx_in_lowercase_hexadecimal() {
    // 0x0b, 0x0e, 0x1a to 0x1f hold every hexadecimal letter that a escape can use.
    let args = computed(
        r#"    let args = [];
    for text in ["\u0000", "\u0001", "\u000b", "\u000e", "\u001a", "\u001b", "\u001c",
                 "\u001d", "\u001e", "\u001f", "\u0020", "/", "\u00ff"] {
        args.push(ravel::json_encode(text));
    }"#,
    );

    assert_eq!(
        args,
        [
            r#""\u0000""#,
            r#""\u0001""#,
            r#""\u000b""#,
            r#""\u000e""#,
            r#""\u001a""#,
            r#""\u001b""#,
            r#""\u001c""#,
            r#""\u001d""#,
            r#""\u001e""#,
            r#""\u001f""#,
            r#"" ""#,
            r#""/""#,
            "\"\u{ff}\"",
        ]
    );
}

#[test]
fn json_encode_writes_integers_in_decimal_and_floats_in_the_shortest_form_that_reads_back() {
    let args = computed(
        r#"    let numbers = [0, -7, 9223372036854775807, -9223372036854775808, 1.5, 1.0, -0.0,
                   123456789.125, 0.1 + 0.2, 1000000000000000000000.0, 0.0000001,
                   9223372036854775807.0 + 1.0, 5.0 / 3.0];
    let args = [];
    for number in numbers {
        let written = ravel::json_encode(number);
        args.push(written);
        let back = ravel::json_parse(written);
        args.push(if back == number && type_of(back) == type_of(number) { "same" } else { "differs" });
    }"#,
    );

    let expected = [
        ("0", "same"),
        ("-7", "same"),
        ("9223372036854775807", "same"),
        ("-9223372036854775808", "same"),
        ("1.5", "same"),
        ("1.0", "same"),
        ("-0.0", "same"),
        ("123456789.125", "same"),
        ("0.30000000000000004", "same"),
        ("1e+21", "same"),
        ("1e-7", "same"),
        ("9.223372036854776e+18", "same"),
        ("1.6666666666666667", "same"),
    ];
    let expected: Vec<&str> = expected.iter().flat_map(|(a, b)| [*a, *b]).collect();
    assert_eq!(args, expected);
}

#[test]
fn json_encode_throws_json_unencodable_and_names_the_location_of_the_value() {
    let args = computed(
        r#"    let args = [];
    for value in [0.0 / 0.0, 1.0 / 0.0, -1.0 / 0.0, 'c', blob(2), |x| x, timestamp(), 1..3,
                  #{ args: ["a", "b", 'c'] }] {
        args.push(caught(|| ravel::json_encode(value)));
    }
    try { ravel::json_encode(#{ args: ["a", "b", 'c'] }); } catch (error) {
        args.push(error.message);
    }
    let deep = [];
    for step in 0..99 { deep = [deep]; }
    args.push(caught(|| ravel::json_encode(deep)));
    deep = [deep];
    args.push(caught(|| ravel::json_encode(deep)));"#,
    );

    let mut expected = vec!["json_unencodable".to_owned(); 9];
    expected.push("value.args[2]: a value of type char has no JSON form".to_owned());
    expected.extend(["none".to_owned(), "json_unencodable".to_owned()]);
    assert_eq!(args, expected);
}

#[test]
fn json_encode_and_json_parse_round_trip_a_nested_document() {
    let args = computed(
        r#"    let document = #{ name: "x", list: [1, 2.5, "é", (), true, [#{ k: -1 }]], map: #{ b: 1, a: #{} } };
    let written = ravel::json_encode(document);
    let again = ravel::json_encode(ravel::json_parse(written));
    let args = [written, (again == written).to_string(),
                (ravel::json_parse(written) == document).to_string()];"#,
    );

    assert_eq!(
        args[0],
        r#"{"list":[1,2.5,"é",null,true,[{"k":-1}]],"map":{"a":{},"b":1},"name":"x"}"#
    );
    assert_eq!(args[1..], ["true", "true"]);
}

#[test]
fn uuid_v4_returns_a_lowercase_version_4_uuid_that_differs_each_call() {
    let args = computed(
        r#"    let seen = #{};
    let bad = [];
    for step in 0..64 {
        let id = ravel::uuid_v4();
        seen[id] = true;
        let digits = id;
        digits.replace("-", "");
        let ok = id.len() == 36 && id[8] == '-' && id[13] == '-' && id[18] == '-' && id[23] == '-'
            && id[14] == '4' && "89ab".contains(id[19]) && digits.len() == 32;
        for digit in digits.chars() { if !"0123456789abcdef".contains(digit) { ok = false; } }
        if !ok { bad.push(id); }
    }
    let args = [bad.len().to_string(), seen.len().to_string()];"#,
    );

    assert_eq!(args, ["0", "64"]);
}

fn rows<const N: usize>(table: &[[&str; N]]) -> Value {
    Value::Array(table.iter().map(|row| json!(row.as_slice())).collect())
}

#[test]
fn path_resolve_equals_the_path_resolve_of_node_for_an_absolute_base() {
    let table: Vec<[&str; 2]> = NODE_RESOLVE.iter().map(|(b, p, _)| [*b, *p]).collect();
    let args = computed_over(
        &rows(&table),
        "    let args = [];\n    for row in rows { args.push(ravel::path_resolve(row[0], row[1])); }",
    );

    let expected: Vec<&str> = NODE_RESOLVE.iter().map(|(_, _, r)| *r).collect();
    assert_eq!(args, expected);
}

#[test]
fn path_parent_equals_the_path_dirname_of_node_for_an_absolute_path() {
    let table: Vec<[&str; 1]> = NODE_DIRNAME.iter().map(|(p, _)| [*p]).collect();
    let args = computed_over(
        &rows(&table),
        "    let args = [];\n    for row in rows { args.push(ravel::path_parent(row[0])); }",
    );

    let expected: Vec<&str> = NODE_DIRNAME.iter().map(|(_, r)| *r).collect();
    assert_eq!(args, expected);
}

#[test]
fn a_relative_path_or_base_throws_path_not_absolute_and_no_path_function_reads_a_file() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(
        r#"    let args = [
        caught(|| ravel::path_resolve("rel", "x")),
        caught(|| ravel::path_resolve("", "x")),
        caught(|| ravel::path_resolve("./a", "/b")),
        caught(|| ravel::path_parent("rel")),
        caught(|| ravel::path_parent("")),
        caught(|| ravel::path_parent("a/b")),
        // Neither file exists, and neither call looks for one.
        ravel::path_resolve("/no/such/dir", "../x/./y"),
        ravel::path_parent("/no/such/file"),
    ];
    try { ravel::path_parent("rel"); } catch (error) { args.push(error.message); }"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "path_not_absolute",
            "path_not_absolute",
            "path_not_absolute",
            "path_not_absolute",
            "path_not_absolute",
            "path_not_absolute",
            "/no/such/x/y",
            "/no/such",
            "ravel::path_parent needs an absolute path, and \"rel\" is not",
        ]
    );
    assert_eq!(report["reads"], json!([]));
}

/// `path.resolve(base, path)` of Node v26.12.1 (POSIX): base, path, result.
const NODE_RESOLVE: &[(&str, &str, &str)] = &[
    ("/", "", "/"),
    ("/", "c", "/c"),
    ("/", "../c", "/c"),
    ("/", "../../..", "/"),
    ("/", "/x/./y/", "/x/y"),
    ("/", "./", "/"),
    ("/", "..", "/"),
    ("/", "c/..", "/"),
    ("/", "a//b", "/a/b"),
    ("/", "/", "/"),
    ("/", "//x", "/x"),
    ("/", "...", "/..."),
    ("/", ".hidden", "/.hidden"),
    ("/", "a/../../b", "/b"),
    ("/", "é/ü", "/é/ü"),
    ("/", "a b", "/a b"),
    ("/a/b", "", "/a/b"),
    ("/a/b", "c", "/a/b/c"),
    ("/a/b", "../c", "/a/c"),
    ("/a/b", "../../..", "/"),
    ("/a/b", "/x/./y/", "/x/y"),
    ("/a/b", "./", "/a/b"),
    ("/a/b", "..", "/a"),
    ("/a/b", "c/..", "/a/b"),
    ("/a/b", "a//b", "/a/b/a/b"),
    ("/a/b", "/", "/"),
    ("/a/b", "//x", "/x"),
    ("/a/b", "...", "/a/b/..."),
    ("/a/b", ".hidden", "/a/b/.hidden"),
    ("/a/b", "a/../../b", "/a/b"),
    ("/a/b", "é/ü", "/a/b/é/ü"),
    ("/a/b", "a b", "/a/b/a b"),
    ("/a/b/", "", "/a/b"),
    ("/a/b/", "c", "/a/b/c"),
    ("/a/b/", "../c", "/a/c"),
    ("/a/b/", "../../..", "/"),
    ("/a/b/", "/x/./y/", "/x/y"),
    ("/a/b/", "./", "/a/b"),
    ("/a/b/", "..", "/a"),
    ("/a/b/", "c/..", "/a/b"),
    ("/a/b/", "a//b", "/a/b/a/b"),
    ("/a/b/", "/", "/"),
    ("/a/b/", "//x", "/x"),
    ("/a/b/", "...", "/a/b/..."),
    ("/a/b/", ".hidden", "/a/b/.hidden"),
    ("/a/b/", "a/../../b", "/a/b"),
    ("/a/b/", "é/ü", "/a/b/é/ü"),
    ("/a/b/", "a b", "/a/b/a b"),
    ("//a//b", "", "/a/b"),
    ("//a//b", "c", "/a/b/c"),
    ("//a//b", "../c", "/a/c"),
    ("//a//b", "../../..", "/"),
    ("//a//b", "/x/./y/", "/x/y"),
    ("//a//b", "./", "/a/b"),
    ("//a//b", "..", "/a"),
    ("//a//b", "c/..", "/a/b"),
    ("//a//b", "a//b", "/a/b/a/b"),
    ("//a//b", "/", "/"),
    ("//a//b", "//x", "/x"),
    ("//a//b", "...", "/a/b/..."),
    ("//a//b", ".hidden", "/a/b/.hidden"),
    ("//a//b", "a/../../b", "/a/b"),
    ("//a//b", "é/ü", "/a/b/é/ü"),
    ("//a//b", "a b", "/a/b/a b"),
    ("/a/./b", "", "/a/b"),
    ("/a/./b", "c", "/a/b/c"),
    ("/a/./b", "../c", "/a/c"),
    ("/a/./b", "../../..", "/"),
    ("/a/./b", "/x/./y/", "/x/y"),
    ("/a/./b", "./", "/a/b"),
    ("/a/./b", "..", "/a"),
    ("/a/./b", "c/..", "/a/b"),
    ("/a/./b", "a//b", "/a/b/a/b"),
    ("/a/./b", "/", "/"),
    ("/a/./b", "//x", "/x"),
    ("/a/./b", "...", "/a/b/..."),
    ("/a/./b", ".hidden", "/a/b/.hidden"),
    ("/a/./b", "a/../../b", "/a/b"),
    ("/a/./b", "é/ü", "/a/b/é/ü"),
    ("/a/./b", "a b", "/a/b/a b"),
];

/// `path.dirname(path)` of Node v26.12.1 (POSIX): path, result.
const NODE_DIRNAME: &[(&str, &str)] = &[
    ("/", "/"),
    ("/a", "/"),
    ("/a/", "/"),
    ("/a/b", "/a"),
    ("/a/b/", "/a"),
    ("/a//b", "/a/"),
    ("//a", "//"),
    ("///a", "//"),
    ("/a/./b", "/a/."),
    ("/a/b/..", "/a/b"),
    ("/a/b//", "/a"),
    ("//", "/"),
    ("///", "/"),
    ("/a/b/c", "/a/b"),
    ("/é/ü", "/é"),
    ("/a b/c", "/a b"),
];

/// `start-session` with `envs` in the caller's environment.
fn start_with(sandbox: &Sandbox, envs: &[(&str, &OsStr)], args: &[&str]) -> Run {
    let mut command = sandbox.command();
    for (name, value) in envs {
        command.env(name, value);
    }
    command
        .args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ])
        .args(args);
    run(&mut command)
}

#[test]
fn env_returns_the_callers_value_of_a_base_name_or_a_granted_name_and_unit_when_unset() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    support::write(
        &sandbox.home.join(".config/ravel/settings.json"),
        r#"{"configEnv": ["SEAM_SETTING", "SEAM_SETTING_UNSET", "SEAM_BOTH", "SEAM_BOTH"]}"#,
    );
    sandbox.personal_rhai(&selecting(
        r#"    let names = ["HOME", "TMPDIR", "LANG", "LC_ALL", "LC_SEAM_ANY", "LC_MESSAGES", "SEAM_FLAG",
                 "SEAM_SETTING", "SEAM_BOTH", "SEAM_FLAG_UNSET", "SEAM_SETTING_UNSET", "SEAM_EMPTY"];
    let args = [];
    for name in names {
        let value = ravel::env(name);
        args.push(if type_of(value) == "()" { name + " is unset" } else { name + "=" + value });
    }"#,
    ));

    let run = start_with(
        &sandbox,
        &[
            ("LANG", OsStr::new("en_AU.UTF-8")),
            ("LC_ALL", OsStr::new("C")),
            ("LC_SEAM_ANY", OsStr::new("any category")),
            ("SEAM_FLAG", OsStr::new("from the flag")),
            ("SEAM_SETTING", OsStr::new("from the setting")),
            ("SEAM_BOTH", OsStr::new("granted twice")),
            ("SEAM_EMPTY", OsStr::new("")),
        ],
        &[
            "--config-env",
            "SEAM_FLAG",
            "--config-env",
            "SEAM_BOTH",
            "--config-env",
            "SEAM_FLAG_UNSET",
            "--config-env",
            "SEAM_EMPTY",
        ],
    );
    let report = run.report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            &format!("HOME={}", text(&sandbox.home)),
            &format!("TMPDIR={}", text(&sandbox.tmp)),
            "LANG=en_AU.UTF-8",
            "LC_ALL=C",
            "LC_SEAM_ANY=any category",
            "LC_MESSAGES is unset",
            "SEAM_FLAG=from the flag",
            "SEAM_SETTING=from the setting",
            "SEAM_BOTH=granted twice",
            "SEAM_FLAG_UNSET is unset",
            "SEAM_SETTING_UNSET is unset",
            "SEAM_EMPTY=",
        ]
    );
    // The notice shows grants and never a value.
    assert_eq!(
        report["configEnvironment"],
        json!([
            {"name": "SEAM_SETTING", "set": true},
            {"name": "SEAM_SETTING_UNSET", "set": false},
            {"name": "SEAM_BOTH", "set": true},
            {"name": "SEAM_FLAG", "set": true},
            {"name": "SEAM_FLAG_UNSET", "set": false},
            {"name": "SEAM_EMPTY", "set": true}
        ])
    );
    assert!(!run.stderr.contains("from the flag"), "{}", run.stderr);
    assert!(!report.to_string().contains("granted twice"));
}

#[test]
fn env_throws_env_not_granted_for_any_other_name_even_one_the_caller_has_set() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    for name in ["SEAM_SECRET", "SEAM_NEVER_SET", "BUN_OPTIONS", "RAVEL_RUN_ID", "HOM", "home",
                 "LC", "", "HOME=x", "PATH ", "ANTHROPIC_API_KEY"] {
        args.push(caught(|| ravel::env(name)));
    }
    try { ravel::env("SEAM_SECRET"); } catch (error) {
        args.push(error.name);
        args.push(if error.message.contains("topsecret") { "leaks" } else { "clean" });
        args.push(if error.message.contains("SEAM_SECRET") { "names it" } else { "silent" });
    }"#,
    ));

    let run = start_with(
        &sandbox,
        &[
            ("SEAM_SECRET", OsStr::new("topsecret")),
            ("BUN_OPTIONS", OsStr::new("--preload x")),
            ("ANTHROPIC_API_KEY", OsStr::new("topsecret")),
        ],
        &[],
    );
    run.report();

    let mut expected = vec!["env_not_granted"; 11];
    expected.extend(["SEAM_SECRET", "clean", "names it"]);
    assert_eq!(recorded.argv().unwrap(), expected);
}

#[test]
fn env_throws_env_not_utf8_for_a_value_that_a_rhai_string_cannot_hold() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&selecting(
        r#"    let args = [caught(|| ravel::env("SEAM_BYTES")), ravel::env("SEAM_TEXT")];"#,
    ));

    start_with(
        &sandbox,
        &[
            ("SEAM_BYTES", OsStr::from_bytes(b"caf\xe9")),
            ("SEAM_TEXT", OsStr::new("café")),
        ],
        &["--config-env", "SEAM_BYTES", "--config-env", "SEAM_TEXT"],
    )
    .report();

    assert_eq!(recorded.argv().unwrap(), ["env_not_utf8", "café"]);
}

#[test]
fn env_works_in_load_context_and_at_the_top_level_of_a_module() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    support::write(
        &sandbox.home.join(".config/ravel/lib/seen.rhai"),
        r#"export const LANG_AT_LOAD = ravel::env("LANG");"#,
    );
    sandbox.personal_rhai(&policy(
        r#"import "lib/seen" as seen;
fn load_context(request) { #{ schemaVersion: 1, facts: #{ lang: ravel::env("LANG") } } }
fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [seen::LANG_AT_LOAD, context.facts.lang],
       provider: "p", model: "m", effort: "e", reason: "r" }
}"#,
    ));

    start_with(&sandbox, &[("LANG", OsStr::new("C.UTF-8"))], &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["C.UTF-8", "C.UTF-8"]);
}

#[test]
fn a_malformed_argument_to_a_data_helper_ends_the_evaluation_and_names_the_function_and_field() {
    for (call, function, field) in [
        ("ravel::json_parse(5)", "ravel::json_parse", "text"),
        ("ravel::json_parse(())", "ravel::json_parse", "text"),
        (
            "ravel::path_resolve(5, \"x\")",
            "ravel::path_resolve",
            "base",
        ),
        (
            "ravel::path_resolve(\"/\", 5)",
            "ravel::path_resolve",
            "path",
        ),
        ("ravel::path_parent([])", "ravel::path_parent", "path"),
        ("ravel::env(5)", "ravel::env", "name"),
        ("ravel::env(())", "ravel::env", "name"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        // `try` does not catch the end.
        let source = selecting(&format!(
            "    let args = [];\n    try {{\n        {call};\n    }} catch (error) {{ args.push(\"caught\"); }}"
        ));
        let line = source.lines().position(|line| line.contains(call)).unwrap() + 1;
        sandbox.personal_rhai(&source);

        let error = refused(&start(&sandbox, &[]), "host_request_invalid", &recorded);

        let location = error["location"].as_str().unwrap();
        assert!(
            location.starts_with(&format!("{function}({field}) at {line}:")),
            "{call}: {error}"
        );
        assert_eq!(error["stage"], "selection", "{call}: {error}");
    }
}

// What Grove's helpers need (`docs/specs/policy-evaluation.md`, *Migration*), each
// proved with a configuration script of this file and no Grove code.

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

const TASK_FILE: &str = "# rhai-data-helpers-k13\n\n**Integrates:** rhai-measured-reads-k52\n\n## Goal\n\nAdd the pure data helpers.\n\n```json\n{\n  \"b\": [1, 2],\n  \"a\": {\"y\": \"é\", \"x\": null}\n}\n```\n";

#[test]
fn grove_needs_a_measured_read_of_a_task_file_within_its_limits() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let path = sandbox.file(".grove/11-k11/04-impl--rhai-data-helpers-k13.md", TASK_FILE);
    sandbox.personal_rhai(&selecting(
        r#"    let task = ravel::read_text(".grove/11-k11/04-impl--rhai-data-helpers-k13.md", 4096);
    let args = [task.text.split("\n")[0], task.source.bytes.to_string()];"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["# rhai-data-helpers-k13", &TASK_FILE.len().to_string()]
    );
    assert_eq!(
        report["reads"],
        json!([{
            "via": "read_text", "phase": "selection",
            "name": text(&fs::canonicalize(&path).unwrap()),
            "bytes": TASK_FILE.len(), "sha256": sha256(TASK_FILE.as_bytes()),
        }])
    );

    // A task file past the limit that the configuration script states ends the evaluation.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("task.md", TASK_FILE);
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    try { ravel::read_text("task.md", 20); args.push("read"); } catch (error) { args.push("caught"); }"#,
    ));
    refused(&start(&sandbox, &[]), "source_too_large", &recorded);
}

#[test]
fn grove_needs_json_parse_and_canonical_json_encode_with_a_fixed_member_order_joined_by_the_caller()
{
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("task.md", TASK_FILE);
    sandbox.personal_rhai(&selecting(
        r#"    let text = ravel::read_text("task.md").text;
    let start = text.index_of("```json\n") + 8;
    let document = ravel::json_parse(text.sub_string(start, text.index_of("\n```", start) - start));
    // Grove's receipt has a fixed member order, so its port joins encoded members.
    let receipt = "{" + `"provider":${ravel::json_encode("p")}` + `,"b":${ravel::json_encode(document.b)}`
                + `,"a":${ravel::json_encode(document.a)}` + "}";
    let args = [ravel::json_encode(document), receipt];"#,
    ));

    start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            r#"{"a":{"x":null,"y":"é"},"b":[1,2]}"#,
            r#"{"provider":"p","b":[1,2],"a":{"x":null,"y":"é"}}"#,
        ]
    );
}

#[test]
fn grove_needs_string_functions_to_parse_a_handle_and_a_marker_line() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("task.md", TASK_FILE);
    sandbox.personal_rhai(&selecting_with(
        r#"fn parse_handle(text) {
    let text = text;
    text.trim();
    let parts = text.split("-");
    let key = parts[parts.len() - 1];
    if parts.len() < 2 || !key.starts_with("k") || key.len() < 2 { return (); }
    let digits = key.sub_string(1);
    for digit in digits.chars() { if !"0123456789".contains(digit) { return (); } }
    #{ slug: text.sub_string(0, text.len() - key.len() - 1), key: parse_int(digits) }
}"#,
        r#"    let lines = ravel::read_text("task.md").text.split("\n");
    let handle = parse_handle(lines[0].sub_string(2));
    let marker = "";
    for line in lines {
        if line.starts_with("**Integrates:**") { marker = line.sub_string(15); marker.trim(); }
    }
    let target = parse_handle(marker);
    let args = [handle.slug, handle.key.to_string(), marker, target.slug, target.key.to_string(),
                type_of(parse_handle("no-handle")), type_of(parse_handle("x-k"))];"#,
    ));

    start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "rhai-data-helpers",
            "13",
            "rhai-measured-reads-k52",
            "rhai-measured-reads",
            "52",
            "()",
            "()",
        ]
    );
}

#[test]
fn grove_needs_a_random_uuid_that_differs_between_evaluations() {
    let uuid = || {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        sandbox.personal_rhai(&selecting("    let args = [ravel::uuid_v4()];"));
        start(&sandbox, &[]).report();
        recorded.argv().unwrap().remove(0)
    };
    let (first, second) = (uuid(), uuid());

    assert_ne!(first, second);
    for id in [&first, &second] {
        let bytes = id.as_bytes();
        assert_eq!(bytes.len(), 36, "{id}");
        assert_eq!(bytes[14], b'4', "{id}");
        assert!(b"89ab".contains(&bytes[19]), "{id}");
        assert!(
            id.chars()
                .all(|c| c == '-' || c.is_ascii_digit() || ('a'..='f').contains(&c)),
            "{id}"
        );
    }
}

#[test]
fn grove_needs_a_wrapper_policy_that_imports_a_module_calls_its_select_and_changes_the_selection() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    support::write(
        &sandbox.home.join(".config/ravel/inner.rhai"),
        &format!(
            "{}fn select(request, context) {{\n    #{{ status: \"selected\", program: \"recording-agent\", \
             args: [request.prompt], provider: \"inner\", model: \"inner-model\", effort: \"low\", \
             reason: \"inner\" }}\n}}\n",
            rhai::HEADER
        ),
    );
    sandbox.personal_rhai(&policy(
        r#"import "inner" as inner;
fn select(request, context) {
    let selection = inner::select(request, context);
    selection.args.push("added by the wrapper");
    selection.model = "wrapped-model";
    selection.reason = "wrapped " + selection.reason;
    selection
}"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["p", "added by the wrapper"]);
    assert_eq!(
        report["selection"],
        json!({ "provider": "inner", "model": "wrapped-model", "effort": "low",
                "reason": "wrapped inner" })
    );
    let modules = report["config"]["modules"].as_array().unwrap();
    assert_eq!(modules.len(), 2, "{modules:?}");
}

#[test]
fn grove_needs_a_module_import_by_a_relative_or_an_absolute_path() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(&config.join("lib/deep/one.rhai"), "fn name() { \"one\" }");
    let elsewhere = sandbox.root.join("elsewhere/two.rhai");
    support::write(&elsewhere, "fn name() { \"two\" }");
    sandbox.personal_rhai(&selecting(&format!(
        r#"    import "lib/deep/one" as one;
    import "{}" as two;
    let args = [one::name(), two::name()];"#,
        elsewhere.display()
    )));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["one", "two"]);
    assert_eq!(report["config"]["modules"].as_array().unwrap().len(), 3);
}
