"Which subsystem, exactly, do you want the walkthrough to cover?"

Rule for all later turns: ask exactly one question per turn, wait for the user's answer, record it verbatim in the intake, then ask the next question only after resolving ambiguities in the prior answer.

Later questions, in order:
1. What is the primary goal of the walkthrough: onboarding, maintenance, debugging, review, handoff, or audit?
2. Who is the target reader, and what level of prior context can I assume?
3. What output format do you want: narrative document, outline, annotated architecture note, slide script, or Q&A brief?
4. What depth do you want: high-level overview, function/class level, or line-by-line critical path?
5. What boundaries are in scope, and what adjacent systems are explicitly out of scope?
6. What specific questions must the walkthrough answer?
7. What outcomes should the reader have by the end?
8. Are there known pain points, risks, regressions, or confusing areas I should prioritize?
9. Do you want emphasis on architecture, runtime behavior, data flow, control flow, APIs, state, persistence, testing, or operational concerns?
10. Are there required conventions for terminology, tone, headings, diagrams, or evidence standards?
11. Should the walkthrough describe current behavior only, or also evaluate design quality and improvement opportunities?
12. Are there sensitive areas I should avoid quoting directly, such as secrets, credentials, proprietary logic, or customer data?
13. What citation style do you want for code references: file-level, symbol-level, or line-linked references?
14. What is the acceptance bar for “complete” in this walkthrough?
15. Do you want a staged deliverable plan before code inspection, or should inspection begin immediately once the intake is complete?

Contract to freeze before inspecting code:
- Objective: produce a walkthrough for the named subsystem only.
- Audience: fixed to the reader profile you provide.
- Deliverable: fixed output format, depth, and acceptance criteria.
- Scope: explicit in-scope and out-of-scope boundaries.
- Priorities: fixed list of questions, risks, and themes to emphasize.
- Method: inspect code only after intake completion; no speculative authoring beforehand.
- Evidence: every substantive claim in the walkthrough must be traceable to inspected code or explicitly labeled as inference.
- Interaction rule: one question per turn, answer recorded before the next question.
- Change control: if scope, audience, or deliverable changes after freeze, the contract is reopened explicitly before inspection proceeds.
