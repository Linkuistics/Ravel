# rhai-book-inside-k120

## Goal

Make the book's installation path and the README agree with the promotion of Ravel 2.0.0.
A new reader then installs the formula that Homebrew now offers as the default.

## Context

- Ravel 2.0.0 is now unversioned `ravel` in the tap `linkuistics/taps`.
  The tap's remote `main` is `06c7198`, “ravel v2.0.0 (promote ravel@2 v2.0.0)”.
  `Formula/ravel@2.rb` carries `deprecate! date: "2026-11-11"` with the replacement `ravel`.
- The readiness receipt names Grove 22.9.0 as the compatible caller, verified on 2026-11-13.
  The tap's `Formula/grove.rb` is 22.9.0 and depends on `linkuistics/taps/ravel`.
- The draft `rhai-book-inside-k36` found this state and corrected chapter 11. Read its running log first.
- These statements still describe the state before promotion. Each one names `ravel@2` as the installation, or says that unversioned `ravel` stays at 1.1.0.
  - Chapter 3, “Install Ravel and check its version”: the keg-only explanation, `brew install --skip-link linkuistics/taps/ravel@2`, the `PATH` line, the `brew trust` explanation and `brew --prefix ravel@2`.
  - Chapter 6: the default skill directory entry and the installed-layout entry that names `brew --prefix ravel@2`.
  - `docs/book/contract.md`, section 1, “Examples policy”, which names `linkuistics/taps/ravel@2`.
  - `README.md`, which says that unversioned `ravel` stays at 1.1.0 until callers accept 2.0.0.
- The `## 2.0.0` section of `CHANGELOG.md` records the release as it was cut. It is history, so keep it.
- Chapter 3 is the reader's first procedure, and Read 2 follows the proof of this node.

## Done when

- Chapter 3 installs unversioned `ravel` and checks its version. Each Homebrew step is checked against Homebrew's current behaviour or documentation, including whether the tap needs `brew trust` for a fully qualified unversioned formula.
- Chapter 3 keeps the reason for `HOMEBREW_NO_INSTALL_CLEANUP`, or the running log says why the step no longer needs it.
- Chapter 6, the contract and the README name no `ravel@2` installation as the default path. They may name `ravel@2` as the deprecated transition formula.
- Each changed procedure follows the Read 1 patterns in section 2 of the contract.
- `task docs:examples`, `task docs:excerpts`, `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass. The book builds without a warning.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.
- The last act cuts nothing, because copy-edit `rhai-book-inside-k119` already stands after this leaf.

## Notes

- Apply L81 and L83 of `docs/documentation-lessons.md`. Search for the transitional values, such as `ravel@2`, `1.1.0` and `keg-only`, not only for “until” and “yet”.
- Do not describe the owner's machine.
- Do not run a real Homebrew installation into the shared prefix to check a step. Use the documentation, `brew info`, or a disposable Homebrew prefix, and record which one.

## Decisions (running log)

**The structure brief is contract section 1, 2026-11-14.**
The node brief names `docs/book/contract.md`. Its section 1 states the reader, the section plan and the emphasis and exclusions.
Chapter 3's row in the plan says that the reader installs Ravel with Homebrew and checks its version.

**Homebrew evidence for chapter 3, 2026-11-14.**
The evidence comes from documentation, source and `brew info`. No installation ran.
The local Homebrew is 7.0.13. Its `docs/Tap-Trust.md` was last reviewed on 2026-10-25.

- `brew install user/repository/formula` adds an absent tap (`cmd/install.rb`, `tap&.ensure_installed!`).
  It then calls `Trust.trust_fully_qualified_items!`, which writes a trust entry for that one formula to the trust file.
- The tap's `Formula/ravel.rb` at `06c7198` has no dependency. `brew info --json=v2`, with auto-update off, reports no dependency, not keg-only and not deprecated.
  `ravel@2` is keg-only and deprecated, with the replacement `ravel`.
- The old `brew trust` reason was that installing a versioned formula also loads the unversioned one. That reason no longer applies.
  Chapter 3 therefore drops `brew tap` and `brew trust`, and it explains the trust that the fully qualified name grants.
- The manual page says that `brew install` upgrades an installed outdated formula, unless `HOMEBREW_NO_INSTALL_UPGRADE` is set. The upgrade then runs cleanup for that formula, unless `HOMEBREW_NO_INSTALL_CLEANUP` is set.
  The variable still protects an older `ravel` keg that a running session needs. Chapter 3 keeps it and now states that reason, which the previous text did not state.

**The published 2.0.0 book is stale for installation, 2026-11-14.**
`https://linkuistics.github.io/Ravel/v2.0.0/` returns HTTP 200, and its chapter 3 has the line `ravel 2.0.0`. That copy still installs `ravel@2`.
A release artifact cannot change, so chapter 3 tells a reader who followed it what changed. The next release publishes the corrected text.

