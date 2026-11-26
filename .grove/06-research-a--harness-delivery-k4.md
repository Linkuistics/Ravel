# harness-delivery-k4

## Goal

Survey how a launcher can deliver skills, and the hooks that skills need, to
coding-agent harnesses. Answer two questions. How far can one generic delivery
through the prompt reach across harnesses? What can one launch configure on
each supported harness?

## Context

- The supported harnesses are Claude Code, Codex and Gemini CLI. Generic
  harnesses, such as Pi and Hermes, are outside that set.
- Ravel launches a harness as a child process, with arguments, an environment
  and a prompt. Today it delivers skills as directories that the harness reads,
  and it stays neutral about harnesses. That neutrality ends for skill
  delivery.
- Root brief, "Research to commission". Issue 6, "Desired user experience" and
  step 5. Issue 4, on per-harness configuration and hooks.

## Done when

The survey answers these questions for each consumer below, in sections named
by the consumer's subject.

- **The skill-contract design** (`skill-contract`):
  - What reaches the model when a skill arrives in the prompt alone? What is
    lost on compaction, on a continuation and in a delegated subagent?
  - How large a prompt does each harness accept, and how does each fail when
    the prompt is too large?
  - Which native mechanisms exist for skills, instructions and plugins? Which
    of them a launch can set without a change to the user's home directory?
  - How does each harness declare a hook, and which hook events exist? Which
    of them a launch can set from the command line or the environment?
  - How does a native skill name collide with a skill that the user installed?
- **The three adapter implementations** (`claude-code-adapter`,
  `codex-adapter`, `gemini-cli-adapter`):
  - The exact flags, environment variables and files for a launch-scoped
    configuration, with the versions that the sources describe.
  - The sign-in that a sandbox home needs for a real trial.
- **The prompt-only delivery for other harnesses** (`skill-selection`):
  - Does delivery through the prompt alone give a usable skill on Pi and
    Hermes? Does it satisfy the hook goals of issue 4 there?
- **The configuration prototype** (`configuration-experience`):
  - Which delivery modes and limitations does an inspection need to name?

## Notes

- Search for what went wrong after real use. Bug reports, change logs that
  remove a flag, and posts about lost instructions after compaction are the
  evidence. A feature list is not.
- The fallback to beat is "put every skill in the prompt, on every harness".
  Show where it fails, with a citation.
- These harnesses change monthly. Give the date and version of each source.
- Record silence where the search found no primary source.
