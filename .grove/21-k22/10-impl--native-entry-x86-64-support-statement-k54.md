# native-entry-x86-64-support-statement-k54

## Goal

Carry the human's k53 decision into the shipping documents. Native x86-64
qualification is deferred, not rejected, and tracked in
[Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1), which
depends on
[Linkuistics/TestAnyware#1](https://github.com/Linkuistics/TestAnyware/issues/1).
No document may still point at the retiring grove handle
`native-entry-x86-64-linux-scope` as an open choice, and none may read as an
x86-64 qualification.

## Context

- The decision, the human's verbatim words and the exact support statement to
  carry are in the decision log of `native-entry-x86-64-linux-scope-k53` (its
  `DONE` file beside this one), under **Settled**. Both issues are public.
  Ravel#1 states the agreed approach: TestAnyware gains an x86_64 Linux guest
  under QEMU full-system emulation, Ravel's x86_64 binaries are cross-built on
  the host and run there, and the result is recorded as a real x86-64 kernel on
  an emulated CPU.
- k53's sweep looked for the pattern `native-entry-x86-64-linux-scope`. Its
  positive control was the spec line, and a mutated pattern came back clean.
  The handle, with "open" wording beside it, is in three shipping documents:
  - `docs/specs/result-job-completion.md`, around lines 374–376: the
    conformance paragraph ending "Open and unmeasured: a native x86-64 kernel
    (`native-entry-x86-64-linux-scope`), any other kernel, libc or Rust
    version, a returning host and independent adoption."
  - `docs/native-entry-transition-design.md`, lines 3–9: the status header,
    "a native x86-64 kernel is the open `native-entry-x86-64-linux-scope`".
  - `docs/native-entry-development-evidence.md`, two places. First, the k52
    "Support limits, per target" bullet for x86_64, around lines 1546–1550:
    "Native x86-64 is the open choice in `native-entry-x86-64-linux-scope`;
    the numbers did not move from k50's." Second, the close of "Exact
    remainder before acceptance", around line 1611: "a native x86-64 kernel
    remains open".
  These line numbers are from before any edit and are only a starting point.
  The claim, not this list, sets the sweep's scope.
- `docs/RELEASING.md` lists x86_64-unknown-linux-gnu among the supported
  release targets, smoke-tested only under emulation, and says nothing about
  native-entry conformance. Decide on the evidence whether a sentence pointing
  to Ravel#1 belongs there. Do not change what is released.

## Done when

- Each statement above names the settled limit and links Ravel#1 instead of the
  grove handle. The limit: x86_64-unknown-linux-gnu is built, linted at Rust
  1.85.0 and run only under QEMU user-mode emulation (146 of 153 rows
  reconciled, 7 emulation-limited, 74 protected identities unable to run).
  Nothing about an x86-64 kernel is qualified. Native qualification is
  deferred, not rejected, and tracked in Ravel#1. Native-entry conformance is
  measured with immutable receipts on aarch64-apple-darwin and
  aarch64-unknown-linux-gnu only.
- A sweep of the whole tree finds no remaining pointer to
  `native-entry-x86-64-linux-scope` outside `.grove/`, and no sentence that
  presents native x86-64 as an open choice in this workstream. The sweep
  includes dotfiles and root files. It needs a positive control seen to hit
  before the edit and a mutated control seen to miss. Sweep the summary layer
  (status headers, tables, overview paragraphs) as well as the sections.
- Nothing is narrowed. The 153-row matrix, the 310 protected identities and
  every recorded limit stay as measured.

## Notes

- Documents only: no product source, instrument, plan or Taskfile change, and
  no measurement or re-run. The edit changes no behaviour.
- Do not edit `docs/native-entry-campaign-plan.json` or
  `docs/native-entry-cases.json`. Both are frozen, digested campaign subjects,
  and the plan's E07 limit text saying no native x86-64 run exists stays true.
  Do not edit anything under `docs/evidence/` (immutable receipts). In the
  evidence note, change only the status pointers, never the measured figures
  or what a run recorded.
- The spec and the design are digested campaign subjects (they are the "two
  documents" in k43's and k52's subject lists); the evidence note is not. After
  this edit, those runs' subject digests describe the tree before it. That is
  expected, since a receipt describes its own tree, and needs no re-run. Where
  the evidence note says the documents claim scope without outcome, keep that
  true.
- After this leaf retires, k22 has no live leaf, so this session closes k22 by
  the four steps in the grove's `references/retire.md`. k22's Notes require
  support obligations to be kept explicit; Ravel#1 now carries the x86-64 one.
  Promotion to the root brief comes before book-structure-k5.
