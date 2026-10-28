//! Measured reads, module imports and `load_context` of a Rhai configuration script through
//! the real `ravel start-session` (`docs/specs/configuration-evaluation.md`, *The
//! bridge between the script and the CLI*, *Configuration entry and discovery*,
//! *Request, context and selection* and *Measured reads*).
//!
//! Everything runs in a sandbox home with fake harness scripts on a private
//! `PATH`. The configuration script hands what it read to the recording harness as arguments,
//! so a test sees what the script saw.

mod support;

use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use support::rhai::{self, Recorded, rhai_script};
use support::{Run, Sandbox, text};

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

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// A `select` whose `body` computes `args`, an array of strings.
fn selecting(body: &str) -> String {
    rhai_script(&format!(
        r#"fn select(request, context) {{
{body}
    #{{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }}
}}"#
    ))
}

#[test]
fn a_read_in_select_returns_the_content_and_the_notice_lists_it_with_its_phase() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let notes = sandbox.file("notes.txt", "first line\nsecond line\n");
    sandbox.personal_rhai(&selecting(
        r#"    let read = ravel::read_text("notes.txt");
    let args = [read.text, read.source.name, read.source.bytes.to_string(), read.source.sha256];"#,
    ));

    let report = start(&sandbox, &[]).report();

    let canonical = text(&fs::canonicalize(&notes).unwrap());
    let content = "first line\nsecond line\n";
    assert_eq!(
        recorded.argv().unwrap(),
        // The text ends in a newline, so the recording has an empty third line.
        [
            "first line",
            "second line",
            "",
            &canonical,
            "23",
            &sha256(content.as_bytes())
        ]
    );
    assert_eq!(
        report["reads"],
        json!([{
            "via": "read_text", "phase": "selection", "name": canonical,
            "bytes": 23, "sha256": sha256(content.as_bytes()),
        }])
    );
    // A read in `select` is in the notice, and in no delivered context.
    assert_eq!(report["context"], Value::Null);
    let config_path = fs::canonicalize(sandbox.personal_rhai_path()).unwrap();
    let modules = &report["config"]["modules"];
    assert_eq!(modules.as_array().unwrap().len(), 1, "{modules}");
    assert_eq!(modules[0]["name"], text(&config_path));
}

#[test]
fn read_json_returns_the_value_and_a_relative_path_resolves_against_the_working_directory() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let document = r#"{"model":"fast","tags":["a","b"],"n":3}"#;
    sandbox.file("data/choice.json", document);
    sandbox.personal_rhai(&selecting(
        r#"    let read = ravel::read_json("data/choice.json");
    let args = [read.value.model, read.value.tags[1], (read.value.n + 1).to_string(),
                read.source.bytes.to_string()];"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["fast", "b", "4", &document.len().to_string()]
    );
    assert_eq!(report["reads"][0]["via"], "read_json");
}

#[test]
fn a_data_failure_throws_a_map_that_a_script_can_catch() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("dir/placeholder", "");
    sandbox.file("not-json.txt", "{ nope");
    fs::write(sandbox.cwd.join("binary.bin"), [0xff, 0xfe, 0x00]).unwrap();
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    for attempt in [["read_text", "absent.txt"], ["read_text", "dir"],
                    ["read_text", "binary.bin"], ["read_json", "not-json.txt"]] {
        try {
            if attempt[0] == "read_text" { ravel::read_text(attempt[1]); }
            else { ravel::read_json(attempt[1]); }
            args.push("no error");
        } catch (error) {
            args.push(error.code);
            args.push(if error.name.ends_with(attempt[1]) { "named" } else { error.name });
            args.push(if error.message.len() > 0 { "said" } else { "silent" });
        }
    }"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "source_missing",
            "named",
            "said",
            "source_not_regular",
            "named",
            "said",
            "source_not_utf8",
            "named",
            "said",
            "source_invalid_json",
            "named",
            "said",
        ]
    );
    // A failed read is no measured source.
    assert_eq!(report["reads"], json!([]));
}

