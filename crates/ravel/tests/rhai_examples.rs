//! The shipped example configuration scripts, checked against frozen TypeScript observations
//! (`docs/specs/configuration-evaluation.md`, *Migration* and *Migration equivalence*).
//!
//! Every case goes through the real `ravel start-session` against a sandbox
//! home. A fake agent on the private `PATH` records the argument vector that it
//! was started with. The comparison covers the routing notice, that argument
//! vector, the delivered context and the refusal. The TypeScript observations
//! in `equivalence/fixtures/` were captured once by
//! `rhai-equivalence-fixtures-k30`. This suite runs only Rhai; never regenerate
//! the historical observations to fix a failure.
//!
//! The comparison leaves out these fields, because they must differ:
//!
//! - `schemaVersion` of the notice: 1 for TypeScript and 2 for Rhai.
//! - `config` (historically `policy`): the path has another extension, the owner's version string
//!   moves with the language, and only Rhai lists its modules.
//! - `worker`: the path and the build identity are those of another runtime.
//! - `selectionMs` and `operationReports`: a time, and a report of the
//!   environment's signals.
//! - `reads` and `harnessRuns`: only the Rhai notice has them. A Rhai configuration script
//!   reads the choice file with `ravel::read_text`, which Ravel measures.
//! - `bounds.harnessRuns`: only the Rhai bounds hold the harness-run budget.
//! - `context.sha256`: it hashes the delivered context, which holds the path of
//!   the sandbox, and the two languages run in two sandboxes. The value, the
//!   source-byte count and the measured sources of the context are compared.
//! - `context.encodedBytes` across sandboxes: the encoded value contains the
//!   temporary root, whose length differs between machines. Each live count is
//!   checked against the actual value before path normalization instead. The
//!   captured fixtures retain their original byte counts unchanged.
//!
//! Frozen notice `policyEnvironment` and refusal `policyCode` are translated
//! to their configuration names by `current_names`, along with the stable
//! refusal code, its exact Ravel message prefix and two exact shipped remedies.
//! All other strings remain exact, including owner codes and captured streams.
//!
//! A case that must differ says why, in `Expect::Differs`, and the test proves
//! that it does.

mod support;

use std::fs;
use std::os::unix::fs::{PermissionsExt as _, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{Map, Value, json};
use support::{FRONT, Run, Sandbox, run, text, write};

const RHAI_STATIC: &str = include_str!("../examples/static.rhai");
const RHAI_SAMPLE: &str = include_str!("../examples/sample.rhai");
const RHAI_HARNESS_TABLE: &str = include_str!("../examples/harness-table.rhai");
const RHAI_LLM: &str = include_str!("../examples/llm.rhai");

const CONFIG: &str = ".config/ravel";

// ---------------------------------------------------------------------------
// The configuration scripts, and the table of requests.
// ---------------------------------------------------------------------------

/// One Rhai configuration script, with the programs its routes start.
struct Config {
    name: &'static str,
    rhai: String,
    /// Files that the Rhai entry imports, beside it.
    rhai_modules: Vec<(&'static str, &'static str)>,
    programs: &'static [&'static str],
}

fn static_config() -> Config {
    Config {
        name: "static",
        rhai: RHAI_STATIC.to_owned(),
        rhai_modules: vec![],
        programs: &["my-agent-wrapper"],
    }
}

fn sample_config() -> Config {
    Config {
        name: "sample",
        rhai: RHAI_SAMPLE.to_owned(),
        rhai_modules: vec![("static.rhai", RHAI_STATIC)],
        programs: &["my-agent-wrapper"],
    }
}

fn harness_table_config() -> Config {
    Config {
        name: "harness-table",
        rhai: RHAI_HARNESS_TABLE.to_owned(),
        rhai_modules: vec![],
        programs: &["lead-agent", "review-agent"],
    }
}

/// The kinds that the harness table lists.
const TABLE_KINDS: [&str; 14] = [
    "scope",
    "scope-check",
    "scope-fold",
    "design",
    "design-check",
    "design-fold",
    "build",
    "build-check",
    "build-fold",
    "survey-a",
    "survey-b",
    "draft",
    "polish",
    "release-notes",
];

