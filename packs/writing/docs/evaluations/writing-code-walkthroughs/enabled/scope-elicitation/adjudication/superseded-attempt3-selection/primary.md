Raw event text matched the final-answer text for all nine samples.

### Alder
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What exact subsystem boundary should this walkthrough cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record the answer verbatim"` |
| A04 | 1 | `"What source corpus is in scope for explanation?"` |
| A05 | 0 | No manifests question; closest: `"What source corpus is in scope for explanation?"` |
| A06 | 1 | `"What source corpus is in scope for explanation?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What sources are authoritative if code, docs, tests, comments, and tickets disagree?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"What source corpus is in scope for explanation?"` |
| A09 | 0 | No corpus-change question; closest: `"What exact revision should be treated as the frozen target?"` |
| A10 | 1 | `"What exact revision should be treated as the frozen target?"` |
| A11 | 0 | No language-proficiency question; closest: `"Who is the primary audience for the walkthrough?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"Who is the primary audience for the walkthrough?"` |
| A13 | 0 | No domain-familiarity question; closest: `"Who is the primary audience for the walkthrough?"` |
| A14 | 1 | Q: `"What level of detail should explanations reach..."`; Contract: `"Detail level: architecture, module, function, or path-specific deep dives."` |
| A15 | 1 | Q: `"What output format and structure do you want?"`; Contract: `"Deliverable format: markdown/book/guide structure and required extras."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"Should the walkthrough include diagrams, tables, glossaries, or call graphs?"` |
| A17 | 1 | Qs: `"source-exact citations only"` and `"confidentiality, tone, terminology, or style constraints"`; Contract: `"Citation policy"` and `"Style constraints"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What output format and structure do you want?"` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What evidence standard do you want for claims...?"` |
| A20 | 0 | No independent technical review question; closest: `"What are the acceptance criteria for calling the walkthrough complete?"` |
| A21 | 0 | No independent editorial review question; closest: `"What are the acceptance criteria for calling the walkthrough complete?"` |

**Total successes: 9**

### Cobalt
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record it verbatim in the intake"` |
| A04 | 1 | `"What exact files, directories, modules, packages, or services are in scope?"` |
| A05 | 0 | No manifests question; closest: `"Should generated code, vendored code, migrations, config, and infrastructure files be included or excluded?"` |
| A06 | 1 | `"What exact files, directories, modules, packages, or services are in scope?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"Should tests be used as normative evidence of intended behavior?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"Should generated code, vendored code, migrations, config, and infrastructure files be included or excluded?"` |
| A09 | 0 | No corpus-change-during-authoring question; closest: `"Are version history and recent architectural changes part of the walkthrough, or should it be strictly snapshot-based?"` |
| A10 | 1 | `"What repository, branch, commit, tag, or release should be treated as the source of truth?"` |
| A11 | 0 | No language-proficiency question; closest: `"What prior knowledge may be assumed?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"What prior knowledge may be assumed?"` |
| A13 | 0 | No domain-familiarity question; closest: `"What prior knowledge may be assumed?"` |
| A14 | 0 | No explicit walkthrough-depth question recorded in contract; closest: `"What depth should be assumed for the intended reader?"` |
| A15 | 1 | Q: `"What output form do you want?"`; Contract: `"Deliverable shape: format..."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"Do you want diagrams, tables, call graphs, sequence narratives, or only prose?"` |
| A17 | 0 | No prose+terminology+citation set; closest: `"Are there naming, terminology, or product-language constraints I must preserve?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What chapter or section structure, if any, should be used?"` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What level of source fidelity is required?"` |
| A20 | 0 | No independent technical review question; closest: `"What are the acceptance criteria for signing off the walkthrough?"` |
| A21 | 0 | No independent editorial review question; closest: `"What are the acceptance criteria for signing off the walkthrough?"` |

**Total successes: 7**

