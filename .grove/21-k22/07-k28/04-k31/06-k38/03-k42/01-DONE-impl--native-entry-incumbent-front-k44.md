# native-entry-incumbent-front-k44

## Goal

Build the incumbent subject as its own frozen tree, add the `baseline` verdict
class to the plan and driver with failing controls, and record a classified
`baseline` result for every manifest row the front's instruments host: E01, E02,
E03, E04 (all three rows), E06 and E08 (46 rows). Invent no baseline failure.

## Context

First child of k42; read the k42 node brief (decomposition, premise rule and
build design), k40's section of `docs/native-entry-development-evidence.md`,
`docs/native-entry-campaign-plan.json` and `scripts/native-campaign.sh`. The
incumbent is `0515419edd29`. Instruments run as `cfg(test)` modules inside the
`ravel` binary built with `--cfg test` as a private fixture; the repaired
instrument sources are shared verbatim and never edited for the incumbent.

## Done when

- `scripts/native-incumbent.sh` builds the incumbent reproducibly from git plus a
  stored seam patch and instrument patch, with its own digest manifest taken
  before and after each run, outside the repaired tree's subjects; Taskfile
  tasks wrap it.
- The plan states a baseline classification for every row the front hosts and
  the driver reconciles baseline records by identity: `observed` rows need a
  `baseline` record and are classified `fails` or `already-correct` from it;
  `rejected-premise` carries a reason. Selftests reject a missing baseline
  record, an unclassified row and a rejected premise without a reason.
- The 46 front rows have `baseline` records from a real incumbent run, with the
  facts recorded. None of them needed a rejected premise: every operation they
  drive exists in the incumbent.
- Rows owned by k45/k46 remain unclassified and visible as such.
- Charges and balances recorded in the brief chain; no immutable evidence.

## Notes

Shared fixture code that has to change to compile against the incumbent changes
the repaired instrument digest: prefer the instrument patch over editing it, and
rerun the affected repaired rows if it must change.

## Decisions (running log)

- **Subject placement.** The incumbent tree is exported from git into
  `target/native-incumbent/tree`, not a sibling checkout: it is outside every
  repaired subject path, cleaned by `cargo clean`, and rebuilt from nothing each
  run. Its inputs (patches, overlay, build script) are under `scripts/`, which
  the driver already digests, and the tree itself is digested by
  `native-incumbent.sh digest` into the same before/after map under an
  `incumbent-` prefix.
- **Shared instruments verbatim.** The repaired front instrument is copied, not
  forked, and judged by its own `case`/`judge`/`judge_readers` claims; the
  baseline module sits beside them and reads row identities and parameters from
  the frozen manifest. `instruments.patch` is the only change to a shared file and
  is applied with `--fuzz=0`, so a moved hunk is a loud failure. The worker binary
  is shared because its sources, build script and compile script are identical,
  asserted from the two trees at build time.
- **Seams observe or place calls; none adds a mechanism.** The incumbent's
  initializer additionally reads pending signals, SIGCHLD flags and the capturing
  thread under `cfg(test)`, as observation only. The repaired instrument's hook
  calls are placed at the point of the incumbent's own flow that corresponds to
  each, including the refused-selection boundaries just before the front's
  settlement, where the choice has already dropped its handlers. That placement is
  recorded in the evidence note because the E06/E08 refusal classification
  depends on it.
- **Instrument integrity is not a baseline failure.** A controller that did not
  settle, or a barrier never reached, fails the test and files no record. The one
  exception is the fixture being ended by the delivered signal while it waits in
  the barrier, with everything the controller owns settled and the missing
  acknowledgement the only barrier fault: that is the incumbent taking the
  signal at the boundary, and it is recorded as a failed claim with the fact in
  the note.
- **Harness controls.** Two controls run the same instruments against the
  incumbent made wrong by a private seam (a wrong entry state handed to the
  child; readers created without the shield) and are seen to fail, so the 39
  already-correct readings are not an instrument that reads true everywhere. They
  file no row.
- **E04 managed is hosted here**, not in k45: the end-to-end claim at that
  boundary runs through the incumbent's front, and its failure (ECHILD on the
  runner's wait) is a recorded fact. The repaired tree's separate runner-level
  mutant discriminator has no counterpart because the incumbent has no repair to
  mutate. K45 therefore owns E05, E07 and S01-S05 (40 rows).
- **Result.** 46 of 46 front rows classified from real runs: 39 already-correct,
  7 fails (E04 worker and managed; E06 refusal-before-finalise x3; E08
  policy-refusal x2). 107 rows are unclassified and visible in the driver for
  k45 and k46. The development campaign (67 tests: 59 repaired, 8 incumbent)
  digested 319 subjects, unchanged after, and the repaired rows still reconcile.
  `task check` passed.
- **Charge** P<=22/E<=6/RC<=1 (decomposition and this slice), D=X=0, no reviewer
  spawned. Cumulative bounds D<=115/RC<=84/P<=419/E<=148/X=0 under caps
  D<=135/RC<=88/P<=467/E<=177/X=0; remaining D<=20 (phase closure),
  RC<=4/P<=48/E<=29/X=0 for k45, k46 and k43. A projection, not an allocation.
