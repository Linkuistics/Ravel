# running-sessions-k65 — brief

## Goal

Chapter 5 (`docs/book/src/05-running-sessions.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
run sessions under real conditions (prompts, kinds, caller data, nesting,
confinement, notices) and interpret every outcome and exit code.

## Context

- Needs chapters 1–3 and the scaffold; chapter 4 should be proved first, so
  that settings and grants are taught before they are used. Does not need
  chapter 6, which links back to this chapter.
- The material, each verified against the code: prompt and `--prompt-file`
  passed byte for byte; kind as an open token; caller data (`--param`,
  `--context`, `--task-file`, `--task-id`) as data the policy may use and
  Ravel gives no meaning; nested sessions and the first-result-wins rule;
  confinement (`--confine`, `--exit-dir`, `--runtime-read`; `sandbox-exec` on
  macOS and bubblewrap on Linux; what is refused before policy runs); the
  routing notice and `--json`, including `operationReports`; every exit
  status, with how a caller should read each (the signal statuses for INT, TERM
  and HUP are read from the source, not recalled).
- The distinct, normal outcomes: a result after confirmed cleanup (even if the
  child exits nonzero later); a natural exit without a result, status 0
  included; cancellation during selection and during supervision, including
  after a result was published; a refusal; a worker, supervision or cleanup
  failure; a selection timeout; a program that cannot be found or executed.
  `docs/specs/session-execution.md`, `docs/specs/result-job-completion.md` and
  `crates/ravel/tests/cli.rs` (cancellation, nested, confined, deadline cases)
  are the sources.
- The stated limits, said plainly, with where each is taught more fully:
  same-user tampering, abrupt death of the front, descendants that leave their
  process group, detached policy children, and the support statement (aarch64
  macOS and aarch64 Linux measured; x86_64 Linux built and run only under
  emulation, tracked in Linkuistics/Ravel#1).
- Figures from the contract: two UML sequences for cancellation during
  selection and during supervision; the state machine from chapter 3,
  extended with the outcomes.
- A cancellation by signal is a fragment unless the examples seam can run it
  deterministically (see `book-examples-k60`'s recorded convention).

## Done when

- Chapter 5 exists, is linked from `SUMMARY.md` and has passed all four stages.
- Every exit status Ravel can return has a statement of what it means and
  what the caller does about it, enumerated from the source (not from this
  brief), and the chapter says so where a reader may arrive directly.
- The limits above are each stated in one place a reader can find, with the
  condition under which they apply.

## Notes

- Do not teach the job-control mechanics here beyond what interpreting an
  outcome needs; chapter 9 teaches them.
