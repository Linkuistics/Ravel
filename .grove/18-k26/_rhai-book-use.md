# rhai-book-use-k26 — brief

## Goal

Revise Parts I and II of the Ravel book for Ravel 2.0.0.
The chapters teach Ravel configuration with Rhai directly. The pipeline is draft, copy-edit, art and proof.

## Done when

- Chapter 4 teaches Rhai and owns the recipes of an LLM call through Claude Code and Codex.
  Each recipe gives the arguments, prompt placement, structured-output flags and answer extraction.
  Complete examples use fake harnesses and agree with the recorded real smoke calls.
- Chapters 1–6 describe Rhai configuration scripts, host capabilities, harness entries and budgets.
  They describe an LLM call as a harness run.
- Every complete example runs through the CLI seam, and `task docs:examples` passes.
  Every excerpt is exact, and `task docs:excerpts` passes.
- The prose follows the contract, and `task docs:ste` reports no problem.
  Figures agree with the text, and `task docs:figures` passes.
- Proof reads both parts as one document and returns to the existing Read 1 leaf, `rhai-book-use-k74`.
- After the Read 1 corrections, chapters 1–6 follow the owner's Read 1 patterns in section 2 of the contract.
  The proof of that correction run cuts nothing, and Read 1 resumes.

## Read 1 corrections

- Read 1, `rhai-book-use-k74`, stopped in chapter 1 on 2026-10-27.
  The owner asked for the patterns of eight findings in chapters 1–6 before a re-review.
- Section 2 of `docs/book/contract.md` holds the patterns. Two new rows of “Figure needs” hold the figures that the owner asked for.
- The correction run is draft `rhai-book-use-k75`, copy-edit `rhai-book-use-k76`, art `rhai-book-use-k77` and proof `rhai-book-use-k78`.
  Each stage finds its successor already cut.
- This run is the first correction run against this document. Another send-back against it goes to the owner.
- The draft node `rhai-book-use-k75` closed on 2026-10-28 after four drafts, `rhai-book-use-k80` to `rhai-book-use-k83`.
  Chapters 1–6 now open each section with the reader's goal and explain each term at its first use on the page. Each mechanism has a real example.
  Chapter 6 restates the words of every page, Rhai and chapter 1's real values, because a reader can enter it at any entry.
  The F2, F4 and F5 entries under “Handed forward” name each place for a figure. No draft placed an image.

## Decomposition

- `rhai-book-use-k27` rewrites chapter 4 and records the owner's revised contract.
- `rhai-book-use-k28` contains two drafts: `rhai-book-use-k70` updates chapters 1, 2, 3 and 5, and `rhai-book-use-k71` updates chapter 6.
- Copy-edit, art and proof follow both drafts.

The two drafts share one stem. Refer to each by key.

## Corrections before Part II

The owner completed Part I on 2026-10-28 and then asked to apply all lessons before proceeding to Part II.
This supersedes the earlier choice to collect the remaining notes first. The exact words and findings F9–F16 are in `rhai-book-use-k74`.
The second correction run is explicitly authorised. Another substantial send-back requires escalation to the owner.

The run is draft `rhai-book-use-k84`, copy-edit `rhai-book-use-k85`, art `rhai-book-use-k86` and proof `rhai-book-use-k87`.
Each stage reads the whole of chapters 1–6 under its charter and finds its successor already standing.
No stage adds another editorial tail. Proof cuts no checkpoint, and Read 1 stays live.

Read the full current contract and `docs/documentation-lessons.md`.
Apply all relevant lessons, including F1–F16, throughout both parts. The new quotations are examples of patterns, not a boundary around isolated sentences.
The work includes concrete cases before general explanations, the complete caller task contract, configuration terms and descriptive headings.
It also includes the worker's purpose, JSON descriptions, Grove links and familiar shell commands.

After proof, Read 1 resumes at chapter 4. It gives the owner a correction recap and access to the amended Part I.
The owner need not restart at chapter 1. The practical reader tasks still need the owner's results.
Parts III and IV apply the same current lessons in their scheduled rewrite before Read 2.

## Corrections after the first Part II finding

The owner reached chapter 4's setup section and reported missing context for its files and command table (F17).
They also requested removal of “policy” from filenames and arguments (F18), then asked the session to continue.
Their words and the TestSubject clarifications are in `rhai-book-use-k74`.
This is an owner-directed third correction run, not an unattended repeat of the previous verdict.

