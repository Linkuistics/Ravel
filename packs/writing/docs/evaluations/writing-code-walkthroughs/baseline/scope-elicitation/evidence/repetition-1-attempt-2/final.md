**First Turn**
> What exact subsystem should the walkthrough cover?

**Question Order**
Rule: ask exactly one question per turn, wait for the answer, record it verbatim, then ask the next question.

1. What exact subsystem should the walkthrough cover?
2. Who is the walkthrough for?
3. What is the walkthrough meant to achieve for that audience?
4. What output format do you want?
5. What depth should the walkthrough go to?
6. What code boundaries are in scope?
7. What related components are explicitly out of scope?
8. Which runtime environments, deployments, or versions matter?
9. Are there specific user flows, jobs, APIs, or failure modes that must be covered?
10. Are there known pain points, misconceptions, or risky areas I should focus on?
11. What level of architectural context do you want before the code-level walkthrough starts?
12. Do you want call graphs, sequence diagrams, data-flow diagrams, or only prose?
13. Should the walkthrough explain historical design tradeoffs, or only current behavior?
14. Are tests, observability, config, migrations, and operational runbooks in scope?
15. What should count as sufficient evidence for claims in the walkthrough?
16. What should the final deliverable exclude even if I discover it during inspection?
17. What completion criteria will make you say the walkthrough is done?

**Frozen Contract**
Before inspecting code, I would freeze this contract:

- Objective: produce a walkthrough of one named subsystem for one named audience and purpose.
- Scope: exact in-scope repos, directories, services, interfaces, and runtime/version boundaries.
- Exclusions: explicit out-of-scope components and topics.
- Deliverable: agreed format, structure, and depth.
- Coverage: required flows, APIs, data paths, control paths, failure modes, and operational concerns.
- Evidence policy: every substantive claim must be tied to inspected code or approved supporting artifacts.
- Terminology: canonical names for the subsystem, components, and domain concepts.
- Assumption policy: unknowns are marked as unknowns, not inferred silently.
- Change policy: if scope changes after inspection starts, pause and re-freeze the contract.
- Done criteria: the exact conditions for acceptance of the walkthrough.
