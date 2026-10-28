use std::ffi::{OsStr, OsString};

use serde_json::{Value, json};

use crate::refusal::{EXIT_MALFORMED, Refusal, Stage};

/// The names the owner granted, each once, in the order first given.
#[derive(Clone, Debug, Default)]
pub struct Grants {
    grants: Vec<Grant>,
}

#[derive(Clone, Debug)]
struct Grant {
    name: String,
    /// Whether the caller's environment has it; unset, the worker lacks it.
    set: bool,
}

impl Grants {
    /// The owner settings' names, already checked, then each `--policy-env`
    /// name, refusing an excluded or malformed one.
    pub fn read(settings: &[String], flags: &[OsString]) -> Result<Grants, Refusal> {
        let mut grants: Vec<Grant> = Vec::new();
        let flags = flags.iter().map(|name| checked(name));
        for name in settings.iter().cloned().map(Ok).chain(flags) {
            let name = name?;
            if grants.iter().all(|grant| grant.name != name) {
                grants.push(Grant {
                    set: std::env::var_os(&name).is_some(),
                    name,
                });
            }
        }
        Ok(Grants { grants })
    }

    /// What `ravel::env` finds for `name` in the caller's environment.
    pub fn lookup(&self, name: &str) -> Lookup {
        if !self.readable(name) {
            return Lookup::NotReadable;
        }
        match std::env::var_os(name) {
            Some(value) => Lookup::Set(value),
            None => Lookup::Unset,
        }
    }

    /// Whether the policy may read `name`. This is the one place that decides.
    fn readable(&self, name: &str) -> bool {
        is_base(name.as_bytes()) || self.grants.iter().any(|grant| grant.name == name)
    }

    /// `[{ name, set }]`, never a value.
    pub fn to_json(&self) -> Value {
        self.grants
            .iter()
            .map(|grant| json!({ "name": grant.name, "set": grant.set }))
            .collect()
    }
}

/// What `ravel::env` finds for a name.
#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    Set(OsString),
    Unset,
    /// A name that neither the base set nor a grant names.
    NotReadable,
}

/// The marker that a harness run carries, and that `start-session` refuses
/// (`docs/specs/policy-evaluation.md`, *Re-entry and completion authority*).
pub const POLICY_RUN: &str = "RAVEL_POLICY_RUN";

/// The variables that the managed process and each harness run never inherit:
/// the launch controls of an enclosing session or driver. They would let the
/// child end somebody else's session. The marker is no part of the list.
/// `start-session` refuses to run when it is set, so the managed process cannot
/// inherit it.
pub const LAUNCH_CONTROLS: [&str; 6] = [
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "RAVEL_RUN_ID",
    "RAVEL_STATE_DIR",
];

/// Whether the policy gets `name` without a grant: `HOME`, `PATH`, `TMPDIR`,
/// `LANG` and `LC_*`.
pub fn is_base(name: &[u8]) -> bool {
    matches!(name, b"HOME" | b"PATH" | b"TMPDIR" | b"LANG") || name.starts_with(b"LC_")
}

/// Why `name` is never granted, if it is not.
fn excluded(name: &str) -> Option<&'static str> {
    if name.starts_with("LD_") || name.starts_with("DYLD_") {
        Some("the dynamic loaders read LD_* and DYLD_* variables, which can inject libraries")
    } else if name.starts_with("RAVEL_") || name.starts_with("HARNESS_DISPATCH_") {
        Some(
            "RAVEL_* variables are ravel's own, set for the harness it runs, \
             and the worker is never told the session completion channel",
        )
    } else {
        None
    }
}

/// What named a grant: a `--policy-env` flag, or `policyEnv` in the owner
/// settings file. One rule holds both, and a refusal says which it was.
pub enum Named<'a> {
    Flag,
    Setting { file: &'a str },
}

impl Named<'_> {
    /// How a message begins: the flag, or the key and its file.
    fn label(&self) -> String {
        match self {
            Named::Flag => "--policy-env".to_owned(),
            Named::Setting { file } => format!("policyEnv in the owner settings file {file}"),
        }
    }

    fn name(&self, refusal: Refusal) -> Refusal {
        match self {
            Named::Flag => refusal.input("--policy-env"),
            Named::Setting { file } => refusal.source(*file).location("policyEnv"),
        }
    }
}

/// Refuse a grant that is empty, holds `=` or is excluded.
pub fn grantable(name: &str, named: &Named) -> Result<(), Refusal> {
    let label = named.label();
    let malformed = |message: String| {
        named.name(Refusal::new(
            "malformed_input",
            Stage::Cli,
            EXIT_MALFORMED,
            message,
            "name one environment variable per grant, such as --policy-env ANTHROPIC_API_KEY; \
             its value comes from ravel's own environment and is never shown",
        ))
    };
    if name.is_empty() {
        return Err(malformed(format!("{label} must not be empty")));
    }
    if name.contains('=') {
        return Err(malformed(format!(
            "{label} {name} contains `=`: it takes a variable's name, never a value"
        )));
    }
    if let Some(why) = excluded(name) {
        return Err(named.name(Refusal::new(
            "excluded_grant",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("{label} {name} is never granted to the policy worker: {why}"),
            "remove it; the worker's environment is HOME, PATH, TMPDIR, LANG and LC_* plus the \
             names you grant; LD_*, DYLD_*, RAVEL_* and HARNESS_DISPATCH_* are excluded from grants",
        )));
    }
    Ok(())
}

