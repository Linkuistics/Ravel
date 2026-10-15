# native-entry-linux-campaign-run-k52

## Goal

Take the immutable GNU/Linux measurement on the corrected tree, as k43 did for
Darwin: aarch64-unknown-linux-gnu natively at Rust 1.85.0, and
x86_64-unknown-linux-gnu under emulation recorded as emulated, each with R01
through the six Taskfile tasks, and reconcile the documents and the evidence note
with what is then measured.

## Context

Read the node brief, the k50 leaf's decisions, the evidence note's development
section and `scripts/native-linux.sh`, `scripts/native-linux/measure.sh` and
`scripts/native-r01.sh`. The development runs showed what to expect: aarch64
reconciles 153 of 153 with one platform override on `E03__disposition=ignore` and
`E04__boundary=raw-executable-entry`; x86_64 emulated reconciles 146 and limits 7;
74 of the plain suite's 338 identities cannot run under emulation.

## Done when

- **Documents first.** The completion spec's closing status paragraph and the
  transition design's status statements are digested subjects. After the
  correction (k51) and before any run, read them against what will be measured and
  edit them once; the campaign's digests then describe the tree that was measured.
  They state per target what is immutable, what is emulated and what is open.
- **aarch64 native, immutable**: `task test:native-linux PLATFORM=arm64 OUT=...`
  from a fresh environment (one writer, never reused), producing
  `domain.json` (kernel, glibc, rustc 1.85.0, nextest, task, tool digests), the
  Rust 1.85.0 build/clippy(-D warnings)/fmt steps, the frozen campaign with
  subjects digested before and after and unchanged, **153 of 153 rows reconciled
  item by item** with receipts, and R01's 310 identities and 20 ignored flags
  preserved through test:cli, test:session, test:runner, test:terminal-recovery,
  test and check, joined identity by identity (the join already has its failing
  controls). Failures and new identities (74 expected) are reported separately.
- **x86_64 emulated, recorded as emulated**: the same, from `PLATFORM=amd64`,
  with `domain.json` naming the emulator and its options, the rows the emulator
  cannot reproduce accounted for with reason and control (not reconciled, not
  dropped), and the protected identities that cannot run (namespace creation)
  listed separately. No emulated receipt is called qualification.
- **Darwin**: because instruments and production source changed since k43, run
  `task test:native-campaign` on the host into a new directory and show its
  receipts equal k43's item by item (it did before the correction); the k43
  directories stay untouched.
- The measured domain per target, the support limits per target and the actual
  charges are written into the evidence note and the brief chain. The native
  x86_64 choice is `native-entry-x86-64-linux-scope`, already cut; update its
  evidence from the immutable emulated run if the numbers moved.
- Large files compress on the way out (the version control refuses a new file
  over 1 MiB); the originals' SHA-256 is recorded beside them.

## Notes

Run the targets one at a time, and do not use the machine for anything else
while one runs: the controllers' deadlines (30 s case, 5 s acknowledgement) are
real. The aarch64 development campaign took 1 minute 11 seconds and the emulated
one 5 minutes 26 seconds with no controller timeout, so no timing re-price is
expected; if a legal limit cannot fit on a re-run, record and price the exact
remainder, do not widen it silently.

## Decisions (running log)

- **Documents edited once, before any run, to say only what holds whatever the
  runs show.** The completion spec's closing paragraph and the transition design's
  status statements (opening paragraph, the Linux/compiler paragraph, the manifest
  heading sentence, the K28 and price-handoff sentences) no longer say "Darwin
  only", "none has immutable receipts" or "the lint step is not yet clean" (k51
  made it clean). They name the three measured targets, say x86_64 is emulated and
  never qualification, state what the emulator cannot do, and point to the
  evidence note for each target's domain, receipts and limits; results live in the
  note, which is not a campaign subject. Open and unmeasured is named: a native
  x86-64 kernel, other kernels/libc/Rust versions, returning host, independent
  adoption. After this edit no digested path changes until the runs end.

- **The first aarch64 run failed, was kept as the finding, and was not reported
  as a measurement.** The incumbent driver's worker-identity guard compares
  `crates/ravel/build.rs` byte for byte and k51's `hex` rewrite is the one
  difference; no run had exercised the guard since the edit. Repaired by pinning
  that difference as the SHA-256 of its normal-format `diff` (a second pinned
  value, not a loosened comparison); selftest gained three controls (pristine
  accepted, repaired-tree file accepted, one further line rejected). A `diff`
  exit status under `pipefail` made the first version of the control die silently:
  fixed with the script's own `{ diff ... || true; }` idiom before it counted.
  `scripts/` is digested, so every reported run was taken after the edit.
- **Order: Darwin first.** It is the cheapest run and exercises the same incumbent
  build path, so a second instrument defect would have invalidated one run and not
  three. It equalled k43 receipt for receipt (0 of 153 differ; free text, pids,
  list order excluded). Linux then ran one target at a time.
- **x86_64's R01 logs are not a partition and are not presented as one.** Under
  emulation nextest's default fail-fast stops each task at two failures. The
  immutable campaign and R01 logs are kept as measured; the full account is a
  supplement in the still-up container after the subjects were digested a final
  time: the Taskfile's `test` command plus `--no-fail-fast`, 261 passed / 77
  failed / 46 skipped, equal to k50's development figures. No instrument was
  edited for it (`measure.sh` is a subject).
- **Two of the 74 namespace failures are inferred, and say so.** Their assertion
  fires (exit 2 where 5) before any diagnostic is captured; both pass `--confine`
  and both pass natively. The classification file marks them.
- **k43's stored `test-cli` join reads scope 0** while its log has 13 PASS lines
  and its own table says 13 of 13. Recorded in the note; k43's directory untouched.
- **Documents state scope, not outcome**, so they stay true whatever a run shows;
  results live in the evidence note, which is not a subject.
- **No reviewer spent (RC=0).** The new guard's claim is covered by an executable
  control seen to reject its wrong input. No review leaf is cut.

## Carried from k51

- The corrected tree passes `task check` at 1.85.0 on aarch64 Linux and at
  Darwin's compiler, and clippy 0.1.85 (1.85.0 and 1.85.1) is clean. The lint
  harness and its failing control are in
  `docs/evidence/native-entry-minimum-rust-lint-correction/lint.sh`; name clippy's
  version in every log and select the toolchain with `RUSTUP_TOOLCHAIN`.
- One `task check` on Darwin missed a 1000 ms selection deadline once
  (`native_f03_first_create`, `first-create-ignored`) while indexers and the
  terminal were running; six whole-suite runs and 30 isolated runs otherwise
  passed. Take the immutable runs on a quiet host, one at a time, and treat a
  repeat as a finding rather than rerunning until it passes.
- The Darwin and Linux R01 summaries of k51 describe the tree before this run's
  document edits; they are not a substitute for this run's receipts.
