# owner-policy-conversion-k39

## Goal

Convert the owner's local configuration to Rhai. The owner's installation of Ravel 2.0.0 then evaluates `~/.config/ravel/policy.rhai`, and the sessions of the running 1.1.0 driver still work.

## Context

- The input is the migration guide in the book, the examples from `rhai-example-policies`, and `~/.config/ravel/policy.ts`. The leaf `rhai-example-policies` translated a policy of the same shape.
- The owner's `policy.ts` has a harness table. It grants the main repository of a jj workspace, and it reads a workspace file through `node:fs`. It uses none of Grove's helpers.
- Sessions under the 1.1.0 driver read `policy.ts`. Ravel 2.0.0 reads `policy.rhai` and ignores `policy.ts` when both exist. Keep both files until the driver restarts.
- Ravel 1.1.0 refuses an unknown key in `settings.json` as `settings_invalid`. Both versions read the same settings file. Do not add `endpoints`, `harnesses` or `budgets` to `~/.config/ravel/settings.json` while the 1.1.0 driver runs. Record the new keys that the owner may want, and add them after the restart.
- Writing in `~/.config/ravel` changes the owner's live configuration. Show the owner the new file and the test results, and ask before you write.

## Done when

- A candidate `policy.rhai` exists outside `~/.config/ravel`, in the scratchpad.
- The candidate runs under the installed Ravel 2.0.0 and the versioned 1.1.0 binary in a sandbox home, with fake harnesses on a private `PATH`. For every kind that the policy lists, the argument vectors are equal. The test includes a secondary jj workspace, so the `--add-dir` grant of the main repository is checked.
- The owner approved the candidate, and the leaf copied it to `~/.config/ravel/policy.rhai`. The file `policy.ts` is unchanged.
- The installed `ravel` evaluates the new file in a sandbox that copies the real configuration directory. The versioned 1.1.0 binary still evaluates `policy.ts` in the same sandbox.
- `settings.json` is unchanged.
- The leaf reports what remains for the owner after the driver restarts. These are the removal of `policy.ts`, the removal of the 1.1.0 version with `brew cleanup ravel`, and any new settings keys.

## Notes

- Do not launch a real harness with the new policy in this session. The sandbox tests use fake harnesses.
- If the translation needs a host capability that Ravel 2.0.0 lacks, stop and report the gap. Do not work around it with a copy of a personal file.
