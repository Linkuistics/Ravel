`"What exact subsystem should the walkthrough cover?"`

Rule for later turns: ask exactly one question per turn, record the answer verbatim, resolve ambiguities before moving on, and never ask the next question until the current answer is fixed.

Later questions, in order:

1. Who is the primary audience for this walkthrough?
2. What outcome should that audience achieve after reading it?
3. What repository, branch, commit, or snapshot should be treated as the source of truth?
4. What exact files, directories, and generated artifacts are in scope?
5. What files, directories, topics, and dependencies are explicitly out of scope?
6. Must the walkthrough cover the full in-scope corpus completely, or may it be selective?
7. Should the walkthrough explain code in execution order, architectural layers, data flow, or another ordering?
8. What level of source exactness is required for quoted snippets and symbol names?
9. What level of detail is expected for APIs, state, control flow, data structures, and failure paths?
10. Which runtime environments, feature flags, configs, or deployment modes matter for this subsystem?
11. What known pain points, confusing areas, or historically buggy paths should receive extra scrutiny?
12. Are tests, benchmarks, scripts, schemas, migrations, or observability assets part of the walkthrough?
13. Should external dependencies be explained, and if so, how deeply?
14. What output format is required: book chapter, Markdown guide, ADR-style document, inline repo docs, or something else?
15. What structural constraints apply: required sections, maximum length, diagram policy, snippet policy, citation style?
16. Should the walkthrough include unresolved questions, risks, and suspected dead code, or remain purely explanatory?
17. What confidentiality or redaction rules apply to code excerpts, identifiers, credentials, and internal URLs?
18. What is the acceptance bar for completion and review?

Contract to freeze before inspecting code:

- Subsystem identity: exact name and boundary.
- Audience: primary reader and prerequisite knowledge.
- Reader outcome: what the walkthrough must enable.
- Source of truth: repo, branch, commit, and any environment assumptions.
- In-scope corpus: exact paths and artifact types to cover.
- Out-of-scope corpus: exact exclusions.
- Coverage rule: complete coverage vs selective coverage.
- Explanatory ordering: execution, architecture, data flow, or agreed alternative.
- Fidelity rule: how exact snippets, names, and references must be.
- Depth rule: what must be covered in detail and what can be summarized.
- Runtime context: configs, flags, and modes that affect meaning.
- Deliverable format: document type, structure, and length constraints.
- Evidence rule: every explanation must trace back to inspected source.
- Open-issues policy: whether to include ambiguities, risks, and questions.
- Redaction policy: what cannot appear in the final text.
- Completion criteria: the explicit standard for “done.”
