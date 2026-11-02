//! Ravel's two public verbs. Result stdout is an opaque byte stream.
use crate::refusal::{EXIT_MALFORMED, Refusal, Stage};
use clap::{Args, Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "ravel",
    version,
    disable_help_subcommand = true,
    about = "Route and supervise a session, returning its opaque result",
    after_help = "Examples:\n  ravel start-session --kind feature --prompt 'Implement the parser; finish with ravel end-session done'\n  ravel end-session done\n  ravel skills resolve"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Route the prompt, with any configured skill directories, through an owner Rhai configuration script and supervise its selected process
    #[command(
        after_help = "Only the exact result is written to stdout, without a newline. Session output and diagnostics go to stderr. A child ending without a result exits 1; cancellation returns its signal with no result. Malformed input exits 2, configuration refusal 3, worker/supervision failure 5, selection timeout 124, program failures 126/127. After end-session, Ravel waits 2 seconds, sends TERM, waits up to 5 seconds, then KILL and reaps the process group.\n\nExamples:\n  ravel start-session --kind feature --prompt-file ./prompt.txt\n  ravel start-session --kind check --config ./config.rhai --param profile=careful --prompt 'Check and end with a result'"
    )]
    StartSession(Box<RunArgs>),
    /// Send exactly one opaque result to Ravel and ask Ravel to end the enclosing session
    #[command(
        after_help = "The command uses the RAVEL_EXIT_FILE that it inherits. It has no session ID argument and no target argument. Ravel keeps the first result that it receives. If the context is missing or stale, the command exits with status 1. The command reads no settings and no Rhai configuration script.\n\nExamples:\n  ravel end-session done\n  ravel end-session -- '--help'\n  ravel end-session ''"
    )]
    EndSession {
        #[arg(value_name = "RESULT")]
        result: String,
    },
    /// Resolve the skill sources of the owner settings, without a configuration script or a harness
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum SkillsCommand {
    /// Capture the configured skill sources, store their snapshots and print the discovery instructions
    #[command(
        after_help = "The command reads the skills object of ~/.config/ravel/settings.json. It evaluates no configuration script, creates no result channel and launches no harness. It takes no skill name, task or source override.\n\nCache side effect: each run copies the current bytes of every local source and stores immutable snapshots and a directory descriptor under ~/.cache/ravel/skills/v1. Ravel never removes them automatically. A failure can leave complete snapshots, but never a partial descriptor.\n\nOutput: the discovery instructions, with a final newline. With --json, one object with schemaVersion 1 and status \"resolved\", and descriptorPath, sources, directories, mandatory and instructions. When delivery is disabled, the plain output is empty, --json prints {\"schemaVersion\":1,\"status\":\"disabled\"}, and the status is 0. A failure goes to stderr with a nonzero status and prints no instructions: 2 for a command-line or settings problem, 3 for a skill refusal, 5 for an internal failure.\n\nExamples:\n  ravel skills resolve\n  ravel skills resolve --json"
    )]
    Resolve {
        /// Print one JSON object, and render a failure as a JSON failure document on stderr
        #[arg(long)]
        json: bool,
    },
}
#[derive(Debug, Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub selection: SelectionArgs,
    /// Confine the managed process; configuration script selection remains outside the sandbox
    #[arg(long)]
    pub confine: bool,
    /// Grant a regular runtime file read-only to a confined process; repeatable
    #[arg(long, value_name = "FILE", requires = "confine")]
    pub runtime_read: Vec<PathBuf>,
    /// Allocate the private session channel beneath this existing writable directory
    #[arg(long, value_name = "DIR")]
    pub exit_dir: Option<PathBuf>,
    /// Render configuration refusals and routing notices as JSON on stderr
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct SelectionArgs {
    /// The caller's session kind: any nonempty token, given to the configuration script's select as it is
    #[arg(long, value_name = "TEXT")]
    pub kind: String,
    /// The harness prompt, given to the configuration script's select byte for byte as originalPrompt; run needs this or --prompt-file
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
    /// Evaluate this Rhai configuration entry instead of ~/.config/ravel/config.rhai; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Caller data for the configuration script's select, by name; repeatable. The name is nonempty and holds no =, the value is any UTF-8 text, a repeated name refuses, and all names and values together are at most 65536 bytes
    #[arg(long, value_name = "NAME=VALUE", allow_hyphen_values = true)]
    pub param: Vec<OsString>,
    /// A version-1 JSON context document for the configuration script, read as data; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub context: Option<PathBuf>,
    /// Stop the configuration script and launch nothing (exit 124) if selection takes longer; 1000 to 600000, replacing the timeoutMs owner setting [default: 30000]
    #[arg(long, value_name = "MS")]
    pub timeout_ms: Option<OsString>,
    /// Refuse a context delivered to selection that encodes to more bytes; 1 to 8388608, replacing the contextBytes owner setting [default: 262144]
    #[arg(long, value_name = "BYTES")]
    pub context_bytes: Option<OsString>,

    /// Let the configuration script read this environment variable with ravel::env, beyond HOME, PATH, TMPDIR, LANG and LC_*; repeatable, added to the configEnv setting. Names Ravel reserves refuse: LD_*, DYLD_*, RAVEL_* and HARNESS_DISPATCH_*. Diagnostics report each name and whether it is set, never a value
    #[arg(long, value_name = "NAME")]
    pub config_env: Vec<OsString>,
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
                "name a configuration entry, or omit --config to use the personal default",
            )
            .input("--config"));
        }
        Ok(())
    }
}
