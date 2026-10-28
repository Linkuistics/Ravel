# retire-frozen-native-tooling-k109


## Goal
Remove the retired frozen native-entry campaign's executable tooling and dependency closure. Preserve current Rust/Rhai/native-entry/Linux checks and historical receipts.



## Context
The removal is authorised by k108/k106/k32. k101 already removed the campaign from `test:native-linux`. Codebase Memory CLI cannot start: `CBM CLI could not start because a pre-coordination or unverified CBM generation is active; close all CBM sessions and commands, then retry`. No graph evidence is claimed; direct source and configuration searches supply the consumer inventory.

## Done when
- Incumbent overlay/patches and native-incumbent/native-campaign/native-r01 scripts are removed, with campaign-only profiles and host Git-object staging reconciled.
- Historical evidence is preserved and current documentation distinguishes the archived campaign from available commands.
- Current repository checks, book examples/excerpts and their checker controls pass; current Linux arm64 checks run without host Git objects. Disclose emulation coverage precisely.

## Notes

## Preservation ledger

| Surface | Consumers and evidence | Action | Verification |
|---|---|---|---|
| Frozen campaign executables and overlay | Mutually calling scripts; no Taskfile entry remains; k101 explicitly retires the campaign | Delete the three drivers and `scripts/native-incumbent/` | Enumerate references across the checkout, classify historical hits |
| Campaign nextest profiles | Only `native-campaign.sh` selects them | Remove campaign profiles; retain default timing override | `task check`, `task test:native-entry` |
| Linux host object mount and alternates | Only incumbent assembly needs historical Git objects | Remove host object mount, guest bind mount and alternates; retain standalone guest Git initialisation | Real arm64 image/check run; inspect container mounts |
| Docker packages | `patch` served incumbent assembly; Git/Perl remain needed by book checker tooling | Drop explicit patch dependency; retain Git/Perl and current tools | Rebuild image and run current Linux checks |
| Historical campaign receipts, plan and narrative | Cited by native-entry evidence, design/spec/book | Preserve receipts/plan byte-for-byte; add retirement context to narrative | Before/after SHA-256 comparison |
| Installed/release layout | Still consumes retired worker, separately owned by k110 | Preserve in this child | No delivery edit; k110 keeps full acceptance contract |

## Plan

1. Capture the current repository check and receipt/fixture digests, and classify all consumers.
2. Remove retired executables/profiles and object staging; correct the directly affected book guidance and evidence introduction.
3. Run repository/native-entry/book checks and real Linux arm64 validation; inspect remaining references and unchanged historical digests.
4. Retire only k109, keeping k110 live, and seal one focused jj change.

## Decisions (running log)

- 2026-10-28: Preserve Git and Perl in the Docker image because examples/excerpts use both. Remove only the explicit `patch` package from the old assembly dependency set. Preserve standalone Git initialisation for checkout-aware tooling; historical objects and alternates are no longer needed.
- 2026-10-28: Baseline `mise exec -- task check` passes all 602 tests (48 skipped), with formatting, shellcheck, book-Bun integrity, fake-harness checks, clippy and terminal recovery passing. The deletion inventory is the three drivers plus all eight overlay/patch files; full-checkout references outside `.grove`, generated state and archived receipts reduce after removal to the dated native-entry evidence narrative. Keep that cited narrative and its case/plan JSON, add explicit retirement context, and remove the book's obsolete available-command rows. The same search was observed dirty in scripts/profiles before deletion and still finds archived receipt hits afterwards. No runtime Rust source or public configuration interface changes.

## Verification

- Post-change `mise exec -- task check`: formatting, shellcheck, book-Bun integrity, fake harnesses, clippy and terminal recovery pass; 602 tests pass, 48 skip. The baseline had the same test outcome. `task test:native-entry` passes its 35 front, 22 runner and two start-check cases.
- `task docs:examples` passes all 22 complete examples; `task docs:examples:test` passes its controls. `task docs:excerpts` passes all nine excerpts; `task docs:excerpts:test` passes its controls and kills all 24 mutants. `task docs:book:build` succeeds without a warning.
- Rebuilt both Linux images through `task native-linux:image PLATFORM=arm64` and `PLATFORM=amd64`. `task test:native-linux PLATFORM=arm64 OUT=target/k109/linux-arm64` completes every step with exit 0: minimum-Rust build/clippy/fmt, setup, 258 Rhai tests, 35/22/2 native-entry cases, check and test (602 pass, 48 skip each), and unchanged source digests. Docker inspection shows no mounts; the guest has neither `/srv/git` nor a Git alternates file.
- The rebuilt amd64 image stages successfully through the actual QEMU path at the Nehalem CPU floor, with its guest mappings below `0x800000000000` and a vDSO. Guest `uname`, Rust 1.99.1, nextest and standalone Git initialisation work. Its only container mount is read-only `/inputs`; the guest has no host Git object path/alternates or retired drivers. This is an emulated staging/tool smoke, not a full amd64 suite, native Linux qualification or release-floor validation. k110 retains all release-target acceptance criteria.
- Compared SHA-256 maps for every file in `docs/evidence` before/after (3,277 unchanged), the three frozen TypeScript observation fixtures, and `docs/native-entry-cases.json`/`docs/native-entry-campaign-plan.json` (unchanged). The public `task --list` output is byte-identical. Full source maps before/after the verification runs agree for Cargo manifests/lock/toolchain, Taskfile, `.cargo`, `.config`, scripts, crates, docs, CONTEXT and README, excluding generated target/node_modules/DS_Store files. CHANGELOG was checked separately. The Linux run's own subject maps also agree. Logs and maps are under `target/k109/`.
- Direct-source reference inventory: the removed drivers/overlay and campaign profiles have no remaining consumers in `scripts` or `.config`. A full-checkout search with hidden files, excluding VCS/generated state, `.grove` and archived receipts, leaves only historical mentions in `docs/native-entry-development-evidence.md`, whose introduction now explains retirement. The same search was dirty against the removed subjects before editing and remains dirty in archived receipts. The cited evidence, case manifest and campaign plan stay. Codebase Memory remained unavailable as recorded above; no graph completeness claim is made.
- Inspected the changed Linux/config/doc content and full deletion inventory. No installed/release consumer or live owner file changed. Both validation containers were removed after checking. No additional reviewer was needed for this deletion/plumbing boundary; current executable checks cover its dependency removal. k108 stays open with k110 next.
