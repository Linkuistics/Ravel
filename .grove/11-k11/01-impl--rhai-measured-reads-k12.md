# rhai-measured-reads-k12

## Goal

Add the bridge between the policy worker and the Ravel CLI. Use it for the first host functions: measured reads, module imports and `load_context`. A Rhai policy can then read a task file, import a helper module and assemble its own context.

## Context

- The input is `docs/specs/policy-evaluation.md`. Read "The bridge between the script and the CLI", "Measured reads", "Policy entry and discovery" and "Request, context and selection".
- The review finding F1 of `rhai-policy-k4` and its integration in `rhai-policy-k5` define the read mechanism. Find both with `grove-llm resolve`.
- A host function is a blocking call in the worker. It sends one request over the private socket and waits for the reply. The script sees synchronous calls.
- The Ravel CLI serves the socket from a Tokio current-thread runtime that exists only for the evaluation phase. The runtime keeps watching the deadline, the cancellation latch, the health of the worker and the output bounds.
- The runtime thread makes no filesystem call. A reader thread does every read. That includes the entry and each module. Do not use a Tokio blocking task, because Tokio cannot stop one.
- A stop does not wait for a read in progress. The reader thread keeps the descriptor until the call returns or the CLI exits. Every descriptor is close-on-exec.
- The worker sends its result only after the reply to its last host call. A worker message during a host call breaks the protocol and refuses as `worker_failed`.
- Add Tokio 1 with a current-thread runtime. Edition 2024 and Rust 1.99.1 are already in place.

## Done when

- `ravel::read_text(path[, max_bytes])` and `ravel::read_json(path[, max_bytes])` work in `load_context`, in `select` and at the top level of a module. A relative path resolves against `request.cwd`.
- A read opens the file without blocking, checks that it is a regular file, and reads at most one byte beyond its bound. It returns the content and a source record with the canonical name, the byte count and the SHA-256.
- A data failure throws a map with `code`, `message` and `name`. The codes are `source_missing`, `source_not_regular`, `source_unreadable`, `source_not_utf8` and `source_invalid_json`. A script can catch them.
- A breach of `source_too_large` or `too_many_sources` ends the evaluation, also when the script catches the error. The bound of 256 sources counts the caller's context document and every read.
- `import "PATH" as NAME;` resolves a relative path from the directory of the importing file. Ravel appends `.rhai` when the path has no extension, and it accepts an absolute path. Each module loads once. The ends are `policy_import_failed` for a missing or unreadable module, a cycle, a 65th module, or a failing top level.
- A syntax error in a module refuses as `policy_parse_failed`, and `source` names that module.
- `load_context(request)` works as the spec says. The result goes to `select` with the measured sources attached. A return of `()` refuses as `context_invalid`. A refusal returns as `policy_refused`, and `select` does not run. An uncaught error refuses as `context_loader_failed` with file, line and column. Remove the temporary refusal that `rhai-policy-worker` added.
- A malformed host request ends the evaluation as `host_request_invalid`. The location names the function, the field and the line.
- The notice lists each module with its canonical name, bytes and SHA-256. It lists every measured read with `via`, phase, name, bytes and SHA-256.
- Tests at the CLI seam cover a read in `select` and a read in `load_context`. They also cover an import, a module syntax error, a cycle, and source bounds that a script catches. The last case is the FIFO case. The FIFO case calls `read_text` on a FIFO that no process writes. The call throws `source_not_regular` at once. The firing control is a blocking open of the same FIFO, which stalls.
- The spec says that no seam test proves that a stop never waits for a read. Keep that statement true, and do not add a claim the seam cannot prove.
- `task check` and `task test` pass.

## Notes

- Decide how the Rhai module resolver reaches the reader thread. Rhai calls its resolver on the worker, so the resolver is one more host call.
- The 1.x helper `readChoice` becomes a few lines of the shipped sample, with `read_text` and `split()`. The example policies leaf writes it.
