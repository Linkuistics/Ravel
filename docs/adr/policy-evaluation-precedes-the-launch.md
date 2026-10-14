# Policy evaluation precedes the launch

Ravel uses a native Rust front and a matching installed Bun-compiled worker.
The front retains caller environment, terminal and supervision authority, gives
the worker a bounded request without completion authority, and requires its reap
before the selected process starts. Owner policy never executes inside the process
that supervises the session. The [specification](../specs/session-execution.md)
owns the protocol and [runtime evidence](../runtime-evidence.md) owns the observations.

That worker-before-job ordering remains required. The
[acquisition evidence](../result-job-source-evidence.md#c2-worker-wait-error-does-not-gate-selection-success)
shows a source path where successful conversation does not check the worker's
teardown wait result. The decision is retained; this missing confirmation is
failure-path conformance debt, not permission to overlap worker and job.

Bun supplies TypeScript, imports and ordinary filesystem/network computation
without Ravel maintaining a language runtime. Compilation removes a separately
installed runtime from the execution contract. Costs are installation size,
a second build tool, runtime notices and target execution checks. A single Bun
supervisor was rejected because startup hooks run before application scrubbing
and policy would share completion authority. Reopen only for a verified equivalent
native startup/signal/supervision boundary. Embedding a smaller engine was rejected
because it makes Ravel own TypeScript/module/host support; reconsider if delivery
cost or supported-target compatibility outweighs those obligations.

Bun's ambient startup/resolver inputs require controls before policy evaluation.
The front scrubs runtime options and disables the runtime transpiler cache before
startup. The worker disables dotenv/bunfig/tsconfig autoloading, starts in a private
empty directory, then moves to `/` before registering exact embedded imports or
loading owner code. package.json autoloading stays on to admit declared main/exports.
The move prevents ancestors of TMPDIR taking part in no-file module resolution.
`/package.json` and /node_modules remain; owner code can choose another cwd itself.

The move leaves the dotenv switch as the only control for a native Worker started
later: each new VM can load dotenv from the process's current directory. That
trade-off is accepted to remove ambient TMPDIR reach. An imports alias whose target
is an embedded specifier remains a package lookup; owning Bun's full resolver to
intercept it was rejected. Use the literal embedded import when that identity is
required. The shadow/autoload/cache/resolver probes make those limits executable,
with source-pinned upgrade instructions in the evidence document.
