# the-parts-of-ravel-k151 — brief

## Goal

Chapter 7 (`docs/book/src/07-the-parts-of-ravel.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
explain the Ravel CLI, why policy runs as a separate TypeScript worker that
Ravel reaps before anything launches, why the runner is its own Rust package,
and how the parts compose. It is the first chapter of Part III.

## Decomposition

One document, one draft session, then the three remaining stages once. The
chapter teaches structure only. The step-by-step path of one session through
the code is chapter 8, and jobs, signals and the terminal are chapter 9.

## Context

- Needs chapters 1–6. Chapter 2 holds the actors, the authority statement and
  the container figure (`process-authority`). Chapter 6 holds the figure for
  the packages, the worker and the SDK in source and in an installation
  (`source-components`, `installed-files`). Chapters 7 and 12 reuse them; art
  decides whether a new figure answers a question that those do not.
- **Title.** The plan's title “How Ravel is built” is in the passive voice with
  a known agent, which STE Rule 3.6 does not allow in descriptive text. The
  planning leaf `ravel-book-second-half-k148` decided on 2026-10-18 that the
  title is **“The parts of Ravel”**, a noun phrase like chapter 2's title, and
  that the file is `07-the-parts-of-ravel.md`. This draft makes the change in
  every place that holds the old title: the plan row in
  `docs/book/contract.md` (and its note on chapter 7), the draft entry in
  `docs/book/src/SUMMARY.md`, the title table in `docs/book/ste.md`, and the
  route table in chapter 1, which names “How Ravel is built”. Find any other
  place with a search that you check against a known hit.
- **Sources** (verify each claim against the code, not the guide):
  `docs/components.md` (“Responsibilities before packages”, “Languages,
  packages and runtime participants”, “Why TypeScript exists”, “Why there are
  two Rust packages”, “Composition and coordination” and its relation table,
  “Boundary questions and retained costs”), `docs/local-runner.md` (“Boundary”),
  `docs/specs/session-execution.md` (“Installed boundary”, “Policy authority
  and runtime discovery”), the three decision records in `docs/adr/`, the two
  Cargo packages (`crates/ravel`, `crates/keyed-launch`), the worker under
  `crates/ravel/worker/`, and `CONTEXT.md`. The guides use words that the book
  does not use (“front”, “admission”, “publication”, “settle”); write in the
  book's words and take each concept from the code.
- **Say what Ravel is made of, in the reader's order.** The Ravel CLI
  (`crates/ravel`), the policy worker (TypeScript, compiled with Bun, started by
  the Ravel CLI and reaped before launch), the SDK that the owner's policy
  imports, the local runner (`crates/keyed-launch`, the second Rust package) and
  the result channel. Give each part its responsibility, its interface and
  what it does not do. Explain the reason for each boundary in terms of a
  change a developer might make: what can change in one part without the
  others. Give the retained costs honestly (what the split costs).
- **The loose contract.** State the contract between the Ravel CLI and the
  configuration in agreement with chapter 1 (the owner's finding R15): the
  policy selects a program, Ravel runs it, and a program in the session sends
  one value with `end-session`. Show where the code keeps it loose, and
  verify that it does.
- **Inventory 4 (component obligations).** The chapter owns O1 (existing owner
  configuration keeps working), O2 (policy has no session-result authority and
  its worker is reaped before the managed launch) and O10 (installed execution
  uses the matched Ravel CLI and worker). O3–O9 belong to chapters 8 and 9; this
  chapter may name them and link. Enumerate the ten obligations from
  `docs/specs/session-execution.md` and `docs/components.md` again at drafting
  and record where each one is taught.
- The relation table (R1–R8) in `docs/components.md` is the guide's evidence
  that these parts compose. Restate its meaning in plain words where the reader
  needs it, or place a row in chapter 8 where it is a step of a session; record
  the destination of each row in the log so that the migration leaf can check it.
- No example is a complete example unless it runs under `task docs:examples`.
  Directory listings and the Cargo manifests are fragments with a label that
  says what they omit.

## Done when

- Chapter 7 exists, builds, is linked from `SUMMARY.md` with the new title, and
  has passed draft, copy-edit, art and proof.
- Each claim about a package, a boundary or an obligation was verified against
  the source at draft and again at proof, and the chapter says what it did not
  verify.
- The chapter says plainly why the worker is TypeScript, why it is a separate
  process, why Ravel reaps it before the launch, and why the runner is its own
  package, each in a form that a reader can quote.
- A reader who arrives from the route “Changing Ravel” or “Reusing a
  component” finds the local context that this chapter needs, and links back to
  the chapter that teaches each term.
- Every sentence of `docs/components.md` and `docs/local-runner.md` that this
  chapter replaces is listed in the log with its destination, so the migration
  leaf can verify it.

## Notes

- The chapter does not claim that a second implementation or an extracted
  component exists. Chapter 10 is not written yet. Say where a boundary would
  matter to a component consumer and name chapter 10 in plain text.
- Each stage records in its own log the guide sentences it covers. The stages
  after it clear the handoffs that they close, as in the first half.

Art k168 keeps the shared figures as links. The chapter's eight tables carry
its comparisons and mappings, with an adjacent takeaway for each. The task
records each declined diagram as an editorial judgement. No separate figure
source, SVG or layout record changes. Proof reads those tables and their
takeaways as direct entries, as well as the whole chapter.
