<!-- tc: pending because this large contract is converted by the sections each edit touches -->

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

<!-- tc: on -->

### What the reader can do afterwards

- Configure and run a session. Write, typecheck and validate an owner policy
  with fake agents. Route kinds to real agent programs. Interpret every
  outcome and exit code.
- Explain why Ravel has a TypeScript policy worker and two Rust packages,
  and what each part owns.
- Follow successful, refused and cancelled sessions through the code.
  Find where a change belongs.
- Use the `ravel` CLI as a child process from another program. Explain why
  Ravel supports no separate component.
- Build, test and deliver the project.
- Look up any CLI input, setting, bound, data shape, environment variable or
  installed path in chapter 6.

### The intended domain

Ravel runs LLM sessions in agent harnesses, such as Claude Code and Codex, as
supervised parts of another program.

<!-- tc: off because these are the owner's quoted words -->

In the owner's words, from read 1: "We
should be explicit that the programs we are running are LLM sessions in
harnesses such as codex or claude code. We should also point out that these
harnesses typically cannot exit themselves, which is why we have the
`end-session` command. Thus ravel allows you to use such interactive harness
as 'sub components' of another process, typically an orchestrator."

<!-- tc: on -->

<!-- tc: off because these are the owner's quoted words -->

The reason for a harness, in the owner's words from read 1 on 2026-10-18:
"We should make it obvious that the reason we want to use an LLM harness is
because it allows us to use a subscription rather than an API session which
is much cheaper."

<!-- tc: on -->

A harness such as Claude Code or Codex can run under the
subscription of its user. Thus, a caller that runs LLM work through Ravel can
use that subscription instead of paid API access. The book says so where it
first describes Ravel. It names no price or plan. Each harness's own
documentation covers its plans and terms.

A **caller**, such as an orchestrator, asks for a session of some kind.
The **owner's** executable TypeScript policy chooses the harness command.
Ravel launches that command as a supervised job. An interactive harness
waits for more input when its LLM finishes the task.

The caller's prompt tells the LLM to run `ravel end-session RESULT`.
This command sends one opaque result to Ravel and tells Ravel to stop the
session. Ravel stops the harness and the rest of its process group if they
still run. After complete job cleanup, Ravel returns the result or reports
why it cannot.

This lets an interactive harness work as one component of a larger program.
The book states this purpose where it first describes Ravel. It gives the
reason for `end-session` where it introduces the command.

The mechanism is generic, and its contract is loose.

<!-- tc: off because these are the owner's quoted words -->

In the owner's words
from read 1 on 2026-10-18: "We should emphasize that the fact that the
configuration runs a harness is not fully required. The contract interface
between the Revel CLI and the configuration is that it runs something and
returns a value. Now it is the case that the Revel CLI specifically talks
about prompts and we expect that it will be a harness that it runs, but this
is a very loose contract."

<!-- tc: on -->

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
  instructions, and interprets the result.
- the **owner**, who writes the policy and settings for their account.
- **Ravel** itself: the Ravel CLI, the policy worker, the runner and the
  result channel.
- the **managed process**, the selected program and its descendants: a
  harness running an LLM session, or a program standing in for one.
- **contributors**, who develop and deliver Ravel.

### The section plan

The book is pedagogical: every chapter teaches toward a reader task.
Chapter 6 is the one lookup chapter, and its entries are self-contained like
every other page. Each chapter's purpose is what the reader can do after it.

| # | Chapter | After it, the reader can… |
|---|---|---|
| **Part I** | **Ravel and its session** | |
| 1 | What Ravel is for | Say what Ravel solves and whether it fits their need. Explain harness subscriptions, `end-session`, and caller responsibilities. Ravel supplies no store, task meaning or model integration. |
| 2 | The actors and their words | name the caller, owner, owner policy, policy worker, managed process and opaque result, and say who holds which authority |
| 3 | A complete session, end to end | <!-- tc: depart synonym-rotation because version checks and policy validation concern different inputs --> Install Ravel with Homebrew and check its version. Run a session with a tiny policy and a fake agent. Describe its stages, with the process and terminal ideas each stage needs. |
| **Part II** | **Use Ravel** | |
| 4 | Write an owner policy | Write, typecheck and validate a policy with fake agents. Use the SDK, refusals, policy context, settings and environment grants. Route to real agents and copy an existing policy. |
| 5 | Run sessions | run sessions under real conditions (prompts, kinds, caller data, nesting, confinement, notices) and interpret every outcome and exit code |
| 6 | Reference | look up any CLI input, setting, bound, request, selection or context shape, environment variable or installed path |
| **Part III** | **Inside Ravel** | |
| 7 | The parts of Ravel | Explain the Ravel CLI and its separate TypeScript worker. Explain why Ravel reaps the worker before the managed launch. Explain the separate runner package and how the parts compose. |
| 8 | Follow a session through the code | Follow a successful session from `main` to the result write, then the refusal and cancellation paths. Explain error and signal precedence. Find where a change belongs. |
| 9 | Jobs, signals and the terminal | reason about process groups, escalation, reaping, terminal lending and recovery, confinement, entry signal state, and the failure guarantees with their limits |
| **Part IV** | **Develop and deliver Ravel** | |
| 10 | Develop Ravel | Prepare a checkout and build Ravel. Use the test seams and `task check`. Add a test and explain the evidence documents. |
| 11 | Deliver Ravel | handle versions and identity, install and smoke tests, archives and platform floors, the formula and release tasks, and the supported targets with their limits |

