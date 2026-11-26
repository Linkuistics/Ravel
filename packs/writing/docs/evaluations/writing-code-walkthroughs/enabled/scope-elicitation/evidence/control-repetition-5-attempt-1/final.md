**First Turn**
> Which subsystem, exactly, should the walkthrough cover?

**Rule**
Ask exactly one question per turn. Record the user’s answer verbatim, normalize any defined terms, and do not ask the next question until the current answer is confirmed.

**Later Questions**
1. Who is the primary audience for this walkthrough?
2. What outcome should the walkthrough enable for that audience?
3. What form should the final deliverable take?
4. What depth do you want: high-level orientation, implementation-level trace, or both?
5. What is explicitly in scope?
6. What is explicitly out of scope?
7. Which use cases, workflows, or execution paths must be covered?
8. Which quality attributes matter most here: correctness, performance, reliability, security, operability, extensibility, or something else?
9. Are there known pain points, confusing areas, or suspected risks I should pay special attention to?
10. Do you want the walkthrough anchored around architecture, runtime behavior, data flow, control flow, public interfaces, or change history?
11. Is there a specific revision, branch, release, or environment the walkthrough must target?
12. Are there terminology, naming, or domain conventions I must preserve?
13. What level of evidence do you want for claims: code citations only, code plus tests, or code plus runtime traces?
14. Are there confidentiality, compliance, or redaction constraints on the output?
15. What would make you reject the walkthrough as incomplete or wrong?

**Freeze Contract Before Inspecting Code**
Before any inspection, freeze this contract:

- Target subsystem: exact boundary and name.
- Audience: who the walkthrough is for.
- Objective: what the walkthrough must help them do.
- Deliverable: format and expected structure.
- Depth: overview, deep dive, or both.
- Scope: included components and excluded areas.
- Required paths: workflows and scenarios that must be traced.
- Priority lenses: architecture, data flow, control flow, APIs, runtime, tests, risks.
- Baseline: branch/version/environment to treat as source of truth.
- Evidence standard: how every claim will be supported.
- Constraints: redaction, compliance, terminology, and time/length limits.
- Acceptance criteria: the conditions for “complete.”

No code inspection starts until that contract is confirmed.
