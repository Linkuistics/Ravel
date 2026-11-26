## Blind sample Amber

| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | `"> "Which subsystem are we covering, who is the audience, and what should the walkthrough enable them to do afterward?""` asks about subsystem, audience, and outcome, so not exactly one question. |
| A02 | 1 | `Ask exactly one question per turn.` |
| A03 | 1 | `Record the answer in a running intake log before asking the next question.` |
| A04 | 1 | `1. What is the exact scope boundary for this subsystem, including what is explicitly out of scope?` |
| A05 | 0 | No later question mentions manifests. |
| A06 | 0 | `8. What are the critical components, services, modules, or boundaries...` does not identify included production source. |
| A07 | 0 | No later question classifies tests and fixtures as included source, evidence, or excluded. |
| A08 | 0 | No later question classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| A09 | 0 | `6. Which code revision should anchor the walkthrough...` does not ask whether the corpus may change during authoring. |
| A10 | 0 | No later question asks which artifact is authoritative if the corpus changes. |
| A11 | 0 | `10. What prior knowledge can I assume from the audience...` does not separately ask language proficiency. |
| A12 | 0 | `10. What prior knowledge can I assume from the audience...` does not separately ask systems or tooling proficiency. |
| A13 | 0 | `10. What prior knowledge can I assume from the audience...` does not separately ask domain familiarity. |
| A14 | 1 | `3. What depth do you want...` and contract `Depth: required technical level...` |
| A15 | 1 | `2. What artifact do you want at the end...` and contract `Deliverable: output type, format, tone, and length.` |
| A16 | 0 | No later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `10. ...what terminology should I avoid or define?` and `13. What constraints apply...` omit citation constraints; contract has no explicit citation field. |
| A18 | 0 | `4. What is the preferred structure...` is not navigation/cross-reference constraints; contract has no explicit navigation/cross-reference field. |
| A19 | 0 | No later question asks for mechanical proof requirements separate from judgment. |
| A20 | 0 | `15. Who will review or approve the walkthrough...` does not elicit independent technical review requirements. |
| A21 | 0 | `15. Who will review or approve the walkthrough...` does not elicit independent editorial review requirements. |

Total successes: **5/21**

Concise adjudication notes: Raw events match the final answer. The protocol gets turn-taking, answer-recording, scope, depth, and output form, but misses the requested corpus taxonomy, change-handling, separate audience proficiency splits, and proof/review requirements.

## Blind sample Birch

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which subsystem, exactly, do you want the walkthrough to cover?"` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `wait for the user's answer, record it verbatim in the intake, then ask the next question` |
| A04 | 1 | `5. What boundaries are in scope, and what adjacent systems are explicitly out of scope?` |
| A05 | 0 | No later question mentions manifests. |
| A06 | 0 | `5. What boundaries are in scope...` does not identify included production source. |
| A07 | 0 | `9. Do you want emphasis on ... testing...` is not a separate tests-and-fixtures source/evidence/excluded classification. |
| A08 | 0 | No later question classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| A09 | 0 | No later question asks whether the corpus may change during authoring. |
| A10 | 0 | No later question asks which artifact is authoritative if the corpus changes. |
| A11 | 0 | `2. Who is the target reader, and what level of prior context can I assume?` does not separately ask language proficiency. |
| A12 | 0 | `2. Who is the target reader, and what level of prior context can I assume?` does not separately ask systems or tooling proficiency. |
| A13 | 0 | `2. Who is the target reader, and what level of prior context can I assume?` does not separately ask domain familiarity. |
| A14 | 1 | `4. What depth do you want...` and contract `Deliverable: fixed output format, depth...` |
| A15 | 1 | `3. What output format do you want...` and contract `Deliverable: fixed output format...` |
| A16 | 0 | No later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `10. Are there required conventions for terminology, tone...` and `13. What citation style...`; the contract does not explicitly record prose, terminology, and citation constraints together. |
| A18 | 0 | `10. ...headings, diagrams...` does not explicitly elicit navigation and cross-reference constraints, and the contract has no explicit navigation/cross-reference field. |
| A19 | 0 | No later question asks for mechanical proof requirements separate from judgment. |
| A20 | 0 | No later question elicits independent technical review requirements. |
| A21 | 0 | No later question elicits independent editorial review requirements. |

Total successes: **6/21**

Concise adjudication notes: Raw events match the final answer. This sample cleanly satisfies the single-question protocol, scope, depth, and format items, but it does not ask the required corpus-management, proficiency-split, tooling-loss, proof, or review questions.

## Blind sample Cobalt

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `What is the exact subsystem you want walked through, identified by repository path or package name and the user-facing capability it supports?` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `record the answer in normalized form, confirm that record, and only then continue to the next question` |
| A04 | 1 | `5. What are the subsystem boundaries: what is in scope, what is explicitly out of scope, and what neighboring systems matter?` |
| A05 | 0 | No later question mentions manifests. |
| A06 | 0 | No later question identifies included production source. |
| A07 | 0 | `8. What evidence standard should I use: code-only, code plus tests...` does not separately classify tests and fixtures as source/evidence/excluded. |
| A08 | 0 | No later question classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| A09 | 0 | No later question asks whether the corpus may change during authoring. |
| A10 | 0 | No later question asks which artifact is authoritative if the corpus changes. |
| A11 | 0 | `6. What prior context should be assumed...` does not separately ask language proficiency. |
| A12 | 0 | `6. What prior context should be assumed...` does not separately ask systems or tooling proficiency. |
| A13 | 0 | `6. What prior context should be assumed: none, basic repo familiarity, domain knowledge...` does not separately ask domain familiarity. |
| A14 | 1 | `3. What depth do you want...` and contract `` `Depth`: architecture, component, function, or line-level.`` |
| A15 | 1 | `7. What output shape do you want...` and contract `` `Output Format`: final structure, tone, and length target.`` |
| A16 | 0 | No later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `9. What constraints apply... terminology preferences...` omits prose and citation constraints; contract has no explicit citation field. |
| A18 | 0 | No later question explicitly elicits navigation and cross-reference constraints, and the contract has no explicit navigation/cross-reference field. |
| A19 | 0 | No later question asks for mechanical proof requirements separate from judgment. |
| A20 | 0 | No later question elicits independent technical review requirements. |
| A21 | 0 | No later question elicits independent editorial review requirements. |

