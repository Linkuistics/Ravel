# In-session adversarial review

The leaf-wide fresh-context reviewer read the frozen rubric, reports, primary
score and four raw streams. Its two findings were reconciled against the raw
text:

| Finding | Classification | Resolution |
|---|---|---|
| The infrastructure section said the unstructured attempts named only fixture reads, but all three enabled streams also describe intended skill-instruction reads outside the run directory. | Valid and actionable | State that all four behavioral attempts have an access-audit protocol breach. Do not infer whether an outside-boundary shell call began because its event and operands are absent. Keep the structured MCP/web invalidation reasons separate. |
| The report claimed all three enabled attempts named `writing-code-walkthroughs` and said its file was unreadable. Attempt 1 uses only a generic phrase, and attempt 2 describes blocked `exec_command` plus MCP fallback rather than an unreadable skill file. | Valid and actionable | Report exact-name announcements as `2/3`; distinguish intended skill reads from a visibly read skill body. |

Both fixes are bounded prose corrections verified directly against the preserved
JSONL. They require no second reviewer or tree-level review leaf.
