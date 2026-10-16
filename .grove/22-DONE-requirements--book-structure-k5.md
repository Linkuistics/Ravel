# book-structure-k5


## Goal

Agree with the human on the structure and reader contract for a book (user guide)
about Ravel and its components, written for a software developer with no
background in this context. Produce the teaching/coverage and delivery agreement
needed for later drafting, not a manuscript or a speculative editorial tree.



## Context

- The human made the book/audience request during `component-contracts-k3` on
  2026-10-14; the root brief records it. General software experience does not
  establish Rust, TypeScript, job-control, policy-runtime or diagram knowledge.
- `docs/components.md` explains current responsibilities and normal/failing
  composition. The CLI/specification, configuration, result-channel, runner and
  delivery guides hold their contracts. Read `result-job-boundary-k4` and its
  supported continuation before assuming a future component/package exists.
- Use the root's pinned editorial intake and source design
  `docs/specs/documentation-guidance.md` at
  `7ca78f01b4d0e8ec3d19cafaaf2f4ae51d75b246` in the named upstream checkout,
  with its evidence limits. Do not rerun unfinished evaluations or infer proven
  comprehension from historical results.
- Current component diagrams are a source explanation, not agreement on book
  figures, media or chapter order. Human usefulness remains unassessed.

## Done when

- The reader model separates domain, Rust, TypeScript, shell/process/terminal
  and notation familiarity. Teach from the stated domain starting point without
  explaining routine development unnecessarily or assuming language proficiency.
- The human agrees substantive chapter order, learning/direct entry routes,
  chapter purposes, emphasis and exclusions. Recommended opening: Ravel's
  problem, vocabulary/actors and a complete session; then practical configuration,
  component contracts/composition, a source walkthrough, independent capabilities
  and project development/delivery. This recommendation is not an approved outline.
- Practical outcomes include configuring/running a session; explaining why
  TypeScript and the Rust packages exist; following success/refusal/cancellation
  through the code; using a supported extracted capability; and building/testing/
  delivering the project. Establish prerequisites and runnable examples against
  resulting interfaces, using fake programs rather than real models.
- Coverage claims use subject-derived inventories for component obligations,
  CLI/configuration inputs, lifecycle/failure scenarios and contributor workflows.
  Name omissions and unknowns; inventory coverage does not prove comprehension.
- Agree a named prose contract, terminology risks, example/figure questions,
  admitted media/formats, book delivery and actual viewing environments. Recheck
  dated tool assumptions if they affect delivery; do not choose a framework just
  because it renders Markdown. Reconcile existing guides with the book so
  authoritative contracts do not fork.
- Arrange the required short human practical reader checkpoint after proof,
  outside draft → copy-edit → art → proof. Record the reader's words and
  find/explain/use performance, coverage and unknowns. Mechanical and model-proxy
  results remain separate from human evidence.
- Save the agreed book contract under Grove's membership test. Cut only the next
  precise drafting or prerequisite planning task; do not invent the whole
  editorial tree before its artifacts and ownership questions are known.

## Notes

This is the root's human documentation-structure agreement. Do not re-ask the
settled book deliverable or domain-newcomer audience question. Language expertise,
chapter order and delivery medium remain open and merit recommendations/evidence.

If preceding design leaves consequential contract uncertainty, insert its precise
design/observation continuation before this leaf. A supported target can inform
structure while implementation is pending; drafting/examples wait for relevant
stable interfaces. The root's independent-use and compatibility work remains
required; the book request does not cancel it.

## Decisions (running log)

