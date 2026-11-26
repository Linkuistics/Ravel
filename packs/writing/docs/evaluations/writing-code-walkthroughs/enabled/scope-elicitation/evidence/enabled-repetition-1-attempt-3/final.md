`writing-code-walkthroughs` is the relevant skill for the eventual authoring pass; I would not invoke it yet because you asked for intake only and no file inspection or tool use.

> "What exact subsystem boundary should this walkthrough cover?"

Rule for the rest of intake: ask exactly one question per turn, record the answer verbatim, and do not ask the next question until the previous answer is confirmed.

1. Who is the primary audience for the walkthrough?
2. What should the reader be able to understand or do after reading it?
3. What kind of walkthrough do you want: onboarding, maintenance, design rationale, incident/debugging, or contributor guide?
4. What completeness standard should apply: exhaustive coverage of the subsystem or selective coverage of the most important paths?
5. What source corpus is in scope for explanation?
6. What sources are authoritative if code, docs, tests, comments, and tickets disagree?
7. What exact revision should be treated as the frozen target?
8. What level of detail should explanations reach: architecture only, file/module level, function level, or line-level for key paths?
9. Which execution paths, scenarios, or use cases must be traced end to end?
10. Which topics are explicitly out of scope even if they appear adjacent in the code?
11. What evidence standard do you want for claims: source-exact citations only, or source plus clearly labeled inference?
12. Should the walkthrough include diagrams, tables, glossaries, or call graphs?
13. What output format and structure do you want?
14. Are there confidentiality, tone, terminology, or style constraints?
15. What are the acceptance criteria for calling the walkthrough complete?

Before inspecting code, I would freeze this contract:

- Subsystem boundary: exact included components and explicit exclusions.
- Audience: primary reader and assumed background.
- Walkthrough goal: the concrete reader outcomes.
- Walkthrough type: onboarding, maintenance, design, debugging, or mixed.
- Completeness rule: exhaustive vs selective, with the selection rule stated.
- Source corpus: allowed files and artifact types.
- Authority order: how conflicts between sources are resolved.
- Revision pin: branch, tag, commit, or snapshot to treat as canonical.
- Detail level: architecture, module, function, or path-specific deep dives.
- Mandatory traces: scenarios or flows that must be covered.
- Citation policy: source-exact reproduction vs labeled inference.
- Deliverable format: markdown/book/guide structure and required extras.
- Style constraints: terminology, tone, redaction, and forbidden content.
- Acceptance criteria: what the finished walkthrough must contain to be approved.
