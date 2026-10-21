# adr-native-waitability-status-k191

## Goal

Make two decision records state the current native-entry status. The record
`docs/adr/policy-evaluation-precedes-the-launch.md` says "native waitability
remains separate conformance work" (lines 17–18). `docs/specs/result-job-completion.md`
(lines 362–374) and chapter 9 of the book say the native entry is agreed,
implemented and measured per target. Decide which text is current and bring the
records into line.

## Context

- Found by the fresh reviewer of `book-architecture-fold-in-k164`. The same
  sentence stood in `docs/components.md` and `docs/local-runner.md`, which that
  leaf removes. The ADR was last changed in the partial-acquisition leaf, before
  the native-entry repair, so the sentence may be stale.
- Check `docs/adr/the-launched-child-is-a-job.md` line 19 too ("does not establish
  ... complete native-entry conformance"). It may be a true limit or stale.
- Read `ADR-FORMAT.md` (the grove skill): rework the record in place, state the
  current design, and keep no changelog. Chase the citations of any sentence that
  you change. The fold-in leaf changed only link targets in these records.
- Evidence: `docs/specs/result-job-completion.md`, `docs/native-entry-development-evidence.md`
  and `docs/book/src/09-jobs-signals-and-the-terminal.md` ("Know the guarantees
  and their limits"). Do not claim more than the measured support: aarch64 macOS
  and aarch64 GNU/Linux are measured, and native x86-64 Linux is not.

## Done when

- Each statement about native waitability or conformance in `docs/adr/` is
  either true of the current tree or removed, with the reason in the log.
- No link or anchor breaks. Strict STE does not apply to the ADRs.

## Notes

Not a book change. Do not edit the book chapters here. If chapter 9 is wrong,
route that to the book's whole-book proof by a note in the log.

## Decisions (running log)

- The spec and the evidence note are current. The ADR sentence "native waitability
  remains separate conformance work" was stale. The completion spec (lines 362–374)
  records native entry as agreed and implemented, and the k52 section of
  `docs/native-entry-development-evidence.md` measures it on aarch64 macOS and
  aarch64 GNU/Linux (Rust 1.85.0). Native x86-64 Linux is not measured.
- `docs/adr/policy-evaluation-precedes-the-launch.md`: replaced that sentence with
  the current status and two links (the spec's "Checked native-entry transitions"
  and the k52 section). Entry waitability of the worker and the managed child
  belongs to the agreed contract. The claim goes no further than the measured
  support.
- `docs/adr/the-launched-child-is-a-job.md`: "does not establish ... complete
  native-entry conformance" was a stale limit. Kept the true half (no atomic
  query/handback). Replaced the other half with the measured scope: two targets,
  nothing qualified on a native x86-64 Linux kernel.
- Left unchanged: `docs/specs/result-job-completion.md` line 62. It says the scoped
  C1–C4 observations do not establish complete native-entry conformance. That
  statement is about those observations and stays true.
- Chapter 9 ("Know the guarantees and their limits") agrees with the spec and the
  evidence note. No note for the whole-book proof is needed.
- Check: no stale phrasing remains in `docs/adr`, `docs/specs` or the book source.
  The positive control found the new text in the same tree. Both anchors exist
  (`#checked-native-entry-transitions`, `#gnulinux-immutable-campaign-k52`).
- The hook reports older STE findings in the ADRs. The brief excludes strict STE
  for the ADRs, so only the new text was kept to the rules (no semicolon).