#[test]
fn an_unreadable_file_is_a_data_failure() {
    use std::os::unix::fs::PermissionsExt as _;
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let secret = sandbox.file("secret.txt", "closed");
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();
    // A process with the privilege to read it anyway shows no failure to see.
    if fs::read(&secret).is_ok() {
        return;
    }
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    try { ravel::read_text("secret.txt"); } catch (error) { args.push(error.code); }"#,
    ));

    start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["source_unreadable"]);
}

/// Run `drive`, and say whether a blocking open of `fifo` for reading stalled
/// for as long as the test watched. This is the firing control of the FIFO case:
/// it shows the fixture is one that holds a reader.
fn a_blocking_open_stalls(fifo: &Path) -> bool {
    let (opened, waiting) = mpsc::channel();
    let path = fifo.to_owned();
    let reader = thread::spawn(move || {
        let _file = fs::File::open(&path);
        let _ = opened.send(());
    });
    let stalled = waiting.recv_timeout(Duration::from_millis(500)).is_err();
    // Release the reader: a nonblocking open for writing succeeds once a reader
    // is waiting, and the blocked open then returns.
    use std::os::unix::fs::OpenOptionsExt as _;
    let released = fs::OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(fifo);
    let _ = released;
    reader.join().unwrap();
    stalled
}

#[test]
fn a_fifo_that_no_process_writes_is_not_regular_at_once_and_a_blocking_open_of_it_stalls() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let fifo = sandbox.cwd.join("never-written");
    support::mkfifo(&fifo);
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    try { ravel::read_text("never-written"); args.push("read"); }
    catch (error) { args.push(error.code); }"#,
    ));

    let started = Instant::now();
    let report = start(&sandbox, &["--timeout-ms", "30000"]).report();

    assert_eq!(recorded.argv().unwrap(), ["source_not_regular"]);
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "the read of the FIFO took {:?}",
        started.elapsed()
    );
    assert_eq!(report["reads"], json!([]));
    // The firing control: the same FIFO holds a blocking open.
    assert!(
        a_blocking_open_stalls(&fifo),
        "a blocking open of the FIFO did not stall, so the case above proves nothing"
    );
}

#[test]
fn a_read_over_its_bound_ends_the_evaluation_although_the_script_catches_the_error() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let big = sandbox.file("big.txt", &"x".repeat(70_000));
    let config_text = selecting(
        r#"    let args = [];
    try { ravel::read_text("big.txt"); args.push("read"); }
    catch (error) { args.push("caught"); }"#,
    );
    sandbox.personal_rhai(&config_text);

    let error = refused(&start(&sandbox, &[]), "source_too_large", &recorded);

    assert_eq!(error["stage"], "context", "{error}");
    assert_eq!(error["bound"]["bytes"], 65_536, "{error}");
    assert_eq!(
        error["source"],
        text(&fs::canonicalize(&big).unwrap()),
        "{error}"
    );

    // The control: a larger `max_bytes`, up to the context budget, reads it.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("big.txt", &"x".repeat(70_000));
    sandbox.personal_rhai(&selecting(
        r#"    let args = [ravel::read_text("big.txt", 100000).source.bytes.to_string()];"#,
    ));
    start(&sandbox, &[]).report();
    assert_eq!(recorded.argv().unwrap(), ["70000"]);
}

#[test]
fn a_max_bytes_above_the_context_budget_ends_the_evaluation() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("small.txt", "s");
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    try { ravel::read_text("small.txt", 262145); } catch (error) { args.push("caught"); }"#,
    ));

    let error = refused(&start(&sandbox, &[]), "source_too_large", &recorded);

    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("context budget of 262144"),
        "{error}"
    );
}

