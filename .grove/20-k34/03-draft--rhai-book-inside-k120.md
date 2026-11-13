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
