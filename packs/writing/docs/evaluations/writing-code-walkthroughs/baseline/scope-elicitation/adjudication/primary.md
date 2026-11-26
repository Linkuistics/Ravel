## Blind sample Amber

| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | Contradictory first-turn quote: `"Which subsystem are we covering, who is the audience, and what should the walkthrough enable them to do afterward?"` |
| A02 | 1 | `"Ask exactly one question per turn."` |
| A03 | 1 | `"Record the answer in a running intake log before asking the next question."` |
| A04 | 1 | `"What is the exact scope boundary for this subsystem, including what is explicitly out of scope?"` |
| A05 | 0 | Omission: no later question mentions manifests. |
| A06 | 0 | Omission: no later question identifies included production source. |
| A07 | 0 | Omission: no later question classifies tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | Omission: no later question classifies models, generated files, examples, and dependencies. |
| A09 | 0 | Omission: no later question asks whether the corpus may change during authoring. |
| A10 | 1 | `"Which code revision should anchor the walkthrough: branch, tag, commit, release, or current workspace state?"` |
| A11 | 0 | Omission: no later question asks about audience language proficiency. |
| A12 | 0 | Omission: no later question asks about audience systems or tooling proficiency. |
| A13 | 0 | Omission: no later question asks about audience familiarity with the codebase's domain. |
| A14 | 1 | Later question: `"What depth do you want: executive overview, engineer onboarding, maintainer depth, or incident-debugging depth?"`; contract: `"Depth: required technical level and assumed audience background."` |
| A15 | 1 | Later question: `"What artifact do you want at the end: narrative document, slide outline, spoken demo script, onboarding guide, or mixed package?"`; contract: `"Deliverable: output type, format, tone, and length."` |
| A16 | 0 | Omission: no later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | Closest later questions are `"What prior knowledge can I assume from the audience, and what terminology should I avoid or define?"` and `"What constraints apply to the output: length, tone, formatting, redactions, internal naming, or compliance requirements?"`; omission: no later question elicits citation constraints. |
| A18 | 0 | Closest later questions are `"What is the preferred structure..."` and `"Do you want diagrams, sequence traces, dependency maps, or call-flow tables included?"`; omission: no later question elicits navigation and cross-reference constraints, and the contract does not record them. |
| A19 | 0 | Omission: no later question asks for mechanical proof requirements separately from judgment. |
| A20 | 0 | `"Who will review or approve the walkthrough, and what is the acceptance bar?"` does not elicit independent technical review requirements. |
| A21 | 0 | `"Who will review or approve the walkthrough, and what is the acceptance bar?"` does not elicit independent editorial review requirements. |

Total successes: 6/21

Concise adjudication notes: Raw events match the final answer. Passes the turn-by-turn intake rule, scope boundary, revision anchor, depth, and deliverable form; misses the required artifact classifications, corpus-change handling question, separated audience proficiencies, review requirements, and most contract-recorded category constraints.

