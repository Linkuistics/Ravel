`"What exact subsystem should the walkthrough cover?"`

Rule for later turns: ask exactly one question per turn, wait for the answer, record it verbatim in the intake, then ask the next question. Do not merge questions, infer missing answers, or inspect code before the contract is frozen.

Later questions, in order:

1. What is the primary audience for this walkthrough?
2. What should that audience be able to do or understand after reading it?
3. What repository, branch, commit, tag, or release should be treated as the source of truth?
4. What exact files, directories, modules, packages, or services are in scope?
5. What exact files, directories, modules, packages, or services are explicitly out of scope?
6. Does “complete” mean full source coverage of the scoped corpus, or only complete coverage of specific flows and concepts?
7. Which runtime paths, user journeys, jobs, protocols, or request flows must be covered end to end?
8. Are there specific entrypoints that define the subsystem boundary?
9. Are there specific abstractions, classes, functions, schemas, or interfaces that must be explained in detail?
10. What level of source fidelity is required?
11. Should the walkthrough quote and explain exact code fragments, or paraphrase structure with references only?
12. What output form do you want?
13. What chapter or section structure, if any, should be used?
14. What depth should be assumed for the intended reader?
15. What prior knowledge may be assumed?
16. Are there design docs, ADRs, tickets, specs, PRs, or incident reports that must be treated as supporting context?
17. Should tests be used as normative evidence of intended behavior?
18. Should generated code, vendored code, migrations, config, and infrastructure files be included or excluded?
19. Are version history and recent architectural changes part of the walkthrough, or should it be strictly snapshot-based?
20. Are known bugs, legacy seams, TODOs, or risky areas required content?
21. Are there naming, terminology, or product-language constraints I must preserve?
22. Are there confidentiality or redaction constraints for identifiers, secrets, customer names, or internal URLs?
23. Do you want diagrams, tables, call graphs, sequence narratives, or only prose?
24. Should the walkthrough optimize for onboarding, maintenance, debugging, extension, auditability, or rewrite planning?
25. What would make the walkthrough unacceptable even if technically accurate?
26. What are the acceptance criteria for signing off the walkthrough?

**Frozen Contract Before Code Inspection**

I would freeze this contract before inspecting any code:

- Scope: exact subsystem boundary, named inclusions, named exclusions.
- Source of truth: repo plus branch/commit/tag/release and any approved supporting documents.
- Audience: intended reader, assumed background, and target outcomes.
- Completeness standard: what “complete” means for this walkthrough.
- Coverage obligations: required flows, entrypoints, components, and artifacts to explain.
- Fidelity rule: whether to use exact source-grounded excerpts, paraphrased explanations, or both.
- Evidence rule: what counts as authoritative when code, tests, and docs disagree.
- Deliverable shape: format, section structure, diagram expectations, and citation/reference style.
- Constraints: redactions, terminology requirements, and out-of-scope material handling.
- Acceptance criteria: objective conditions for “done.”

No code inspection begins until you confirm that frozen contract.
