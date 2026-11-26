**First Turn**

> Which repository, package, executable, or bounded subsystem is the walkthrough about, and what subsystem boundaries are in scope?

**Later Questions**

Rule: ask exactly one question per turn, record the answer verbatim, and only then ask the next question. If an answer is broad, mixed, or leaves a field unfrozen, ask the narrowing follow-up next instead of advancing.

1. Which manifests and production files are authoritative source for this walkthrough, and how should tests, fixtures, models, generated files, examples, and dependencies each be classified: included source, evidence, or excluded material?
2. May any authoritative bytes change during authoring, and if so, which revision, snapshot, or artifact is authoritative and how will change be detected?
3. What is the audience’s language proficiency, systems/tooling proficiency, and familiarity with this domain?
4. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicitly named level?
5. What output files and Markdown structure are required, and what must remain usable if walkthrough-specific tooling disappears?
6. What style constraints are fixed for terminology, prose, citation, headings, navigation, repetition, and cross-references?
7. What assurance is required: which deterministic checks must pass, which claims need independent technical review, and which need independent editorial review?

**Frozen Contract Before Inspecting Code**

I would freeze an authoring contract with these fields:

- `Target`: named repository/package/executable/subsystem, plus exact in-scope and out-of-scope boundaries.
- `Authoritative source`: complete classified inventory of manifests and production files, with tests/fixtures/models/generated files/examples/dependencies marked as `included source`, `evidence`, or `excluded`.
- `Corpus stability`: source-change policy, authoritative revision/snapshot, and change-detection method.
- `Audience`: language, tooling, and domain proficiency assumptions.
- `Depth`: exact deliverable level and any completeness requirement.
- `Output and walk-away behavior`: required files, Markdown structure, and what remains usable without custom tooling.
- `Style`: frozen terminology, citation, heading, navigation, repetition, and cross-reference rules.
- `Assurance`: deterministic checks, acceptance thresholds, and required independent review passes.
- `Acceptance checks`: the specific checks whose passing permits the final claims.
- `Change policy`: any later scope or source changes are treated as contract changes, not informal additions.
