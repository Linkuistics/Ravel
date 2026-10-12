//! Hostile directories, environments and policy output, through the public
//! launcher (`docs/specs/session-execution.md`, Policy authority and runtime
//! discovery; `docs/runtime-evidence.md`, Controls and firing configurations).
//!
//! Each class stays inert through the front, and in the same test its
//! fixture is seen to fire in the class's firing configuration, so a clean
//! result cannot come from a fixture that never could have fired. The
//! configurations that need a probe build, or the shipped worker without the
//! front's scrubbing, drive the worker directly (`support::direct`). The cwd
//! `package.json` class fires for an entry explicitly admitted in that directory.

mod support;

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};
use support::direct::{self, probe_build, shipped_worker, Probe};
use support::{run, text, Sandbox, DEEP, FRONT, ROUTED};

/// The reason `ROUTED`'s `select` gives: seen in a report, it shows that the
/// policy the test installed is the one that selected.
const ROUTED_REASON: &str = "fixture route";

/// A module that writes `fired` to `sentinel` when it is imported or preloaded.
fn sentinel_module(sentinel: &Path) -> String {
    format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\nexport const shadowed = true;\n",
        text(sentinel)
    )
}

/// The environment a directly driven worker gets: the front's base set.
fn base_env(sandbox: &Sandbox) -> Vec<(&'static str, OsString)> {
    vec![
        ("HOME", sandbox.home.clone().into()),
        ("PATH", "/usr/bin:/bin".into()),
        ("TMPDIR", sandbox.tmp.clone().into()),
    ]
}

/// The shipped worker's build identity, as the checkout's front reports it.
fn shipped_build() -> String {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let report = sandbox.start(&["--kind", "check", "--json"]).report();
    report["worker"]["buildId"].as_str().unwrap().to_owned()
}

/// Assert that a probe announced itself, built from this checkout's source.
fn assert_probe_identity(driven: &direct::Driven, probe: Probe, shipped: &str) {
    let hello = driven.hello.as_ref().unwrap_or_else(|| {
        panic!(
            "the {} probe sent no hello\nstderr: {}",
            probe.name(),
            driven.stderr
        )
    });
    assert_eq!(
        hello["buildId"],
        format!("probe-{}-{shipped}", probe.name()),
        "the {} probe is stale; run `task probes`",
        probe.name()
    );
}

fn read_json(path: &Path) -> Value {
    let view = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{} was not written: {error}", path.display()));
    fs::remove_file(path).unwrap();
    serde_json::from_str(&view).unwrap()
}

/// A policy that records, at import, what the dotenv fixtures set.
fn dotenv_view_policy(view: &Path) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
writeFileSync({view:?}, JSON.stringify({{
  dotenv: process.env.HOSTILE_DOTENV ?? null,
  dotenvLocal: process.env.HOSTILE_DOTENV_LOCAL ?? null,
}}));
{ROUTED}"#,
        view = text(view)
    )
}

#[test]
fn a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let preloaded = sandbox.root.join("bunfig-preload-ran");
    let view = sandbox.root.join("view.json");
    // The caller's cwd is the hostile directory.
    sandbox.file(".env", "HOSTILE_DOTENV=from-dotenv\n");
    sandbox.file(".env.local", "HOSTILE_DOTENV_LOCAL=from-dotenv-local\n");
    sandbox.file("bunfig.toml", "preload = [\"./hostile-preload.ts\"]\n");
    sandbox.file("hostile-preload.ts", &sentinel_module(&preloaded));
    let entry = sandbox.personal_policy(&dotenv_view_policy(&view));
    let inert = json!({ "dotenv": null, "dotenvLocal": null });

    // Through the public launcher, the public command.
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(read_json(&view), inert);
    let ran = sandbox.run(&["--kind", "check", "--prompt", "p"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert_eq!(read_json(&view), inert);
    assert!(
        !preloaded.exists(),
        "a cwd bunfig preload ran through the front"
    );

    // The firing configuration: the autoload probe, run in the hostile
    // directory, loads both files and runs the preload.
    let driven = direct::drive(
        &probe_build(Probe::Autoload),
        &sandbox.cwd,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Autoload, &shipped);
    driven.loaded();
    assert_eq!(
        read_json(&view),
        json!({ "dotenv": "from-dotenv", "dotenvLocal": "from-dotenv-local" })
    );
    assert!(
        preloaded.exists(),
        "the autoload probe never ran the preload"
    );
    fs::remove_file(&preloaded).unwrap();

    // Either control alone keeps the class inert: the shipped worker in the
    // hostile directory, and the autoload probe in an empty one, as the
    // front's private directory is.
    let empty = sandbox.root.join("empty");
    fs::create_dir(&empty).unwrap();
    for (worker, cwd) in [
        (shipped_worker(), &sandbox.cwd),
        (probe_build(Probe::Autoload), &empty),
    ] {
        direct::drive(&worker, cwd, &base_env(&sandbox), &entry).loaded();
        assert_eq!(read_json(&view), inert, "{}", worker.display());
        assert!(!preloaded.exists(), "{}", worker.display());
    }
}

/// A module for a native `Worker`, which reports to the thread that started
/// it what the dotenv fixtures set in its own environment.
const DOTENV_REPORTING_WORKER: &str = r#"postMessage({
  dotenv: process.env.HOSTILE_DOTENV ?? null,
  dotenvLocal: process.env.HOSTILE_DOTENV_LOCAL ?? null,
});
"#;

/// A policy that moves the worker into `dir`, starts a native `Worker`
/// there from `module`, and records what the dotenv fixtures set in each VM:
/// `started` is the `Worker`'s report, and `moved` is read afterwards in the
/// VM that made the move.
fn dotenv_view_policy_moved_into(dir: &Path, module: &Path, view: &Path) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
process.chdir({dir:?});
const started = await new Promise((resolve, reject) => {{
  const worker = new Worker({module:?});
  worker.onmessage = (event) => {{ resolve(event.data); worker.terminate(); }};
  worker.onerror = (event) => reject(new Error(event.message));
}});
writeFileSync({view:?}, JSON.stringify({{
  moved: {{
    dotenv: process.env.HOSTILE_DOTENV ?? null,
    dotenvLocal: process.env.HOSTILE_DOTENV_LOCAL ?? null,
  }},
  started,
}}));
{ROUTED}"#,
        dir = text(dir),
        module = text(module),
        view = text(view)
    )
}