**L81 and L83 search results, 2026-11-14.**
The search of all non-evidence files for `ravel@2`, `keg-only`, `keg_only`, `skip-link`, `1.1.0`, `brew trust` and “until callers” gave these classes:

| Location | Class | Action |
|---|---|---|
| Chapter 3, installation section | Stale present-tense claim | Rewrite for unversioned `ravel` |
| Chapter 3, “a checkout between releases has the placeholder” | Stale. Chapter 11 says the placeholder or the last release's version | Correct |
| Chapter 6, default skill directory and installed layout | Stale default path | Name `ravel`. Name `ravel@2` only as the deprecated formula |
| Chapter 1, “The book of Ravel 1.1.0 is on GitHub Pages” | Incomplete. The site lists `v1.1.0/` and `v2.0.0/` | Correct |
| Contract section 1, examples policy | Stale default path | Name `linkuistics/taps/ravel` |
| Contract section 1, omissions | A rule that stays true | Keep |
| README | Stale present-tense claim | Rewrite |
| Chapter 11, `release-delivery` figure | Rules and history, checked by k36 | Keep |
| `CHANGELOG.md`, the ADR, `docs/specs/release-channels.md` | History or the channel rules | Keep |
| `docs/specs/skill-discovery.md`, design captions | True statements about `ravel@2` or 1.1.0 | Keep |
| Scripts, tests, fixtures, evidence logs | Code, test data or history | Keep |

**The changes, 2026-11-14.**

- Chapter 3 installs `linkuistics/taps/ravel` with `HOMEBREW_NO_INSTALL_CLEANUP=1` and checks `ravel --version`. It explains the full name, why there is no `brew trust` step and why the variable stays.
  A new paragraph tells a reader of an earlier copy, such as the 2.0.0 book on GitHub Pages, to remove the `ravel@2` directory from `PATH` and check `command -v ravel`.
- Chapter 3 and the contract's examples policy now agree with chapter 11. A checkout between releases shows the placeholder or the last release's version.
- Chapter 6 names `brew --prefix ravel`. It names `ravel@2` only as the deprecated versioned formula.
- Chapter 1 says that the book of each release from 1.1.0 is on GitHub Pages.
- The contract's examples policy and the README name `linkuistics/taps/ravel`. The README names `ravel@2` as the deprecated transition formula.
- L85 and L86 record the two new lessons. L85 checks the reason of each step when its state ends.
  L86 tells the readers of a release copy that cannot change what changed.

**Gates, 2026-11-14.**
One script ran the gates in order on macOS 27, arm64, without `RAVEL_EXIT_FILE` and `HOMEBREW_NO_INSTALL_CLEANUP`.
A SHA-256 digest of the subjects was the same before and after. The subjects were the tracked and new files under `docs/book`, `scripts` and `crates`.
They also included the Taskfile, the Cargo files, `rust-toolchain.toml`, the README and the lessons record.

| Task | Result |
|---|---|
| `setup`, `docs:book:build` | Passed. The build log has no `WARN` or `ERROR` line |
| `docs:examples` | 27 passed, 0 failed, 73 fragments not run |
| `docs:excerpts` | 22 passed |
| `docs:figures` | 21 figures match |
| `docs:ste -- --strict`, `docs:breaks -- --strict` | No problem in 35 files. No run longer than 450 words in 11 files |

The first script passed the strict tasks with their arguments as one word, and `task` refused them as unknown. They ran again with correct arguments and passed.
A link check of the built chapters 1, 3 and 6 found 512 local links and no broken link. A copy with a planted bad anchor failed.

The installation fragment is not a complete example, so no gate runs it. Its evidence is the Homebrew source and documentation above.

**Checker findings outside the changed lines, 2026-11-14.**
The session's general prose checker reports three findings in chapter 6, one in the contract and 11 in the lessons record. Each was present at HEAD, outside the changed lines. The repository's gate, `task docs:ste -- --strict`, passes.

**No cut, 2026-11-14.**
Copy-edit `rhai-book-inside-k119` stands after this leaf, and its body already asks it to copy-edit these changes. This session cuts nothing. Nothing is handed forward.