/// How two runs of a case must compare.
#[derive(Clone, Copy)]
enum Expect {
    /// Every compared field is equal.
    Same,
    /// The refusals have the same code, exit, stage and remedy. The message
    /// carries the text of a system error, and that text differs by runtime.
    SameExceptMessage(&'static str),
    /// The results differ, for this reason. The test proves that they do.
    Differs(&'static str),
}

/// A step of the setup of a case, in the working directory.
#[derive(Clone)]
enum Step {
    File {
        path: String,
        bytes: Vec<u8>,
        mode: Option<u32>,
    },
    Dir(String),
    Symlink {
        path: String,
        target: String,
    },
    Fifo(String),
}

#[derive(Clone)]
struct Case {
    name: String,
    kind: String,
    prompt: String,
    /// More arguments of `start-session`. A path in them is relative to the
    /// working directory.
    args: Vec<String>,
    steps: Vec<Step>,
    expect: Expect,
}

impl Case {
    fn new(name: &str, kind: &str) -> Self {
        Self {
            name: name.to_owned(),
            kind: kind.to_owned(),
            prompt: "Explain the parser.".to_owned(),
            args: vec![],
            steps: vec![],
            expect: Expect::Same,
        }
    }

    fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_owned();
        self
    }

    fn args(mut self, args: &[&str]) -> Self {
        self.args = args.iter().map(|arg| (*arg).to_owned()).collect();
        self
    }

    fn file(mut self, path: &str, bytes: impl AsRef<[u8]>) -> Self {
        self.steps.push(Step::File {
            path: path.to_owned(),
            bytes: bytes.as_ref().to_vec(),
            mode: None,
        });
        self
    }

    fn file_with_mode(mut self, path: &str, bytes: &str, mode: u32) -> Self {
        self.steps.push(Step::File {
            path: path.to_owned(),
            bytes: bytes.as_bytes().to_vec(),
            mode: Some(mode),
        });
        self
    }

    fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    fn expect(mut self, expect: Expect) -> Self {
        self.expect = expect;
        self
    }
}

const CHOICE: &str = ".ravel-choice";
const JJ_STORE: &str = ".jj/repo";

/// The table for the starter route table. The sample reuses the kinds.
fn static_cases() -> Vec<Case> {
    let mut cases: Vec<Case> = ["question", "bugfix", "feature", "migration", "architecture"]
        .iter()
        .map(|kind| Case::new(&format!("route {kind}"), kind))
        .collect();
    for kind in [
        "nope",
        "Question",
        "question ",
        "constructor",
        "__proto__",
        "toString",
        "hasOwnProperty",
        "a\"quote\\and\u{e9}",
        "\u{43a}\u{438}\u{440}\u{438}\u{43b}\u{43b}\u{438}\u{446}\u{430}",
        "tab\there",
        "\u{1}\u{7f}\u{2028}",
        "line\nbreak",
    ] {
        cases.push(Case::new(&format!("unlisted {kind:?}"), kind));
    }
    cases.extend([
        Case::new("a prompt with unicode", "bugfix").prompt("Fix \u{1f980} in \u{e9}t\u{e9}\u{2003}now"),
        Case::new("a prompt with lines", "feature").prompt("one\n\ntwo\n  three\n"),
        Case::new("a prompt that looks like options", "question").prompt("--model other --effort low"),
        Case::new("a long prompt", "migration").prompt(&"long ".repeat(4000)),
        Case::new("params and task", "bugfix")
            .args(&[
                "--param",
                "profile=careful",
                "--param",
                "empty=",
                "--task-id",
                "task-7",
                "--task-file",
                "task.md",
            ])
            .file("task.md", "# a task\n"),
        Case::new("a context document", "feature")
            .args(&["--context", "context.json"])
            .file(
                "context.json",
                r#"{"schemaVersion":1,"summary":"Fix the parser","acceptanceCriteria":["it parses"],"facts":{"area":"parser","n":3}}"#,
            ),
        Case::new("a context document and an unlisted kind", "nope")
            .args(&["--context", "context.json"])
            .file("context.json", r#"{"schemaVersion":1,"summary":"s"}"#),
    ]);
    cases
}

/// A choice file with `bytes`, for kind `bugfix`.
fn choice(name: &str, bytes: impl AsRef<[u8]>) -> Case {
    Case::new(&format!("choice {name}"), "bugfix").file(CHOICE, bytes)
}

fn sample_cases() -> Vec<Case> {
    let mut cases = static_cases();
    let padded = |word: &str, size: usize| format!("{word}{}", " ".repeat(size - word.len()));
    cases.extend([
        Case::new("no choice file", "bugfix"),
        choice("default", "default\n"),
        choice("careful", "careful\n"),
        choice("careful, no newline", "careful"),
        choice("careful, many spaces and lines", "\n\t careful \r\n\n"),
        choice("two names", "careful default\n"),
        choice("the same name twice", "careful careful\n"),
        choice("empty", ""),
        choice("only white space", "  \n\t \n"),
        choice("a name that is not offered", "fast\n"),
        choice("a name in another case", "Careful\n"),
        choice("a name with a quote", "fa\"st\n"),
        choice("a name with a control character", "fa\u{1}st\n"),
        choice("a name with accents", "caf\u{e9}\n"),
        choice("an offered name and one that is not", "default fast\n"),
        choice("an em space", "careful\u{2003}"),
        choice("a no-break space", "careful\u{a0}"),
        choice("exactly 4096 bytes", padded("careful", 4096)),
    ]);
    // The choice file is not a plain file in these cases, or it is not valid.
    cases.push(Case::new("choice is a directory", "bugfix").step(Step::Dir(CHOICE.to_owned())));
    cases.push(
        Case::new("choice is a dangling link", "bugfix").step(Step::Symlink {
            path: CHOICE.to_owned(),
            target: "nowhere".to_owned(),
        }),
    );
    cases.push(
        Case::new("choice is a link to a file", "bugfix")
            .file("real-choice", "careful\n")
            .step(Step::Symlink {
                path: CHOICE.to_owned(),
                target: "real-choice".to_owned(),
            }),
    );
    cases.push(Case::new("choice is a FIFO", "bugfix").step(Step::Fifo(CHOICE.to_owned())));
    cases.push(
        choice("not UTF-8", [0x63u8, 0xff, 0xfe, 0x0a]).expect(Expect::SameExceptMessage(
            "the message carries the text of the decoder's error",
        )),
    );
    cases.push(
        Case::new("choice is not readable", "bugfix")
            .file_with_mode(CHOICE, "careful\n", 0o000)
            .expect(Expect::SameExceptMessage(
                "the message carries the text of the system error",
            )),
    );
    cases.push(
        choice("4097 bytes", padded("careful", 4097)).expect(Expect::Differs(
            "TypeScript refuses the file as choice_unreadable. read_text ends the evaluation as source_too_large, because a read bound cannot be caught",
        )),
    );
    cases.push(
        choice("a byte order mark between names", "careful\u{feff}").expect(Expect::Differs(
            "U+FEFF is white space in a JavaScript regular expression and not for split() in Rhai",
        )),
    );
    cases.push(
        choice("a next line between names", "careful\u{85}default").expect(Expect::Differs(
            "U+0085 is white space for split() in Rhai and not in a JavaScript regular expression",
        )),
    );
    cases
}

fn harness_table_cases() -> Vec<Case> {
    let mut cases: Vec<Case> = TABLE_KINDS
        .iter()
        .map(|kind| Case::new(&format!("route {kind}"), kind))
        .collect();
    for kind in [
        "nope",
        "Build",
        "build ",
        "constructor",
        "__proto__",
        "toString",
        "a\"b\\c\u{e9}",
    ] {
        cases.push(Case::new(&format!("unlisted {kind:?}"), kind));
    }
    cases.extend([
        Case::new("a prompt with unicode", "build").prompt("Fix \u{1f980}\n--add-dir /etc"),
        Case::new("a prompt that looks like options", "build-check").prompt("--add-dir /tmp"),
    ]);
    let store = |name: &str, kind: &str, bytes: &str| {
        Case::new(&format!("store {name}"), kind).file(JJ_STORE, bytes)
    };
    cases.extend([
        store("relative", "build", "../../elsewhere/.jj/repo"),
        store("relative for a reviewer", "build-check", "../../elsewhere/.jj/repo"),
        store("relative with white space around", "build", "  ../../elsewhere/.jj/repo \r\n"),
        store("relative with a trailing slash", "build", "../../elsewhere/.jj/repo/"),
        store("relative with dot segments", "build", "../.././elsewhere//.jj/../.jj/repo"),
        store("relative above the root", "build", &"../".repeat(40)),
        store("absolute", "build", "/srv/main/.jj/repo"),
        store("absolute with a double slash", "build", "/srv//main/.jj/repo"),
        store("the root", "build", "/"),
        store("a dot", "build", "."),
        store("a no-break space around", "build", "\u{a0}../../elsewhere/.jj/repo\u{a0}"),
        store("empty", "build", ""),
        store("only white space", "build", " \n\t "),
        store("empty, unlisted kind", "nope", ""),
        Case::new("store is a directory", "build").step(Step::Dir(JJ_STORE.to_owned())),
        Case::new("store is a dangling link", "build").step(Step::Symlink {
            path: JJ_STORE.to_owned(),
            target: "nowhere".to_owned(),
        }),
        Case::new("jj is a file", "build")
            .file(".jj", "not a directory")
            .expect(Expect::SameExceptMessage(
                "the message carries the text of the system error",
            )),
        Case::new("store is not readable", "build")
            .file_with_mode(JJ_STORE, "../../elsewhere/.jj/repo", 0o000)
            .expect(Expect::SameExceptMessage(
                "the message carries the text of the system error",
            )),
        Case::new("store is not UTF-8", "build")
            .file(JJ_STORE, [0x2eu8, 0x2e, 0x2f, 0xff, 0xfe])
            .expect(Expect::Differs(
                "TypeScript decodes the bytes with replacement characters. read_text refuses a file that is not UTF-8",
            )),
        store("a byte order mark", "build", "\u{feff}../../elsewhere/.jj/repo").expect(
            Expect::Differs(
                "trim() in JavaScript removes U+FEFF and trim() in Rhai does not",
            ),
        ),
        store("a next line", "build", "\u{85}../../elsewhere/.jj/repo").expect(Expect::Differs(
            "trim() in Rhai removes U+0085 and trim() in JavaScript does not",
        )),
    ]);
    cases
}

fn cases_of(config: &Config) -> Vec<Case> {
    match config.name {
        "static" => static_cases(),
        "sample" => sample_cases(),
        "harness-table" => harness_table_cases(),
        other => panic!("no table of requests for {other}"),
    }
}

// ---------------------------------------------------------------------------
// Running a case.
// ---------------------------------------------------------------------------

/// The file where the fake agents record what they were started with.
fn launch_record(sandbox: &Sandbox) -> PathBuf {
    sandbox.tmp.join("launched")
}

/// A fake agent `program` on the private `PATH`. It records its own name and
/// its arguments, each followed by a NUL, and then it reports a result.
fn recording_agent(sandbox: &Sandbox, program: &str) {
    let script = sandbox.bin.join(program);
    write(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\0' \"${{0##*/}}\" \"$@\" > {:?}\nexec {FRONT:?} end-session done\n",
            launch_record(sandbox)
        ),
    );
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
}

/// The program and the arguments that the last fake agent recorded, or `None`
/// when no agent started.
fn launched(sandbox: &Sandbox) -> Option<Value> {
    let bytes = fs::read(launch_record(sandbox)).ok()?;
    let mut fields: Vec<String> = bytes
        .split(|byte| *byte == 0)
        .map(|field| String::from_utf8_lossy(field).into_owned())
        .collect();
    assert_eq!(
        fields.pop().as_deref(),
        Some(""),
        "a record ends with a NUL"
    );
    let program = fields.remove(0);
    Some(json!({ "program": program, "args": fields }))
}

fn install(sandbox: &Sandbox, config: &Config) {
    sandbox.personal_rhai(&config.rhai);
    for (name, source) in &config.rhai_modules {
        write(&sandbox.home.join(CONFIG).join(name), source);
    }
    for program in config.programs {
        recording_agent(sandbox, program);
    }
}

fn apply(sandbox: &Sandbox, step: &Step) {
    let at = |path: &str| sandbox.cwd.join(path);
    match step {
        Step::File { path, bytes, mode } => {
            let target = at(path);
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::write(&target, bytes).unwrap();
            if let Some(mode) = mode {
                fs::set_permissions(&target, fs::Permissions::from_mode(*mode)).unwrap();
            }
        }
        Step::Dir(path) => fs::create_dir_all(at(path)).unwrap(),
        Step::Symlink { path, target } => {
            fs::create_dir_all(at(path).parent().unwrap()).unwrap();
            symlink(target, at(path)).unwrap();
        }
        Step::Fifo(path) => support::mkfifo(&at(path)),
    }
}

/// Whether the tests run as the superuser, for whom a mode of 000 stops nothing.
fn superuser() -> bool {
    // SAFETY: `geteuid` reads the process's credentials.
    unsafe { libc::geteuid() == 0 }
}

fn start(sandbox: &Sandbox, cwd: &Path, case: &Case) -> Run {
    let mut command = sandbox.command();
    command
        .current_dir(cwd)
        .args([
            "start-session",
            "--prompt",
            &case.prompt,
            "--kind",
            &case.kind,
            "--json",
        ])
        .args(&case.args);
    run(&mut command)
}

/// `value` with every string that holds `from` changed to hold `to`.
fn replace_in(value: &mut Value, from: &str, to: &str) {
    match value {
        Value::String(string) => *string = string.replace(from, to),
        Value::Array(items) => items.iter_mut().for_each(|item| replace_in(item, from, to)),
        Value::Object(map) => map.values_mut().for_each(|item| replace_in(item, from, to)),
        _ => {}
    }
}

/// The fields of a notice that the comparison keeps. See the head of this file
/// for the fields that it leaves out.
fn compared_notice(notice: &Value) -> Value {
    let notice = notice.as_object().expect("a notice is an object");
    // The notice must come from the path under test.
    assert_eq!(notice["schemaVersion"], 2, "{notice:?}");
    assert!(notice["worker"]["rhaiVersion"].is_string(), "{notice:?}");
    let mut kept = Map::new();
    for key in [
        "bounds",
        "context",
        "diagnostics",
        "configEnvironment",
        "promptSource",
        "resolution",
        "selection",
    ] {
        kept.insert(key.to_owned(), notice[key].clone());
    }
    kept["bounds"]
        .as_object_mut()
        .unwrap()
        .remove("harnessRuns");
    if let Some(context) = kept["context"].as_object_mut() {
        context.remove("sha256");
    }
    Value::Object(kept)
}

/// What a case gave, with the paths of the sandbox and of the configuration script file
/// replaced, so that it compares with the frozen observation from another
/// sandbox.
fn observe_in(sandbox: &Sandbox, cwd: &Path, case: &Case) -> Value {
    let _ = fs::remove_file(launch_record(sandbox));
    let result = start(sandbox, cwd, case);
    let mut observation = match result.code {
        Some(0) => {
            let notice = result.report();
            if notice["context"].is_object() {
                assert_eq!(
                    notice["context"]["encodedBytes"],
                    serde_json::to_vec(&notice["context"]["value"])
                        .unwrap()
                        .len(),
                    "context byte count before path normalization"
                );
            }
            json!({
                "exit": 0,
                "launched": launched(sandbox).expect("a selected route starts its agent"),
                "notice": compared_notice(&notice),
                "refusal": null,
            })
        }
        Some(3) => {
            assert!(
                launched(sandbox).is_none(),
                "a refusal starts nothing: {}",
                result.stderr
            );
            let document = result.refusal(3);
            json!({
                "exit": 3,
                "launched": null,
                "notice": null,
                "refusal": { "error": document["error"], "diagnostics": document["diagnostics"] },
            })
        }
        other => panic!("Rhai {}: exit {other:?}\n{}", case.name, result.stderr),
    };
    replace_in(
        &mut observation,
        &text(&sandbox.personal_rhai_path()),
        "<policy>",
    );
    replace_in(&mut observation, &text(&sandbox.root), "<sandbox>");
    observation
}

fn observe(config: &Config, case: &Case) -> Value {
    let sandbox = Sandbox::new();
    install(&sandbox, config);
    for step in &case.steps {
        apply(&sandbox, step);
    }
    observe_in(&sandbox, &sandbox.cwd, case)
}

/// `observation` as `expect` compares it.
fn comparable(observation: &Value, expect: Expect) -> Value {
    let mut copy = observation.clone();
    if let Some(context) = copy["notice"]["context"].as_object_mut() {
        context.remove("encodedBytes");
    }
    if let Expect::SameExceptMessage(_) = expect
        && let Some(error) = copy["refusal"]["error"].as_object_mut()
    {
        error.remove("message");
    }
    copy
}

/// Translate the frozen pre-2.0 observation, never the live output. Only these
/// contract fields and exact Ravel/shipped-example messages changed names.
/// Owner labels, arguments, diagnostics and arbitrary refusal text stay exact.
fn current_names(historical: &Value) -> Value {
    let mut current = historical.clone();
    if let Some(notice) = current["notice"].as_object_mut()
        && let Some(grants) = notice.remove("policyEnvironment")
    {
        assert!(notice.insert("configEnvironment".into(), grants).is_none());
    }
    if let Some(error) = current
        .get_mut("refusal")
        .and_then(|refusal| refusal.get_mut("error"))
        .and_then(Value::as_object_mut)
    {
        if let Some(code) = error.remove("policyCode") {
            assert!(error.insert("configCode".into(), code).is_none());
        }
        if error.get("code") == Some(&json!("policy_refused")) {
            error.insert("code".into(), json!("config_refused"));
            if let Some(suffix) = error["message"].as_str().and_then(|message| {
                message.strip_prefix("the policy <policy> refused the selection: ")
            }) {
                error.insert(
                    "message".into(),
                    json!(format!(
                        "the configuration script <policy> refused the selection: {suffix}"
                    )),
                );
            }
            let remedy = match error.get("remedy").and_then(Value::as_str) {
                Some("add a route for this kind in your copy of the policy") => {
                    Some("add a route for this kind in your copy of the configuration script")
                }
                Some("add a mapping for this kind in your policy") => {
                    Some("add a mapping for this kind in your configuration script")
                }
                _ => None,
            };
            if let Some(remedy) = remedy {
                error.insert("remedy".into(), json!(remedy));
            }
        }
    }
    current
}

#[test]
fn historical_name_translation_is_field_specific_and_preserves_owner_data() {
    let no_refusal = json!({"notice": null, "refusal": null});
    assert_eq!(current_names(&no_refusal), no_refusal);
    let original = json!({
        "notice": {"policyEnvironment": [{"name": "policy_token", "set": true}],
                   "selection": {"reason": "policy_refused"}},
        "refusal": {"error": {
            "code": "policy_refused", "policyCode": "policy_owner",
            "message": "the policy <policy> refused the selection: policy_refused policyCode",
            "remedy": "owner policy_refused", "source": "<policy>",
            "input": "--kind policy_refused"},
            "diagnostics": {"stdout": "policy stdout: policy_refused"}},
        "launched": {"args": ["policy_refused", "policyCode"]}
    });
    let mut expected = original.clone();
    let notice = expected["notice"].as_object_mut().unwrap();
    let grants = notice.remove("policyEnvironment").unwrap();
    notice.insert("configEnvironment".into(), grants);
    let error = expected["refusal"]["error"].as_object_mut().unwrap();
    let code = error.remove("policyCode").unwrap();
    error.insert("configCode".into(), code);
    error.insert("code".into(), json!("config_refused"));
    error.insert(
        "message".into(),
        json!("the configuration script <policy> refused the selection: policy_refused policyCode"),
    );
    assert_eq!(current_names(&original), expected);
    assert_ne!(original, expected);

    // Only the exact shipped remedies are translated. A changed remedy must
    // remain changed so that the existing mutation comparison catches it.
    for (old, new) in [
        (
            "add a route for this kind in your copy of the policy",
            "add a route for this kind in your copy of the configuration script",
        ),
        (
            "add a mapping for this kind in your policy",
            "add a mapping for this kind in your configuration script",
        ),
    ] {
        let mut historical = original.clone();
        historical["refusal"]["error"]["remedy"] = json!(old);
        assert_eq!(
            current_names(&historical)["refusal"]["error"]["remedy"],
            new
        );
        historical["refusal"]["error"]["remedy"] = json!(format!("{old} changed"));
        assert_eq!(
            current_names(&historical)["refusal"]["error"]["remedy"],
            format!("{old} changed")
        );
    }
}

/// The failures of `cases` against `config`, one text each. It runs the cases in
/// parallel against the frozen fixture, without starting TypeScript.
fn compare_with(config: &Config, cases: &[Case], fixture: &Value) -> Vec<String> {
    let next = AtomicUsize::new(0);
    let failures = std::sync::Mutex::new(vec![]);
    std::thread::scope(|scope| {
        for _ in 0..6 {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::SeqCst);
                    let Some(case) = cases.get(index) else { break };
                    if let Some(failure) = check_case(config, case, fixture) {
                        failures.lock().unwrap().push(failure);
                    }
                }
            });
        }
    });
    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    failures
}

