# homebrew-upgrade-k38

## Goal

Make the owner's Homebrew installation run Ravel 2.0.0, and keep Ravel 1.1.0 beside it. The Grove session that Ravel 1.1.0 launched must still end through its versioned binary.

## Context

- The release task runs `brew upgrade ravel` as one of its last steps. Read the running log of `release-ravel-2-0-0`. This leaf checks the result, and it runs the upgrade only if the task did not.
- The driver resolved `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` once. The end of every session goes through that path. Homebrew removes the old version on an upgrade unless `HOMEBREW_NO_INSTALL_CLEANUP` is set.
- Sessions under the 1.1.0 driver read `policy.ts`. Ravel 2.0.0 reads `policy.rhai`. The next leaf creates that file.
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
