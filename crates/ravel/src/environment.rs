use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt as _;

use serde_json::{json, Value};

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

/// The names the owner granted, each once, in the order first given.
#[derive(Debug, Default)]
pub struct Grants(Vec<Grant>);

#[derive(Debug)]
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
        Ok(Grants(grants))
    }

    /// The worker's whole environment: the base set and the granted names,
    /// with the caller's values, and the front's own [`HOST`] settings.
    pub fn worker_environment(&self) -> Vec<(OsString, OsString)> {
        std::env::vars_os()
            .filter(|(name, _)| {
                base(name.as_bytes())
                    || self
                        .0
                        .iter()
                        .any(|grant| grant.name.as_bytes() == name.as_bytes())
            })
            .chain(
                HOST.iter()
                    .map(|(name, value)| (OsString::from(name), OsString::from(value))),
            )
            .collect()
    }

    /// `[{ name, set }]`, never a value.
    pub fn to_json(&self) -> Value {
        self.0
            .iter()
            .map(|grant| json!({ "name": grant.name, "set": grant.set }))
            .collect()
    }
}

/// The values the front itself sets in the worker's environment, each under an
/// excluded name, so no caller value or grant can replace it.
/// `BUN_RUNTIME_TRANSPILER_CACHE_PATH` of `0` turns Bun's runtime transpiler
/// cache off (bun-v1.4.2 `src/jsc/RuntimeTranspilerCache.rs`,
/// `really_get_cache_dir`).
pub const HOST: [(&str, &str); 1] = [("BUN_RUNTIME_TRANSPILER_CACHE_PATH", "0")];

/// Whether the worker gets `name` without a grant.
fn base(name: &[u8]) -> bool {
    matches!(name, b"HOME" | b"PATH" | b"TMPDIR" | b"LANG") || name.starts_with(b"LC_")
}

/// Why `name` is never granted, if it is not.
fn excluded(name: &str) -> Option<&'static str> {
    if name.starts_with("BUN_") {
        Some(
            "Bun reads its runtime options from BUN_* variables, and BUN_OPTIONS can preload \
             code and BUN_BE_BUN turn the worker into Bun itself",
        )
    } else if matches!(
        name,
        "NODE_OPTIONS" | "NODE_PATH" | "NODE_PRESERVE_SYMLINKS"
    ) {
        Some(
            "NODE_OPTIONS, NODE_PATH and NODE_PRESERVE_SYMLINKS change what code loads, and from \
             where",
        )
    } else if name.starts_with("NODE_CHANNEL_") {
        Some(
            "NODE_CHANNEL_* variables name a descriptor that Bun adopts as its IPC channel, which \
             could make the policy worker's private protocol channel its own",
        )
    } else if name.starts_with("LD_") || name.starts_with("DYLD_") {
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
             names you grant, and BUN_*, NODE_OPTIONS, NODE_PATH, NODE_PRESERVE_SYMLINKS, \
             NODE_CHANNEL_*, LD_*, DYLD_* and RAVEL_* are excluded from grants",
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
            "BUN_OPTIONS",
            "BUN_BE_BUN",
            "BUN_INSTALL",
            "NODE_OPTIONS",
            "NODE_PATH",
            "NODE_PRESERVE_SYMLINKS",
            "NODE_CHANNEL_FD",
            "NODE_CHANNEL_SERIALIZATION_MODE",
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
            "BUN",
            "BUNDLE_GEMFILE",
            "NODE_EXTRA_CA_CERTS",
            "NODE_PRESERVE_SYMLINKS_MAIN",
            "NODE_CHANNEL",
            "NODE_UNIQUE_ID",
            "LDFLAGS",
            "RAVELER",
            "bun_options",
            "GROVE_SIGNAL_FILE",
        ] {
            assert_eq!(excluded(name), None, "{name} is excluded");
        }
    }

    #[test]
    fn a_grant_is_one_name_and_duplicates_are_one_grant() {
        let flags = ["A_TOKEN".into(), "OTHER".into(), "A_TOKEN".into()];
        let grants = Grants::read(&[], &flags).unwrap();
        let names: Vec<&str> = grants.0.iter().map(|grant| grant.name.as_str()).collect();
        assert_eq!(names, ["A_TOKEN", "OTHER"]);
        for (name, code) in [
            ("", "malformed_input"),
            ("A=B", "malformed_input"),
            ("NODE_OPTIONS", "excluded_grant"),
        ] {
            let refusal = Grants::read(&[], &[name.into()]).unwrap_err();
            assert_eq!(refusal.code, code, "{name}");
            assert_eq!(refusal.input.as_deref(), Some("--policy-env"), "{name}");
        }
    }

    #[test]
    fn a_flag_adds_to_the_names_the_owner_settings_grant() {
        let settings = ["SETTING".to_owned(), "BOTH".to_owned()];
        let grants = Grants::read(&settings, &["BOTH".into(), "FLAG".into()]).unwrap();
        let names: Vec<&str> = grants.0.iter().map(|grant| grant.name.as_str()).collect();
        assert_eq!(names, ["SETTING", "BOTH", "FLAG"]);
    }
}
