# running-sessions-k74

## Goal

Draft chapter 5 as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/05-running-sessions.md`, linked
from `docs/book/src/SUMMARY.md`.

## Context

- Chapters 1–4 are proved before this runs. Use their examples and terms; link
  back with anchors.
- Enumerate the exit statuses and outcomes from the source first, then draft
  from the list, then check the draft against the list.

## Done when

- The chapter exists, builds, and meets the node brief's "Done when" for the
  draft stage; its complete examples pass `task docs:examples` and its
  cancellation material is labelled a fragment where the checker cannot run it.
- Last act: `leaf-add` the `copy-edit` leaf under the family rule.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. Its reader is “a
  software engineer with no background in Ravel's context”; its chapter 5
  outcome is “run sessions under real conditions … and interpret every
  outcome and exit code”. The ordered section plan places this chapter after
  policy authoring and before reference. Its emphasis names success after
  cleanup and the authority limits; its exclusions keep job-control depth,
  evidence campaigns, personal configuration and agent installation elsewhere.
- Before drafting, enumerated `main`, `refusal`, `run::supervision_failure`,
  `run::settle`, `Signal::exit`/`reraise` and program resolution. Status inventory:
  start success 0; missing result 1; malformed CLI/settings/confinement 2;
  policy/admission refusal 3; worker/supervision/result/cleanup/output failure
  5; evaluation deadline 124; pre-Started launch failure 126 or ENOENT 127;
  resolution unexecutable 126/not-found 127; HUP 129, INT 130, TERM 143.
  End-session success/repetition 0, publication/context failure 1, parse failure
  2. Help/version succeed with 0; missing subcommand displays help with 2.
  Arbitrary external fatal signals are outside normal settlement, not additional
  successful session outcomes. Cancellation re-raises its signal, with numeric
  fallback if propagation fails.
- Outcome inventory: valid result (empty/multiline/Unicode/option-looking,
  later nonzero child exit allowed); first complete publication wins; natural
  exit without result even at child status 0; nested channel replacement only
  for the inner child; selection/supervision cancellation, including after
  publication; refusal, timeout, missing/mismatched worker, resolution/spawn
  failure, invalid result, surviving group, cleanup/recovery and final-write
  failures. Original failures retain precedence over later cleanup errors;
  cancellation suppresses results and retains diagnostics. A final write may
  leave a prefix on failure. A routing notice is not completion evidence.
- Structure: request data → result handling/nesting → confinement → diagnostics
  → exit-status lookup → cancellation examples → operational limits. Complete
  examples use the established POSIX-shell fake convention and real worker;
  terminal-only interaction remains a fragment. No product behaviour changes.
- Tier 2 graph project `Users-antony-Development-Ravel.simplify-and-componentize`,
  generation `2026-10-16T04:54:32Z`: narrowed discovery and main outbound trace,
  exact settlement snippet and direct source reads. Coverage reported matching
  metadata with no recorded issue for all evidence paths and the front-source
  scope; that is best-effort coverage, not completeness proof. Broad discovery
  was narrowed rather than treating its truncated page as exhaustive.
- Draft coverage was checked against that inventory: the exit-status table
  includes all listed values and caller actions; the result/nesting, notices,
  cancellation and limits sections distinguish every listed outcome. Complete
  examples include after-publication cancellation; physical terminal interaction
  remains a labelled fragment. The contract's two sequences and outcome-machine
  extension are handed to art in the document brief, preserving stage ownership.
- Used the one in-session reviewer for a bounded adversarial check of result
  acceptance, diagnostic notices, the status table and cancellation teaching.
  Its instrument was source/test counterexamples, including existing failure
  branches beyond the new runnable examples. It returned no actionable finding;
  there were no findings to classify and no second reviewer. Read-only review
  establishes neither runtime evidence nor reader comprehension.
- Initial validation passed: `task docs:examples` reported 13 passed, 0 failed,
  0 skipped and no page errors (six chapter-5 examples); `task docs:book:build`
  exited 0 with no warnings. The final source check then added the unconfined
  cwd/program/environment explanation, using `program::resolve` and `run::run`.
  No ADR changes: this draft teaches existing behaviour without selecting a
  new architecture or changing the specifications.
- Final validation after the wording addition: `task docs:examples` exited 0,
  all 13 examples passed individually, none failed/skipped, no page errors;
  six fragments remain explicitly unrun across the book. `task docs:book:build`
  exited 0 without warnings. All 27 local anchor links in the built chapter
  resolve. The missing-subcommand invocation was also observed returning 2.
  SHA-256 checks of 163 subjects before and after the final run were unchanged:
  files enumerated beneath `crates`, `scripts` and `docs/book`, root Taskfile
  and Cargo manifest/lock, plus the built front and matched worker. This is
  mechanical validation on this host, not a new platform qualification,
  real-agent test, rendered visual inspection or human comprehension result.
- The node remains open for copy-edit, art and proof. No review leaf names this
  producer or node, so no Creator receipt is applicable. The successor is the
  mandatory copy-edit stage, with the chapter's bare stem. No parent cascade
  closes and no guide migration or book-level proof is claimed.