### Ember
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `"Ask exactly one question per turn."` |
| A03 | 1 | `"Record the answer in a running intake record before asking the next question."` |
| A04 | 1 | `"What is explicitly in scope for the subsystem?"` |
| A05 | 0 | No manifests question; closest: `"What is explicitly in scope for the subsystem?"` |
| A06 | 0 | No production-source question; closest: `"What is explicitly in scope for the subsystem?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What evidence standard should I use: code only, code plus tests, code plus docs, or code plus runtime/config artifacts?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"code plus runtime/config artifacts"` |
| A09 | 0 | No corpus-change question; closest: `"Which repository, branch, commit, build, or deployment version should be treated as the source of truth?"` |
| A10 | 1 | `"Which repository, branch, commit, build, or deployment version should be treated as the source of truth?"` |
| A11 | 0 | Not separate; closest combined question: `"Who is the primary audience, and what is their assumed technical baseline?"` |
| A12 | 0 | Not separate; closest combined question: `"Who is the primary audience, and what is their assumed technical baseline?"` |
| A13 | 0 | Not separate; closest combined question: `"Who is the primary audience, and what is their assumed technical baseline?"` |
| A14 | 1 | Q: `"What depth do you want..."`; Contract: `"Depth level"` |
| A15 | 1 | Q: `"What deliverable shape do you want..."`; Contract: `"Required deliverable format"` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What deliverable shape do you want..."` |
| A17 | 0 | No prose+terminology+citation set; closest: `"What constraints apply to the walkthrough: time budget, length, confidentiality, redaction, or prohibited areas?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What deliverable shape do you want..."` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What evidence standard should I use..."` |
| A20 | 0 | No independent technical review question; closest: `"What counts as done for you?"` |
| A21 | 0 | No independent editorial review question; closest: `"What counts as done for you?"` |

**Total successes: 7**

### Harbor
| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | Quoted first turn bundles two asks: `"What exact subsystem are we walking through, and what outcome should the walkthrough enable for its audience?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record the answer"` |
| A04 | 1 | `"What is the scope boundary?"` |
| A05 | 0 | No manifests question; closest: `"What is the scope boundary?"` |
| A06 | 0 | No production-source question; closest: `"What is the scope boundary?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What evidence standard should be used for claims?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"Are there repository, privacy, compliance, or redaction constraints on what may be quoted or described?"` |
| A09 | 0 | No corpus-change question; closest: `"Should the walkthrough explain historical design intent and tradeoffs, or only current behavior?"` |
| A10 | 0 | No authoritative-artifact question; closest: `"Are there repository, privacy, compliance, or redaction constraints on what may be quoted or described?"` |
| A11 | 0 | Not separate; closest combined question: `"What is their starting level with the codebase, language, framework, and business domain?"` |
| A12 | 0 | Not separate; closest combined question: `"What is their starting level with the codebase, language, framework, and business domain?"` |
| A13 | 0 | Not separate; closest combined question: `"What is their starting level with the codebase, language, framework, and business domain?"` |
| A14 | 1 | Q: `"What level of depth do you want?"`; Contract: `"Output: exact deliverable format and target depth."` |
| A15 | 1 | Q: `"What deliverable format do you want?"`; Contract: `"Output: exact deliverable format and target depth."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"Do you want diagrams, tables, glossaries, or sequence narratives included?"` |
| A17 | 0 | No prose+terminology+citation set; closest: `"What tone should the final walkthrough use?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What deliverable format do you want?"` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What evidence standard should be used for claims?"` |
| A20 | 0 | No independent technical review question; closest: `"What acceptance criteria will make you say the walkthrough is complete?"` |
| A21 | 0 | No independent editorial review question; closest: `"What acceptance criteria will make you say the walkthrough is complete?"` |

**Total successes: 5**

### Juniper
| ID | score | citation/note |
|---|---:|---|
| A01 | 0 | Quoted first turn bundles two asks: `"Who is the primary audience for this walkthrough, and what should they be able to do after reading it?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record the answer verbatim"` |
| A04 | 1 | `"What exact subsystem should the walkthrough cover?"` |
| A05 | 0 | No manifests question; closest: `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A06 | 1 | `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A09 | 0 | No corpus-change question; closest: `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A10 | 0 | No authoritative revision/artifact question; closest: `"What repository, package, service, or directory defines the authoritative source corpus for that subsystem?"` |
| A11 | 0 | No language-proficiency question; closest: `"What level should it target: newcomer, working engineer, senior maintainer, or mixed audience?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"What level should it target: newcomer, working engineer, senior maintainer, or mixed audience?"` |
| A13 | 0 | No domain-familiarity question; closest: `"What level should it target: newcomer, working engineer, senior maintainer, or mixed audience?"` |
| A14 | 0 | No explicit walkthrough-depth question recorded in contract; closest: `"Do you want a complete coverage walkthrough... or a selective walkthrough...?"` |
| A15 | 1 | Q: `"What output form do you want..."`; Contract: `"Document form: target artifact and structure expectations."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What supporting artifacts are required..."` |
| A17 | 0 | No prose+terminology+citation set; closest: `"What constraints should govern the walkthrough: length limit, tone... citation style, or house style?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What should the organizing spine be..."` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What correctness bar should the walkthrough meet before delivery...?"` |
| A20 | 0 | No independent technical review question; closest: `"What final acceptance criteria will tell you the walkthrough is complete?"` |
| A21 | 0 | No independent editorial review question; closest: `"What final acceptance criteria will tell you the walkthrough is complete?"` |

