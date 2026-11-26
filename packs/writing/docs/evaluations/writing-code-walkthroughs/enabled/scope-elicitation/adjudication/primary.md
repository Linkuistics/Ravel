## Alder

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `> "What exact subsystem boundary should this walkthrough cover?"` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `record the answer verbatim, and do not ask the next question until the previous answer is confirmed` |
| A04 | 0 | Near miss: `5. What source corpus is in scope for explanation?` No later question explicitly bounds the repository, package, executable, or subsystem target. |
| A05 | 0 | Near miss: `5. What source corpus is in scope for explanation?` No manifest question. |
| A06 | 1 | `5. What source corpus is in scope for explanation?` |
| A07 | 0 | Near miss: `6. What sources are authoritative if code, docs, tests, comments, and tickets disagree?` Mentions tests only; no separate classification for tests and fixtures. |
| A08 | 0 | Near miss: `5. What source corpus is in scope for explanation?` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `7. What exact revision should be treated as the frozen target?` No question asking whether the corpus may change during authoring. |
| A10 | 0 | Near miss: `7. What exact revision should be treated as the frozen target?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `1. Who is the primary audience for the walkthrough?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `1. Who is the primary audience for the walkthrough?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `1. Who is the primary audience for the walkthrough?` No separate codebase-domain-familiarity question. |
| A14 | 1 | `8. What level of detail should explanations reach...`; contract: `Detail level: architecture, module, function, or path-specific deep dives.` |
| A15 | 1 | `13. What output format and structure do you want?`; contract: `Deliverable format: markdown/book/guide structure and required extras.` |
| A16 | 0 | Near miss: `12. Should the walkthrough include diagrams, tables, glossaries, or call graphs?` No question about what must remain usable if custom tooling disappears. |
| A17 | 1 | `11. What evidence standard... source-exact citations...`; `14. ... terminology, or style constraints?`; contract: `Citation policy...` and `Style constraints: terminology, tone...` |
| A18 | 0 | Near miss: `13. What output format and structure do you want?` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `11. What evidence standard do you want for claims...` No separate mechanical-proof requirement. |
| A20 | 0 | Near miss: `15. What are the acceptance criteria for calling the walkthrough complete?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `15. What are the acceptance criteria for calling the walkthrough complete?` No independent editorial-review requirement. |

Total successes: 7