## Blind sample Birch

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which subsystem, exactly, do you want the walkthrough to cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"wait for the user's answer, record it verbatim in the intake, then ask the next question"` |
| A04 | 1 | `"What boundaries are in scope, and what adjacent systems are explicitly out of scope?"` |
| A05 | 0 | Omission: no later question mentions manifests. |
| A06 | 0 | Omission: no later question identifies included production source. |
| A07 | 0 | Omission: no later question classifies tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | Omission: no later question classifies models, generated files, examples, and dependencies. |
| A09 | 0 | Omission: no later question asks whether the corpus may change during authoring. |
| A10 | 0 | Omission: no later question identifies an authoritative branch, tag, commit, release, or equivalent artifact. |
| A11 | 0 | Omission: no later question asks about audience language proficiency. |
| A12 | 0 | Omission: no later question asks about audience systems or tooling proficiency. |
| A13 | 0 | Omission: no later question asks about audience familiarity with the codebase's domain. |
| A14 | 1 | Later question: `"What depth do you want: high-level overview, function/class level, or line-by-line critical path?"`; contract: `"Deliverable: fixed output format, depth, and acceptance criteria."` |
| A15 | 1 | Later question: `"What output format do you want: narrative document, outline, annotated architecture note, slide script, or Q&A brief?"`; contract: `"Deliverable: fixed output format, depth, and acceptance criteria."` |
| A16 | 0 | Omission: no later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | Later questions split pieces across `"Are there required conventions for terminology, tone, headings, diagrams, or evidence standards?"` and `"What citation style do you want for code references..."`; contract omission: no contract field records terminology or citation constraints. |
| A18 | 0 | `"Are there required conventions for terminology, tone, headings, diagrams, or evidence standards?"` does not elicit navigation and cross-reference constraints, and the contract does not record them. |
| A19 | 1 | Proof requirement: `"Are there required conventions for terminology, tone, headings, diagrams, or evidence standards?"`; judgment: `"Should the walkthrough describe current behavior only, or also evaluate design quality and improvement opportunities?"` |
| A20 | 0 | Omission: no later question elicits independent technical review requirements. |
| A21 | 0 | Omission: no later question elicits independent editorial review requirements. |

Total successes: 7/21

Concise adjudication notes: Raw events match the final answer. This sample cleanly satisfies the single-question first turn, turn discipline, scope boundary, depth/output capture, and separate evidence-vs-judgment prompt; it still omits the required source-classification, corpus-change, audience-proficiency, and independent review questions.

## Blind sample Cobalt

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What is the exact subsystem you want walked through, identified by repository path or package name and the user-facing capability it supports?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record the answer in normalized form, confirm that record, and only then continue to the next question"` |
| A04 | 1 | `"What are the subsystem boundaries: what is in scope, what is explicitly out of scope, and what neighboring systems matter?"` |
| A05 | 0 | Omission: no later question mentions manifests. |
| A06 | 0 | Omission: no later question identifies included production source. |
| A07 | 0 | Omission: no later question classifies tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | Omission: no later question classifies models, generated files, examples, and dependencies. |
| A09 | 0 | Omission: no later question asks whether the corpus may change during authoring. |
| A10 | 0 | Omission: no later question identifies an authoritative branch, tag, commit, release, or equivalent artifact. |
| A11 | 0 | Omission: no later question asks about audience language proficiency. |
| A12 | 0 | Omission: no later question asks about audience systems or tooling proficiency. |
| A13 | 0 | Omission: no later question asks about audience familiarity with the codebase's domain. |
| A14 | 1 | Later question: `"What depth do you want: architectural overview, component-level trace, function-level analysis, or line-by-line execution path?"`; contract: `` `Depth`: architecture, component, function, or line-level. `` |
| A15 | 1 | Later question: `"What output shape do you want: narrative document, outline, onboarding guide, review memo, slide-ready structure, or annotated trace?"`; contract: `` `Output Format`: final structure, tone, and length target. `` |
| A16 | 0 | Omission: no later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `"What constraints apply: time budget, length limit, confidentiality rules, terminology preferences, or prohibited areas?"` omits citation constraints, and the contract has no citation field. |
| A18 | 0 | Omission: no later question elicits navigation and cross-reference constraints, and the contract does not record them. |
| A19 | 0 | `"What evidence standard should I use: code-only, code plus tests, code plus docs, or code plus runtime/config artifacts?"` is not paired with a separate later question for judgment. |
| A20 | 0 | Omission: no later question elicits independent technical review requirements. |
| A21 | 0 | Omission: no later question elicits independent editorial review requirements. |

Total successes: 6/21

Concise adjudication notes: Raw events match the final answer. Strengths are the single-question first turn, one-question intake rule, scope boundary, and explicit depth/output capture. The required source classifications, corpus governance, audience-proficiency splits, navigation/citation constraints, and review requirements are absent.

