# rhai-measured-reads-k52

**Reviews:** rhai-measured-reads-k12

## Goal

Read the bridge between the Rhai worker and the Ravel CLI as a hostile script and a failing machine would. Find the sequence, the race or the bound that the tests of `rhai-measured-reads-k12` do not reach. Every later host function (data helpers, discovery, LLM calls, harness runs) uses this bridge, so a flaw here repeats in each of them.

## Context

- Read the commit of `rhai-measured-reads-k12` and its running log.
- Read these spec sections: "The bridge between the script and the CLI", "Measured reads", "Policy entry and discovery", "Request, context and selection" and "Cancellation and deadlines".
- The front side is in `crates/ravel/src/rhai_worker/`.
  - `serve.rs` holds the Tokio current-thread runtime and its stops.
  - `host.rs` holds the ledger, the reads, the imports and the entry read.
  - `reader.rs` holds the reader thread and the file rules.
  - `front.rs` holds the handshake and the hand-over to the runtime.
- The worker side is in the same directory.
  - `bridge.rs` holds the blocking call and the ends that `try` cannot catch.
  - `resolver.rs` holds the imports.
  - `role.rs` holds the thread that reads frames from the front.
  - `engine.rs` holds the phases, `load_context` and the failure mapping.
- Shared refusal text moved behind a `Language` in `context.rs` and `policy.rs`.
- `choice.rs` maps the new failure codes in `script_refusal`.
- The tests are `crates/ravel/tests/rhai_reads.rs`, the unit tests in `serve.rs` and `reader.rs`, and the changed engine and front unit tests.

## Done when

- Each question below has an answer with evidence, or a finding.
- Questions to answer:
  - Is the stop order of the spec true in the code?
    - A stop must drop the call in flight and abandon a read. It must leave no task.
    - Check that no path waits for the reader thread.
    - Check that the runtime and the copy of the socket are gone before `supervise` stops the worker.
    - Check the case where the deadline, a signal and the output bound arrive together.
  - Can the worker and the front deadlock or lose a frame?
    - The worker reads frames in a thread and waits on a channel.
    - Check a reply that arrives after the worker stopped.
    - Check a front that closes during a write.
    - Check a reply of 17 MiB against a worker that is still writing.
  - Does every end stay uncatchable?
    - `bridge.end` raises `ErrorTerminated`. The engine reads the record after the script returns.
    - Look for a path where the script swallows the error and nobody reads the record.
    - One example is an error that Rhai wraps in a way `pointer_calls` does not unwrap. Another is an `import` inside a function.
  - Do the file rules hold?
    - Check the nonblocking open for a socket, a device and a symlink loop.
    - Check the byte bound at exactly the bound and at one byte over.
    - Check that the front reads each module once.
    - Check that the module count and the cycle check agree when two files import one module.
  - Is the ledger right?
    - Check the count of 256 sources against the caller's document.
    - Check it against a read that failed on UTF-8 or JSON, and against a read that the script retries.
    - Check that the measured list of a loader holds only the reads of the `context` phase.
  - Does the move of the entry read behind the worker launch change anything a caller can see?
    - Compare the refusal, the exit status and the order of events.
    - Compare the old text of `authority.rs` with the new `RhaiEntry`.
  - Does any new test have a control that cannot fail?
    - The producer saw two controls fail on a mutation.
    - Name any test that would pass on a broken bridge.

## Notes

- The producer ran the Rhai tests on Linux arm64 in the Docker image. 140 of 141 passed.
- The one failure needs the Bun worker, which the ad-hoc run did not build.
- Nobody ran the full `native-linux` campaign.
- The spec says that no test at the seam proves that a stop never waits for a read. The unit test `a_deadline_does_not_wait_for_the_call_the_reader_is_in` proves the mechanism only. Say if the spec still states this correctly.
- The notice lists the entry as the first element of `policy.modules`, and the spec says so. Say if another reading of "the entry and every module" fits the spec better.

## Decisions (running log)

