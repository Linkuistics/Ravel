# reference-chapter-k66 — brief

## Goal

Chapter 6 (`docs/book/src/06-reference.md`), taken through draft → copy-edit →
art → proof under `docs/book/contract.md`. After it the reader can look up any
CLI input, setting, bound, request, selection or context shape, environment
variable or installed path. It is the book's one lookup chapter, and its
entries are self-contained like every other page.

## Decomposition

One document drafted in two sessions along the contract's two inventories,
then the three remaining stages once:

- **Part A, the CLI (inventory 1)**: every verb, option and argument of
  `ravel --help`, `start-session --help` and `end-session --help` and of the
  argument definitions in `crates/ravel/src/cli.rs`; every exit status
  including the signal statuses; the stdout and stderr contract and the
  `--json` notices; every environment variable Ravel reads, sets, passes on
  or refuses to grant.
- **Part B, configuration (inventory 2) and installed paths**: the policy
  entry and how it is found and admitted; every settings key and the refusal
  of unknown keys; every bound; the policy object's fields; the request,
  selection and refusal fields; the policy-context document's fields and the
  host operations; the SDK's exported declarations and the embedded imports;
  the module-resolution rules; the shipped sample; and the installed layout
  (`bin/ravel`, `libexec/ravel/ravel-policy`, SDK, examples, notices).

## Context

- Needs chapters 1–5 proved first: each entry links back to where its subject
  is taught, with an anchor, and carries enough local context to be read alone.
- Coverage is enumerated from the subject at drafting time, not from the
  contract or from this brief: for part A run `ravel --help` and each verb's
  help, read `crates/ravel/src/cli.rs`, `environment.rs`, `run.rs` and the
  exit constants; for part B read `crates/ravel/src/settings.rs`, `limits.rs`,
  `context.rs`, `crates/ravel/worker/sdk/index.ts`, `examples/static.ts` and
  the spec. Report what is missing, and report any entry whose chapter 4 or 5
  statement disagrees with it.
- Style: entries are front-loaded and self-contained; a note, example or
  summary beside an entry is a blockquote opening with its label
  (`> **Note:** …`).
- The "packages, worker and SDK in source and in an installation" figure (C4
  component and UML deployment) first appears here; chapters 7 and 12 reuse it.
  Draw it so it can be reused.
- `--policy-env` has its description from `policy-env-help-k61`; use it.

## Done when

- Chapter 6 exists, is linked from `SUMMARY.md` and has passed all four stages.
- Every item of inventories 1 and 2 has an entry, and each entry names where
  the book teaches it; the chapter's own enumeration, as run, is recorded in
  this unit's log with what each count was taken from.
- The unknowns and omissions the contract lists are stated where they matter,
  including the real agents' flags known only as of their recorded check.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit, the same way
  chapter 4 does.

## Current draft boundary

Part A supplies chapter 6's command line, statuses, streams/notices and
environment entries and links it from SUMMARY. Its subject-derived inventory
and validation are in `reference-chapter-k75`. The configuration draft fills
the eight reserved headings already in the chapter: Policy entry and settings;
Bounds; Policy and request; Selection and refusal; Policy context and host
operations; SDK and module resolution; Shipped sample; Installed layout.
It removes the placeholders and the opening incomplete-draft statement.

Part A found no discrepancy in the overlapping teaching in chapters 4/5.
Preserve settings validation before overrides, JSON notice emission before
final settlement, conditional operationReports, signal-vs-status distinctions,
and the different worker/managed environment rules. Its 13 complete examples,
seven existing figures and warning-free book build pass on unchanged subjects;
this does not establish comprehension, terminal or real-agent operation, native
x86-64 qualification, or remote viewing. Part B and proof re-enumerate their
own scope against the subject. Art still owes the reusable package/deployment
views named above.