#[test]
fn the_bound_of_256_sources_counts_the_callers_document_and_every_read() {
    let reads = r#"    let args = [];
    for i in 0..256 {
        try { ravel::read_text("f.txt"); } catch (error) { args.push("caught"); }
    }
    args.push("all read");"#;
    // The control: 256 reads and no caller document are 256 sources.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("f.txt", "f");
    sandbox.personal_rhai(&selecting(reads));
    let report = start(&sandbox, &[]).report();
    assert_eq!(recorded.argv().unwrap(), ["all read"]);
    assert_eq!(report["reads"].as_array().unwrap().len(), 256);

    // The caller's document is the 257th source with them.
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("f.txt", "f");
    let document = sandbox.file("context.json", r#"{"schemaVersion":1}"#);
    sandbox.personal_rhai(&selecting(reads));
    let run = start(&sandbox, &["--context", &text(&document)]);
    let error = refused(&run, "too_many_sources", &recorded);
    assert_eq!(error["bound"]["sources"], 256, "{error}");
}

/// A `select` that reads `file` with `function` `count` times and catches each
/// data failure.
fn retrying(function: &str, file: &str, count: usize) -> String {
    selecting(&format!(
        r#"    let args = [];
    for i in 0..{count} {{
        try {{ ravel::{function}("{file}"); args.push("read"); }} catch (error) {{ }}
    }}
    args.push("done");"#
    ))
}

#[test]
fn a_read_whose_bytes_fail_to_decode_is_a_source_but_no_measured_read() {
    // Each row reads the same bad file again and again. The bound is 256
    // sources, with the caller's document as one of them when there is one.
    for (function, file, caller, count, accepted) in [
        ("read_json", "bad.json", false, 256, true),
        ("read_json", "bad.json", false, 257, false),
        ("read_text", "binary.bin", false, 256, true),
        ("read_text", "binary.bin", false, 257, false),
        ("read_json", "bad.json", true, 255, true),
        ("read_json", "bad.json", true, 256, false),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        sandbox.file("bad.json", "{ nope");
        fs::write(sandbox.cwd.join("binary.bin"), [0xff, 0xfe, 0x00]).unwrap();
        let document = sandbox.file("context.json", r#"{"schemaVersion":1}"#);
        sandbox.personal_rhai(&retrying(function, file, count));
        let context = text(&document);
        let args: &[&str] = if caller {
            &["--context", &context]
        } else {
            &[]
        };
        let case = format!("{function} {file} caller {caller} x{count}");

        let run = start(&sandbox, args);

        if accepted {
            let report = run.report();
            assert_eq!(recorded.argv().unwrap(), ["done"], "{case}");
            // A read that failed is no measured read.
            assert_eq!(report["reads"], json!([]), "{case}");
        } else {
            let error = refused(&run, "too_many_sources", &recorded);
            assert_eq!(error["bound"]["sources"], 256, "{case}: {error}");
        }
    }
}

#[test]
fn a_read_that_finds_no_file_is_no_source() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&retrying("read_text", "absent.txt", 300));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["done"]);
    assert_eq!(report["reads"], json!([]));
}

#[test]
fn a_json_file_nests_at_most_100_levels_and_a_deeper_one_is_a_data_failure() {
    let nested = |levels: usize| format!("{}0{}", "[".repeat(levels), "]".repeat(levels));
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    // 127 levels fit the parser of the file but not the parser of the reply,
    // which has the reply object around the value.
    for levels in [100, 101, 127, 128] {
        sandbox.file(&format!("d{levels}.json"), &nested(levels));
    }
    sandbox.personal_rhai(&selecting(
        r#"    let args = [];
    for name in ["d100.json", "d101.json", "d127.json", "d128.json"] {
        try {
            ravel::read_json(name);
            args.push("read");
        } catch (error) {
            args.push(error.code);
            args.push(if error.message.contains("deeper than 100 levels") { "deep" } else { "other" });
        }
    }"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "read",
            "source_invalid_json",
            "deep",
            "source_invalid_json",
            "deep",
            "source_invalid_json",
            "other",
        ]
    );
    // Only the file that fits is a measured read.
    assert_eq!(report["reads"].as_array().unwrap().len(), 1);
}

#[test]
fn a_malformed_host_request_ends_the_evaluation_and_names_the_function_the_field_and_the_line() {
    for (call, field) in [
        ("ravel::read_text(5)", "path"),
        ("ravel::read_text(\"\")", "path"),
        ("ravel::read_json(\"f.txt\", 0)", "max_bytes"),
        ("ravel::read_text(\"f.txt\", \"many\")", "max_bytes"),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        sandbox.file("f.txt", "{}");
        // `try` does not catch the end.
        sandbox.personal_rhai(&selecting(&format!(
            "    let args = [];\n    try {{\n        {call};\n    }} catch (error) {{ args.push(\"caught\"); }}"
        )));

        let error = refused(&start(&sandbox, &[]), "host_request_invalid", &recorded);

        let function = call.split('(').next().unwrap();
        let location = error["location"].as_str().unwrap();
        assert!(
            location.starts_with(&format!("{function}({field}) at 6:")),
            "{call}: {error}"
        );
        assert_eq!(error["stage"], "selection", "{call}: {error}");
    }
}

