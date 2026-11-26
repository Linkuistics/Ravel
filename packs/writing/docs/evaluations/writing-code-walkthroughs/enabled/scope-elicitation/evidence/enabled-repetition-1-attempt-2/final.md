> Which repository, package, executable, or bounded subsystem is the walkthrough about, and what boundaries are in scope?

Rule for later turns: ask exactly one question per turn, record the answer verbatim in the intake record, and do not ask the next question until the current field is frozen.

Later questions, in order:

1. Which manifests and production files are part of the authoritative source for this walkthrough?
2. How should tests, fixtures, formal models, generated files, examples, and dependencies each be classified: included source, evidence, or excluded material?
3. May any authoritative bytes change during authoring?
4. If the corpus may change, which revision, snapshot, tag, commit, build artifact, or export is authoritative?
5. How should a source change be detected while the walkthrough is being produced?
6. What is the audience’s English-language proficiency?
7. What is the audience’s systems and tooling proficiency?
8. What is the audience’s familiarity with the target domain?
9. What depth is required: orientation, subsystem explanation, complete production-source coverage, or another explicitly defined level?
10. What output files are required?
11. What Markdown structure is required?
12. What parts of the result must remain usable if walkthrough-specific tooling disappears?
13. What terminology requirements are fixed?
14. What prose-style requirements are fixed?
15. What citation rules are fixed?
16. What heading and navigation rules are fixed?
17. What repetition and cross-reference rules are fixed?
18. What deterministic checks must pass before the walkthrough is acceptable?
19. Which claims require independent technical review?
20. Which claims require independent editorial review?

Contract to freeze before inspecting code:

```md
# Authoring Contract

## 1. Target
- Repository / package / executable / subsystem:
- In-scope boundaries:
- Explicitly out of scope:

## 2. Authoritative Source
- Included manifests:
- Included production files:
- Tests classification:
- Fixtures classification:
- Models/specs classification:
- Generated files classification:
- Examples classification:
- Dependencies classification:

## 3. Corpus Stability
- Bytes may change during authoring: yes/no
- Authoritative revision / snapshot / artifact:
- Change-detection method:
- Source-change policy:

## 4. Audience
- Language proficiency:
- Systems/tooling proficiency:
- Domain familiarity:

## 5. Depth
- Deliverable depth:
- Coverage expectation:

## 6. Output and Walk-Away Behavior
- Required output files:
- Required Markdown structure:
- Walk-away usability requirement if tooling disappears:

## 7. Style
- Terminology rules:
- Prose rules:
- Citation rules:
- Heading/navigation rules:
- Repetition rules:
- Cross-reference rules:

## 8. Assurance
- Deterministic acceptance checks:
- Independent technical review requirements:
- Independent editorial review requirements:

## Acceptance Freeze
- Scope changes after this point are contract changes, not informal additions.
- Code inspection begins only after every field above is frozen.
```
