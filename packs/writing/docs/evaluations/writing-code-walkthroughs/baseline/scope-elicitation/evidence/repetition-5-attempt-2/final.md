**First Turn**
> "Which subsystem are we covering, who is the audience, and what should the walkthrough enable them to do afterward?"

**Rule**
Ask exactly one question per turn. Record the answer in a running intake log before asking the next question.

**Later Questions**
1. What is the exact scope boundary for this subsystem, including what is explicitly out of scope?
2. What artifact do you want at the end: narrative document, slide outline, spoken demo script, onboarding guide, or mixed package?
3. What depth do you want: executive overview, engineer onboarding, maintainer depth, or incident-debugging depth?
4. What is the preferred structure: architecture-first, request-flow-first, component-by-component, or use-case-first?
5. Which runtime, language, framework, and deployment context should the walkthrough assume?
6. Which code revision should anchor the walkthrough: branch, tag, commit, release, or current workspace state?
7. What are the most important user journeys, execution paths, or jobs this subsystem handles?
8. What are the critical components, services, modules, or boundaries you already know must be covered?
9. What level of operational detail is required: config, observability, scaling, failure modes, security, and data handling?
10. What prior knowledge can I assume from the audience, and what terminology should I avoid or define?
11. Are there known pain points, confusing areas, recent regressions, or risky assumptions I should prioritize?
12. Do you want diagrams, sequence traces, dependency maps, or call-flow tables included?
13. What constraints apply to the output: length, tone, formatting, redactions, internal naming, or compliance requirements?
14. What would make the walkthrough unacceptable or incomplete from your perspective?
15. Who will review or approve the walkthrough, and what is the acceptance bar?

**Contract To Freeze Before Inspecting Code**
- Target: named subsystem, named audience, and explicit learning outcome.
- Scope: in-scope components and hard out-of-scope boundaries.
- Source of truth: exact revision or workspace state to analyze.
- Deliverable: output type, format, tone, and length.
- Depth: required technical level and assumed audience background.
- Coverage priorities: key flows, components, and risks to emphasize.
- Required lenses: architecture, runtime behavior, data flow, dependencies, ops, security, failure modes, or only a subset.
- Evidence standard: every claim must be traceable to inspected code or identified as inference.
- Diagram policy: which visual artifacts are required.
- Terminology policy: canonical names, abbreviations, and redaction rules.
- Acceptance criteria: what “complete” means and who signs off.
- Process gate: no code inspection begins until this contract is confirmed.