## Blind sample Dune

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which subsystem should the walkthrough cover?"` |
| A02 | 1 | `"Ask exactly one question per turn."` |
| A03 | 1 | `"After each answer, record it in the intake"` |
| A04 | 0 | Closest later questions are `"What artifacts should the walkthrough include?"` and `"What artifacts should be excluded?"`; omission: no later question bounds the repository, package, executable, or subsystem target. |
| A05 | 0 | Omission: no later question mentions manifests. |
| A06 | 0 | Omission: no later question identifies included production source. |
| A07 | 0 | Omission: no later question classifies tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | Omission: no later question classifies models, generated files, examples, and dependencies. |
| A09 | 0 | Omission: no later question asks whether the corpus may change during authoring. |
| A10 | 0 | Omission: no later question identifies an authoritative branch, tag, commit, release, or equivalent artifact. |
| A11 | 0 | Omission: no later question asks about audience language proficiency. |
| A12 | 0 | Omission: no later question asks about audience systems or tooling proficiency. |
| A13 | 0 | Omission: no later question asks about audience familiarity with the codebase's domain. |
| A14 | 1 | Later question: `"What level of depth do you want?"`; contract: `` `Depth`: summary, standard, or exhaustive. `` |
| A15 | 1 | Later question: `"What output format do you want for the final walkthrough?"`; contract: `` `Deliverable`: exact output type and format. `` |
| A16 | 0 | Omission: no later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | Later questions include `"What tone should the walkthrough use?"` and `"Are there domain terms, acronyms, or business rules I should treat as required glossary items?"`; omission: no later question elicits citation constraints, and the contract does not record them. |
| A18 | 0 | `"What structure do you prefer?"` does not elicit navigation and cross-reference constraints, and the contract does not record them. |
| A19 | 0 | `"Should the walkthrough explain current behavior only, or also note risks, technical debt, and likely refactor seams?"` is judgment, but there is no separate later question for mechanical proof requirements. |
| A20 | 0 | Omission: no later question elicits independent technical review requirements. |
| A21 | 0 | Omission: no later question elicits independent editorial review requirements. |

Total successes: 5/21

Concise adjudication notes: Raw events match the final answer. It satisfies the basic single-question intake mechanics plus depth and output-form capture, but it never asks the later subsystem-boundary question required by A04 and omits nearly all source-classification, corpus-control, audience-proficiency, and review-specific criteria.

## Blind sample Slate

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"wait for the answer, record it verbatim, then ask the next question"` |
| A04 | 1 | `"What code boundaries are in scope?"` |
| A05 | 0 | Omission: no later question mentions manifests. |
| A06 | 0 | Omission: no later question identifies included production source. |
| A07 | 0 | `"Are tests, observability, config, migrations, and operational runbooks in scope?"` does not separately classify tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | `"Are tests, observability, config, migrations, and operational runbooks in scope?"` does not separately classify models, generated files, examples, and dependencies. |
| A09 | 0 | Omission: no later question asks whether the corpus may change during authoring. |
| A10 | 0 | `"Which runtime environments, deployments, or versions matter?"` does not identify an authoritative branch, tag, commit, release, or equivalent artifact. |
| A11 | 0 | Omission: no later question asks about audience language proficiency. |
| A12 | 0 | Omission: no later question asks about audience systems or tooling proficiency. |
| A13 | 0 | Omission: no later question asks about audience familiarity with the codebase's domain. |
| A14 | 1 | Later question: `"What depth should the walkthrough go to?"`; contract: `"Deliverable: agreed format, structure, and depth."` |
| A15 | 1 | Later question: `"What output format do you want?"`; contract: `"Deliverable: agreed format, structure, and depth."` |
| A16 | 0 | Omission: no later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `"What should count as sufficient evidence for claims in the walkthrough?"` is not a prose/terminology/citation question, and the contract has no citation field. |
| A18 | 0 | `"Do you want call graphs, sequence diagrams, data-flow diagrams, or only prose?"` does not elicit navigation and cross-reference constraints, and the contract does not record them. |
| A19 | 0 | `"What should count as sufficient evidence for claims in the walkthrough?"` is not paired with a separate later question for judgment. |
| A20 | 0 | Omission: no later question elicits independent technical review requirements. |
| A21 | 0 | Omission: no later question elicits independent editorial review requirements. |

Total successes: 6/21

Concise adjudication notes: Raw events match the final answer. This sample gets the first-turn requirement, intake mechanics, subsystem boundary, and depth/output capture, but misses the requested source classifications, corpus-change governance, separated audience-proficiency questions, navigation/citation constraints, and review requirements.

## Aggregate success counts

| ID | success count |
|---|---:|
| A01 | 4 |
| A02 | 5 |
| A03 | 5 |
| A04 | 4 |
| A05 | 0 |
| A06 | 0 |
| A07 | 0 |
| A08 | 0 |
| A09 | 0 |
| A10 | 1 |
| A11 | 0 |
| A12 | 0 |
| A13 | 0 |
| A14 | 5 |
| A15 | 5 |
| A16 | 0 |
| A17 | 0 |
| A18 | 0 |
| A19 | 1 |
| A20 | 0 |
| A21 | 0 |
