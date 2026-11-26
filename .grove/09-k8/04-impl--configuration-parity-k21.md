# configuration-parity-k21

## Goal

Add the three acceptance checks of Lua routing as Taskfile tasks, and run
them. They are the owner comparison, the build without Rhai and the release
smoke without a separate Lua installation.

## Context

- Root brief, "Test seams".
- The existing Taskfile tasks `release:smoke` and `test:cli`.

## Done when

- A Taskfile task for the owner's machine runs the owner's real `config.rhai`
  and its Lua translation through the command-line seam. It covers every
  configured kind and an unknown kind.
- It compares the program, the arguments, the effort, the provider, the
  Jujutsu workspace grants and the unknown-kind refusal.
- It writes its results outside the repository, and the comparison matches.
- A check builds Ravel without the Rhai module and runs the Lua tests at the
  command-line seam. It passes.
- The release smoke shows that the release binary needs no separate Lua
  installation.

## Notes

- The owner's Lua translation lives in the owner's configuration directory,
  not in the repository. Never publish it or the owner's secrets as a fixture.
- If the translation does not exist yet, write it with the owner's agreement,
  outside the repository.
