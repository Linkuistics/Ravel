# Materialising the reviewer, per harness

The discipline (ARTIFACT + CONTRACT, never the CLAIM) is harness-neutral;
only the spawn mechanics differ. Cross-model review is preferred: a different
model family reviewing the author's output catches failure modes
self-review cannot.

## From a codex session → K3 (Kimi Code subscription)

Spawn pi headless in the worktree; it can read files and run commands:

    pi -p --model kimi-coding/k3 "<adversarial review prompt>"

pi persists the session, so a finding worth interrogating can be resumed
interactively afterwards (`pi --resume`).

## From a pi session → GPT-5.6-sol (OpenAI subscription)

The codex binary is the only sanctioned consumer of the OpenAI sub; spawn it
headless:

    codex exec --profile sol-xhigh "<adversarial review prompt>"

## From a Claude Code session

Use a fresh Task subagent (built-in), or either spawn above for a
cross-model read.

Model ids/profiles here match the grove trial config (~/.zshenv,
~/.codex/sol-xhigh.config.toml, ~/.codex/sol-high.config.toml); update this
file if those move.
