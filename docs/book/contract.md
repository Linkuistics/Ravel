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

Ravel runs LLM sessions in agent harnesses, such as Claude Code and Codex, as
supervised parts of another program. In the owner's words, from read 1: "We
should be explicit that the programs we are running are LLM sessions in
harnesses such as codex or claude code. We should also point out that these
harnesses typically cannot exit themselves, which is why we have the
`end-session` command. Thus ravel allows you to use such interactive harness
as 'sub components' of another process, typically an orchestrator."

The reason for a harness, in the owner's words from read 1 on 2026-10-18:
"We should make it obvious that the reason we want to use an LLM harness is
because it allows us to use a subscription rather than an API session which
is much cheaper." A harness such as Claude Code or Codex can run under the
subscription of its user. Thus, a caller that runs LLM work through Ravel can
use that subscription instead of paid API access. The book says so where it
first describes Ravel. It names no price or plan. Each harness's own
documentation covers its plans and terms.

A **caller**, such as an orchestrator that runs sessions as steps of its own
work, asks for a session of some kind; the **owner's** executable TypeScript
policy chooses the harness command to run; Ravel launches that command as a
supervised job. An interactive harness does not exit when its LLM has finished
the task: it waits for more input. So the caller's prompt tells the LLM to run
`ravel end-session RESULT`. This command sends one opaque result to Ravel and
tells Ravel to stop the session. Ravel then stops the harness and the rest of its process
group if they are still running and, once the job has been completely cleaned
up, returns the result or reports why it cannot. That is what lets an
interactive harness work as one component of a larger program. The book says
so where it first describes Ravel, and gives the reason for `end-session`
where it introduces the command.

The mechanism is generic, and its contract is loose. In the owner's words
from read 1 on 2026-10-18: "We should emphasize that the fact that the
configuration runs a harness is not fully required. The contract interface
between the Revel CLI and the configuration is that it runs something and
returns a value. Now it is the case that the Revel CLI specifically talks
about prompts and we expect that it will be a harness that it runs, but this
is a very loose contract."

The policy selects a program, and Ravel runs it. A program in the session
then sends one value with `end-session`. Ravel needs nothing more from the
selected program. `start-session` takes a prompt, and the book expects a
harness to receive it. But the policy decides which program runs and whether
that program receives the prompt. Thus, any program that runs `end-session`
can stand where a harness stands.

For this reason, every runnable example uses a fake agent. Ravel assigns no
meaning to kinds or results, integrates no model provider, and stores
nothing. Its first caller is the Grove task-tree driver, which the book shows
only as one labelled illustrative caller.

The book keeps these roles apart:

- the **caller**, which invokes Ravel, supplies the prompt and its ending
  instructions, and interprets the result;
- the **owner**, who writes the policy and settings for their account;
- **Ravel** itself: the Ravel CLI, the policy worker, the runner and the
  result channel;
- the **managed process**, the selected program and its descendants: a
  harness running an LLM session, or a program standing in for one;
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
| 1 | What Ravel is for | say what Ravel solves, why it runs harnesses (a subscription instead of paid API access), why `end-session` exists, what it leaves to callers (no store, no task meaning, no model integration), and whether it fits their need |
| 2 | The actors and their words | name the caller, owner, owner policy, policy worker, managed process and opaque result, and say who holds which authority |
| 3 | A complete session, end to end | install Ravel with Homebrew and check its version, run a real session with a tiny policy and a fake agent, then describe its stages; the process and terminal ideas are introduced here as the session needs them |
| **Part II** | **Use Ravel** | |
| 4 | Write an owner policy | write, typecheck and validate a policy with fake agents, covering the SDK, refusals, policy context, settings, environment grants, routes to real agent programs and copying an existing policy |
| 5 | Run sessions | run sessions under real conditions (prompts, kinds, caller data, nesting, confinement, notices) and interpret every outcome and exit code |
| 6 | Reference | look up any CLI input, setting, bound, request, selection or context shape, environment variable or installed path |
| **Part III** | **Inside Ravel** | |
| 7 | The parts of Ravel | explain the Ravel CLI, why policy runs as a separate TypeScript worker that Ravel reaps before the managed launch, why the runner is its own Rust package, and how the parts compose |
| 8 | Follow a session through the code | follow one successful session from `main` to the result write, then the refusal and cancellation forks and the error and signal precedence, and find where a change belongs |
| 9 | Jobs, signals and the terminal | reason about process groups, escalation, reaping, terminal lending and recovery, confinement, entry signal state, and the failure guarantees with their limits |
| **Part IV** | **Reuse and develop Ravel** | |
| 10 | Use Ravel's components in your project | use an extracted component through its interface and meet its obligations, without Ravel's policy or session conventions |
| 11 | Develop Ravel | set up a checkout, build, use the test seams and `task check`, add a test, and know what the evidence documents are for |
| 12 | Deliver Ravel | handle versions and identity, install and smoke tests, archives and platform floors, the formula and release tasks, and the supported targets with their limits |

