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

- **Assumed, and not taught:** everyday shell use, Git, JSON and Rust.
  Shell knowledge covers PATH, environment variables, quotation rules and pipes.
  OS knowledge covers processes, PIDs, exec, exit status, signals, file descriptors,
  pipes and file permissions.
  The reader knows the diagram notations or consults their documentation.
  The book explains only its own figure conventions.
- **Not assumed, and taught where the book first needs it:**
  - Ravel's own domain: callers, owners, configuration script, sessions, kinds,
    results, and who holds which authority.
  - The job-control mechanics Ravel depends on: process groups, the
    controlling terminal and its foreground process group, saved terminal
    modes, reaping and why PID reuse makes it matter, signal dispositions and
    masks across exec, and PTYs as the tests use them.
  - The Rhai syntax that configuration scripts need: constants, variables, maps, arrays,
    functions, conditions, loops, imports and error handling.
    Chapter 4 teaches these through configuration scripts and fake agents.
  - The restrictions of Ravel's Rhai engine and its host capabilities.
  - The `libc` job-control calls the runner makes.
- **Never taught:** TypeScript, Rust language features, shell, Git, JSON,
  general programming, diagram notation, or the general OS concepts above.

<!-- tc: on -->

### What the reader can do afterwards

- Configure and run a session. Write and validate a Rhai configuration script
  with fake agents. Route kinds to real agent programs. Interpret every
  outcome and exit code.
- Explain why Ravel has a separate Rhai worker and two Rust packages,
  and what each part owns.
- Follow successful, refused and cancelled sessions through the code.
  Find where a change belongs.
- Invoke the Ravel CLI as a child process from another program. Explain why
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
The **owner's** executable Rhai configuration script chooses the harness command.
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

The configuration script selects a program, and Ravel runs it. A program in the session
then sends one value with `end-session`. Ravel needs nothing more from the
selected program. `start-session` takes a prompt, and the book expects a
harness to receive it. But the configuration script decides which program runs and whether
that program receives the prompt. Thus, any program that runs `end-session`
can stand where a harness stands.

For this reason, every runnable example uses a fake agent. Ravel assigns no
meaning to kinds or results, integrates no model provider, and stores
nothing. Its first caller is the Grove task-tree driver, which the book shows
only as one labelled illustrative caller.

The book keeps these roles apart:

- the **caller**, which invokes Ravel, supplies the prompt and its ending
  instructions, and interprets the result.
- the **owner**, who writes the configuration script and settings for their
  account.
- **Ravel** itself: the Ravel CLI, the Rhai worker, the runner and the
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
| 1 | What Ravel is for | Say what Ravel solves and whether it fits their need. Explain harness subscriptions, `end-session`, and caller responsibilities. Ravel supplies no store, task meaning or model integration. Say what Rhai is, read a short Rhai configuration script and find Rhai's own documentation. Explain how the routing notice tells the caller what the configuration script chose. Explain how `end-session` finds its session. |
| 2 | Who does what in a Ravel session | name the caller, owner, configuration script, settings, Rhai worker, managed process and opaque result, and say who holds which authority |
| 3 | A complete session, end to end | <!-- tc: depart synonym-rotation because version checks and configuration script validation concern different inputs --> Install Ravel with Homebrew and check its version. Run a session with a tiny configuration script and a fake agent. Describe its stages, with the process and terminal ideas each stage needs. |
| **Part II** | **Use Ravel** | |
| 4 | Write a configuration script | Write and validate a Rhai configuration script with fake agents. Use refusals, selection context, host capabilities, settings and environment grants. Route to harnesses and ask a model through a harness run. |
| 5 | Run sessions | run sessions under real conditions (prompts, kinds, caller data, nesting, what the managed process receives, notices) and interpret every outcome and exit code |
| 6 | Reference | look up any CLI input, setting, bound, request, selection or context shape, environment variable or installed path |
| **Part III** | **Inside Ravel** | |
| 7 | The parts of Ravel | Explain the Ravel CLI and its separate Rhai worker. Explain why Ravel reaps the worker before the managed launch. Explain the separate runner package and how the parts compose. |
| 8 | Follow a session through the code | Follow a successful session from `main` to the result write, then the refusal and cancellation paths. Explain error and signal precedence. Find where a change belongs. |
| 9 | Jobs, signals and the terminal | reason about process groups, escalation, reaping, terminal lending and recovery, entry signal state, and the failure guarantees with their limits |
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