#[test]
fn a_dotenv_where_a_policy_starts_a_worker_stays_inert_and_fires_under_the_autoload_probe() {
    // Bun loads dotenv files again for each VM it starts, from the directory
    // the process is then in. The shipped worker has moved to `/` by the time
    // a policy can start a native `Worker`, and `/` can hold no fixture, so
    // the policy moves into the hostile directory first. The front's private
    // start directory is no control here: the dotenv switch alone is.
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let view = sandbox.root.join("view.json");
    let hostile = sandbox.root.join("moved-into");
    support::write(&hostile.join(".env"), "HOSTILE_DOTENV=from-dotenv\n");
    support::write(
        &hostile.join(".env.local"),
        "HOSTILE_DOTENV_LOCAL=from-dotenv-local\n",
    );
    let module = sandbox.root.join("reporting-worker.ts");
    support::write(&module, DOTENV_REPORTING_WORKER);
    let entry = sandbox.personal_policy(&dotenv_view_policy_moved_into(&hostile, &module, &view));
    let inert = json!({ "dotenv": null, "dotenvLocal": null });
    let fired = json!({ "dotenv": "from-dotenv", "dotenvLocal": "from-dotenv-local" });

    // Through the public launcher, the public command.
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(read_json(&view)["started"], inert);
    let ran = sandbox.run(&["--kind", "check", "--prompt", "p"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert_eq!(read_json(&view)["started"], inert);

    // The firing configuration: the autoload probe, started in an empty
    // directory as the front's private one is, loads both files in the
    // `Worker` the policy starts.
    let empty = sandbox.root.join("empty");
    fs::create_dir(&empty).unwrap();
    let driven = direct::drive(
        &probe_build(Probe::Autoload),
        &empty,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Autoload, &shipped);
    driven.loaded();
    assert_eq!(
        read_json(&view)["started"],
        fired,
        "the autoload probe's `Worker` never loaded the dotenv files"
    );

    // The switch alone is the difference: the shipped worker, started in that
    // same directory with the same entry, loads nothing.
    direct::drive(&shipped_worker(), &empty, &base_env(&sandbox), &entry).loaded();
    assert_eq!(read_json(&view)["started"], inert);

    // And where the process is when the `Worker` starts is what the runtime
    // reads: the autoload probe, moved into an empty directory instead, loads
    // nothing. A worker that stayed in its empty start directory had that
    // second control, and one that moves to `/` does not.
    let elsewhere = sandbox.root.join("moved-into-empty");
    fs::create_dir(&elsewhere).unwrap();
    let entry = sandbox.personal_policy(&dotenv_view_policy_moved_into(&elsewhere, &module, &view));
    direct::drive(
        &probe_build(Probe::Autoload),
        &empty,
        &base_env(&sandbox),
        &entry,
    )
    .loaded();
    assert_eq!(read_json(&view)["started"], inert);
}

#[test]
fn bun_runtime_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let preloaded = sandbox.root.join("options-preload-ran");
    let preload = sandbox.root.join("options-preload.ts");
    support::write(&preload, &sentinel_module(&preloaded));
    let options = format!("--preload {}", text(&preload));

    // Through the public launcher the worker is itself: it identifies itself,
    // evaluates the policy and runs no preload.
    for (name, value) in [("BUN_OPTIONS", options.as_str()), ("BUN_BE_BUN", "1")] {
        let mut command = sandbox.command();
        command.env(name, value).args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ]);
        let report = run(&mut command).report();
        assert_eq!(report["selection"]["reason"], ROUTED_REASON, "{name}");
        assert!(!preloaded.exists(), "{name} preloaded through the front");
    }

    // The firing configurations: the shipped worker, started directly with
    // each variable. BUN_OPTIONS preloads the module before the worker's
    // own code, and BUN_BE_BUN turns the worker into Bun, running any code
    // it is handed.
    let mut env = base_env(&sandbox);
    env.push(("BUN_OPTIONS", options.clone().into()));
    let entry = sandbox.personal_path();
    direct::drive(&shipped_worker(), &sandbox.root, &env, &entry).loaded();
    assert!(preloaded.exists(), "BUN_OPTIONS never preloaded");

    let be_bun = sandbox.root.join("be-bun-ran");
    let output = Command::new(shipped_worker())
        .env_clear()
        .envs(base_env(&sandbox))
        .env("BUN_BE_BUN", "1")
        .args([
            "-e",
            &format!(
                "require(\"node:fs\").writeFileSync({:?}, \"fired\")",
                text(&be_bun)
            ),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(be_bun.exists(), "BUN_BE_BUN did not make the worker Bun");
}

/// A policy of at least 4 KiB, from which size Bun caches an imported file's
/// transpiled output. Its version is `admitted`, which its `select` gives as
/// its reason, so the version a worker reports and the reason a selection
/// carries both say which text of the file ran.
fn cacheable_policy() -> String {
    format!(
        r#"export const policy = {{
  schemaVersion: 2,
  version: "admitted",
  select(request) {{ return {{ ...{DEEP}, reason: this.version }}; }},
}};
// {}
"#,
        "padding ".repeat(640)
    )
}

/// Every entry of Bun's runtime transpiler cache under `dir`, each named
/// `<input hash>.pile`.
fn cache_entries(dir: &Path) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            entries.extend(cache_entries(&path));
        } else if path
            .extension()
            .is_some_and(|extension| extension == "pile")
        {
            entries.push(path);
        }
    }
    entries
}

/// Change a cached `cacheable_policy`'s version from `admitted` to `poisoned`,
/// leaving everything that ties the entry to the policy file's bytes. The
/// header is bun-v1.4.2's (`src/jsc/RuntimeTranspilerCache.rs`,
/// `Metadata::encode`): version 28 as a little-endian u32, then the module
/// type and output encoding bytes, then little-endian u64s, among them the
/// output's offset at byte 30, its length at 38 and its hash at 46. A zero
/// hash is never checked, so the altered output needs no new one.
fn poison(entry: &Path) {
    let mut bytes = fs::read(entry).unwrap();
    assert_eq!(
        bytes[..4],
        28u32.to_le_bytes(),
        "{} is not in Bun 1.4.2's cache layout; re-derive this control at each Bun upgrade",
        entry.display()
    );
    assert!(
        [1, 3].contains(&bytes[5]),
        "the cached output is not UTF-8 or Latin-1"
    );
    let field = |at: usize| {
        usize::try_from(u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap())).unwrap()
    };
    let (offset, length) = (field(30), field(38));
    let output = &mut bytes[offset..offset + length];
    let (from, to) = (b"version: \"admitted\"", b"version: \"poisoned\"");
    let at = output
        .windows(from.len())
        .position(|window| window == from)
        .expect("the cached output holds the version admitted");
    output[at..at + from.len()].copy_from_slice(to);
    bytes[46..54].fill(0);
    fs::write(entry, bytes).unwrap();
}