The existing TypeScript-removal node now precedes this node, followed by `configuration-names-k92`.
That leaf implements the name map, updates exact examples and preserves working consumers.
This node then owns draft `rhai-book-use-k93`, copy-edit `rhai-book-use-k94`, art `rhai-book-use-k95` and proof `rhai-book-use-k96`.
All four stages are already present. Each reads chapters 1–6 under its charter and cuts no duplicate successor.

Apply the full contract and lessons, including L67. Show example inputs and complete commands before explaining their fragments.
Reconcile all implemented configuration names and remove obsolete explanations for retained old literals.
The removal has supplied the final installed layout, so proof resolves that dependency here.
Parts III and IV still own their whole-book reconciliation and Read 2.

The bounded LLM trial in `rhai-book-use-k74` supplies execution and model evidence, with limits.
Its successful run did not identify the owner's setup complaint and does not settle instructional sufficiency.
After proof, Read 1 resumes at the interrupted setup section. The owner need not repeat Part I, and their practical results remain outstanding.

## Pointers

- Structure brief and prose contract: `docs/book/contract.md`.
  It states the reader, ordered section plan, emphasis and exclusions.
- Mechanics: `docs/book/examples.md`, `docs/book/excerpts.md`, `docs/book/figures.md` and `docs/book/ste.md`.
- Technical sources: `docs/specs/configuration-evaluation.md`, `CONTEXT.md`, the source and the built CLI.
- Example policies and equivalence evidence: `rhai-example-policies-k25`.
- Measured model-call arguments and versions: the running log of `rhai-harness-smoke-k66`.

## Owner decisions

The owner settled these changes in `rhai-book-use-k27` on 2026-10-26:

- The book has no migration chapter or TypeScript prerequisite. Chapter numbers stay unchanged.
- The owner's configuration conversion is a one-off manual task under `owner-policy-conversion-k39`.
- Shell, Git, JSON and Rust remain prerequisites. Chapter 4 teaches the Rhai that its policies need.
- Read 1 and Read 2 remain required under section 4 of the contract.

## Notes

- The book describes Ravel 2.0.0. Cutover and delivery removal have landed, followed by the configuration-name correction.
  Chapter 6 describes the retired-format refusal and the final notice path. The release still supplies the version placeholder.
- Chapters 7–11 remain with `rhai-book-inside` after the removal increment.
  Mechanical link corrections can precede that rewrite.
- Run `task setup` before the book checks. The checkout-local Bun remains a development tool after release.

## Draft closure

- `rhai-book-use-k28` delivered chapters 1, 2, 3, 5 and 6 through its two drafts.
  Chapter 6 is the lookup chapter, with Rhai host calls, harness settings and results, constant bounds and notice schema 2.
  The example-convention note now uses a runnable Rhai sample and explains cancellation through a fake evaluation harness.
- The book assumes no TypeScript knowledge. References to retired extensions and reserved environment names remain explicit interface rules.
  The default-entry refusal and final installed layout still depend on `typescript-worker-cutover-k31` and delivery removal.
- `rhai-book-use-k69` is the existing next copy-edit stage for the whole document.
  No second editorial pipeline belongs inside the closed draft subtree.
- The final draft checks passed: all 19 book examples and the separate convention sample, nine excerpts, 14 figure reproductions and the book build.
  Chapter 6 passes strict STE and prose-break checks. All 317 rendered links to or from it resolve.
  Mechanical figure reproduction does not discharge the art updates above.

## First Read 1 correction closure

Proof `rhai-book-use-k78` read chapters 1–6 after the correction run and
checked the Read 1 patterns throughout. It corrected notice-time summaries,
the final-launch scope of refusals and bounds, a partial-output claim and
repeated inheritance explanations. It also corrected the prose of the three
new figures and regenerated their artifacts. No hand-forward entry remains.

All 21 examples, nine excerpts, seventeen figure reproductions, strict prose
checks and the book build passed. The local link check passed 762 targets,
with a broken-anchor control. Safari in an isolated macOS guest supplied the
local viewing evidence. The proof task records its precise scope and limits.

At that boundary, the node's Done when held. Read 1 stayed live as `rhai-book-use-k74`, ready
for the owner's re-review from chapter 1. No second checkpoint was cut.
The later findings now require the correction run above before the owner proceeds to Part II.