- **Reader expertise.** Put as three profiles beyond Ravel's own concepts
  (shell-fluent generalist, recommended; systems programmer; TypeScript
  developer), with the evidence that a policy is a TypeScript module, the code
  walkthrough crosses about 17,500 lines of Rust in the two `src` trees, and
  Ravel's promises are stated in job-control terms. The human answered verbatim
  **"1 and Typescript and Rust"**. Recorded meaning, stated back to the human:
  the reader uses a Unix shell, PATH, environment variables, Git and JSON daily
  and reads and writes both TypeScript and Rust. Not assumed: Ravel's domain;
  Unix job control (process groups, INT/TERM/HUP beyond Ctrl-C, reaping, the
  controlling terminal's foreground group and saved modes, PTYs); the Bun
  runtime's compile, autoload and resolution behaviour; the `libc` job-control
  calls; diagram notation. The book teaches Ravel's concepts from zero and the
  process and terminal model as part of Ravel's subject, explains Bun- and
  `libc`-specific behaviour where Ravel depends on it, and explains each diagram
  kind where it first appears. It does not teach TypeScript or Rust language
  features, shell use, Git or JSON.
- **The book and today's guides.** Put with the evidence that contracts are
  already stated in several places and already disagree: exit codes and bounds
  appear in the CLI guide, the specification and `--help`, and
  `docs/RELEASING.md` still says development is 0.0.0 with a first release
  planned at 0.1.0 while the workspace is 1.0.0 and the changelog records
  v1.0.0. Options: replace the guides and keep the specifications normative
  (recommended); book teaches and the guides stay; book absorbs the
  specifications too. The human chose **"Replace guides; specs stay
  (Recommended)"**. Meaning: the CLI, owner configuration, runner,
  result-channel, component, release and authority guides fold into the book
  and are removed, with every inbound link redirected, and the README becomes a
  short entry into the book. `docs/specs/session-execution.md`,
  `docs/specs/result-job-completion.md` and the three decision records stay as
  the normative design record; every behaviour the book states links to the
  rule it restates. Evidence documents stay outside the book and are cited only
  where a limit matters to the reader.
- **Chapters, order and entry routes.** Put as a twelve-chapter, four-part
  table with each chapter's purpose stated as what the reader can do afterwards,
  the direct entry routes, and the reasons (the human's own lifecycle-first
  direction; practice before internals; whole, then path, then parts).
  Alternatives: internals before practice; two separate books. The human chose
  **"Practice, then internals (Recommended)"**, with the preview: Part I *Ravel
  and its session* (1 What Ravel is for; 2 The actors and their words; 3 A
  complete session, end to end), Part II *Using Ravel* (4 Writing an owner
  policy; 5 Running sessions; 6 Reference), Part III *Inside Ravel* (7 How Ravel
  is built; 8 Following a session through the code; 9 Jobs, signals and the
  terminal), Part IV *Reusing and developing Ravel* (10 Using Ravel's components
  in your project; 11 Developing Ravel; 12 Delivering Ravel). The chapter
  purposes and entry routes agreed with it are the ones put in the question and
  carried into the book contract: the learning route is 1 to 12; someone
  running sessions from their own tool reads 1–3 then 5; a policy author reads
  1–3 then 4; someone reusing a component reads 1, 2 then 10, with 7 and 9 as
  background; a contributor reads 1–3, 7, 8 then 11. Every reference entry,
  exit code, error message and figure is a direct entry with local context and
  a link back to where its subject is taught.
- **Emphasis and exclusions.** Proposed emphasis: what success means (a result
  only after reap, group absence and cleanup; refusal, cancellation, natural
  exit without a result and cleanup failure as normal outcomes); who holds which
  authority; validating with fake agents; the stated limits and the support
  statement. Proposed exclusions: history (one-line provenance pointer at most);
  the grove's evidence campaigns (limits cited only); real agent setup (real
  programs named only as examples); the owner's personal configuration record
  (kept outside the book, not deleted); Grove's conventions (Grove at most one
  labelled illustrative caller); undelivered contracts (the persistent runner,
  native x86-64 qualification) as limits only. Alternatives: add real agent
  setup; add a short history. The human chose **"Add real agent setup"**, whose
  stated meaning was: as proposed, but chapter 4 also shows routing to real
  agent CLIs (for example Claude Code and Codex) with their actual flags, still
  validated with fakes. Recorded consequences: the emphasis and every other
  exclusion stand as proposed. Chapter 4 shows real routes for the agent CLIs
  the owner's policy routes to today, `claude` (Claude Code) and `codex`
  (Codex), with their actual command-line flags and the ending instructions a
  caller's prompt gives them. Because those flags belong to other projects and
  drift, each real-agent route records the agent version and the date its
  flags were checked against that agent's own help, and the book's runnable
  checks validate the routes through fake executables of the same names on a
  private PATH; no example launches a real agent or model. Installing those
  agents, their accounts and credentials stay out of scope, with a pointer to
  each agent's own documentation.
- **When drafting starts.** Put with the facts that the component redesign and
  extraction the root asks for have no selected target and no live leaf (k4
  retained the incumbent; k22's closure left the returning host, independent
  adoption, the comparison, extraction and delivery as scope questions for a
  requirements session with the human); that chapters 1–6 describe the
  compatibility-protected surface; that chapters 7–9, 11 and 12 describe this
  workstream's code, which is 48 commits past the v1.0.0 release (`main` is
  still at v1.0.0) and not yet merged or released; and that chapter 10 needs a
  supported extracted component, while both Rust packages are `publish =
  false`. Options: draft the current code now (recommended); stable parts now
  and the rest later; redesign first, then the book. The human chose **"Draft
  current code now (Recommended)"**. Meaning: chapters 1–9, 11 and 12 are
  drafted against this workstream's code; chapter 10 is drafted only once a
  supported extracted component exists. The component-work requirements step
  (returning host, independent adoption, the unfinished comparison, extraction
  and delivery) comes after the book's practical reader check, so that it can
  use what the reader found hard in the code. Any later change to components
  updates the chapters it touches as part of its own work. This replaces the
  root brief's earlier order, in which target and migration design preceded
  documentation drafting.
