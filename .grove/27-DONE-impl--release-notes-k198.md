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

## Decisions (running log)

- The notes of 1.1.0 stand under the single `## Unreleased` heading of
  `CHANGELOG.md`. They replace the earlier help-rewording bullet and cover the
  five failure-path corrections, `operationReports`, the `end-session` help,
  the `--policy-env` description, the book and the measured targets. They name
  no task handle. The entries from `## v1.0.0` down stay unchanged: the diff
  against tag `v1.0.0` has no removed line.
- `operationReports` is described as the book's reference chapter describes
  it: an array, empty when Ravel kept no report, present when the runner
  returned its account of how the job ended.
- `ste-lint.py` passes the new notes: 0 hard findings in 216 words. A mutated
  copy with a semicolon and an American spelling failed with 2 hard findings,
  so the check can fail. The editor hook still reports 3 findings, all in the
  released `v0.1.0` and `v1.0.0` entries. They stay, as the brief requires.
- `task release:doctor` passed with every prerequisite met. No prerequisite is
  missing, so no remedy is due. It reports the disabled Pages setting as a
  note: Pages is not enabled for `Linkuistics/Ravel`, and the first release
  needs the one-time setup in chapter 11, "Set up Pages for the first
  release".
- `task release:lifecycle:test` passed (exit 0). Its log is
  `target/release-lifecycle.MWV2lS`.
- `release:patch`, `release:minor` and `release:major` did not run. The owner
  reads the notes before `task release:minor` runs.