#[test]
fn an_import_resolves_from_the_importing_file_appends_rhai_and_loads_each_module_once() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(
        &config.join("lib/helper.rhai"),
        r#"print("helper loads");
import "sibling" as sibling;
fn shout(word) { sibling::wrap(word.to_upper()) }"#,
    );
    support::write(
        &config.join("lib/sibling.rhai"),
        "print(\"sibling loads\");\nfn wrap(word) { \"<\" + word + \">\" }",
    );
    // An absolute path is accepted, and the same module is not loaded twice.
    let absolute = fs::canonicalize(config.join("lib/sibling.rhai")).unwrap();
    sandbox.personal_rhai(&rhai_script(&format!(
        r#"import "lib/helper" as helper;
import "{}" as again;
fn select(request, context) {{
    #{{ status: "selected", program: "recording-agent",
       args: [helper::shout("hi"), again::wrap("x")],
       provider: "p", model: "m", effort: "e", reason: "r" }}
}}"#,
        absolute.display()
    )));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["<HI>", "<x>"]);
    assert_eq!(
        report["diagnostics"]["stdout"], "helper loads\nsibling loads\n",
        "each module runs its top level once"
    );
    let modules = report["config"]["modules"].as_array().unwrap();
    let canonical = |relative: &str| text(&fs::canonicalize(config.join(relative)).unwrap());
    let names: Vec<_> = modules
        .iter()
        .map(|module| module["name"].clone())
        .collect();
    assert_eq!(
        names,
        [
            json!(canonical("config.rhai")),
            json!(canonical("lib/helper.rhai")),
            json!(canonical("lib/sibling.rhai")),
        ]
    );
    let helper = fs::read(config.join("lib/helper.rhai")).unwrap();
    assert_eq!(modules[1]["bytes"], helper.len());
    assert_eq!(modules[1]["sha256"], sha256(&helper));
}

