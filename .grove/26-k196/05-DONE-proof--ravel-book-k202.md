# ravel-book-k202

## Goal

Proof the whole eleven-chapter book after the art stage. Check every class
in the document's contract and close the publishing pipeline for this node.

## Context

- The node brief `ravel-book-without-chapter-ten-k196` states the owner's
  decision: Ravel supports no separate component. Develop is chapter 10,
  Deliver is chapter 11, and Part IV covers development and delivery.
- Read the contract, the book's prose and figure conventions, `CONTEXT.md`
  and the whole book. The prior stages are `ravel-book-k197`,
  `release-script-chapter-names-k199`, `ravel-book-k200` and `ravel-book-k201`.
- Art checked all fourteen figures and changed only `source-components`.
  Its key, caption and alt text now explain that C4 components are internal
  code. Ravel supports no separate component. The C4 term remains.
  The SVG and layout record match the revised Typst source.
- Art inspected all SVGs at the book's 750-pixel column width. It also
  inspected the revised chapter and its full-size link in local Chrome.
  That evidence establishes no view on GitHub or GitHub Pages.
- The node's Handed forward list has a chapter 11 placement question for
  proof. Resolve it under the editorial family rules. A structural move
  belongs to a draft correction run.
- Read the log of `adr-native-waitability-status-k191` for any chapter 9
  finding sent to this whole-book proof.

## Done when

- The whole-book read covers the contract's requirements, technical truth,
  terms, links, chapter numbers, figure roles and prose rules. No finding
  remains unresolved or silently passed beyond proof.
- The node's Handed forward list is clear. If an earlier charter owns a
  defect, use the editorial family's ordered correction run.
- The node's required checks pass: `task docs:book:build`, `docs:examples`,
  `docs:excerpts`, `docs:figures`, `docs:ste -- --strict`,
  `docs:breaks -- --strict`, `release:lifecycle:test` and `check`.
- Resolve each remaining node acceptance condition before closing it.
  Retire and commit under the Grove procedure, including any parent cascade.

## Notes

- Art's final figure comparison, book build, strict STE and strict breaks
  checks passed with unchanged validation inputs. Its task records the
  visual inspection and declined figures.
- Proof normally adds no successor. Follow the family rules if a finding
  requires a correction run.

## Decisions (running log)

- Chapter 11's separate-component paragraph does not belong under Pages
  setup. It describes the delivered tool, not the website setup. A draft
  correction run will move it beside the chapter's introduction. The run
  continues through copy-edit, art and proof under the editorial rules.
- The contract records codex-cli 0.162.1 for the 2026-10-16 flag check.
  Chapter 4 records 0.163.0. The k85 draft log explicitly replaces the
  earlier check with 0.163.0. The correction run will reconcile this dated
  provenance without claiming a new check of current agents.
- The whole-book read covered all eleven chapters, the README, contents,
  glossary, contract, word rules and figure conventions. The two session
  specifications and the three cited ADRs agree with the book's lifecycle,
  authority and platform limits. The k191 log sends no chapter 9 defect.
- The source-derived inventories cover fifteen session options, both verbs,
  help, version, exit statuses, three settings keys and nineteen SDK exports.
  Chapter 6 holds their lookup entries. Chapters 4 and 6 agree on request,
  selection, refusal, context and host fields. The environment tables retain
  the worker grants, reserved classes and managed-process rules.
- All 56 public tasks from `task --list` occur in the book. Chapters 10 and
  11 distinguish development, evidence instruments and public release work.
  The ten internal obligations in the contract remain explained across
  chapters 2–9. These checks establish coverage, not reader comprehension.
- Local HTML validation found no broken local link among 1,604 links on
  fifteen pages. The eighteen inbound Markdown links to the renamed
  chapters also resolved. The checker rejected an old chapter path and a
  missing anchor, and accepted the current development anchor.
- Local Chrome showed the eleven-chapter contents and chapter 11's Pages
  section. Its final paragraph changes topic from website setup to the
  tool's component policy. That topic change supports the placement finding. Raw Markdown
  and the local build were available. GitHub and GitHub Pages remain
  unchecked for this branch and first book release.
- Graph verification used Tier 2 in project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T14:48:50Z`. Source coverage recorded no gap in the inspected
  Rust and SDK paths. Changed Markdown and configuration needed direct
  reads. The graph excludes SVGs. The art record supplies their
  prior visual inspection, and this proof checks their textual roles and
  fresh regeneration. The graph does not establish complete coverage.
- The first `task check` failed after 308 passes in
  `worker_reap_admission_primary_reports_refusal_both`. Its refusal fixture
  uses a one-second selection deadline. It received `selection_timeout`
  instead of `policy_refused`. The unchanged case passed alone. This supports
  a timing hypothesis, but does not establish the cause. No source or test
  changed. The full check will run again, with both logs retained.
- The chapter-reference inventory covers the README, Taskfile, Markdown
  under `docs/` and shell scripts under `scripts/`. Current references use
  chapters 10 and 11 for development and delivery. Dated evidence and figure
  history keep their earlier labels. The search detected the planted stale chapter,
  and the same pattern found the historical chapter 12 in the figure record.
  The CLI inventory control also rejected a planted absent option.
- The second full check failed in a different one-second worker fixture,
  `worker_reap_admission_recovered_poll_success`. Its diagnostic says the
  worker was not ready within 1,000 milliseconds. That unchanged case also
  passed alone. The next check limits nextest to four concurrent tests,
  using its documented `NEXTEST_TEST_THREADS` option. This changes scheduling,
  not test selection or the fixture deadline. Both default-schedule failures
  remain evidence. No claim about their root cause follows from a rerun.
- Final validation passed on 2026-10-21: book build, fourteen complete
  examples, eight excerpts, fourteen figure comparisons, strict book STE,
  strict prose breaks and release lifecycle tests. No example failed or
  skipped. With `NEXTEST_TEST_THREADS=4`, `task check` passed all stages and
  all 339 selected tests, with 46 skipped. The two default-schedule runs
  failed as recorded above. This is a scheduling limit on the check result.
- The final SHA-256 comparison found all 4,642 inputs unchanged. The inputs
  included tracked files outside `.grove/`, checkout tools and assets, worker
  dependencies, the prose checker and the link checker. Scratch manifests
  and logs are under `/tmp/ravel-k202-*`. Build outputs were not inputs.
  No product, book, test, specification or ADR changed during this proof.
- The correction run replaces the ordinary end of the pipeline. It owns
  both findings, so this proof leaves no unowned handoff. The document node
  stays open until the correction run's proof checks its acceptance conditions.
