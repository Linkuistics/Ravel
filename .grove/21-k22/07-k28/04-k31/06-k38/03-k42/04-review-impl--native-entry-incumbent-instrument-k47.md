# native-entry-incumbent-instrument-k47

**Reviews:** native-entry-incumbent-baseline-k42

## Goal

A fresh-context, inspection-only adversarial read of the whole incumbent
instrument before k43 freezes it: the incumbent build, its seams and overlay,
the baseline modules, and the plan's classification of all 153 rows (65 fails,
56 already-correct, 32 rejected premises). Produce findings and no fixes. The
question is whether any row's class is wrong, in either direction: a failure the
incumbent was never observed to commit, or a held claim read by an instrument
that cannot fail.

## Context

Read the k42 node brief, then `scripts/native-incumbent.sh`,
`scripts/native-incumbent/{seams.patch,instruments.patch,files/}`,
`docs/native-entry-campaign-plan.json` (the `baseline` entries) and the k44, k45
and k46 sections of `docs/native-entry-development-evidence.md`. The producer
leaves are k44, k45 and k46 under k42; their decisions logs carry each session's
judgement calls. The k45 session spent its one in-session reviewer on the runner
rows; k46 spent none, because this leaf is its scheduled review.

Where to look first, in order of what a wrong call would cost:

1. k44's placement of the refused-selection barriers (`before-finalise`,
   `selection-failure-before-finalise` in the incumbent's `run.rs`): the E06/E08
   refusal classification depends on it, and the F04 selection rows' unmet
   "handled signals blocked at finalisation" claim depends on the same boundary.
2. The F04 selection and final-restore rows (14) and F05's four final-mask-restore
   rows, where the one unmet claim in several rows is a repaired-design property
   (a retained block) or a label ("final mask release"), restated here as "the
   report names the failed operation". Is that restatement fair, or does it
   manufacture a failure? The evidence note lists these as judgement calls 1
   and 2.
3. The 32 rejected premises. For each, is the named missing operation really
   absent from the incumbent under the row's own state, and is a hosted
   alternative (a seam at an operation the incumbent does perform) being refused
   without cause? Especially F07 TERM-default (the incumbent's `Handlers::drop`
   restores the entry action; is that the "default-action transition" the row
   names?), F02 managed repair (the incumbent repairs `SIG_IGN` only), and S01.
4. F01: the substituted report is applied by re-running the incumbent's own
   `record()` after `main`, keeping the real record except for TERM. Does that
   measure the incumbent's initializer, or the seam?
5. The paired F05 Started row, hosted at the runner with the observer standing in
   for the front's swallowed restore.
6. Whether any seam adds behaviour. The incumbent's non-test build must be
   unchanged in effect; `cargo build -p ravel` and `-p keyed-launch` with and
   without `--features native-faults` were built, but read the `cfg(not(test))`
   branches rather than trusting that.

## Done when

- Every finding names the row or file, the claim it puts in doubt and a concrete
  failure scenario, or says the read found nothing.
- The review states which rows it re-derived from source and which it took from
  the evidence note.
- If it has findings worth acting on it cuts one `integrate-review-impl` leaf
  whose body carries this handle and not the findings.

## Notes

Inspection only: do not edit the incumbent tree, the plan or the instruments.
`task test:native-incumbent-controls` and `task test:native-campaign-controls`
reproduce the build and the driver's failing controls. A development campaign of
the finished state ran 109 tests with none failing and reconciled 153 of 153.

## Decisions (running log)
