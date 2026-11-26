**First Turn**

> "Which repository, package, executable, or bounded subsystem is the walkthrough about, and what subsystem boundaries are in scope?"

**Rule For Later Turns**

Ask exactly one question per turn. Record the answer in the intake record before asking the next question. If an answer is broad, infer nothing: ask the next clarifying question needed to freeze that field before moving on.

**Later Questions, In Order**

1. What is explicitly out of scope?
2. Which repository root, package path, executable name, or artifact identifier is the authoritative target?
3. Which production manifests are included as authoritative source?
4. Which production source files are included as authoritative source?
5. How should tests be classified: included source, evidence only, or excluded?
6. How should fixtures be classified: included source, evidence only, or excluded?
7. How should examples be classified: included source, evidence only, or excluded?
8. How should generated files be classified: included source, evidence only, or excluded?
9. How should formal models or specs be classified: included source, evidence only, or excluded?
10. How should third-party dependencies be classified: included source, evidence only, or excluded?
11. May any authoritative bytes change during authoring?
12. Which revision, snapshot, commit, tag, or build artifact is authoritative?
13. How will source changes be detected and handled during authoring?
14. What is the audience’s language proficiency?
15. What is the audience’s systems and tooling proficiency?
16. What is the audience’s familiarity with the target domain?
17. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicit level?
18. What output files are required?
19. What Markdown structure, navigation, or publishing shape is required?
20. If walkthrough-specific tooling disappears, what source and prose must still remain usable on their own?
21. What terminology is fixed?
22. What prose, citation, heading, cross-reference, and repetition rules are fixed?
23. What deterministic checks must pass before the walkthrough is accepted?
24. Which claims require independent technical review?
25. Which claims require independent editorial review?

**Contract To Freeze Before Inspecting Code**

Freeze a written authoring contract with these fields:

- `Target`: exact subsystem, repository/artifact identity, in-scope boundaries, out-of-scope boundaries.
- `Authoritative Source`: included manifests and production files, plus explicit classification of tests, fixtures, examples, generated files, models/specs, and dependencies.
- `Corpus Stability`: whether bytes may change, which revision or snapshot is authoritative, and how change detection works.
- `Audience`: language proficiency, systems/tooling proficiency, and domain familiarity.
- `Depth`: orientation, subsystem explanation, complete-source coverage, or other named level.
- `Output And Walk-Away Behavior`: required files, Markdown structure, navigation, and what remains usable without special tooling.
- `Style`: fixed terminology, prose rules, citation rules, heading rules, cross-reference rules, and repetition policy.
- `Assurance`: deterministic checks, independent technical review requirements, and independent editorial review requirements.
- `Source-Change Policy`: what happens if the authoritative corpus changes after intake is frozen.
- `Acceptance Checks`: the exact conditions that permit claiming the walkthrough is complete.

No code inspection starts until every field above is frozen.
