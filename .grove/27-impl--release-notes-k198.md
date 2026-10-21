# release-notes-k198

## Goal

Write the notes of release 1.1.0 under `## Unreleased` in `CHANGELOG.md`.
Then run the release checks that change nothing. Do not cut, push or tag a
release.

## Context

- The owner's delivery decisions are in the log of `component-work-scope-k57`
  (question 4) and in `Linkuistics/Ravel#3`.
- These changes since 1.0.0 matter to users:
  - Failure-path corrections. Cancellation survives wait and reap errors. An
    unconfirmed worker reap stops the session before the managed process
    starts. Cleanup after a partial acquisition reports its failures. The
    terminal recovers after a failed exec. Ravel checks each signal and
    terminal transition at its native entry.
    `docs/completion-failure-design-evidence.md` and
    `docs/native-entry-development-evidence.md` give the exact scope.
  - The JSON notice has a new field, `operationReports`.
  - The `end-session` help has new words, and `--policy-env` has a
    description.
  - The book is new. Each release puts it on GitHub Pages.
  - Measurements cover aarch64 macOS and aarch64 GNU/Linux. x86-64 GNU/Linux
    runs only under emulation (`Linkuistics/Ravel#1`).
- The earlier entries in `CHANGELOG.md` record released history. Keep them
  unchanged.
- The book's chapter “Deliver Ravel”, section “Prepare a version cut”, gives
  the rules for the notes. There is one `## Unreleased` heading. The release
  keeps a nonempty body byte for byte. Notes must not contain internal
  details.

## Done when

- `CHANGELOG.md` has one `## Unreleased` heading. Its notes cover the changes
  above in words for users, with no task handle.
- The new notes pass `ste-lint.py`.
- `task release:doctor` and `task release:lifecycle:test` ran. The log
  records their results, and each missing prerequisite with its remedy. Before
  the first release, the doctor reports the disabled Pages setting as a note.

## Notes

- The owner reads the notes before `task release:minor` runs, as
  `Linkuistics/Ravel#3` describes.
- Do not run `release:patch`, `release:minor` or `release:major`. They have
  public effects, and the delivery follows the finish session.