#[test]
fn a_wrapper_config_imports_another_and_changes_its_selection() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(
        &config.join("base.rhai"),
        r#"fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [request.prompt],
       provider: "base", model: "m", effort: "e", reason: "base route" }
}"#,
    );
    sandbox.personal_rhai(&rhai_script(
        r#"import "base" as base;
fn select(request, context) {
    let chosen = base::select(request, context);
    chosen.args.push("added by the wrapper");
    chosen.provider = "wrapper";
    chosen
}"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["p", "added by the wrapper"]);
    assert_eq!(report["selection"]["provider"], "wrapper");
}

#[test]
fn a_module_reads_at_its_top_level_and_in_its_functions_with_the_phase_of_each() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    sandbox.file("table.json", r#"{"route":"fast"}"#);
    sandbox.file("late.txt", "late");
    support::write(
        &config.join("reader.rhai"),
        r#"export const route = ravel::read_json("table.json").value.route;
fn late() { ravel::read_text("late.txt").text }"#,
    );
    sandbox.personal_rhai(&rhai_script(
        r#"import "reader" as reader;
fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [reader::route, reader::late()],
       provider: "p", model: "m", effort: "e", reason: "r" }
}"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["fast", "late"]);
    let reads: Vec<_> = report["reads"]
        .as_array()
        .unwrap()
        .iter()
        .map(|read| {
            (
                read["via"].as_str().unwrap(),
                read["phase"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(reads, [("read_json", "load"), ("read_text", "selection")]);
}

#[test]
fn a_syntax_error_in_a_module_refuses_as_config_parse_failed_and_names_the_module() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(&config.join("broken.rhai"), "fn oops( {\n}\n");
    sandbox.personal_rhai(&rhai_script(
        "import \"broken\" as broken;\nfn select(request, context) { () }",
    ));

    let error = refused(&start(&sandbox, &[]), "config_parse_failed", &recorded);

    let module = fs::canonicalize(config.join("broken.rhai")).unwrap();
    assert_eq!(error["source"], text(&module), "{error}");
    assert_eq!(error["stage"], "load", "{error}");
    assert!(
        error["location"].as_str().unwrap().starts_with("1:"),
        "{error}"
    );
}

#[test]
fn a_missing_module_a_cycle_and_a_failing_top_level_refuse_as_config_import_failed() {
    let entry = "import \"m\" as m;\nfn select(request, context) { () }";
    // (label, the module's source, a fragment of the message, the file named)
    for (label, module, fragment, names_module) in [
        ("missing", None, "cannot be found", false),
        (
            "cycle",
            Some("import \"config\" as back;"),
            "closes a cycle",
            true,
        ),
        (
            "failing top level",
            Some("throw \"module failed\";"),
            "module failed",
            true,
        ),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let config = sandbox.home.join(".config/ravel");
        if let Some(module) = module {
            support::write(&config.join("m.rhai"), module);
        }
        sandbox.personal_rhai(&rhai_script(entry));

        let error = refused(&start(&sandbox, &[]), "config_import_failed", &recorded);

        assert_eq!(error["stage"], "load", "{label}: {error}");
        assert!(
            error["message"].as_str().unwrap().contains(fragment),
            "{label}: {error}"
        );
        let named = fs::canonicalize(config.join("m.rhai"))
            .map(|path| text(&path))
            .unwrap_or_default();
        assert_eq!(
            error["source"] == json!(named),
            names_module,
            "{label}: {error}"
        );
    }
}

#[test]
fn a_cycle_through_two_modules_is_found_and_the_error_names_the_chain() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(&config.join("a.rhai"), "import \"b\" as b;");
    support::write(&config.join("b.rhai"), "import \"a\" as a;");
    sandbox.personal_rhai(&rhai_script(
        "import \"a\" as a;\nfn select(request, context) { () }",
    ));

    let error = refused(&start(&sandbox, &[]), "config_import_failed", &recorded);

    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains("a.rhai -> ")
            && message.contains("b.rhai -> ")
            && message.contains("closes a cycle"),
        "{error}"
    );
}

#[test]
fn the_64th_module_loads_and_the_65th_refuses() {
    let imports = |count: usize| -> String {
        (0..count)
            .map(|index| format!("import \"m{index}\" as m{index};\n"))
            .collect()
    };
    for (count, accepted) in [(64, true), (65, false)] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let config = sandbox.home.join(".config/ravel");
        for index in 0..count {
            support::write(&config.join(format!("m{index}.rhai")), "");
        }
        sandbox.personal_rhai(&format!(
            "{}{}{}",
            rhai::HEADER,
            imports(count),
            rhai::ROUTED.lines().skip(2).collect::<Vec<_>>().join("\n")
        ));
        // `ROUTED` selects `fake-agent`, which the sandbox has.
        let run = start(&sandbox, &[]);
        if accepted {
            let report = run.report();
            // The entry and the 64 modules.
            assert_eq!(report["config"]["modules"].as_array().unwrap().len(), 65);
        } else {
            let error = refused(&run, "config_import_failed", &recorded);
            assert_eq!(error["stage"], "load", "{error}");
        }
    }
}

#[test]
fn the_bound_of_modules_counts_distinct_files_and_not_import_statements() {
    // Eight modules import seven leaves each: 64 files, and no file has more
    // than eight import statements, so only the front's count can refuse the
    // 65th file.
    let graph = |extra: bool| {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let config = sandbox.home.join(".config/ravel");
        let mut entry = String::new();
        for mid in 0..8 {
            let leaves: String = (0..7)
                .map(|leaf| {
                    support::write(&config.join(format!("leaf{mid}_{leaf}.rhai")), "");
                    format!("import \"leaf{mid}_{leaf}\" as leaf{leaf};\n")
                })
                .collect();
            support::write(&config.join(format!("mid{mid}.rhai")), &leaves);
            entry.push_str(&format!("import \"mid{mid}\" as mid{mid};\n"));
        }
        if extra {
            support::write(&config.join("extra.rhai"), "");
            entry.push_str("import \"extra\" as extra;\n");
        }
        sandbox.personal_rhai(&format!(
            "{}{entry}{}",
            rhai::HEADER,
            rhai::ROUTED.lines().skip(2).collect::<Vec<_>>().join("\n")
        ));
        (start(&sandbox, &[]), recorded)
    };

    let (run, _) = graph(false);
    assert_eq!(
        run.report()["config"]["modules"].as_array().unwrap().len(),
        65,
        "the entry and 64 modules"
    );
    let (run, recorded) = graph(true);
    let error = refused(&run, "config_import_failed", &recorded);
    assert_eq!(error["stage"], "load", "{error}");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("over the bound of 64"),
        "{error}"
    );
}

