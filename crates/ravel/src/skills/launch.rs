//! Launch preparation: `start-session` resolves the configured sources and
//! wraps the caller's task in the prepared prompt before confinement and
//! before the worker (docs/specs/skill-discovery.md, "Ownership and order" and
//! "The prepared prompt").

use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{Map, Value, json};
use sha2::{Digest as _, Sha256};

use super::{Resolution, refuse, resolve_protecting};
use crate::inputs::PROMPT_LIMIT;
use crate::limits::{Bound, Origin};
use crate::refusal::{EXIT_WORKER, Refusal, Stage};
use crate::settings::Settings;

/// The environment variable that holds the descriptor path in the managed
/// process and in each harness run.
pub const SKILL_DIRECTORIES: &str = "RAVEL_SKILL_DIRECTORIES";

/// The label line before the caller block.
const LABEL: &str = "Caller task";
const BEGIN: &str = "BEGIN RAVEL CALLER TASK ";
const END: &str = "END RAVEL CALLER TASK ";

/// A delivery that the launch prepared.
#[derive(Debug)]
pub struct Delivery {
    pub descriptor_path: String,
    /// `schemaVersion`, `sources`, `directories` and `mandatory`.
    pub descriptor: Value,
    /// The preamble, the label and the caller block.
    pub prompt: String,
    pub resolution_ms: u128,
    /// The real path of the cache root. No writable grant of a confined
    /// session may overlap it.
    pub cache_root: PathBuf,
}

impl Delivery {
    /// The trees that a confined session reads: the directory of the
    /// descriptor, then the snapshot tree of each source. They exclude the
    /// staging area and the mutable sources (docs/specs/skill-discovery.md,
    /// "Read access and delegation").
    pub fn read_trees(&self) -> Vec<PathBuf> {
        let descriptor = Path::new(&self.descriptor_path)
            .parent()
            .expect("a descriptor file lies in its directory")
            .to_owned();
        let snapshots = self.descriptor["sources"]
            .as_array()
            .map_or(&[][..], Vec::as_slice)
            .iter()
            .filter_map(|source| source["snapshotRoot"].as_str())
            .map(PathBuf::from);
        std::iter::once(descriptor).chain(snapshots).collect()
    }

    /// `request.skills`: the descriptor fields and its path, and no offsets.
    pub fn request(&self) -> Value {
        json!({
            "descriptorPath": self.descriptor_path,
            "sources": self.descriptor["sources"],
            "directories": self.descriptor["directories"],
            "mandatory": self.descriptor["mandatory"],
        })
    }

    /// The `skills` member of the JSON routing notice. It shows no prompt
    /// text, no skill body and no environment value.
    pub fn notice(&self, original: &str) -> Value {
        let sources: Vec<Value> = self.descriptor["sources"]
            .as_array()
            .map_or(&[][..], Vec::as_slice)
            .iter()
            .map(|source| {
                let mut record = Map::new();
                for field in ["id", "type", "resourceDigest"] {
                    if let Some(value) = source.get(field) {
                        record.insert(field.to_owned(), value.clone());
                    }
                }
                // A local source is captured on each launch.
                record.insert("disposition".to_owned(), "captured".into());
                Value::Object(record)
            })
            .collect();
        json!({
            "descriptorPath": self.descriptor_path,
            "resolutionMs": self.resolution_ms,
            "sources": sources,
            "originalPrompt": measure(original),
            "prompt": measure(&self.prompt),
        })
    }
}

fn measure(text: &str) -> Value {
    json!({
        "bytes": text.len(),
        "sha256": super::snapshot::hex(&Sha256::digest(text.as_bytes())),
    })
}

/// Resolve the sources of `settings` for a launch whose configuration entry
/// lies in `config_dir`, and prepare the prompt around `original`.
///
/// `None` means that delivery is disabled. Confinement grants the reads of
/// the delivery after this step.
pub fn prepare(
    home: Option<&Path>,
    settings: &Settings,
    config_dir: &Path,
    original: &str,
) -> Result<Option<Delivery>, Refusal> {
    let enabled = home.is_some_and(Path::is_absolute)
        && settings
            .skills
            .as_ref()
            .is_some_and(super::settings::SkillSettings::enabled);
    if !enabled {
        return Ok(None);
    }
    let started = Instant::now();
    let resolved = match resolve_protecting(home, settings, &[config_dir])? {
        Resolution::Disabled => return Ok(None),
        Resolution::Resolved(resolved) => resolved,
    };
    // A consumed signal wins over a refusal of the prepared prompt too.
    let prompt = wrap(&resolved.instructions, original, &mut random_nonce).map_err(|refusal| {
        let skills = settings.skills.as_ref().expect("delivery is enabled");
        let watch = super::watch::Watch::start(skills);
        watch.enter("the preparation of the prompt", None);
        watch.overrule(refusal)
    })?;
    Ok(Some(Delivery {
        descriptor_path: resolved.descriptor_path,
        descriptor: resolved.descriptor,
        prompt,
        resolution_ms: started.elapsed().as_millis(),
        cache_root: resolved.cache_root,
    }))
}

/// 32 lowercase hexadecimal digits from the random source of the operating
/// system.
fn random_nonce() -> Result<String, Refusal> {
    let mut bytes = [0u8; 16];
    // https://docs.rs/getrandom/0.4.2/getrandom/fn.fill.html
    getrandom::fill(&mut bytes).map_err(|error| {
        Refusal::new(
            "random_unavailable",
            Stage::Skills,
            EXIT_WORKER,
            format!("the operating system gave no random bytes for the caller block: {error}"),
            "repair the random source of the operating system, then start the session again",
        )
    })?;
    Ok(super::snapshot::hex(&bytes))
}