## Second correction draft closure

The draft node `rhai-book-use-k84` closed on 2026-10-28 after four drafts, `rhai-book-use-k88` to `rhai-book-use-k91`.
Chapters 1–6 use the term map of `ste.md`, “Configuration words”. Outside code and link targets, “policy” remains only where a page explains Ravel's literal names, help and diagnostic words.
Each chapter opens with chapter 1's review, and chapter 6 restates the worker's three reasons. JSON version labels stay only in examples and in the entries that teach their values.

The last draft passed all 22 examples, nine excerpts, seventeen figure reproductions and the book build.
Strict prose and break checks passed for chapters 1–6. A local link check of the built book and the README passed, with failing controls.
The figure labels with the old terms remain under “Handed forward” for art.

## Second correction proof closure

Proof `rhai-book-use-k87` read chapters 1–6 against the current contract, F1–F16 and all accumulated documentation lessons.
It corrected chapter 3's refusal summary and chapter 6's description of environment-helper grants.
The whole correction run now teaches the concrete review and the caller's complete task before the general roles.
Configuration terms, headings, contents and figures agree. No hand-forward item remains for these parts.

All 22 examples, nine excerpts, 18 figure reproductions, strict prose checks and the book build passed.
Local link checks passed 2,075 HTML targets and nine README links, with failing controls. The proof records fresh source inventories and unchanged subject digests.
Safari inspection covered the six openings and all 13 figures at chapter width, including the lower parts and keys of tall figures.
The task states the visual bounds and the source-graph access limitation.

The node's Done when holds at this correction boundary. Read 1 stays live as `rhai-book-use-k74` and resumes at chapter 4.
Its correction recap makes amended Part I available without a repeat read. The owner's practical results remain outstanding.
The root and later book brief retain the cutover, final installed-layout, whole-book proof and Read 2 responsibilities.

## Third correction draft closure

The draft node `rhai-book-use-k93` closed on 2026-10-29 after four drafts, `rhai-book-use-k115` to `rhai-book-use-k118`.
Chapters 3, 4 and 5 name each example and show its workspace, paths, files and complete command before the table of its parts (F17).
Chapters 1 and 2 say that their code blocks are fragments and name the first session that the reader runs.
The chapters use the renamed interface, with no old-name explanation (F18). The default entry is `config.rhai`.
Chapter 6 now names four reserved grant classes, lists the installed notice file and quotes the current examples of chapters 1 and 5.

The last draft passed all 22 examples, nine excerpts, 18 figure reproductions and the book build.
Strict prose and break checks passed for chapters 1–6. A local check of the built chapters resolved 613 links, and each planted fault failed it.
The installed-files figure is under “Handed forward” for art. Copy-edit `rhai-book-use-k94` reads the new setup sections and the corrected entries next.

## Third correction proof closure

Proof `rhai-book-use-k96` read chapters 1–6 against the current contract and all lessons through L72.
The setup sequences now name their examples, directories, saved files and generated paths before the actions.
The complete command precedes its explanation. Current configuration names agree with the implementation and CLI help.
Chapter 6 and the installed-files figure now include `share/ravel/notices/THIRD-PARTY-NOTICES.md`, closing that delivery dependency.

The proof corrected chapter 5's repeated prompt label: Ravel reads the file but does not interpret its text.
It also corrected one word in the installed-files figure's key, then regenerated and inspected the figure at 750 pixels.
No Handed forward entry remains. No correction run or new checkpoint is necessary.

The final gates passed all 22 complete book examples, nine exact excerpts, 18 figure reproductions, strict prose and break checks, the book build and local links.
The proof task records fresh source inventories, failing controls, unchanged subjects and the viewing limits.
One physically bounded LLM reader ran chapter 4's first example successfully and reported no instruction gap.
Its first report misstated standard error before inspecting it; the captured stream and separate correction preserve that limitation.
Neither that model judgement nor mechanical execution establishes novice comprehension or replaces the owner's read.

The node's Done when holds at this correction boundary. Read 1 remains live as `rhai-book-use-k74`.
Resume at chapter 4, “Run the examples of this chapter”, after refreshing the local book and private reader shell.
Part I need not be repeated. The owner's practical tasks and the later whole-book proof and Read 2 remain required.

## Handed forward

None.
