# Whole-book proof — rhai-book-inside-k136

## Result and scope

On 14 November 2026, the proof producer freshly read all eleven chapters as one
document, against `docs/book/contract.md`, `docs/book/ste.md`, `CONTEXT.md`,
lessons L01–L86 and the five ADRs in the k131 obligation map. Bounded reads
recovered passages clipped by tool output. Previous tasks' partial reads did
not count. This is the final editorial proof, not a reader-comprehension result.

Two corrections resulted:

- Chapter 3 now attributes `home/` to the reader. Its setup step names
  `home/.config/ravel`, which the visible `mkdir` creates before settings are saved.
- Chapter 10 distinguishes private writing and flushing from atomic publication
  through a hard link. It says that the CLI reads the result after job cleanup.
  `crates/ravel/src/completion.rs:175` performs those operations in that order.

The correction needed one paragraph break: the first strict STE run rejected a
seven-sentence paragraph. The corrected paragraph passes. The earlier failure
remains in `strict-ste.log`; `strict-ste-final.log` is the final result.
No executable example block, source implementation, excerpt or figure changed.
Chapter 3's textual directory diagram did change, so its aggregate fenced-block
hash differs. Chapter 10's fenced blocks are identical; see `fenced-blocks.json`.

## Whole-document reconciliation

| Chapters | Cross-document questions checked |
| --- | --- |
| 1–2 | Concrete review request, caller instructions and next actions precede the general roles. Result words remain opaque. The worker and managed job have distinct authority. |
| 3 | Visible files, private setup, settings and launch command agree. Current unversioned installation and the fixed 2.0.0 site's versioned instructions remain distinct. |
| 4 | All twelve saved-file inventories include settings. Imports, caller-relative reads, context, host operations, fake harnesses and engine bounds agree with the reference. |
| 5–6 | First result, natural exit, statuses, streams, environment grants, cancellation and cleanup retain the same meanings in task teaching and direct lookup. |
| 7–8 | The concrete parts example exposes the CLI's parent role. Two Rust packages are distinguished from processes. The same executable starts the separate worker, whose host requests run in the CLI. Exact excerpts follow the actual call path. |
| 8–9 | Result observation, group cleanup before reap, conditional terminal recovery, final cancellation and output suppression agree. Signal-source priority is not described as arrival order. |
| 10–11 | Contributor tasks, installed layout, archive contents, release channels, promotion and recovery agree with current sources and dated delivery evidence. |

The producer checked the owner's Read 1 patterns throughout the eleven chapters:
concrete examples before abstraction; actors, mechanisms and outcomes; local term
explanations; complete task instructions and result use; saved and generated file
attribution; useful figures and tables; explicit evidence limits; familiar
commands and descriptive headings. STE and prose-break checks support that read,
but do not establish that a new reader understands the book.

The source check retained chapter 3's distinction between checking a selected
command and resolving its executable. `evaluate_and_resolve_rhai` judges the
worker reply, then checks cancellation and resolves the program after admitted
worker teardown. `choose_prepared` checks cancellation again. The book's
worker/package explanation and final-cancellation trace need no further repair.

The two corrections instantiate existing lessons about visible setup and precise
mechanism/order (including L55, L67 and L76). No new general lesson was needed.
No unresolved `Handed forward` list or entry remains in the editorial subtree.

## Five inventories and adversarial review

The producer reconciled the k125–k129 inventories against the fresh book read
and current input digests. The one permitted fresh reviewer independently read
all CLI tables, 236 configuration rules, 134 lifecycle rules across 16 classes,
six components and ten obligations, 57 task entries and 77 script entries.
It also read the three script-only details. It found no new actionable omission,
mapping defect or contradiction. This allowance is spent; no second reviewer ran.

Independent semantic probes covered failed UTF-8/JSON read accounting, fatal
harness-request validation, canonical import caching, CLI definitions,
cancellation-source precedence, cleanup bounds and identity ordering, detached
launch modes, book-tool integrity controls and installation fixtures.

Material source anchors were `crates/ravel/src/rhai_worker/host.rs:492`,
`crates/ravel/src/rhai_worker/bridge.rs:258`,
`crates/ravel/src/rhai_worker/resolver.rs:57`, `crates/ravel/src/cli.rs:20`,
`scripts/book.sh:58`, `scripts/book-bun.sh:42` and
`scripts/installed-check.sh:7`. The reviewer challenged teaching at chapter 8's
final cancellation section and chapter 9's cleanup and detached-launch sections.
The reviewer made no changes and ran no tests.

Graph evidence used Tier 2 for project
`Users-antony-Development-AgenticOrchestration-Ravel.gh-issue-2`.
The producer's exact `completion.rs` and `choice.rs` paths, inventory paths and
bounded Rust scopes reported `no_recorded_issue` and `metadata_match` at generation
`2026-11-14T03:42:26Z`. The review's exact relied paths had the same clean metadata
at generation `2026-11-14T03:58:46Z`. Clean metadata means no recorded gap, not
proof of completeness. Broad script coverage has known gaps in
`scripts/examples.test.sh:1–779`, `scripts/native-linux-qemu.sh:116`,
`scripts/release-smoke-qemu.sh:184` and an ignored SVG fixture. No exhaustive
script-source claim relies on that coverage.

The review read every inventory row but challenged selected high-risk source
semantics. It did not independently execute the inventories or verify every
cited source line. Inventory coverage and model review do not establish owner
comprehension.

## Measurements and reuse

