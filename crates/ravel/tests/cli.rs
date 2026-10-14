//! Actual command boundary: temporary policies and fixture processes only.
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

struct Fixture {
    _dir: tempfile::TempDir,
    root: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        for path in ["home", "work", "tmp"] {
            fs::create_dir(root.join(path)).unwrap();
        }
        Self { _dir: dir, root }
    }
    fn front() -> PathBuf {
        PathBuf::from(option_env!("CARGO_BIN_EXE_ravel").unwrap_or("target/debug/ravel"))
    }
    fn command(&self) -> Command {
        let mut command = Command::new(Self::front());
        command
            .env_clear()
            .env("HOME", self.root.join("home"))
            .env("TMPDIR", self.root.join("tmp"))
            .env("PATH", "/usr/bin:/bin")
            .current_dir(self.root.join("work"));
        command
    }
    fn start(&self, policy: &str, prompt: &str) -> Output {
        let path = self.root.join("policy.ts");
        fs::write(&path, policy).unwrap();
        self.command()
            .args(["start-session", "--kind", "check", "--config"])
            .arg(path)
            .args(["--prompt", prompt])
            .output()
            .unwrap()
    }
    fn selecting(&self, script: &str, result: &str) -> String {
        let front = format!("{:?}", Self::front().to_str().unwrap());
        format!(
            r#"import {{ definePolicy }} from "ravel/sdk";
export const policy = definePolicy({{
 schemaVersion: 2, version: "cli-test",
 select(request) {{ return {{ status: "selected", program: "/bin/sh",
 args: ["-c", {script:?}, "fixture", {front}, {result:?}, request.prompt],
 provider: "fixture", model: "fixture", effort: "fixture", reason: "command seam" }}; }}
}});"#
        )
    }
}

#[test]
fn two_verbs_help_version_and_no_compatibility_commands() {
    let fixture = Fixture::new();
    let help = fixture.command().arg("--help").output().unwrap();
    assert!(help.status.success());
    let text = String::from_utf8(help.stdout).unwrap();
    assert!(text.contains("start-session") && text.contains("end-session"));
    for verb in ["run", "exit", "inspect", "init", "record", "help"] {
        assert!(
            !fixture
                .command()
                .arg(verb)
                .output()
                .unwrap()
                .status
                .success(),
            "{verb}"
        );
    }
    let version = fixture.command().arg("--version").output().unwrap();
    assert_eq!(
        version.stdout,
        format!("ravel {}\n", env!("CARGO_PKG_VERSION")).into_bytes()
    );
}

#[test]
fn routed_result_is_exact_prompt_is_unchanged_and_child_stdout_is_separate() {
    for result in ["ordinary", "", "first\nsecond\n", "--help", "résultat 🪶"] {
        let fixture = Fixture::new();
        let script =
            r#"printf 'child output\n'; printf '%s' "$3" >&2; "$1" end-session -- "$2"; exit 7"#;
        let policy = fixture.selecting(script, result);
        let prompt = "unaltered\n prompt 🪶 --help\n";
        let output = fixture.start(&policy, prompt);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, result.as_bytes());
        let diagnostics = String::from_utf8(output.stderr).unwrap();
        assert!(diagnostics.contains("child output\n") && diagnostics.contains(prompt));
        assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
    }
}

#[test]
fn refused_policy_never_launches_and_unreported_exit_is_failure() {
    let fixture = Fixture::new();
    let refused = fixture.start(r#"export const policy = { schemaVersion: 2, version: "1",
select() { return {status: "refused", code: "no_route", message: "no route", remedy: "choose a route"}; }};"#, "caller");
    assert_eq!(refused.status.code(), Some(3));
    assert!(refused.stdout.is_empty());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("no_route"));
    let exit = fixture.start(&fixture.selecting("exit 0", "unused"), "caller");
    assert_eq!(exit.status.code(), Some(1));
    assert!(exit.stdout.is_empty());
    assert!(String::from_utf8_lossy(&exit.stderr).contains("without a result"));
}