#[test]
fn the_runtime_transpiler_cache_stays_inert_through_the_front_and_fires_in_the_worker_started_directly(
) {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&cacheable_policy());
    assert!(fs::metadata(&entry).unwrap().len() >= 4096);

    // Through the public launcher, the worker writes no cache under HOME.
    let report = sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(report["selection"]["reason"], "admitted");
    assert_eq!(
        cache_entries(&sandbox.home),
        Vec::<PathBuf>::new(),
        "the front's worker wrote Bun's cache"
    );

    // The firing configuration: the shipped worker, started directly without
    // the front's setting, caches the policy under HOME. With the cached
    // output altered, the same file is the policy the cache says.
    let drive = |env: &[(&str, OsString)]| {
        direct::drive(&shipped_worker(), &sandbox.root, env, &entry).loaded()["policy"]["version"]
            .clone()
    };
    assert_eq!(drive(&base_env(&sandbox)), "admitted");
    let entries = cache_entries(&sandbox.home);
    assert_eq!(entries.len(), 1, "the worker cached nothing under HOME");
    poison(&entries[0]);
    assert_eq!(
        drive(&base_env(&sandbox)),
        "poisoned",
        "the altered cache under HOME never fired"
    );
    // The same entry under a cache that XDG_CACHE_HOME names, which Bun
    // prefers to HOME's, fires the same way.
    let xdg = sandbox.root.join("xdg-cache");
    fs::create_dir_all(xdg.join("bun/@t@")).unwrap();
    fs::copy(
        &entries[0],
        xdg.join("bun/@t@").join(entries[0].file_name().unwrap()),
    )
    .unwrap();
    let empty_home = sandbox.root.join("empty-home");
    fs::create_dir(&empty_home).unwrap();
    let mut env = base_env(&sandbox);
    env.retain(|(name, _)| *name != "HOME");
    env.push(("HOME", empty_home.into()));
    env.push(("XDG_CACHE_HOME", xdg.clone().into()));
    assert_eq!(
        drive(&env),
        "poisoned",
        "the altered cache under XDG_CACHE_HOME never fired"
    );

    // Through the public launcher, beside both altered caches, the admitted
    // file selects, the XDG_CACHE_HOME one granted.
    let report = sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(report["policy"]["version"], "admitted");
    assert_eq!(report["selection"]["reason"], "admitted");
    let mut command = sandbox.command();
    command.env("XDG_CACHE_HOME", &xdg).args([
        "start-session",
        "--prompt",
        "p",
        "--kind",
        "check",
        "--policy-env",
        "XDG_CACHE_HOME",
        "--json",
    ]);
    let report = run(&mut command).report();
    assert_eq!(report["selection"]["reason"], "admitted");
    assert_eq!(
        report["policyEnvironment"],
        json!([{ "name": "XDG_CACHE_HOME", "set": true }])
    );
}

/// A policy whose version is whichever `dep` its helper imports. The helper
/// sits in `lib/real`, reached through the directory symlink
/// `policies/linked`, and `lib` and `policies` each hold a `dep` named for
/// its side: the real directory's parent, or the link's.
fn symlinked_helper_policy(root: &Path) -> PathBuf {
    support::write(
        &root.join("lib/real/helper.ts"),
        "export { which } from \"dep\";\n",
    );
    support::write(
        &root.join("lib/node_modules/dep/index.js"),
        "export const which = \"real\";\n",
    );
    support::write(
        &root.join("policies/node_modules/dep/index.js"),
        "export const which = \"link\";\n",
    );
    std::os::unix::fs::symlink("../lib/real", root.join("policies/linked")).unwrap();
    let entry = root.join("policies/policy.ts");
    support::write(
        &entry,
        &format!(
            r#"import {{ which }} from "./linked/helper.ts";
export const policy = {{
  schemaVersion: 2,
  version: which,
  select: (request) => ({DEEP}),
}};
"#
        ),
    );
    entry
}

#[test]
fn node_resolver_and_channel_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly(
) {
    let sandbox = Sandbox::new();
    let linked = symlinked_helper_policy(&sandbox.root);
    // An ordinary module that reports to a parent process when it has one.
    let reporting = sandbox.root.join("reporting.ts");
    support::write(
        &reporting,
        &format!("if (process.send) process.send({{ via: \"ipc\" }});\n{ROUTED}"),
    );
    let caller = [
        ("NODE_PRESERVE_SYMLINKS", "1"),
        ("NODE_CHANNEL_FD", "3"),
        ("NODE_CHANNEL_SERIALIZATION_MODE", "json"),
    ];

    // Through the public launcher the caller's values never reach the
    // worker, and granting any of them refuses even where the caller lacks
    // it (`tests/environment.rs` grants each one set).
    for (entry, version) in [(&linked, "real"), (&reporting, "seam-1")] {
        let mut command = sandbox.command();
        command.envs(caller).args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--config",
            &text(entry),
            "--json",
        ]);
        assert_eq!(run(&mut command).report()["policy"]["version"], version);
    }
    for (name, _) in caller {
        let refusal = sandbox
            .start(&["--kind", "check", "--policy-env", name, "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], "excluded_grant", "{name}");
    }

    // The firing configurations: the shipped worker, started directly. With
    // NODE_PRESERVE_SYMLINKS the helper's `dep` resolves beside the link
    // rather than beside the real directory, with no import changed.
    let drive = |entry: &Path, set: &[(&'static str, &str)]| {
        let mut env = base_env(&sandbox);
        env.extend(set.iter().map(|&(name, value)| (name, value.into())));
        direct::drive(&shipped_worker(), &sandbox.root, &env, entry)
    };
    let version = |driven: direct::Driven| driven.loaded()["policy"]["version"].clone();
    assert_eq!(version(drive(&linked, &[])), "real");
    assert_eq!(
        version(drive(&linked, &[("NODE_PRESERVE_SYMLINKS", "1")])),
        "link",
        "NODE_PRESERVE_SYMLINKS never changed the resolution"
    );
    // With NODE_CHANNEL_FD naming descriptor 3, Bun adopts the private
    // protocol channel as its IPC channel, and the module's message reaches
    // it as Bun's JSON, where the policy's report belongs.
    drive(&reporting, &[]).loaded();
    let driven = drive(&reporting, &[("NODE_CHANNEL_FD", "3")]);
    assert_eq!(
        driven.report,
        Some(json!({ "type": "malformed", "header": b"{\"vi" })),
        "Bun never wrote to the channel\nstderr: {}",
        driven.stderr
    );
}

/// Every specifier the worker documents: its SDK, one per
/// shipped example.
fn documented_specifiers() -> Vec<String> {
    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("worker/examples");
    let mut specifiers = vec!["ravel/sdk".to_owned()];
    let mut stems: Vec<String> = fs::read_dir(&examples)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "ts"))
        .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    stems.sort();
    assert!(
        !stems.is_empty(),
        "no shipped examples in {}",
        examples.display()
    );
    specifiers.extend(stems.iter().map(|stem| format!("ravel/examples/{stem}")));
    specifiers
}

/// A file-name form of a specifier.
fn slug(specifier: &str) -> String {
    specifier.replace('/', "_")
}

