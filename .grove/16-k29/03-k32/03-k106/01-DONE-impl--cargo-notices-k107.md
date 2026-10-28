# cargo-notices-k107


## Goal
Generate the Rust dependency notices from locked Cargo metadata with a pinned checkout-local tool. Refuse unapproved licences, document the smartstring exception and establish the notice path that the installed-layout child consumes.



## Context
The policy-evaluation spec's installed-layout contract and the parent's delivery inventory are authoritative. Use cargo-about 0.8.4, installed under `target/tools/cargo-about`, and its version-specific official configuration and template documentation. Existing Bun notices and all installed/release behavior stay with k108.

## Done when
- `task notices` creates `target/notices/THIRD-PARTY-NOTICES.md` from the locked dependency graph, with dependency names, versions and full licence texts.
- The allowlist refuses an unapproved licence. MPL-2.0 is approved only for smartstring, with its compact_str exclusion explained.
- `task notices:test` runs real Cargo/cargo-about against isolated fixtures, proving acceptance, refusal, preservation of a previous output after failure, version-pin enforcement and repeatability.
- The generator and Taskfile pass repository checks. The final installed notice path reaches k108 and the k29/root briefs for k34.

## Notes
The graph CLI refused startup because a pre-coordination or unverified generation was active. No project/generation could be read; all discovery in this bounded script/config change uses exact source reads and path/config searches.

## Plan
1. Add a failing real-tool fixture test for accepted and refused licences, atomic output and pin enforcement.
2. Add the checkout-local setup/generation script, minimal allowlist and Markdown template; expose setup, generation and tests through the Taskfile.
3. Run the fixture controls and generate the real dependency notices twice with unchanged subjects. Verify the dependency inventory and licence texts, then run repository checks.
4. Promote the notice contract, retire this child and seal its jj change. Leave the coordinated layout child live.

## Decisions (running log)

- 2026-10-28: Pin cargo-about 0.8.4 under the checkout's target directory; install with Cargo's locked published dependency set. Generation uses local licence data with ClearlyDefined disabled, a locked/offline Cargo graph, and the union of the three release targets. Include build dependencies; exclude dependencies used only for development.
- 2026-10-28: The installed notice path is `share/ravel/notices/THIRD-PARTY-NOTICES.md`; the generator writes `target/notices/THIRD-PARTY-NOTICES.md`. k108 copies that generated file into the archive/formula/install tree. The generator stages output before replacing a prior valid notice.
- 2026-10-28: The real-tool fixture suite now proves distinct copyright texts under one SPDX licence both survive, build-only dependencies remain, dev-only dependencies are excluded, GPL refuses, MPL approval applies only to smartstring, missing lockfiles refuse, wrong tool versions refuse, and failures preserve the previous notice and remove temporary files. Prefer these executable counterexamples to a second prose read; no in-session reviewer is needed for this bounded tool integration.
- 2026-10-28: Baseline `task check` passed formatting, shell lint, book Bun integrity, fake harness, Clippy and terminal recovery, then failed the existing one-second `worker_reap_admission_success_gate_both_front` case (selection_timeout instead of worker_failed). The independent cargo-about install was compiling concurrently. Do not change deadlines, retries or runtime code; rerun the final check without competing compilation.
- 2026-10-28: Final `mise exec -- task check` passed formatting, shell lint, book Bun integrity, fake harness, Clippy, terminal recovery and all 602 Rust tests (48 skipped), including the baseline's failing admission case. Complete output is `/tmp/ravel-k107-final-check.log`; the check's persistent local log is named in its opening output. No runtime code, timeout, retry or test selection changed.
- 2026-10-28: Final `task notices:test` passed through the Taskfile. Two subsequent `task notices` runs produced byte-identical Markdown, SHA-256 `c8ffbb1e5730585f4a1607179d43a0ee5943a191330645e7b7e0867e291c95c9`. Independent `cargo tree --frozen --workspace --target <target> --edges normal,build --prefix none --format '{p}'` inventories for each of the three release targets, normalized as name/version pairs and unioned, match all 69 generated components item by item. Removing smartstring from a scratch inventory made the same comparison fail by name. Separate scratch mutants also failed as expected for collapsed licence texts, global MPL acceptance and inclusion of development-only dependencies.
- 2026-10-28: The generation measurement digested subjects before and after, with no differences: Cargo.toml, Cargo.lock, rust-toolchain.toml, Taskfile.yml, about.toml, LICENSE, notices.sh, notices.test.sh, release-common.sh, notices.md.hbs, the pinned cargo-about executable, and every regular file under each Cargo-metadata package directory (node_modules pruned). The resulting 7,530 file records cover workspace and registry source/manifest/licence inputs. `target/k107-notices-verification/` holds the per-file digests, Cargo metadata, component inventories, deliberate omission diff, first generated output and command logs; the second output is `target/notices/THIRD-PARTY-NOTICES.md`. These are local test outputs, not new tracked state. The measurement's final output is `/tmp/ravel-k107-verification.log`.
- 2026-10-28: This child meets its notice-generator contract. It publishes nothing and changes no installed layout. k108 remains live with the original release/install/formula/target-floor/book acceptance criteria and retired-artifact consumer inventory. k106 and k32 therefore remain open; no parent close is claimed. The final notice path is promoted through the k32/k29/root brief chain for k34.