## Cobalt

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `` `"What exact subsystem should the walkthrough cover?"` `` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `wait for the answer, record it verbatim in the intake, then ask the next question` |
| A04 | 1 | `4. What exact files, directories, modules, packages, or services are in scope?` |
| A05 | 0 | Near miss: `18. Should generated code, vendored code, migrations, config, and infrastructure files be included or excluded?` No manifest question. |
| A06 | 1 | `4. What exact files, directories, modules, packages, or services are in scope?` |
| A07 | 0 | Near miss: `17. Should tests be used as normative evidence of intended behavior?` No fixtures, and no included-source/evidence/excluded classification for both. |
| A08 | 0 | Near miss: `18. Should generated code, vendored code, migrations, config, and infrastructure files be included or excluded?` Missing models, examples, and dependencies; not the required separate classification set. |
| A09 | 0 | Near miss: `19. Are version history and recent architectural changes part of the walkthrough, or should it be strictly snapshot-based?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `3. What repository, branch, commit, tag, or release should be treated as the source of truth?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `14. What depth should be assumed for the intended reader?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `15. What prior knowledge may be assumed?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `15. What prior knowledge may be assumed?` No separate codebase-domain-familiarity question. |
| A14 | 0 | Near miss: `6. Does “complete” mean full source coverage...` The contract records completeness, but no later question explicitly elicits walkthrough depth and records it. |
| A15 | 1 | `12. What output form do you want?`; contract: `Deliverable shape: format...` |
| A16 | 0 | Near miss: `23. Do you want diagrams, tables, call graphs, sequence narratives, or only prose?` No question about usability if custom tooling disappears. |
| A17 | 0 | Near miss: `21. Are there naming, terminology, or product-language constraints I must preserve?` and `11. ... exact code fragments... or ... references only?` No prose/terminology/citation set recorded together as required. |
| A18 | 0 | Near miss: `13. What chapter or section structure, if any, should be used?` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `10. What level of source fidelity is required?` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `26. What are the acceptance criteria for signing off the walkthrough?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `26. What are the acceptance criteria for signing off the walkthrough?` No independent editorial-review requirement. |

Total successes: 6

## Ember

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `> "What exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `Ask exactly one question per turn.` |
| A03 | 1 | `Record the answer in a running intake record before asking the next question.` |
| A04 | 1 | `4. What is explicitly in scope for the subsystem?` |
| A05 | 0 | Near miss: `11. What evidence standard should I use: code only, code plus tests, code plus docs, or code plus runtime/config artifacts?` No manifest question. |
| A06 | 0 | Near miss: `4. What is explicitly in scope for the subsystem?` No later question explicitly identifies included production source. |
| A07 | 0 | Near miss: `11. ... code only, code plus tests...` Mentions tests only; no fixtures and no required classification. |
| A08 | 0 | Near miss: `11. ... code plus runtime/config artifacts?` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `6. Which repository, branch, commit, build, or deployment version should be treated as the source of truth?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `6. Which repository, branch, commit, build, or deployment version should be treated as the source of truth?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `2. Who is the primary audience, and what is their assumed technical baseline?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `2. Who is the primary audience, and what is their assumed technical baseline?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `2. Who is the primary audience, and what is their assumed technical baseline?` No separate codebase-domain-familiarity question. |
| A14 | 1 | `3. What depth do you want...`; contract: `Depth level` |
| A15 | 1 | `9. What deliverable shape do you want...`; contract: `Required deliverable format` |
| A16 | 0 | Near miss: `9. What deliverable shape do you want...` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `10. What constraints apply... confidentiality, redaction...` No prose/terminology/citation constraint set. |
| A18 | 0 | Near miss: `9. What deliverable shape do you want...` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `11. What evidence standard should I use...` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `12. What counts as done for you?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `12. What counts as done for you?` No independent editorial-review requirement. |

Total successes: 6

## Harbor

| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | Quoted first turn asks two things: `What exact subsystem are we walking through, and what outcome should the walkthrough enable for its audience?` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `record the answer, then choose the next question based on that answer before continuing` |
| A04 | 1 | `3. What is the scope boundary?` |
| A05 | 0 | Near miss: `15. Are there repository, privacy, compliance, or redaction constraints...` No manifest question. |
| A06 | 0 | Near miss: `3. What is the scope boundary?` No later question explicitly identifies included production source. |
| A07 | 0 | Near miss: `13. What evidence standard should be used for claims?` No separate classification for tests and fixtures. |
| A08 | 0 | Near miss: `15. Are there repository, privacy, compliance, or redaction constraints...` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `11. Should the walkthrough explain historical design intent and tradeoffs, or only current behavior?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `3. What is the scope boundary?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `2. What is their starting level with the codebase, language, framework, and business domain?` Not a separate audience language-proficiency question. |
| A12 | 0 | Near miss: `2. What is their starting level with the codebase, language, framework, and business domain?` Not a separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `2. What is their starting level with the codebase, language, framework, and business domain?` Not a separate codebase-domain-familiarity question. |
| A14 | 1 | `7. What level of depth do you want?`; contract: `Output: exact deliverable format and target depth.` |
| A15 | 1 | `6. What deliverable format do you want?`; contract: `Output: exact deliverable format...` |
| A16 | 0 | Near miss: `12. Do you want diagrams, tables, glossaries, or sequence narratives included?` No question about tooling disappearing. |
| A17 | 0 | Near miss: `15. ... privacy... redaction constraints...` and `17. What tone should the final walkthrough use?` No citation-plus-terminology-plus-prose constraint set. |
| A18 | 0 | Near miss: `6. What deliverable format do you want?` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `13. What evidence standard should be used for claims?` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `19. What acceptance criteria will make you say the walkthrough is complete?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `19. What acceptance criteria will make you say the walkthrough is complete?` No independent editorial-review requirement. |

Total successes: 5

## Juniper

| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | Quoted first turn asks two things: `"Who is the primary audience for this walkthrough, and what should they be able to do after reading it?"` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `record the answer verbatim, then ask the next question only after the prior answer is fixed` |
| A04 | 1 | `1. What exact subsystem should the walkthrough cover?` |
| A05 | 0 | Near miss: `2. What repository, package, service, or directory defines the authoritative source corpus for that subsystem?` No manifest question. |
| A06 | 1 | `2. What repository, package, service, or directory defines the authoritative source corpus for that subsystem?` |
| A07 | 0 | Near miss: `8. What kinds of source excerpts are acceptable...` No separate classification for tests and fixtures. |
| A08 | 0 | Near miss: `2. What repository, package, service, or directory defines the authoritative source corpus...` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `2. ... authoritative source corpus...` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `2. ... authoritative source corpus...` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `6. What level should it target: newcomer, working engineer, senior maintainer, or mixed audience?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `6. What level should it target...` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `6. What level should it target...` No separate codebase-domain-familiarity question. |
| A14 | 0 | Near miss: `4. Do you want a complete coverage walkthrough... or a selective walkthrough...` The contract records coverage mode, not explicit walkthrough depth. |
| A15 | 1 | `5. What output form do you want...`; contract: `Document form: target artifact and structure expectations.` |
| A16 | 0 | Near miss: `9. What supporting artifacts are required...` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `10. What constraints should govern the walkthrough: length limit, tone... citation style...` No terminology constraint, and the contract does not record the full prose/terminology/citation set. |
| A18 | 0 | Near miss: `5. What output form do you want...`; contract: `structure expectations.` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `11. What correctness bar should the walkthrough meet...` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `11. What correctness bar should the walkthrough meet before delivery: ... review-ready draft...` No independent technical-review requirement. |
| A21 | 0 | Near miss: `11. What correctness bar should the walkthrough meet before delivery: ... publish-ready?` No independent editorial-review requirement. |

Total successes: 5

## Lichen

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `> "Which subsystem, by exact name or path, do you want the walkthrough to cover?"` |
| A02 | 1 | `I would ask exactly one question per turn` |
| A03 | 1 | `record your answer, and only then ask the next question` |
| A04 | 1 | `5. What are the hard scope boundaries: included components, excluded components...` |
| A05 | 0 | Near miss: `11. Are there repository areas or file types that must not be inspected or cited?` No manifest question. |
| A06 | 0 | Near miss: `5. What are the hard scope boundaries: included components, excluded components...` No later question explicitly identifies included production source. |
| A07 | 0 | Near miss: `10. What artifacts are allowed in the final deliverable...` No separate classification for tests and fixtures. |
| A08 | 0 | Near miss: `11. Are there repository areas or file types that must not be inspected or cited?` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: contract `Change control: if scope changes after freeze...` This is not a later question asking whether the corpus may change during authoring. |
| A10 | 0 | Near miss: contract `Change control: if scope changes after freeze...` No later question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `2. Who is the intended reader, and what is their expected technical baseline?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `2. Who is the intended reader, and what is their expected technical baseline?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `2. Who is the intended reader, and what is their expected technical baseline?` No separate codebase-domain-familiarity question. |
| A14 | 1 | `4. What depth should the walkthrough reach...`; contract: `Depth: fixed analysis level...` |
| A15 | 1 | `3. What output shape do you want...`; contract: `Depth: fixed analysis level and artifact format.` |
| A16 | 0 | Near miss: `10. What artifacts are allowed in the final deliverable...` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `13. What level of certainty labeling do you want for inferred behavior versus directly evidenced behavior?` No prose/terminology/citation constraint set. |
| A18 | 0 | Near miss: `3. What output shape do you want...` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `13. What level of certainty labeling...` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `14. What acceptance criteria will tell you the walkthrough is complete and usable?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `14. What acceptance criteria will tell you the walkthrough is complete and usable?` No independent editorial-review requirement. |

Total successes: 6

## Quartz

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `wait for the answer, record it verbatim in the intake log, then ask the next question` |
| A04 | 1 | `6. What subsystem boundaries should be treated as in-scope?` |
| A05 | 0 | Near miss: `15. Are there documents I may rely on besides the codebase?` No manifest question. |
| A06 | 1 | `11. Which files, directories, modules, or services do you already believe are central?` |
| A07 | 0 | Near miss: `13. Should the walkthrough emphasize ... testing?` No separate classification for tests and fixtures. |
| A08 | 0 | Near miss: `15. Are there documents I may rely on besides the codebase?` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `1. What repository, branch, commit, or release should the walkthrough be anchored to?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `1. What repository, branch, commit, or release should the walkthrough be anchored to?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `2. Who is the primary audience for the walkthrough?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `2. Who is the primary audience for the walkthrough?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `2. Who is the primary audience for the walkthrough?` No separate codebase-domain-familiarity question. |
| A14 | 1 | `5. How deep should the walkthrough go?`; contract: `Fixed depth: architectural only...` |
| A15 | 1 | `4. What format should the final walkthrough take?`; contract: `Fixed deliverable: output format...` |
| A16 | 0 | Near miss: `17. Should I include diagrams, call graphs, sequence traces, or dependency maps?` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `16. Are there naming, terminology, or domain conventions I must preserve?` and `18. What level of code citation do you want?` No prose constraint, and the contract does not record the full prose/terminology/citation set. |
| A18 | 0 | Near miss: `4. What format should the final walkthrough take?` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `18. What level of code citation do you want?` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `20. What does “done” mean for this walkthrough?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `20. What does “done” mean for this walkthrough?` No independent editorial-review requirement. |

Total successes: 7

## Sable

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `` `"What exact subsystem should the walkthrough cover?"` `` |
| A02 | 1 | `ask exactly one question per turn` |
| A03 | 1 | `record the answer verbatim... and never ask the next question until the current answer is fixed` |
| A04 | 1 | `4. What exact files, directories, and generated artifacts are in scope?` |
| A05 | 0 | Near miss: `4. What exact files, directories, and generated artifacts are in scope?` No manifest question. |
| A06 | 1 | `4. What exact files, directories, and generated artifacts are in scope?` |
| A07 | 0 | Near miss: `12. Are tests, benchmarks, scripts, schemas, migrations, or observability assets part of the walkthrough?` No fixtures, and no included-source/evidence/excluded classification for both. |
| A08 | 0 | Near miss: `13. Should external dependencies be explained, and if so, how deeply?` and `4. ... generated artifacts...` Missing models and examples; no required separate classification set. |
| A09 | 0 | Near miss: `3. What repository, branch, commit, or snapshot should be treated as the source of truth?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `3. What repository, branch, commit, or snapshot should be treated as the source of truth?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `1. Who is the primary audience for this walkthrough?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `1. Who is the primary audience for this walkthrough?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `1. Who is the primary audience for this walkthrough?` No separate codebase-domain-familiarity question. |
| A14 | 1 | `9. What level of detail is expected...`; contract: `Depth rule: what must be covered in detail...` |
| A15 | 1 | `14. What output format is required...`; contract: `Deliverable format: document type...` |
| A16 | 0 | Near miss: `13. Should external dependencies be explained, and if so, how deeply?` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `15. What structural constraints apply... citation style?` and `17. What confidentiality or redaction rules apply...` No terminology-plus-prose-plus-citation set. |
| A18 | 0 | Near miss: `15. What structural constraints apply... required sections...` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `8. What level of source exactness is required...` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `18. What is the acceptance bar for completion and review?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `18. What is the acceptance bar for completion and review?` No independent editorial-review requirement. |