The owner agreed the Rhai revision on 2026-10-26. The book assumes no
TypeScript knowledge and has no migration chapter. Chapter numbers stay unchanged.
The owner's configuration conversion is a separate, one-off manual task.
Chapter 4 teaches the Rhai that its configuration scripts need, without a separate language
tutorial. Both reader checkpoints remain obligations.

The owner's Read 1 findings of 2026-10-27 moved the first presentation of
Rhai to chapter 1. Chapter 1 says what Rhai is and which of its
characteristics matter to a configuration script. It shows a short configuration script and links to
Rhai's own documentation. That documentation is the Rhai Book,
<https://rhai.rs/book/>, with its language reference,
<https://rhai.rs/book/ref/>, and an online playground,
<https://rhai.rs/playground/stable/>. On 2026-10-27, the Rhai Book documented
Rhai 1.27.0, the version that Ravel embeds. The playground does not have
Ravel's host capabilities or limits.

Ravel supports no separate component. Its runner, `keyed-launch`, stays a
private package of the Ravel CLI, which supervises one session and then exits.
The runner can change process-wide signal state, and its return does not
restore that state. A program that needs supervised harness sessions invokes
the Ravel CLI as a child process. The book teaches that caller interface.

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
- **Writing a configuration script:** chapters 1–3, then 4, with 6 for lookup.
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
  use a subscription instead of paid API access. The configuration script can select any
  program, and Ravel needs only one value sent with `end-session`. A harness
  that receives the prompt is the expected use, not a requirement (the
  owner's findings R14 and R15 of 2026-10-18).
- **What success means.** A result reaches the caller only after the managed
  process has been reaped, its whole process group is gone, and cleanup has
  succeeded. Refusal, cancellation, natural exit without a result and cleanup
  failure are distinct, normal outcomes, not edge cases.
- **Who holds which authority.** Configuration script is trusted code that chooses
  the program but is given no result authority: it never receives a
  session-result context, and its worker is reaped before anything launches.
  Only the Ravel CLI supervises. Each managed process holds only its own
  result context.
- **Validating with fake agents.** Every runnable example uses fake programs,
  and a configuration script author learns to check every route and refusal that way.
- **The stated limits, said plainly:** same-user tampering, abrupt death of
  the Ravel CLI, descendants that leave their process group, detached configuration script
  children, and the support statement (aarch64 macOS and aarch64 Linux are
  measured; x86_64 Linux is built and run only under emulation).

### Exclusions

- **History.** The retired actor-daemon project, the dispatch history and
  Ravel's extraction from Grove get at most a one-line provenance pointer.
- **Evidence campaigns.** Campaign plans, cases, receipts and check logs stay
  as evidence documents outside the book. The book states only the limits a
  reader needs, such as the support statement.
- **Installing and configuring agents.** Chapter 4 shows routes to the agent
  programs the owner's configuration script uses today, Claude Code (`claude`) and Codex
  (`codex`), with their actual command-line flags and the ending instructions a
  caller's prompt gives them. Installing those agents, their accounts and
  credentials are out of scope, with a pointer to each agent's own
  documentation.
- **The owner's personal configuration.** The dated handoff record of the
  owner's routing table, `docs/configuration-handoff-2026-10-12.md`, is not
  teaching material. It stays outside the book; it was kept, not deleted, when
  the owner configuration guide was folded in and removed.
- **Configuration script migration.** The book teaches Rhai configuration scripts directly. The owner's
  one-off manual conversion and comparisons with TypeScript stay outside it.
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
  managed process. It is not a POSIX session (`setsid`) or a login session. The
  managed process stays in the caller's POSIX session, in a process
  group of its own.
- **Context**: three different things. The *session-result context* is the
  inherited `RAVEL_EXIT_FILE` path. The *selection context* is the JSON
  data delivered to `select`. The caller's working directory is the request's
  `cwd`. The book never says "context" unqualified.
- **Kind**: the caller's open token, interpreted only by configuration script. It is
  not Grove's task kind, even when Grove passes its task kind as the session
  kind.
- **Channel**: the result channel is Ravel's private directory and completion
  file, through which `end-session` sends the result. It is built on the runner's
  completion `Channel`, which only signals that a file appeared.
- **Worker**: the Rhai worker is a separate process, not a thread.
  The Ravel CLI starts its own binary in an internal role to evaluate the
  configuration script.
- **Result**: the opaque bytes that `end-session` sends. It is never the
  managed process's exit status or the agent's work.
- **Configuration names**: `config.rhai`, `--config-env`, `configEnv` and
  `config_*` diagnostic codes name the configuration script.
- **Harness**: the agent program, such as Claude Code or Codex, in which an
  LLM session works on the prompt; configuration script selects it. It is not a test
  harness.
- **Refusal**: a decision not to launch, made by the configuration script or by Ravel
  checking that configuration script's file, behaviour or data. It is not a worker or supervision
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

**Configuration inputs.** Sources are the configuration script evaluation specification,
the Rhai engine, its host capabilities and the settings reader. Coverage includes:

- The configuration script entry, its discovery and its checks.
- Every settings key, the refusal of unknown keys and every bound.
- The configuration script object's fields and the request, selection and refusal fields.
- The selection context document's fields and the host operations.
- The module exports, file imports, module-resolution rules and shipped examples.

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
- A refusal by the configuration script or of the configuration script's file, behaviour or data.
- A missing, stale or mismatched worker and other worker or supervision failures.
- A selection timeout.
- A selected program that Ravel cannot execute or find.
- Cancellation by INT, TERM or HUP during selection and supervision,
  including after the session sends a result.
- Escalation after Ravel receives the result, and descendant cleanup before reap.
- A surviving group, a cleanup failure or an unwritable result stream.
- Operation reports and nested sessions.
- The refusal of the withdrawn confinement flags.
- Terminal lending and recovery, noninteractive isolation, and recovery after a failed exec.
- The accepted limits under Emphasis.

**Component obligations.** The session specification authorises these ten
obligations. The component guide tabulated them before its removal:

- Caller commands retain their meaning. The owner supplies a Rhai configuration script.
- Configuration script execution has no session-result authority. Ravel reaps its worker
  before the managed launch.
- Ravel judges configuration script output without fallback.
- The selected program runs directly in the caller's context.
- Ravel keeps the first complete result that it receives, unchanged.
- Success requires reap and confirmed group cleanup.
- Cancellation suppresses results.
- Ravel recovers the terminal's foreground and modes.
- Only result bytes reach stdout.
- Installed execution uses one binary for the Ravel CLI and its Rhai worker.

**Contributor workflows.** The source is `task --list`. Coverage includes
setup, build, tests, checks, install, smoke, release tasks and the book's tasks.
The native-entry, incumbent and Linux measurement tasks are evidence instruments.
The book names them and points to their evidence without teaching their use.

**Omissions and unknowns.** Each coverage report states these limits.
The book also states them wherever they matter:

- Whether a developer new to Ravel can use the book is unknown.
  The reader checkpoints are the owner's stand-in reads (part 4).
- Ravel supports no separate component. The private runner serves the Ravel
  CLI, which supervises one session and then exits. The component
  inventory describes internal obligations, not a library interface for
  another project.
- Native x86-64 Linux is unqualified. The project built it, linted it at
  Rust 1.85.0 and ran it only under emulation.
- Real agents' flags are known only as of their recorded check. The final
  check on 2026-10-16 recorded Claude Code 2.1.301 and codex-cli 0.163.0.
  Drafting rechecks and records the versions it uses.
- GitHub's rendering is unchecked until the branch is pushed.
  The site is unchecked until the first release that carries the book.
- Inventory coverage shows what the book covers, not whether a reader understands it.

<!-- tc: pending because this large contract is converted by the sections each edit touches -->

### Examples policy

- **Every complete example runs**, through the seam Ravel's CLI tests
  already use: the built `ravel`, real Rhai configuration script evaluation, fake
  agent programs on a private PATH, a cleared environment and a temporary
  HOME and TMPDIR. A Taskfile task runs every complete example and compares
  its output exactly.
- **No example launches a real agent or model**, and none uses a personal
  configuration script or account. A route to a real agent program is validated through a
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
| What does the configuration script return, and how does Ravel tell the caller which harness, model and effort the configuration script chose? | UML sequence, with a real routing notice as its example | chapter 1, then chapters 3 and 5 |
| How does `ravel end-session`, which runs inside the harness, find the session that launched the harness, also when sessions nest? | Process tree with the inherited `RAVEL_EXIT_FILE` (C4 dynamic or UML object) | chapter 1, then chapters 2 and 5 |
| How does an evaluation harness return data to the configuration script before the managed launch? | UML sequence | chapters 4 and 6 |
| Which processes exist during a session, and which holds which authority? | C4 container | chapters 2 and 7 |
| What happens, in order, in a successful session? | UML sequence | chapter 3, then in full in chapter 8 |
| What changes when cancellation arrives during selection versus during supervision? | UML sequence, one per phase | chapters 5 and 9 |
| Which states does a session pass through to each outcome? | UML state machine | chapters 3 and 5 |
| What is the timing after `end-session` sends the result (2-second grace, TERM, up to 5 seconds, KILL)? | UML timing | chapter 9 |
| How is the terminal lent to the managed process and recovered? | UML sequence | chapter 9 |
| How do the packages, the Rhai worker and the examples fit together in the source and in an installation? | C4 component and UML deployment | chapters 6, 7 and 11 |
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

## 2. Prose contract

- <!-- tc: depart long-sentence because the sentence quotes the owner's words verbatim --> **Base**: ASD-STE100 Simplified Technical English, Issue 9 (2025-01-15).
  STE applies to all text in the book: prose, headings, tables, lists, figure
  labels, captions, keys and alt text. The owner made this decision on
  2026-10-17. In the owner's words: “I intend to add a global instruction to use ADE-STE100 for all
  LLM communication (such as when you talk to me here) as well as in all
  documentation. Apply that rule now.” On 2026-10-18, the owner limited the
  base to the writing rules. In the owner's words: “The skill I am going to adopt does not use the
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
- **Term list**: `CONTEXT.md` plus the term risks above. When the book first
  needs a Ravel term that `CONTEXT.md` does not have, the stage that writes it
  adds the term there.

The [documentation and communication lessons](../documentation-lessons.md) record what this grove learnt for later documents and books.
Each later session adds its new lessons there, with their sources, evidence classes and Ravel applications.
The record distinguishes the owner's decisions from a session's analysis.

The owner's Read 1 patterns follow, as this book takes them. The owner took
them from six passages of chapter 1 on 2026-10-27 and asked for them in every
chapter. In the owner's words:

<!-- tc: off because these are the owner's quoted words -->

> I would like you to apply the patterns of these re-statements to the result
> of Part I and II, and then I will re-review.
>
> Good human-centered writing doesn't use DRY at all. Repetition and
> restatement are important techniques.
>
> We should also use a more conversational tone where possible

<!-- tc: on -->

The patterns add to STE. Each example below comes from the owner's comment on
one of the six passages. The history of `rhai-book-use-k74` keeps the passages
and the comments in full.

- **Give a concrete example before the general case.** Show a
  particular task, real values, the actions and the result before the abstract
  roles or model. Then explain which parts apply to other tasks. Carry the
  same example through the explanation so that the reader can connect each
  general statement to something they already saw.
- **Introduce a technology where the book first names it.** Say what it is
  and which of its characteristics matter to the reader. Show a short example.
  Link to the technology's own documentation. For Rhai, the owner's words were
  “describe its characteristics, show an example, and link to some
  documentation”.
- **Explain a term before you use it.** At the first use of a term on each
  page, explain it in plain words, even when another chapter teaches it. Say
  what it is, where it comes from and why it matters here. A link to the full
  explanation can follow, but it never replaces the explanation. Chapter 1
  used “a selection” before the book said what a selection is.
- **Say who does what, where and why.** Name the actor, the action, the place
  and the reader of each output. When a write gives information to another
  actor, say that it is a means of communication. For example, Ravel writes
  the routing notice as one line on the caller's standard error. That line
  tells the caller which harness, model and effort the configuration script chose.
- **Show the mechanism.** When the text says that something is inherited,
  found, sent or chosen, explain how. For example, Ravel puts the path of the
  result file in the `RAVEL_EXIT_FILE` environment variable of the program
  that it launches. Each child process inherits that variable from its
  parent, unless the parent removes it. Thus, `ravel end-session` finds the
  path in its own environment. A nested session gives its own child a new
  path.
- **Give a diagram and a concrete example.** Each mechanism and each
  communication path gets a figure and an example with real values. Real
  values are a real command, its real output, a real path or a real JSON
  document. “Figure needs” lists the figures.
- <!-- tc: depart synonym-rotation because the quote keeps the owner's word "modify" --> **Describe the effect in the reader's terms.** Say what changes for the
  reader or for the thing that they make. The caller can use the kind “to
  extend or modify the prompt”, not “to choose instructions for the prompt”.
- **Put the reader's goal first.** Before a capability, say why you may want
  it. Then say how to do it. For example: “You may want an LLM to choose the
  best harness, model, effort and prompt additions for a kind and a prompt.
  You can do this with a harness run in your configuration script.”
- **Follow an action to its result.** Name the actors in the order that they
  act, and end with what the caller receives. For example, the program that
  Ravel launched, usually a harness that runs an LLM, runs `ravel end-session`
  with a result. Ravel then stops that program and the rest of its process
  group. After cleanup, that result becomes the result of `ravel start-session`.
- **Repeat and restate.** The book does not apply DRY (“Don't Repeat
  Yourself”) to its explanations. Restate a key fact where the reader needs
  it, even when an earlier page states it. Repeat a short explanation instead
  of a link to another page. A preview at the top of a long section and a
  summary at its end help the reader. Use the same terms each time. The text
  beside a figure still does not repeat the figure's colour key (the owner's
  finding R8 of 2026-10-17).