The owner's STE decision of 2026-10-17 changed six chapter titles and two part
titles to commands. STE Rule 3.5 permits “-ing” forms only in technical names. That title change kept the chapter file names unchanged. The planning for Parts III and IV changed chapter 7's title to
“The parts of Ravel” on 2026-10-18. This noun phrase avoids passive voice
with a known agent (Rule 3.6). Its file is `07-the-parts-of-ravel.md`.

The owner removed the planned component chapter on 2026-10-21.
“Develop Ravel” and “Deliver Ravel” now form Part IV, as chapters 10 and 11.
Their files are `10-develop-ravel.md` and `11-deliver-ravel.md`.

Ravel supports no separate component. Its runner, `keyed-launch`, stays a
private package for a launcher that exits after one session. The runner can
change process-wide signal state, and its return does not restore that state.
A program that needs supervised harness sessions invokes the `ravel` CLI as a
child process. The book teaches that caller interface.

Chapters 1–11 describe the current code. Chapters 1–6 describe the CLI,
configuration and session behaviour that Ravel's compatibility promise keeps
unchanged. Chapters 7–11 describe structure, packages and delivery.
A later change updates each chapter it affects.

Extraction can be reconsidered
when a project needs the runner without Ravel's CLI. Such work would add its
own chapter and renumber the book. This is a condition, not planned work.

Chapter 8 is an orientation that follows one session through the code. It
quotes source excerpts but does not cover the complete source.

### Entry routes

- **The learning route** reads chapters 1 to 11 in order.
- **Running sessions from your own tool:** chapters 1–3, then 5, with 6 for
  lookup.
- **Writing a policy:** chapters 1–3, then 4, with 6 for lookup.
- **Reuse from another program:** chapter 1 explains why no separate
  component is supported. Chapters 2, 3 and 5 teach the CLI caller interface.
  Chapters 7 and 9 explain the private runner and its limits.
- **Changing Ravel:** chapters 1–3, 7 and 8, then 10, with 9 and 11 as needed.
- **Direct entries:** readers may arrive at any chapter 6 entry, exit code,
  error message or figure without the preceding chapters. Each carries the local context it needs and
  links back to where its subject is taught.

<!-- tc: pending because this large contract is converted by the sections each edit touches -->

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

<!-- tc: on -->

### Coverage inventories

The book claims complete coverage of five inventories. Draft and proof each
derive these inventories from the subject. Each stage checks the book against
its list and reports omissions. Memory and this contract do not replace the
source inventory. The book teaches each item where a task needs it. The first
two inventories also have entries in chapter 6.

**CLI inputs and outputs.** Sources are the CLI argument definitions and the
help for `ravel`, `ravel start-session` and `ravel end-session`. Coverage includes:

- Every verb, option and argument.
- Every exit status, including the signal statuses of INT, TERM and HUP.
- The stdout and stderr contract and the `--json` notices.
- Every environment variable that Ravel reads, sets, passes on or refuses to grant.

**Configuration inputs.** Sources are the session specification, the SDK's
exported declarations and the settings reader. Coverage includes:

- The policy entry, its discovery and its checks.
- Every settings key, the refusal of unknown keys and every bound.
- The policy object's fields and the request, selection and refusal fields.
- The policy context document's fields and the host operations.
- The SDK's exports, embedded imports, module-resolution rules and shipped sample.

**Lifecycle and failure outcomes.** Sources are the session specification
and the result/job completion specification. At this agreement, coverage includes:

<!-- tc: depart synonym-rotation because cleanup confirmation establishes an outcome rather than validating input data -->
- A result sent with `end-session` and returned unchanged after confirmed cleanup.
  Values include empty, multiline, Unicode and option-looking strings.
  Success remains possible if the managed process later exits nonzero.
- A repeated `end-session`, where the first value wins.
- A natural exit without a result, including status 0.
- `end-session` with missing, empty or stale context.
- Malformed CLI input or settings.
- A refusal by the policy or of the policy's file, behaviour or data.
- A missing, stale or mismatched worker and other worker or supervision failures.
- A selection timeout.
- A selected program that Ravel cannot execute or find.
- Cancellation by INT, TERM or HUP during selection and supervision,
  including after the session sends a result.
- Escalation after Ravel receives the result, and descendant cleanup before reap.
- A surviving group, a cleanup failure or an unwritable result stream.
- Operation reports and nested sessions.
- Confinement granted or refused.
- Terminal lending and recovery, noninteractive isolation, and recovery after a failed exec.
- The accepted limits under Emphasis.

