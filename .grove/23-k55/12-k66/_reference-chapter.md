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

Both planned drafting parts are complete. Part A supplies the CLI, statuses,
streams/notices and environment entries; its inventory is in
`reference-chapter-k75`. Part B fills all eight configuration/layout headings
and removes the draft placeholders; its inventory is in
`reference-chapter-k94`: three settings, six bounds, policy/request/selection/
refusal/context and nested shapes, four host members, 19 SDK exports, six
static-example exports, 49 diagnostic codes and eleven archive paths (nine
installed pair/assets plus README/LICENSE).

Neither draft found a discrepancy in the overlapping chapter 4/5 teaching.
Preserve settings validation before overrides, caller versus delivered policy
context, JSON notice emission before final settlement, conditional
operationReports, signal-vs-status distinctions, original-error/cancellation
precedence and different worker/managed environments. The message_too_large
entry includes oversized loader failure reports at the context stage. The
sample's careful choice sets exactly high, including for xhigh static routes.

Part B's whole-book checks pass: 13 complete examples, seven existing figures,
warning-free build, and all 276 chapter links resolved against built HTML.
The 3,442 source/document/script/manifest subjects remained unchanged through
validation. This establishes no comprehension, terminal or real-agent
operation, native x86-64 qualification, or remote viewing. Proof re-enumerates
against the subject. Copy-edit (`reference-chapter-k95`) is complete: its
spellings and checked wording are in its running log, and it found no
structural or technical defect. Art (`reference-chapter-k96`) is complete;
proof is complete, with its final boundary recorded below.

## Art boundary

The installed-layout entry retains its lookup table and now carries
`source-components` (C4 component) and `installed-files` (UML deployment), each
with editable PlantUML, SVG, local notation teaching, a reader takeaway, alt
text and a full-size link. Chapters 7 and 12 can reuse them. The runner stays
linked inside the native front, the worker is separate, and embedded modules
remain distinct from installed editable sources and declarations. No product,
architecture, extraction or renderer decision changed.

The sample's default/careful comparison is now a page-native table, retaining
exactly high even for xhigh routes and refusal on missing routes. Existing
stream and import tables have explicit takeaways. Art read the whole chapter;
it found no defect requiring an earlier-stage correction run.

All nine figures match, thirteen complete examples pass (zero failed/skipped;
six labelled fragments not run), and the book builds without warnings.
All 289 chapter link/image destinations resolve against built HTML; both
figures retain alt text in inline and zoom copies. Missing-file, missing-anchor,
missing-alt and missing-image controls reject their bad inputs. The 173
enumerated crate/script/book/root-manifest subjects stayed unchanged across
validation. These are bounded source and mechanical checks, not an inventory
re-enumeration or human-comprehension result. Proof re-enumerates the subject.
`docs/book/figures.md` records local Chrome/raw viewing and its uninspected
surfaces; the sample-choice table was also inspected after the final build.

## Proof boundary

Chapter 6 has completed all four editorial stages. Proof reference-chapter-k97
read the whole chapter and re-enumerated both inventories from the current
CLI/source/specification, including 49 diagnostic codes, 19 SDK exports and
all eleven archive paths (nine installed files). No missing item, conflicting
chapter 4/5 statement or chapter correction was found. No handed-forward
defect or correction run remains. The chapter is linked from SUMMARY and
every entry has its teaching destination; this document node's Done when holds.

The nine-figure comparison, thirteen complete examples and warning-free book
build pass; all 289 chapter link/image destinations resolve. All four inline
and zoom img elements have meaningful alt text. The 178 enumerated validation
subjects stay unchanged. The proof leaf records counts, source boundaries,
controls and the two corrected ad hoc assertions. docs/book/figures.md records
proof's own Chrome/raw viewing and unavailable editor-preview/remote surfaces.
These checks establish no human comprehension, real-agent behaviour, terminal
interaction or native x86-64 qualification. No architectural decision changed.

## Read 1 correction run (2026-10-17)

The owner's read 1 (`book-read-one-k70`) made ASD-STE100 the prose base for
all documentation, chose “send” to replace “publish” for the result, and
changed six chapter titles to commands. The owner did not read this
chapter yet, so these changes come from the contract, not from findings.
Copy-edit k129 converts chapter 6 to STE, with the word decisions in
`docs/book/ste.md` (from k123).
Set art `book-first-half-k130` and set proof `book-first-half-k132` serve
this document. Thus, no stage under this node cuts an art or proof leaf.
Copy-edit k129 is complete: chapter 6 passes the strict STE check and uses
“send” for the result. Its running log has the word decisions and the
review classification. The art handoff for set art k130 is in the parent
brief.

## Read 1 correction run (2026-10-18)

The owner's full read 1 (`book-read-one-k70`) found R10–R17. Three reach this
document: the race of a second `end-session` (R16), the installation in
chapter 3 that the entries now describe (R17), and the run of 2,200 words
of command-line entries, which is longer than the new limit (R11). Draft
k142 and copy-edit k143 run under this node. They are the first draft re-run
and the second copy-edit re-run. The owner authorised both on 2026-10-18.
Set art `book-first-half-k144` (the crisp figure style, R10) and set proof
`book-first-half-k147` serve this document. Thus, no stage under this node
cuts an art or proof leaf. Draft k142 writes each figure need in this brief,
under a new `## Handed forward` section.

## Draft correction boundary (k142)

Chapter 6 now states that a second `end-session` races shutdown. It keeps
the first result even if a second sender succeeds. A second sender can
also fail or stop during shutdown. Success of the send is separate from
successful cleanup and return to the caller.

The chapter describes the version named in chapter 3, installed with
Homebrew. All seven installation links use
`#install-ravel-and-check-its-version`. `PREFIX` is the Ravel installation
root that `brew --prefix ravel` gives. The contributor install task stays
only as a note in the installed-layout entry. All eleven archive paths
retain entries.

Nine page tables break the command-line prose. All 50 entry headings stay
unchanged, as do the status, stream, environment, configuration and sample
sections. The strict prose and STE checks, the warning-free book build and
all thirteen complete examples pass. All 341 rendered links and images
resolve. The draft leaf records the source inventory and controls. No
architecture or product behaviour changes.

## Copy-edit correction boundary (k143)

Chapter 6 says that Ravel writes the sent result unchanged and does not
interpret it (R12), in the `start-session` entry, status 0 and the stdout
row. Cancellation, a failure of required cleanup, statuses 129, 130 and
143, `session_cleanup_failed` and `session_cancelled` return no result to
the caller (R13). The race entry gives the mechanism, as chapter 5 does,
and links “harness” to chapter 2. An input now “gives status 2”, and only
a command “exits”. The two table summaries give information. All 50
headings stay unchanged. The strict STE and prose-break checks, the
warning-free build and all thirteen complete examples pass. The leaf
records the words and their sources. This closes document node
`reference-chapter-k66` again. Set art k144 and set proof k147 complete
its second correction run.

## Handed forward

- **Set art k144:** no new figure is needed in chapter 6. The nine new page
  tables answer lookup questions about help/version, prompt sources, task
  fields, parameters, timeout values, grants, confined access, channel
  parents and result inputs. Keep the questions and positions of
  `source-components` and `installed-files`; the Homebrew formula retains
  the same paths below `PREFIX`. Apply the crisp style under the set leaf.
