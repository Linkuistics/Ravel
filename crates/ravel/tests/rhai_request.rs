//! Request schema 4 with skill delivery disabled, through the real `ravel start-session`
//! (`docs/specs/skill-discovery.md`, *Request and configuration script*, *Schemas and
//! identities* and *Test seams and acceptance*; `docs/specs/configuration-evaluation.md`,
//! *Frame bounds*).
//!
//! The rows "Disabled delivery", "Request and evaluation" and "Version transition"
//! of the acceptance table run here with delivery disabled, because this build
//! has no `skills` settings key yet. Tests 1 and 3 of *Frame bounds* run here too.
//! Test 2 is the unit test of the check in `rhai_worker::serve`.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
#[cfg(target_os = "macos")]
use std::path::Path;
use std::path::PathBuf;

use serde_json::{Value, json};
use support::rhai::{Recorded, rhai_script};
use support::{FRONT, Run, Sandbox, text, write};

/// The public bounds of a request, as `docs/specs/configuration-evaluation.md`
/// states them.
const ENTRY_BYTES: usize = 1024 * 1024;
const PROMPT_BYTES: usize = 1024 * 1024;
const TASK_ID_BYTES: usize = 1024;
const PARAMS_BYTES: usize = 64 * 1024;
const CONTEXT_BYTES: usize = 8 * 1024 * 1024;

/// The byte that JSON escapes to the most bytes, six: `\u0001`.
const ESCAPED: char = '\u{1}';

/// What the exact harness saw: each argument in its own file, so that empty
/// and multiline arguments survive.
struct Exact {
    dir: PathBuf,
}

impl Exact {
    fn args(&self) -> Vec<String> {
        let mut args = Vec::new();
        while let Ok(arg) = fs::read_to_string(self.dir.join(args.len().to_string())) {
            args.push(arg);
        }
        args
    }
}

/// A harness named `exact-agent` on the private `PATH`. It writes each of its
/// arguments to a numbered file, and then reports a result like `fake-agent`.
fn exact_agent(sandbox: &Sandbox) -> Exact {
    let dir = sandbox.tmp.join("exact-agent");
    fs::create_dir(&dir).unwrap();
    let script = sandbox.bin.join("exact-agent");
    write(
        &script,
        &format!(
            "#!/bin/sh\ni=0\nfor arg in \"$@\"; do\n  printf '%s' \"$arg\" > {dir:?}/$i\n  \
             i=$((i + 1))\ndone\nexec {FRONT:?} end-session done\n"
        ),
    );
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    Exact { dir }
}

/// A configuration script whose `select` routes to the exact harness with
/// `args`, a Rhai array expression.
fn exact_route(args: &str) -> String {
    rhai_script(&format!(
        r#"fn select(request, context) {{
    #{{ status: "selected", program: "exact-agent", args: {args},
       provider: "fixture", model: "fixture", effort: "high", reason: "exact" }}
}}"#
    ))
}

/// The arguments that show the three prompt fields of the request.
const PROMPT_FIELDS: &str = r#"[request.prompt, request.originalPrompt, type_of(request.skills), `${request.schemaVersion}`]"#;

#[test]
fn disabled_delivery_keeps_empty_newline_and_unicode_prompts_exact() {
    let prompts = [
        "",
        "\n",
        "line one\r\nline two\n\n",
        "\u{e9} \u{2014} \u{6f22}\u{5b57} \u{1f642}\u{2028}end",
    ];
    for prompt in prompts {
        for from_file in [true, false] {
            let sandbox = Sandbox::new();
            let exact = exact_agent(&sandbox);
            sandbox.personal_rhai(&exact_route(PROMPT_FIELDS));
            let file = sandbox.file("prompt.txt", prompt);
            let file = text(&file);
            let source = if from_file {
                ["--prompt-file", file.as_str()]
            } else {
                ["--prompt", prompt]
            };
            let mut args = vec!["--kind", "check", "--json"];
            args.extend_from_slice(&source);
            let report = sandbox.run(&args).report();
            assert_eq!(
                exact.args(),
                [prompt, prompt, "()", "4"],
                "{prompt:?} from a file: {from_file}"
            );
            assert_eq!(report["skills"], Value::Null, "{report}");
            // Disabled delivery does no cache work.
            assert!(!sandbox.home.join(".cache").exists());
        }
    }
}

