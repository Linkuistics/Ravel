# What Ravel is for

Ravel runs
[LLM sessions in agent harnesses](02-actors-and-words.md#the-managed-process-and-its-result),
such as Claude Code and Codex, as supervised parts of another program. A
harness is the program in which an LLM works on a prompt. It gives the prompt
to the LLM and runs the commands that the LLM requests, for example shell
commands. The person who configures Ravel chooses which harness runs each
request. Ravel starts that harness and supervises it. After cleanup succeeds,
Ravel returns the result that the session sent.

Why use a harness, and not a direct call to a model? A harness such as Claude
Code or Codex can run under the subscription of its user. Thus, your program
can use that subscription for LLM work instead of paid API access. The cost
can be much lower for work that the subscription covers. The
[Claude Code documentation](https://code.claude.com/docs/en/authentication)
and [Codex documentation](https://learn.chatgpt.com/docs/auth) explain their
access options and link to their plans and terms. This paragraph agrees with
those pages as they were on 18 October 2026.

An [orchestrator](02-actors-and-words.md#caller-and-owner) is a program that
runs LLM sessions as steps of its own work. It can request a
[session](02-actors-and-words.md#caller-and-owner) from Ravel and wait for
its result. A session is one run of `ravel start-session`, from the request
to the result. Then the orchestrator uses that result to decide its next step.

For example, an orchestrator can ask an LLM to review a code change. The
orchestrator knows the files to review and what to do with the answer. The
person who operates the orchestrator wants to choose the harness for their
account. Each participant has a different responsibility in that review:

| Participant | Responsibility in the review |
|---|---|
| Orchestrator | Choose the work to review and interpret the result |
| Person who configures Ravel | Choose the harness command through their policy |
| Ravel | Run the selected command and supervise it until cleanup completes |

An interactive harness waits for more input when the LLM completes its task.
It does not exit by itself. Then how does the orchestrator know that the work
is complete? Ravel's `end-session` command gives the answer. The LLM runs
`ravel end-session` with a result. Ravel then stops the harness, and after
cleanup the orchestrator receives the result and continues.

## From a request to a result

This section follows one request through Ravel. It names each actor in the
order in which that actor acts. Each subsection below explains one row of this
table, with a real example.

<!-- tc: depart synonym-rotation because launch names process creation, while start also names the beginning of output -->
| Order | Who acts | What happens |
|---|---|---|
| 1 | The caller | Runs `ravel start-session` with a kind and a prompt |
| 2 | The owner policy | Selects a program and its arguments, or refuses the request |
| 3 | Ravel | Writes the routing notice, which tells the caller what the policy chose |
| 4 | Ravel | Launches the selected program and supervises it |
| 5 | The selected program | Sends a result with `ravel end-session` |
| 6 | Ravel | Stops the program, completes cleanup and returns the result to the caller |

### The caller asks for a session

The person or program that makes the request is the
[caller](02-actors-and-words.md#caller-and-owner). An orchestrator is one
caller, and you are the caller when you type the command yourself. The caller
runs `ravel start-session` with a prompt and a
[kind](02-actors-and-words.md#caller-and-owner). The prompt is the text that
tells the LLM what to do. A kind is a short token that names the sort of work
that the caller wants.

In the review example above, the caller can use `review` as the kind. The
caller and the owner policy agree on that token. Ravel checks only that the
kind is not empty. It has no built-in review category and gives the kind no
meaning.

Fragment: the request of the review example. This block omits the program
that runs the command and the policy that Ravel uses.

```sh
ravel start-session --kind review --prompt 'Review the last commit. Then run ravel end-session continue.'
```

<!-- tc: depart synonym-rotation because the sentence keeps the owner's word "modify" from finding F3 -->
The caller can also use the kind to extend or modify the prompt. For example,
if the harness supports skills, the caller can add a line to the prompt for
the kind `review`. That line tells the LLM which skill to load. A skill is a
set of instructions for a particular task. This use of the kind is a caller
convention. Ravel gives the prompt to the policy unchanged and adds no task
instructions.

### The owner policy chooses a program

<!-- tc: depart synonym-rotation because launch names process creation, while start also names the beginning of output -->
The [owner](02-actors-and-words.md#caller-and-owner) is the person who
controls the configuration of Ravel for the account. The owner can be you.
Their [owner policy](02-actors-and-words.md#caller-and-owner) is a file of
Rhai code. Its default path is `$HOME/.config/ravel/policy.rhai`. Ravel gives
the request to the policy. Then the policy selects a program and its
arguments, or it refuses to launch a program.

What is Rhai? Rhai is a small computer language for programs that are
written in Rust. A Rust program can contain the Rhai engine and run Rhai code
in its own process. Ravel does this. Thus, the `ravel` executable already
contains everything that runs your policy, and you install no separate
language runtime.

[Rhai's feature list](https://rhai.rs/book/about/features.html) names these
characteristics. Each one is important for a policy:

- **A familiar syntax.** Rhai looks like a mix of JavaScript and Rust. It has
  variables, functions, `if`, loops, arrays and maps.
- **Dynamic types.** You do not declare a type. Each value has its type while
  the policy runs.
- **A sandbox.** Rhai code changes things outside the script only through
  functions that the host program gives it. In Ravel, the policy reads files
  and runs harnesses only through `ravel::` functions. The book calls these
  functions
  [host capabilities](02-actors-and-words.md#policy-evaluation-and-supervision).
- **Protection against runaway scripts.** Rhai can stop a script that runs
  too long, calls functions too deeply or makes data that is too large. Ravel
  sets these limits, and one deadline covers each evaluation.
- **Modules.** A policy can import other Rhai files.

Here is a short policy for the review example. It sends each `review` request
to a program and refuses every other kind.

Fragment: a short owner policy for the review example. This block omits the
selected program `fake-agent` and the command that runs the policy.
[Chapter 3](03-a-complete-session.md#run-the-first-session) runs a policy of
this sort with a fake agent.

```rhai
export const SCHEMA_VERSION = 3;
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
        message: "This policy has no rule for the kind " + request.kind + ".",
        remedy: "Use the kind review.",
    }
}
```

Read the policy from the top. The table says what each part does:

| Part of the policy | What it does |
|---|---|
| `export const SCHEMA_VERSION = 3;` | Names the version of Ravel's policy interface that this file uses |
| `export const VERSION = "chapter-1";` | Names this revision of the owner's policy |
| `fn select(request, context)` | Defines the function that Ravel calls. `request` holds the kind, the prompt and other data of the request. `context` holds optional policy context, which is data for the choice. |
| `if request.kind == "review"` | Applies the first rule only to the kind `review` |
| `#{ ... }` | Makes a map, which is a set of named fields |
| `return` | Ends the function with the map that follows it |
| `"You are a careful code reviewer. " + request.prompt` | Puts an instruction before the caller's prompt |
| The last map | Is the value of the function when no `return` ran. This map is a refusal. |

Do you want to learn more Rhai? Rhai has its own documentation. Chapter 4
teaches the Rhai that a policy needs, so you do not need these pages for this
book:

- [The Rhai Book](https://rhai.rs/book/) is the manual. On 27 October 2026, it
  documented Rhai 1.27.0, the version that Ravel contains.
- [The language reference](https://rhai.rs/book/ref/) gives the syntax in a
  short form.
- [The online playground](https://rhai.rs/playground/stable/) runs Rhai code
  in your web browser. It has no `ravel::` functions and none of Ravel's
  limits. Thus, a policy that calls a host capability does not run there.

### The selection and the routing notice

What does the policy give back to Ravel? It gives back a selection or a
refusal. A **selection** is the map that `select` returns when it chooses a
program. Its `status` field is `"selected"`. A
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision) is a map
with `status: "refused"`. After a refusal, Ravel launches nothing and returns
no result to the caller.

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
labels. A label describes the choice of the policy, but it does not configure
the harness. For example, a policy that selects Claude Code puts the model
option of Claude Code in `args`. The `model` label only tells the caller
which model the policy chose. Ravel does not check that the labels agree with
the arguments.

Why does the caller need these labels? The caller usually wants to know what
the policy chose, but the caller does not read the owner's policy. Ravel
tells the caller in the **routing notice**:

- **Who writes it:** the
  [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision), the
  `ravel` command-line program.
- **When:** after the policy selects a program, and before Ravel launches it.
- **Where:** one line on the caller's standard error. Standard output carries
  only the result.
- **Who reads it:** the caller, or the person who watches the terminal.
- **Why:** this line is the way in which Ravel tells the caller which harness,
  model and effort the policy chose.

Fragment: the routing notice of the review example, as Ravel writes it on
standard error. This block omits the other lines on standard error.

```text
ravel: starting provider local, model fake, effort low for kind "review"
```

The selection separates the command that Ravel launches from the labels that the caller reads.

![The policy worker returns the fake-agent command and the local, fake and low labels. The Ravel CLI reaps the worker, writes the routing notice on the caller's standard error and launches fake-agent with args. Labels do not go to the program. With JSON, the notice follows the supervision attempt.](figures/routing-notice.svg)

*Figure: The review selection reaches the program and the caller by separate paths.*

[Open the selection and routing notice at full size](figures/routing-notice.svg).

With `--json`, Ravel writes a version-2 JSON document in place of that line.
The document gives more detail, such as the `reason` and the program file
that Ravel found. Ravel writes this document after the supervision attempt,
not before the launch.

Fragment: the `selection` member of the JSON routing notice of the same
session. This block omits the other members of the document.

```json
{"selection":{"effort":"low","model":"fake","provider":"local","reason":"Reviews go to the fake agent."}}
```

[Chapter 5, “Read notices separately from results”](05-running-sessions.md#read-notices-separately-from-results),
teaches both forms of the routing notice.

### An LLM can help the policy choose

You may want an LLM to choose the best harness, model, effort and prompt
additions for a kind and a prompt. You can do this with a
[harness run](02-actors-and-words.md#policy-evaluation-and-supervision) in
your policy. A harness run is one run of a program from the owner's
`harnesses` settings. The policy starts it with the host capability
`ravel::run_harness`. The harness uses the owner's sign-in, and Ravel holds
no model credential.

The policy reads the answer of the harness as data. Then it builds its
selection from that answer. A harness run does not complete the session. Its
result goes back to the policy, not to the caller. Ravel supervises each
harness run within the owner's
[evaluation budget](02-actors-and-words.md#policy-evaluation-and-supervision),
which limits the work of one policy evaluation.
[Chapter 4](04-writing-an-owner-policy.md) gives the recipes for Claude Code
and Codex.

### Ravel runs the program, and the program sends a result

One `start-session` invocation is a
[session](02-actors-and-words.md#caller-and-owner). During the session, Ravel
supervises the program that the policy selected. This program is the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)
of the session. The harness runs the LLM session that works on the prompt.
That LLM session is not the Ravel session. Ravel supervises the harness
process, not the conversation of the LLM in that process.

The contract with the selected program is loose. The policy selects a
program, Ravel runs it, and a program in the session sends one value with
`end-session`. Ravel needs nothing more from the program. The selected
program does not have to use an LLM or understand a prompt.

The table separates what Ravel needs from the harness use that this book
teaches. The runnable examples use ordinary programs as fake agents.

| Part of a session | What Ravel needs | What this book expects from a harness |
|---|---|---|
| Selected command | The policy selects a program and its arguments | The program runs an LLM session |
| Prompt | The caller supplies `--prompt` or `--prompt-file`. Ravel gives that text to the policy unchanged. The policy decides whether the program receives it. | The policy passes the prompt to the harness |
| Result | A program in the session sends one value with `end-session` | The LLM follows the caller's instructions and tells the harness to run that command |

An interactive harness does not exit when its LLM completes the task. It
waits for more input. Thus, the caller's prompt tells the LLM to run
`ravel end-session RESULT` when the task is complete. `RESULT` is a
placeholder for the text to send. The caller's prompt gives the permitted
values and tells the LLM when to use each value.

For example, the caller can choose the text `continue` as a result value. The
prompt of the review example tells the LLM to run `ravel end-session continue`
when the review is complete. The program that Ravel launched, usually a
harness that runs an LLM, then runs that command. Ravel stops that program
and the rest of its process group. After cleanup, `continue` becomes the
result of `ravel start-session`.

Ravel writes `continue` unchanged on the caller's standard output. The caller
can interpret `continue` as permission to start its next task. Ravel gives
`continue` no meaning and does not start a next task. The book calls this
text an
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
| `ravel end-session continue` | That shell | The same path, inherited from the shell. The command writes the result there. |

Each child inherits its parent's path unless Ravel sets a new path for the inner harness.

![A caller starts outer start-session without a result path, and its policy worker has no path. Ravel gives the outer harness an outer path, which its shell and end-session inherit. The shell starts inner start-session with the outer path. The inner harness receives a new inner path, which its end-session inherits. Each end-session writes to its own path. An evaluation harness run gets a separate path and returns its result to the policy.](figures/inherited-result-path.svg)

*Figure: The nested run gives each session's children its own result path.*

[Open the inherited result paths at full size](figures/inherited-result-path.svg).

Here is a real run. A fake agent printed its own `RAVEL_EXIT_FILE`. Then it
was the caller of a second, inner session. The inner fake agent printed its
value, and the outer fake agent printed its value again.

Fragment: standard error of a real nested session on macOS with
`TMPDIR=/tmp`. This block omits the cleanup reports. The random parts of each
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

The [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)
never receives `RAVEL_EXIT_FILE`. The policy worker is the separate process
that runs the owner policy. A harness run that the policy starts gets a path
of its own, so its result goes back to the policy. What if `end-session`
finds no path? Then it stops with status 1 and writes a line on standard
error.

Fragment: standard error of `ravel end-session continue` in a shell outside
any session. This block omits the command and its status.

```text
ravel: end-session: no enclosing Ravel session (RAVEL_EXIT_FILE is unset or empty)
```

### Ravel stops the program and returns the result

When the program sends the result, it requests the end of the session. This
request does not prove that cleanup succeeded. A process group is an
operating-system unit that contains the managed process and its descendants.
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

- A [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) means
  that Ravel launches no final managed process. Evaluation harness runs can
  occur before the policy refuses.
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
| Opaque result | The value sent with `end-session`, returned after successful supervision and cleanup | That the requested work is correct |
| Managed process's exit status | How that process ended | That it sent a result or that the session succeeded |
| Work, such as changed files | What the selected program changed | That it sent a result or that the cleanup completed |

In short, the caller asks with a kind and a prompt. The owner's Rhai policy
selects a program, and the routing notice tells the caller what it chose.
Ravel launches the program and gives it a path in `RAVEL_EXIT_FILE`. The
program sends a result to that path with `ravel end-session`. Ravel stops the
program, completes cleanup and then returns the result on standard output.

## Decide whether Ravel fits

Ravel fits when your program needs to use an LLM session in an interactive
harness as a step of its own work. Your program can let the owner policy of the
account choose the harness. In each harness that the policy selects, the LLM
uses the same command, `end-session`, to send the result. Ravel supervises the
session and returns its explicit result after cleanup. Your program still owns
the task and the decisions after the session ends.

Use this table to find which part of your need Ravel meets. The other parts
need a caller, a service or a selected program.

| If you need… | Ravel's part | What stays outside Ravel |
|---|---|---|
| Different owners to choose different harnesses for the same LLM task | Evaluate the policy of each owner and run the command that it selects | The caller's task meaning and the owner's policy decisions |
| An interactive harness to finish as a step of your program | Accept the result that `end-session` sends, stop the group members that remain and return the result after successful cleanup | The caller's instructions to end the session, the interpretation of the result and all further work |
| Task queues, schedules, retries or durable history | Run a session when a caller invokes Ravel | A caller or service that stores and schedules work |
| Model access, credentials, usage measurement or rules for model selection | Supervise evaluation harness runs and the command that the policy selects | The harness's model integration, sign-in and usage reports, and the owner's selection rules |
| A host that continues to do other work after a launch | The [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision) supervises one session and then exits | A separate host that continues to run after each launch, which Ravel does not provide |

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

The owner policy has no direct filesystem, network or process access. It
asks Ravel to read files or run an owner-configured harness through
[host capabilities](02-actors-and-words.md#policy-evaluation-and-supervision),
the `ravel::` functions that Ravel performs for the policy. That harness can
use files and network services within its own permissions.

Grove, a task-tree driver, is one illustrative caller. It is an orchestrator
that runs LLM sessions in harnesses as steps of the work on its task tree. It
supplies its own task kinds and interprets its own result values. These
conventions belong to Grove. Another caller can use different kinds and
results without a Ravel adapter.

This chapter writes INT, TERM, HUP and KILL without the SIG prefix.
Other signal names keep that prefix. Code and diagnostic text keep their literal names.

The supervision guarantee has limits. This table shows the consequence of
each limit for the caller.

| Condition | Limit of the guarantee |
|---|---|
| A descendant leaves the process group of the managed process | Group cleanup does not reach it. The session can succeed while that descendant continues to run. |
| An evaluation harness run starts descendants that leave its process group | Cleanup of that run does not reach those descendants. The [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision), the separate process that runs the policy, itself starts no program. |
| The Ravel CLI stops abruptly, for example because of KILL | The managed process or temporary files can remain. The Ravel CLI cannot catch KILL. |
| Other code runs as the same user as Ravel | That code can tamper with Ravel's files, `RAVEL_EXIT_FILE` included. The separation of sessions is not a security check against that code. |

[Chapter 2](02-actors-and-words.md#trust-and-confinement) explains why the
separation of these roles is useful within these limits.

## Choose a route through the book

The book assumes everyday use of shell, Git and JSON, the ability to read
Rust, and general operating-system knowledge. It teaches the domain of Ravel
and the job-control details that Ravel needs. This chapter showed a short
Rhai policy. Chapter 4 teaches the Rhai syntax and host capabilities that
policies need.

Choose the route that serves your task. The table gives the chapter order
and the chapters for further detail.

| Your task | Read in this order | Further detail |
|---|---|---|
| Call Ravel from your own tool | Chapters 1–3, then [chapter 5, “Run sessions”](05-running-sessions.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Write an owner policy | Chapters 1–3, then [chapter 4, “Write an owner policy”](04-writing-an-owner-policy.md) | [Chapter 6, “Reference”](06-reference.md), for lookup |
| Change Ravel | Chapters 1–3, then [chapter 7, “The parts of Ravel”](07-the-parts-of-ravel.md), [8, “Follow a session through the code”](08-follow-a-session-through-the-code.md), and [10, “Develop Ravel”](10-develop-ravel.md) | [Chapter 9, “Jobs, signals and the terminal”](09-jobs-signals-and-the-terminal.md), and [chapter 11, “Deliver Ravel”](11-deliver-ravel.md) |
| Reuse Ravel from another program | Ravel supports no separate component. Invoke the Ravel CLI as a child process. Chapters 2, 3 and [5](05-running-sessions.md) teach that interface. | [Chapters 7](07-the-parts-of-ravel.md) and [9](09-jobs-signals-and-the-terminal.md) explain the private runner and its limits |

Why can another program not use Ravel's parts as a library? Ravel's
[runner](02-actors-and-words.md#policy-evaluation-and-supervision),
`keyed-launch`, is the Rust code that launches and supervises the managed
process. It is a private package of the Ravel CLI. The runner can change
process-wide signal state, and its return does not restore that state. Thus,
a program that needs supervised harness sessions invokes the Ravel CLI as a
child process. The Ravel CLI supervises one session and then exits.

The learning route is to read the chapters in order. The first practical
session uses a small policy and a fake agent. A fake agent is an ordinary
program that sends a result with `end-session`. It takes the place of a
harness that runs an LLM session. Thus, you can learn the protocol without a
model service or a personal agent account. The installation and the
configuration of real agents are outside the scope of the book.

## Understand the evidence limits

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
the book. You can build and examine the local book. Nobody can check how GitHub shows the
book until the project pushes the branch. The book on GitHub Pages stays
unchecked until the first release that contains the book.
