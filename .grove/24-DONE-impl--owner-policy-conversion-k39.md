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

## Decisions (running log)

- 14 November 2026: Reconciled the historical conversion request with the live directory before making changes. It contains `config.rhai` and `settings.json`, but no `policy.ts`. The Rhai script routes its selected Codex/Claude arguments through `bili`; the settings already name mandatory skill guidance. This launch uses the versioned 2.0.0 ending command. The original assumptions of an unconverted policy and a 1.1.0 driver no longer describe this session.
- Leave the live configuration and both Homebrew versions unchanged. Ask the owner whether verification of the existing conversion can satisfy this leaf. The original TypeScript is unavailable from the live directory, so no exact old/new argument comparison or preservation claim can be made. While awaiting that decision, check the current script's complete routing table under the installed 2.0.0 binary with fake programs, private homes and a sandbox denying the real home.
- Verified the existing conversion under the installed 2.0.0 binary: all 24 named kinds passed in plain, primary jj and secondary jj directories (72 exact argument checks), including the main-repository grant, the `bili` wrapper and exact caller text within Ravel's skill preamble. The unknown kind refused before launch. The copied current settings refused under 1.1.0 as `settings_invalid`. Digests of the live configuration and both installed kegs match before and after. Evidence and the one-off check are in `docs/evidence/owner-policy-conversion-k39/`. Kill-attempt diagnostics are retained; no descendant-cleanup claim is made.
- The scope question remains unanswered. Recommend accepting the existing conversion on this current-behavior evidence and explicitly waiving the unavailable historical comparison; alternatively, obtain the original `policy.ts` before making that comparison. Retirement and commit remain pending the owner's answer. There is no live file to install or `policy.ts` to remove; leave older-keg cleanup and optional settings changes to the owner. Add the distinction between current-behavior evidence and transition evidence to the documentation lessons.
- The owner answered yes to closing this leaf on the current verification and waiving the unavailable historical comparison. This settles the completion scope: accept the already-installed configuration and its 72 passing route checks, with no claim about historical equivalence, prior file preservation or continued 1.1.0 compatibility with the current shared settings. No live configuration write, settings change or older-keg removal is needed. Retire the leaf and commit the evidence and L87.
