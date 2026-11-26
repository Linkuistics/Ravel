"Which exact subsystem should the walkthrough cover?"

Rule for the rest of the intake: ask exactly one question per turn, wait for the answer, record it verbatim in the intake log, then ask the next question.

Later questions, in order:

1. What repository, branch, commit, or release should the walkthrough be anchored to?
2. Who is the primary audience for the walkthrough?
3. What is the main outcome you want from it?
4. What format should the final walkthrough take?
5. How deep should the walkthrough go?
6. What subsystem boundaries should be treated as in-scope?
7. What adjacent components should be treated as out-of-scope?
8. Which runtime environments, deployment targets, or operating modes matter?
9. Which user flows, APIs, jobs, or execution paths must be covered end-to-end?
10. Which entry points should I prioritize first?
11. Which files, directories, modules, or services do you already believe are central?
12. Are there known pain points, confusing areas, or recent regressions I should pay special attention to?
13. Should the walkthrough emphasize architecture, control flow, data flow, state management, failure handling, performance, security, or testing?
14. What assumptions may I make, and what assumptions must I explicitly verify?
15. Are there documents I may rely on besides the codebase?
16. Are there naming, terminology, or domain conventions I must preserve?
17. Should I include diagrams, call graphs, sequence traces, or dependency maps?
18. What level of code citation do you want?
19. What should the walkthrough explicitly avoid?
20. What does “done” mean for this walkthrough?

Contract to freeze before inspecting code:

- Fixed target: repository plus exact branch/commit/release.
- Fixed subject: named subsystem with explicit in-scope and out-of-scope boundaries.
- Fixed audience: who the walkthrough is for.
- Fixed objective: what decisions or understanding the walkthrough must enable.
- Fixed deliverable: output format, structure, and citation style.
- Fixed depth: architectural only, file/module level, function level, or line-by-line where needed.
- Fixed emphasis: control flow, data flow, state, failure modes, tests, security, performance, or onboarding.
- Fixed operating context: environments and execution modes that matter.
- Fixed required paths: must-cover entry points and end-to-end flows.
- Fixed exclusions: topics and artifacts to ignore.
- Fixed evidence rules: whether to use only code, or code plus approved docs.
- Fixed completion criteria: concrete acceptance standard for the walkthrough.