- Review method: inspect commit `05208e3d9dcd7fff7b18e0d60bf6fbde493bc4e3` against its parent and read the current source and recorded verification. No test, build, lint or format command runs in this inspection-only leaf. The graph CLI refused `list_projects` because a pre-coordination or unverified generation is active; direct source reads replace graph evidence, and no graph completeness claim is made.
- Source accounting is an actionable finding: `Host::read` counts only entries in `reads`, and `fetch` returns decoding failures without a measurement. Repeated reads of invalid UTF-8 or JSON consume no source budget, despite the specification counting every read.
- Import accounting is a second actionable finding: the host counts canonical files, but the Rhai engine also caps import statements at 64. A cached import increments Rhai's counter too. The existing flat 64/65 test cannot isolate the host's enforcement.
- JSON reply nesting is a third actionable finding: `fetch` accepts a value with the JSON parser's full nesting allowance, then adds a reply object. The worker parses that envelope with the same allowance and can reject a reply generated by its own front. The reverse converter also does not apply the documented 100-level boundary.
- Reader signal eligibility is the highest-priority finding: `Reader::start` spawns without the checked mask inheritance used by diagnostic readers. An abandoned read can leave this eligible thread alive through the main thread's blocked finalisation and process-wide disposition changes. This violates the existing native handoff contract even though the read is never joined.
- Recorded all question dispositions and cut adjacent integration `rhai-measured-reads-k53` for independent triage. The PATH shim selected Grove 18.2.0 and its insertion failed without tree changes; the mandated `/opt/homebrew/Cellar/grove/22.6.0/bin/grove-llm` performed the insertion. Later host-capability leaves remain live, so no parent closes in this session.

## Findings

### F1 — P1: The abandoned reader can consume signals during finalisation

**Location:** `crates/ravel/src/rhai_worker/reader.rs:34–39`.

`Reader::start` creates its thread without blocking the handled signals on the
creator. It runs after `Handlers::begin` restored the calling thread's previous
mask (`run.rs:49–51`, `front.rs:193`). For an ordinary caller with INT, TERM and
HUP unblocked, the new thread inherits eligibility for all three. Nothing in
the reader changes that mask.

This matters because abandoning a read deliberately allows the thread to
outlive evaluation. A stalled read can leave it alive throughout worker cleanup
and common finalisation. `Handlers::finalise` blocks only the calling thread
before taking the last signal snapshot and restoring the process-wide
dispositions (`cancellation.rs:291–398`). The reader remains eligible: a signal
can run a handler outside the thread whose mask establishes that snapshot, or
take a restored default action while the CLI is still writing its reports.
Keeping an unknown pending state blocked on the main thread does not protect
against delivery to this reader. Required refusal and secondary cleanup reports
can be lost.

The existing contract explicitly requires auxiliary threads to be ineligible
during handoff and readers to remain shielded through disposition release
(`docs/specs/result-job-completion.md:268–273`, `:302–306`, `:318–325`). The
diagnostic-reader implementation establishes checked mask inheritance before
creation and restores only the creator (`worker.rs:1491–1569`;
`cancellation::Shield`). The new reader bypasses that mechanism.

Integration should triage the reader's mask ownership, including failed
creation, failed creator restoration and a read that remains alive after a
stop. Existing native receipts observe the two diagnostic readers, not this
third thread; their green results do not establish its shielding. Do not repair
this by joining a stalled reader.

### F2 — P2: Failed decodes escape the source budget and measured-read ledger

**Location:** `crates/ravel/src/rhai_worker/host.rs:209–244`.

The source count is `reads.len()` plus the caller's document. `reads` grows only
for `Fetched::Read`. Once a regular file has been read completely and hashed,
invalid UTF-8 and invalid JSON return `Fetched::Data` instead
(`host.rs:406–427`). Those reads consume no budget and leave no record, although
their bytes have already been read.

A policy can catch `read_json("bad.json")` in a loop 257 times, then select a
harness. With `bad.json` containing `{ nope`, each operation reaches the
filesystem, but the count stays zero and the notice's `reads` stays empty. A
caller document changes the count to one without closing the escape. The same
path exists for a non-UTF-8 file and for repeated retries of that file. By
contrast, retrying a successful read correctly consumes another source.

The specification counts the caller document and every read in the evaluation
and requires every read to be measured (`policy-evaluation.md`, Measured reads
and Measured context and neutral discovery). This finding concerns completed
byte reads whose decoding fails; it does not assume a missing file has bytes
that can be measured. The deadline still bounds the loop, so this is a source
budget and provenance failure, not a deadline escape.

The current data-failure test explicitly expects an empty ledger after reading
invalid UTF-8 and JSON (`tests/rhai_reads.rs:110–154`); that assertion endorses
the gap. Integration should settle accounting separately from whether content
decoding succeeds, and verify the boundary with caught decode failures, retries
and a caller document.

### F3 — P2: Cached imports still exhaust a separate import-statement limit

**Location:** `crates/ravel/src/rhai_worker/engine.rs:188`.

