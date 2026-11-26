**First Turn**
> "Which subsystem, by exact name or path, do you want the walkthrough to cover?"

**Rule For Later Intake**
I would ask exactly one question per turn, record your answer, and only then ask the next question.

**Later Questions In Order**
1. What is the primary goal of this walkthrough: onboarding, maintenance, debugging, audit, refactor prep, or architecture documentation?
2. Who is the intended reader, and what is their expected technical baseline?
3. What output shape do you want: narrative walkthrough, annotated call flow, sequence diagram notes, file-by-file map, or a mixed format?
4. What depth should the walkthrough reach: conceptual overview, execution path detail, or line-level behavior?
5. What are the hard scope boundaries: included components, excluded components, and adjacent systems that may be referenced but not unpacked?
6. What is the authoritative entrypoint for the subsystem: API, job, CLI command, event consumer, UI action, or another trigger?
7. Which runtime context matters: local dev, production behavior, specific environment, or a particular deployment mode?
8. Are there known pain points, risks, or confusing areas that the walkthrough must prioritize?
9. Should the walkthrough include data flow, control flow, state transitions, error handling, concurrency, and external dependencies, or only a subset?
10. What artifacts are allowed in the final deliverable: prose only, tables, diagrams described in text, code excerpts, glossary, checklist, or FAQ?
11. Are there repository areas or file types that must not be inspected or cited?
12. Should the walkthrough explain historical design intent if inferable, or remain strictly limited to present behavior?
13. What level of certainty labeling do you want for inferred behavior versus directly evidenced behavior?
14. What acceptance criteria will tell you the walkthrough is complete and usable?

**Contract To Freeze Before Inspecting Code**
- Objective: produce a complete walkthrough for the named subsystem aligned to the stated goal.
- Audience: fixed to the reader profile you provide.
- Scope: fixed inclusions, exclusions, and allowed adjacent references.
- Entrypoint: fixed starting trigger or execution path.
- Depth: fixed analysis level and artifact format.
- Evidence policy: distinguish observed facts from inference; no unstated assumptions.
- Coverage policy: include only the dimensions you approve, such as control flow, data flow, state, errors, dependencies, and operational context.
- Source boundary: inspect only approved repository areas and artifact types.
- Completion standard: fixed acceptance criteria agreed up front.
- Change control: if scope changes after freeze, I stop and renegotiate the contract before inspecting code.
