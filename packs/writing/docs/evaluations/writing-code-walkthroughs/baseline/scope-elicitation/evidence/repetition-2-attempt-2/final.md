**First Turn**
> What is the exact subsystem you want walked through, identified by repository path or package name and the user-facing capability it supports?

**Later Questions**
Rule: ask exactly one question per turn, record the answer in normalized form, confirm that record, and only then continue to the next question.

1. Who is the walkthrough for: new engineer, adjacent team, senior reviewer, operator, auditor, or another audience?
2. What outcome should the walkthrough enable: orientation, safe modification, debugging, incident response, performance work, security review, or handoff?
3. What depth do you want: architectural overview, component-level trace, function-level analysis, or line-by-line execution path?
4. What runtime or use case should anchor the walkthrough: a specific request flow, job, event, command, screen action, or failure mode?
5. What are the subsystem boundaries: what is in scope, what is explicitly out of scope, and what neighboring systems matter?
6. What prior context should be assumed: none, basic repo familiarity, domain knowledge, or existing system ownership?
7. What output shape do you want: narrative document, outline, onboarding guide, review memo, slide-ready structure, or annotated trace?
8. What evidence standard should I use: code-only, code plus tests, code plus docs, or code plus runtime/config artifacts?
9. What constraints apply: time budget, length limit, confidentiality rules, terminology preferences, or prohibited areas?
10. What are the highest-risk questions you want the walkthrough to answer first?
11. What acceptance criteria will make the walkthrough complete enough to stop?

**Pre-Inspection Contract**
Before inspecting code, I would freeze this contract:

- `Subject`: exact subsystem identifier and scope boundary.
- `Audience`: named reader and assumed background.
- `Goal`: what the walkthrough must enable the reader to do.
- `Depth`: architecture, component, function, or line-level.
- `Anchor Flow`: the concrete execution path or scenario to trace.
- `Inputs`: which evidence sources are allowed.
- `Exclusions`: files, services, environments, or topics out of scope.
- `Output Format`: final structure, tone, and length target.
- `Decision Rules`: one-question-per-turn intake; no scope drift without explicit approval.
- `Completion Bar`: explicit criteria for “done enough to deliver.”