#[test]
fn the_text_notice_does_not_change() {
    let sandbox = Sandbox::new();
    let exact = exact_agent(&sandbox);
    sandbox.personal_rhai(&exact_route(PROMPT_FIELDS));
    let run = sandbox.run(&["--kind", "check", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(exact.args(), ["p", "p", "()", "4"]);
    // Operational reports of the stop can follow the notice line.
    assert_eq!(
        run.stderr.lines().next(),
        Some("ravel: starting provider fixture, model fixture, effort high for kind \"check\""),
        "{}",
        run.stderr
    );
    assert!(!run.stderr.contains("skill"), "{}", run.stderr);
}

#[test]
fn load_context_and_select_receive_copies_of_one_request() {
    let sandbox = Sandbox::new();
    let exact = exact_agent(&sandbox);
    sandbox.personal_rhai(&rhai_script(
        r#"fn load_context(request) {
    let seen = request;
    request.prompt = "changed by the loader";
    request.originalPrompt = "changed by the loader";
    request.kind = "changed";
    request.params.added = "by the loader";
    #{ schemaVersion: 1, facts: #{ request: seen } }
}
fn select(request, context) {
    #{ status: "selected", program: "exact-agent",
       args: [`${context.facts.request == request}`, request.prompt, request.originalPrompt,
              request.kind, `${request.params.keys()}`, type_of(request.skills)],
       provider: "fixture", model: "fixture", effort: "high", reason: "exact" }
}"#,
    ));
    sandbox
        .run(&[
            "--kind",
            "check",
            "--prompt",
            "the task\n",
            "--param",
            "mode=fast",
            "--json",
        ])
        .report();
    assert_eq!(
        exact.args(),
        [
            "true",
            "the task\n",
            "the task\n",
            "check",
            "[\"mode\"]",
            "()"
        ]
    );
}

#[test]
fn top_level_code_sees_no_request() {
    let sandbox = Sandbox::new();
    let exact = exact_agent(&sandbox);
    sandbox.personal_rhai(&rhai_script(
        r#"const SEEN = request.prompt;
fn select(request, context) { () }"#,
    ));
    let run = sandbox.run(&["--kind", "check", "--prompt", "p", "--json"]);
    let error = run.refusal(3)["error"].clone();
    assert_eq!(error["code"], "config_parse_failed", "{error}");
    assert!(
        error["message"].as_str().unwrap().contains("request"),
        "{error}"
    );
    assert!(exact.args().is_empty(), "a harness started after a refusal");
}

