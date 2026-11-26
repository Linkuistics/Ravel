# Scope elicitation — contemporaneous control repetition 1

## Attempts

- Attempt 1 is invalid infrastructure evidence: network/DNS failed, no final
  assistant message was emitted, and the run directory stayed empty.
- Attempt 2 exited `0` with one final and no tool call or write. It is the
  selected behavioral sample because prompt drift is not in the rubric's
  exhaustive replacement list. Its command argument nevertheless omitted the
  canonical terminal LF (`67d60e…d9` rather than `dc74d6…e3`), so it breaches
  the campaign protocol and makes the comparison non-equivalent.
- Attempt 3 used the exact frozen bytes and otherwise satisfies the validity
  checks, but it was launched after a non-replaceable attempt 2. It is preserved
  as a post-breach diagnostic, not counted or scored.

All raw streams and access manifests are preserved under [`evidence/`](evidence/).

## Atomic score — selected attempt 2, blind sample Quartz

| Criterion | `A01` | `A02` | `A03` | `A04` | `A05` | `A06` | `A07` | `A08` | `A09` | `A10` | `A11` | `A12` | `A13` | `A14` | `A15` | `A16` | `A17` | `A18` | `A19` | `A20` | `A21` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Score | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |

Total successes: **7/21**. The answer satisfies turn discipline, scope,
production-source intake, depth, and output form, while omitting the narrower
corpus, audience, assurance, and navigation contracts. The score is descriptive
because the selected prompt bytes breach the execution contract. See
[blind citations](adjudication/primary.md#quartz).
