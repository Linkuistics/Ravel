# External source and fragments — enabled repetition 1

Enabled repetition 1 has no valid sample. All three attempts exited 0, returned
a final refusal, and left the fixture unchanged, but each violated the declared
model-interface boundary:

| Attempt | UTC start | Wall time | Prohibited access |
|---:|---|---:|---|
| 1 | `2026-08-29T12:12:02Z` | 86 s | `list_mcp_resources`; `list_mcp_resource_templates` |
| 2 | `2026-08-29T12:13:50Z` | 76 s | `list_mcp_resources`; `list_mcp_resource_templates` |
| 3 | `2026-08-29T12:15:19Z` | 66 s | empty web search |

Each attempt said it intended to read walkthrough skill instructions. Attempts
2 and 3 named the installed skill exactly; attempt 1 did not. None visibly read
the skill body, and the missing direct shell events leave every claimed skill
read attempt unauditable. Those statements are invalid-attempt evidence, not a
discovery or improvement result. Attempt 3 exhausted the replacement allowance,
so the frozen rule stopped the leaf before schedule position 3.

All raw interactions and manifests are preserved under [`evidence/`](evidence/).