#[test]
fn end_requires_one_result_and_live_inherited_context() {
    let fixture = Fixture::new();
    for args in [
        vec!["end-session"],
        vec!["end-session", "one", "two"],
        vec!["end-session", "value"],
    ] {
        let output = fixture.command().args(args).output().unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
    }
    let output = fixture
        .command()
        .args(["end-session", "value"])
        .env("RAVEL_EXIT_FILE", fixture.root.join("lost/result"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn context_reads_params_choice_and_environment_grants_reach_policy() {
    let fixture = Fixture::new();
    let file = fixture.root.join("work/data.json");
    fs::write(file, r#"{"answer":"from context"}"#).unwrap();
    fs::write(fixture.root.join("work/.ravel-choice"), "careful").unwrap();
    let mut policy = fixture.selecting(r#""$1" end-session "$2""#, "unused");
    policy = policy.replace("definePolicy }", "definePolicy, readChoice }")
        .replace("select(request) { return", r#"loadContext(request, host) {
 const read = host.readJson('data.json');
 return {schemaVersion: 1, facts: read.value, sources: [read.source]};
}, select(request, context, host) {
 if (process.env.RAVEL_EXIT_FILE || process.env.HARNESS_DISPATCH_EXIT_FILE) throw new Error('ambient session authority');
 if (process.env.TOKEN !== 'granted') throw new Error('no granted token');
 if (process.env.UNGRANTED !== undefined) throw new Error('ungranted environment');
 const choice = readChoice(request.cwd, ['careful']);
 if (choice?.[0] !== 'careful') throw new Error('choice was not read');
 if (context.measured[0].via !== 'readJson' || context.measured[0].bytes !== 25) throw new Error('read not measured');
 if (request.params.profile !== 'exact\ntext') throw new Error('parameters changed');
 console.log('policy stdout'); host.diagnostic('policy stderr');
 return"#)
        .replace("\"unused\", request.prompt", "context.facts.answer, request.prompt");
    let path = fixture.root.join("policy.ts");
    fs::write(&path, policy).unwrap();
    let output = fixture
        .command()
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "same",
            "--config",
        ])
        .arg(path)
        .args(["--policy-env", "TOKEN", "--param", "profile=exact\ntext"])
        .env("TOKEN", "granted")
        .env("UNGRANTED", "private")
        .env(
            "HARNESS_DISPATCH_EXIT_FILE",
            fixture.root.join("old-never-created"),
        )
        .env("RAVEL_EXIT_FILE", fixture.root.join("new-never-created"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"from context");
    let diagnostics = String::from_utf8(output.stderr).unwrap();
    assert!(diagnostics.contains("policy stdout") && diagnostics.contains("policy stderr"));
    assert!(!fixture.root.join("old-never-created").exists());
    assert!(!fixture.root.join("new-never-created").exists());
}

#[test]
fn settings_bounds_and_discarded_storage_refuse_without_launching() {
    let fixture = Fixture::new();
    let config = fixture.root.join("home/.config/ravel");
    fs::create_dir_all(&config).unwrap();
    fs::write(config.join("settings.json"), r#"{"contextBytes": 1}"#).unwrap();
    let policy = fixture.selecting("exit 0", "unused").replace(
        "select(request)",
        "loadContext() { return { schemaVersion: 1, facts: {} }; }, select(request)",
    );
    let output = fixture.start(&policy, "caller");
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("context_too_large"));
    fs::write(
        config.join("settings.json"),
        format!(r#"{{"stateDir":{:?}}}"#, fixture.root.join("records")),
    )
    .unwrap();
    let output = fixture.start(&policy, "caller");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("settings_invalid"));
    assert!(!fixture.root.join("records").exists());
    fs::remove_file(config.join("settings.json")).unwrap();
    for name in [
        "RAVEL_EXIT_FILE",
        "HARNESS_DISPATCH_EXIT_FILE",
        "BUN_OPTIONS",
    ] {
        let path = fixture.root.join("policy.ts");
        let output = fixture
            .command()
            .args([
                "start-session",
                "--kind",
                "check",
                "--prompt",
                "caller",
                "--config",
            ])
            .arg(path)
            .args(["--policy-env", name])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn await_file(path: &std::path::Path) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !path.exists() {
        assert!(
            std::time::Instant::now() < deadline,
            "fixture never became ready: {}",
            path.display()
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn cancellation_during_selection_launches_nothing_and_during_session_reaps_group() {
    use std::os::unix::process::ExitStatusExt as _;
    use std::process::Stdio;
    for selection in [true, false] {
        let fixture = Fixture::new();
        let marker = fixture.root.join("ready");
        let path = fixture.root.join("policy.ts");
        let policy = if selection {
            format!(
                r#"import {{writeFileSync}} from 'node:fs';
export const policy = {{schemaVersion: 2, version: '1', select() {{writeFileSync({marker:?}, 'ready'); for (;;) {{}}}}}};"#
            )
        } else {
            fixture.selecting(
                &format!("echo $$ > {marker:?}; while :; do sleep 1; done"),
                "unused",
            )
        };
        fs::write(&path, policy).unwrap();
        let front = fixture
            .command()
            .args([
                "start-session",
                "--kind",
                "check",
                "--prompt",
                "caller",
                "--config",
            ])
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        await_file(&marker);
        // SAFETY: signal only the PID of this fixture's live child.
        assert_eq!(unsafe { libc::kill(front.id() as i32, libc::SIGTERM) }, 0);
        let output = front.wait_with_output().unwrap();
        assert_eq!(output.status.signal(), Some(libc::SIGTERM));
        assert!(output.stdout.is_empty());
        assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
        if !selection {
            let child: i32 = fs::read_to_string(marker).unwrap().trim().parse().unwrap();
            // SAFETY: signal zero checks whether the fixture process/group remains.
            assert_eq!(unsafe { libc::kill(child, 0) }, -1);
            assert_eq!(unsafe { libc::kill(-child, 0) }, -1);
        }
    }
}

#[test]
fn nested_command_context_is_isolated_and_first_result_wins() {
    let fixture = Fixture::new();
    let inner = fixture.root.join("inner.ts");
    let inner_policy = fixture.selecting(
        r#""$1" end-session inner; "$1" end-session ignored"#,
        "unused",
    );
    fs::write(&inner, inner_policy).unwrap();
    let script = r#"original=$RAVEL_EXIT_FILE; result=$("$1" start-session --kind check --config "$2" --prompt nested) || exit 91; [ "$original" = "$RAVEL_EXIT_FILE" ] || exit 92; "$1" end-session "$result|outer""#;
    let output = fixture.start(&fixture.selecting(script, inner.to_str().unwrap()), "outer");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"inner|outer");
    assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
}

#[test]
fn confined_route_delivers_result_and_overlapping_owner_paths_refuse_before_policy() {
    let fixture = Fixture::new();
    let owner = fixture.root.join("owner");
    fs::create_dir(&owner).unwrap();
    let policy = owner.join("policy.ts");
    fs::write(
        &policy,
        fixture.selecting(
            r#"[ -z "${PRIVATE_TOKEN-}" ] || exit 91; "$1" end-session "$2""#,
            "confined",
        ),
    )
    .unwrap();
    let output = fixture
        .command()
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "caller",
            "--confine",
            "--config",
        ])
        .arg(&policy)
        .env("PRIVATE_TOKEN", "must not reach confined child")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"confined");
    assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
    let overlap = fixture
        .command()
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "caller",
            "--confine",
            "--config",
        ])
        .arg(policy)
        .args(["--runtime-read"])
        .arg(owner.join("policy.ts"))
        .output()
        .unwrap();
    assert_eq!(overlap.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&overlap.stderr).contains("confinement_overlap"));
    assert!(overlap.stdout.is_empty());
}

#[test]
fn selection_deadline_stops_real_worker_and_missing_or_mismatched_pair_refuses() {
    use std::os::unix::fs::PermissionsExt as _;
    let fixture = Fixture::new();
    let policy = fixture.root.join("policy.ts");
    fs::write(
        &policy,
        r#"export const policy = {schemaVersion: 2, version: '1', select() { for (;;) {} }};"#,
    )
    .unwrap();
    let output = fixture
        .command()
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "caller",
            "--timeout-ms",
            "1000",
            "--config",
        ])
        .arg(&policy)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(124));
    assert!(output.stdout.is_empty());
    let prefix = fixture.root.join("prefix");
    fs::create_dir_all(prefix.join("bin")).unwrap();
    let front = prefix.join("bin/ravel");
    fs::copy(Fixture::front(), &front).unwrap();
    let invoke = || {
        let mut command = {
            let mut local = Command::new(&front);
            local
                .env_clear()
                .env("HOME", fixture.root.join("home"))
                .env("TMPDIR", fixture.root.join("tmp"))
                .env("PATH", "/usr/bin:/bin")
                .current_dir(fixture.root.join("work"));
            local
        };
        command
            .args([
                "start-session",
                "--kind",
                "check",
                "--prompt",
                "caller",
                "--config",
            ])
            .arg(&policy)
            .output()
            .unwrap()
    };
    let missing = invoke();
    assert_eq!(missing.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("worker_missing"));
    let worker = prefix.join("libexec/ravel/ravel-policy");
    fs::create_dir_all(worker.parent().unwrap()).unwrap();
    let hello = r#"{"type":"hello","protocol":1,"packageVersion":"0.0.0","buildId":"wrong-source","bunVersion":"1.4.2"}"#;
    let length = u32::try_from(hello.len()).unwrap().to_be_bytes();
    let prefix_bytes: String = length.iter().map(|byte| format!("\\{byte:03o}")).collect();
    fs::write(
        &worker,
        format!("#!/bin/sh\nprintf '{prefix_bytes}' >&3\nprintf '%s' '{hello}' >&3\n"),
    )
    .unwrap();
    fs::set_permissions(&worker, fs::Permissions::from_mode(0o755)).unwrap();
    let mismatched = invoke();
    assert_eq!(mismatched.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&mismatched.stderr).contains("worker_identity_mismatch"));
    assert!(mismatched.stdout.is_empty());
    assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
}

