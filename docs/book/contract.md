# The Ravel book: document contract

This contract is the agreement the book is written to. It has four parts: the
**structure brief** (who the book is for, what it covers and in what order),
the **prose contract** (how its sentences are written), **format and media**
(how it is built, illustrated and read), and the **reader checkpoints** (who
reads it with practical tasks, and when). The owner decided the structure in
the book-structure agreement of 2026-10-16. Every session that drafts,
copy-edits, illustrates or proofs the book reads this file. A change to the
book's structure is a change to this contract, made with the owner and written
here in place.

The contract's shape follows Grove's documentation-guidance design (Linkuistics
grove, `docs/specs/documentation-guidance.md` at commit
`7ca78f01b4d0e8ec3d19cafaaf2f4ae51d75b246`). Everything the book needs from
that design is stated in this file, so the book depends on no other checkout.

In the owner's words: "This book is user guide, for both how to use Ravel, and
the interal architecture and implementation. It allows a software engineer
familiar with shell scripting, Typescript, and Rust, plus general OS
knowledge, to use, and potentially modify, maintain and reuse Ravel
components. It is written in simple language, without requiring the user to
follow links to read ADRs or Specs. These can be useful, but at not required
for understanding at the book's level. It is not a reference work, but
instead has a strong pedagogical focus."

## 1. Structure brief

### The reader

The reader is a 16-year-old software engineer with no background in Ravel.
Assume the programming knowledge below, and write in plain, direct language.
Do not assume familiarity with specialist systems terminology beyond the
stated prerequisites.

- **Assumed, and not taught:** everyday shell use (PATH, environment
  variables, quoting, pipes), Git and JSON; reading and writing TypeScript and
  Rust; general OS concepts: processes and PIDs, exec, exit status, signals as
  a mechanism, file descriptors and pipes, and file permissions; and all
  diagram types used in the book. Readers unfamiliar with a notation can look
  it up; the book explains only its own figure conventions.
- **Not assumed, and taught where the book first needs it:**
  - Ravel's own domain: callers, owners, owner policy, sessions, kinds,
    results, and who holds which authority.
  - The job-control mechanics Ravel depends on: process groups, the
    controlling terminal and its foreground process group, saved terminal
    modes, reaping and why PID reuse makes it matter, signal dispositions and
    masks across exec, PTYs as the tests use them, and the confinement
    backends (`sandbox-exec` on macOS, bubblewrap on Linux).
  - The Bun runtime's compile, autoload and module-resolution behaviour, where
    Ravel depends on it.
  - The `libc` job-control calls the runner makes.
- **Never taught:** TypeScript or Rust language features, shell, Git, JSON,
  general programming, diagram notation, or the general OS concepts above.

### What the reader can do afterwards

- Configure and run a session: write, typecheck and validate an owner policy
  with fake agents, route kinds to real agent programs, run sessions, and
  interpret every outcome and exit code.
- Explain why Ravel has a TypeScript policy worker and two Rust packages, and
  what each part owns.
- Follow a successful, a refused and a cancelled session through the code, and
  find where a change belongs.
- Use a supported extracted component through its interface, once chapter 10
  exists.
- Build, test and deliver the project.
- Look up any CLI input, setting, bound, data shape, environment variable or
  installed path in chapter 6.

### The intended domain

Ravel is a generic policy router and session supervisor. A **caller** asks for
a session of some kind; the **owner's** executable TypeScript policy chooses
the program to run; Ravel launches that program as a supervised job and,
once the job has been completely cleaned up, returns one opaque result or
reports why it cannot. The programs it
launches are often AI coding agents (harnesses), and its first caller is the
Grove task-tree driver, but neither defines the domain: Ravel assigns no
meaning to kinds or results, integrates no model provider, and stores nothing.
The book keeps these roles apart:

- the **caller**, which invokes Ravel, supplies the prompt and its ending
  instructions, and interprets the result;
- the **owner**, who writes the policy and settings for their account;
- **Ravel** itself: the Ravel CLI, the policy worker, the runner and the
  result channel;
- the **managed process**, the selected program and its descendants;
- **contributors**, who develop and deliver Ravel;
- **component consumers**, who reuse an extracted component in another
  project.

### The section plan