#[test]
fn an_import_of_a_loaded_module_is_not_a_new_module_however_often_it_runs() {
    // Rhai counts import statements, and counts a cached module again. The bound
    // is the spec's 65th module, so 70 statements for one file load it once, at
    // the top level and in a function that `select` runs.
    let statements = |prefix: &str| -> String {
        (0..70)
            .map(|index| format!("{prefix}import \"helper\" as helper{index};\n"))
            .collect()
    };
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let config = sandbox.home.join(".config/ravel");
    support::write(&config.join("helper.rhai"), "fn word() { \"w\" }");
    sandbox.personal_rhai(&rhai_script(&format!(
        r#"{}
fn select(request, context) {{
{}
    #{{ status: "selected", program: "recording-agent", args: [helper69::word()],
       provider: "p", model: "m", effort: "e", reason: "r" }}
}}"#,
        statements(""),
        statements("    ")
    )));

    let report = start(&sandbox, &[]).report();

    assert_eq!(recorded.argv().unwrap(), ["w"]);
    assert_eq!(report["config"]["modules"].as_array().unwrap().len(), 2);
}

#[test]
fn load_context_assembles_the_context_that_select_receives_with_its_measured_sources() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let task = sandbox.file("task.json", r#"{"title":"build the bridge"}"#);
    let extra = sandbox.file("extra.txt", "extra facts");
    sandbox.personal_rhai(&rhai_script(
        r#"fn load_context(request) {
    let task = ravel::read_json("task.json");
    let extra = ravel::read_text("extra.txt");
    #{ schemaVersion: 1, summary: task.value.title, facts: #{ extra: extra.text } }
}
fn select(request, context) {
    let args = [context.summary, context.facts.extra, context.measured.len().to_string(),
                context.measured[0].via, context.measured[1].via];
    #{ status: "selected", program: "recording-agent", args: args, provider: "p", model: "m",
       effort: "e", reason: "r" }
}"#,
    ));

    let report = start(&sandbox, &[]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        [
            "build the bridge",
            "extra facts",
            "2",
            "read_json",
            "read_text"
        ]
    );
    let context = &report["context"];
    assert_eq!(context["loader"], true, "{context}");
    assert_eq!(
        context["sources"][0]["name"],
        text(&fs::canonicalize(&task).unwrap())
    );
    assert_eq!(
        context["sources"][1]["name"],
        text(&fs::canonicalize(&extra).unwrap())
    );
    assert_eq!(
        context["sourceBytes"],
        r#"{"title":"build the bridge"}"#.len() + "extra facts".len()
    );
    let phases: Vec<_> = report["reads"]
        .as_array()
        .unwrap()
        .iter()
        .map(|read| read["phase"].as_str().unwrap())
        .collect();
    assert_eq!(phases, ["context", "context"]);
}

#[test]
fn the_callers_document_heads_the_measured_list_of_a_loaders_context() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("more.txt", "more");
    let document = sandbox.file("caller.json", r#"{"schemaVersion":1,"summary":"caller"}"#);
    sandbox.personal_rhai(&rhai_script(
        r#"fn load_context(request) {
    ravel::read_text("more.txt");
    #{ schemaVersion: 1, summary: request.context.summary + " and loader" }
}
fn select(request, context) {
    #{ status: "selected", program: "recording-agent",
       args: [context.summary, context.measured[0].via, context.measured[1].via],
       provider: "p", model: "m", effort: "e", reason: "r" }
}"#,
    ));

    start(&sandbox, &["--context", &text(&document)]).report();

    assert_eq!(
        recorded.argv().unwrap(),
        ["caller and loader", "--context", "read_text"]
    );
}