/// A `--policy-env` name, which must also be UTF-8.
fn checked(name: &OsStr) -> Result<String, Refusal> {
    let name = name.to_str().ok_or_else(|| {
        Refusal::new(
            "malformed_input",
            Stage::Cli,
            EXIT_MALFORMED,
            "a --policy-env name is not valid UTF-8",
            "name one environment variable per --policy-env, such as --policy-env \
             ANTHROPIC_API_KEY",
        )
        .input("--policy-env")
    })?;
    grantable(name, &Named::Flag)?;
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusion_is_by_class_and_leaves_its_near_misses_grantable() {
        for name in [
            "LD_PRELOAD",
            "LD_LIBRARY_PATH",
            "LD_AUDIT",
            "DYLD_INSERT_LIBRARIES",
            "DYLD_LIBRARY_PATH",
            "RAVEL_STATE_DIR",
            "RAVEL_RUN_ID",
        ] {
            assert!(excluded(name).is_some(), "{name} is grantable");
        }
        for name in [
            "BUN_OPTIONS",
            "BUN_BE_BUN",
            "BUN_INSTALL",
            "NODE_OPTIONS",
            "NODE_PATH",
            "NODE_PRESERVE_SYMLINKS",
            "NODE_CHANNEL_FD",
            "NODE_CHANNEL_SERIALIZATION_MODE",
            "BUN",
            "BUNDLE_GEMFILE",
            "NODE_EXTRA_CA_CERTS",
            "NODE_PRESERVE_SYMLINKS_MAIN",
            "NODE_CHANNEL",
            "NODE_UNIQUE_ID",
            "LDFLAGS",
            "RAVELER",
            "bun_options",
            "CUSTOM_SIGNAL_FILE",
        ] {
            assert_eq!(excluded(name), None, "{name} is excluded");
        }
    }

    #[test]
    fn a_grant_is_one_name_and_duplicates_are_one_grant() {
        let flags = ["A_TOKEN".into(), "OTHER".into(), "A_TOKEN".into()];
        let grants = Grants::read(&[], &flags).unwrap();
        let names: Vec<&str> = grants
            .grants
            .iter()
            .map(|grant| grant.name.as_str())
            .collect();
        assert_eq!(names, ["A_TOKEN", "OTHER"]);
        for (name, code) in [
            ("", "malformed_input"),
            ("A=B", "malformed_input"),
            ("LD_PRELOAD", "excluded_grant"),
        ] {
            let refusal = Grants::read(&[], &[name.into()]).unwrap_err();
            assert_eq!(refusal.code, code, "{name}");
            assert_eq!(refusal.input.as_deref(), Some("--policy-env"), "{name}");
        }
    }

    #[test]
    fn a_name_is_readable_when_it_is_in_the_base_set_or_granted() {
        // Cargo sets this for every test run, and the test does not change it.
        let granted = Grants::read(&["CARGO_MANIFEST_DIR".to_owned()], &[]).unwrap();
        let none = Grants::default();
        assert!(matches!(none.lookup("HOME"), Lookup::Set(_)));
        assert!(matches!(none.lookup("PATH"), Lookup::Set(_)));
        assert_eq!(none.lookup("LC_NO_SUCH_CATEGORY"), Lookup::Unset);
        assert_eq!(none.lookup("RAVEL_NO_SUCH_NAME"), Lookup::NotReadable);
        assert_eq!(none.lookup("CARGO_MANIFEST_DIR"), Lookup::NotReadable);
        assert!(matches!(
            granted.lookup("CARGO_MANIFEST_DIR"),
            Lookup::Set(_)
        ));
        assert_eq!(granted.lookup("RAVEL_NO_SUCH_NAME"), Lookup::NotReadable);
        for name in ["", "HOM", "home", "LC", "HOME=", "HOME\0"] {
            assert_eq!(none.lookup(name), Lookup::NotReadable, "{name:?}");
        }
    }

    #[test]
    fn a_flag_adds_to_the_names_the_owner_settings_grant() {
        let settings = ["SETTING".to_owned(), "BOTH".to_owned()];
        let flags = ["BOTH".into(), "FLAG".into()];
        let grants = Grants::read(&settings, &flags).unwrap();
        let names: Vec<&str> = grants
            .grants
            .iter()
            .map(|grant| grant.name.as_str())
            .collect();
        assert_eq!(names, ["SETTING", "BOTH", "FLAG"]);
    }
}