fn check_case(config: &Config, case: &Case, fixture: &Value) -> Option<String> {
    let unreadable = case.steps.iter().any(|step| {
        matches!(
            step,
            Step::File {
                mode: Some(0o000),
                ..
            }
        )
    });
    if unreadable && superuser() {
        return None;
    }
    let typescript = &fixture[&case.name];
    assert!(
        typescript.is_object(),
        "missing fixture case: {}",
        case.name
    );
    let rhai = observe(config, case);
    let (left, right) = (
        comparable(&current_names(typescript), case.expect),
        comparable(&rhai, case.expect),
    );
    match case.expect {
        Expect::Same | Expect::SameExceptMessage(_) => (left != right).then(|| {
            let note = match case.expect {
                Expect::SameExceptMessage(why) => format!(" (the message is left out: {why})"),
                _ => String::new(),
            };
            format!(
                "{} / {}: the results differ{note}\n  typescript: {typescript}\n  rhai:       {rhai}",
                config.name, case.name
            )
        }),
        Expect::Differs(reason) => (left == right).then(|| {
            format!(
                "{} / {}: the results are equal, but the case records a difference ({reason})\n  both: {rhai}",
                config.name, case.name
            )
        }),
    }
}

fn fixture(config: &Config) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/equivalence/fixtures")
        .join(format!("{}.json", config.name));
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).expect("a frozen TypeScript observation document")
}