The owner's STE decision of 2026-10-17 changed six chapter titles and two part
titles to commands, because STE Rule 3.5 permits “-ing” forms only in
technical names. The chapter file names stay as they are, so links do not
change. The planning for Parts III and IV changed chapter 7's title to
“The parts of Ravel” on 2026-10-18. This noun phrase avoids passive voice
with a known agent (Rule 3.6). Its file is `07-the-parts-of-ravel.md`.

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

- **Why a harness, and how loose the contract is.** A harness lets a caller
  use a subscription instead of paid API access. The policy can select any
  program, and Ravel needs only one value sent with `end-session`. A harness
  that receives the prompt is the expected use, not a requirement (the
  owner's findings R14 and R15 of 2026-10-18).
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
  owner's routing table, `docs/configuration-handoff-2026-10-12.md`, is not
  teaching material. It stays outside the book; it was kept, not deleted, when
  the owner configuration guide was folded in and removed.
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
  file, through which `end-session` sends the result. It is built on the runner's
  completion `Channel`, which only signals that a file appeared.
- **Worker**: the policy worker is a separate process. It is not a Bun
  `Worker` thread, which owner code may itself start.
- **Result**: the opaque bytes that `end-session` sends. It is never the
  managed process's exit status or the agent's work.
- **Policy**: the owner's routing code. It is not a security or sandbox
  policy; that is confinement.
- **Harness**: the agent program, such as Claude Code or Codex, in which an
  LLM session works on the prompt; owner policy selects it. It is not a test
  harness.
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
   result/job completion specification. At this agreement they are: a result
   sent with `end-session` and returned after confirmed cleanup, exact for empty, multiline, Unicode and
   option-looking values, and successful even if the managed process later
   exits nonzero; a repeated `end-session` (the first value wins); a natural
   exit without a result, including status 0; `end-session` with missing,
   empty or stale context; malformed CLI input or settings; a refusal by the
   policy or of the policy's file, behaviour or data; a missing, stale or
   mismatched worker and other worker or supervision failures; a selection
   timeout; a selected program that cannot be executed or found; cancellation
   by INT, TERM or HUP during selection and during supervision, including after
   a result was sent; the escalation schedule after Ravel receives the result and the
   cleanup of descendants before reap; a surviving group, a cleanup failure or
   an unwritable result stream; operation reports; nested sessions;
   confinement granted or refused; terminal lending and recovery, noninteractive
   isolation, and recovery after a failed exec; and the accepted limits named
   under Emphasis.
4. **Component obligations**: the ten obligations the session specification
   authorises and the component guide tabulated before its removal. Existing owner
   configuration keeps working; policy execution has no session-result
   authority and its worker is reaped before the managed launch; policy output
   is judged without fallback; the selected program runs directly in the
   caller's context; Ravel keeps the first complete result that it receives, exactly;
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
- **The reader's Ravel**: chapter 3 tells the reader to install Ravel with
  Homebrew (`linkuistics/taps/ravel`) and to check that `ravel --version`
  prints the version that the book names. The book names that version in one
  place. The release writes its own version there and checks it. Until that
  release, the book shows a placeholder. The owner chose this on 2026-10-18
  (“Set at release time”), in place of a build from the checkout. The
  examples check still runs the build of the checkout through the seam.
- **Fragments are labelled** as fragments, with what they omit.
- **The convention and the check** (how an example's files, command and
  expected output are marked in Markdown, the seam's environment, the one
  normalisation, skips and signals) are in `examples.md` beside this contract.
- **Source excerpts are exact.** Each walkthrough excerpt names the file it
  comes from, and a Taskfile task checks that the excerpt still appears there
  byte for byte. The convention and the check (`task docs:excerpts`) are in
  `excerpts.md` beside this contract.

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
| Where does Ravel sit between a caller, such as an orchestrator, and the harness or other program it launches? | C4 system context | chapters 1–2 |
| Which processes exist during a session, and which holds which authority? | C4 container | chapters 2 and 7 |
| What happens, in order, in a successful session? | UML sequence | chapter 3, then in full in chapter 8 |
| What changes when cancellation arrives during selection versus during supervision? | UML sequence, one per phase | chapters 5 and 9 |
| Which states does a session pass through to each outcome? | UML state machine | chapters 3 and 5 |
| What is the timing after `end-session` sends the result (2-second grace, TERM, up to 5 seconds, KILL)? | UML timing | chapter 9 |
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
  `docs/configuration-handoff-2026-10-12.md`), the local runner guide,
  the result-channel guide and the component guide (all three removed:
  chapters 7, 8 and 9 absorbed them, and their editable views under
  `docs/design/components/` were removed with them; the source and observation
  evidence of the guides is kept outside the book in
  `docs/component-evidence.md`), the release guide (`docs/RELEASING.md`) and the authority note
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

