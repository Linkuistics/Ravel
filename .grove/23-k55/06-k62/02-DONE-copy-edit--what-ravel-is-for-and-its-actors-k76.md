# what-ravel-is-for-and-its-actors-k76

## Goal

Copy-edit the whole opening unit, `docs/book/src/01-what-ravel-is-for.md` and
`docs/book/src/02-actors-and-words.md`, under section 2 of
`docs/book/contract.md`. Preserve the agreed structure and verified behaviour;
wording, terminology and consistency are this stage's work.

## Context

- The document node's brief names the structure/prose contract and carries the
  figures for art and link checks for proof. `CONTEXT.md` includes the terms
  added by this draft. The parent brief carries the guide-fold-in distinctions
  and future chapter-link conversion so those survive this unit.
- The draft's technical read is recorded in
  `01-DONE-draft--what-ravel-is-for-and-its-actors-k71.md`. Preserve the distinction
  between the unchanged prompt delivered to policy and the arguments selected
  for the program, and between no Ravel store and trusted policy's ability to
  access files or network services.
- Read especially for clear first-use terminology, the authority table and
  the separation of publication from successful completion. Canonical term
  definitions have linked anchors, with short explanations in chapter 1 so it
  stands alone. No teaching gap is knowingly handed to copy-edit; if wording
  reveals missing teaching, route it under the editorial correction rule.
- Chapter 1 states the owner's stand-in reader evidence, unwritten chapter 10,
  limited platform qualification and uninspected remote rendering/publication.
  Keep those evidence limits distinct from claims about product behaviour.

## Done when

- Both files have been read and edited under the prose contract without taking
  the art or draft charter; any out-of-charter defect is routed by the family
  procedure.
- `task docs:book:build` and `task docs:examples` pass. This unit contains no
  code blocks: the draft's example scan reported zero examples, fragments and
  page errors, not executed behavioural evidence.
- As the last work act, add the `art` leaf under this document node with the
  same bare slug, unless a live later sibling already holds that stage.

## Notes

The draft built and served the book locally and checked generated chapter links
against heading IDs. Browser visual proof, the two figures and reader testing
remain their scheduled stages' work. No in-session draft reviewer was used.

## Decisions (running log)

- Read under contract §2 and `CONTEXT.md`; no named external precedent beyond
  the house-style points the contract restates. Headings stay as drafted: the
  section plan fixes them and chapter 1 links to chapter 2's heading anchors.
- One spelling per term: "managed program" and "managed job" become "managed
  process"; "the front" becomes "the native front"; "declines" becomes
  "refuses". "Job" stays only as the cleanup scope (the process group).
- A term is linked at its first use on a page; a bold definition inside the
  section a first-use link points to is plain bold, not a link to itself.
- Figures of speech replaced by mechanisms: "escapes" (group cleanup),
  "uncatchable death" (killed by SIGKILL), "crosses"/"hide", "takes
  responsibility", "own execution environment" (the worker's private
  directory).
- Uncheckable claims made checkable, each verified against
  `docs/specs/session-execution.md` or source: a kind is checked only for
  being nonempty; required cleanup is removal of the session's temporary files
  (worker: its private directory); a stale session-result context is one left
  from a session that has ended (`completion_tests.rs`); measured host reads
  record canonical name, byte count and SHA-256; `--config` resolves against
  the caller's working directory; the measured compilers are Rust 1.99 on macOS
  and the declared minimum 1.85.0 on Linux (`Cargo.toml`, completion spec).
- The one lowercase normative "must" becomes an indicative statement.

## Copy-edit validation

- Both chapters read whole and edited for wording only: no heading, section
  order, teaching step or behavioural claim was added or removed. Each sharpened
  claim was checked against the session specification or source (see the log).
  No structural or teaching defect was found, so nothing is sent back to draft;
  the node brief's art and proof hand-forwards stand unchanged.
- Sweep of both chapters for "managed job/program", bare "the front",
  "declin", normative must/MUST/SHOULD/MAY, "uncatch", "escape" and
  self-linked bold definitions is empty; the same pattern, run against the
  pre-edit chapter 2 from `HEAD`, found each class (positive control). An
  enumeration of every -ize/-yze/-or word finds only neutral spellings. Every
  remaining bare "context" is qualified by its sentence.
- `task docs:book:build` exits 0 with no warnings. `task docs:examples` exits 0:
  0 passed, 0 failed, 0 skipped, 0 fragments, 0 page errors (a page scan; the
  unit has no code blocks). Every `02-actors-and-words.html#…` link in built
  chapter 1, including the new `#trust-and-confinement`, matches exactly one
  heading id; a misspelt anchor matches none.
- No in-session reviewer was used. `CONTEXT.md` and the ADR set are unchanged.
