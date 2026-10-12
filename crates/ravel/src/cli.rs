//! Ravel's two public verbs. Result stdout is an opaque byte stream.
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};
use clap::{Args, Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "ravel",
    version,
    disable_help_subcommand = true,
    about = "Route and supervise a session, returning its opaque result",
    after_help = "Examples:\n  ravel start-session --kind feature --prompt 'Implement the parser; finish with ravel end-session done'\n  ravel end-session done"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Route the prompt unchanged through owner policy and supervise its selected process
    #[command(
        after_help = "Only the exact result is written to stdout, without a newline. Session output and diagnostics go to stderr. A child ending without a result exits 1; cancellation returns its signal with no result. Malformed input exits 2, policy refusal 3, worker/supervision failure 5, selection timeout 124, program failures 126/127. After end-session, Ravel waits 2 seconds, sends TERM, waits up to 5 seconds, then KILL and reaps the process group.\n\nExamples:\n  ravel start-session --kind feature --prompt-file ./prompt.txt\n  ravel start-session --kind check --config ./policy.ts --param profile=careful --prompt 'Check and end with a result'"
    )]
    StartSession(Box<RunArgs>),
    /// Publish exactly one opaque result to the enclosing session and request its end
    #[command(
        after_help = "Uses inherited RAVEL_EXIT_FILE; no session ID or target argument. The first publication wins. Missing or stale context exits 1. No settings or policy are read.\n\nExamples:\n  ravel end-session done\n  ravel end-session -- '--help'\n  ravel end-session ''"
    )]
    EndSession {
        #[arg(value_name = "RESULT")]
        result: String,
    },
}
#[derive(Debug, Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub selection: SelectionArgs,
    /// Confine the managed process; policy selection remains outside the sandbox
    #[arg(long)]
    pub confine: bool,
    /// Grant a regular runtime file read-only to a confined process; repeatable
    #[arg(long, value_name = "FILE", requires = "confine")]
    pub runtime_read: Vec<PathBuf>,
    /// Allocate the private session channel beneath this existing writable directory
    #[arg(long, value_name = "DIR")]
    pub exit_dir: Option<PathBuf>,
    /// Render policy refusals and routing notices as JSON on stderr
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct SelectionArgs {
    /// The caller's session kind: any nonempty token, given to the policy's select as it is
    #[arg(long, value_name = "TEXT")]
    pub kind: String,
    /// The harness prompt, given to the policy's select byte for byte; run needs this or --prompt-file
    #[arg(
        long,
        value_name = "TEXT",
        allow_hyphen_values = true,
        conflicts_with = "prompt_file"
    )]
    pub prompt: Option<OsString>,
    /// Read the harness prompt from this file, relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub prompt_file: Option<PathBuf>,
    /// The task's file, relative to the current directory; supplies no kind or identity
    #[arg(long, value_name = "PATH")]
    pub task_file: Option<PathBuf>,
    /// The task's stable identity: opaque UTF-8, at most 1024 bytes
    #[arg(long, value_name = "ID", allow_hyphen_values = true)]
    pub task_id: Option<OsString>,
    /// Evaluate this policy entry instead of the personal default; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Caller data for the policy's select, by name; repeatable. The name is nonempty and holds no =, the value is any UTF-8 text, a repeated name refuses, and all names and values together are at most 65536 bytes
    #[arg(long, value_name = "NAME=VALUE", allow_hyphen_values = true)]
    pub param: Vec<OsString>,
    /// A version-1 JSON context document for the policy, read as data; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub context: Option<PathBuf>,
    /// Stop the policy and launch nothing (exit 124) if selection takes longer; 1000 to 600000, replacing the timeoutMs owner setting [default: 30000]
    #[arg(long, value_name = "MS")]
    pub timeout_ms: Option<OsString>,
    /// Refuse a context delivered to selection that encodes to more bytes; 1 to 8388608, replacing the contextBytes owner setting [default: 262144]
    #[arg(long, value_name = "BYTES")]
    pub context_bytes: Option<OsString>,

    #[arg(long, value_name = "NAME")]
    pub policy_env: Vec<OsString>,
}

impl SelectionArgs {
    /// Refuse the empty values clap admits. The values themselves are read and
    /// checked by `inputs`.
    pub fn refuse_empty(&self) -> Result<(), Refusal> {
        if self.kind.is_empty() {
            return Err(Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                "--kind must not be empty",
                "pass the caller's session kind, such as --kind impl",
            )
            .input("--kind"));
        }
        if self
            .config
            .as_ref()
            .is_some_and(|config| config.as_os_str().is_empty())
        {
            return Err(Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                "--config must not be empty",
                "name a policy entry, or omit --config to use the personal default",
            )
            .input("--config"));
        }
        Ok(())
    }
}
