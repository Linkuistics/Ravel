# a-complete-session-k83


## Goal

Add chapter 3's two contract figures to
`docs/book/src/03-a-complete-session.md` under `docs/book/contract.md` and
`docs/book/figures.md`: the successful-session UML sequence and the outcome
UML state machine, each with notation teaching, caption and text alternative,
in the opening unit's visual vocabulary.

## Context

The document brief's `## Handed forward` carries both figure obligations,
their placement and what each must distinguish; it is the specification for
this stage. Copy-edit (`a-complete-session-k82`) changed no structure and no
example, but fixed vocabulary the figures should label with the same words:

- the job: the managed process and the members of its group, defined in its
  job-control sense at the process-group paragraph;
- “the managed process's group”, never “the managed group”;
- lending the terminal: making the managed process's group the terminal's
  foreground process group, done only when Ravel's own group is the
  foreground group, then taken back and the saved modes restored;
- the worker starts in its empty private directory and changes to `/` before
  importing policy; the front removes that directory after confirmed reap,
  before resolving the selected executable;
- settlement removes the result channel's temporary directory; reap comes
  after the final group signals because the group ID is the managed process's
  PID, which is not reused before reap;
- cancellation is by INT, TERM or HUP, in selection or supervision.

The opening unit's figures (`system-context`, `process-authority`) fix the
colour and label conventions; reuse them. Chapter 8 owns the full code trace
and chapter 9 the timing and terminal figures, so keep the sequence
orientation-sized.

## Done when

- Both figures exist as editable sources beside committed SVGs, are placed
  and explained in the chapter, and `task docs:figures`, `task
  docs:book:build` and `task docs:examples` pass.
- The node brief's two art entries under `## Handed forward` are cleared.
- Cut the `proof` successor with the document's bare stem, unless a live
  later sibling already holds that stage, then retire and commit this leaf.

## Notes

## Decisions (running log)

- Reuse the opening unit's blue Ravel, grey external-participant and gold
  temporary-channel vocabulary. Add UML sequence and state conventions beside
  their first use; keep the runner inside the native-front lifeline.
- The sequence follows this chapter's successful unconfined example, with
  grouped messages rather than a code trace. The outcome machine joins both
  phases at cleanup and settlement, then separates the final output write from
  success. Cleanup attempts on a failure path do not imply confirmed cleanup.
- Use the contract's existing design and figure tooling. No new renderer,
  media choice or architecture decision is needed. Detailed escalation timing
  and terminal handoff figures are declined as an editorial judgement: chapter
  9 owns those questions; the medium can carry them.
- Source verification puts the worker's change to `/` before its identity
  greeting. Created participants now begin their sequence lifelines at
  creation; the worker ends before the managed process begins. State-machine
  guards name the primary settled outcome, so a secondary cleanup problem
  does not create a second eligible outcome.
- Native Chrome inspection found the first state layout too small at chapter
  width. Moving repeated notes into its key and wrapping outcome guards made
  the final view legible there. Both final figures were inspected on Auto's
  light page and Navy; `docs/book/figures.md` records viewing limits and the
  failed guest screenshot route. No in-session reviewer was used.

## Verification

- Tier 2 source check in graph project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-16T03:56:41Z`: exact snippets for
  `choice::evaluate_and_resolve`, `worker::evaluate`, `run::run`, runner
  `watch` and `supervise_reported`; the front's outbound depth-one trace
  returned all 43 reported callees without truncation. Coverage reported no
  recorded issue and matching metadata for these Rust paths. Direct source
  read of `crates/ravel/worker/src/main.ts` establishes change-to-`/` before
  the identity greeting; the completion spec's outcome table and final-write
  paragraph ground the state view. This is bounded source evidence, not a
  complete-codebase or new runtime claim.
- Figure sources have untracked freshness in the graph; SVGs are deliberately
  excluded. Both were checked directly through editable source, regeneration
  comparison and browser rendering rather than treating graph coverage as
  completeness.
- Final `task docs:figures`: all four book SVGs match their sources.
  `task docs:book:build`: clean build. `task docs:examples`: chapter 3's
  example passes, zero failures or page errors, one labelled installation
  fragment not run. The existing opening-unit SVGs are unchanged.
- The final measurement digested 160 files enumerated under `docs/book`,
  `scripts`, `crates`, `.cargo` and the root Cargo manifests/Taskfile, plus
  12 checkout-local tool/font files under `target/tools`, before and after;
  both per-file manifests compare byte for byte. No subject was edited while
  that measurement ran. The manifests are scratch evidence in
  `/tmp/ravel-k83-{subjects,tools}-{before,after}.sha256`.
- Both art handoffs are fulfilled and removed from the document brief.
  Proof remains required, so neither this document node nor a parent closes.
  No architectural decision changed; the ADR set needs no edit.
