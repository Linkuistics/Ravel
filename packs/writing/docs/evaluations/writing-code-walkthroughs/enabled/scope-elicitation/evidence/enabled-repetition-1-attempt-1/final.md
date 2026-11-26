> Which repository, package, executable, or bounded subsystem is the walkthrough about, and what are the exact subsystem boundaries in scope?

Rule for all later turns: ask exactly one question per turn, record the answer verbatim, and if the answer is broad or ambiguous, ask the necessary narrowing follow-up before moving to the next field.

Later questions, in order:

1. Which manifests and production files are authoritative source for this walkthrough, and how should tests, fixtures, models, generated files, examples, and dependencies each be classified: included source, evidence, or excluded material?
2. May any authoritative bytes change during authoring, and if so, which revision, snapshot, or artifact is authoritative and how will change be detected?
3. What is the intended audience’s language proficiency, systems/tooling proficiency, and familiarity with the target domain?
4. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicitly defined level?
5. What output files and Markdown structure are required, and what source/prose must remain usable if walkthrough-specific tooling disappears?
6. What style constraints must be frozen for terminology, prose, citations, headings, navigation, repetition, and cross-references?
7. What assurance is required: which deterministic checks must pass, which claims need independent technical review, and which need independent editorial review?

Contract to freeze before inspecting code:

- `target`: repository/package/executable/subsystem and exact in-scope boundaries
- `authoritative_source`: manifests and production files included as source
- `material_classification`: tests, fixtures, models, generated files, examples, dependencies classified as source, evidence, or excluded
- `corpus_stability`: whether bytes may change during authoring
- `source_change_policy`: authoritative revision/snapshot/artifact and change-detection method
- `audience`: language, tooling/systems, and domain familiarity
- `depth`: required walkthrough level
- `output`: required files and Markdown structure
- `walk_away_behavior`: what remains usable without walkthrough-specific tooling
- `style`: frozen terminology and writing/navigation rules
- `assurance`: deterministic checks, technical review scope, editorial review scope
- `acceptance_checks`: the explicit checks that define completion

No code inspection starts until that contract is fully frozen; later scope changes are treated as contract changes, not informal additions.
