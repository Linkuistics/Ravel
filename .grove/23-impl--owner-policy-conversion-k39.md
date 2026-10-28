# owner-policy-conversion-k39

## Goal

Convert the owner's local configuration to Rhai. Ravel 2.0.0 then evaluates `~/.config/ravel/config.rhai`, and the running 1.1.0 driver still works.

## Context

- The inputs are the Rhai examples from `rhai-example-policies` and `~/.config/ravel/policy.ts`.
  That leaf translated a policy of the same shape.
- The owner made this a one-off manual conversion in `rhai-book-use-k27`.
  The book has no migration chapter and assumes no TypeScript knowledge.
  Chapter 4 teaches the Rhai capabilities that the conversion uses.
- The owner's `policy.ts` has a harness table. It grants the main repository of a jj workspace, and it reads a workspace file through `node:fs`. It uses none of Grove's helpers.
- Sessions under the 1.1.0 driver read `policy.ts`. Ravel 2.0.0 reads `config.rhai` and ignores `policy.ts` when both exist. Keep both files until the driver restarts.
- Ravel 1.1.0 refuses an unknown key in `settings.json` as `settings_invalid`. Both versions read the same settings file. Do not add `harnesses` or `budgets` to `~/.config/ravel/settings.json` while the 1.1.0 driver runs. The owner's configuration gains harness entries for Claude Code and Codex, and it gains no endpoints. Record the new keys that the owner may want, and add them after the restart.
- Writing in `~/.config/ravel` changes the owner's live configuration. Show the owner the new file and the test results, and ask before you write.

## Done when

- A candidate `config.rhai` exists outside `~/.config/ravel`, in the scratchpad.
- Compare the candidate under Ravel 2.0.0 with the original `policy.ts` under the versioned 1.1.0 binary in private homes with fake harnesses.
  For every listed kind, the argument vectors are equal. Include a secondary jj workspace to check the main repository's `--add-dir` grant.
- The owner approved the candidate, and the leaf copied it to `~/.config/ravel/config.rhai`. The file `policy.ts` is unchanged.
- The installed `ravel` evaluates the new file in a sandbox that copies the real configuration directory. The versioned 1.1.0 binary still evaluates `policy.ts` in the same sandbox.
- `settings.json` is unchanged.
- The leaf reports what remains for the owner after the driver restarts. These are the removal of `policy.ts`, the removal of the 1.1.0 version with `brew cleanup ravel`, and any new settings keys.

## Notes

- Read `configuration-names-k92` for the final public names. The old settings key `policyEnv` becomes `configEnv`.
  Ravel 1.1.0 rejects the new key. Keep the active shared settings unchanged until the driver restarts, then apply any required settings conversion.
  No personal settings file existed during Read 1's check. Recheck rather than assuming it is still absent.
- Do not launch a real harness with the new policy in this session. The sandbox tests use fake harnesses.
- If the translation needs a host capability that Ravel 2.0.0 lacks, stop and report the gap. Do not work around it with a copy of a personal file.
