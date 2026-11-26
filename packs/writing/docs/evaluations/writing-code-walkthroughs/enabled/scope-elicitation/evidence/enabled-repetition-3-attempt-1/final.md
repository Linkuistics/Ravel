> Which repository, package, executable, or bounded subsystem is this walkthrough about, and what subsystem boundaries are in scope?

Later intake protocol, in order. Rule: ask exactly one question per turn, record the answer verbatim, and do not ask the next question until the current field is frozen. If an answer is broad, ask the narrower follow-up needed to freeze that field before continuing.

1. Which repository-relative manifests and production files are included as authoritative source?
2. Should tests be treated as included source, evidence, or excluded material?
3. Should fixtures be treated as included source, evidence, or excluded material?
4. Should models or formal specs be treated as included source, evidence, or excluded material?
5. Should generated files be treated as included source, evidence, or excluded material?
6. Should examples or sample apps be treated as included source, evidence, or excluded material?
7. Should dependencies and external code be treated as included source, evidence, or excluded material?
8. May the authoritative bytes change during authoring?
9. If they may change, which revision, snapshot, commit, tag, archive, or other artifact is authoritative, and how will change be detected?
10. What is the audience’s language proficiency?
11. What is the audience’s systems and tooling proficiency?
12. What is the audience’s familiarity with the target domain?
13. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicit level?
14. What output files and Markdown structure are required?
15. What must remain usable if walkthrough-specific tooling disappears?
16. What terminology requirements must be frozen?
17. What prose style requirements must be frozen?
18. What citation requirements must be frozen?
19. What heading and navigation requirements must be frozen?
20. What repetition and cross-reference requirements must be frozen?
21. What deterministic checks must pass before the walkthrough is accepted?
22. Which claims require independent technical review?
23. Which claims require independent editorial review?

Contract to freeze before inspecting code:

- `Target`: named repository/package/executable/subsystem and explicit in-scope boundaries
- `Authoritative source`: exact included manifests and production files, plus separate classifications for tests, fixtures, models/specs, generated files, examples, and dependencies
- `Corpus stability`: whether source may change; if yes, the exact authoritative revision/snapshot and change-detection rule
- `Audience`: language proficiency, systems/tooling proficiency, domain familiarity
- `Depth`: exact deliverable level
- `Output and walk-away behavior`: required files, Markdown structure, and what remains usable without custom tooling
- `Style`: frozen terminology, prose, citation, headings, navigation, repetition, and cross-reference rules
- `Assurance`: deterministic checks, technical review scope, editorial review scope
- `Source-change policy`: how contract changes are handled if scope or bytes move during authoring
- `Acceptance checks`: the explicit conditions that must be true before claiming completeness or quality

I would not inspect code until that contract is frozen.
