I would normally use the `writing-code-walkthroughs` skill for this, but your constraint forbids reading skill instructions or inspecting anything yet, so this is the fallback intake protocol.

"Who is the primary audience for this walkthrough, and what should they be able to do after reading it?"

Rule for the rest of the intake: ask exactly one question per turn, record the answer verbatim, then ask the next question only after the prior answer is fixed.

Later questions, in order:

1. What exact subsystem should the walkthrough cover?
2. What repository, package, service, or directory defines the authoritative source corpus for that subsystem?
3. What is out of scope even if the code references it?
4. Do you want a complete coverage walkthrough of the bounded corpus, or a selective walkthrough of only the most important paths?
5. What output form do you want: Markdown guide, chaptered document, onboarding note, ADR-style explainer, or something else?
6. What level should it target: newcomer, working engineer, senior maintainer, or mixed audience?
7. What should the organizing spine be: request flow, control flow, data model, dependency graph, lifecycle/state machine, or file-by-file?
8. What kinds of source excerpts are acceptable: exact snippets, paraphrased summaries, pseudocode, or minimal quoting only?
9. What supporting artifacts are required: diagrams, tables, call graphs, glossary, sequence traces, or none?
10. What constraints should govern the walkthrough: length limit, tone, mandatory sections, forbidden sections, citation style, or house style?
11. What correctness bar should the walkthrough meet before delivery: source-exact, behaviorally accurate, review-ready draft, or publish-ready?
12. Are there known pain points, risky areas, or misconceptions the walkthrough must explicitly address?
13. Should the walkthrough explain current behavior only, or also include design rationale, tradeoffs, and historical context if recoverable from code/comments?
14. What final acceptance criteria will tell you the walkthrough is complete?

**Frozen Contract**
Before inspecting code, I would freeze this contract:

- `Audience`: named reader and expected post-read capability.
- `Subsystem`: exact name and boundary.
- `Source corpus`: authoritative repo/path/module set to inspect.
- `Out of scope`: explicit exclusions.
- `Coverage mode`: complete bounded coverage or selective coverage.
- `Document form`: target artifact and structure expectations.
- `Explanatory spine`: the ordering principle for the walkthrough.
- `Excerpt policy`: how literally source may be quoted.
- `Required artifacts`: diagrams/tables/glossary/traces.
- `Style constraints`: tone, length, section requirements, citation rules.
- `Accuracy standard`: draft vs review-ready vs publish-ready.
- `Acceptance criteria`: concrete definition of done.

Once that contract is frozen, only then would inspection start.