#[test]
fn rhai_examples_match_the_frozen_typescript_observations() {
    for config in [static_config(), sample_config(), harness_table_config()] {
        let fixture = fixture(&config);
        let cases = cases_of(&config);
        // Missing and extra cases both fail: changing the table needs an
        // explicit decision, never regeneration of the historical reference.
        assert_eq!(fixture["cases"].as_object().unwrap().len(), cases.len());
        let failures = compare_with(&config, &cases, &fixture["cases"]);
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}

#[test]
fn the_fixture_comparison_fails_on_a_changed_fixture() {
    let config = static_config();
    let fixture = fixture(&config);
    let cases: Vec<Case> = cases_of(&config)
        .into_iter()
        .filter(|case| {
            ["route bugfix", "unlisted \"nope\"", "a context document"]
                .contains(&case.name.as_str())
        })
        .collect();
    assert_eq!(cases.len(), 3);
    assert!(compare_with(&config, &cases, &fixture["cases"]).is_empty());

    for (name, field) in [
        ("route bugfix", "/notice/selection/effort"),
        ("unlisted \"nope\"", "/refusal/error/remedy"),
        ("a context document", "/notice/context/value/summary"),
    ] {
        let mut changed = fixture["cases"].clone();
        *changed[name]
            .pointer_mut(field)
            .expect("an existing fixture field") = json!("changed");
        assert_ne!(changed, fixture["cases"]);
        let failures = compare_with(&config, &cases, &changed);
        assert_eq!(failures.len(), 1, "{field}: {failures:?}");
        assert!(failures[0].contains(name), "{failures:?}");
        assert!(failures[0].contains("the results differ"), "{failures:?}");
    }
}

#[test]
fn the_fixture_comparison_fails_on_a_changed_translation() {
    // The firing control of the comparison: a translation that changes one
    // effort, and one that changes one refusal, must not compare equal.
    let case = Case::new("route bugfix", "bugfix");
    let unlisted = Case::new("unlisted \"nope\"", "nope");
    let original = static_config();
    let fixture = fixture(&original);
    let effort = Config {
        rhai: RHAI_STATIC.replace("bugfix: agent(\"medium\")", "bugfix: agent(\"high\")"),
        ..static_config()
    };
    assert_ne!(effort.rhai, original.rhai);
    let failures = compare_with(&effort, std::slice::from_ref(&case), &fixture["cases"]);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("the results differ"), "{failures:?}");

    let remedy = Config {
        rhai: RHAI_STATIC.replace("add a route for this kind", "add a kind"),
        ..static_config()
    };
    assert_ne!(remedy.rhai, original.rhai);
    let failures = compare_with(&remedy, std::slice::from_ref(&unlisted), &fixture["cases"]);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("the results differ"), "{failures:?}");

    // And the unchanged translation compares equal on the same cases.
    assert!(compare_with(&original, &[case, unlisted], &fixture["cases"]).is_empty());
}