The book is pedagogical: every chapter teaches toward a reader task.
Chapter 6 is the one lookup chapter, and its entries are self-contained like
every other page. Each chapter's purpose is what the reader can do after it.

| # | Chapter | After it, the reader can… |
|---|---|---|
| **Part I** | **Ravel and its session** | |
| 1 | What Ravel is for | say what Ravel solves, what it leaves to callers (no store, no task meaning, no model integration), and whether it fits their need |
| 2 | The actors and their words | name the caller, owner, owner policy, policy worker, managed process and opaque result, and say who holds which authority |
| 3 | A complete session, end to end | run a real session with a tiny policy and a fake agent, then describe its stages; the process and terminal ideas are introduced here as the session needs them |
| **Part II** | **Using Ravel** | |
| 4 | Writing an owner policy | write, typecheck and validate a policy with fake agents, covering the SDK, refusals, policy context, settings, environment grants, routes to real agent programs and copying an existing policy |
| 5 | Running sessions | run sessions under real conditions (prompts, kinds, caller data, nesting, confinement, notices) and interpret every outcome and exit code |
| 6 | Reference | look up any CLI input, setting, bound, request, selection or context shape, environment variable or installed path |
| **Part III** | **Inside Ravel** | |
| 7 | How Ravel is built | explain the Ravel CLI, why policy runs as a separate TypeScript worker that is reaped before anything launches, why the runner is its own Rust package, and how the parts compose |
| 8 | Following a session through the code | follow one successful session from `main` to the result write, then the refusal and cancellation forks and the error and signal precedence, and find where a change belongs |
| 9 | Jobs, signals and the terminal | reason about process groups, escalation, reaping, terminal lending and recovery, confinement, entry signal state, and the failure guarantees with their limits |
| **Part IV** | **Reusing and developing Ravel** | |
| 10 | Using Ravel's components in your project | use an extracted component through its interface and meet its obligations, without Ravel's policy or session conventions |
| 11 | Developing Ravel | set up a checkout, build, use the test seams and `task check`, add a test, and know what the evidence documents are for |
| 12 | Delivering Ravel | handle versions and identity, install and smoke tests, archives and platform floors, the formula and release tasks, and the supported targets with their limits |

Chapters 1–9, 11 and 12 describe the code as it stands in the workstream that
wrote this contract. Chapters 1–6 describe the CLI, configuration and session
behaviour that Ravel's compatibility promise keeps unchanged. Chapters 7–9, 11
and 12 describe structure, packages and delivery that later component work may
change; such work updates the chapters it touches as part of its own change.
Chapter 10 is written by the work that delivers a supported extracted
component, together with that component's interface and consumer, and not
before.

Chapter 8 is an orientation that follows one session through the code. It
quotes source excerpts but does not cover the complete source.

### Entry routes

- **The learning route** reads chapters 1 to 12 in order.
- **Running sessions from your own tool:** chapters 1–3, then 5, with 6 for
  lookup.
- **Writing a policy:** chapters 1–3, then 4, with 6 for lookup.
- **Reusing a component:** chapters 1 and 2, then 10, with 7 and 9 as
  background.
- **Changing Ravel:** chapters 1–3, 7 and 8, then 11, with 9 and 12 as needed.
- **Direct entries:** every chapter 6 entry, every exit code, every error
  message the book explains, and every figure is a place a reader may arrive
  without reading what comes before. Each carries the local context it needs and
  links back to where its subject is taught.

### Emphasis

- **What success means.** A result reaches the caller only after the managed
  process has been reaped, its whole process group is gone, and cleanup has
  succeeded. Refusal, cancellation, natural exit without a result and cleanup
  failure are distinct, normal outcomes, not edge cases.
- **Who holds which authority.** Owner policy is trusted code that chooses
  the program but is given no result authority: it never receives a
  session-result context, and its worker is reaped before anything launches.
  Only the Ravel CLI supervises. Each managed process holds only its own
  result context.
- **Validating with fake agents.** Every runnable example uses fake programs,
  and a policy author learns to check every route and refusal that way.
- **The stated limits, said plainly:** same-user tampering, abrupt death of
  the Ravel CLI, descendants that leave their process group, detached policy
  children, and the support statement (aarch64 macOS and aarch64 Linux are
  measured; x86_64 Linux is built and run only under emulation).

