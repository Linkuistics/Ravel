**First Turn**
> "Which exact subsystem should this walkthrough cover?"

**Later Questions**
Rule: ask exactly one question per turn, wait for the user's answer, record it, and only then ask the next question.

1. What is the primary purpose of this walkthrough?
2. Who is the intended audience?
3. What decisions or tasks should a reader be able to perform after reading it?
4. What is in scope for the walkthrough?
5. What is explicitly out of scope?
6. What form should the final deliverable take?
7. What level of depth is required?
8. Which runtime, deployment, or environment boundaries matter for this subsystem?
9. Which entrypoints, interfaces, or user-visible behaviors matter most?
10. Which risks, incidents, regressions, or confusing areas should receive extra attention?
11. Are there specific architectural questions the walkthrough must answer?
12. Are there required conventions for terminology, naming, or diagrams?
13. Are there constraints on time, length, or reviewability?
14. What evidence standard should I use for claims?
15. What should be excluded even if it appears technically relevant?
16. What would make the walkthrough unacceptable?

**Contract To Freeze Before Inspecting Code**
I would freeze this contract before any code inspection begins:

- Named subsystem: exact subsystem identifier and boundary.
- Objective: what the walkthrough is meant to achieve.
- Audience: who will read it and their expected baseline knowledge.
- Deliverable: format, structure, and expected artifacts.
- Scope: what components, layers, and behaviors are included.
- Exclusions: what is deliberately omitted.
- Depth: overview, implementation detail, operational detail, and failure-mode coverage.
- Priority questions: the specific questions the walkthrough must answer.
- Evidence rule: whether every statement must be traceable to code, config, tests, docs, or runtime artifacts.
- Terminology: canonical names to use.
- Constraints: time, length, confidentiality, and style requirements.
- Acceptance criteria: what must be true for the walkthrough to be considered complete.
