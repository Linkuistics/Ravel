**First Turn**
> "Which subsystem should the walkthrough cover?"

**Rule For Later Turns**

Ask exactly one question per turn. After each answer, record it in the intake, restate it briefly for confirmation, and only then ask the next question. If an answer is ambiguous, spend the next turn on one clarification instead of advancing.

**Later Questions, In Order**

1. What is the primary goal of this walkthrough?
2. Who is the audience?
3. What decisions or tasks should the audience be able to perform after reading it?
4. What artifacts should the walkthrough include?
5. What artifacts should be excluded?
6. What level of depth do you want?
7. What output format do you want for the final walkthrough?
8. What structure do you prefer?
9. Are there priority areas you want covered first?
10. Are there known pain points, bugs, or confusing areas that must be explained carefully?
11. Are there domain terms, acronyms, or business rules I should treat as required glossary items?
12. Are there security, privacy, compliance, or sensitivity constraints on what can be described?
13. Should the walkthrough explain current behavior only, or also note risks, technical debt, and likely refactor seams?
14. Do you want architecture context outside the subsystem when it affects understanding?
15. Do you want runtime behavior covered, such as request flows, state transitions, jobs, or failure handling?
16. Do you want tests, observability, and operational diagnostics included?
17. Are there existing docs, tickets, diagrams, or tribal-knowledge sources I should reconcile against the code later?
18. What counts as complete for this walkthrough?
19. How should disagreements between code and existing documentation be handled?
20. What tone should the walkthrough use?

**Contract To Freeze Before Inspecting Code**

I would freeze a short written contract with these fields:

- `Subsystem`: exact name and boundary.
- `Objective`: why the walkthrough is being created.
- `Audience`: intended readers and their baseline knowledge.
- `Deliverable`: exact output type and format.
- `Depth`: summary, standard, or exhaustive.
- `In Scope`: files, services, behaviors, and concerns to cover.
- `Out of Scope`: explicit exclusions.
- `Priority Topics`: areas that must receive the most attention.
- `Required Sections`: mandatory headings or artifacts.
- `Risk Posture`: whether to include bugs, debt, and refactor commentary.
- `Constraints`: security, privacy, compliance, and disclosure limits.
- `Reference Sources`: any docs or tickets to compare against later.
- `Completion Criteria`: what the user will accept as done.
- `Conflict Rule`: how to report code-vs-doc mismatches.
- `Change Freeze`: no inspection or authoring until this contract is confirmed.