The host counts distinct canonical module files, excluding the entry from its
64 imported-module allowance. The resolver caches evaluated modules by that
name. But `set_max_modules(MAX_MODULES)` additionally enables Rhai's counter of
executed import statements. The pinned Rhai source checks
`global.num_modules_loaded` before resolving, then increments it even when the
resolver returns a cached module (`rhai-1.27.0/src/eval/stmt.rs:837–899`).

For example, the following top-level code with an empty `helper.rhai` loads
only one distinct imported module, yet the 65th iteration refuses:

```rhai
for i in 0..65 {
    import "helper" as helper;
}
```

The host never receives that last import. Its canonical-file count and cache
cannot prevent the premature failure. In a selection function the same
`ErrorTooManyModules` also maps through `selection_failure` to
`selection_threw`, rather than the specified `policy_import_failed`.
`import_failure` already describes the extra statement limit, but the
specification defines a 65th module, not a 65th use of a cached module.

The flat test `the_64th_module_loads_and_the_65th_refuses`
(`tests/rhai_reads.rs:552`) still passes if the host's module-count check is
deleted: Rhai rejects its 65th top-level statement first. That is a concrete
test that passes with broken bridge enforcement. A graph of files with fewer
than 64 direct imports per file is needed to distinguish the host's total
module bound from this engine counter. Repeated cached imports need a separate
successful control.

### F4 — P2: Accepted JSON nesting can break the private reply protocol

**Location:** `crates/ravel/src/rhai_worker/host.rs:418–420`.

`fetch` parses the file with `serde_json::from_str::<Value>`, adds the parsed
value beneath a reply object, and serialises it. It checks encoded length but
not nesting depth. The worker reads this frame through the same JSON parser
(`role.rs:140`, `frame.rs:75–86`). Adding the object consumes another level of
the parser's allowance.

A file containing 127 nested arrays around `0` is only 255 bytes. The raw file
fits the default parser's depth allowance, but its enclosing reply object does
not. The pinned `serde_json` 1.0.151 starts with 128 remaining levels and rejects
when the next container decrements the allowance to zero
(`src/de.rs:63`, `:1372–1385`). Thus the front emits a reply its own worker
cannot decode. `role::watch` drops the parse error, sets the stop flag and sends
`None`; `Bridge::ask` terminates, and the front reports a worker failure instead
of a classified source or boundary failure. A script's catch block cannot
handle that as a data error.

Even away from the parser's boundary, `Bridge::read` calls the unchecked
`convert::from_json` on the reply (`bridge.rs:199`). A value with 101 nested
arrays can enter Rhai, despite the boundary contract allowing at most 100.
Only conversion in the other direction applies that check. The read capability
therefore adds an input path that does not uphold the documented common
conversion rule.

Integration should triage nesting before a reply crosses the socket, preserve
the classified failure and its location, and cover both the documented depth
boundary and the envelope-induced parser failure. Increasing the frame's byte
limit does not address either case.

## Answers to the review questions

- **Stop order and residue.** `Watch::until` drops the work future when a stop
  wins (`serve.rs:90–99`). Dropping the oneshot abandons the read, and dropping
  `Host` drops the job sender. `Reader` retains no join handle or runtime
  blocking task (`reader.rs:28–55`). The worker call is scoped inside
  `runtime.block_on`; the socket copy and host drop with that future, then
  `front.rs:218` drops the runtime. Only after `converse` returns does
  `worker.rs:624–632` drop the original socket and stop the worker. No spawned
  Tokio task appears in this bridge. This supports the required non-waiting
  order, but F1 prevents claiming the remaining reader is safe during signal
  transitions. The seam disclaimer remains correct: the deadline unit test
  proves the mechanism with a sleeping closure, not a kernel-held read.
- **Competing stops.** The watch checks output before cancellation, and its
  timer can win before the next poll. These are provisional stop causes.
  Supervision checks captured output after teardown, retains an earlier
  conversation failure, then reads cancellation again
  (`worker.rs:667–719`). A consumed signal wins over output and timeout; an
  earlier timeout remains in the nested original report when it was the
  initiating conversation failure. This ordering is visible in source, not a
  newly executed race test. F1 undermines the premise that all later signal
  observations occur on the protected main thread.