/// How a package shadowing `ravel` beside an entry declares its
/// modules. The worker reads `package.json`, so each way a package can answer
/// a specifier is a way to shadow one.
#[derive(Clone, Copy, Debug)]
enum Shadow {
    /// `node_modules/ravel`, one file per subpath and no
    /// `package.json`.
    Files,
    /// `node_modules/ravel`, whose `exports` maps each subpath.
    Exports,
    /// The entry's own package: a `package.json` beside it named
    /// `ravel`, whose `exports` maps each subpath. A module's
    /// import of its own package's name resolves ahead of any `node_modules`.
    OwnName,
}

/// A `ravel` shadow in `dir`, laid out as `shadow` says, answering
/// each of `specifiers` with a module that writes its sentinel under `fired`
/// and exports only `shadowed`.
fn shadow_package(dir: &Path, shadow: Shadow, specifiers: &[String], fired: &Path) {
    let package = match shadow {
        Shadow::Files | Shadow::Exports => dir.join("node_modules/ravel"),
        Shadow::OwnName => dir.to_owned(),
    };
    let mut exports = serde_json::Map::new();
    for specifier in specifiers {
        let subpath = specifier.strip_prefix("ravel/").unwrap();
        let file = match shadow {
            Shadow::Files => format!("{subpath}.js"),
            Shadow::Exports | Shadow::OwnName => format!("shadow/{}.js", slug(specifier)),
        };
        support::write(
            &package.join(&file),
            &sentinel_module(&fired.join(slug(specifier))),
        );
        exports.insert(format!("./{subpath}"), json!(format!("./{file}")));
    }
    if !matches!(shadow, Shadow::Files) {
        support::write(
            &package.join("package.json"),
            &json!({ "name": "ravel", "type": "module", "exports": exports }).to_string(),
        );
    }
}

/// An entry importing each specifier and recording each module's export
/// names at `view`.
fn importing_entry(specifiers: &[String], view: &Path) -> String {
    let mut source = String::from("import { writeFileSync } from \"node:fs\";\n");
    for (index, specifier) in specifiers.iter().enumerate() {
        source.push_str(&format!("import * as m{index} from {specifier:?};\n"));
    }
    let fields: Vec<String> = specifiers
        .iter()
        .enumerate()
        .map(|(index, specifier)| format!("{specifier:?}: Object.keys(m{index}).sort()"))
        .collect();
    source.push_str(&format!(
        "writeFileSync({:?}, JSON.stringify({{ {} }}));\n{ROUTED}",
        text(view),
        fields.join(", ")
    ));
    source
}

#[test]
fn every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow() {
    let shipped = shipped_build();
    let documented = documented_specifiers();
    // An unregistered name under the prefix, which the shadow also answers:
    // the prefix itself reserves nothing, so it resolves to the shadow even
    // through the front, and shows the fixture is live beside this entry.
    let unregistered = "ravel/unregistered-name".to_owned();
    let mut imported = documented.clone();
    imported.push(unregistered.clone());

    for shadow in [Shadow::Files, Shadow::Exports, Shadow::OwnName] {
        let sandbox = Sandbox::new();
        let fired = sandbox.root.join("shadow-fired");
        fs::create_dir(&fired).unwrap();
        let view = sandbox.root.join("view.json");
        shadow_package(&sandbox.cwd.join("policies"), shadow, &imported, &fired);
        let entry = sandbox.file("policies/policy.ts", &importing_entry(&imported, &view));

        let _report = sandbox
            .start(&[
                "--kind",
                "check",
                "--config",
                "policies/policy.ts",
                "--json",
            ])
            .report();
        let seen = read_json(&view);
        for specifier in &documented {
            let keys = seen[specifier].as_array().unwrap();
            assert!(
                !keys.is_empty() && !keys.contains(&json!("shadowed")),
                "{shadow:?}: {specifier} did not resolve to its embedded module: {seen}"
            );
            assert!(
                !fired.join(slug(specifier)).exists(),
                "{shadow:?}: the shadow of {specifier} loaded through the front"
            );
        }
        assert_eq!(seen[&unregistered], json!(["shadowed"]), "{shadow:?}");
        assert!(fired.join(slug(&unregistered)).exists(), "{shadow:?}");
        fs::remove_file(fired.join(slug(&unregistered))).unwrap();

        // The firing configuration: the unregistered probe, with the same
        // entry.
        let driven = direct::drive(
            &probe_build(Probe::Unregistered),
            &sandbox.root,
            &base_env(&sandbox),
            &entry,
        );
        assert_probe_identity(&driven, Probe::Unregistered, &shipped);
        driven.loaded();
        let seen = read_json(&view);
        for specifier in &imported {
            assert_eq!(
                seen[specifier],
                json!(["shadowed"]),
                "{shadow:?}: {specifier}: {seen}"
            );
            assert!(
                fired.join(slug(specifier)).exists(),
                "{shadow:?}: the shadow of {specifier} never fired"
            );
        }
    }
}

#[test]
fn an_imports_alias_to_a_registered_specifier_is_a_package_lookup_and_never_the_embedded_module() {
    // The registration answers a specifier as an import writes it. A name an
    // `imports` map produces never comes back to it: the resolver looks a
    // package target up in `node_modules` itself (`load_package_imports`):
    // https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs
    // So an alias to a registered specifier is a limit of the shadow
    // control, stated in the spec, and this case holds both of its sides.
    let sandbox = Sandbox::new();
    let registered = "ravel/sdk".to_owned();
    let alias = "#sdk".to_owned();
    let fired = sandbox.root.join("shadow-fired");
    fs::create_dir(&fired).unwrap();
    let view = sandbox.root.join("view.json");
    sandbox.file(
        "policies/package.json",
        &json!({ "name": "owner-policies", "type": "module", "imports": { &alias: &registered } })
            .to_string(),
    );
    let start = || {
        sandbox.start(&[
            "--kind",
            "check",
            "--config",
            "policies/policy.ts",
            "--json",
        ])
    };

    // The registered name itself loads beside that `package.json`.
    sandbox.file(
        "policies/policy.ts",
        &importing_entry(std::slice::from_ref(&registered), &view),
    );
    start().report();
    let embedded = read_json(&view)[&registered].clone();
    assert!(
        !embedded.as_array().unwrap().is_empty() && embedded != json!(["shadowed"]),
        "{embedded}"
    );

    // With no package of the name, the alias is a missing package, and the
    // refusal names it.
    sandbox.file(
        "policies/policy.ts",
        &importing_entry(&[alias.clone(), registered.clone()], &view),
    );
    let refusal = start().refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains(&alias),
        "{refusal}"
    );

    // Beside a shadow, the alias is the shadow. The registered name, in the
    // same entry, is still the embedded module.
    shadow_package(
        &sandbox.cwd.join("policies"),
        Shadow::Files,
        std::slice::from_ref(&registered),
        &fired,
    );
    start().report();
    let seen = read_json(&view);
    assert_eq!(seen[&alias], json!(["shadowed"]), "{seen}");
    assert_eq!(seen[&registered], embedded, "{seen}");
    assert!(
        fired.join(slug(&registered)).exists(),
        "the shadow never loaded for the alias"
    );
}

