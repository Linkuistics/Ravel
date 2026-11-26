**First Turn**
> "Which repository, package, executable, or bounded subsystem is the walkthrough about, and what subsystem boundaries are in scope?"

**Later Questions**
Rule: ask exactly one question per turn, record its answer before continuing, and if an answer is partial or broad, ask a narrowing follow-up for that same field before moving to the next field.

1. Which manifests and production files are the authoritative source for this walkthrough?
2. How should tests be classified: included source, evidence, or excluded material?
3. How should fixtures be classified: included source, evidence, or excluded material?
4. How should models be classified: included source, evidence, or excluded material?
5. How should generated files be classified: included source, evidence, or excluded material?
6. How should examples be classified: included source, evidence, or excluded material?
7. How should dependencies be classified: included source, evidence, or excluded material?
8. May the authoritative bytes change during authoring?
9. If yes: which revision, snapshot, or artifact is authoritative?
10. If yes: how will a source change be detected?
11. What is the audience’s language proficiency?
12. What is the audience’s systems and tooling proficiency?
13. What is the audience’s familiarity with the target domain?
14. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicit level?
15. What output files are required?
16. What Markdown structure is required?
17. What must remain usable if walkthrough-specific tooling disappears?
18. What terminology requirements are frozen?
19. What prose requirements are frozen?
20. What citation requirements are frozen?
21. What heading requirements are frozen?
22. What navigation requirements are frozen?
23. What repetition policy is required?
24. What cross-reference policy is required?
25. What deterministic checks must pass?
26. Which claims require independent technical review?
27. Which claims require independent editorial review?

**Contract To Freeze Before Inspecting Code**
No code inspection begins until this authoring contract is explicitly frozen:

- `Target`: repository/package/executable/subsystem and exact in-scope boundaries.
- `Authoritative source`: included manifests and production files.
- `Classification ledger`: tests, fixtures, models, generated files, examples, dependencies each marked as `included source`, `evidence`, or `excluded`.
- `Corpus stability`: whether bytes may change; if so, the authoritative revision/snapshot/artifact and the change-detection rule.
- `Audience`: language proficiency, systems/tooling proficiency, domain familiarity.
- `Depth`: orientation, subsystem explanation, complete production-source coverage, or other named level.
- `Output and walk-away behavior`: required files, Markdown structure, and what remains usable without special tooling.
- `Style`: terminology, prose, citation, heading, navigation, repetition, and cross-reference rules.
- `Assurance`: deterministic acceptance checks, technical review scope, editorial review scope.
- `Acceptance checks`: the exact claims that may be made only after verification.

Once frozen, later scope or output changes are treated as contract changes, not informal additions.