- **Framing and deadlock.** Worker writes and the demultiplexing reader use
  separate socket copies. While evaluation waits for a reply, the demux drains
  it independently (`role.rs:82–95`, `:133–149`; `bridge.rs:101–110`). A reply
  up to 17 MiB therefore does not require the writer to read; the synchronous
  evaluator finishes writing its request before waiting. Front writes are
  asynchronous and watched for stops. If the front closes during a worker
  write, the write errors and `ask` sets the stop flag; a blocked native call
  is ended by worker supervision. A reply arriving after the evaluator has
  stopped makes the demux send fail and end, or is discarded when the worker
  is reaped. During a host call, a worker byte or EOF is selected before its
  reply and ends the evaluation (`serve.rs:252–292`). No lost-frame or circular
  write dependency was established on this path. F4 is a separate framing
  failure for replies produced by a valid capability.
- **Uncatchable ends.** Front-detected source and import failures return
  `Answer::End` and send no catchable error reply. Worker-detected invalid
  requests, cycles and module parse failures record the first failure in
  `Bridge::end`. `evaluate_with` reads that record after `run`, even when its
  result is a selection, a loader result, a wrapped error or an import in a
  function (`engine.rs:130–146`). The resolver checks a loading name before
  its loaded cache and routes compile/evaluation failures through that record
  (`resolver.rs:86–128`). No path to an admitted selection after a recorded
  end was established. This statement concerns bridge ends; F3 identifies a
  separate engine-generated import failure with the wrong classification in
  a function.
- **File rules and module reuse.** `read_file` opens with `O_NONBLOCK`, checks
  the descriptor's regular-file status, and uses `take(bound + 1)` before
  checking length (`reader.rs:85–118`). Exact-bound files succeed and one byte
  over fails without truncation. A socket whose open fails is classified by
  metadata as nonregular; an opened device is rejected by the descriptor
  check. A symlink loop returns an unreadable resolution/open failure. Path
  operations remain on the reader thread. The host canonicalises module
  targets and returns no source for known names; the resolver reuses the
  evaluated module, so a diamond importing the same canonical file evaluates
  it once. The loading stack detects cycles before cache reuse. The host's
  `modules.len() > MAX_MODULES` test intentionally allows entry plus 64
  imports; its next new file fails. F3 covers the competing statement count.
- **Ledger and phase.** Caller documents count once and successful repeated
  reads count separately. F2 covers decode failures and retries. A delivered
  loader context includes the caller document followed only by reads marked
  `Phase::Context` (`host.rs:120–136`); the bridge enters context before the
  loader and selection before `select` (`engine.rs:242–250`, `:286`). Module
  top levels run under the phase in which they are first imported, so an
  import inside the loader contributes its reads to that context. Entry and
  modules have their own ledger and do not use the source count.
- **Entry read and caller-visible changes.** The new reader executes after
  worker launch and handshake. Its ordinary read failures still become
  `policy_unreadable`, authority stage, exit 3, naming the originally selected
  path through `RhaiEntry::unreadable`. UTF-8 and oversize wording is retained.
  Open failures now say `cannot be opened` instead of the old `cannot be read`;
  disappearance during the deferred read says `cannot be found`. These are
  text changes. A failed worker launch, a handshake failure, a deadline or
  cancellation can now precede the unreadable-entry refusal, and a malformed
  entry can have worker diagnostics. These ordering changes follow the
  intended move behind launch; the unchanged-code comparison is
  `authority.rs` in the producer's parent against `RhaiEntry`, `Host::read_entry`
  and `front::converse` in the reviewed commit.
- **Controls and verification evidence.** The FIFO control actually attempts
  a blocking open of the same FIFO; the larger-read control actually reads
  the oversize file with a raised bound. The recorded producer mutations
  support those two instruments only. F3 names the flat module-count test
  whose refusal is independently enforced by Rhai, and F2 names the
  data-failure test that asserts the incorrect empty ledger. The reader's
  abandonment test and deadline test do not inspect its signal mask. The
  producer commit reports `task check` and `task test` passing 481 cases;
  the mandated review notes report 140 of 141 Rhai cases passing on Linux
  arm64, with a missing Bun worker accounting for the failure. These are
  reported results, not independent measurements. No full Linux campaign
  or repaired-implementation verification is claimed here. A root
  `Taskfile.yml` exists and already defines the reusable check and test tasks.

## Review boundary

The findings derive from the committed producer, current source, the binding
specifications and local pinned dependency source. No executable probe or
mutation was used. No production or test file changed. The knowledge graph
supplied no project, generation or coverage result because its CLI could not
start. This is bounded source evidence, not a repository-wide completeness or
platform-conformance claim. Integration owns every disposition, correction and
post-correction verification.
