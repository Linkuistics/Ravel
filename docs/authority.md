# Authority boundaries

The owner policy is trusted executable configuration. Ravel chooses it only from
an explicit --config or the absolute-HOME default. The native front holds the
caller's environment, terminal, result stdout and supervision authority. The
compiled worker gets a bounded frozen request, measured context operations and
explicit environment grants, but no enclosing-session completion authority.
It is reaped before the selected process starts. Filesystem/network access by
owner policy is outside the managed-process confinement boundary.

The managed process gets its own ephemeral RAVEL_EXIT_FILE. Nested sessions
replace only their child's context. end-session addresses inherited context,
without a target argument or persistent lookup. Same-user code can still tamper
with files or launch unrelated processes; neither identity matching nor context
is an authentication scheme. Detached policy children and abruptly orphaned
managed processes are explicit limits in the
[executable specification](specs/session-execution.md).

Tests must never act as the enclosing development session. Cargo's
[forced-empty controls](../.cargo/config.toml) cover old launch/signal/dispatch
controls and current/obsolete Ravel spellings. Empty values disable authority;
they do not redirect it to another path. Runner subprocess helpers independently
scrub the same names when invoked directly. Front/hostile command fixtures use
env_clear plus temporary HOME/TMPDIR and fixture PATH; installed smoke uses env -i.
No test inherits personal owner policy or a real completion path.

The legacy GROVE_* and HARNESS_DISPATCH_* spellings in those guards are intentional
extraction safety exceptions, not product dependencies. Production environment
grants exclude HARNESS_DISPATCH_* as well as RAVEL_*; ordinary unrelated owner
grants remain generic. Caller-specific paths in source provenance and the retired
project document are historical attribution. Ravel neither loads nor interprets
those projects or their task trees. [The caller-reference inventory](caller-references.md)
classifies active references, including dotfiles.
