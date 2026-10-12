//! The caller's data inputs, read and checked once, before any policy runs
//! (`docs/specs/session-execution.md`, *Command interface*).
//!
//! The kind, the task file, the task identity and the parameters are caller
//! data: the task file supplies neither kind nor identity, nothing is recovered
//! from a file name, and a parameter's name and value mean nothing here. The
//! prompt is read once and kept byte for byte, and the policy's `select`
//! receives it as it is. Terminal stdin is never read. The caller's bounds,
//! `--context` document and `--policy-env` grants are read here too, over the
//! owner's settings, so a malformed or excluded one refuses before any policy
//! runs.

use crate::cli::SelectionArgs;
use crate::context::{self, CallerContext};
use crate::environment::Grants;
use crate::limits::Limits;
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED, EXIT_REFUSED};
use crate::settings::Settings;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::io::Read as _;
use std::os::fd::AsRawFd as _;
use std::path::{Path, PathBuf};

/// The prompt's fixed bound, in bytes. Platform argv limits can refuse a
/// smaller one at exec.
pub const PROMPT_LIMIT: usize = 1024 * 1024;

/// The task identity's bound, in bytes of UTF-8.
pub const TASK_ID_LIMIT: usize = 1024;

/// The parameters' fixed bound: every name and value together, in bytes.
pub const PARAMS_LIMIT: usize = 64 * 1024;

#[derive(Debug)]
pub struct Inputs {
    pub kind: String,
    /// The original cwd: relative inputs resolve against it, and the harness
    /// runs in it.
    pub cwd: PathBuf,
    /// Absolute, joined to the original cwd; not read, and not required to
    /// exist.
    pub task_file: Option<String>,
    pub task_id: Option<String>,
    /// Every `--param`, by name.
    pub params: BTreeMap<String, String>,
    pub prompt: Prompt,
    /// The caller's `--context` document, read, measured and validated.
    pub context: Option<CallerContext>,
    /// Every bound in effect: the owner's settings, and the caller's
    /// `--timeout-ms` and `--context-bytes` over them.
    pub limits: Limits,
    /// The names the owner settings and `--policy-env` grant the worker
    /// beyond its base set.
    pub grants: Grants,
}

#[derive(Debug)]
pub struct Prompt {
    pub text: String,
    pub source: PromptSource,
}

#[derive(Debug)]
pub enum PromptSource {
    Argument,
    /// The file it was read from, joined to the original cwd.
    File(PathBuf),
}

impl Inputs {
    pub fn read(args: &SelectionArgs, settings: &Settings) -> Result<Inputs, Refusal> {
        args.refuse_empty()?;
        let cwd = std::env::current_dir().map_err(|error| {
            Refusal::new(
                "cwd_unavailable",
                Stage::Authority,
                EXIT_REFUSED,
                format!("the current directory cannot be read: {error}"),
                "run ravel from an existing, readable directory",
            )
            .input("cwd")
        })?;
        let task_file = args
            .task_file
            .as_deref()
            .map(|path| task_file(path, &cwd))
            .transpose()?;
        let task_id = args.task_id.clone().map(task_id).transpose()?;
        let params = params(&args.param)?;
        let prompt = match (&args.prompt, &args.prompt_file) {
            (Some(text), _) => Prompt {
                text: prompt_text(text.clone(), "--prompt", None)?,
                source: PromptSource::Argument,
            },
            (None, Some(path)) => read_prompt(&cwd.join(path))?,
            (None, None) => {
                return Err(Refusal::new(
                    "malformed_input",
                    Stage::Cli,
                    EXIT_MALFORMED,
                    "start-session needs the harness prompt, and none was given",
                    "pass exactly one of --prompt TEXT or --prompt-file PATH; ravel \
                     never reads a prompt from stdin",
                )
                .input("--prompt"));
            }
        };
        let grants = Grants::read(&settings.policy_env, &args.policy_env)?;
        let limits = Limits::read(
            args.timeout_ms.as_deref(),
            args.context_bytes.as_deref(),
            settings,
        )?;
        let context = args
            .context
            .as_deref()
            .map(|path| context::read_caller(path, &cwd, &limits))
            .transpose()?;
        Ok(Inputs {
            kind: args.kind.clone(),
            cwd,
            task_file,
            task_id,
            params,
            prompt,
            context,
            limits,
            grants,
        })
    }
}

fn malformed(input: &str, message: impl Into<String>, remedy: &str) -> Refusal {
    Refusal::new(
        "malformed_input",
        Stage::Cli,
        EXIT_MALFORMED,
        message,
        remedy,
    )
    .input(input)
}

/// Joined, not canonicalized: the caller's spelling of the path is kept, and
/// the file need not exist. It travels in JSON and as one argument, so it must
/// be UTF-8. (Clap has already refused an empty path.)
fn task_file(path: &Path, cwd: &Path) -> Result<String, Refusal> {
    cwd.join(path)
        .into_os_string()
        .into_string()
        .map_err(|path| {
            malformed(
                "--task-file",
                format!(
                    "--task-file {} is not valid UTF-8",
                    Path::new(&path).display()
                ),
                "name the task's file as a UTF-8 path, or omit --task-file",
            )
        })
}

