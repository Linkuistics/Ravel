# release-ravel-2-0-0-k37

## Goal

Release Ravel 2.0.0 and keep the Ravel 1.1.0 binary that the running Grove driver uses. The release publishes a GitHub Release, the book on the Pages branch and the Homebrew formula.

## Context

- The input is `docs/book/src/11-deliver-ravel.md`, the Taskfile task `release`, and `scripts/release-*.sh`. The workstream makes no release before this leaf.
- All earlier leaves are done, and `task check`, `task test` and `task release:smoke` pass.
- `task release:major` refuses outside Ravel's default colocated jj workspace. The work of this workstream lives in the secondary workspace `Ravel.gh-issue-2`. Landing the work on main is a decision for the owner. Ask how, and do not choose a way yourself.
- The task ends with `brew upgrade ravel` and `brew test ravel`. The upgrade removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. The Grove driver that runs this session resolved the versioned binary `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` once, and the session ends through it. That version must stay until the driver restarts.
- A release publishes outside the machine, and nobody can retract it fully. The owner asked for the release in the requirements session. Ask once more, with the version and the list of what the task publishes, before you run the task.

## Done when

- The owner approved the landing of the work on main and the publication, and the running log records both answers.
- `CHANGELOG.md` has an entry for 2.0.0. It states that the policy format changed, names the migration guide, and lists the new settings keys, the host functions and the removed Bun worker.
- `task release:doctor` passes.
- `HOMEBREW_NO_INSTALL_CLEANUP=1 task release:major` completes from the default workspace. The tag `v2.0.0`, the GitHub Release with the three archives, the book on the Pages branch and the tap formula exist.
- `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel` still exists after the task.
- If the task stops part way, follow the section "Recover a partial release without another bump" in chapter 11. Never cut a second version.
- The running log records the commands, the outputs that matter and the state of the three publications.

## Notes

- Do not run `brew cleanup`.
- If a step fails and the book does not cover the recovery, stop. Ask the owner, with the trade-off, a recommendation and the evidence.
- The release builds the book from the tag's own source. A book check that fails stops the release before anything is pushed.
