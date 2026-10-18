# Session execution

Ravel is an independent policy router and process supervisor. Its public
boundary is [start-session and end-session](../book/src/06-reference.md#command-line). Caller prompts,
result meanings, task conventions and durable provenance belong to callers.
This specification describes the active executable, not the retired daemon.

The [component contracts](../book/src/07-the-parts-of-ravel.md) explain the current responsibility,
package and process map, with scoped source evidence and session traces. This
specification remains the behavioural authority for that explanation.

## Installed boundary

`bin/ravel` is the native Rust front. Its private worker is
`libexec/ravel/ravel-policy`, resolved relative to the **canonical real front**,
including when bin/ravel is reached through a symlink. PATH, argv[0], an ambient
runtime or a neighboring prefix cannot select another worker. The worker embeds
Bun 1.4.2; installed execution requires neither a host Bun/Node nor another
project's binaries. Rust and Bun agree on protocol 1, package version and the
SHA-256 source identity before the worker receives a policy. Probe identities
are never accepted. A missing, stale or mismatched worker refuses with exit 5.

The source identity hashes TypeScript under worker/src, worker/sdk and
worker/examples, plus both worker tsconfigs and scripts/dispatch.sh. Rust
build.rs and the worker build enumerate the same sorted paths and hash the
same per-file digest listing. The identity pairs builds; it is not code signing
or authentication against the owner of the installation.

The installed tree includes SDK source and generated declarations, the static
example source/declarations, an editable sample and runtime notices. The sample
is copied by the owner into their configuration; there is no init command.
`task install PREFIX=/path` builds a release pair, installs those assets and
verifies it from the installation alone. `task smoke` does that in a disposable
fresh prefix through a direct front and relative symlink.

## Archives and platform floors

`task release:archives` builds unreleased archives for aarch64-apple-darwin,
aarch64-unknown-linux-gnu and x86_64-unknown-linux-gnu. Each is the independent
installation prefix above plus README/LICENSE; an exact manifest refuses
omissions, extras, probes and caller-owned assets. The build retains the pinned
Bun/runtime digest path and uses Zig's glibc 2.17 target for the Linux front.

`task release:smoke` rebuilds and checks the archives on their target userlands,
without a host TypeScript runtime or caller binary. The macOS target runs
natively. Linux targets run in pinned glibc 2.17 userlands, with floor/above-floor
symbol probes and QEMU models for Nehalem and Cortex-A53. Above-floor CPU
instructions must fail with SIGILL at the floor and run under `-cpu max`.
The same direct/symlink installed seam runs on every target. Containers/QEMU
do not measure a kernel floor. [Delivery instructions](../RELEASING.md) own
the complete prerequisites, manifest/formula controls and evidence rules.

The formula installs the same bin/libexec layout and uses no Grove dependency.
Its temporary-prefix preparation test routes a fake agent through the actual
SDK/static example and verifies opaque-result bytes through a symlink. These
tasks cut/tag/publish/install no release; publication is a separate boundary.

## Policy authority and runtime discovery

The default entry is HOME/.config/ravel/policy.ts. HOME must be absolute when
that default is used. `--config PATH` explicitly chooses another entry relative
to the caller's cwd. No repository search, environment override or policy inside
Ravel selects the entry. It must canonicalize to a readable regular file whose
path can be imported exactly. Non-UTF-8 and `?` paths refuse before evaluation.
A symlink is admitted by its real file. Relative imports then belong to that
file's directory. Settings remain HOME/.config/ravel/settings.json even with
an explicit entry. They accept only timeoutMs, contextBytes and policyEnv;
unknown keys, including obsolete storage keys, refuse without opening storage.

Policy is executable owner configuration, evaluated **outside** any managed
process sandbox. It is not a security sandbox against malicious owner code.
The worker starts in a private mode-0700 empty directory with null stdin and a
fresh environment, private socket at fd 3 and no caller descriptors above it.
Only HOME, PATH, TMPDIR, LANG, LC_* and granted environment names reach it.
Runtime injection names are never grantable: BUN_*, NODE_OPTIONS, NODE_PATH,
NODE_PRESERVE_SYMLINKS, NODE_CHANNEL_*, LD_*, DYLD_*, RAVEL_* and legacy dispatch
controls. Ravel sets BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 before startup.
Grant diagnostics report names/presence, never values.

The compiled worker disables dotenv, bunfig and tsconfig autoloading explicitly.
It enables package.json autoloading, then moves to `/` before registering modules
or importing the policy. This admits declared package main/exports while
preventing ancestors of its temporary start directory from participating.
Modules in files resolve from their own file locations; modules with no file
location resolve from the process directory. `/package.json` and /node_modules
remain part of the runtime's resolution boundary. Policy code can itself change
cwd or import arbitrary code; the controls prevent ambient code taking part
before the owner entry has been admitted.

Exact imports of ravel/sdk and ravel/examples/static resolve to embedded modules,
even beside package shadows. The prefix reserves no other names. The nearest
package.json that reads as a package supplies a module's imports map and own
package name/exports; an unparseable/non-package file is passed over. An imports
alias targeting a registered name is a package lookup and can load a shadow;
it does not return to the embedded registration. A standalone worker never
installs a missing dependency automatically. [Runtime evidence](../runtime-evidence.md)
and [the worker decision](../adr/policy-evaluation-precedes-the-launch.md)
state the source grounding and limits.

## Request, context and selection

The request has schemaVersion 2, kind, prompt, cwd, params and effective limits;
optional taskFile, taskId and context are caller data. Kind is an open nonempty
token; taskId is nonempty UTF-8 of at most 1024 bytes. taskFile is an absolute
UTF-8 path and is not read automatically. Prompt requires exactly one of
--prompt or --prompt-file and is passed unchanged, including empty/newline text.
Each --param NAME=VALUE is caller data; names are nonempty without `=`, duplicates
refuse and total name/value bytes are at most 65536. Ravel injects no instruction.

The policy entry exports one plain object named policy, with schemaVersion 2,
a nonblank version, select and optionally loadContext. Unknown fields refuse.
loadContext receives a frozen request and a host with readText/readJson,
diagnostic and an AbortSignal. select receives the request, delivered context
and a host without reads. Both functions may be asynchronous. No context loader
means select gets caller context, or undefined when none was supplied.

Context schemaVersion 1 admits summary, acceptanceCriteria, facts, assessments
and sources; unknown/executable fields refuse. Host reads resolve relative to
request.cwd, read regular files, and return canonical names, byte counts and
SHA-256 of the actual bytes. Ravel attaches measured sources to the delivered
context; a loader may not invent measured. The caller --context is measured
first. The delivered context is frozen. Direct policy filesystem/network reads
are possible but are not measured host reads. A read-bound breach still refuses
if policy catches the exception. Attribution is data, not authentication.

select returns either a selected command or a refusal. Selected fields are
status, program, args, provider, model, effort and reason; all labels/program
are nonblank, args is an array of UTF-8 strings with no NUL. Ravel performs no
shell expansion or word splitting. A shell runs only if the policy selects it.
Absolute programs, relative paths with separators and PATH names resolve in the
caller's execution context. Labels are owner text, not provider integrations.
A refusal carries status=refused, code, message and remedy and launches nothing.
Abstention, malformed values, throws, rejected/unsettled promises, invalid
context and exceeded bounds also refuse. Ravel never substitutes another route.

| Bound | Default | Override/range |
|---|---:|---|
| Whole selection | 30000 ms | timeoutMs / --timeout-ms, 1000..600000 |
| Delivered context | 262144 bytes | contextBytes / --context-bytes, 1..8388608 |
| One host source | 65536 bytes | host read maxBytes, at most context budget |
| Sources | 256 | Fixed |
| Protocol frame | 1048576 bytes | Fixed |
| Worker stdout + stderr | 262144 bytes | Fixed |

The front judges serialized policy/context/selection data, not worker stdout.
The length-prefixed JSON protocol uses a separate socket. Output is drained
concurrently and bounded. The deadline starts at worker spawn and covers import,
context and selection. Cancellation or timeout stops the worker; cleanup offers
at most one second before KILL/reap and diagnostic drain at most one second.
Detached policy children are outside the worker cleanup guarantee. A worker
must be reaped before the managed process starts.

## Managed process and result

Each launch allocates a fresh private session directory and result path.
RAVEL_EXIT_FILE reaches only the managed process as its enclosing context;
nested launches replace that value only for their own child. Old dispatch and
obsolete Ravel identity/storage controls are scrubbed. A nonconfined managed
process otherwise receives the caller environment. The caller must put ravel
on the managed PATH or supply its absolute path in ending instructions.

end-session takes exactly one opaque UTF-8 argument, reads no policy/settings
and accepts no target. It writes a complete temporary regular file, then publishes
with a no-clobber hard link. The first value wins; a repeated publication succeeds
without replacing it. The limit is 1 MiB. Missing/empty/stale context refuses.
Readers are held-directory anchored and reject symlinks, FIFOs and nonregular
entries. Directory identity is checked for cleanup: replacements are preserved
and renamed owned directories may require manual cleanup. This is isolation by
inherited context, not authentication against same-user code.

Publication requests the supervisor's 2-second grace, then TERM, up to 5 seconds
kill-grace, then KILL when needed. Descendant group cleanup occurs before reap,
including on natural exit. The runner preserves job control and recovers the
terminal; [the child is a job](../adr/the-launched-child-is-a-job.md). After
confirmed reap/group cleanup and explicit temporary cleanup, only the exact
result bytes go to stdout, with no added newline. Managed stdout is directed to
stderr before spawn, leaving stdin/terminal ownership with the runner. Diagnostics
and optional --json routing notices/refusals are on stderr too. Result stdout
must be writable before launch; write failure after reap is still an error.

A supplied result succeeds regardless of the child's later status. Natural
exit without a result is exit 1 even for status 0. Cancellation by INT/TERM/HUP
returns the signal, emits no result and reports cleanup failures/group survivors.
No successful result is returned on cleanup/supervision failure. CLI errors use
2, policy refusals 3, worker/supervision failures 5, selection timeout 124 and
program failures 126/127. Ravel assigns no meaning to any result token.

--confine confines the managed process only, without an unconfined fallback.
It requires macOS sandbox-exec or Linux bubblewrap. Canonical cwd and a private
run directory are writable; --exit-dir adds an existing writable channel parent.
--runtime-read grants repeatable regular files read-only and requires confinement.
System runtime reads and the real front are admitted. Canonical grants must not
overlap the policy directory or owner settings. Confined environment keeps HOME,
USER, LOGNAME, PATH, LANG and LC_*; TMP/TEMP/TMPDIR point into the private run.
The backend's filesystem contract and platform limits live in
[the runner documentation](../book/src/09-jobs-signals-and-the-terminal.md#apply-filesystem-confinement).

## Verification and scope

Taskfile is the reusable entry point. task check formats/lints scripts and all
Rust targets, rebuilds the worker/probes, typechecks source and shipped declarations,
checks pinned-runtime integrity, and runs cargo-nextest with complete saved logs.
Ignored subprocess roles are invoked by actual acceptance tests. task smoke
verifies the native installed pair, separately from future target archive checks.
No models, owner credentials, personal policy or old store are needed.

Hostile classes have firing controls in the same test: autoload, tsconfig,
unregistered and unmoved builds, or the real worker started without front scrubs.
Probes have identities no front accepts and never enter an installed layout.
Classes without known firing configurations are reported as unmeasured tripwires,
never counted as controls. See runtime-evidence.md for cases and upgrade rules.

There is no persistence, run identity, observation/record/history API, importer,
caller adapter, or public direct-command bypass. Ephemeral debris after abrupt
kill is possible. The retired-project retrieval record remains independent.
