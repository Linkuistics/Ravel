//! A Rhai policy through the real `ravel start-session`
//! (`docs/specs/policy-evaluation.md`, *Policy entry and discovery*, *The
//! policy module*, *Request, context and selection* and *Diagnostics, notices
//! and results*).
//!
//! Everything runs in a sandbox home, with fake harness scripts on a private
//! `PATH`. The tests of the TypeScript path stay in `cli.rs` and `hostile.rs`.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;

use serde_json::{Value, json};
use support::rhai::{self, Recorded, policy, routing};
use support::{Run, Sandbox, text};

/// The refusal document of `run`, which must have failed with `code` and
/// launched no harness.
fn refused(run: &Run, code: &str, recorded: Option<&Recorded>) -> Value {
    let document = run.refusal(3);
    assert_eq!(document["schemaVersion"], 1, "{document}");
    assert_eq!(document["error"]["code"], code, "{document}");
    assert_eq!(document["error"]["exit"], 3, "{document}");
    if let Some(recorded) = recorded {
        assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    }
    document["error"].clone()
}

fn start(sandbox: &Sandbox, args: &[&str]) -> Run {
    let mut all = vec!["--kind", "check", "--json"];
    all.extend_from_slice(args);
    sandbox.start(&all)
}

#[test]
fn a_route_table_selects_and_the_harness_gets_the_argument_vector_of_the_table() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let policy_path = sandbox.personal_rhai(&policy(
        r#"const TABLE = #{ check: ["--print", "--model", "fast", "two words"] };
print("printed by the policy");
fn select(request, context) {
    debug("debugged by the policy");
    #{ status: "selected", program: "recording-agent", args: global::TABLE[request.kind],
       provider: "table", model: "m", effort: "e", reason: "route table" }
}"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["--print", "--model", "fast", "two words"]
    );
    assert_eq!(report["schemaVersion"], 2);
    assert_eq!(
        report["selection"],
        json!({ "provider": "table", "model": "m", "effort": "e", "reason": "route table" })
    );
    let canonical = fs::canonicalize(&policy_path).unwrap();
    assert_eq!(report["policy"]["path"], text(&canonical));
    assert_eq!(report["policy"]["version"], "rhai-seam-1");
    // The members that exist: no Bun.
    let worker = report["worker"].as_object().unwrap();
    let mut names: Vec<_> = worker.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(names, ["buildId", "path", "rhaiVersion", "version"]);
    assert_eq!(worker["rhaiVersion"], "1.27.0");
    assert_eq!(worker["version"], env!("CARGO_PKG_VERSION"));
    assert!(report.get("modules").is_none(), "modules in {report}");
    // The policy ran no harness, so the notice lists none.
    assert_eq!(report["harnessRuns"], json!([]));
    // The policy made no host call, so it read nothing, and the entry is its only
    // module. The notice has no member for an LLM call, which is a harness run.
    assert_eq!(report["reads"], json!([]));
    assert!(report.get("llmCalls").is_none(), "llmCalls in {report}");
    // The bounds hold the harness-run budget and no LLM budget.
    let bounds = report["bounds"].as_object().unwrap();
    assert!(bounds.contains_key("harnessRuns"), "{report}");
    for retired in ["llmCalls", "llmOutputTokens"] {
        assert!(!bounds.contains_key(retired), "{retired} in {report}");
    }
    let modules = report["policy"]["modules"].as_array().unwrap();
    assert_eq!(modules.len(), 1, "{report}");
    assert_eq!(modules[0]["name"], text(&canonical));
    assert_eq!(report["diagnostics"]["stdout"], "printed by the policy\n");
    // `debug` names its position, so only its text is exact.
    let stderr = report["diagnostics"]["stderr"].as_str().unwrap();
    assert!(stderr.contains("debugged by the policy"), "{stderr}");
}

#[test]
fn the_request_has_every_field_and_an_absent_optional_one_is_unit() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&routing(
        r#"[request.schemaVersion.to_string(), request.kind, request.prompt, request.cwd,
            request.params.mode, type_of(request.taskFile), request.taskId,
            type_of(request.context), request.limits.selectionMs.to_string(),
            request.limits.harnessRuns.to_string()]"#,
    ));

    start(&sandbox, &["--param", "mode=fast", "--task-id", "T-9"]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "3",
            "check",
            "p",
            &text(&sandbox.cwd),
            "fast",
            "()",
            "T-9",
            "()",
            "30000",
            "0",
        ]
    );
}

#[test]
fn the_caller_context_reaches_select_with_its_measurement() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    let where_from = if request.context.contains("measured") { "in request" } else { "not in request" };
    let args = [context.facts.answer, context.measured[0].via,
                context.measured[0].bytes.to_string(), where_from];
    #{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }
}"#,
    ));
    let document = r#"{"schemaVersion":1,"facts":{"answer":"from context"}}"#;
    let file = sandbox.file("context.json", document);

    let report = start(&sandbox, &["--context", &text(&file)]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "from context",
            "--context",
            &document.len().to_string(),
            "not in request",
        ]
    );
    assert_eq!(report["context"]["sources"][0]["via"], "--context");
    assert_eq!(
        report["context"]["value"]["facts"]["answer"],
        "from context"
    );
}