#[test]
fn a_schema_3_script_refuses_without_skill_sources_and_names_both_changes() {
    for settings in [None, Some(r#"{ "timeoutMs": 30000 }"#)] {
        let sandbox = Sandbox::new();
        let recorded = sandbox.recording_agent();
        if let Some(settings) = settings {
            write(&sandbox.home.join(".config/ravel/settings.json"), settings);
        }
        let config = sandbox.personal_rhai(
            "export const SCHEMA_VERSION = 3;\nexport const VERSION = \"v\";\n\
             fn select(request, context) { () }\n",
        );
        let run = sandbox.run(&["--kind", "check", "--prompt", "p", "--json"]);
        let document = run.refusal(3);
        let error = &document["error"];
        assert_eq!(
            (&error["code"], &error["stage"], &error["exit"]),
            (
                &json!("unsupported_version"),
                &json!("validation"),
                &json!(3)
            ),
            "{document}"
        );
        assert_eq!(
            error["source"],
            text(&fs::canonicalize(config).unwrap()),
            "{document}"
        );
        let remedy = error["remedy"].as_str().unwrap();
        assert!(
            remedy.contains("export const SCHEMA_VERSION = 4;")
                && remedy.contains("request.originalPrompt"),
            "{remedy}"
        );
        assert_eq!(recorded.argv(), None, "a harness started after a refusal");
    }
}

/// A mismatched pair: the file at the path of the Ravel CLI is replaced by
/// another build before the CLI launches its worker. On macOS the CLI launches
/// the file now at its canonical path, so the worker reports the other build.
/// Linux launches `/proc/self/exe`, the running image, so a replaced file
/// cannot make a pair there. The unit tests of `rhai_worker::front` cover the
/// handshake on both platforms.
#[cfg(target_os = "macos")]
#[test]
fn a_mismatched_cli_and_worker_pair_refuses() {
    use std::io::Write as _;
    use std::process::Stdio;

    let route = rhai_script(
        r#"fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    );
    // The identity of this build, as its own routing notice reports it.
    let probe = Sandbox::new();
    probe.recording_agent();
    probe.personal_rhai(&route);
    let report = probe
        .run(&["--kind", "check", "--prompt", "p", "--json"])
        .report();
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    sandbox.personal_rhai(&route);
    let build = report["worker"]["buildId"].as_str().unwrap().to_owned();
    assert_eq!(build.len(), 64, "{report}");
    let front = sandbox.root.join("ravel");
    fs::copy(FRONT, &front).unwrap();
    let other = other_build(&sandbox.root.join("ravel-other"), &build);

    let prompt = sandbox.tmp.join("prompt.fifo");
    support::mkfifo(&prompt);
    let child = sandbox
        .command_for(&front)
        .args(["start-session", "--kind", "check", "--json"])
        .arg("--prompt-file")
        .arg(&prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // The CLI waits for the prompt before it resolves the worker's path.
    fs::rename(&other, &front).unwrap();
    fs::OpenOptions::new()
        .write(true)
        .open(&prompt)
        .unwrap()
        .write_all(b"p")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let run = Run {
        code: output.status.code(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };
    let error = run.refusal(5)["error"].clone();
    assert_eq!(error["code"], "worker_identity_mismatch", "{error}");
    assert!(
        error["message"].as_str().unwrap().contains(&build),
        "{error}"
    );
    assert_eq!(recorded.argv(), None, "a harness started after a refusal");
}

/// A copy of this build at `path` that reports another build identity, signed
/// again so that macOS runs it.
#[cfg(target_os = "macos")]
fn other_build(path: &Path, build: &str) -> PathBuf {
    let mut bytes = fs::read(FRONT).unwrap();
    let other: String = build
        .chars()
        .map(|digit| match digit {
            'f' => '0',
            '9' => 'a',
            digit => char::from(digit as u8 + 1),
        })
        .collect();
    let mut replaced = 0;
    let mut at = 0;
    while let Some(found) = bytes[at..]
        .windows(build.len())
        .position(|window| window == build.as_bytes())
    {
        let start = at + found;
        bytes[start..start + build.len()].copy_from_slice(other.as_bytes());
        replaced += 1;
        at = start + build.len();
    }
    assert!(replaced > 0, "the build identity is not in the executable");
    fs::write(path, bytes).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    let signed = std::process::Command::new("codesign")
        .args(["--force", "--sign", "-"])
        .arg(path)
        .output()
        .unwrap();
    assert!(signed.status.success(), "{signed:?}");
    path.to_owned()
}

/// The arguments of a request that fills every public bound with the largest
/// escape. `kind` is the caller's kind, which Ravel does not bound.
struct Filled {
    sandbox: Sandbox,
    recorded: Recorded,
    args: Vec<String>,
}

fn filled(kind: &str) -> Filled {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    let head = rhai_script(
        r#"fn select(request, context) {
    #{ status: "selected", program: "recording-agent", args: [], provider: "p",
       model: "m", effort: "e", reason: "r" }
}"#,
    );
    // A comment pads the entry to its bound with bytes that escape six-fold.
    let mut entry = format!("{head}// ");
    let padding = ENTRY_BYTES - entry.len() - 1;
    entry.extend(std::iter::repeat_n(ESCAPED, padding));
    entry.push('\n');
    assert_eq!(entry.len(), ENTRY_BYTES);
    sandbox.personal_rhai(&entry);

    let prompt = sandbox.file("prompt.txt", &ESCAPED.to_string().repeat(PROMPT_BYTES));
    // The delivered context carries the measurement of this document, so the
    // document leaves room for it within the budget.
    let mut summary = String::new();
    let document = loop {
        let document = json!({ "schemaVersion": 1, "summary": summary });
        let size = serde_json::to_vec(&document).unwrap().len();
        let room = CONTEXT_BYTES - 1024 - size;
        if room < 6 {
            break document;
        }
        summary.extend(std::iter::repeat_n(ESCAPED, room / 6));
    };
    let context = sandbox.file("context.json", &document.to_string());
    let task_id = ESCAPED.to_string().repeat(TASK_ID_BYTES);
    let param = format!("a={}", ESCAPED.to_string().repeat(PARAMS_BYTES - 1));
    let args = [
        "--kind",
        kind,
        "--json",
        "--prompt-file",
        &text(&prompt),
        "--context",
        &text(&context),
        "--context-bytes",
        &CONTEXT_BYTES.to_string(),
        "--task-id",
        &task_id,
        "--param",
        &param,
        "--timeout-ms",
        "600000",
    ]
    .map(str::to_owned)
    .to_vec();
    Filled {
        sandbox,
        recorded,
        args,
    }
}

impl Filled {
    fn run(&self) -> Run {
        let args = self.args.iter().map(String::as_str).collect::<Vec<_>>();
        self.sandbox.run(&args)
    }

    /// The same request from a working directory `depth` levels below the
    /// sandbox's, each level a name of 255 escaped bytes. The path is too long
    /// for one `chdir`, so the child enters it one level at a time.
    fn run_deep(&self, depth: usize) -> Run {
        use std::ffi::CString;
        use std::os::unix::process::CommandExt as _;

        let name = CString::new(ESCAPED.to_string().repeat(255)).unwrap();
        let cwd = CString::new(text(&self.sandbox.cwd)).unwrap();
        // SAFETY: each call gets a valid NUL-terminated name and a descriptor
        // that this loop opened and owns.
        unsafe {
            let mut parent = libc::open(cwd.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
            assert!(parent >= 0);
            for _ in 0..depth {
                assert_eq!(libc::mkdirat(parent, name.as_ptr(), 0o700), 0);
                let child = libc::openat(parent, name.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
                assert!(child >= 0);
                libc::close(parent);
                parent = child;
            }
            libc::close(parent);
        }
        let mut command = self.sandbox.command();
        command.arg("start-session").args(&self.args);
        // SAFETY: `chdir` is async-signal-safe, and the strings live until the
        // command has run.
        unsafe {
            command.pre_exec(move || {
                for _ in 0..depth {
                    if libc::chdir(name.as_ptr()) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                Ok(())
            });
        }
        support::run(&mut command)
    }
}

/// Test 1 of *Frame bounds*: a request at every public bound, with the largest
/// escape, fits and evaluates.
#[test]
fn a_request_at_every_public_bound_fits_the_frame_bound_and_evaluates() {
    let filled = filled("check");
    let report = filled.run().report();
    let encoded = report["context"]["encodedBytes"].as_u64().unwrap();
    assert!(encoded > 8_380_000, "{encoded}");
    assert!(
        filled.recorded.argv().is_some(),
        "the harness did not start"
    );
}

/// Test 3 of *Frame bounds*: the same request with a long kind takes its encoded
/// frame over the bound, and refuses as caller input before evaluation. A
/// removed check gives another failure, so this test detects that removal.
///
/// It runs where the platform admits such a kind. Six escaped bytes for each of
/// 600,000 bytes of kind take the frame over the bound, and macOS admits 1 MiB
/// of arguments and environment together. GNU/Linux admits 131,071 bytes in one
/// argument, so no kind can pass the bound there
/// (`docs/skill-discovery-design-evidence.md`, *Frame-bound platforms*).
#[cfg(target_os = "macos")]
#[test]
fn a_long_kind_takes_the_frame_over_the_bound_and_refuses_before_evaluation() {
    let kind = ESCAPED.to_string().repeat(600_000);
    let filled = filled(&kind);
    let run = filled.run();
    let document = run.refusal(2);
    let error = &document["error"];
    assert_eq!(
        (&error["code"], &error["stage"], &error["exit"]),
        (&json!("malformed_input"), &json!("cli"), &json!(2)),
        "{}",
        run.stderr
    );
    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains("over the frame bound of 38541312 bytes")
            && message.contains("largest members are selectContext")
            && message.contains("request.kind (3600002 bytes)"),
        "{message}"
    );
    assert_eq!(
        filled.recorded.argv(),
        None,
        "a harness started after a refusal"
    );
}

/// A variant of test 3 for each release platform: a deep working directory,
/// which Ravel does not bound, takes the frame over the bound. The platform did
/// not bound a working directory of 600,000 bytes on macOS or GNU/Linux
/// (`docs/skill-discovery-design-evidence.md`, *Frame-bound platforms*). The
/// directory here is 2,000 levels of 256 bytes, and each of its 510,000 escaped
/// bytes encodes to six.
#[test]
fn a_deep_working_directory_takes_the_frame_over_the_bound_and_refuses() {
    let filled = filled("check");
    let run = filled.run_deep(2_000);
    let document = run.refusal(2);
    let error = &document["error"];
    assert_eq!(
        (&error["code"], &error["stage"], &error["exit"]),
        (&json!("malformed_input"), &json!("cli"), &json!(2)),
        "{}",
        run.stderr
    );
    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains("over the frame bound of 38541312 bytes")
            && message.contains("request.cwd ("),
        "{message}"
    );
    assert_eq!(
        filled.recorded.argv(),
        None,
        "a harness started after a refusal"
    );
}
