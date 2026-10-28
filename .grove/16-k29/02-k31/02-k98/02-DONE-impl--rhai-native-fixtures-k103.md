# rhai-native-fixtures-k103

## Goal

Port `native_entry_tests.rs`, `partial_acquisition_tests.rs` and `worker_admission_tests.rs` to Rhai without losing their fault discriminators or cleanup and native-entry assertions.

## Context

The parent k98 running log records the graph refusal. Establish graph availability again; no graph evidence exists for this handoff. Source reads found ordinary policies beside fake protocol workers: native E08 installs a script speaking the TypeScript protocol; admission and acquisition use test executables which do not dispatch the Rhai role by argv[0]. The existing Rhai `front.rs` tests use `rhai_worker::role::tests::role_fixture` through a shell wrapper. The actual private native-main binary supports the real role. Inspect these seams before changing formats, and retain each injected failure, stage, diagnostic and no-launch assertion. `context.rs`, `settings.rs`, `choice.rs`, `limits.rs`, `frame.rs` and `run.rs` must also be checked for any remaining fixture work after k102.

During k102 verification, `partial_acquisition_cleanup_worker_timeout_remove_wait` passed in `task check`, then failed in a separate full test run and an isolated run while build work was active: the 1000 ms timeout arrived before the expected owner diagnostics (once before worker readiness). It passed alone after build/book work finished. Host load averages reached 31.80, 17.39, 12.61. The unchanged fixture assumes startup reaches its diagnostic prints before the whole selection deadline. Preserve those diagnostics assertions in the Rhai port and make readiness and the intended timeout phase explicit; do not accept missing diagnostics merely because the clock expired. Failed full-run evidence: `/tmp/ravel-k102-test.log` (ephemeral).

## Done when

- Native-entry, admission and acquisition policies run Rhai, with fake protocol fixtures updated to its protocol as necessary.
- Every existing fault discriminator remains effective, including native E08 readers and escaped pipe holder.
- Focused suites, `task check`, `task test`, and both book checkers pass. Re-anchor quoted source if moved.
- Parent k98's full done conditions hold; keep both production runtime paths until k101.
## Implementation plan

1. Port the three suites' scripts to schema 3 Rhai. Run representative cases to expose the missing libtest role dispatch before adding its test-only seam.
2. Supply an explicit test-only executable override and the existing real-role shell wrapper. Port both fake protocol workers to the Rhai hello/evaluate/selection exchange, retaining E08's descriptor handoff.
3. Keep failure codes, stages, cleanup receipts, cancellation and no-launch assertions. Require timeout readiness diagnostics before accepting the deadline failure; accommodate Rhai's source-position prefix on debug output while still requiring stderr's payload.
4. Inspect the remaining named runtime units and the ordinary Rust fixture surface. Run focused acquisition/admission/native suites, task check, task test, and both book checkers; repair any moved quotation.
5. Audit the parent done conditions, retire this leaf, close k98 if satisfied, and seal the focused jj change before signalling Grove.

## Decisions (running log)

2026-10-28: The graph CLI again refused list_projects because a pre-coordination or unverified CBM generation is active. No project/generation or graph evidence is available. Use direct source reads and targeted searches, without inferring absence from the graph.

2026-10-28: Keep the production executable lookup and both runtimes unchanged. The libtest fixtures need a cfg(test)-only executable override because libtest does not dispatch argv[0]. Use the existing real-role wrapper pattern; native main uses its actual Rhai role except E08, whose fake protocol and escaped stdout holder remain necessary. Preserve timeout diagnostics and add an explicit selection-readiness marker before a non-yielding sleep. The existing approved migration brief supplies the design; no new product behavior is introduced.

2026-10-28: Red evidence first showed that a Rhai policy without the launch seam recursively entered libtest instead of serving the role. After adding the seam, the migrated refusal-plus-cleanup cases exposed a production gap: Rhai returned the owner refusal as a successful conversation and only judged it after supervise had replaced it with worker_failed. Preserve the existing fault assertions and repair that ordering by giving the Rhai conversation a reply judge, before teardown. Program resolution and launch still require successful teardown. This necessary parity repair stays within the migration goal; neither runtime nor its discovery is removed. The initial focused run is recorded at /tmp/ravel-k103-focused.log (ephemeral).