#[test]
fn unwritable_result_stdout_refuses_before_launch_and_reports_private_cleanup_failure() {
    use std::os::unix::fs::PermissionsExt as _;
    let fixture = Fixture::new();
    let owner = fixture.root.join("owner");
    fs::create_dir(&owner).unwrap();
    let policy = owner.join("policy.ts");
    let code = fixture
        .selecting("exit 91", "unused")
        .replace(
            "import { definePolicy }",
            "import { chmodSync } from 'node:fs';\nimport { definePolicy }",
        )
        .replace(
            "select(request) {",
            "select(request) { chmodSync(process.env.TMPDIR, 0o500);",
        );
    fs::write(&policy, code).unwrap();
    let mut command = fixture.command();
    command
        .args([
            "start-session",
            "--kind",
            "check",
            "--prompt",
            "caller",
            "--confine",
            "--config",
        ])
        .arg(policy);
    command.stdout(std::process::Stdio::from(
        fs::File::open("/dev/null").unwrap(),
    ));
    let output = command.output().unwrap();
    fs::set_permissions(fixture.root.join("tmp"), fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(output.status.code(), Some(5));
    let diagnostics = String::from_utf8(output.stderr).unwrap();
    assert!(diagnostics.contains("output_unavailable"), "{diagnostics}");
    assert!(
        diagnostics.contains("cannot remove confined run directory"),
        "{diagnostics}"
    );
    assert!(output.stdout.is_empty());
}
