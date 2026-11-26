> “What exact subsystem are we walking through, and what outcome should the walkthrough enable for its audience?”

Rule for the rest of the intake: ask exactly one question per turn, record the answer, then choose the next question based on that answer before continuing.

Later questions, in order:

1. Who is the primary audience for this walkthrough?
2. What is their starting level with the codebase, language, framework, and business domain?
3. What is the scope boundary?
4. What is explicitly out of scope?
5. Is the goal comprehension, onboarding, debugging, modification, incident response, design review, or handoff?
6. What deliverable format do you want?
7. What level of depth do you want?
8. Should the walkthrough prioritize runtime behavior, static structure, data flow, control flow, APIs, state management, persistence, concurrency, security, performance, or operations?
9. Are there specific scenarios, requests, jobs, or user journeys the walkthrough must trace end to end?
10. Are there known pain points, confusing areas, or failure modes that must receive extra attention?
11. Should the walkthrough explain historical design intent and tradeoffs, or only current behavior?
12. Do you want diagrams, tables, glossaries, or sequence narratives included?
13. What evidence standard should be used for claims?
14. How should uncertainty be handled when the code is ambiguous or inconsistent?
15. Are there repository, privacy, compliance, or redaction constraints on what may be quoted or described?
16. Should the walkthrough include defects, risks, and technical debt, or stay descriptive only?
17. What tone should the final walkthrough use?
18. What length target or time-to-read target should it hit?
19. What acceptance criteria will make you say the walkthrough is complete?
20. Is there anything else that must be true before I inspect the code?

**Frozen Contract Before Inspecting Code**

I would freeze this contract:

- Objective: one explicit walkthrough goal, stated in user terms.
- Audience: one named primary audience with known baseline knowledge.
- Scope: named subsystem, entry points, and hard boundaries.
- Output: exact deliverable format and target depth.
- Priorities: ordered topics to emphasize.
- Scenarios: required end-to-end traces to cover.
- Constraints: redaction, compliance, quoting, and evidence rules.
- Review posture: descriptive only, or descriptive plus risks/defects.
- Acceptance: concrete completion criteria.

Once that contract is frozen, I would inspect code only to satisfy that contract and not silently expand scope.