- **Format and viewing environments.** Facts rechecked on 2026-10-16 and put
  with the question: GitHub renders Markdown, relative links, anchors, tables
  and SVG images and draws Mermaid blocks itself without stating a Mermaid
  version (GitHub Docs, "Creating diagrams"); mdBook's latest release is v0.5.4
  of 6 July (GitHub releases page, which omits the year; 2026 by release order),
  not installed here; d2 0.9.0 is installed here; this workstream's commits are
  not on GitHub, so GitHub's rendering can only be checked after a push.
  Options: mdBook site plus sources (recommended); Markdown on GitHub only;
  GitHub with native Mermaid. The human answered verbatim **"option 1, but
  including a range of diagram technologies such as C4 / SysUML etc"**. Recorded
  meaning, stated back to the human: chapters are Markdown under `docs/book/`
  with mdBook's contents file; `task setup` fetches a pinned mdBook 0.5.4 and
  `task docs:book` builds and serves it; the built book is published to GitHub
  Pages with each release. Declared viewing environments: the published site,
  GitHub's view of the sources, and a local checkout. Figures are committed SVG
  beside their editable text source, each with its renderer version and
  regeneration command, checked by a regenerate-and-compare task. The admitted
  notations widen from D2 alone to a range chosen per reader question: the C4
  model (system context, container and component levels, and dynamic views);
  UML diagram kinds (sequence, state machine, activity, component, deployment,
  timing); SysML diagram kinds where one answers a question better than its UML
  counterpart ("SysUML" read as UML and SysML); and others where a question
  calls for them. Each notation is explained where the book first uses it.
  Admitted renderers: PlantUML 1.2026.8, using its bundled C4-PlantUML 2.13.0
  library and its built-in Smetana layout so that no separate Graphviz is
  needed, for C4, UML and SysML-style diagrams; and d2 0.9.0 for structure and
  flow diagrams those notations do not fit. A probe in the session scratchpad
  rendered a C4 container diagram and a sequence diagram with PlantUML, and one
  D2 diagram, twice each; each pair of SVGs was byte-identical. That supports,
  but does not establish, regeneration stability for every figure; the
  regenerate-and-compare task checks each one.
- **Prose contract.** Put with the house style's content (the closed list
  agreed on 2026-09-30 in the pinned documentation design, §9), the fact that
  it is not installed in this checkout's Grove, the spelling count across the
  README, glossary and docs ("behavior" 23 against "behaviour" 10;
  "authoris-" 15 against "authoriz-" 3; the glossary writes "authorised"), and
  the recommendation of no normative keywords. Options: house style with
  British spelling (recommended); house style with US spelling; Google's guide
  in full. The human chose **"House style, British (Recommended)"**. Meaning:
  the base is Grove's house style, whose points the book's contract carries
  with attribution and the pinned source rather than by reference to another
  checkout; spelling is British (behaviour, authorise) in prose, while code
  identifiers keep their own spelling in code font; there are no BCP 14
  keywords, because every behaviour the book states links to its rule in the
  specification; the term list is `CONTEXT.md` plus the book's term risks.
- **Who reads at the practical checkpoint.** Put with the proposed tasks (write
  a policy routing to a fake agent, run a session, interpret a refusal and a
  cancellation; explain the TypeScript worker and the two Rust packages and
  follow one session through the code to where cancellation is settled; build
  and test Ravel from a checkout; use an extracted component, deferred until
  chapter 10 exists), the entry surfaces (chapter 1 plus at least one direct
  entry such as an exit code or error message) and the environment (the built
  book, figures confirmed to load first). Options: a developer new to Ravel
  (recommended); the human as a labelled stand-in; a newcomer then the human.
  The human chose **"You, as a labelled stand-in"**, whose stated meaning was
  that the human does the tasks themselves, recorded as stand-in evidence.
  Consequence recorded: every checkpoint result is labelled as the owner's
  stand-in read. It can establish accuracy, coverage and the owner's own
  find/explain/use results, but not whether a developer new to Ravel can find,
  understand and use the book; that remains a named unknown in the contract and
  in every report of the read, never inferred from the stand-in's results or
  from mechanical or model-proxy checks.
