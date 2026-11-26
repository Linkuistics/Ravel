# Documents are gated, and replies are measured

The always-on layer of `technical-communication` treats a document and a reply
differently. After each write of a prose document, a hook checks the whole file
and gives the findings to the model beside the tool's result. The model fixes
them, or marks a justified departure, before it finishes. A reply is scored
after it is complete, and the score goes to the reply log. No reply is held
back, blocked or sent back for a rewrite, and the hook that sees a reply prints
nothing.

This binds the experiment. The reply log measures how the model writes when no
gate acts on its replies. A reply gate added later would change what the log
measures, and the series before and after it would not compare.

The document gate uses the quiet channel, not a block. On every harness the
write stays on disk whatever the hook returns. The blocking channels show the
human an error, or tell the model that the write failed. Added context gives the
same findings with no failure and no extra turn
([harness survey](../research/harness-hooks-a.md), findings 6 and 7).

## Considered options

- **Gate replies too.** Rejected: a reply is on screen before any check can run
  on it. So a gate only adds a second, rewritten reply, at the cost of one turn
  for each failure. Any output from a stop hook continues the turn: up to eight
  times on Claude Code, and with no limit on Codex. The checker's passive-voice
  check matches word shapes, not grammar, so the gate would also act on false
  alarms. Reopen if a harness can check a reply before the human sees it, and
  the log shows a low false-alarm rate.
- **Instruction only, with no checks.** Rejected: it leaves no evidence, and an
  instruction fades over a long session. Reopen if the experiment ends and the
  skill keeps only its instruction.
- **Block a document write that has findings.** Rejected for the reasons above.
  Reopen if the log and field use show that the model ignores findings that it
  gets as added context.
