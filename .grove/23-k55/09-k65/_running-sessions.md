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

## Draft coverage and guide fold-in

Chapter 5 supplies the `session-cli.md` teaching for caller inputs, unchanged
prompts, results/first-publication/nesting, confinement, JSON notices and
`operationReports`, and every normal exit status. Settings, worker pairing
and policy context are introduced locally with links to chapters 3–4; chapter
6 still supplies the reference entries and full inventories. The limits table
absorbs the operational authority limits for the later guide migration. No
guide is removed by this stage.

Six complete examples cover prompt-file bytes and caller data; publication,
empty publication and natural exit; nesting; confinement; selection
cancellation; and cancellation after publication. Two local fragments cover
caller result capture and physical terminal interaction. The automated seam
establishes no real-agent behaviour, terminal interaction or human
comprehension. The proof views and owner's stand-in read remain outstanding;
art's viewing coverage and limits are in `docs/book/figures.md`.
Native x86-64 qualification and remote/published rendering remain
the book contract's declared unknowns.