Total successes: **6/21**

Concise adjudication notes: Raw events match the final answer. The sample covers protocol mechanics plus scope, depth, and output shape, but not the required corpus taxonomy, change-handling, audience proficiency splits, tooling fallback, citation/navigation constraints, or review/proof requirements.

## Blind sample Dune

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"> "Which subsystem should the walkthrough cover?""` |
| A02 | 1 | `Ask exactly one question per turn.` |
| A03 | 1 | `After each answer, record it in the intake... and only then ask the next question.` |
| A04 | 0 | No later question asks for repository, package, executable, or subsystem target; the direct target question appears only in the first turn. |
| A05 | 0 | No later question mentions manifests. |
| A06 | 0 | No later question identifies included production source. |
| A07 | 0 | `16. Do you want tests, observability, and operational diagnostics included?` does not separately classify tests and fixtures as source/evidence/excluded. |
| A08 | 0 | No later question classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| A09 | 0 | No later question asks whether the corpus may change during authoring. |
| A10 | 0 | No later question asks which artifact is authoritative if the corpus changes. |
| A11 | 0 | `2. Who is the audience?` does not separately ask language proficiency. |
| A12 | 0 | `2. Who is the audience?` does not separately ask systems or tooling proficiency. |
| A13 | 0 | `11. Are there domain terms, acronyms, or business rules...` is about glossary items, not audience domain familiarity. |
| A14 | 1 | `6. What level of depth do you want?` and contract `` `Depth`: summary, standard, or exhaustive.`` |
| A15 | 1 | `7. What output format do you want for the final walkthrough?` and contract `` `Deliverable`: exact output type and format.`` |
| A16 | 0 | No later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `11. Are there domain terms...` and `20. What tone should the walkthrough use?` omit citation constraints; contract has no explicit citation field. |
| A18 | 0 | `8. What structure do you prefer?` is not explicit navigation/cross-reference constraints, and the contract has no explicit navigation/cross-reference field. |
| A19 | 0 | No later question asks for mechanical proof requirements separate from judgment. |
| A20 | 0 | No later question elicits independent technical review requirements. |
| A21 | 0 | No later question elicits independent editorial review requirements. |

Total successes: **5/21**

Concise adjudication notes: Raw events match the final answer. This sample satisfies the single-question protocol, answer-recording, depth, and output format, but it lacks a later target-bounding question and omits the required corpus, change, proficiency, tooling-loss, citation/navigation, and review/proof items.

## Blind sample Slate

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `What exact subsystem should the walkthrough cover?` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `wait for the answer, record it verbatim, then ask the next question` |
| A04 | 1 | `1. What exact subsystem should the walkthrough cover?` |
| A05 | 0 | No later question mentions manifests. |
| A06 | 0 | No later question identifies included production source. |
| A07 | 0 | `14. Are tests, observability, config, migrations, and operational runbooks in scope?` does not separately classify tests and fixtures as source/evidence/excluded. |
| A08 | 0 | No later question classifies models, generated files, examples, and dependencies as included source, evidence, or excluded. |
| A09 | 0 | No later question asks whether the corpus may change during authoring. |
| A10 | 0 | No later question asks which artifact is authoritative if the corpus changes. |
| A11 | 0 | `2. Who is the walkthrough for?` does not separately ask language proficiency. |
| A12 | 0 | `2. Who is the walkthrough for?` does not separately ask systems or tooling proficiency. |
| A13 | 0 | `2. Who is the walkthrough for?` does not separately ask domain familiarity. |
| A14 | 1 | `5. What depth should the walkthrough go to?` and contract `Deliverable: agreed format, structure, and depth.` |
| A15 | 1 | `4. What output format do you want?` and contract `Deliverable: agreed format, structure, and depth.` |
| A16 | 0 | No later question asks what must remain usable if custom tooling disappears. |
| A17 | 0 | `12. Do you want ... or only prose?` and `15. What should count as sufficient evidence...` omit terminology and citation constraints; contract has `Terminology` but no explicit citation field. |
| A18 | 0 | No later question explicitly elicits navigation and cross-reference constraints, and the contract has no explicit navigation/cross-reference field. |
| A19 | 0 | No later question asks for mechanical proof requirements separate from judgment. |
| A20 | 0 | No later question elicits independent technical review requirements. |
| A21 | 0 | No later question elicits independent editorial review requirements. |

Total successes: **6/21**

Concise adjudication notes: Raw events match the final answer. The sample meets the protocol mechanics plus later scope, depth, and format, but it does not ask the required corpus-management, proficiency-split, tooling fallback, citation/navigation, or review/proof questions.

## Aggregate success counts

| ID | successes across 5 samples |
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
| A10 | 0 |
| A11 | 0 |
| A12 | 0 |
| A13 | 0 |
| A14 | 5 |
| A15 | 5 |
| A16 | 0 |
| A17 | 0 |
| A18 | 0 |
| A19 | 0 |
| A20 | 0 |
| A21 | 0 |
