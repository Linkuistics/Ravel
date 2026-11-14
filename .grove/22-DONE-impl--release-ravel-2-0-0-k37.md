# release-ravel-2-0-0-k37

## Goal

Release Ravel 2.0.0 and keep the Ravel 1.1.0 binary that the running Grove driver uses. The release publishes a GitHub Release, the book on the Pages branch and the Homebrew formula.

## Context

- The input is `docs/book/src/11-deliver-ravel.md`, the Taskfile task `release`, and `scripts/release-*.sh`. The workstream makes no release before this leaf.
- All earlier leaves are done, and `task check`, `task test` and `task release:smoke` pass.
- `task release:major` refuses outside Ravel's default colocated jj workspace. The work of this workstream lives in the secondary workspace `Ravel.gh-issue-2`. Landing the work on main is a decision for the owner. Ask how, and do not choose a way yourself.
- The task ends with `brew upgrade ravel` and `brew test ravel`. The upgrade removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. The Grove driver that runs this session resolved the versioned binary `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` once, and the session ends through it. That version must stay until the driver restarts.
- A release publishes outside the machine, and nobody can retract it fully. The owner asked for the release in the requirements session. Ask once more, with the version and the list of what the task publishes, before you run the task.

## Done when

- The owner approved the landing of the work on main and the publication, and the running log records both answers.
- `CHANGELOG.md` has an entry for 2.0.0. It states that the policy format changed and links to chapter 4 for Rhai policies.
  It lists the new settings keys, the host functions and the removed Bun worker.
  The owner removed the migration guide in `rhai-book-use-k27` and chose a one-off manual configuration conversion.
- `task release:doctor` passes.
- `HOMEBREW_NO_INSTALL_CLEANUP=1 task release:major` completes from the default workspace. The tag `v2.0.0`, the GitHub Release with the three archives, the book on the Pages branch and the tap formula exist.
- `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` still exists after the task.
- If the task stops part way, follow the section "Recover a partial release without another bump" in chapter 11. Never cut a second version.
- The running log records the commands, the outputs that matter and the state of the three publications.

## Notes

- Do not run `brew cleanup`.
- If a step fails and the book does not cover the recovery, stop. Ask the owner, with the trade-off, a recommendation and the evidence.
- The release builds the book from the tag's own source. A book check that fails stops the release before anything is pushed.

## Decisions (running log)

- 2026-11-14: The owner asked to complete this Grove and publish its useful work before addressing GitHub issue 5.
  This resumes the historical tail. Reconcile each task with delivered work before repeating it.
- GitHub already publishes the non-draft `v2.0.0` release, dated 2026-11-12, with all three target archives.
  The release source is `ed6cedbf`, and `gh-pages@origin` is `eb704fbf`.
  The tap already promotes 2.0.0 through unversioned `ravel`.
  Keep that release and its fixed book. Do not run another major cut.
- Both `/opt/homebrew/Cellar/ravel/1.1.0` and `/opt/homebrew/Cellar/ravel/2.0.0` exist.
  `/opt/homebrew/bin/ravel --version` reports `ravel 2.0.0`.
  The current launch ends through the versioned 2.0.0 binary. The older launch instructions above describe history.
- The remaining diff from `main` contains book corrections, documentation lessons and evidence, with no runtime source changes.
  The owner still chooses how this work lands on `main`.
  Publication of corrected source can preserve the fixed 2.0.0 book.
  Publication of a corrected rendered book needs a separate version or an explicitly agreed publication route.
- The owner approved advancing `main` directly to the checked work.
  They chose publication of corrected source on GitHub while retaining the fixed 2.0.0 Pages book.
  This requires no version cut, archive replacement, tap update or installation change.

## Delivered-release reconciliation, 2026-11-14

The original release procedure above is historical: 2.0.0 was already published
before this session. This leaf verifies that delivery and lands the later book
corrections. It does not rerun `release:major` or replace released artifacts.

### Existing publications and installation

- `gh api repos/Linkuistics/Ravel/releases/tags/v2.0.0` returns a non-draft
  release published at `2026-11-12T10:00:02Z`:
  <https://github.com/Linkuistics/Ravel/releases/tag/v2.0.0>.
  Its three assets report these SHA-256 digests:

  | Archive | Bytes | GitHub-reported SHA-256 |
  | --- | ---: | --- |
  | `ravel-v2.0.0-aarch64-apple-darwin.tar.xz` | 1608220 | `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
  | `ravel-v2.0.0-aarch64-unknown-linux-gnu.tar.xz` | 1468208 | `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
  | `ravel-v2.0.0-x86_64-unknown-linux-gnu.tar.xz` | 1712168 | `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |

- `mise exec -- scripts/release-book.sh build v2.0.0` passes and builds from
  tagged source `ed6cedbf9a14cd088bea13fcfb8b86300cb6aaca`.
  The script's `verify v2.0.0` and `live v2.0.0` commands both pass:
  `gh-pages` holds the complete book and
  <https://linkuistics.github.io/Ravel/v2.0.0/> serves it.
- `HOMEBREW_NO_AUTO_UPDATE=1 brew info --json=v2 ravel` identifies
  `linkuistics/taps/ravel`, stable and linked version `2.0.0`, with both
  `1.1.0` and `2.0.0` installed. Each versioned binary reports its own version.
  No upgrade, cleanup, uninstall or reinstall ran in this session.
- `CHANGELOG.md` already records the Rhai format, chapter 4 remedy, new keys,
  host functions, removed Bun worker and formula promotion procedure.

### Fresh checks of the source being published

All commands below completed successfully in this session. Task commands ran
through `mise exec --`. Scratch output is under ignored `target/k37-*.log`.

| Command | Result |
| --- | --- |
| `task release:doctor` | All release prerequisites met. |
| `HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1 brew test ravel` | Installed formula test exits 0. |
| `task docs:book:build` | Book builds. |
| `task docs:examples` | 27 passed, 0 failed, 73 fragments not run, 0 page errors. |
| `task docs:excerpts` | 22 excerpts pass. |
| `task docs:figures` | All 21 figures match their sources. |
| `task docs:ste -- --strict` | 0 problems across 35 files. |
| `task docs:breaks -- --strict` | 0 runs over 450 words across 11 chapters. |
| `shasum -a 256 -c docs/evidence/rhai-book-proof-k136/subjects-after.sha256` | All 437 proof subjects match. |

The first STE and prose-break invocations passed `docs/book/src` as a file and
failed with `EISDIR: illegal operation on a directory, read`. The final commands
above omit file arguments and check their default complete surfaces.

The unchanged proof inputs support reuse of the committed k136 proof; this is
not another editorial proof or a reader-comprehension result. Read 2 remains
abandoned by the owner's earlier decision. No runtime implementation changed
from `main`, so this session does not repeat the full Rust test and release-smoke
suites or claim that it ran them.

### Landing and remaining scope

The approved landing fast-forwards `main` from
`d6d817654daf229f3b2edc3d51cfeee52ec565c3` through the checked documentation and
evidence and this retirement. It preserves the historical `out.err` transcript
and its undescribed ancestor rather than rewriting the preceding task history.
`jj git push --bookmark main --allow-empty-description` permits that existing
description without changing its content.

The following leaves still own installation sandbox verification and personal
configuration reconciliation. They must inspect current state before repeating
historical upgrade or conversion instructions. Neither leaf is completed by
this session.
