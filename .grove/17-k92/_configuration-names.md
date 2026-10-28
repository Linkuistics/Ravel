# configuration-names-k92 — brief


## Goal

Use configuration names throughout Ravel's 2.0.0 interface and current product files.
The owner asked to eliminate “policy” from filenames and arguments during Read 1.



## Context

- F18 and the owner's exact words are in `rhai-book-use-k74`. The owner then asked the session to continue.
  The consistent public-interface scope below is the session's interpretation.
- `typescript-removal-k29` precedes this leaf. It freezes the old comparison fixtures and removes the old worker before this rename.
- Inspect the current authority, CLI, settings, environment, notice, refusal and Rhai-worker code and all consumers.
  The requirements session had no usable graph generation and used direct source reads.
- The correction under `rhai-book-use-k26` follows this leaf. Keep executable examples and exact excerpts working during the rename.
  That draft owns explanatory restructuring and F17. This leaf owns the mechanical reconciliation needed for working consumers.
- The agreed test seam is the real CLI in a private home with fake programs on a private path.
  Read 1 put the rename cases below to the owner through that same seam.

## Done when

- The default script is `$HOME/.config/ravel/config.rhai`. `--config PATH` retains arbitrary-path semantics.
  An explicitly named `.rhai` file can have any basename. That is not a default-path alias.
- `--config-env` replaces `--policy-env`, and `configEnv` replaces the settings key `policyEnv`.
  Old option and settings spellings are not accepted as aliases in 2.0.0.
- JSON output uses `config`, `configEnvironment` and `configCode` in place of their old counterparts.
  Ravel-owned diagnostic codes and location prefixes use `config_` and `config.`.
  Output prefixes use `config stdout:` and `config stderr:`. Messages, remedies and help use configuration terms.
- The evaluation-run marker is `RAVEL_CONFIG_RUN`. Re-entry refusal, grants, environment scrubbing and stop order retain their behaviour.
  Test the marker through the fake-harness CLI seam, including refusal before evaluation.
- The renamed failure document has schema version 2, because version 1 already names the old fields.
  Routing-notice schema version 2 and Rhai script schema version 3 retain their numbers: those new interfaces have not shipped.
  Reconcile every producer, consumer, example and schema statement.
- Current Ravel-owned source and documentation filenames that name this concept use configuration terms.
  Include the retained `policy.rs` module, configuration-evaluation spec, affected ADR slugs, design paths and chapter 4's filename.
  Use `04-write-a-configuration-script.md` for the chapter. Reconcile current links, citations, tasks and excerpt paths.
- Current prose, examples, release checks, help and public names agree. Classify residual old terms by ownership and meaning.
  Historical evidence, old-input refusal cases, third-party identifiers and stable Grove handles remain exact.
  Do not change arbitrary caller prompts, results, filenames or owner-supplied codes because their values contain the word.
- Preserve the frozen pre-cutover observations. Make any comparison translation of renamed fields explicit and tested.
  Do not regenerate historical fixtures to match the implementation. Their mutation control still detects a behavioural change.
- CLI tests cover default and explicit paths, both forms of environment grant, text and JSON refusals, notice fields and retired spelling rejection.
  Check exact opaque-result preservation. Existing cancellation and cleanup tests retain their coverage.
- `task check`, `task test`, complete book examples, example-checker controls and exact excerpts pass.
  Run relevant release/help checks for changed interfaces. Record results and platform limits.

## Notes

- This changes names, not routing or supervision. Reconcile the current spec and ADR set in place, with their consumers.
- Do not change the installed 1.1.0 executable, the active Grove ending command or the owner's live files.
  `homebrew-upgrade-k38` and `owner-policy-conversion-k39` retain that handover.
- The requirements session found no personal `settings.json`. Recheck at conversion time.
  Do not add `configEnv` to settings that the running 1.1.0 driver still reads.
- If this proves too large, decompose explicitly and preserve working consumers at each boundary.
  Remove any temporary compatibility form before this node closes and the book correction uses the final interface.

## Decomposition

The interface crosses discovery, environment authority, serialized observations and
the current documentation set. Each child completes one consumer-visible change
with its tests and documentation; no compatibility alias is introduced.

- `configuration-names-k111`: change default discovery to `config.rhai`, preserve
  explicit arbitrary basenames and the historical TypeScript refusal, and update
  default-path consumers.
- `configuration-names-k112`: rename environment grants and the harness-run marker,
  with settings/flag refusal, re-entry and scrubbing checks and current consumers.
- `configuration-names-k113`: rename diagnostics and JSON fields, advance the
  failure schema, and explicitly translate frozen comparison observations.
- `configuration-names-k114`: reconcile remaining current source/document filenames
  and prose, classify residual old tokens, and run the node's complete checks.

## Decisions (running log)

2026-10-28: Split the original leaf into four bounded implementation sessions.
Discovery is an independent vertical slice: its flags and output fields keep their
current names until their respective children change them. The node must satisfy
the original full contract before the book correction runs. The installed 1.1.0
driver and the owner's live configuration stay unchanged.