#[test]
fn a_loader_that_returns_unit_refuses_as_context_invalid() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&rhai_script(
        "fn load_context(request) { }\nfn select(request, context) { throw \"select ran\"; }",
    ));

    let error = refused(&start(&sandbox, &[]), "context_invalid", &recorded);

    assert_eq!(error["stage"], "context", "{error}");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("load_context returned no context")
    );
}

#[test]
fn a_loader_that_refuses_ends_as_config_refused_and_select_does_not_run() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&rhai_script(
        r#"fn load_context(request) {
    #{ status: "refused", code: "no_task", message: "there is no task", remedy: "pass a task" }
}
fn select(request, context) { throw "select ran"; }"#,
    ));

    let error = refused(&start(&sandbox, &[]), "config_refused", &recorded);

    assert_eq!(error["configCode"], "no_task", "{error}");
    assert_eq!(error["stage"], "context", "{error}");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("there is no task")
    );
}

#[test]
fn an_uncaught_error_in_load_context_refuses_with_its_file_line_and_column() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let entry = sandbox.personal_rhai(&rhai_script(
        "fn load_context(request) {\n    throw \"no task\";\n}\nfn select(request, context) { () }",
    ));

    let error = refused(&start(&sandbox, &[]), "context_loader_failed", &recorded);

    assert_eq!(error["stage"], "context", "{error}");
    assert_eq!(
        error["source"],
        text(&fs::canonicalize(entry).unwrap()),
        "{error}"
    );
    assert_eq!(error["location"], "4:5", "{error}");
    assert!(
        error["message"].as_str().unwrap().contains("no task"),
        "{error}"
    );
}

#[test]
fn a_context_over_the_budget_refuses_as_context_too_large() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&rhai_script(
        r#"fn load_context(request) {
    let text = "x";
    for i in 0..19 { text += text; }
    #{ schemaVersion: 1, summary: text }
}
fn select(request, context) { () }"#,
    ));

    let error = refused(&start(&sandbox, &[]), "context_too_large", &recorded);

    assert_eq!(error["stage"], "context", "{error}");
}

#[test]
fn a_context_with_an_invalid_shape_refuses_as_context_invalid_in_the_words_of_the_loader() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&rhai_script(
        "fn load_context(request) { #{ schemaVersion: 1, program: \"sh\" } }\nfn select(request, context) { () }",
    ));

    let error = refused(&start(&sandbox, &[]), "context_invalid", &recorded);

    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains("load_context returned is invalid"),
        "{error}"
    );
    assert!(!message.contains("loadContext"), "{error}");
}

#[test]
fn a_read_breach_in_load_context_refuses_in_the_words_of_the_rhai_path() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.file("big.txt", &"x".repeat(70_000));
    sandbox.personal_rhai(&rhai_script(
        "fn load_context(request) { ravel::read_text(\"big.txt\"); #{ schemaVersion: 1 } }\nfn select(request, context) { () }",
    ));

    let error = refused(&start(&sandbox, &[]), "source_too_large", &recorded);

    let message = error["message"].as_str().unwrap();
    assert!(message.contains("the configuration script "), "{error}");
    assert!(!message.contains("loadContext"), "{error}");
}

#[test]
fn an_end_raised_inside_a_closure_or_a_function_under_try_still_ends_the_evaluation() {
    // Rhai wraps every error that crosses a call of a function pointer in one
    // that `try` catches, and `pointer_calls` undoes that for the ends that
    // the worker raises itself.
    for (label, body) in [
        (
            "a closure of map",
            "    let args = [];\n    try { [1].map(|x| ravel::read_text(5)); } catch (error) { args.push(\"caught\"); }",
        ),
        (
            "a closure of sort",
            "    let args = [];\n    try { [2, 1].sort(|a, b| { ravel::read_text(5); 0 }); } catch (error) { args.push(\"caught\"); }",
        ),
        (
            "a function",
            "    let args = [];\n    try { inner(); } catch (error) { args.push(\"caught\"); }",
        ),
    ] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        let mut source = selecting(body);
        source.push_str("fn inner() { ravel::read_json(\"f.json\", \"many\") }\n");
        sandbox.personal_rhai(&source);

        let error = refused(&start(&sandbox, &[]), "host_request_invalid", &recorded);

        assert_eq!(error["stage"], "selection", "{label}: {error}");
    }
}