#[test]
fn an_unknown_field_refuses_with_its_location_and_launches_nothing() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let entry = sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [], provider: "p", model: "m",
       effort: "e", reason: "r", extra: 1 }
}"#,
    ));

    let error = refused(
        &start(&sandbox, &[]),
        "selection_malformed",
        Some(&recorded),
    );

    assert_eq!(error["stage"], "selection");
    assert_eq!(error["location"], "result.extra");
    assert_eq!(error["source"], text(&fs::canonicalize(entry).unwrap()));
    let remedy = error["remedy"].as_str().unwrap();
    assert!(remedy.contains("#{ status: \"selected\""), "{remedy}");
    assert!(!remedy.contains("sdk"), "{remedy}");
}

#[test]
fn a_value_that_cannot_cross_is_refused_at_its_location() {
    let sandbox = Sandbox::new();
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: ["ok", 'c'], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    ));
    let error = refused(&start(&sandbox, &[]), "selection_malformed", None);
    assert_eq!(error["location"], "result.args[1]");
}

#[test]
fn a_policy_that_refuses_is_a_policy_refusal_with_its_own_code() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    #{ status: "refused", code: "no_route", message: "no route for " + request.kind,
       remedy: "add one" }
}"#,
    ));
    let error = refused(&start(&sandbox, &[]), "policy_refused", Some(&recorded));
    assert_eq!(error["policyCode"], "no_route");
    assert_eq!(error["input"], "--kind check");
    assert_eq!(error["remedy"], "add one");
}

#[test]
fn a_parse_failure_names_the_file_the_line_and_the_column() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let entry = sandbox.personal_rhai(&policy("let x = ;"));

    let error = refused(
        &start(&sandbox, &[]),
        "policy_parse_failed",
        Some(&recorded),
    );

    assert_eq!(error["stage"], "load");
    assert_eq!(error["source"], text(&fs::canonicalize(entry).unwrap()));
    let location = error["location"].as_str().unwrap();
    let (line, column) = location.split_once(':').expect(location);
    assert_eq!(line, "3", "{location}");
    assert!(column.parse::<u32>().unwrap() > 0, "{location}");
}

#[test]
fn a_missing_export_a_wrong_version_and_a_blank_version_refuse_by_name() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();

    sandbox.personal_rhai("export const SCHEMA_VERSION = 3;\nexport const VERSION = \"v\";\n");
    let missing = refused(&start(&sandbox, &[]), "policy_invalid", Some(&recorded));
    assert_eq!(missing["stage"], "validation");
    assert!(missing["message"].as_str().unwrap().contains("select"));

    sandbox.personal_rhai(
        "export const SCHEMA_VERSION = 2;\nexport const VERSION = \"v\";\nfn select(r, c) { () }\n",
    );
    let wrong = refused(
        &start(&sandbox, &[]),
        "unsupported_version",
        Some(&recorded),
    );
    assert_eq!(wrong["stage"], "validation");
    // The position of the name in `export const SCHEMA_VERSION`.
    assert_eq!(wrong["location"], "1:14");

    sandbox.personal_rhai(
        "export const SCHEMA_VERSION = 3;\nexport const VERSION = \" \";\nfn select(r, c) { () }\n",
    );
    let blank = refused(&start(&sandbox, &[]), "policy_invalid", Some(&recorded));
    assert!(blank["message"].as_str().unwrap().contains("blank"));
}

#[test]
fn abstention_refuses_and_ravel_picks_nothing_in_its_place() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&policy("fn select(request, context) { () }"));
    let error = refused(
        &start(&sandbox, &[]),
        "selection_abstained",
        Some(&recorded),
    );
    assert_eq!(error["location"], "result");
    assert!(error["message"].as_str().unwrap().contains("`()`"));
}

#[test]
fn an_uncaught_error_refuses_with_its_file_line_and_column() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let entry = sandbox.personal_rhai(&policy(
        "fn select(request, context) {\n    throw \"no route\";\n}",
    ));

    let error = refused(&start(&sandbox, &[]), "selection_threw", Some(&recorded));

    assert_eq!(error["stage"], "selection");
    assert_eq!(error["source"], text(&fs::canonicalize(entry).unwrap()));
    assert_eq!(error["location"], "4:5");
    assert!(error["message"].as_str().unwrap().contains("no route"));
}

#[test]
fn both_files_prefer_policy_rhai_and_never_open_policy_ts() {
    let sandbox = Sandbox::new();
    let typescript = sandbox.personal_policy("this is not a policy");
    // An unreadable policy.ts refuses if Ravel opens it at all.
    fs::set_permissions(&typescript, fs::Permissions::from_mode(0o000)).unwrap();
    let entry = sandbox.personal_rhai(rhai::ROUTED);

    let report = start(&sandbox, &[]).report();

    assert_eq!(report["schemaVersion"], 2);
    assert_eq!(
        report["policy"]["path"],
        text(&fs::canonicalize(entry).unwrap())
    );
}