Total successes: 7

## Vale

| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `> Which subsystem, exactly, should the walkthrough cover?` |
| A02 | 1 | `Ask exactly one question per turn.` |
| A03 | 1 | `Record the user’s answer verbatim... and do not ask the next question until the current answer is confirmed.` |
| A04 | 1 | `5. What is explicitly in scope?` |
| A05 | 0 | Near miss: `5. What is explicitly in scope?` No manifest question. |
| A06 | 0 | Near miss: `5. What is explicitly in scope?` No later question explicitly identifies included production source. |
| A07 | 0 | Near miss: `13. What level of evidence do you want for claims: code citations only, code plus tests, or code plus runtime traces?` Mentions tests only; no fixtures and no required classification. |
| A08 | 0 | Near miss: `13. What level of evidence do you want for claims...` No separate classification for models, generated files, examples, and dependencies. |
| A09 | 0 | Near miss: `11. Is there a specific revision, branch, release, or environment the walkthrough must target?` No question about corpus changes during authoring. |
| A10 | 0 | Near miss: `11. Is there a specific revision, branch, release, or environment the walkthrough must target?` No question identifying the authoritative artifact if the corpus changes. |
| A11 | 0 | Near miss: `1. Who is the primary audience for this walkthrough?` No separate audience language-proficiency question. |
| A12 | 0 | Near miss: `1. Who is the primary audience for this walkthrough?` No separate systems/tooling-proficiency question. |
| A13 | 0 | Near miss: `12. Are there terminology, naming, or domain conventions I must preserve?` No separate audience domain-familiarity question. |
| A14 | 1 | `4. What depth do you want...`; contract: `Depth: overview, deep dive, or both.` |
| A15 | 1 | `3. What form should the final deliverable take?`; contract: `Deliverable: format and expected structure.` |
| A16 | 0 | Near miss: `10. Do you want the walkthrough anchored around architecture...` No question about what must remain usable if custom tooling disappears. |
| A17 | 0 | Near miss: `12. Are there terminology, naming, or domain conventions I must preserve?` and `13. What level of evidence do you want for claims...` No prose constraint, and the contract does not record the full prose/terminology/citation set. |
| A18 | 0 | Near miss: `3. What form should the final deliverable take?` No navigation or cross-reference constraints. |
| A19 | 0 | Near miss: `13. What level of evidence do you want for claims...` No separate mechanical-proof requirement apart from judgment. |
| A20 | 0 | Near miss: `15. What would make you reject the walkthrough as incomplete or wrong?` No independent technical-review requirement. |
| A21 | 0 | Near miss: `15. What would make you reject the walkthrough as incomplete or wrong?` No independent editorial-review requirement. |

Total successes: 6

## Aggregate success counts

| ID | success count |
|---|---:|
| A01 | 7 |
| A02 | 9 |
| A03 | 9 |
| A04 | 8 |
| A05 | 0 |
| A06 | 5 |
| A07 | 0 |
| A08 | 0 |
| A09 | 0 |
| A10 | 0 |
| A11 | 0 |
| A12 | 0 |
| A13 | 0 |
| A14 | 7 |
| A15 | 9 |
| A16 | 0 |
| A17 | 1 |
| A18 | 0 |
| A19 | 0 |
| A20 | 0 |
| A21 | 0 |
