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