**Total successes: 5**

### Lichen
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which subsystem, by exact name or path, do you want the walkthrough to cover?"` |
| A02 | 1 | `"I would ask exactly one question per turn"` |
| A03 | 1 | `"record your answer, and only then ask the next question"` |
| A04 | 1 | `"What are the hard scope boundaries: included components, excluded components..."` |
| A05 | 0 | No manifests question; closest: `"Are there repository areas or file types that must not be inspected or cited?"` |
| A06 | 0 | No production-source question; closest: `"What are the hard scope boundaries: included components, excluded components..."` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"Are there repository areas or file types that must not be inspected or cited?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"Should the walkthrough include data flow, control flow, state transitions, error handling, concurrency, and external dependencies, or only a subset?"` |
| A09 | 0 | No later question on corpus change; closest near-miss is contract: `"if scope changes after freeze, I stop and renegotiate the contract"` |
| A10 | 0 | No authoritative-artifact question; closest near-miss is contract: `"if scope changes after freeze, I stop and renegotiate the contract"` |
| A11 | 0 | Not separate; closest combined question: `"Who is the intended reader, and what is their expected technical baseline?"` |
| A12 | 0 | Not separate; closest combined question: `"Who is the intended reader, and what is their expected technical baseline?"` |
| A13 | 0 | Not separate; closest combined question: `"Who is the intended reader, and what is their expected technical baseline?"` |
| A14 | 1 | Q: `"What depth should the walkthrough reach..."`; Contract: `"Depth: fixed analysis level and artifact format."` |
| A15 | 1 | Q: `"What output shape do you want..."`; Contract: `"Depth: fixed analysis level and artifact format."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What artifacts are allowed in the final deliverable..."` |
| A17 | 0 | No prose+terminology+citation set; closest: `"What level of certainty labeling do you want for inferred behavior versus directly evidenced behavior?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What output shape do you want: narrative walkthrough, annotated call flow... file-by-file map..."` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What level of certainty labeling do you want for inferred behavior versus directly evidenced behavior?"` |
| A20 | 0 | No independent technical review question; closest: `"What acceptance criteria will tell you the walkthrough is complete and usable?"` |
| A21 | 0 | No independent editorial review question; closest: `"What acceptance criteria will tell you the walkthrough is complete and usable?"` |

**Total successes: 6**

### Quartz
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which exact subsystem should this walkthrough cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"wait for the user's answer, record it"` |
| A04 | 1 | `"What is in scope for the walkthrough?"` |
| A05 | 0 | No manifests question; closest: `"What is in scope for the walkthrough?"` |
| A06 | 0 | No production-source question; closest: `"What is in scope for the walkthrough?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What evidence standard should I use for claims?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"What should be excluded even if it appears technically relevant?"` |
| A09 | 0 | No corpus-change question; closest near-miss: no later question on change control or authoritative revision |
| A10 | 0 | No authoritative-artifact question; closest near-miss: no later question on source-of-truth revision |
| A11 | 0 | No language-proficiency question; closest: `"Who is the intended audience?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"Who is the intended audience?"` |
| A13 | 0 | No domain-familiarity question; closest: `"Who is the intended audience?"` |
| A14 | 1 | Q: `"What level of depth is required?"`; Contract: `"Depth: overview, implementation detail, operational detail, and failure-mode coverage."` |
| A15 | 1 | Q: `"What form should the final deliverable take?"`; Contract: `"Deliverable: format, structure, and expected artifacts."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What form should the final deliverable take?"` |
| A17 | 0 | No prose+terminology+citation set; closest: `"Are there required conventions for terminology, naming, or diagrams?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What form should the final deliverable take?"` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What evidence standard should I use for claims?"` |
| A20 | 0 | No independent technical review question; closest: `"Are there constraints on time, length, or reviewability?"` |
| A21 | 0 | No independent editorial review question; closest: `"Are there constraints on time, length, or reviewability?"` |

**Total successes: 6**