- **Talk to the reader.** Use a conversational tone where the STE rules permit
  it. Address the reader as “you”. Ask the question that the reader has, then
  answer it. Give the reason before the detail. The STE rules still apply, so
  the book has no contractions, slang or jokes.

The owner added the concrete-first rule after the complete chapter 1 read on
2026-10-28. The owner's words were:

<!-- tc: off because this is the owner's explanation of the teaching order -->

> It is always more useful I have found to use a concrete example/explanation and then abstract/broaden/generalise that, as opposed to introducing an abstract model and then concretising the concept.

<!-- tc: on -->

That read also adds these requirements for the next correction of the book:

- **Explain the caller's complete task contract.** The opening review example
  names the files, the work and the prompt. Its prompt gives the permitted
  results and the condition for each `ravel end-session` command. The example
  then shows what the orchestrator does after each result. These values are
  the caller's convention. Ravel assigns them no meaning.
- **Prefer familiar configuration terms.** Replace “owner policy” in
  reader-facing prose with “configuration”. Distinguish the executable Rhai
  script from the JSON settings wherever the distinction matters. Reconcile
  the glossary, related terms, headings and figures with that choice. The
  correction keeps literals exact against the implemented interface. Owner
  finding F18 now requests the product-name change too. `configuration-names-k92`
  owns that change before the next book correction. The draft then updates
  filenames, options, fields, diagnostics and their explanations together. Draft
  `rhai-book-use-k88` chose “configuration script”, “settings”, “Rhai worker”
  and “selection context”. The table is in `ste.md`, “Configuration words”.