### Exclusions

- **History.** The retired actor-daemon project, the dispatch history and
  Ravel's extraction from Grove get at most a one-line provenance pointer.
- **Evidence campaigns.** Campaign plans, cases, receipts and check logs stay
  as evidence documents outside the book. The book states only the limits a
  reader needs, such as the support statement.
- **Installing and configuring agents.** Chapter 4 shows routes to the agent
  programs the owner's policy uses today, Claude Code (`claude`) and Codex
  (`codex`), with their actual command-line flags and the ending instructions a
  caller's prompt gives them. Installing those agents, their accounts and
  credentials are out of scope, with a pointer to each agent's own
  documentation.
- **The owner's personal configuration.** The dated handoff record of the
  owner's routing table in today's configuration guide is not teaching
  material. It stays outside the book and is kept, not deleted, when that guide
  is folded in.
- **Grove's conventions.** Grove may appear as one illustrative caller,
  labelled as such. Its task kinds and result values are never presented as
  Ravel's.
- **Undelivered contracts.** The persistent runner (a host that outlives one
  launch) and native x86-64 qualification (tracked in Linkuistics/Ravel#1)
  appear only as limits where a reader needs them, never as features or plans.

### Term risks

The term list is `CONTEXT.md`. These words carry an everyday or neighbouring
meaning that will intrude, and the book states the difference wherever it
matters:

- **Session**: one `start-session` run, routing a request and supervising one
  managed process. It is not a POSIX session (`setsid`) or a login session. An
  unconfined managed process stays in the caller's POSIX session, in a process
  group of its own; a confined launch starts a new POSIX session.
- **Context**: three different things. The *session-result context* is the
  inherited `RAVEL_EXIT_FILE` path. The *policy context* is the version-1 JSON
  data delivered to `select`. The caller's working directory is the request's
  `cwd`. The book never says "context" unqualified.
- **Kind**: the caller's open token, interpreted only by owner policy. It is
  not Grove's task kind, even when Grove passes its task kind as the session
  kind.
- **Channel**: the result channel is Ravel's private directory and completion
  file, through which `end-session` publishes. It is built on the runner's
  completion `Channel`, which only signals that a file appeared.
- **Worker**: the policy worker is a separate process. It is not a Bun
  `Worker` thread, which owner code may itself start.
- **Result**: the opaque bytes published by `end-session`. It is never the
  managed process's exit status or the agent's work.
- **Policy**: the owner's routing code. It is not a security or sandbox
  policy; that is confinement.
- **Harness**: the agent program a policy selects. It is not a test harness.
- **Refusal**: a decision not to launch, made by the owner policy or by Ravel
  checking that policy's file, behaviour or data. It is not a worker or supervision
  failure, nor a selection timeout.

### Coverage inventories

The book claims complete coverage of five inventories. Each is enumerated
from the subject at drafting time and again at proof, not from this file or
from memory: a session derives the list from its source, checks the book
against it, and reports what is missing. An item is covered when the book
teaches it where a task needs it and, for the first two inventories, also
gives it a chapter 6 entry.

1. **CLI inputs and outputs**, from `ravel --help`, `ravel start-session
   --help`, `ravel end-session --help` and the CLI's argument definitions in
   the source: every verb, option and argument; every exit status, including
   the signal statuses of INT, TERM and HUP; the stdout and stderr contract and
   the `--json` notices; and every environment variable Ravel reads, sets,
   passes on or refuses to grant.
2. **Configuration inputs**, from the session specification, the SDK's
   exported declarations and the settings reader: the policy entry and how it
   is found and checked; every settings key and the refusal of unknown keys;
   every bound; the policy object's fields; the request, selection and refusal
   fields; the policy context document's fields and the host operations; the
   SDK's exports and the embedded imports; the module-resolution rules; and
   the shipped sample.
3. **Lifecycle and failure outcomes**, from the session specification and the
   result/job completion specification. At this agreement they are: a published
   result after confirmed cleanup, exact for empty, multiline, Unicode and
   option-looking values, and successful even if the managed process later
   exits nonzero; a repeated `end-session` (the first value wins); a natural
   exit without a result, including status 0; `end-session` with missing,
   empty or stale context; malformed CLI input or settings; a refusal by the
   policy or of the policy's file, behaviour or data; a missing, stale or
   mismatched worker and other worker or supervision failures; a selection
   timeout; a selected program that cannot be executed or found; cancellation
   by INT, TERM or HUP during selection and during supervision, including after
   a result was published; the escalation schedule after publication and the
   cleanup of descendants before reap; a surviving group, a cleanup failure or
   an unwritable result stream; operation reports; nested sessions;
   confinement granted or refused; terminal lending and recovery, noninteractive
   isolation, and recovery after a failed exec; and the accepted limits named
   under Emphasis.
4. **Component obligations**: the ten obligations the session specification
   authorises and the component guide tabulates today. Existing owner
   configuration keeps working; policy execution has no session-result
   authority and its worker is reaped before the managed launch; policy output
   is judged without fallback; the selected program runs directly in the
   caller's context; the first complete publication wins and stays exact;
   success requires reap and confirmed group cleanup; cancellation suppresses
   results; the terminal's foreground and modes are recovered; only result
   bytes reach stdout; installed execution uses the matched Ravel CLI and worker.
5. **Contributor workflows**, from `task --list`: setup, build, the test and
   check tasks, install and smoke, the release tasks, and the book's own tasks.
   The native-entry, incumbent and Linux measurement tasks are evidence
   instruments: the book names them and points to the evidence they produce,
   without teaching them.

**Omissions and unknowns**, stated wherever they matter and in every report of
coverage:

- Whether a developer new to Ravel can use the book is unknown, because the
  reader checkpoints are the owner's stand-in reads (part 4).
- Chapter 10 is unwritten until a supported extracted component exists.
- Native x86-64 Linux is unqualified (built, linted at Rust 1.85.0 and run only
  under emulation).
- The real agents' flags are known only as of their recorded check. Claude
  Code 2.1.301 and codex-cli 0.162.1 were installed on 2026-10-16; drafting
  rechecks and records the versions it uses.
- GitHub's rendering is unchecked until the branch is pushed, and the
  published site until the first release that carries the book.
- Coverage of an inventory shows what the book covers, not that a reader
  understands it.

### Examples policy

- **Every complete example runs**, through the seam Ravel's CLI tests
  already use: the built `ravel`, real TypeScript policy evaluation, fake
  agent programs on a private PATH, a cleared environment and a temporary
  HOME and TMPDIR. A Taskfile task runs every complete example and compares
  its output exactly.
- **No example launches a real agent or model**, and none uses a personal
  policy or account. A route to a real agent program is validated through a
  fake executable of the same name, and records the agent's version and the
  date its flags were checked against that agent's own help.
- **Fragments are labelled** as fragments, with what they omit.
- **The convention and the check** (how an example's files, command and
  expected output are marked in Markdown, the seam's environment, the one
  normalisation, skips and signals) are in `examples.md` beside this contract.
- **Source excerpts are exact.** Each walkthrough excerpt names the file it
  comes from, and a Taskfile task checks that the excerpt still appears there
  byte for byte.

### Figure needs

Each figure answers a reader question at a stated level. The art stage may
refine, merge or add to this list; it keeps one visual vocabulary across the
book and adds a diagram kind only when the question changes.

The book also has one visual style across diagram types: a shared colour
palette, fonts and type sizes, line weights, arrow styling and spacing
conventions. Shared style definitions carry these choices; a notation-specific
difference has an explicit reason. Art checks these properties across the
whole book after the chapter units exist, before the final whole-book proof.
This checks visual consistency, not the reader's knowledge of notation.

| Reader question | Level and notation | Where |
|---|---|---|
| Where does Ravel sit between a caller and the program it launches? | C4 system context | chapters 1–2 |
| Which processes exist during a session, and which holds which authority? | C4 container | chapters 2 and 7 |
| What happens, in order, in a successful session? | UML sequence | chapter 3, then in full in chapter 8 |
| What changes when cancellation arrives during selection versus during supervision? | UML sequence, one per phase | chapters 5 and 9 |
| Which states does a session pass through to each outcome? | UML state machine | chapters 3 and 5 |
| What is the timing after `end-session` publishes (2-second grace, TERM, up to 5 seconds, KILL)? | UML timing | chapter 9 |
| How is the terminal lent to the managed process and recovered? | UML sequence | chapter 9 |
| How do the packages, the worker and the SDK fit together in the source and in an installation? | C4 component and UML deployment | chapters 6, 7 and 12 |
| How is the outcome chosen when failures and signals combine? | UML activity | chapter 8 |
| How is a release built and checked? | UML activity | chapter 12 |

### Relation to other documents

- **The book replaces these guides**, each removed once the chapters that
  absorb it land, with every inbound link redirected: the README (reduced to a
  short entry into the book), the session CLI guide and the owner
  configuration guide (both removed: Parts I and II absorbed them, and the
  owner's dated configuration handoff is kept outside the book in
  `docs/configuration-handoff-2026-10-12.md`), the local runner guide
  (`docs/local-runner.md`), the result-channel guide
  (`docs/session-channel.md`), the component guide (`docs/components.md`,
  whose editable views under `docs/design/components/` become figure sources),
  the release guide (`docs/RELEASING.md`) and the authority note
  (`docs/authority.md`; Parts I and II absorbed its authority statements, so
  it is reduced to the test-isolation guards and legacy-name exceptions, and
  removed when chapter 11 absorbs those).
- **These stay normative**: the session execution specification, the
  result/job completion specification and the decision records. The book
  explains every behaviour and design reason in its own words, so a reader
  never has to follow a link to them. Links to them are optional depth, kept
  out of the way of the teaching, and proof checks each behaviour the book
  restates against its rule.
- **These stay outside the book**: the evidence documents and their receipts,
  check logs and campaign files, the caller-reference inventory and the
  retired-project record.

## 2. Prose contract

- **Base**: Grove's house style, from the documentation-guidance design named
  above. It follows [Google's developer documentation style
  guide](https://developers.google.com/style) on the points it names, and on
  no others. Google's guide is licensed CC BY 4.0; the points below are stated
  in this file's own words.
- **Spelling**: British (behaviour, authorise, colour). Code identifiers keep
  their own spelling, in code font.
- **Normative keywords**: none. The book does not use MUST, SHOULD, MAY or
  their kin. The specifications hold the rules; the book states behaviour in
  plain indicative sentences.
- **Plain vocabulary**: call the Rust command-line program the “Ravel CLI”,
  not “front” or “native front”. Say what it does: it checks
  inputs, accepts a value, allows a launch, starts a program or chooses the
  final outcome. Avoid specialist substitutes such as “admits”, “admission”
  and unexplained “settlement”. Keep code identifiers exact. Explain a
  technical term when the reader needs the concept; do not use a synonym
  that hides a different technical meaning. Apply this language to figure
  labels, captions, keys and alt text as well as prose.
- **Simple, self-contained language**: every page can be understood without
  following a link. A link is optional depth, and it names its destination and
  purpose.
- **Term list**: `CONTEXT.md` plus the term risks above. A Ravel term not yet
  in `CONTEXT.md` is added there when the book first needs it.

The house style's points, as this book takes them:

- **Task headings** start with a bare infinitive: "Write a policy", not
  "Writing a policy" or "To write a policy".
- **Introducing steps**: a complete sentence introduces a procedure, or nothing
  does when its heading already says what it does. A fragment that the steps
  complete, such as "To install:", does not.
- **Register**: plain written prose in a conversational rather than formal
  tone, with no slang, idiom or joke, and no transcribed speech.
- **Length**: no length target. Explanation is as long as this reader needs.
- **Steps**: the location, condition or goal the reader needs first comes
  before the action, and the result after it. A step's first sentence is an
  imperative; running text is indicative.
- **Demonstratives**: "this" or "these" takes a noun where its referent could
  be unclear.
- **Voice**: active where the actor matters; passive or stative where the
  reader does not need the actor, or where a statement concerns an artifact.
- **Front-loading**: headings, list items and paragraph openings start with
  their information-carrying words.
- **Terms**: a term from the term list is linked at its first use on each page
  to the place the book teaches it.
- **Code**: code font for identifiers, commands, file names, paths and literal
  values. Code blocks are fenced and tagged with their language.
- **Example labels**: each example is introduced by a label beginning
  "Complete example" or "Fragment"; a fragment's label says what it omits.
- **Stock phrasing**: no filler opener or closer ("It's worth noting", "In
  summary"), no signpost that repeats its heading, no closing paragraph that
  restates the section, no hedge without a condition ("typically" without
  saying when), no inflated word where a plain one serves ("leverage",
  "robust", "seamless"), and no reflexive list of three.
- **Simple layout**: the headings the section plan states stand. Beyond them, a
  heading level is added only for a section with at least two subsections. A
  paragraph carries one idea. Parallel items form a list, and items compared on
  the same attributes form a table. Bold marks only a run-in label or a term at
  its definition. Nothing is added for decoration.
- **Statements and asides**: in chapter 6, a note, example or summary beside an
  entry's statement is set apart as a Markdown blockquote that opens with its
  label (`> **Note:** …`), with every line marked and a blank line closing it.

## 3. Format and media

- **Format**: Markdown chapters under `docs/book/src/`, ordered by mdBook's
  contents file `docs/book/src/SUMMARY.md`, with `docs/book/book.toml`. This
  contract sits beside them and is not part of the published book. Links
  between chapters are relative links to `.md` files, so they work both in
  mdBook and on GitHub.
- **Build**: mdBook 0.5.4, pinned and fetched by `task setup` alongside the
  pinned Bun. A `docs:book` task builds and serves the book locally.
- **Publication**: the built book is published to GitHub Pages with each
  release, so the site always matches a released Ravel.
- **Viewing environments**: the published site; the book built locally by the
  `docs:book` task; GitHub's view of the Markdown sources; and a local checkout
  in an editor preview or as raw files. Proof inspects the book in each one it
  can reach and says which it could not.
- **Figure notations**: C4 (system context, container and component levels,
  and dynamic views); UML diagram kinds (sequence, state machine, activity,
  component, deployment, timing); SysML diagram kinds where one answers a
  question better than its UML counterpart; and others where a reader question
  calls for them. Readers are assumed to know these notations or look them up.
  Every book-specific convention a figure uses is identifiable on or beside it;
  the book does not teach the notation itself.
- **Figure renderers**: PlantUML 1.2026.8, using its bundled C4-PlantUML 2.13.0
  library and its built-in Smetana layout so that no separate Graphviz is
  needed, for the C4, UML and SysML-style diagrams; and d2 0.9.0 for structure
  and flow diagrams those notations do not fit. A change of renderer or version
  is a change to this contract.
- **Figure delivery**: every figure is committed as its editable text source
  beside the SVG it generates, with the renderer version, the layout settings
  and the regeneration command recorded. A Taskfile task regenerates every
  figure and compares it with the committed SVG. No figure relies on GitHub's
  own Mermaid rendering, whose version GitHub does not state. A figure made
  only of the page's own text, such as a table, needs no source file. The
  layout on disk and the regeneration procedure are in `figures.md` beside
  this contract.
- **Dated facts**: the versions above were current on 2026-10-16. A session
  whose choice turns on one of them rechecks it first and records the date.

## 4. Reader checkpoints

- **The reader** is the owner, reading as a labelled stand-in for the intended
  reader. Every result is recorded as the owner's stand-in read. It can
  establish accuracy, coverage and the owner's own results, not whether a
  developer new to Ravel can find, understand and use the book.
- **Read 1** follows proof of Parts I and II. Tasks: write a policy that routes
  to a fake agent, run a session, and interpret a refusal and a cancellation;
  and arrive at one direct entry, such as an exit code or an error message,
  without reading what precedes it. Its corrections land before Parts III and
  IV are drafted.
- **Read 2** follows proof of Parts III and IV. Tasks: explain why Ravel has a
  TypeScript worker and two Rust packages; follow one session through the code
  to where its final cancellation outcome is chosen; and build and test Ravel from a checkout.
- **Chapter 10's read** follows that chapter's proof. Task: use the extracted
  component through its interface.
- **Every read** begins at chapter 1 or at its stated entry, in the locally
  built book, with the figures confirmed to load there first. It records the
  reader's own words, how far they got, the locations they report and what they
  did not reach. An early comprehension failure stops the read, and its
  correction goes to the stage that owns it before dependent drafting
  continues. The owner's results, task performance, mechanical checks and any
  model-proxy results are reported separately, and none stands in for another.
- **Placement**: each read is a `requirements` session placed after the proof
  it follows, outside draft → copy-edit → art → proof.