/// A package in `dir` that a module there could reach three ways, each to a
/// module that writes `sentinel`: `#hostile-alias` through its `imports` map,
/// its own name `hostile-self` through its `exports`, and `hostile-dep`, a
/// package in a `node_modules` beside it whose entry its `main` declares.
fn hostile_package(dir: &Path, sentinel: &Path) -> [&'static str; 3] {
    support::write(
        &dir.join("package.json"),
        r##"{ "name": "hostile-self", "type": "module", "main": "./hostile.js", "imports": { "#hostile-alias": "./hostile.js" }, "exports": { ".": "./hostile.js" }, "dependencies": { "hostile-dep": "1.0.0" } }"##,
    );
    support::write(&dir.join("hostile.js"), &sentinel_module(sentinel));
    support::write(
        &dir.join("node_modules/hostile-dep/package.json"),
        r#"{ "name": "hostile-dep", "main": "./lib/entry.js" }"#,
    );
    support::write(
        &dir.join("node_modules/hostile-dep/lib/entry.js"),
        &sentinel_module(sentinel),
    );
    ["#hostile-alias", "hostile-self", "hostile-dep"]
}

/// A policy whose own file imports `specifier`.
fn importing_from_its_file(specifier: &str) -> String {
    format!("import {{ shadowed }} from {specifier:?};\nvoid shadowed;\n{ROUTED}")
}

/// Assert that a directly driven worker refused the entry at its import of
/// `specifier`; `context` says which case this is. The failure names the
/// specifier, so an entry that failed at an earlier line, before it imported
/// anything, is not taken for a refusal.
fn assert_import_refused(driven: &direct::Driven, specifier: &str, context: &str) {
    let report = driven.report.as_ref().unwrap_or_else(|| {
        panic!(
            "{context}: no report on the entry\nstderr: {}",
            driven.stderr
        )
    });
    assert_eq!(report["type"], "failure", "{context}: {report}");
    assert_eq!(report["stage"], "load", "{context}: {report}");
    assert!(
        report["message"]
            .as_str()
            .is_some_and(|message| message.contains(specifier)),
        "{context}: the failure does not name {specifier}: {report}"
    );
}

#[test]
fn a_cwd_package_json_stays_inert_and_fires_for_an_entry_admitted_there() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let fired = sandbox.root.join("cwd-package-ran");
    // The caller's cwd is the hostile directory.
    for specifier in hostile_package(&sandbox.cwd, &fired) {
        let source = importing_from_its_file(specifier);

        // Through the public launcher, personal policy finds nothing there.
        let entry = sandbox.personal_policy(&source);
        let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
        assert_eq!(
            refusal["error"]["code"], "policy_import_failed",
            "{specifier}: {refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(specifier),
            "{refusal}"
        );
        assert!(
            !fired.exists(),
            "the cwd package answered {specifier} through the front"
        );

        // Nor is it where the worker is that keeps it out. A module in a
        // file is resolved from that file, so the worker started in the
        // hostile directory finds nothing either: not the shipped one, which
        // leaves it, and not the unmoved probe, which stays.
        let driven = direct::drive(&shipped_worker(), &sandbox.cwd, &base_env(&sandbox), &entry);
        assert_import_refused(&driven, specifier, specifier);
        let driven = direct::drive(
            &probe_build(Probe::Unmoved),
            &sandbox.cwd,
            &base_env(&sandbox),
            &entry,
        );
        assert_probe_identity(&driven, Probe::Unmoved, &shipped);
        assert_import_refused(&driven, specifier, specifier);
        assert!(
            !fired.exists(),
            "the cwd package answered {specifier} for a worker started there"
        );

        // The firing configuration: the same import from an entry in that
        // directory, named explicitly.
        sandbox.file("admitted.ts", &source);
        let _report = sandbox
            .start(&["--kind", "check", "--config", "admitted.ts", "--json"])
            .report();
        assert!(
            fired.exists(),
            "the package never answered {specifier} for an entry beside it"
        );
        fs::remove_file(&fired).unwrap();
    }
}

/// A tsconfig and package.json in `dir` mapping the bare `hostile-alias` to a
/// module there that writes `sentinel`.
fn tsconfig_alias(dir: &Path, sentinel: &Path) {
    support::write(
        &dir.join("tsconfig.json"),
        r#"{ "compilerOptions": { "baseUrl": ".", "paths": { "hostile-alias": ["./hostile-alias.ts"] } } }"#,
    );
    support::write(
        &dir.join("package.json"),
        r#"{ "name": "owner-policies", "private": true, "type": "module" }"#,
    );
    support::write(&dir.join("hostile-alias.ts"), &sentinel_module(sentinel));
}

fn aliased_entry() -> String {
    format!("import {{ shadowed }} from \"hostile-alias\";\nvoid shadowed;\n{ROUTED}")
}

/// The same import, made once the policy has moved the worker into `dir`.
fn aliased_entry_moved_into(dir: &Path) -> String {
    format!(
        "process.chdir({:?});\nconst {{ shadowed }} = await import(\"hostile-alias\");\nvoid shadowed;\n{ROUTED}",
        text(dir)
    )
}

#[test]
fn tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let aliased = sandbox.root.join("alias-ran");
    tsconfig_alias(&sandbox.cwd.join("policies"), &aliased);
    let entry = sandbox.file("policies/aliased.ts", &aliased_entry());

    // Through the front the alias does not apply, so the import is missing.
    let refusal = sandbox
        .start(&[
            "--kind",
            "check",
            "--config",
            "policies/aliased.ts",
            "--json",
        ])
        .refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("hostile-alias"),
        "{refusal}"
    );
    assert!(
        !aliased.exists(),
        "a tsconfig alias loaded through the front"
    );

    // The firing configuration: the tsconfig probe, with the same entry.
    let driven = direct::drive(
        &probe_build(Probe::Tsconfig),
        &sandbox.root,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Tsconfig, &shipped);
    driven.loaded();
    assert!(
        aliased.exists(),
        "the tsconfig probe never applied the alias"
    );
}

/// A package `chain-planted`, in `dir/node_modules`, whose module writes
/// `sentinel` when it loads.
fn planted_package(dir: &Path, sentinel: &Path) {
    support::write(
        &dir.join("node_modules/chain-planted/index.js"),
        &sentinel_module(sentinel),
    );
}