- **Make example setup concrete.** Before a procedure asks the reader to create files,
  name the example and show its workspace, directories and file paths. Explain each file's purpose and give its contents.
  Distinguish files that the reader saves from directories or files that a command creates.
  Show a complete command before a table explains its parts. Tie that explanation to the named example.
  State a later example's differences where they occur. Do not imply that every command has the same parts.
  Owner finding F17 of Read 1, 2026-10-28, requires this pattern throughout the book.
- **Explain JSON by its purpose.** Say “JSON document”, “routing notice” or
  “failure document” where that description is sufficient. Do not attach a
  version number to each mention of JSON. Exact examples keep required
  `schemaVersion` fields. The reference explains their values where a reader
  must construct or interpret a document.
- **Explain why the worker is a separate process.** At the first relevant
  mention, say that Ravel launches another copy of its own executable to
  evaluate the configuration script. Explain that the parent can enforce a
  deadline and handle requests while the worker evaluates the script.
  Explain how the process boundary isolates a script failure. The final
  managed launch still waits for the worker to finish.
- **Link a named external project.** Link [Grove](https://github.com/Linkuistics/grove)
  to its repository where each page introduces it. Keep the local explanation
  of its role, so the reader need not follow that link.
- **Use descriptive headings.** Name the subject, reader question or outcome
  in familiar words. Avoid unusual shorthand that the reader must decode.
  Replace “Understand the evidence limits” with a heading about the actual
  checks and their scope. Replace “The actors and their words” with a title
  about who does what in a Ravel session. Reconcile the contents, chapter
  references and links when a title changes.
- **Use commands that the reader already knows.** When the prerequisites
  include a tool, name its familiar command in the procedure. Chapter 3 can
  say “and `cd` into it”, because the book assumes everyday shell use.
  Prefer that concrete instruction to “make it the current directory”.
  Explain unfamiliar commands when the reader first needs them.

Findings F9–F16 in `rhai-book-use-k74` preserve the owner's words and each
finding's disposition. These requirements apply throughout the book.
The later draft of Parts III and IV uses them, and its proof checks them.

The house style's points, as this book takes them:

- **Task headings**: the first word of a task heading is a bare infinitive.
  Write "Write a configuration script", not "Writing a configuration script" or "To write a configuration script".
- **Introducing steps**: a complete sentence introduces a procedure, or nothing
  does when its heading already says what it does. A fragment that the steps
  complete, such as "To install:", does not.
- **Register**: plain, conversational prose in STE, as “Talk to the reader”
  states. It has no slang, idiom, joke or transcribed speech.
- **Length**: no length target for a section. STE sets the limits for
  sentences and paragraphs. An instruction has 20 words or fewer (Rule 5.1). A
  descriptive sentence has 25 words or fewer (Rule 6.3). A paragraph has one
  topic and six sentences or fewer (Rules 6.5 and 6.6). Use a vertical list
  for complex text (Rule 4.3).
- **Punctuation**: do not use semicolons (Rule 8.1) or contractions (Rule
  4.2).
- **Steps**: the location, condition or goal the reader needs first comes
  before the action, and the result after it. A step's first sentence is an
  imperative. Running text is indicative.
- **Demonstratives**: "this" or "these" takes a noun where its referent could
  be unclear.
- **Voice and verbs**: use the active voice. In descriptive text, use the
  passive voice only when the agent is unknown (Rule 3.6). Use only the simple
  tenses (Rule 3.2). Use “-ing” forms only in technical names (Rule 3.5). Do
  not use phrasal verbs (Rule 9.3).
- **Front-loading**: the first words of a heading, a list item or a paragraph
  carry its information.
- **Terms**: at its first use on each page, a term from the term list gets a
  short explanation, as “Explain a term before you use it” states. A link to
  the place where the book teaches the term follows that explanation.
- **Code**: code font for identifiers, commands, file names, paths and literal
  values. A code block has fences and a language tag.
- **Example labels**: a label introduces each example. Its first words are
  "Complete example" or "Fragment". The label of a fragment says what it
  omits.
- **Stock phrasing**: the book does not use these forms:
  - a filler opener or closer, such as "It's worth noting".
  - a hedge without a condition, such as "typically" without saying when.
  - an inflated word where a plain one serves, such as "leverage", "robust"
    or "seamless".
  - a reflexive list of three.

  A preview, a restatement or a summary that helps the reader is not stock
  phrasing. “Repeat and restate” gives the owner's decision of 2026-10-27.
- **Simple layout**: the headings the section plan states stand. Beyond them, a
  stage adds a heading level only for a section with at least two subsections. A
  paragraph carries one idea. Parallel items form a list, and items compared on
  the same attributes form a table. Bold marks only a run-in label or a term at
  its definition. A stage adds nothing for decoration.
- <!-- tc: depart long-sentence because the sentence quotes the owner's words verbatim --> **Breaks in prose**: the owner made this decision on 2026-10-18. In the owner's words: “I also
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
  with `|` as its first character) and an image (a line with `![` as its
  first characters) break a run. `task docs:breaks -- --strict FILE…` stops with status 1 when it finds
  a longer run. `task docs:breaks:test` shows that the check finds a longer
  run and that each rule changes the result.
- **Statements and asides**: in chapter 6, a Markdown blockquote sets a note,
  an example or a summary apart from the statement of an entry. The
  blockquote opens with its label, for example `> **Note:** …`. Each of its
  lines has the `>` mark, and a blank line ends it.

<!-- tc: pending because this large contract is converted by the sections each edit touches -->

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
- **Read 1** follows proof of Parts I and II. The tasks are to write a configuration script
  for a fake agent and run a session. The reader interprets a refusal and a
  cancellation. The reader also uses one direct entry, such as an exit code
  or error message, without the preceding text. Corrections land before the
  draft of Parts III and IV.
- **Read 2** follows proof of Parts III and IV. The reader explains why Ravel
  has a separate Rhai worker and two Rust packages. The reader follows a session
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
