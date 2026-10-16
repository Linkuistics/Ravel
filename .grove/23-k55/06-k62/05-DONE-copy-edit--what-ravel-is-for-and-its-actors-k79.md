# what-ravel-is-for-and-its-actors-k79

## Goal

Copy-edit chapters 1 and 2 as one document under `docs/book/contract.md`, after
draft k78 corrects the worker-directory teaching. Read the whole unit; keep
changes inside the copy-edit charter.

## Context

This is the first correction run, sent back by art k77. Keep the worker's
initial directory, policy-time directory, request's `cwd` and cleanup directory
distinct. The original art now supplies two C4 figures and in-page comparisons;
their names and captions follow `CONTEXT.md` and `docs/book/figures.md`.

Art k80 and proof k81 are already live later siblings. Do not append another
art or proof leaf. The editorial family bounds a second rerun by escalation.

## Done when

- The whole unit meets the prose contract without absorbing another charter.
- `task docs:book:build` and `task docs:examples` pass, and handoffs are current.
- Retire this leaf with the existing art sibling next.

## Decisions (running log)

- Read both chapters whole, with closest attention to text added since k76
  (k77's figure paragraphs and tables, k78's directory paragraphs). No
  structural or technical defect found, so nothing is sent back to draft.
- One term for the worker's directory: k78's "private empty directory" and
  "initial private directory still belongs to the native front" become "the
  worker's private directory", as the admission paragraph already said; the
  ownership claim becomes the checkable "the native front creates … and starts
  the worker there" (`crates/ravel/src/worker.rs`, `evaluate`, private-dir
  creation and `current_dir`; `crates/ravel/worker/src/main.ts`, `chdir("/")`).
- Hedges without conditions replaced by their condition: "normally" and "in
  normal use" become "a completion command that inherits the inner path" and
  "while each process uses the path it inherited".
- Figures of speech replaced: "opening the Ravel box into" becomes what C4's
  container level shows; "a map of participants, not a picture" becomes what
  the figure does and does not show. "the temporary channel" becomes the term
  "result channel".
- "managed process group" (parses as a third term) becomes "the managed
  process's process group" in both chapters; the information table's "runs
  there" becomes "starts there", matching the paragraph above it.
- Glossary aligned with the book and contract: refusal grounds are "behaviour,
  data or admission" (`CONTEXT.md` said "output"); the session specification's
  refusal list includes invalid context and bounds, which "output" omits.
- Smaller sentence repairs: refusal paragraph's repeated "Ravel", the worker's
  "run TypeScript and imports", the policy-worker table row's unclear "that
  code", and source reflow of over-long or broken lines.
- No in-session reviewer spent: changes are wording-level and each technical
  claim touched was checked against source or specification directly.

## Validation

- Sweep of both chapters for the replaced phrasings and stock hedges is clean;
  the same command was seen to find each class in the pre-edit revision, and
  "private" still matches its legitimate result-channel and alt-text uses.
- `task docs:book:build` passed with no warning; `task docs:examples` reported
  zero page errors (this unit has no runnable examples); `task docs:figures`
  reports both figures matching their sources.
- Art k80 and proof k81 already stand as later siblings, so no leaf is cut.
