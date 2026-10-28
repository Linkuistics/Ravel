//! Public configuration diagnostics through the real CLI in a private home.

mod support;

use serde_json::json;
use support::rhai::{policy, routing};
use support::{Sandbox, text};

const OWNER_TEXT: &str = "policy_owner policy.select policyCode";

#[test]
fn configuration_refusals_use_schema_two_and_preserve_owner_values() {
    for loader in [false, true] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let function = if loader {
            "load_context(request)"
        } else {
            "select(request, context)"
        };
        sandbox.personal_rhai(&policy(&(format!(
            "fn {function} {{ #{{ status: \"refused\", code: {OWNER_TEXT:?}, message: {OWNER_TEXT:?}, remedy: {OWNER_TEXT:?} }} }}"
        ) + if loader { "\nfn select(request, context) { throw \"never select\"; }" } else { "" })));
        let document = sandbox.start(&["--kind", OWNER_TEXT, "--json"]).refusal(3);
        assert_eq!(document["schemaVersion"], 2, "{document}");
        let error = &document["error"];
        assert_eq!(error["code"], "config_refused");
        assert_eq!(error["configCode"], OWNER_TEXT);
        assert!(error.get("policyCode").is_none());
        assert_eq!(error["remedy"], OWNER_TEXT);
        assert_eq!(error["input"], format!("--kind {OWNER_TEXT}"));
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .starts_with("the configuration script ")
        );
        assert!(error["message"].as_str().unwrap().ends_with(OWNER_TEXT));
        if loader {
            assert_eq!(error["location"], "config.load_context");
            assert_eq!(error["stage"], "context");
        }
        assert!(recorded.argv().is_none());
        assert_eq!(std::fs::read_dir(&sandbox.tmp).unwrap().count(), 0);

        let plain = sandbox.start(&["--kind", OWNER_TEXT]);
        assert_eq!(plain.code, Some(3));
        assert!(plain.stderr.contains("ravel: refused (config_refused,"));
        assert!(
            plain
                .stderr
                .contains(&format!("  config code: {OWNER_TEXT}\n"))
        );
    }
}

#[test]
fn notices_and_stream_prefixes_use_configuration_names_without_rewriting_output() {
    let sandbox = Sandbox::new();
    sandbox.recording_agent();
    let source = routing("[request.schemaVersion.to_string(), request.prompt]").replace(
        "fn select(request, context) {",
        &format!("fn select(request, context) {{ print({OWNER_TEXT:?}); debug({OWNER_TEXT:?});"),
    );
    let path = sandbox.personal_rhai(&source);
    let report = sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(report["schemaVersion"], 2);
    assert_eq!(
        report["config"]["path"],
        text(&path.canonicalize().unwrap())
    );
    assert_eq!(report["configEnvironment"], json!([]));
    assert!(report["config"]["modules"].is_array());
    assert!(report.get("policy").is_none());
    assert!(report.get("policyEnvironment").is_none());
    assert_eq!(report["diagnostics"]["stdout"], format!("{OWNER_TEXT}\n"));
    let plain = sandbox.start(&["--kind", "check"]);
    assert_eq!(plain.code, Some(0), "{}", plain.stderr);
    assert_eq!(plain.stdout, "done");
    assert!(
        plain
            .stderr
            .contains(&format!("config stdout: {OWNER_TEXT}\n"))
    );
    assert!(
        plain
            .stderr
            .lines()
            .any(|line| line.starts_with("config stderr:") && line.contains(OWNER_TEXT))
    );
}

#[test]
fn entry_and_engine_refusals_use_configuration_codes() {
    for (source, expected) in [
        (None, "config_missing"),
        (Some("not valid Rhai !!!"), "config_parse_failed"),
        (
            Some("export const SCHEMA_VERSION = 3; export const VERSION = \"v\";"),
            "config_invalid",
        ),
        (
            Some("import \"missing\" as missing;"),
            "config_import_failed",
        ),
    ] {
        let sandbox = Sandbox::new();
        if let Some(source) = source {
            sandbox.personal_rhai(source);
        }
        let document = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
        assert_eq!(document["schemaVersion"], 2);
        assert_eq!(document["error"]["code"], expected, "{document}");
    }
}
