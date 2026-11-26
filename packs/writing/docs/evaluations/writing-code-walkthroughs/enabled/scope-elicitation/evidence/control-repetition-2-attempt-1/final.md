**First Turn**

> "What exact subsystem should the walkthrough cover?"

**Question Order Rule**

Ask exactly one question per turn. Record the answer in a running intake record before asking the next question.

**Later Questions**

1. What decision or outcome should this walkthrough enable?
2. Who is the primary audience, and what is their assumed technical baseline?
3. What depth do you want: executive overview, engineer onboarding, maintenance guide, or implementation deep dive?
4. What is explicitly in scope for the subsystem?
5. What is explicitly out of scope?
6. Which repository, branch, commit, build, or deployment version should be treated as the source of truth?
7. Which runtime flow, user journey, or operational scenario should anchor the walkthrough first?
8. What are the current pain points, unknowns, or failure modes you most want clarified?
9. What deliverable shape do you want: narrative document, annotated outline, diagram set, API trace, call-chain map, or mixed format?
10. What constraints apply to the walkthrough: time budget, length, confidentiality, redaction, or prohibited areas?
11. What evidence standard should I use: code only, code plus tests, code plus docs, or code plus runtime/config artifacts?
12. What counts as done for you?

**Frozen Contract Before Inspecting Code**

I would freeze a short written contract with these fields:

- Subsystem identifier
- Objective of the walkthrough
- Primary audience and assumed baseline
- Depth level
- In-scope boundaries
- Out-of-scope boundaries
- Source-of-truth code revision/environment
- Anchor flow(s) to trace first
- Required deliverable format
- Evidence sources allowed
- Constraints and redactions
- Acceptance criteria for completion

Once that contract is frozen, only then would I inspect code.