The k135 437-input manifest was the comparison baseline. Only chapters 3 and 10
changed; `changed-subjects.json` names them. `subjects-initial.sha256` retains the
first freeze. After the paragraph repair, `subjects-before.sha256` was frozen
again before the final checks. All 437 subjects match `subjects-after.sha256`;
`subjects-verification.json` records each comparison.

Each command below used the existing mise environment and one measurement writer.
No tool or global configuration was installed or changed for this proof.
The observed environment is Darwin arm64, Rust 1.99.1
(`58cf8f9c3 2026-10-13`) and Task 3.54.0.

| Command | Observed result and record |
| --- | --- |
| `mise exec -- task docs:book:build docs:excerpts docs:ste docs:breaks` | Exit 0; full build, 22 exact excerpts, 35 STE subjects and 11 prose-break subjects pass in `checks-final.log`. |
| `mise exec -- task docs:ste -- --strict` and `mise exec -- task docs:breaks -- --strict` | Both exit 0 for the full default subjects; `strict-checks.log`. The earlier affected-file strict STE result remains in `strict-ste-final.log`. |
| `mise exec -- task docs:examples -- docs/book/src/03-a-complete-session.md docs/book/src/10-develop-ravel.md` | Exit 0; one complete example passes, 14 fragments recognised, no page errors; `examples.log`. |
| `mise exec -- task docs:ste:test docs:breaks:test docs:examples:test docs:examples:mutants` | Exit 0; applicable checker controls pass, all 30 example mutants killed; `controls.log`. |

Unchanged subjects retain k132/k135 measurements: 27 complete examples in total,
73 recognised fragments, 22 exact excerpts, 21 reproducible figures, tool-integrity
controls and 24 killed excerpt mutants. The changed complete example was measured
again. In particular, the unchanged lone-`policy.ts` example retains the measured
`config_format_retired` refusal, empty standard output and status 3. The manifests
also retain the tools and implementations on which those measurements depend.
No Rust implementation changed, so this proof adds no claim of a new full Rust
or platform test campaign.

## Links and rendering

`links.json` holds 2,956 source/built destinations with zero failures. The inline
Python check excluded fenced Markdown, resolved local inline links/images, and
parsed built HTML `href`, `src` and element IDs with `HTMLParser`. Markdown source
paths were checked for existence; built HTML fragments were checked against IDs
in their destination documents. Three dirty controls rejected a missing source
file, a missing built page and a missing built anchor. This measures local
paths and built destinations, not the continuing availability of every external
site or every possible Markdown extension.

`figure-surfaces.json` identifies all 21 unique figures at a 750-pixel chapter
width in local built mdBook pages, in Chrome 157.0.8103.79. It records 31 visible
occurrences and excludes 31 hidden mdBook duplicates. Each unique figure was
visually inspected: labels and connections were readable, with no overlap or
clipped diagram content found. `figure-observations.json` records each figure,
page, observation and screenshot hash. Long `session-code-path` and
`release-delivery` figures also received three overlapping 1,200-pixel viewport
captures each; `figure-detail-surfaces.json` records actual scroll positions.
Earlier unsuccessful detail captures repeated a position; they were replaced
before these recorded views were inspected. Reproducible SVG bytes and visual
quality are separate observations.

The producer also viewed chapter 7's `session-authority` figure and neighbouring
text on GitHub's `main` preview and the published `v2.0.0` site. Both loaded and
were readable. `remote-surfaces.json` records their exact URLs and loaded images.
Those are remote revisions, not this unpushed correction. The other remote
figures and pages, editor preview and raw SVG browser rendering were not visually
inspected. Local raw Markdown was read for all eleven chapters. Screenshots live
under ignored `target/k136-visual/`; the manifests retain hashes, not a promise
that generated local screenshots survive checkout cleanup. No published rendering
claim applies to the two local prose corrections.

## Private reference and release facts

The producer directly consulted applicable Part 1 rules in the owner's exact
`/Users/antony/Downloads/ASD-STE100_ISSUE9.pdf`. Its expected SHA-256 is
`d1f4ea9e7cd6e46b47aa9057209f99e78c0e9cfc4e27a5b07895b05c1a166431`.
Consultation included verbs, complete sentences, procedures, descriptions,
safety instructions, punctuation/counting and writing practice (physical pages
67, 79, 87, 95, 103, 107 and 115). The contract excludes dictionary enforcement
and requires British spelling. Neither the PDF nor its private extracts is
included here.

`external-reconciliation.json` records dated HTTP 200 observations and digests
for the v2.0.0 release and the tap's `ravel`, `ravel@2` and `grove` formulae.
The release has three target archives. Both Ravel formulae select 2.0.0;
`ravel@2` is keg-only and deprecated in favour of `ravel`. Grove 22.9.0 depends
on unversioned `ravel`. This supports the book's promoted-channel account.
It does not prove successful installation on another machine. Historical
`docs/evidence/ravel-book-k311/` and the ancestor's `out.err` remain unchanged.

## Limits and remaining reader work

New executable measurements are native arm64 macOS with fake harness programs.
They add no Linux, real-agent, security-isolation or novice-comprehension evidence.
Historical measurements keep their original dates and limits. The final proof
cut `rhai-book-read2-k137`, a live requirements leaf at the grove root, outside
the editorial node and ahead of the remaining release work. That live requirements task
must retain the owner's own words, locations, progress and unreached material,
and stop at the first comprehension failure. No model result completes it.