/// One policy per kind of module with no file location, each of which
/// imports `specifier` from such a module: one imported from a `data:` URL,
/// one from a `blob:` URL, and a virtual module the policy registers.
fn policies_importing_from_no_file(specifier: &str) -> [(&'static str, String); 3] {
    let module = format!("import {specifier:?};");
    [
        (
            "data:",
            format!("await import(\"data:text/javascript,\" + encodeURIComponent({module:?}));\n{ROUTED}"),
        ),
        (
            "blob:",
            format!(
                "await import(URL.createObjectURL(new Blob([{module:?}], {{ type: \"text/javascript\" }})));\n{ROUTED}"
            ),
        ),
        (
            "registered virtual module",
            format!(
                r#"Bun.plugin({{
  name: "policy virtual module",
  setup(build) {{
    build.module("policy-virtual", () => ({{ contents: {module:?}, loader: "js" }}));
  }},
}});
await import("policy-virtual");
{ROUTED}"#
            ),
        ),
    ]
}

#[test]
fn a_package_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let loaded = sandbox.root.join("chain-planted-loaded");
    // The front creates the worker's directory in the caller's TMPDIR, so
    // this package sits one level above where the worker starts.
    planted_package(&sandbox.tmp, &loaded);
    // The firing configuration starts its worker beside where the front
    // would: in a directory of its own under the same TMPDIR.
    let started = sandbox.tmp.join("started-here");
    fs::create_dir(&started).unwrap();

    for (kind, policy) in policies_importing_from_no_file("chain-planted") {
        let entry = sandbox.personal_policy(&policy);

        // Through the public launcher the name is missing, through the
        // commands: no directory between where the worker started and `/`
        // answers it. `/` itself still would, and can hold no fixture.
        let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
        assert_eq!(
            refusal["error"]["code"], "policy_import_failed",
            "{kind}: {refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains("chain-planted"),
            "{kind}: {refusal}"
        );
        let ran = sandbox.run(&["--kind", "check", "--prompt", "p"]);
        assert_eq!(ran.code, Some(3), "{kind}: {}", ran.stderr);
        assert!(
            !loaded.exists(),
            "a package above the worker's directory answered a {kind} import through the front"
        );

        // The firing configuration: the unmoved probe, started under the
        // same TMPDIR, loads the package for the same entry.
        let driven = direct::drive(
            &probe_build(Probe::Unmoved),
            &started,
            &base_env(&sandbox),
            &entry,
        );
        assert_probe_identity(&driven, Probe::Unmoved, &shipped);
        driven.loaded();
        assert!(
            loaded.exists(),
            "the unmoved probe never loaded the package for a {kind} import"
        );
        fs::remove_file(&loaded).unwrap();

        // The move alone is the difference: the shipped worker, started in
        // that same directory, finds nothing.
        let driven = direct::drive(&shipped_worker(), &started, &base_env(&sandbox), &entry);
        assert_import_refused(&driven, "chain-planted", kind);
        assert!(
            !loaded.exists(),
            "the shipped worker loaded the package for a {kind} import"
        );
    }
}

