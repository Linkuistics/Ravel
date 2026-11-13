# What Ravel is for

Ravel lets a program use an LLM in Claude Code or Codex as one step of its
own work. This chapter opens with one such step, a code review. It follows
that review from the request to the result. Then it explains which parts of
the review apply to every use of Ravel.

You do not run anything in this chapter. Each code block is a fragment, a part
of the review or of a real run, and its label says what it omits.
[Chapter 3, “Run the first session”](03-a-complete-session.md#run-the-first-session),
gives the first session that you run. It names the directory, each file and
each command.

## Follow one code review from request to result

An **orchestrator** is a program that runs LLM work as steps of its own work.
Suppose that an orchestrator wants an LLM to review the last commit of a
project. The commit changes two files, `src/parse.rs` and `tests/parse.rs`.
The orchestrator knows these files, because it chose the work. It also knows
what it will do after the review.

First, the orchestrator writes a prompt. The prompt is the text that tells the
LLM what to do. This prompt also tells the LLM how to report the outcome of
the review.

Fragment: the whole prompt file `review-prompt.txt` of the review example.
This block omits the code of the orchestrator that writes the file. Ravel
keeps the text unchanged in `request.originalPrompt` and does not interpret
it.

```text
Review the changes to src/parse.rs and tests/parse.rs in the last commit.
Check that the code is correct and that the tests cover each change.

When the review is complete, end this session with one command:
- If you found no problem, run: ravel end-session approved
- If you found a problem, write your findings to review-notes.md.
  Then run: ravel end-session changes-needed
```

The prompt has four parts. Each part has a different purpose:

| Part of the prompt | Purpose |
|---|---|
| The work: review two named files | Tells the LLM what to do |
| The two permitted results, `approved` and `changes-needed` | Gives the only values that the orchestrator expects |
| The condition for each result | Tells the LLM which value to send |
| The command `ravel end-session RESULT` | Tells the LLM how to end the session and send the value |

Then the orchestrator runs Ravel as a child process. `--kind review` names the
sort of work. The prompt file is the prompt.

Fragment: the request of the review example. This block omits the code of the
orchestrator that runs the command.

```sh
ravel start-session --kind review --prompt-file review-prompt.txt
```

Do not type this command now. It needs an installed Ravel, the file
`review-prompt.txt` in the current directory and a configuration script with
a rule for the kind `review`. Chapter 3 installs Ravel. Then it gives you the
files and the command of a simpler first session.

Ravel gives the kind and the prompt to the owner's configuration. The
**owner** is the person who controls Ravel's configuration for an account. The
configuration chooses a harness for each kind. A **harness** is the program in
which an LLM works on a prompt, such as Claude Code. In this example, the
configuration chooses Claude Code for the kind `review`.

What happens next? The table follows the review in order. In this run, the
LLM finds a problem.

<!-- tc: depart synonym-rotation because launch names process creation, while start also names the beginning of output -->
| Order | Who acts | What happens in the review |
|---|---|---|
| 1 | The orchestrator | Runs `ravel start-session` with the kind `review` and the prompt file |
| 2 | The owner's configuration | Chooses Claude Code and its arguments for the kind `review` |
| 3 | Ravel | Launches Claude Code with the prompt and supervises it |
| 4 | The LLM in Claude Code | Reviews the two files, finds a problem and writes `review-notes.md` |
| 5 | The LLM in Claude Code | Runs `ravel end-session changes-needed` |
| 6 | Ravel | Stops Claude Code and the other processes that remain in its process group, the set of processes that Ravel can signal together. Then Ravel completes cleanup. |
| 7 | Ravel | Writes `changes-needed` on standard output. `ravel start-session` exits with status 0. |

The sequence connects the review request to the orchestrator’s next action.

![The orchestrator sends the review kind and prompt to Ravel. The configuration selects Claude Code. Ravel launches it, and the review writes notes and sends changes-needed. After successful cleanup, Ravel returns status 0 and changes-needed. The orchestrator reads the notes and requests a fix.](figures/review-flow.svg)

*Figure: The orchestrator acts on the result only after Ravel completes cleanup.*

[Open the review sequence at full size](figures/review-flow.svg).

Here is a real run of this request, with the checkout build on macOS. Its
configuration script was the short script in
[“The configuration script chooses a program”](#the-configuration-script-chooses-a-program),
below, at the default path of a private home directory. That script selects a
fake agent in place of Claude Code. Thus, the routing notice names the
provider `local` and the model `fake`.

The fake agent wrote `review-notes.md` and sent `changes-needed`, as the LLM
did in step 5. The table omits the environment of the run and two cleanup
reports on standard error.

| Output of the run | Value | Who reads it |
|---|---|---|
| Standard output | `changes-needed`, with no newline | The orchestrator, as the result |
| Standard error | `ravel: starting provider local, model fake, effort low for kind "review"` | The person or the log that watches the session |
| Exit status | `0` | The orchestrator, to know that the session succeeded |

The orchestrator now acts on the result. Its own code decides what each value
means:

| Outcome of `ravel start-session` | What the orchestrator does next |
|---|---|
| Status 0 and the result `approved` | Marks the review as complete and moves to its next task |
| Status 0 and the result `changes-needed` | Reads `review-notes.md`, then requests a session of the kind `fix` with the notes in its prompt |
| Status 0 and another value | Treats the review as failed, because its prompt permitted only two values |
| A status other than 0 | Reads standard error, then decides whether to request the review again. It does not use standard output as a result. |

Ravel does not know what `approved` and `changes-needed` mean. These two
values are the orchestrator's convention, which its prompt states. Ravel
returns the value that the LLM sent, unchanged, after it stops Claude Code and
completes cleanup. The orchestrator gives the value its meaning.

## What the review shows about every session

The review had three participants with different responsibilities. Every use
of Ravel has the same three. The table connects each general role to its part
in the review.

| Participant | In the review | In general |
|---|---|---|
| [Caller](02-actors-and-words.md#caller-and-owner) | The orchestrator wrote the prompt and acted on `changes-needed` | Supplies the kind and the prompt. The prompt gives the permitted results and the `end-session` instruction. The caller interprets the result. |
| [Owner](02-actors-and-words.md#caller-and-owner) | Configured Claude Code for the kind `review` | Writes the configuration that chooses a program for each request |
| Ravel | Launched Claude Code, stopped it and returned `changes-needed` | Runs the chosen program, supervises it and returns the result after cleanup |

The **caller** is the person or program that runs `ravel start-session`. The
orchestrator is one caller, and you are the caller when you type the command
yourself. The owner and the caller can be the same person, but their
responsibilities stay separate. One run of `ravel start-session`, from the
request to the result, is a
[session](02-actors-and-words.md#caller-and-owner).

Why use a harness, and not a direct call to a model? A harness such as Claude
Code or Codex can run under the subscription of its user. Thus, your program
can use that subscription for LLM work instead of paid API access. The cost
can be much lower for work that the subscription covers. The
[Claude Code documentation](https://code.claude.com/docs/en/authentication)
and [Codex documentation](https://learn.chatgpt.com/docs/auth) explain their
access options and link to their plans and terms. This paragraph agrees with
those pages as they were on 18 October 2026.

Why does the prompt tell the LLM to run `ravel end-session`? An interactive
harness waits for more input when the LLM completes its task. It does not exit
by itself. Thus, the orchestrator cannot wait for the harness to exit.
`ravel end-session RESULT` gives the LLM a way to say that the work is
complete.

The caller chooses the permitted values and the condition for each value.
Ravel stops the harness and, after cleanup, returns the value to the caller.

## From a request to a result

This section follows the same review through Ravel in more detail. It names
each actor in the order in which that actor acts. Each subsection below
explains one row of this table, with a real example.

<!-- tc: depart synonym-rotation because launch names process creation, while start also names the beginning of output -->
| Order | Who acts | What happens |
|---|---|---|
| 1 | The caller | Runs `ravel start-session` with a kind and a prompt |
| 2 | The configuration script | Selects a program and its arguments, or refuses the request |
| 3 | Ravel | Writes the text routing notice, which tells the caller what the configuration script chose. With `--json`, the notice follows the supervision attempt. |
| 4 | Ravel | Launches the selected program and supervises it |
| 5 | The selected program | Sends a result with `ravel end-session` |
| 6 | Ravel | Stops the program, completes cleanup and returns the result to the caller |

### The caller asks for a session

The caller runs `ravel start-session` with a prompt and a
[kind](02-actors-and-words.md#caller-and-owner). The prompt is the text that
tells the LLM what to do. A kind is a short token that names the sort of work
that the caller wants. The caller gives the prompt with `--prompt TEXT` or, as
in the review, with `--prompt-file PATH`.

In the review example, the caller uses `review` as the kind. The caller and
the owner's configuration agree on that token. Ravel checks only that the kind
is not empty. It has no built-in review category and gives the kind no
meaning.

<!-- tc: depart synonym-rotation because the sentence keeps the owner's word "modify" from finding F3 -->
The caller can also use the kind to extend or modify the prompt. For example,
if the harness supports skills, the caller can add a line to the prompt for
the kind `review`. That line tells the LLM which skill to load. A skill is a
set of instructions for a particular task. This use of the kind is a caller
convention.

Ravel keeps the caller's prompt unchanged in
`request.originalPrompt`. With skill delivery enabled, `request.prompt`
holds discovery instructions followed by the caller's text in a separate
block. The script passes that prepared prompt to the harness. Ravel adds no
task meaning or instruction that ends the session.
[Chapter 6](06-reference.md#skill-delivery-at-launch) explains skill delivery.

### The configuration script chooses a program

The owner's configuration has two parts. The **configuration script** is a
file of Rhai code that selects a program for each request. The **settings**
are values in a JSON file, such as limits and the harnesses that the script
can run. The table gives the default path of each part:

| Part of the configuration | Default path | What it holds |
|---|---|---|
| Configuration script | `$HOME/.config/ravel/config.rhai` | Rhai code that selects a program or refuses the request. `--config PATH` names another script for one session. |
| Settings | `$HOME/.config/ravel/settings.json` | The deadline, limits, environment grants, harnesses, budgets and skill sources. The script cannot change them. |

Ravel gives the request to the configuration script. Then the script selects
a program and its arguments, or it refuses to launch a program.

What is Rhai? Rhai is a small computer language for programs that are
written in Rust. A Rust program can contain the Rhai engine and run Rhai code
in its own process. Ravel does this. Thus, the `ravel` executable already
contains everything that runs your configuration script, and you install no
separate language runtime.

[Rhai's feature list](https://rhai.rs/book/about/features.html) names these
characteristics. Each one is important for a configuration script:

- **A familiar syntax.** Rhai looks like a mix of JavaScript and Rust. It has
  variables, functions, `if`, loops, arrays and maps.
- **Dynamic types.** You do not declare a type. Each value has its type while
  the script runs.
- **A sandbox.** Rhai code changes things outside the script only through
  functions that the host program gives it. In Ravel, the configuration script
  reads files and runs harnesses only through `ravel::` functions. The book
  calls these functions
  [host capabilities](02-actors-and-words.md#evaluation-and-supervision).
- **Protection against runaway scripts.** Rhai can stop a script that runs
  too long, calls functions too deeply or makes data that is too large. Ravel
  sets these limits, and one deadline covers each evaluation.
- **Modules.** A configuration script can import other Rhai files.

Here is a short configuration script for the review example. The book's
examples must run without a real harness. Thus, this script selects
`fake-agent`, a small program that stands in for Claude Code. It refuses every
kind except `review`.

Fragment: a short configuration script for the review example. This block
omits the program `fake-agent` and the command that runs the script. You read
this script here, and you save nothing.
[Chapter 3](03-a-complete-session.md#run-the-first-session) runs a script of
this sort with a fake agent.

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "chapter-1";

fn select(request, context) {
    if request.kind == "review" {
        return #{
            status: "selected",
            program: "fake-agent",
            args: ["You are a careful code reviewer. " + request.prompt],
            provider: "local",
            model: "fake",
            effort: "low",
            reason: "Reviews go to the fake agent.",
        };
    }
    #{
        status: "refused",
        code: "unknown_kind",
        message: "This configuration script has no rule for the kind " + request.kind + ".",
        remedy: "Use the kind review.",
    }
}
```

Read the configuration script from the top. The table says what each part
does:

| Part of the script | What it does |
|---|---|
| `export const SCHEMA_VERSION = 4;` | Names the version of the interface between Ravel and the script that this file uses |
| `export const VERSION = "chapter-1";` | Names this revision of the owner's script |
| `fn select(request, context)` | Defines the function that Ravel calls. `request` holds the kind, the prompt and other data of the request. `context` holds optional selection context, which is data for the choice. |
| `if request.kind == "review"` | Applies the first rule only to the kind `review` |
| `#{ ... }` | Makes a map, which is a set of named fields |
| `return` | Ends the function with the map that follows it |
| `"You are a careful code reviewer. " + request.prompt` | Puts an instruction before the caller's prompt |
| The last map | Is the value of the function when no `return` ran. This map is a refusal. |

How does Ravel run the script? The Ravel CLI, the `ravel` command-line
program, does not run it in its own process. It launches a second copy of
its own executable as a child process. That child is the
[Rhai worker](02-actors-and-words.md#evaluation-and-supervision). The worker
evaluates the configuration script and sends the selection or the refusal to
its parent.

Why a separate process? The parent stays in control while the script runs:

- **The deadline.** Each evaluation has a deadline. When it expires, the
  parent ends evaluation and can signal the worker process. Rhai cannot
  interrupt some of its own functions, such as `sleep`. A kernel stall can
  delay the worker's exit.
- **Requests from the script.** The worker only computes. When the script
  reads a file or runs a harness, the worker asks the parent. The parent does
  the work and sends the data back.
- **Isolation of a failure.** If the script exhausts the memory of the worker
  or overflows its stack, only the worker stops. The parent continues,
  reports the failure to the caller and launches no program.

The worker does not run at the same time as the selected program. Ravel waits
until the worker exits and collects its exit status. Only then does Ravel
launch the program that the script selected.

Do you want to learn more Rhai? Rhai has its own documentation. Chapter 4
teaches the Rhai that a configuration script needs, so you do not need these
pages for this book:

- [The Rhai Book](https://rhai.rs/book/) is the manual. On 27 October 2026, it
  documented Rhai 1.27.0, the version that Ravel contains.
- [The language reference](https://rhai.rs/book/ref/) gives the syntax in a
  short form.
- [The online playground](https://rhai.rs/playground/stable/) runs Rhai code
  in your web browser. It has no `ravel::` functions and none of Ravel's
  limits. Thus, a script that calls a host capability does not run there.

### The selection and the routing notice

What does the configuration script give back to Ravel? It gives back a
selection or a refusal. A **selection** is the map that `select` returns when
it chooses a program. Its `status` field is `"selected"`. A
[refusal](02-actors-and-words.md#evaluation-and-supervision) is a map
with `status: "refused"`.

After a refusal, Ravel launches no final managed
process and returns no result to the caller. Evaluation harness runs can occur
before the refusal.

The selection of the review example has these fields:

| Field | Value in the example | Who uses it, and how |
|---|---|---|
| `program` | `fake-agent` | Ravel finds this program and runs it |
| `args` | One string: the instruction, then the caller's prompt | Ravel gives these arguments to the program, without a shell |
| `provider` | `local` | Ravel writes it in the routing notice |
| `model` | `fake` | Ravel writes it in the routing notice |
| `effort` | `low` | Ravel writes it in the routing notice |
| `reason` | `Reviews go to the fake agent.` | Ravel writes it in the JSON form of the routing notice |

`program` and `args` are the command that Ravel runs. The other fields are
labels. A label describes the choice of the configuration script, but it does
not configure the harness. For example, a script that selects Claude Code puts
the model option of Claude Code in `args`. The `model` label only tells the
caller which model the script chose. Ravel does not check that the labels
agree with the arguments.

Why does the caller need these labels? The caller usually wants to know what
the configuration script chose, but the caller does not read the owner's
script. Ravel tells the caller in the **routing notice**:

- **Who writes it:** the
  [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision), the
  `ravel` command-line program.
- **When:** in text form, after selection and before the launch. In JSON form,
  after the supervision attempt.
- **Where:** one line on the caller's standard error. Standard output carries
  only the result.
- **Who reads it:** the caller, or the person who watches the terminal.
- **Why:** this line is the way in which Ravel tells the caller which harness,
  model and effort the configuration script chose.

Fragment: the routing notice of the review example, as Ravel writes it on
standard error. This block omits the other lines on standard error.

```text
ravel: starting provider local, model fake, effort low for kind "review"
```

The selection separates the command that Ravel launches from the labels that the caller reads.

![The Rhai worker returns the fake-agent command and the local, fake and low labels. The Ravel CLI reaps the worker, writes the routing notice on the caller's standard error and launches fake-agent with args. Labels do not go to the program. With JSON, the notice follows the supervision attempt.](figures/routing-notice.svg)

*Figure: The review selection reaches the program and the caller by separate paths.*

[Open the selection and routing notice at full size](figures/routing-notice.svg).

With `--json`, Ravel writes a JSON routing notice in place of that line. The
JSON document gives more detail, such as the `reason` and the program file
that Ravel found. Ravel writes this document after the supervision attempt,
not before the launch.

Fragment: the `selection` member of the JSON routing notice of the same
request, run again with `--json`. This block omits the other members of the
document.

```json
{"selection":{"effort":"low","model":"fake","provider":"local","reason":"Reviews go to the fake agent."}}
```

[Chapter 5, “Read notices separately from results”](05-running-sessions.md#read-notices-separately-from-results),
teaches both forms of the routing notice.

### An LLM can help the configuration script choose

<!-- tc: depart synonym-rotation because launch names Ravel's launch of the selected program, while start names a child that another process or the script creates -->
You may want an LLM to choose the best harness, model, effort and prompt
additions for a kind and a prompt. You can do this with a
[harness run](02-actors-and-words.md#evaluation-and-supervision) in
your configuration script. A harness run is one run of a program from the
`harnesses` entry of your settings. The script starts it with the host
capability `ravel::run_harness`. The harness uses the owner's sign-in, and
Ravel holds no model credential.

The configuration script reads the answer of the harness as data. Then it
builds its selection from that answer. A harness run does not complete the
session. Its result goes back to the script, not to the caller. Ravel
supervises each harness run within the owner's
[evaluation budget](02-actors-and-words.md#evaluation-and-supervision),
which limits the work of one evaluation of the configuration script.
[Chapter 4](04-write-a-configuration-script.md) gives the recipes for Claude Code
and Codex.

### Ravel runs the program, and the program sends a result

During the session, Ravel supervises the program that the configuration
script selected. This program is the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
of the session. The harness runs the LLM session that works on the prompt.
That LLM session is not the Ravel session. Ravel supervises the harness
process, not the conversation of the LLM in that process.

The contract with the selected program is loose. The configuration script
selects a program, Ravel runs it, and a program in the session sends one value
with `end-session`. Ravel needs nothing more from the program. The selected
program does not have to use an LLM or understand a prompt.

The table separates what Ravel needs from the harness use that this book
teaches. The runnable examples use ordinary programs as fake agents.

| Part of a session | What Ravel needs | What this book expects from a harness |
|---|---|---|
| Selected command | The configuration script selects a program and its arguments | The program runs an LLM session |
| Prompt | The caller supplies `--prompt` or `--prompt-file`. Ravel keeps that text unchanged in `request.originalPrompt`. The script decides whether its program receives `request.prompt`, which also holds discovery instructions when skill delivery is enabled. | The script passes the prompt to the harness |
| Result | A program in the session sends one value with `end-session` | The LLM follows the caller's instructions and tells the harness to run that command |

The caller's prompt tells the LLM to run `ravel end-session RESULT` when the
task is complete. `RESULT` is a placeholder for the text to send. As in the
review, the prompt gives the permitted values and tells the LLM when to use
each value.

A **process group** is a set of processes that the operating system can
signal together. Ravel puts the selected program in a new group. Its children
stay in that group unless they leave it.

In the review, the LLM found a problem. It ran
`ravel end-session changes-needed`, as the prompt told it. The program that
Ravel launched, usually a harness that runs an LLM, runs that command. Ravel
then stops that program and the rest of its process group. After cleanup,
`changes-needed` becomes the result of `ravel start-session`.

Ravel writes `changes-needed` unchanged on the caller's standard output. The
orchestrator interprets `changes-needed` as a request to read the notes and to
request a `fix` session. Ravel gives `changes-needed` no meaning and starts no
next task. The book calls this text an
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result),
because Ravel does not interpret it.

### How `end-session` finds its session

How does `ravel end-session` know which session to end? It takes no session
ID and no target argument. It finds the session through an environment
variable, `RAVEL_EXIT_FILE`.

Before the launch, Ravel makes a
[result channel](02-actors-and-words.md#the-managed-process-and-its-result)
for the session. The result channel is a private temporary directory, with a
path for one result file in it. Ravel puts that path in the `RAVEL_EXIT_FILE`
environment variable of the program that it launches. The book calls this
path the
[session-result context](02-actors-and-words.md#three-different-kinds-of-information).

How does the path reach `end-session`? When a process starts a child, the
child gets a copy of the environment of its parent, unless the parent changes
it. A harness runs each command that the LLM requests in a child process,
for example a shell. That shell starts `ravel end-session`. If each parent leaves `RAVEL_EXIT_FILE` unchanged, each child inherits the
same path. Thus, `ravel end-session` finds the path in its own environment
and writes the result to that file.

| Process | Which process starts it | Its `RAVEL_EXIT_FILE` |
|---|---|---|
| `ravel start-session` | The caller | None, unless the caller itself runs in a session |
| The harness, which is the managed process | Ravel | A new path that Ravel makes for this session |
| A shell for a command of the LLM | The harness | The same path, inherited from the harness |
| `ravel end-session changes-needed` | That shell | The same path, inherited from the shell. The command writes the result there. |

Each child inherits its parent's path unless its parent changes it.
Ravel sets a new path for the inner harness.

![A caller starts outer start-session without a result path, and its Rhai worker has no path. Ravel gives the outer harness an outer path, which its shell and end-session inherit. The shell starts inner start-session with the outer path. The inner harness receives a new inner path, which its end-session inherits. Each end-session writes to its own path. An evaluation harness run gets a separate path and returns its result to the configuration script.](figures/inherited-result-path.svg)

*Figure: The nested run gives each session's children its own result path.*

[Open the inherited result paths at full size](figures/inherited-result-path.svg).

Here is a real run. A fake agent printed its own `RAVEL_EXIT_FILE`. Then it
was the caller of a second, inner session. The inner fake agent printed its
value, and the outer fake agent printed its value again.

Fragment: standard error of a real nested session on macOS with
`TMPDIR=/tmp`. This block omits the configuration script, the two fake agents
and the cleanup reports. The random parts of each
path change in each run. macOS gives `/tmp` the real path `/private/tmp`.

```text
ravel: starting provider local, model fake, effort none for kind "outer"
outer-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-mlcN86/signal-c7d5f2eed4198eb768996f7b4868bbcc
ravel: starting provider local, model fake, effort none for kind "inner"
inner-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-QpztGn/signal-b643cbee529e5887c7d2a3d0496210c9
outer-agent: the inner session returned inner-done
outer-agent: RAVEL_EXIT_FILE=/private/tmp/ravel-session-mlcN86/signal-c7d5f2eed4198eb768996f7b4868bbcc
```

Ravel made a directory under `TMPDIR` for each session. The inner session got
a new path, but only the inner fake agent received it. The outer fake agent
kept its own path. Thus, each `end-session` goes to the session that encloses
it. A nested session gives only its own child a new path.

The [Rhai worker](02-actors-and-words.md#evaluation-and-supervision)
never receives `RAVEL_EXIT_FILE`. The Rhai worker is the second copy of the
`ravel` executable that evaluates the configuration script. A harness run that
the script starts gets a path of its own, so its result goes back to the
script. What if `end-session` finds no path? Then it stops with status 1 and
writes a line on standard error.

Fragment: standard error of `ravel end-session changes-needed` in a shell
outside any session. This block omits the command and its status.

```text
ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or empty)
```

### Ravel stops the program and returns the result

When the program sends the result, it requests the end of the session. This
request does not prove that cleanup succeeded. A process group is an
operating-system unit that contains the managed process and the descendants
that remain in its group.
Ravel stops the harness and each other process that remains in that group.
Then it completes the cleanup of the group.

<!-- tc: depart synonym-rotation because confirmation establishes completed cleanup, while checks also cover input validation -->
To reap a child process, its parent collects the exit status of that child.
Before Ravel returns a successful result, it confirms that the managed
process ended and that Ravel reaped it. Ravel also confirms that the process
group is gone and that it removed the temporary files of the session.
[Chapter 3, “A complete session, end to end”](03-a-complete-session.md#launch-the-managed-job),
explains these mechanics at the point where a session needs them.

The table follows the action of the LLM to the result that the caller
receives. The caller receives a successful result only after Ravel stops the
managed process and its process group and completes the cleanup.

| Step after the LLM completes its task | Who acts and what happens |
|---|---|
| Send | The program that Ravel launched, usually a harness that runs an LLM, runs `ravel end-session RESULT`. This command writes the result to the path in `RAVEL_EXIT_FILE` and requests the end of the session. |
| Stop | Ravel stops that program and all other processes that still run in its process group. |
| Confirm cleanup | Ravel confirms that it reaped the managed process, that the group is gone and that it removed the temporary files of the session. |
| Return | Ravel writes the result unchanged on standard output. It becomes the result of `ravel start-session`, and the caller decides its next step. |

The figure shows Ravel between an orchestrator that requests work and a
harness that runs an LLM session. The owner chooses the harness. Ravel
supervises the harness until cleanup lets the result return. Other callers
and selected programs use the same boundary.

The C4 system-context view shows Ravel as one system. These shapes show
responsibilities, not security boundaries.

![The owner configures which harness runs the caller's request. An orchestrator or a person sends a kind and a prompt with start-session. Ravel launches and supervises the harness, or another selected program, as the managed process. The LLM tells the harness to run end-session, which sends an opaque result and asks Ravel to stop. Ravel stops the group members that remain. Ravel returns the result to the caller on standard output only after successful cleanup.](figures/system-context.svg)

[Open the system-context figure at full size.](figures/system-context.svg)

After a successful session, Ravel writes the result unchanged to standard
output. Ravel does not interpret the result and adds no newline. Ravel
redirects the standard output of the managed process to standard error.
Thus, the caller can keep the output of the managed process and the
diagnostics of Ravel separate from the result stream.

<!-- tc: depart synonym-rotation because checking an exit status and confirming completed cleanup are different operations -->
Check whether `start-session` succeeded before you interpret that stream. A
failure during the write of the final result can leave only the start of the
result on standard output. Thus, the output alone does not prove success.

Some outcomes return no result to the caller:

- A [refusal](02-actors-and-words.md#evaluation-and-supervision) means
  that Ravel launches no final managed process. Evaluation harness runs can
  occur before the configuration script refuses.
- If the managed process exits and no program sent a result, the session has
  no result and fails. This is also true when the exit status is zero.
- Cancellation of the session suppresses the result, even if a program sent
  the result before the cancellation.
- A supervision failure or a cleanup failure also prevents success.

But a session with a sent result can succeed after cleanup, even if the
managed process then exits with a nonzero status. The result is the value
that the caller's instructions to end the session asked for. The exit status
tells how the process ended. The work is what the program changed. None of
these three outputs can replace another.

The table keeps the three outputs separate. Thus, you do not confuse an exit
status or a changed file with the result that the caller requested.

| What the session produces | What it tells the caller | What it does not prove |
|---|---|---|
| Opaque result | The value sent with `end-session`, returned after successful supervision and cleanup | <!-- tc: depart synonym-rotation because "correct" describes the work, while `fix` is the literal kind of a later session --> That the requested work is correct |
| Managed process's exit status | How that process ended | That it sent a result or that the session succeeded |
| Work, such as changed files | What the selected program changed, such as `review-notes.md` | That it sent a result or that the cleanup completed |

In short, the caller asks with a kind and a prompt. The prompt gives the
permitted results and the `end-session` instruction. The owner's Rhai
configuration script selects a program, and the routing notice tells the
caller what it chose. Ravel launches the program and gives it a path in
`RAVEL_EXIT_FILE`. The program sends a result to that path with
`ravel end-session`. Ravel stops the program, completes cleanup and then
returns the result on standard output.

## Decide whether Ravel fits

Ravel fits when your program needs to use an LLM session in an interactive
harness as a step of its own work. Your program can let the owner's
configuration choose the harness. In each harness that the configuration
script selects, the LLM uses the same command, `end-session`, to send the
result. Ravel supervises the session and returns its explicit result after
cleanup. Your program still owns the task and the decisions after the session
ends.

Use this table to find which part of your need Ravel meets. The other parts
need a caller, a service or a selected program.

| If you need… | Ravel's part | What stays outside Ravel |
|---|---|---|
| Different owners to choose different harnesses for the same LLM task | Evaluate the configuration script of each owner and run the command that it selects | The caller's task meaning and the owner's decisions in the configuration |
| An interactive harness to finish as a step of your program | Accept the result that `end-session` sends, stop the group members that remain and return the result after successful cleanup | The caller's instructions to end the session, the interpretation of the result and all further work |
| Task queues, schedules, retries or durable history | Run a session when a caller invokes Ravel | A caller or service that stores and schedules work |
| Model access, credentials, usage measurement or rules for model selection | Supervise evaluation harness runs and the command that the configuration script selects | The harness's model integration, sign-in and usage reports, and the owner's selection rules |
| A host that continues to do other work after a launch | The [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) supervises one session and then exits | A separate host that continues to run after each launch, which Ravel does not provide |

The Ravel CLI, the `ravel` command-line program, is a **launcher CLI**. One
invocation supervises one managed launch, reports its result or failure, and
then exits. A **persistent runner** is a host that continues to run after a
launch is complete. Ravel does not provide one. A program that exits after one
session does not give what such a host needs.

Do you need a record of past sessions? Ravel keeps none. It has no persistent
session store and no history lookup. Its
[result channel](02-actors-and-words.md#the-managed-process-and-its-result),
the private temporary directory that receives the result, exists for one
session only. If a caller needs a permanent record, that caller saves it.

The configuration script has no direct filesystem, network or process access.
It asks Ravel to read files or run a harness from the settings through
[host capabilities](02-actors-and-words.md#evaluation-and-supervision),
the `ravel::` functions that Ravel performs for the script. That harness can
use files and network services within its own permissions.

[Grove](https://github.com/Linkuistics/grove), a task-tree driver, is one
illustrative caller. It is an orchestrator that runs LLM sessions in harnesses
as steps of the work on its task tree. It supplies its own task kinds and
interprets its own result values. These conventions belong to Grove. Another
caller can use different kinds and results without a Ravel adapter.

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

The supervision guarantee has limits. This table shows the consequence of
each limit for the caller.

| Condition | Limit of the guarantee |
|---|---|
| A descendant leaves the process group of the managed process | Group cleanup does not reach it. The session can succeed while that descendant continues to run. |
| An evaluation harness run starts descendants that leave its process group | Cleanup of that run does not reach those descendants. The [Rhai worker](02-actors-and-words.md#evaluation-and-supervision), the separate process that evaluates the configuration script, itself starts no program. |
| The Ravel CLI stops abruptly, for example because of KILL | The managed process or temporary files can remain. The Ravel CLI cannot catch KILL. |
| Other code runs as the same user as Ravel | That code can tamper with Ravel's files, `RAVEL_EXIT_FILE` included. The separation of sessions is not a security check against that code. |

[Chapter 2](02-actors-and-words.md#trust-limits) explains why the
separation of these roles is useful within these limits.

## Choose a route through the book

The book assumes everyday use of shell, Git and JSON, the ability to read
Rust, and general operating-system knowledge. It teaches the domain of Ravel
and the job-control details that Ravel needs. This chapter showed a short
Rhai configuration script. Chapter 4 teaches the Rhai syntax and host
capabilities that configuration scripts need.

Choose the route that serves your task. The table gives the chapter order
and the chapters for further detail.

| Your task | Read in this order | Further detail |
|---|---|---|
| Call Ravel from your own tool | Chapters 1–3, then [chapter 5, “Run sessions”](05-running-sessions.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Write a configuration script | Chapters 1–3, then [chapter 4, “Write a configuration script”](04-write-a-configuration-script.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Change Ravel | Chapters 1–3, then [chapter 7, “The parts of Ravel”](07-the-parts-of-ravel.md), [8, “Follow a session through the code”](08-follow-a-session-through-the-code.md), and [10, “Develop Ravel”](10-develop-ravel.md) | [Chapter 9, “Jobs, signals and the terminal”](09-jobs-signals-and-the-terminal.md), and [chapter 11, “Deliver Ravel”](11-deliver-ravel.md) |
| Reuse Ravel from another program | Ravel supports no separate component. Invoke the Ravel CLI as a child process. Chapters 2, 3 and [5](05-running-sessions.md) teach that interface. | [Chapters 7](07-the-parts-of-ravel.md) and [9](09-jobs-signals-and-the-terminal.md) explain the private runner and its limits |

Why can another program not use Ravel's parts as a library? Ravel's
[runner](02-actors-and-words.md#evaluation-and-supervision),
`keyed-launch`, is the Rust code that launches and supervises the managed
process. It is a private package of the Ravel CLI. The runner can change
process-wide signal state, and its return does not restore that state. Thus,
a program that needs supervised harness sessions invokes the Ravel CLI as a
child process. The Ravel CLI supervises one session and then exits.

The learning route is to read the chapters in order. The first practical
session, in chapter 3, uses a small configuration script and a fake agent.
Its directory is `~/ravel-first-session`, and the chapter gives each file and
command.

A fake agent is
an ordinary program that sends a result with `end-session`. It takes the
place of a harness that runs an LLM session. Thus, you can learn the protocol
without a model service or a personal agent account. The installation and
the configuration of real agents are outside the scope of the book.

## See what we tested and what remains unchecked

Which platforms did the project check, and what does that evidence show? This
section answers that question. It also says which checks of the book remain
open.

Recorded measurements checked how Ravel starts a session and completes its
cleanup on two platforms. One platform is aarch64 macOS with Rust 1.99. The
other platform is native aarch64 GNU/Linux with Rust 1.85.0, which was the
declared minimum at that time. Each measurement applies only to the platform,
kernel, C library and compiler that it records.

The project built the x86_64 GNU/Linux target, linted it at Rust 1.85.0 and
ran it only under QEMU user-mode emulation. This emulation translates x86-64
programs to run on the kernel of the host. Thus, no x86-64 kernel takes part.
No x86-64 kernel is qualified.

These facts are limits of the evidence. The three targets do not have the
same evidence. [Chapter 11](11-deliver-ravel.md#read-the-platform-checks-correctly) explains the limits of delivery and of the platforms
in detail.

The table shows why a built or emulated target does not have the same
evidence as a measured native target.

| Target | Recorded execution | Compiler | Limit |
|---|---|---|---|
| aarch64 macOS | Native measurements | Rust 1.99 | Only the recorded platform and compiler |
| aarch64 GNU/Linux | Native measurements | Rust 1.85.0, the earlier declared minimum | Only the recorded kernel, C library and compiler |
| x86_64 GNU/Linux | Built, linted and run under QEMU user-mode emulation | Rust 1.85.0 | Uses the kernel of the host. No x86-64 kernel is qualified. |

The reader checkpoints of the book use the owner as a labelled stand-in for a
developer new to Ravel. These reads can show the owner's findings, for
example errors in the book. They cannot show whether a new developer can use
the book.

You can build and examine the local book. GitHub shows the
Markdown of a pushed branch, and nobody checked that view of this copy. The
book of each release from Ravel 1.1.0 is on GitHub Pages. The book of a later version reaches
GitHub Pages only with its release. Thus, that copy stays unchecked until
then.
