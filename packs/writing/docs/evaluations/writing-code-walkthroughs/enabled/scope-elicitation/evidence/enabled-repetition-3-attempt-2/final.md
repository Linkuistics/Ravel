> “Which repository, package, executable, or bounded subsystem is the walkthrough about, and what subsystem boundaries are in scope?”

Rule for the rest of intake: ask exactly one question per turn, record the answer verbatim, freeze that field, and only then ask the next question. A broad answer does not discharge a narrower field; follow-ups still get asked until each field below is explicit.

Later questions, in order:

1. Which manifests and production files are included as authoritative source for this walkthrough?
2. How should tests be classified: included source, supporting evidence, or excluded material?
3. How should fixtures be classified: included source, supporting evidence, or excluded material?
4. How should formal models or specs be classified: included source, supporting evidence, or excluded material?
5. How should generated files be classified: included source, supporting evidence, or excluded material?
6. How should examples or sample apps be classified: included source, supporting evidence, or excluded material?
7. How should third-party dependencies be treated: covered as source, referenced as versioned context, or excluded from source coverage?
8. May the authoritative bytes change during authoring?
9. If they may change, which revision, snapshot, tag, commit, release artifact, or export is authoritative?
10. How should source changes be detected while the walkthrough is being produced?
11. What is the audience’s English-language proficiency for dense technical prose?
12. What is the audience’s systems and tooling proficiency?
13. What is the audience’s familiarity with the target domain?
14. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicitly defined level?
15. What output files are required?
16. What Markdown structure is required?
17. What must remain usable if walkthrough-specific tooling disappears?
18. Which terminology must be fixed and preferred?
19. What prose constraints apply: terse, instructional, reference-like, narrative, or another defined style?
20. What citation style or source-attribution rules are required?
21. What heading and navigation conventions are required?
22. What repetition policy should govern local restatement versus linking?
23. What cross-reference format and density are required?
24. Which properties must deterministic checks prove before the walkthrough can be accepted?
25. Which claims require independent technical review?
26. Which claims or qualities require independent editorial review?

Contract to freeze before inspecting code:

1. `Target`
   Repository/package/executable/subsystem, explicit in-scope boundaries, explicit out-of-scope boundaries.
2. `Authoritative Source`
   Exact manifests and production files included; separate classification for tests, fixtures, models/specs, generated files, examples, and dependencies.
3. `Corpus Stability`
   Whether bytes may change; if so, the authoritative revision/snapshot artifact and the change-detection rule.
4. `Audience`
   Language proficiency, systems/tooling proficiency, and domain familiarity.
5. `Depth`
   Orientation, subsystem explanation, complete production-source coverage, or another named level with acceptance meaning.
6. `Output And Walk-Away Behavior`
   Required files, required Markdown structure, and what remains readable/usable without custom tooling.
7. `Style`
   Frozen terminology, prose style, citation rules, heading conventions, navigation rules, repetition policy, and cross-reference rules.
8. `Assurance`
   Deterministic checks required for acceptance, plus independent technical-review and editorial-review requirements.
9. `Source-Change Policy`
   How contract drift and source drift are handled after intake; later scope changes are treated as contract changes, not informal additions.
10. `Acceptance Checks`
    The exact checks whose passing status permits claims about completeness, accuracy, and review status.

Additional freeze rule: no chapter sequence is designed before the evidence inventory is known.
