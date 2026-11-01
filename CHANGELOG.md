# Changelog

## Unreleased

- Deliver Ravel as one executable with three Rhai examples and Rust dependency
  notices under `share/ravel/`. Remove the retired Bun worker, TypeScript SDK
  and their delivery tooling. Installation, archives and the Homebrew formula
  now exercise Rhai directly.

- Remove retired TypeScript worker prerequisites from development setup, builds,
  checks and native-entry checks. Keep the checkout-local Bun for book tools.
- Retire the frozen native-entry campaign drivers and their Linux Git-object
  staging. Preserve historical receipts and current Rust, Rhai and native-entry
  checks.
- Move configuration scripts and their request to schema 4. The request gains
  `originalPrompt`, the caller's prompt unchanged, and `skills`, which is `()`.
  `prompt` holds the same bytes as `originalPrompt`. A script that exports
  `SCHEMA_VERSION` 3 refuses as `unsupported_version`, exit 3. Export 4, and
  read `request.originalPrompt` where the script analyses the task. The
  shipped `llm.rhai` sends `originalPrompt` to its classifier. The JSON routing
  notice gains the member `skills`, which is `null`.
- Check the evaluate frame before the worker receives it. The frame bound is
  38,541,312 bytes, and every request within the public bounds fits it. A
  larger frame refuses as `malformed_input`, stage `cli`, exit 2, and the
  refusal names the largest members. Only a very long kind, working directory
  or task-file path can pass the bound.

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