### Sable
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"What exact subsystem should the walkthrough cover?"` |
| A02 | 1 | `"ask exactly one question per turn"` |
| A03 | 1 | `"record the answer verbatim"` |
| A04 | 1 | `"What exact files, directories, and generated artifacts are in scope?"` |
| A05 | 0 | No manifests question; closest: `"What exact files, directories, and generated artifacts are in scope?"` |
| A06 | 1 | `"What exact files, directories, and generated artifacts are in scope?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"Are tests, benchmarks, scripts, schemas, migrations, or observability assets part of the walkthrough?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"Should external dependencies be explained, and if so, how deeply?"` |
| A09 | 0 | No corpus-change question; closest: `"What repository, branch, commit, or snapshot should be treated as the source of truth?"` |
| A10 | 1 | `"What repository, branch, commit, or snapshot should be treated as the source of truth?"` |
| A11 | 0 | No language-proficiency question; closest: `"Who is the primary audience for this walkthrough?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"Who is the primary audience for this walkthrough?"` |
| A13 | 0 | No domain-familiarity question; closest: `"Who is the primary audience for this walkthrough?"` |
| A14 | 1 | Q: `"What level of detail is expected..."`; Contract: `"Depth rule: what must be covered in detail and what can be summarized."` |
| A15 | 1 | Q: `"What output format is required..."`; Contract: `"Deliverable format: document type, structure, and length constraints."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What output format is required..."` |
| A17 | 0 | No prose+terminology+citation set; closest: `"What structural constraints apply: required sections, maximum length, diagram policy, snippet policy, citation style?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What structural constraints apply..."` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What level of source exactness is required for quoted snippets and symbol names?"` |
| A20 | 0 | No independent technical review question; closest: `"What is the acceptance bar for completion and review?"` |
| A21 | 0 | No independent editorial review question; closest: `"What is the acceptance bar for completion and review?"` |

**Total successes: 8**

### Vale
| ID | score | citation/note |
|---|---:|---|
| A01 | 1 | `"Which subsystem, exactly, should the walkthrough cover?"` |
| A02 | 1 | `"Ask exactly one question per turn."` |
| A03 | 1 | `"Record the user’s answer verbatim"` |
| A04 | 1 | `"What is explicitly in scope?"` |
| A05 | 0 | No manifests question; closest: `"What is explicitly in scope?"` |
| A06 | 0 | No production-source question; closest: `"What is explicitly in scope?"` |
| A07 | 0 | No tests-and-fixtures classification; closest: `"What level of evidence do you want for claims: code citations only, code plus tests, or code plus runtime traces?"` |
| A08 | 0 | No models/generated/examples/dependencies classification; closest: `"What is explicitly in scope?"` |
| A09 | 0 | No corpus-change question; closest: `"Is there a specific revision, branch, release, or environment the walkthrough must target?"` |
| A10 | 1 | `"Is there a specific revision, branch, release, or environment the walkthrough must target?"` |
| A11 | 0 | No language-proficiency question; closest: `"Who is the primary audience for this walkthrough?"` |
| A12 | 0 | No systems/tooling-proficiency question; closest: `"Who is the primary audience for this walkthrough?"` |
| A13 | 0 | No domain-familiarity question; closest: `"Who is the primary audience for this walkthrough?"` |
| A14 | 1 | Q: `"What depth do you want..."`; Contract: `"Depth: overview, deep dive, or both."` |
| A15 | 1 | Q: `"What form should the final deliverable take?"`; Contract: `"Deliverable: format and expected structure."` |
| A16 | 0 | No custom-tooling-fallback question; closest: `"What form should the final deliverable take?"` |
| A17 | 0 | No prose+terminology+citation set; closest: `"Are there terminology, naming, or domain conventions I must preserve?"` |
| A18 | 0 | No navigation/cross-reference constraints; closest: `"What form should the final deliverable take?"` |
| A19 | 0 | No mechanical-proof-vs-judgment question; closest: `"What level of evidence do you want for claims..."` |
| A20 | 0 | No independent technical review question; closest: `"What would make you reject the walkthrough as incomplete or wrong?"` |
| A21 | 0 | No independent editorial review question; closest: `"What would make you reject the walkthrough as incomplete or wrong?"` |

**Total successes: 7**

### Aggregate Success Counts
| ID | success count |
|---|---:|
| A01 | 7 |
| A02 | 9 |
| A03 | 9 |
| A04 | 9 |
| A05 | 0 |
| A06 | 4 |
| A07 | 0 |
| A08 | 0 |
| A09 | 0 |
| A10 | 5 |
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
