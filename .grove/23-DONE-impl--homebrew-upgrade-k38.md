# homebrew-upgrade-k38

## Goal

Make the owner's Homebrew installation run Ravel 2.0.0, and keep Ravel 1.1.0 beside it. The Grove session that Ravel 1.1.0 launched must still end through its versioned binary.

## Context

- The release task runs `brew upgrade ravel` as one of its last steps. Read the running log of `release-ravel-2-0-0`. This leaf checks the result, and it runs the upgrade only if the task did not.
- The driver resolved `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` once. The end of every session goes through that path. Homebrew removes the old version on an upgrade unless `HOMEBREW_NO_INSTALL_CLEANUP` is set.
- Sessions under the 1.1.0 driver read `policy.ts`. Ravel 2.0.0 reads `config.rhai` after `configuration-names-k92`. The next leaf creates that file.
- Do not touch `~/.config/ravel`. The leaf `owner-policy-conversion` owns it.

## Done when

- `brew list --versions ravel` shows 2.0.0 and 1.1.0.
- `/opt/homebrew/bin/ravel --version` prints `ravel 2.0.0`. The versioned 1.1.0 binary still prints `ravel 1.1.0`.
- `brew test ravel` passes.
- A smoke run of the installed 2.0.0 works in a sandbox home with a Rhai policy and a fake harness. The run does not touch the owner's real configuration.
- If the release task did not upgrade, `HOMEBREW_NO_INSTALL_CLEANUP=1 brew upgrade ravel` did, and the running log says so.
- The leaf reports two things to the owner. After the driver restarts, `brew cleanup ravel` removes 1.1.0. Until then, no command may remove it.

## Notes

- Never run `brew cleanup`, `brew uninstall` or `brew reinstall` in this leaf.
- If 1.1.0 is gone, stop and say so. A reinstall of 1.1.0 from the tap needs the owner's decision.

## Decisions (running log)

- 14 November 2026: Reconciled the historical upgrade request with the delivered installation. `brew list --versions ravel` reports `ravel 2.0.0 1.1.0`; the linked binary and the versioned 2.0.0 binary report `ravel 2.0.0`, and the versioned 1.1.0 binary reports `ravel 1.1.0`. No upgrade is needed. Keep both kegs and perform no cleanup, uninstall or reinstall.
- This launch explicitly uses `/opt/homebrew/Cellar/ravel/2.0.0/bin/ravel`, so its supplied ending command governs this session. The 1.1.0-driver notes above describe the original launch arrangement; preserving that binary remains required.
- Use the existing `smoke:pair` Taskfile recipe against the two installed Homebrew versions. It runs overlapping fake-harness sessions in separate temporary homes, with Rhai for 2.0.0 and a macOS sandbox that denies access to the real home. Leave the owner's configuration conversion to `owner-policy-conversion-k39`.
- Fresh verification passed without changing either installation. `HOMEBREW_NO_AUTO_UPDATE=1 brew test ravel` exited 0. Its four successful fixture launches printed two `Operation not permitted (os error 1)` stop-attempt diagnostics each; record those diagnostics rather than claiming a silent run.
- `mise exec -- task smoke:pair OLD=/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel NEW=/opt/homebrew/Cellar/ravel/2.0.0` exited 0. Both installed versions ran concurrently under separate temporary homes; the 2.0.0 Rhai configuration delivered a retained skill descriptor and mandatory guidance to the fake harness, and the 1.1.0 release refused skills settings with status 2. The directory build was `7f2ff6b3f5222c13b69b78b8cd2c33d1c7edaa6a8599cfb27ebfb59d209d8ec5`.
- The smoke test's before/after tree digests matched for both kegs and the owner's real configuration and skill cache. The sandbox's control also confirmed that managed runs could not read the real home. No source, Taskfile, configuration or ADR change is needed for this installation check; the existing recipe covers the required seam.
- Reported the cleanup handover to the owner: keep 1.1.0 until any driver using its versioned path has restarted. After that restart, `brew cleanup ravel` can remove the retained 1.1.0 keg. This session ran no cleanup, uninstall, reinstall or upgrade, and leaves both versions installed.

## Verification evidence

The paired smoke test reported these unchanged SHA-256 tree digests (files, modes and links):

| Subject | Before and after |
| --- | --- |
| `/opt/homebrew/Cellar/ravel/1.1.0` | `ca5e9de82b220caf7973c10adc718e7225d07df0cb20903abca52ccdf621d796` |
| `/opt/homebrew/Cellar/ravel/2.0.0` | `88a61a4058fa2f223059dbedda3589f448997c11c5c1df14208964eb6fc18193` |
| `/Users/antony/.config/ravel` | `d17e99b4a39cec39ddd69d91f6e65eec16c984e62743e5460d8d52dfda537b3a` |
| `/Users/antony/.cache/ravel` | `6b01124e6742e25db9d84dada32ea1ed95e01e633414d1c3047e5cc0e9be3e83` |