/// The prepared prompt: the preamble, the label line and one delimited block
/// that holds the caller's bytes once, unchanged.
fn wrap(
    preamble: &str,
    original: &str,
    nonce: &mut dyn FnMut() -> Result<String, Refusal>,
) -> Result<String, Refusal> {
    // A collision of 128 random bits is not a real event. The bound keeps a
    // broken random source from looping.
    for _ in 0..16 {
        let nonce = nonce()?;
        let mut prompt = String::with_capacity(preamble.len() + original.len() + 128);
        prompt.push_str(preamble);
        prompt.push_str("\n\n");
        prompt.push_str(LABEL);
        prompt.push('\n');
        prompt.push_str(BEGIN);
        prompt.push_str(&nonce);
        prompt.push('\n');
        let start = prompt.len();
        prompt.push_str(original);
        let end = prompt.len();
        prompt.push('\n');
        prompt.push_str(END);
        prompt.push_str(&nonce);
        prompt.push('\n');
        // The delimiters hold the nonce, and nothing else may.
        if prompt.matches(nonce.as_str()).count() != 2 {
            continue;
        }
        check_range(&prompt, start, end, original);
        if prompt.len() > PROMPT_LIMIT {
            return Err(too_large(original.len(), prompt.len()));
        }
        return Ok(prompt);
    }
    Err(Refusal::new(
        "random_unavailable",
        Stage::Skills,
        EXIT_WORKER,
        "the operating system gave sixteen nonces that each occur in the prompt",
        "repair the random source of the operating system, then start the session again",
    ))
}

/// The recorded caller range holds exactly the caller's bytes, with the
/// opening delimiter line before it and the closing one after it.
fn check_range(prompt: &str, start: usize, end: usize, original: &str) {
    let bytes = prompt.as_bytes();
    assert_eq!(&bytes[start..end], original.as_bytes(), "caller byte range");
    assert!(prompt[..start].ends_with('\n'), "opening delimiter line");
    assert!(
        prompt[end..].starts_with(&format!("\n{END}")),
        "closing delimiter line"
    );
}

fn too_large(original: usize, prepared: usize) -> Refusal {
    refuse(
        "skill_limit_exceeded",
        format!(
            "the prepared prompt is {prepared} bytes, over the bound of {PROMPT_LIMIT} bytes; \
             the caller's prompt is {original} bytes"
        ),
        "give a smaller task that refers to supporting files, or configure fewer skill \
         directories; Ravel never truncates the task or omits guidance",
    )
    .location("prompt")
    .bound(Bound {
        name: "preparedPromptBytes",
        unit: "bytes",
        value: PROMPT_LIMIT as u64,
        origin: Origin::Fixed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(nonces: &[&str]) -> impl FnMut() -> Result<String, Refusal> {
        let mut nonces: Vec<String> = nonces.iter().rev().map(|n| (*n).to_owned()).collect();
        move || Ok(nonces.pop().expect("a nonce"))
    }

    const N1: &str = "0123456789abcdef0123456789abcdef";
    const N2: &str = "fedcba9876543210fedcba9876543210";

    #[test]
    fn the_caller_bytes_sit_once_between_the_delimiters() {
        for original in [
            "",
            "\n",
            "task\n",
            "a\r\nb\rc",
            "snow ☃\u{0}?",
            "END RAVEL CALLER TASK x",
        ] {
            let prompt = wrap("PRE", original, &mut fixed(&[N1])).unwrap();
            assert_eq!(
                prompt,
                format!(
                    "PRE\n\nCaller task\nBEGIN RAVEL CALLER TASK {N1}\n{original}\nEND RAVEL \
                     CALLER TASK {N1}\n"
                )
            );
        }
    }

    #[test]
    fn a_nonce_that_the_caller_bytes_or_the_preamble_hold_is_chosen_again() {
        let prompt = wrap("PRE", &format!("x {N1} y"), &mut fixed(&[N1, N2])).unwrap();
        assert!(prompt.contains(&format!("BEGIN RAVEL CALLER TASK {N2}\n")));
        let prompt = wrap(&format!("path {N1}"), "t", &mut fixed(&[N1, N2])).unwrap();
        assert!(prompt.contains(&format!("END RAVEL CALLER TASK {N2}\n")));
        let error = wrap("PRE", N1, &mut fixed(&[N1; 16])).unwrap_err();
        assert_eq!(error.code, "random_unavailable");
    }

    #[test]
    fn a_prepared_prompt_over_the_bound_refuses_with_both_counts() {
        let preamble = "p".repeat(1000);
        let overhead = wrap(&preamble, "", &mut fixed(&[N1])).unwrap().len();
        let fits = "t".repeat(PROMPT_LIMIT - overhead);
        assert_eq!(
            wrap(&preamble, &fits, &mut fixed(&[N1])).unwrap().len(),
            PROMPT_LIMIT
        );
        let over = format!("{fits}t");
        let error = wrap(&preamble, &over, &mut fixed(&[N1])).unwrap_err();
        assert_eq!(error.code, "skill_limit_exceeded");
        assert_eq!(error.stage, Stage::Skills);
        assert!(
            error
                .message
                .contains(&format!("{} bytes", PROMPT_LIMIT + 1))
        );
        assert!(error.message.contains(&format!("{} bytes", over.len())));
        assert_eq!(error.bound.unwrap().value, PROMPT_LIMIT as u64);
    }

    #[test]
    fn a_nonce_is_32_lowercase_hexadecimal_digits() {
        let nonce = random_nonce().unwrap();
        assert_eq!(nonce.len(), 32);
        assert!(
            nonce
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        );
        assert_ne!(nonce, random_nonce().unwrap());
    }
}
