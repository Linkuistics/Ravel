# Codex isolation-flag probe

This is harness evidence, not an evaluated repetition. The prompt was a
throwaway request to report startup sources and did not reproduce or refer to a
frozen campaign case.

## Question and result

For `codex-cli 0.150.1`, `--ignore-user-config` and `--ignore-rules` do not form
a no-instruction boundary:

- `$CODEX_HOME/skills` is not suppressed. A launch against a fresh writable
  probe home populated `skills/.system` while both flags were present.
- `$CODEX_HOME/AGENTS.md` is not suppressed. The `rust-v0.150.1` global
  instruction provider reads `AGENTS.override.md` or `AGENTS.md` directly from
  `CODEX_HOME`; the exec flag is passed only to the configuration loader.
- `$CODEX_HOME/hooks.json` is not suppressed at discovery. Ignoring
  `config.toml` leaves an empty user configuration layer carrying the same
  `CODEX_HOME/config.toml` path, and hook discovery scans that layer's directory
  for `hooks.json`. Actual hook execution remains subject to the hooks feature
  and trust policy, so absence—not a flag inference—is the campaign control.

The compensating preflight used a fresh home containing only an authentication
symlink and added `-c skills.bundled.enabled=false`. After the CLI initialized,
`skills/`, `AGENTS.md`, `hooks.json`, and `config.toml` were all absent. The
rubric therefore requires sealed purpose-built homes, an absent instruction and
hook surface, and bundled-skill disablement.

## Method and limitation

The local commands scrubbed the parent session's `CODEX_*`, `GROVE_SIGNAL_FILE`,
and Herdr control variables before launching the child. The child initialized a
thread but this session's network sandbox denied access to the model service, so
it produced no final assistant message. The transcript records that failure and
the filesystem preflight; it cannot be mistaken for a scored sample or for a
successful end-to-end model-visible sentinel test.

The exact-version source audit used OpenAI's `rust-v0.150.1` tag:

- `codex-rs/exec/src/lib.rs` passes the two flags only as
  `LoaderOverrides.ignore_user_config` and
  `ignore_user_and_project_exec_policy_rules`.
- `codex-rs/config/src/loader/mod.rs` replaces ignored user TOML with an empty
  user layer while preserving its file path.
- `codex-rs/codex-home/src/instructions/mod.rs` independently reads global
  `AGENTS.md` from `CODEX_HOME`.
- `codex-rs/hooks/src/engine/discovery.rs` scans every config layer's directory
  for `hooks.json`.

Source URLs:

- <https://github.com/openai/codex/blob/rust-v0.150.1/codex-rs/exec/src/lib.rs>
- <https://github.com/openai/codex/blob/rust-v0.150.1/codex-rs/config/src/loader/mod.rs>
- <https://github.com/openai/codex/blob/rust-v0.150.1/codex-rs/codex-home/src/instructions/mod.rs>
- <https://github.com/openai/codex/blob/rust-v0.150.1/codex-rs/hooks/src/engine/discovery.rs>