- **When the reads happen.** Put with the dependency of chapters 7–12 on the
  model chapters 1–3 teach and the pinned rule that an early comprehension
  failure stops the read and routes a bounded correction to its owning stage
  before dependent work. Options: after each half (recommended); once, after
  the whole book. The human chose **"After each half (Recommended)"**. Meaning:
  read 1 follows proof of Parts I and II and covers configuring and running a
  session, interpreting a refusal and a cancellation, and one direct entry; its
  corrections land before Parts III and IV are drafted. Read 2 follows proof of
  Parts III and IV and covers following a session through the code and building
  and testing the project. Chapter 10's task gets its own read once that
  chapter exists. Each read is a `requirements` leaf placed after the proof it
  follows, outside draft → copy-edit → art → proof.
- **Restatement at the agreement point.** After the full summary (the nine
  decisions above; derived term risks, coverage inventories with unknowns,
  examples policy and figure questions; and what would be written and cut), the
  human was asked to restate the agreement and answered verbatim: **"This book
  is user guide, for both how to use Ravel, and the interal architecture and
  implementation. It allows a software engineer familiar with shell scripting,
  Typescript, and Rust, plus general OS knowledge, to use, and potentially
  modify, maintain and reuse Ravel components. It is written in simple
  language, without requiring the user to follow links to read ADRs or Specs.
  These can be useful, but at not required for understanding at the book's
  level. It is not a reference work, but instead has a strong pedagogical
  focus."** Reconciled with the decisions above and stated back to the human:
  (a) "general OS knowledge" refines the reader model. Assumed: general OS
  concepts (processes and PIDs, exec, exit status, signals as a mechanism, file
  descriptors and pipes, permissions). Still taught, as Ravel's subject: the
  job-control mechanics Ravel depends on (process groups, the controlling
  terminal's foreground group and saved modes, reaping and PID reuse, signal
  dispositions and masks across exec, PTYs as the tests use them, the
  confinement backends). (b) The book is self-contained and in simple
  language: every behaviour and design reason the reader needs is explained in
  the book itself; links to the specifications and decision records are
  optional depth, never needed to follow the text, and are kept out of the way
  of the teaching. The fork protection of the guides decision is carried by
  proof checking each restated behaviour against its rule, not by requiring the
  reader to follow links. (c) "It is not a reference work" conflicts with the
  agreed Reference chapter and with the proposed complete-coverage claim for
  CLI and configuration inputs, so it was put back to the human (next entry).
- **"Not a reference work" against the Reference chapter.** Put with the
  facts that the session specification already states the bounds, exit codes,
  settings keys, data shapes and environment rules and that `start-session
  --help` lists every flag with its range. Options: drop the chapter
  (recommended); a short lookup appendix; keep chapter 6 as agreed. The human
  chose **"Keep chapter 6 as agreed"**, whose stated meaning was that the
  restatement describes the book's focus and a reference chapter can sit beside
  the teaching. Settled: the twelve chapters stand, chapter 6 is the one lookup
  chapter, the coverage inventories stand as proposed, and the rest of the book
  is pedagogical. Chapter 6's entries are self-contained like every other page:
  each states its value or behaviour in the book, with any link to the
  specification as optional depth.
- **Shared understanding.** The human's restatement, its reconciliation and
  the answer above settle every point. The summary's derived items (term risks,
  coverage inventories and unknowns, examples policy, figure questions, the
  contract's location, the cuts, and the consequence that merging and releasing
  this workstream's repairs waits for the component-work scope step) were put
  with an invitation to correct any of them in the same reply; none was
  corrected. The contract is saved as `docs/book/contract.md`.
- **Corrections while writing, against the source.** Three statements put to
  the human were sharpened for technical truth without changing any decision:
  owner policy is *given* no result authority (it never receives a
  session-result context and its worker is reaped before launch), rather than
  "can never publish or end a session", since trusted owner code is not
  sandboxed; an *unconfined* managed process stays in the caller's POSIX
  session in a process group of its own, while a confined launch calls
  `setsid` (the runner's mode check, not "interactive"); and a refusal is
  defined by what it is, because settings and CLI input problems exit 2 while
  policy-side refusals exit 3.
- **Glossary.** `CONTEXT.md` gains Session, Kind, Policy context, Refusal and
  Harness, and a flagged-ambiguities section for session, context, kind,
  channel, worker, policy and result, matching the contract's term risks.
  `docs/caller-references.md` gains a row classifying the two new surfaces'
  references to Grove; a sweep of both files with a positive control (the
  authority note's known references) and a mutated pattern observed to miss
  them found only documentation references.
- **Cuts.** `ravel-book-k55` is a node at the root whose brief charters the
  book, with first child `ravel-book-first-half-k56` (planning: scaffold,
  drafting units for chapters 1–6, guide migration, the `--policy-env` gap and
  read 1). `component-work-scope-k57` (requirements) follows the node at the
  root. The root brief's forward-looking paragraphs now state this order. No
  review leaf is cut for this agreement: the human made every decision and
  restated the agreement, and the book's proof and the two stand-in reads
  check what the contract produces.
