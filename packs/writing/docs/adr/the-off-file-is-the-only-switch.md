# The off file is the only switch

One file turns off every always-on layer of `technical-communication`, on every
harness: `technical-communication/off` in the user's state directory
(`$XDG_STATE_HOME`, by default `~/.local/state`). When the file is absent, the
layers are on. That is the state after installation. Every hook reads the file
first, on every event, and does nothing while the file exists.

This binds because a changed location would turn the skill on again, silently,
for each person who turned it off. The user document names the path, and so
does the optional line that the human can paste into a global instruction file.

A file is the only form that every harness's hooks can read
([harness survey](../research/harness-hooks-a.md), finding 9). The scripts read
it themselves, so it needs no support from a harness.

## Consequences

- The switch acts at each layer's next event. Text that a running session
  already has stays in its context until that session ends.
- The skill stays installed when the layers are off. Its description makes it
  load when the always-on instruction is in context, or when the human asks for
  controlled English. So with the layers off, it does not load for ordinary
  writing.
- A line in a global `AGENTS.md` is outside every hook. This repository never
  writes one. The line that the user document offers names the off file, so the
  model obeys the switch there too.

## Considered options

- **An environment variable.** Rejected: a Codex TUI session that the shared
  background daemon hosts gets the daemon's environment, which was captured when
  the daemon started. A variable set later in a terminal does not reach its
  hooks. Reopen if every harness gives its hooks the environment of the
  terminal that opened the session.
- **Each harness's own switch.** Rejected: each works on one harness only, and
  most turn off too much. Claude Code's `disableAllHooks` and Codex's
  `features.hooks` also stop the human's other hooks. Disabling the Claude Code
  hooks plugin works on Claude Code only. Nothing reopens this while the
  skill runs on more than one harness.
- **A key in a configuration file.** Rejected: no configuration file is read by
  every harness. Nothing reopens this.