**Component obligations.** The session specification authorises these ten
obligations. The component guide tabulated them before its removal:

- Existing owner configuration keeps working.
- Policy execution has no session-result authority. Ravel reaps its worker
  before the managed launch.
- Ravel judges policy output without fallback.
- The selected program runs directly in the caller's context.
- Ravel keeps the first complete result that it receives, unchanged.
- Success requires reap and confirmed group cleanup.
- Cancellation suppresses results.
- Ravel recovers the terminal's foreground and modes.
- Only result bytes reach stdout.
- Installed execution uses the matched Ravel CLI and worker.

**Contributor workflows.** The source is `task --list`. Coverage includes
setup, build, tests, checks, install, smoke, release tasks and the book's tasks.
The native-entry, incumbent and Linux measurement tasks are evidence instruments.
The book names them and points to their evidence without teaching their use.

**Omissions and unknowns.** Each coverage report states these limits.
The book also states them wherever they matter:

- Whether a developer new to Ravel can use the book is unknown.
  The reader checkpoints are the owner's stand-in reads (part 4).
- Ravel supports no separate component. The private runner serves the
  one-session CLI. The component inventory describes internal obligations,
  not a library interface for another project.
- Native x86-64 Linux is unqualified. The project built it, linted it at
  Rust 1.85.0 and ran it only under emulation.
- Real agents' flags are known only as of their recorded check. The installed
  versions on 2026-10-16 were Claude Code 2.1.301 and codex-cli 0.162.1.
  Drafting rechecks and records the versions it uses.
- GitHub's rendering is unchecked until the branch is pushed.
  The site is unchecked until the first release that carries the book.
- Inventory coverage shows what the book covers, not whether a reader understands it.

<!-- tc: pending because this large contract is converted by the sections each edit touches -->

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

<!-- tc: on -->

### Figure needs

Each figure answers a reader question at a stated level. The art stage may
refine, merge or add to this list. It keeps one visual vocabulary across the
book and adds a diagram kind only when the question changes.

The book also has one visual style across diagram types: a shared colour
palette, fonts and type sizes, line weights, arrow styling and spacing
conventions. Shared style definitions carry these choices. A difference for
one notation has an explicit reason. Art checks these properties across the
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
| How do the packages, the worker and the SDK fit together in the source and in an installation? | C4 component and UML deployment | chapters 6, 7 and 11 |
| How is the outcome chosen when failures and signals combine? | UML activity | chapter 8 |
| How is a release built and checked? | UML activity | chapter 11 |

### Relation to other documents

**Guide replacements.** Each guide leaves the tree when the chapters that
absorb it land. Every inbound link changes to its replacement:

| Earlier document | Replacement and retained material |
|---|---|
| README | A short entry into the book |
| Session CLI and owner configuration guides | Parts I and II absorbed both guides. The dated owner handoff stays in `docs/configuration-handoff-2026-10-12.md` |
| Local runner, result-channel and component guides | Chapters 7–9 absorbed the guides. Their editable views under `docs/design/components/` left with them. Source and observation evidence stays in `docs/component-evidence.md` |
| Release guide | Chapter 11 absorbed the guide |
| Authority note | Parts I and II and chapter 10 absorbed it. Chapter 10, “Keep tests separate from live sessions”, holds the test-isolation guards and legacy-name exceptions |

**Normative documents.** The session execution specification, result/job
completion specification and decision records keep their authority.
The book explains every behaviour and design reason in its own words.
A reader need not follow links to these documents to understand the book.
Links offer optional depth outside the teaching. Proof checks each restated
behaviour against its rule.

**Documents outside the book.** Evidence documents, receipts, check logs and
campaign files stay separate. So do the caller-reference inventory and the
retired-project record.

<!-- tc: pending because this large contract is converted by the sections each edit touches -->

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

<!-- tc: on -->

## 4. Reader checkpoints

- **The reader** is the owner, as a labelled stand-in for the intended reader.
  Each result records that limit. A read can establish accuracy, coverage and
  the owner's own results. It cannot establish whether a developer new to
  Ravel can find, understand and use the book.
- **Read 1** follows proof of Parts I and II. The tasks are to write a policy
  for a fake agent and run a session. The reader interprets a refusal and a
  cancellation. The reader also uses one direct entry, such as an exit code
  or error message, without the preceding text. Corrections land before the
  draft of Parts III and IV.
- **Read 2** follows proof of Parts III and IV. The reader explains why Ravel
  has a TypeScript worker and two Rust packages. The reader follows a session
  through the code to its final cancellation decision, then builds and tests
  Ravel from a checkout.
- **Every read** opens at chapter 1 or its stated entry in the locally built
  book. Figures must load there before the read. The record keeps the reader's
  words, progress, reported locations and places they did not reach.
  An early comprehension failure stops the read. The stage that owns the
  correction fixes it before dependent drafting continues. Reports separate
  the owner's results, task performance, mechanical checks and model-proxy
  results. No result stands in for another.
- **Placement:** each read is a `requirements` session after its proof,
  outside draft → copy-edit → art → proof.
