# The ephemeral session result

The private `ravel` Rust package contains the session-result transport that the
policy-routed command front uses. The
[session CLI contract](session-cli.md) remains the full product boundary.

`SessionChannel::allocate(parent)` creates a fresh private temporary directory
beneath an existing caller-selected parent, then allocates a `keyed-launch`
channel inside it. Choosing the parent lets a caller place completion inside a
directory its managed process can write through a sandbox. Allocation writes no
completion file. The inherited `RAVEL_EXIT_FILE` is the channel's absolute path.

The consumer passes `channel.channel()` and `EXIT_FILE_VARIABLE` as the runner's
`Launch.channel`. It scrubs old enclosing-session authority and lets this launch
publish its own path. A nested launch therefore replaces the path only in its
child's environment. No worker/policy request needs to learn this authority.

`end_session(result)` reads that inherited context and publishes one opaque
UTF-8 value. There is no session identifier or target argument in this interface.
`publish_result(path, result)` is its private Rust mechanism for an explicit
channel path; the CLI exposes no path or target argument.

Publication writes a temporary regular file in the same private directory,
then adds the completion name with a no-clobber hard link. Appearance therefore
reveals a fully written result to the runner. First publication wins: repeated
calls leave the original value intact. Empty strings, Unicode and newline bytes
are preserved. Missing, empty or stale inherited context refuses. Results are
limited to 1 MiB of UTF-8 bytes, above supported OS command-argument limits;
reading also enforces that bound against managed-process modifications.

The runner still observes only appearance, then supervises grace, termination,
group cleanup and reap. After that corresponding runner call returns, the
consumer calls `result_after(&ended)`. A present process group or cancellation
refuses. An ending without an observed signal returns `None`; a signalled empty
value returns `Some(String::new())`. The consumer must supply the ending of
this channel's own launch; the Rust type does not establish that relationship.
The command front owns CLI exit status and separates managed output on stderr
from exact result bytes on stdout.

Reads use a held directory descriptor, refuse final-entry symlinks, reject
nonregular files and never block on FIFOs. Renaming or replacing the directory
pathname cannot redirect those reads. `close()` removes ordinary session files
and reports cleanup errors; Drop is best effort. Cleanup checks the directory's
identity before removal. If it was already renamed or replaced, cleanup refuses
to delete the replacement and the moved directory needs manual cleanup. These
are process-context isolation guarantees, not authentication against code running
as the same OS user. A process killed without destructors can also leave temporary
files. There is no store, history index or durable session record.

`task test:session` runs the focused nextest suite, including real supervised
fixture processes. Optional nextest arguments follow `--`, for example
`task test:session -- --lib`. Fixtures independently scrub old/new live authority
using the existing runner helper, while Cargo's guard also clears those controls.
Ignored fixture roles are invoked by acceptance tests, not omitted checks.

`task check` retains complete lint/test diagnostics and runs the full Rust suite.
No Bun/Node runtime, policy, model account or installed launcher is needed here.