#[test]
fn a_package_json_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe()
{
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let fired = sandbox.root.join("chain-package-ran");
    // The front creates the worker's directory in the caller's TMPDIR, so
    // this package sits one level above where the worker starts.
    let specifiers = hostile_package(&sandbox.tmp, &fired);
    // Each directly driven worker starts beside where the front would start
    // it: in a directory of its own under the same TMPDIR.
    let started = sandbox.tmp.join("started-here");
    fs::create_dir(&started).unwrap();
    let unmoved = |entry: &Path| {
        let driven = direct::drive(
            &probe_build(Probe::Unmoved),
            &started,
            &base_env(&sandbox),
            entry,
        );
        assert_probe_identity(&driven, Probe::Unmoved, &shipped);
        driven
    };

    for specifier in specifiers {
        // Each way a policy can import the name: from its own file, and from
        // each kind of module with no file location.
        let mut policies = vec![("policy file", importing_from_its_file(specifier), false)];
        policies.extend(
            policies_importing_from_no_file(specifier)
                .into_iter()
                .map(|(kind, policy)| (kind, policy, true)),
        );
        for (kind, policy, has_no_file) in policies {
            let context = format!("{specifier} from a {kind}");
            let entry = sandbox.personal_policy(&policy);

            // Through the public launcher the name is missing, through the
            // commands: no `package.json` between where the worker started
            // and `/` answers it. `/package.json` still would answer a module
            // with no file location, and `/` can hold no fixture.
            let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
            assert_eq!(
                refusal["error"]["code"], "policy_import_failed",
                "{context}: {refusal}"
            );
            assert!(
                refusal["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains(specifier),
                "{context}: {refusal}"
            );
            let ran = sandbox.run(&["--kind", "check", "--prompt", "p"]);
            assert_eq!(ran.code, Some(3), "{context}: {}", ran.stderr);
            assert!(
                !fired.exists(),
                "the package above the worker's directory answered {context} through the front"
            );

            let driven = unmoved(&entry);
            if has_no_file {
                // The firing configuration: the unmoved probe, started under
                // the same TMPDIR, resolves such a module from where it
                // stayed, and the package answers.
                driven.loaded();
                assert!(
                    fired.exists(),
                    "the unmoved probe never loaded the package for {context}"
                );
                fs::remove_file(&fired).unwrap();
            } else {
                // A module in a file is resolved from that file in every
                // build, so the package has no firing configuration for it.
                // The same fixture fires above for the other three.
                assert_import_refused(&driven, specifier, &context);
                assert!(
                    !fired.exists(),
                    "the package answered {context} under the unmoved probe"
                );
            }

            // The move alone is the difference: the shipped worker, started
            // in that same directory, finds nothing.
            let driven = direct::drive(&shipped_worker(), &started, &base_env(&sandbox), &entry);
            assert_import_refused(&driven, specifier, &context);
            assert!(
                !fired.exists(),
                "the shipped worker loaded the package for {context}"
            );
        }
    }
}

/// How long a worker gets to report on an entry beside a `package.json` that
/// never yields, in milliseconds: the bound the front is given, and the
/// patience each directly driven worker gets. A worker that opens nothing
/// answers in a hundredth of it.
const STALL_MS: u64 = 5_000;

/// Run `drive`, and say whether any process opened `fifo` for reading
/// meanwhile.
///
/// A reader of a FIFO waits in its open until a writer opens it too, and an
/// open for writing that does not block fails with ENXIO until a reader is
/// there. So that open succeeding is the read, seen, and nothing else in a
/// sandbox opens the file. The write end is then held, with nothing written,
/// until `drive` returns: the reader gets no content and no end of file, so
/// it goes on waiting. Closing the end at once instead would let a reader
/// through on Linux and leave one waiting on macOS, as each was seen to.
fn watching_for_a_reader<T>(fifo: &Path, drive: impl FnOnce() -> T) -> (T, bool) {
    // Dropped when `drive` returns or panics, which is what stops the watch.
    let (running, stopped) = mpsc::channel::<()>();
    thread::scope(|scope| {
        let watch = scope.spawn(move || {
            let mut write_end = None;
            while stopped.recv_timeout(Duration::from_millis(2))
                == Err(mpsc::RecvTimeoutError::Timeout)
            {
                if write_end.is_some() {
                    continue;
                }
                match fs::OpenOptions::new()
                    .write(true)
                    .custom_flags(libc::O_NONBLOCK)
                    .open(fifo)
                {
                    Ok(end) => write_end = Some(end),
                    Err(error) if error.raw_os_error() == Some(libc::ENXIO) => {}
                    Err(error) => panic!("cannot watch {}: {error}", fifo.display()),
                }
            }
            write_end.is_some()
        });
        let result = drive();
        drop(running);
        (result, watch.join().expect("the watch ends"))
    })
}

#[test]
fn a_package_json_that_never_yields_above_the_workers_start_directory_stays_unopened_and_stalls_the_unmoved_probe(
) {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    // A policy that imports nothing: whatever a worker does with this
    // `package.json`, no import asked it to.
    let entry = sandbox.personal_policy(ROUTED);
    // The front creates the worker's directory in the caller's TMPDIR, so
    // this `package.json` sits one level above where the worker starts. Each
    // directly driven worker starts beside where the front would start it:
    // in a directory of its own under the same TMPDIR.
    let manifest = sandbox.tmp.join("package.json");
    let started = sandbox.tmp.join("started-here");
    fs::create_dir(&started).unwrap();
    let bound = STALL_MS.to_string();
    let patience = Duration::from_millis(STALL_MS);
    let directly = |worker: &Path| {
        direct::drive_within(worker, &started, &base_env(&sandbox), &entry, patience)
    };
    let unmoved = || {
        let driven = directly(&probe_build(Probe::Unmoved));
        assert_probe_identity(&driven, Probe::Unmoved, &shipped);
        driven
    };

    // The baseline: beside a `package.json` it can read, the unmoved probe
    // loads this entry from this directory. So what it does below, it does
    // on account of the file.
    support::write(&manifest, r#"{ "name": "readable" }"#);
    let driven = unmoved();
    assert!(!driven.stalled, "the unmoved probe stalled at its baseline");
    driven.loaded();
    fs::remove_file(&manifest).unwrap();

    // The fixture: a `package.json` that is a link to a FIFO. Whoever opens
    // it waits, and is seen. It must be a link: the runtime passes over a
    // FIFO of that name unopened, and opens a link to find what it is.
    let fifo = sandbox.root.join("never-yields");
    support::mkfifo(&fifo);
    std::os::unix::fs::symlink(&fifo, &manifest).unwrap();

    // Through the public launcher the selection is made under the public command,
    // well inside a bound the firing configuration below outlasts, and
    // nothing opens the file: importing an entry reads no `package.json`
    // between where the worker started and `/`.
    let ((report, ran), opened) = watching_for_a_reader(&fifo, || {
        (
            sandbox
                .start(&["--kind", "check", "--timeout-ms", &bound, "--json"])
                .report(),
            sandbox.run(&["--kind", "check", "--prompt", "p", "--timeout-ms", &bound]),
        )
    });
    assert_eq!(report["selection"]["reason"], ROUTED_REASON);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert!(
        !opened,
        "the package.json above the worker's directory was opened through the front"
    );

    // The firing configuration: the unmoved probe, started under the same
    // TMPDIR, opens the file, because importing the entry records every
    // directory from the worker's to the root. Then it never reports, and is
    // killed once its time is up. Both are asserted, since a probe that died
    // for some other reason would be silent too: it would have opened
    // nothing, and would not be running when the time ran out.
    let (driven, opened) = watching_for_a_reader(&fifo, unmoved);
    assert!(
        opened,
        "the unmoved probe never opened the package.json above its directory: {:?}\nstderr: {}",
        driven.report, driven.stderr
    );
    assert!(
        driven.stalled,
        "the unmoved probe was not waiting on the package.json it opened: {:?}\nstderr: {}",
        driven.report, driven.stderr
    );

    // The move alone is the difference, and the bound is not what stopped
    // the probe: the shipped worker, started in that same directory with the
    // same patience, opens nothing and loads the policy.
    let (driven, opened) = watching_for_a_reader(&fifo, || directly(&shipped_worker()));
    assert!(
        !opened,
        "the shipped worker opened the package.json above its directory"
    );
    assert!(!driven.stalled, "the shipped worker stalled as well");
    driven.loaded();
}

/// Fail loudly if a class reported as having no firing configuration has
/// gained one: it would then be a class to prove and count, not to report.
fn still_unfired(class: &str, fired: bool) {
    assert!(
        !fired,
        "the {class} class fired under its most permissive probe build: it now has a firing \
         configuration, so prove it inert through the front, count it, and update the spec's \
         firing-configuration table and the runtime evidence"
    );
}

#[test]
fn classes_with_no_known_firing_configuration_are_reported_not_counted() {
    // These are tripwires, not controls. No class here has been seen to fire
    // in any build, so an inert result here proves nothing about the front.
    // What the test does show is that the pinned Bun still gives them no
    // firing configuration; if one appears, it fails and says so.
    let sandbox = Sandbox::new();
    let shipped = shipped_build();

    // A HOME bunfig preload. HOME is in the worker's base environment.
    let preloaded = sandbox.root.join("home-bunfig-ran");
    support::write(
        &sandbox.home.join("hostile-preload.ts"),
        &sentinel_module(&preloaded),
    );
    support::write(
        &sandbox.home.join(".bunfig.toml"),
        &format!(
            "preload = [{:?}]\n",
            text(&sandbox.home.join("hostile-preload.ts"))
        ),
    );
    let entry = sandbox.personal_policy(ROUTED);
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert!(
        !preloaded.exists(),
        "a HOME bunfig preload ran through the front"
    );
    let empty = sandbox.root.join("empty");
    fs::create_dir(&empty).unwrap();
    let driven = direct::drive(
        &probe_build(Probe::Autoload),
        &empty,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Autoload, &shipped);
    driven.loaded();
    still_unfired("HOME bunfig", preloaded.exists());

    // A `.env`, a `.env.local` and a bunfig preload in a directory the worker
    // moves into after it has started. The shipped worker moves to `/`, where
    // no fixture can be planted, so the policy makes the move, into a hostile
    // directory, under the probe that autoloads. Two things are watched. The
    // VM that makes the move loaded its environment when the process started,
    // and must not load it again. A bunfig is read once, at process start, so
    // its preload must run neither in that VM nor in a native `Worker` the
    // policy then starts there. The dotenv files do load in that `Worker`,
    // which is a class with a firing configuration, counted above.
    let moved_into = sandbox.root.join("moved-into");
    let view = sandbox.root.join("view.json");
    support::write(&moved_into.join(".env"), "HOSTILE_DOTENV=from-dotenv\n");
    support::write(
        &moved_into.join(".env.local"),
        "HOSTILE_DOTENV_LOCAL=from-dotenv-local\n",
    );
    support::write(
        &moved_into.join("bunfig.toml"),
        "preload = [\"./hostile-preload.ts\"]\n",
    );
    let moved_preloaded = sandbox.root.join("moved-into-preload-ran");
    support::write(
        &moved_into.join("hostile-preload.ts"),
        &sentinel_module(&moved_preloaded),
    );
    let module = sandbox.root.join("reporting-worker.ts");
    support::write(&module, DOTENV_REPORTING_WORKER);
    let entry =
        sandbox.personal_policy(&dotenv_view_policy_moved_into(&moved_into, &module, &view));
    let inert = json!({ "dotenv": null, "dotenvLocal": null });
    sandbox.start(&["--kind", "check", "--json"]).report();
    assert_eq!(read_json(&view)["moved"], inert);
    assert!(
        !moved_preloaded.exists(),
        "a bunfig preload in a directory the worker moved into ran through the front"
    );
    direct::drive(
        &probe_build(Probe::Autoload),
        &empty,
        &base_env(&sandbox),
        &entry,
    )
    .loaded();
    still_unfired(
        "dotenv in a directory moved into, for the VM that made the move",
        read_json(&view)["moved"] != inert,
    );
    still_unfired(
        "bunfig in a directory moved into, for the VM that made the move and a `Worker` \
         started there",
        moved_preloaded.exists(),
    );

    // A tsconfig alias in the caller's cwd, for an entry elsewhere. Every
    // worker but the unmoved probe leaves the directory it starts in, so the
    // entry moves the worker back there before it imports the alias.
    let aliased = sandbox.root.join("cwd-alias-ran");
    tsconfig_alias(&sandbox.cwd, &aliased);
    let entry = sandbox.personal_policy(&aliased_entry_moved_into(&sandbox.cwd));
    let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert!(
        !aliased.exists(),
        "a cwd tsconfig alias loaded through the front"
    );
    let driven = direct::drive(
        &probe_build(Probe::Tsconfig),
        &sandbox.cwd,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Tsconfig, &shipped);
    // The other arms see the probe load its policy. This one expects the
    // import refused, so the refusal is what shows the probe moved and
    // reached the import: one that failed earlier would leave the alias
    // unloaded too.
    assert_import_refused(&driven, "hostile-alias", "cwd tsconfig");
    still_unfired("cwd tsconfig", aliased.exists());
}

#[test]
fn a_probe_build_is_never_accepted_as_an_installations_worker() {
    // So no archive can ship one as its worker: the installed smoke test
    // starts a fixture through each archive's front before a release publishes.
    //
    // A probe is a real worker, and would evaluate an entry it was handed.
    // The policy marks that it was evaluated, so the mark's absence shows
    // that no policy code ran under a probe, which a front that refused one
    // only once it had answered would allow. Whether a refused worker is
    // No marker means policy code did not run before the identity refusal.
    let sandbox = Sandbox::new();
    let evaluated = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"evaluated\");\n{ROUTED}",
        text(&evaluated)
    ));
    let prefix = sandbox.root.join("prefix");
    let front = prefix.join("bin/ravel");
    fs::create_dir_all(front.parent().unwrap()).unwrap();
    fs::copy(FRONT, &front).unwrap();
    let layout: PathBuf = prefix.join("libexec/ravel/ravel-policy");
    fs::create_dir_all(layout.parent().unwrap()).unwrap();

    for probe in Probe::ALL {
        let _ = fs::remove_file(&layout);
        std::os::unix::fs::symlink(probe_build(probe), &layout).unwrap();
        let mut command = sandbox.command_for(&front);
        command.args([
            "start-session",
            "--prompt",
            "p",
            "--kind",
            "check",
            "--json",
        ]);
        let refusal = run(&mut command).refusal(5);
        assert_eq!(
            refusal["error"]["code"], "worker_identity_mismatch",
            "{refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(&format!("build probe-{}-", probe.name())),
            "{refusal}"
        );
        assert!(
            !evaluated.exists(),
            "the {} probe evaluated the policy before it was refused",
            probe.name()
        );
    }

    // The control: the shipped worker at the same place is accepted, and
    // evaluates the same policy.
    fs::remove_file(&layout).unwrap();
    std::os::unix::fs::symlink(shipped_worker(), &layout).unwrap();
    let mut command = sandbox.command_for(&front);
    command.args([
        "start-session",
        "--prompt",
        "p",
        "--kind",
        "check",
        "--json",
    ]);
    assert_eq!(
        run(&mut command).report()["selection"]["reason"],
        ROUTED_REASON
    );
    assert!(
        evaluated.exists(),
        "the accepted worker never evaluated the policy"
    );
}