// ---------------------------------------------------------------------------
// The owner-shaped configuration script in a secondary jj workspace.
// ---------------------------------------------------------------------------

fn jj_available() -> bool {
    Command::new("jj")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

/// `jj` with `args` in `dir`, with a home and a user of its own, so that no
/// configuration of the machine reaches it.
fn jj(sandbox: &Sandbox, dir: &Path, args: &[&str]) {
    let output = Command::new("jj")
        .current_dir(dir)
        .env("HOME", &sandbox.tmp)
        .env("XDG_CONFIG_HOME", sandbox.tmp.join("jj-config"))
        .env("JJ_USER", "Example")
        .env("JJ_EMAIL", "example@example.invalid")
        .args(args)
        .output()
        .expect("jj starts");
    assert!(
        output.status.success(),
        "jj {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A colocated main repository and a secondary workspace of it in `sandbox`.
/// It returns the canonical path of each.
fn jj_workspaces(sandbox: &Sandbox) -> (PathBuf, PathBuf) {
    jj(
        sandbox,
        &sandbox.root,
        &["git", "init", "--colocate", "main"],
    );
    let main = sandbox.root.join("main");
    jj(sandbox, &main, &["workspace", "add", "../second"]);
    let second = sandbox.root.join("second");
    // The store of the secondary workspace is a file, as the configuration script expects.
    assert!(second.join(".jj/repo").is_file());
    assert!(main.join(".jj/repo").is_dir());
    (main.canonicalize().unwrap(), second.canonicalize().unwrap())
}

fn observe_workspaces() -> Value {
    let config = harness_table_config();
    let sandbox = Sandbox::new();
    install(&sandbox, &config);
    let (main, second) = jj_workspaces(&sandbox);
    let mut secondary = Map::new();
    for kind in TABLE_KINDS {
        let case = Case::new(&format!("secondary workspace {kind}"), kind);
        let observed = observe_in(&sandbox, &second, &case);
        // The grant is the main repository, once, beside `--add-dir`.
        let args = observed["launched"]["args"].as_array().unwrap();
        let grants: Vec<&Value> = args
            .windows(2)
            .filter(|pair| pair[0] == "--add-dir")
            .map(|pair| &pair[1])
            .collect();
        assert_eq!(grants, [&json!("<sandbox>/main")], "Rhai {kind}: {args:?}");
        assert!(secondary.insert(kind.to_owned(), observed).is_none());
    }
    // A primary workspace needs no grant, for either layout of arguments.
    let mut primary = Map::new();
    for kind in ["build", "build-check"] {
        let case = Case::new(&format!("primary workspace {kind}"), kind);
        let observed = observe_in(&sandbox, &main, &case);
        let args = observed["launched"]["args"].as_array().unwrap();
        assert!(!args.contains(&json!("--add-dir")), "Rhai {kind}: {args:?}");
        assert!(primary.insert(kind.to_owned(), observed).is_none());
    }
    json!({ "secondary": secondary, "primary": primary })
}

#[test]
fn rhai_harness_table_matches_the_frozen_jj_workspace_observations() {
    if !jj_available() {
        eprintln!("SKIPPED: jj is not installed, and this test needs a secondary jj workspace");
        return;
    }
    let fixture = fixture(&harness_table_config());
    let mut historical = fixture["workspaces"].clone();
    for workspace in historical.as_object_mut().unwrap().values_mut() {
        for observation in workspace.as_object_mut().unwrap().values_mut() {
            *observation = current_names(observation);
        }
    }
    assert_eq!(observe_workspaces(), historical);
}

#[test]
fn the_harness_table_grants_the_store_that_the_file_of_a_secondary_workspace_names() {
    // The firing control of the jj test: a grant that came from the store file
    // and not from a default. A second main repository, named by a hand-made
    // store file, gives another grant.
    let config = harness_table_config();
    let sandbox = Sandbox::new();
    install(&sandbox, &config);
    write(&sandbox.cwd.join(JJ_STORE), "../../moved/.jj/repo\n");
    let case = Case::new("hand-made store", "build");
    let observation = observe_in(&sandbox, &sandbox.cwd, &case);
    let args = observation["launched"]["args"].as_array().unwrap();
    assert_eq!(
        args[..2],
        [json!("--add-dir"), json!("<sandbox>/moved")],
        "{args:?}"
    );
}

// ---------------------------------------------------------------------------
// llm.rhai: an LLM-assisted selection through a fake harness.
// ---------------------------------------------------------------------------

/// The schema that `llm.rhai` sends, as `ravel::json_encode` writes it.
const LLM_SCHEMA: &str = r#"{"additionalProperties":false,"properties":{"kind":{"enum":["architecture","bugfix","feature","migration","question"],"type":"string"}},"required":["kind"],"type":"object"}"#;

const QUESTION: &str =
    "Which kind of work does this request describe? Answer with the JSON object only.\n\n";

/// A prompt that no other text of a notice can hold.
const PROMPT: &str = "PROMPT-MARKER-c47e fix the failing parser test";

/// A sandbox for `llm.rhai` with the harness entry `claude` and `runs` runs.
fn llm_sandbox(runs: u64) -> Sandbox {
    let sandbox = Sandbox::new();
    sandbox.personal_rhai(RHAI_LLM);
    write(&sandbox.home.join(CONFIG).join("static.rhai"), RHAI_STATIC);
    let settings = json!({
        "harnesses": { "claude": { "program": "claude" } },
        "budgets": { "harnessRuns": runs },
    });
    write(
        &sandbox.home.join(CONFIG).join("settings.json"),
        &settings.to_string(),
    );
    recording_agent(&sandbox, "my-agent-wrapper");
    sandbox
}

/// What the fake `claude` saw.
struct Claude {
    argv: PathBuf,
    stdin: PathBuf,
}

impl Claude {
    fn argv(&self) -> Vec<String> {
        fs::read_to_string(&self.argv)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn stdin(&self) -> String {
        fs::read_to_string(&self.stdin).unwrap()
    }
}

/// A fake `claude` that records its arguments and its standard input, and then
/// runs `body`, a shell fragment that plays the model.
fn fake_claude(sandbox: &Sandbox, body: &str) -> Claude {
    let claude = Claude {
        argv: sandbox.root.join("claude.argv"),
        stdin: sandbox.root.join("claude.stdin"),
    };
    let script = sandbox.bin.join("claude");
    write(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > {:?}\ncat > {:?}\n{body}\n",
            claude.argv, claude.stdin
        ),
    );
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    claude
}

/// A fake `claude` whose JSON document holds `structured_output`.
fn answering(sandbox: &Sandbox, structured_output: &str) -> Claude {
    let document = format!(
        r#"{{"type":"result","is_error":false,"result":"done","structured_output":{structured_output}}}"#
    );
    fake_claude(sandbox, &format!("printf '%s' '{document}'"))
}

fn session(sandbox: &Sandbox, kind: &str) -> Run {
    let mut command = sandbox.command();
    command.args([
        "start-session",
        "--prompt",
        PROMPT,
        "--kind",
        kind,
        "--json",
    ]);
    run(&mut command)
}

/// The notice of a session that launched its route.
fn notice(run: &Run) -> Value {
    run.report()
}

fn route(program_args: &Value) -> Vec<&str> {
    program_args["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect()
}

#[test]
fn llm_rhai_launches_the_route_of_the_kind_that_claude_names() {
    let attempt = |answer: &str| {
        let sandbox = llm_sandbox(1);
        let claude = answering(&sandbox, answer);
        // The caller's kind is `question`, and the answer decides.
        let run = session(&sandbox, "question");
        (notice(&run), launched(&sandbox).unwrap(), claude, sandbox)
    };

    let (notice, started, claude, _keep) = attempt(r#"{"kind":"bugfix"}"#);

    // Ravel launched the route of the answer, with the prompt as its last argument.
    assert_eq!(started["program"], "my-agent-wrapper");
    assert_eq!(
        route(&started),
        ["--model", "your-model", "--effort", "medium", PROMPT]
    );
    assert_eq!(
        notice["selection"]["reason"],
        "claude's answer: routes[\"bugfix\"] gives my-agent-wrapper with model your-model at effort medium"
    );
    // Claude Code ran once, as one harness run, with the arguments that the
    // smoke task proved with a real call, and with the question on stdin.
    assert_eq!(
        claude.argv(),
        [
            "-p",
            "--model",
            "haiku",
            "--tools",
            "",
            "--no-session-persistence",
            "--output-format",
            "json",
            "--json-schema",
            LLM_SCHEMA,
        ]
    );
    assert_eq!(claude.stdin(), format!("{QUESTION}{PROMPT}"));
    let runs = notice["harnessRuns"].as_array().unwrap();
    assert_eq!(runs.len(), 1, "{notice}");
    assert_eq!(runs[0]["harness"], "claude");
    assert_eq!(runs[0]["status"], "exited");
    // No diagnostic, and neither the prompt nor the answer in the notice.
    assert_eq!(notice["diagnostics"], json!({ "stderr": "", "stdout": "" }));
    assert!(!notice.to_string().contains("PROMPT-MARKER"), "{notice}");

    // The firing control: another answer is another route, so the route came
    // from the answer and not from the caller's kind.
    let (notice, started, _, _keep) = attempt(r#"{"kind":"feature"}"#);
    assert_eq!(
        route(&started),
        ["--model", "your-model", "--effort", "high", PROMPT]
    );
    assert_eq!(
        notice["selection"]["reason"],
        "claude's answer: routes[\"feature\"] gives my-agent-wrapper with model your-model at effort high"
    );
}

#[test]
fn llm_rhai_falls_back_to_the_callers_kind_when_the_call_fails() {
    // The first call fails the way a rate limit does: exit 1 and a message.
    let sandbox = llm_sandbox(1);
    fake_claude(
        &sandbox,
        "echo 'RATE-LIMIT-9a1d: try again later' >&2\nexit 1",
    );

    let run = session(&sandbox, "feature");
    let notice = notice(&run);

    // The script saw `exited` with exit code 1, and the table of the caller's
    // kind decided.
    let started = launched(&sandbox).unwrap();
    assert_eq!(
        route(&started),
        ["--model", "your-model", "--effort", "high", PROMPT]
    );
    assert_eq!(
        notice["selection"]["reason"],
        "the caller's kind: routes[\"feature\"] gives my-agent-wrapper with model your-model at effort high"
    );
    assert!(
        notice["diagnostics"]["stderr"]
            .as_str()
            .unwrap()
            .contains("claude gave no answer: status exited, exit code 1"),
        "{notice}"
    );
    assert_eq!(notice["harnessRuns"][0]["status"], "exited");
    // The message of the failed call reaches the script only. The notice holds
    // neither it nor the prompt.
    assert!(!notice.to_string().contains("RATE-LIMIT"), "{notice}");
    assert!(!notice.to_string().contains("PROMPT-MARKER"), "{notice}");

    // The firing control: the fallback is the table. A kind that the table does
    // not list gives the table's refusal after a failed call.
    let other = llm_sandbox(1);
    fake_claude(&other, "exit 1");
    let refusal = session(&other, "triage").refusal(3);
    assert_eq!(refusal["error"]["configCode"], "incomplete_mapping");
    assert!(launched(&other).is_none());
}

#[test]
fn llm_rhai_checks_the_answer_with_json_check_and_falls_back_on_any_other_answer() {
    // Each answer that is no usable kind gives the fallback, and the diagnostic
    // names the reason, never a part of the answer.
    let sandbox_for = |body: Option<&str>| {
        let sandbox = llm_sandbox(1);
        if let Some(body) = body {
            fake_claude(&sandbox, body);
        }
        sandbox
    };
    let document = |structured_output: &str| {
        format!(r#"printf '%s' '{{"result":"x","structured_output":{structured_output}}}'"#)
    };
    let cases: [(&str, Option<String>, &str); 7] = [
        (
            "a kind that the table does not list",
            Some(document(r#"{"kind":"ANSWER-deploy-5b3e"}"#)),
            "claude gave an unusable answer: json_mismatch",
        ),
        (
            "a property that is missing",
            Some(document(r#"{"other":"ANSWER-other-5b3e"}"#)),
            "claude gave an unusable answer: json_mismatch",
        ),
        (
            "an extra property",
            Some(document(r#"{"kind":"bugfix","note":"ANSWER-note-5b3e"}"#)),
            "claude gave an unusable answer: json_mismatch",
        ),
        (
            "a kind of the wrong type",
            Some(document(r#"{"kind":7}"#)),
            "claude gave an unusable answer: json_mismatch",
        ),
        (
            "a document with no structured_output",
            Some(r#"printf '%s' '{"result":"ANSWER-text-5b3e"}'"#.to_owned()),
            "claude gave no structured_output",
        ),
        (
            "an output that is no JSON",
            Some("echo ANSWER-text-5b3e".to_owned()),
            "claude gave an unusable answer: json_invalid",
        ),
        (
            "a program that is not there",
            None,
            "claude gave no answer: status unavailable",
        ),
    ];
    for (name, body, diagnostic) in cases {
        let sandbox = sandbox_for(body.as_deref());
        let run = session(&sandbox, "migration");
        let notice = notice(&run);
        let started = launched(&sandbox).unwrap();
        assert_eq!(
            route(&started),
            ["--model", "your-model", "--effort", "xhigh", PROMPT],
            "{name}"
        );
        assert!(
            notice["diagnostics"]["stderr"]
                .as_str()
                .unwrap()
                .contains(diagnostic),
            "{name}: {notice}"
        );
        assert!(
            notice["selection"]["reason"]
                .as_str()
                .unwrap()
                .starts_with("the caller's kind: "),
            "{name}: {notice}"
        );
        assert!(!notice.to_string().contains("ANSWER-"), "{name}: {notice}");
    }
}

#[test]
fn llm_rhai_without_a_harness_run_budget_ends_the_evaluation() {
    // The default budget is 0 runs. The call then ends the evaluation, and the
    // fallback never runs, because the configuration script cannot catch the breach.
    let sandbox = llm_sandbox(0);
    let claude = answering(&sandbox, r#"{"kind":"bugfix"}"#);

    let refusal = session(&sandbox, "bugfix").refusal(3);

    assert_eq!(refusal["error"]["code"], "harness_budget_exhausted");
    assert!(launched(&sandbox).is_none());
    assert!(!claude.argv.exists(), "the harness never started");

    // The firing control: one run of budget lets the same session through.
    let allowed = llm_sandbox(1);
    answering(&allowed, r#"{"kind":"bugfix"}"#);
    assert_eq!(session(&allowed, "bugfix").code, Some(0));
}