- **Base**: ASD-STE100 Simplified Technical English, Issue 9 (2025-01-15).
  STE applies to all text in the book: prose, headings, tables, lists, figure
  labels, captions, keys and alt text. The owner made this decision on
  2026-10-17: “I intend to add a global instruction to use ADE-STE100 for all
  LLM communication (such as when you talk to me here) as well as in all
  documentation. Apply that rule now.” On 2026-10-18 the owner limited the
  base to the writing rules: “The skill I am going to adopt does not use the
  word list, only the rules.” Thus, the book uses STE's writing rules (Part 1
  of the specification) and does not use its dictionary (Part 2). The points
  below add to STE. If a point and STE do not
  agree, use STE. Before this decision, the base was Grove's house style,
  which follows [Google's developer documentation style
  guide](https://developers.google.com/style) on the points it names. The
  points that stay are below, in this file's own words. Google's guide is
  licensed CC BY 4.0.
- **The STE specification**: the owner's copy is
  `~/Downloads/ASD-STE100_ISSUE9.pdf` (SHA-256
  `d1f4ea9e7cd6e46b47aa9057209f99e78c0e9cfc4e27a5b07895b05c1a166431`). ASD
  gives it free, on request, through <https://www.asd-ste100.org>. Its
  copyright notice prohibits unauthorised distribution. Thus, do not commit
  the PDF or text from it. A note can give a rule number and its point in
  its own words, for example “Rule 3.5: ‘-ing’ forms only in technical
  names”. If the file is not at that path, ask the owner for it.
- **Word decisions**: `docs/book/ste.md` holds the word decisions that all
  chapters share. They give the words for the result and the chapter and part
  titles. They also give the words that the book does not use and the words
  in “-ing” that it keeps.
  `task docs:ste` examines the book's text against these decisions and the
  STE rules that a script can check. Every stage uses these decisions. A
  stage that needs a new decision adds it to that file.
- **Spelling**: British (behaviour, authorise, colour). This contract is the
  official directive that STE Rule 1.14 permits. Code identifiers keep their
  own spelling, in code font.
- **Technical names**: the terms in `CONTEXT.md`, code identifiers, commands,
  file names, paths and product names are technical names. A word in “-ing”
  can be part of a technical name, for example “working directory” (Rule
  3.5).
- **Normative keywords**: none. The book does not use MUST, SHOULD, MAY or
  their kin. The specifications hold the rules. The book gives behaviour in
  plain indicative sentences.
- **Plain vocabulary**: call the Rust command-line program the “Ravel CLI”,
  not “front” or “native front”. `end-session` sends the result to Ravel, and
  Ravel keeps the first result that it receives. Do not use “publish”,
  “publication” or “submit” for this (the owner's decision of 2026-10-17). Do
  not use specialist words such as “admits”, “admission” and unexplained
  “settlement”. For the actions of the Ravel CLI, use plain verbs such as
  “checks”, “accepts” and “allows”. Keep code identifiers exact. Explain a technical term when the reader needs the
  concept. Do not use a synonym that hides a different technical meaning.
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
- **Register**: plain written prose in STE, with no slang, idiom or joke, and
  no transcribed speech.
- **Length**: no length target for a section. STE sets the limits for
  sentences and paragraphs. An instruction has 20 words or fewer (Rule 5.1). A
  descriptive sentence has 25 words or fewer (Rule 6.3). A paragraph has one
  topic and six sentences or fewer (Rules 6.5 and 6.6). Use a vertical list
  for complex text (Rule 4.3).
- **Punctuation**: do not use semicolons (Rule 8.1) or contractions (Rule
  4.2).
- **Steps**: the location, condition or goal the reader needs first comes
  before the action, and the result after it. A step's first sentence is an
  imperative; running text is indicative.
- **Demonstratives**: "this" or "these" takes a noun where its referent could
  be unclear.
- **Voice and verbs**: use the active voice. In descriptive text, use the
  passive voice only when the agent is unknown (Rule 3.6). Use only the simple
  tenses (Rule 3.2). Use “-ing” forms only in technical names (Rule 3.5). Do
  not use phrasal verbs (Rule 9.3).
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
- **Breaks in prose**: the owner made this decision on 2026-10-18: “I also
  want to make sure that we don't have too long expanses of prose without a
  diagram or a table to break it up.” A run of prose has about 450 words or
  fewer, one screen at the 750-pixel column. Then a figure, a table or a code
  block breaks it. Headings and lists do not break a run. A break carries
  information that the reader needs, not decoration. `task docs:breaks`
  reports each longer run, with its file, its lines, its word count and its
  headings. The check uses one number: a run is longer when it has more than
  450 words. The words are those of paragraphs, list items and blockquotes.
  The text of a heading, a list marker, the `>` of a blockquote, a link target
  and an HTML comment add no words. A fenced code block, a table row (a line
  that starts with `|`) and an image (a line that starts with `![`) break a
  run. `task docs:breaks -- --strict FILE…` stops with status 1 when it finds
  a longer run. `task docs:breaks:test` shows that the check finds a longer
  run and that each rule changes the result.
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
  the book does not teach the notation itself. The book's own Typst modules draw
  each notation's elements (boxes, cylinders, lifelines, orthogonal relations),
  so a figure follows a notation where its question needs it, and
  `figures.md` records where a drawing departs from the notation's usual form.
- **Figure renderer**: Typst 0.15.1 with the CeTZ 0.4.2 package (and its one
  dependency, oxifmt 1.0.0), for every figure. Text is set in the pinned DejaVu
  Sans 2.37. `task setup` fetches all of them into `target/tools` with SHA-256
  checks, and a render runs offline from those pins alone. One renderer draws
  every notation, replacing the book's earlier notation-specific tools on the
  owner's decision of 2026-10-17 ("migrate full"; the history is in
  `figures.md`). A change of renderer or version is a change to this contract.
- **Figure sources**: one authoritative `.typ` source per figure and no second
  editable source in any notation. A figure source is data for a layout
  module (`_sequence.typ`, `_c4.typ`, `_deployment.typ`, `_state.typ`), which
  measures the figure's text and returns a layout record: positions, roles and
  measured text, with no colour, weight, font size or roughness. `_draw.typ` is
  the only module that draws, and draws any record under a named style from
  `_style.typ`. A file whose name starts with an underscore is such a shared
  module, not a figure, and has no SVG.
- **Figure style**: the figures use the crisp style, with exact geometry
  (roughness 0), in DejaVu Sans. Page tables are the page's own. The owner
  chose this on 2026-10-18, after a view of all nine figures in both styles:
  “I've changed my mind about the hand-drawn look. I don't think it looks
  friendly, I think it just looks wobbly, so I'd like to change that.” It
  replaces the sketched style (roughness 2) of 2026-10-17. Thus, the reserve
  sketch backend (Rough.js on Bun) has no use.
- **Figure delivery**: every figure is committed as its `.typ` source beside the
  SVG it generates and its `<figure>.layout.json`, the layout record the source
  evaluates to. The renderer versions, the layout and style settings and the
  regeneration command are recorded in `figures.md` beside this contract. A
  Taskfile task regenerates every figure and compares both the SVG and the
  layout record with what is committed. The SVG holds the text as paths, by
  design: its text is not selectable, so the alternative to a figure is the
  alt text, caption and prose on the page, and the SVG is a full-size link,
  never the only place a fact is stated. No figure relies on GitHub's own
  Mermaid rendering, whose version GitHub does not state. A figure made only of
  the page's own text, such as a table, needs no source file. The layout on disk
  and the regeneration procedure are in `figures.md` beside this contract.
- **Dated facts**: mdBook 0.5.4 was current on 2026-10-16; Typst 0.15.1 was
  current on 2026-10-17, when CeTZ 0.5.2 also existed and was not adopted
  (moving up is a new trial of every drawing primitive). A session whose choice
  turns on one of them rechecks it first and records the date.

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