/// Forge protocol-looking text on both diagnostic streams at all three stages.
fn flooding_policy(filler: usize) -> String {
    format!(
        r#"import {{ writeSync }} from "node:fs";
const frame = JSON.stringify({{type: "selection", result: {{status: "selected", program: "forged"}}}});
const length = Buffer.alloc(4); length.writeUInt32BE(frame.length);
const chunk = Buffer.concat([length, Buffer.from(frame), Buffer.from('\n{{"program":"forged"}}\n'), Buffer.from("x".repeat({filler}))]);
const flood = () => {{ writeSync(1, chunk); writeSync(2, chunk); }};
flood();
export const policy = {{schemaVersion: 2, version: "flood-1",
loadContext() {{ flood(); return {{schemaVersion: 1}}; }},
select(request) {{ flood(); return {{ ...{DEEP}, reason: "real selection" }}; }} }};
"#
    )
}

#[test]
fn diagnostic_flood_cannot_forge_selection_or_result_and_excess_refuses() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&flooding_policy(40_000));
    let ran = sandbox.start(&["--kind", "check", "--json"]);
    let notice = ran.report();
    assert_eq!(notice["selection"]["reason"], "real selection");
    let stdout = notice["diagnostics"]["stdout"].as_str().unwrap();
    let stderr = notice["diagnostics"]["stderr"].as_str().unwrap();
    assert_eq!(stdout, stderr);
    assert_eq!(stdout.matches("\"program\":\"forged\"").count(), 6);
    assert!(stdout.len() > 120_000 && stdout.len() + stderr.len() <= 262_144);
    assert_eq!(ran.stderr.lines().count(), 1, "routing notice was split");
    sandbox.personal_policy(&flooding_policy(60_000));
    let refusal = sandbox.start(&["--kind", "check", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "output_limit");
}