2026-10-28: The one fresh-context reviewer identified an actionable gap: cancellation bypassed the child readiness assertion in the admission timeout case. Require readiness and both owner streams in the parent cancellation branch too. The shared assertion now checks each payload on its actual rendered diagnostic stream and accepts Rhai's debug position and quoted string. No other material findings were reported. The reply-judgement repair is covered by the previously failing refusal/cleanup/cancellation cases; no second reviewer is needed.

2026-10-28: All 131 focused admission/acquisition cases passed. A deliberate wrong readiness marker then made both the acquisition timeout/remove/wait case and the admission cancellation/timeout case fail specifically with “deadline expired before select”; restore the marker before final verification. No deadline widening or retry was added. Ephemeral controls: /tmp/ravel-k103-readiness-control.log.

2026-10-28: The native run passed E08 and its holder/shielding variants but exposed five F03 broken-subject failures with the wrong discriminator. Libtest's banner panicked when the fixture deliberately omitted a stdout reader; its native-reader-only wrapper now redirects diagnostics to /dev/null (those rows assert ownership, not script diagnostics). The diagnostic mask fault also retriggered in Rhai's later filesystem reader; mark the end of diagnostic-reader creation at the existing reader_reports boundary and limit the injected mask fault to that phase. Preserve every named F03 detection assertion. The red mutant receipts in /tmp/ravel-k103-native.log (ephemeral) directly exercise both repairs; the runtime's signal or thread behavior is unchanged.

2026-10-28: After both fixture repairs, all six F03 rows passed with their named broken-subject detections. The complete task test:native-entry passed: 35 front cases, 22 runner cases and two Started-check API cases. E08 retained both-complete/escaped-holder, success/refusal and shielding breaks. No case was removed.

2026-10-28: Checked context.rs, settings.rs, choice.rs, limits.rs, frame.rs and run.rs: their unit fixtures are already Rhai or language-neutral JSON/data; no further executable policy port is needed. The remaining BUN_OPTIONS grant check still belongs to the production TypeScript path until k101. A source-wide Rust inventory classified remaining TypeScript references as the pending hostile suite and its shared support, never-evaluated discovery sentinels, frozen observations, pure legacy validator/path tests, or production/help/comments retained for k101. The three migrated suites contain no policy.ts reference; the same search found it in the parent revision and in the hostile suite, providing positive and cross-tree controls. No frozen observation was changed. Inventory and controls are ephemeral under /tmp/ravel-k103-*; the migration scope and classification are recorded here.

2026-10-28: Final verification passed on unchanged subjects: task check (formatting, shell checks, worker build/probes/types, runtime integrity, fake harness smoke, Clippy, terminal recovery and all 613 workspace tests), task test (all 613 tests; 48 skipped), task docs:examples (22 complete examples; 57 fragments intentionally not run), and task docs:excerpts (nine excerpts). Compared each named workspace outcome between check and test: all agree. No excerpt needed re-anchoring. Logs are target/check.ZwbhFU and /tmp/ravel-k103-{check,test,examples,excerpts,native-fixed}.log (ephemeral). The before/after SHA-256 manifests and path sets agree for every tracked non-Grove subject, including source, scripts, manifests/pins, book source and frozen observations, as listed in /tmp/ravel-k103-subject-files. No source changed during these measurements.

2026-10-28: Parent k98 audit: k102 delivered ordinary CLI/shared-support ports, and this child delivered native-entry/admission/acquisition ports with no test removals. Process/channel and JSON-only fixtures need no script conversion. All parent behavior, cleanup, native and confinement checks hold; both production runtime paths remain. Close rhai-runtime-fixtures-k98 and promote the remaining cutover context to k31. The ADR set already requires refusal retention and teardown before launch; this parity repair adds no new architecture decision. No live review declares k103 or k98 as its producer, so no Creator line is written.