#[test]
fn a_policy_ts_alone_and_every_other_extension_keep_the_typescript_path() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let alone = start(&sandbox, &[]).report();
    assert_eq!(alone["schemaVersion"], 1);
    assert!(alone["worker"]["bunVersion"].is_string(), "{alone}");
    assert!(alone["worker"].get("rhaiVersion").is_none());

    let other = sandbox.file("route.mts", support::ROUTED);
    let named = start(&sandbox, &["--config", &text(&other)]).report();
    assert_eq!(named["schemaVersion"], 1);
    assert!(named["worker"]["bunVersion"].is_string(), "{named}");
}

#[test]
fn a_rhai_file_given_to_config_takes_the_rhai_path_beside_a_policy_ts() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let named = sandbox.file("elsewhere/route.rhai", rhai::ROUTED);

    let report = start(&sandbox, &["--config", &text(&named)]).report();

    assert_eq!(report["schemaVersion"], 2);
    assert_eq!(
        report["policy"]["path"],
        text(&fs::canonicalize(named).unwrap())
    );
    // A relative path resolves against the caller's directory.
    let relative = start(&sandbox, &["--config", "elsewhere/route.rhai"]).report();
    assert_eq!(relative["policy"], report["policy"]);
}

#[test]
fn the_entry_must_exist_and_be_a_regular_file() {
    let sandbox = Sandbox::new();

    let neither = refused(&start(&sandbox, &[]), "policy_missing", None);
    assert_eq!(neither["stage"], "authority");
    assert!(neither["message"].as_str().unwrap().contains("policy.rhai"));
    assert_eq!(neither["source"], text(&sandbox.personal_rhai_path()));

    let named = refused(
        &start(&sandbox, &["--config", "absent.rhai"]),
        "policy_missing",
        None,
    );
    assert_eq!(named["source"], text(&sandbox.cwd.join("absent.rhai")));

    fs::create_dir(sandbox.cwd.join("directory.rhai")).unwrap();
    let directory = refused(
        &start(&sandbox, &["--config", "directory.rhai"]),
        "policy_unreadable",
        None,
    );
    assert_eq!(
        directory["source"],
        text(&sandbox.cwd.join("directory.rhai"))
    );
}

#[test]
fn a_dangling_policy_rhai_is_missing_and_does_not_fall_back_to_policy_ts() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let link = sandbox.personal_rhai_path();
    std::os::unix::fs::symlink(sandbox.root.join("nowhere.rhai"), &link).unwrap();

    let error = refused(&start(&sandbox, &[]), "policy_missing", None);

    assert_eq!(error["source"], text(&link));
}

#[test]
fn the_entry_must_be_utf_8_and_at_most_one_mebibyte() {
    let sandbox = Sandbox::new();

    fs::write(sandbox.cwd.join("bytes.rhai"), b"// \xff\n").unwrap();
    let bytes = refused(
        &start(&sandbox, &["--config", "bytes.rhai"]),
        "policy_unreadable",
        None,
    );
    assert!(bytes["message"].as_str().unwrap().contains("UTF-8"));

    // Exactly 1 MiB is admitted, and one byte more is not.
    let padded = |size: usize| {
        let mut source = rhai::ROUTED.to_owned();
        source.push_str("// ");
        let fill = size - source.len() - 1;
        source.push_str(&"x".repeat(fill));
        source.push('\n');
        assert_eq!(source.len(), size);
        source
    };
    sandbox.file("exact.rhai", &padded(1_048_576));
    start(&sandbox, &["--config", "exact.rhai"]).report();

    sandbox.file("over.rhai", &padded(1_048_577));
    let over = refused(
        &start(&sandbox, &["--config", "over.rhai"]),
        "policy_unreadable",
        None,
    );
    assert!(
        over["message"].as_str().unwrap().contains("1048576"),
        "{over}"
    );
}

#[test]
fn a_symlink_is_admitted_by_its_real_file() {
    let sandbox = Sandbox::new();
    let real = sandbox.file("real/policy-real.rhai", rhai::ROUTED);
    std::os::unix::fs::symlink(&real, sandbox.cwd.join("link.rhai")).unwrap();

    let report = start(&sandbox, &["--config", "link.rhai"]).report();

    assert_eq!(
        report["policy"]["path"],
        text(&fs::canonicalize(real).unwrap())
    );
}

#[test]
fn a_policy_that_names_no_program_on_the_path_refuses_at_resolution() {
    let sandbox = Sandbox::new();
    sandbox.personal_rhai(&policy(
        r#"fn select(request, context) {
    #{ status: "selected", program: "no-such-harness", args: [], provider: "p", model: "m",
       effort: "e", reason: "r" }
}"#,
    ));
    let run = start(&sandbox, &[]);
    let document = run.refusal(127);
    assert_eq!(document["error"]["stage"], "resolution", "{document}");
}
