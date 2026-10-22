# Changelog

## Unreleased

## v1.1.0

- Correct five failure paths. Only the behaviour after a failure changes. The
  command-line arguments, the owner configuration and the results of a
  successful session stay the same.
  - Ravel keeps a cancellation when the wait for the managed process fails.
    It also keeps the cancellation when the reap of that process fails.
  - If Ravel cannot check that it reaped the policy worker, the session stops
    before the managed process starts.
  - After Ravel acquires only some of the resources for a session, it reports
    each failure of the cleanup that follows.
  - The terminal recovers after the exec of the managed process fails.
  - When its process starts, Ravel checks each signal and terminal
    transition.
- Add the field `operationReports` to the JSON notice that `--json` prints. The
  field is an array of the reports that Ravel kept while it supervised the
  managed process. The array is empty when Ravel kept no report.
- Reword the help text of `ravel end-session`. The LLM sends the result, and
  Ravel keeps the first result that it receives. The behaviour does not change.
- Describe `--policy-env` in the help text of `ravel start-session`.
- Add the book, a user guide for Ravel. Each release publishes it on GitHub
  Pages.
- Measure Ravel on aarch64 macOS and aarch64 GNU/Linux. The x86-64 GNU/Linux
  target runs only under emulation. The measurements of that target come from
  the emulation.

## v1.0.0

- Declare the two-verb public CLI stable: `ravel start-session` and
  `ravel end-session RESULT`, with opaque caller-owned results, owner policy
  routing and confinement behaving exactly as released in 0.1.0.
- Commit to semantic versioning for that CLI: compatible changes arrive in minor
  and patch releases; incompatible changes require a new major version.
- Ship the same matching front and private worker, generic SDK/examples and
  runtime notices as 0.1.0, with no source or behavior change.

## v0.1.0

- Introduce the independent Ravel session process manager with `start-session`
  and `end-session RESULT`, preserving opaque results and owner policy routing.
- Ship a matching front and private worker, generic SDK/examples and runtime
  notices without a Grove or host TypeScript runtime dependency.
- Add independent release tasks, exact three-target archives, installed-layout
  smoke checks, Linux glibc/CPU-floor controls and a tested Homebrew formula.