fn task_id(id: OsString) -> Result<String, Refusal> {
    let remedy = "pass the task's stable identity as nonempty UTF-8 of at most 1024 bytes, such \
                  as its handle, or omit --task-id";
    let id = id
        .into_string()
        .map_err(|_| malformed("--task-id", "--task-id is not valid UTF-8", remedy))?;
    if id.is_empty() {
        return Err(malformed(
            "--task-id",
            "--task-id must not be empty",
            remedy,
        ));
    }
    if id.len() > TASK_ID_LIMIT {
        return Err(malformed(
            "--task-id",
            format!(
                "--task-id is {} bytes, over the {TASK_ID_LIMIT}-byte limit",
                id.len()
            ),
            remedy,
        ));
    }
    Ok(id)
}

/// Every `--param NAME=VALUE`, by name. The name ends at the first `=`, so it
/// holds none, and the value is the rest, exactly as given. Nothing here reads
/// either: a parameter means what the policy makes of it.
fn params(given: &[OsString]) -> Result<BTreeMap<String, String>, Refusal> {
    let remedy = "pass each parameter once as --param NAME=VALUE, with a nonempty name that \
                  holds no = and a UTF-8 value";
    let mut params = BTreeMap::new();
    let mut bytes = 0;
    for param in given {
        let param = param
            .to_str()
            .ok_or_else(|| malformed("--param", "--param is not valid UTF-8", remedy))?;
        let (name, value) = match param.split_once('=') {
            Some((name, value)) if !name.is_empty() => (name, value),
            Some(_) => {
                return Err(malformed(
                    "--param",
                    format!("--param {param:?} has no name before its ="),
                    remedy,
                ))
            }
            None => {
                return Err(malformed(
                    "--param",
                    format!("--param {param:?} has no =, so it names no value"),
                    remedy,
                ))
            }
        };
        bytes += name.len() + value.len();
        if params.insert(name.to_owned(), value.to_owned()).is_some() {
            return Err(malformed(
                "--param",
                format!("--param {name:?} is given more than once"),
                remedy,
            ));
        }
    }
    if bytes > PARAMS_LIMIT {
        return Err(malformed(
            "--param",
            format!(
                "the parameters' names and values are {bytes} bytes together, over the \
                 {PARAMS_LIMIT}-byte limit"
            ),
            "pass less in parameters; a policy can read a larger input from a file that a \
             parameter names",
        ));
    }
    Ok(params)
}

/// Read a prompt file once, bounded, refusing a terminal rather than waiting
/// on it.
fn read_prompt(path: &Path) -> Result<Prompt, Refusal> {
    let shown = path.to_string_lossy().into_owned();
    let unreadable = |what: String| {
        Refusal::new(
            "prompt_unreadable",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("the prompt file {shown} {what}"),
            "name a readable prompt file with --prompt-file, or pass the prompt with --prompt",
        )
        .input("--prompt-file")
        .source(shown.clone())
    };
    let file =
        fs::File::open(path).map_err(|error| unreadable(format!("cannot be opened: {error}")))?;
    // SAFETY: isatty only inspects the descriptor, which `file` owns.
    if unsafe { libc::isatty(file.as_raw_fd()) } == 1 {
        return Err(unreadable(
            "is a terminal, and ravel never reads its prompt interactively".to_owned(),
        ));
    }
    let mut bytes = Vec::new();
    file.take(PROMPT_LIMIT as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| unreadable(format!("cannot be read: {error}")))?;
    let text = prompt_bytes(bytes, "--prompt-file", Some(&shown))?;
    Ok(Prompt {
        text,
        source: PromptSource::File(path.to_owned()),
    })
}

fn prompt_text(text: OsString, input: &str, source: Option<&str>) -> Result<String, Refusal> {
    use std::os::unix::ffi::OsStringExt as _;
    prompt_bytes(text.into_vec(), input, source)
}

/// Check a prompt's bytes: within the bound, UTF-8, and free of NUL, which no
/// argument can carry. The bytes are otherwise kept exactly, trailing newlines
/// included.
fn prompt_bytes(bytes: Vec<u8>, input: &str, source: Option<&str>) -> Result<String, Refusal> {
    let invalid = |message: String| {
        let refusal = Refusal::new(
            "prompt_invalid",
            Stage::Cli,
            EXIT_MALFORMED,
            message,
            format!(
                "supply the prompt as UTF-8 text of at most {PROMPT_LIMIT} bytes with no NUL; it \
                 is given to the policy unchanged and never truncated"
            ),
        )
        .input(input);
        match source {
            Some(source) => refusal.source(source),
            None => refusal,
        }
    };
    if bytes.len() > PROMPT_LIMIT {
        return Err(invalid(format!(
            "the prompt is over the {PROMPT_LIMIT}-byte limit"
        )));
    }
    if let Some(at) = bytes.iter().position(|byte| *byte == 0) {
        return Err(invalid(format!(
            "the prompt contains a NUL byte at offset {at}"
        )));
    }
    String::from_utf8(bytes).map_err(|error| {
        invalid(format!(
            "the prompt is not valid UTF-8 (at byte {})",
            error.utf8_error().valid_up_to()
        ))
    })
}
