# Write a configuration script

In chapter 1, an orchestrator asked Ravel for a code review with the kind
`review`. A short Rhai script chose the program for that kind, and Ravel ran
it. You write that script. This chapter teaches you to write and check a
configuration script, and to let an LLM help it choose.

First, the words that this chapter uses on every page:

| Word | What it names |
|---|---|
| [Session](02-actors-and-words.md#caller-and-owner) | One run of `ravel start-session`, from the request to the result |
| [Caller](02-actors-and-words.md#caller-and-owner) | The person or program that runs that command, for example an orchestrator. The caller gives a prompt and a kind. |
| [Kind](02-actors-and-words.md#caller-and-owner) | A short token from the caller that names the sort of work, such as `review` |
| [Owner](02-actors-and-words.md#caller-and-owner) | The person who controls the configuration of Ravel for the account. In this chapter, the owner is you. |
| [Configuration script](02-actors-and-words.md#caller-and-owner) | A file of Rhai code that you write. Ravel gives the request to the script. The script then selects a program and its arguments, or it refuses the request. |
| [Settings](02-actors-and-words.md#caller-and-owner) | Values in the JSON file `$HOME/.config/ravel/settings.json`, such as limits and the harnesses that the script can run. The script cannot change them. |
| **Configuration** | The configuration script and the settings together |

Ravel uses configuration names in its file paths, options and diagnostic codes,
for example `config.rhai`, `--config-env` and `config_refused`.

## Know the Rhai language

What is Rhai? Rhai is a small computer language for programs in the Rust
language. The Rhai engine is part of the `ravel` executable. Thus, you install
no separate language runtime for your configuration script.
[Chapter 1](01-what-ravel-is-for.md#the-configuration-script-chooses-a-program)
introduces Rhai with a short script. These characteristics of Rhai are
important for a configuration script:

- **A familiar syntax.** Rhai looks like a mix of JavaScript and Rust. It has
  variables, functions, `if`, loops, arrays and maps.
- **Dynamic types.** You do not declare a type. Each value has its type while
  the script runs.
- **A sandbox.** Rhai code changes things outside the script only through
  functions that the host program gives it. In Ravel, these are the `ravel::`
  functions.
- **Protection against runaway scripts.** Ravel sets limits on the operations,
  the call depth and the data size of a script. One deadline covers each
  evaluation.
- **Modules.** A configuration script can import other Rhai files.

Each example explains the Rhai that it uses. Rhai also has its own
documentation:

- [The Rhai Book](https://rhai.rs/book/) is the manual. On 27 October 2026, it
  documented Rhai 1.27.0, the version that Ravel contains.
- [The language reference](https://rhai.rs/book/ref/) gives the syntax in a
  short form.
- [The online playground](https://rhai.rs/playground/stable/) runs Rhai code in
  your web browser. It has no `ravel::` functions and none of Ravel's limits.
  Thus, a script that calls a `ravel::` function does not run there.

The sections of this chapter go from a fixed route to an LLM that helps the
configuration script choose. Read them in order, or go to the task that you
need:

| Section | Why you may want it |
|---|---|
| [Define a configuration script and read the request](#define-a-configuration-script-and-read-the-request) | Make Ravel find your script, and read what the caller asked for |
| [Select a command or refuse](#select-a-command-or-refuse) | Choose the program and its arguments, or refuse with a message that the caller can act on |
| [Validate a configuration script with fake agents](#validate-a-configuration-script-with-fake-agents) <!-- tc: depart synonym-rotation because validate names the end-to-end test of a script, while check names a single diagnostic inspection --> | Test each route and each refusal before a real harness runs |
| [Put each kind and its route in one table](#put-each-kind-and-its-route-in-one-table) | See each kind and its route in one place |
| [Import other Rhai files](#import-other-rhai-files) | Keep routes or helper functions in separate files |
| [Let a repository choose an arrangement](#let-a-repository-choose-an-arrangement) | Let a repository ask for one of the arrangements that you offer |
| [Supply selection context](#supply-selection-context) | Let facts about the task change the choice |
| [Check JSON with the data helpers](#check-json-with-the-data-helpers) | Reject JSON from a file or a model when it has the wrong shape |
| [Set bounds and environment grants in the settings](#set-bounds-and-environment-grants-in-the-settings) | Give the script more time, or let it read an environment variable |
| [Run a harness during evaluation](#run-a-harness-during-evaluation) | Ask another program a question before you choose |
| [Ask a model through Claude Code](#ask-a-model-through-claude-code) | Let an LLM in Claude Code help with the choice |
| [Ask a model through Codex](#ask-a-model-through-codex) | Let an LLM in Codex help with the choice |
| [Select Claude Code or Codex for the caller's work](#select-claude-code-or-codex-for-the-callers-work) | Run an interactive LLM session as the program of the session |
| [Copy a configuration script and its files](#copy-a-configuration-script-and-its-files) | Move a script and its files to another place |

## Follow the evaluation of a configuration script

What happens between the caller's command and the start of a program? Start
with the review of chapter 1. The orchestrator ran the command below.

Fragment: the request of the review example in chapter 1. This block omits
the code of the orchestrator that runs the command and the prompt file.

```sh
ravel start-session --kind review --prompt-file review-prompt.txt
```

The configuration script of that chapter had one rule. For the kind `review`,
it selected `fake-agent` and put `You are a careful code reviewer.` before the
caller's prompt. The table follows that request from the command to the start
of `fake-agent`. In the table, the **Rhai worker** is a second copy of the
`ravel` executable that evaluates the script. This section explains it after
the table.

| Order | Who acts | What happens in the review |
|---|---|---|
| 1 | The orchestrator | Runs `ravel start-session` with the kind `review` and the prompt file |
| 2 | The Ravel CLI | Reads the settings, finds the configuration script and starts the Rhai worker |
| 3 | The Rhai worker | Runs the top-level statements of the script one time. Here, they are only the two `export const` lines. |
| 4 | The Rhai worker | Calls `select(request, context)`. `request.kind` is `review`, so `select` returns a selection of `fake-agent`. |
| 5 | The Ravel CLI | Reaps the worker, then checks the map that `select` returned |
| 6 | The Ravel CLI | Writes `ravel: starting provider local, model fake, effort low for kind "review"` on the orchestrator's standard error, then starts `fake-agent` |

A successful selection follows this order. The values depend on the request
and the script. A script can also define `load_context(request)`, which the
worker calls between steps 3 and 4. The section
[“Supply selection context”](#supply-selection-context) teaches it. With
`--json`, the routing notice is a JSON document, and it follows the
supervision attempt.

To reap a child process, its parent collects the exit status of that child.
Ravel reaps the worker before it starts the selected program. Thus, the Rhai
worker no longer runs when the selected program starts.

A **selection** is the map that `select` returns when it chooses a program.
In the review, it held `fake-agent`, one argument and the labels `local`,
`fake` and `low` for the provider, model and effort. A
[refusal](02-actors-and-words.md#evaluation-and-supervision) is a map that
says why Ravel starts no final managed process. The **routing notice** is the
line in step 6. It tells the caller which provider, model and effort the
configuration script chose.
[Chapter 1, “The selection and the routing notice”](01-what-ravel-is-for.md#the-selection-and-the-routing-notice),
shows both with real values.

What is the Rhai worker? The
[Ravel CLI](02-actors-and-words.md#evaluation-and-supervision), the `ravel`
command-line program, does not run your script in its own process. It
starts a second copy of its own executable as a child process. That child
is the [Rhai worker](02-actors-and-words.md#evaluation-and-supervision). The
worker evaluates the configuration script and sends the selection or the
refusal to its parent.

The worker has an empty environment. It has no direct access to files, the
network or other programs.

Why a separate process? The parent stays in control while the script runs:

| Reason | What the separate process permits |
|---|---|
| The deadline | When the deadline expires, the parent ends evaluation and can signal the worker process. Rhai cannot interrupt some of its own functions, such as `sleep`. A kernel stall can delay the worker's exit. |
| Requests from the script | The worker only computes. When the script reads a file or runs a harness, the worker asks the parent. The parent does the work and sends the data back. |
| Isolation of a failure | If the script exhausts the memory of the worker or overflows its stack, only the worker stops. The parent continues, reports the failure to the caller and starts no program. |

The worker does not run at the same time as the selected program. Ravel waits
until the worker exits and collects its exit status. Only then does Ravel
start the program that the script selected.

How does the script read a file or start a program, if the worker cannot? It
asks the Ravel CLI. A
[host capability](02-actors-and-words.md#evaluation-and-supervision) is an
operation that the configuration script asks Ravel to perform. Each host
capability is a function in the `ravel` namespace, such as `ravel::read_text`.

When the script calls a host capability, the worker sends the request to the
Ravel CLI over a private socket and waits. The Ravel CLI does the work within
the limits of your settings and sends the answer back. For the script, the
call looks like an ordinary function call. The data helpers, such as
`ravel::json_parse`, are different. They run inside the worker and send no
request.

The script can call host capabilities at the top level of the module, in
`load_context` and in `select`. A **measured read** is a read of a file through
`ravel::read_text` or `ravel::read_json`. Ravel reads the file for the script
and records its canonical path, its byte count and its SHA-256 digest. The JSON
routing notice lists each measured read and each harness run of the
evaluation.

A [harness](02-actors-and-words.md#the-managed-process-and-its-result) is the
program in which an LLM works on a prompt, such as Claude Code or Codex. Your
configuration script can use a harness in two different ways:

| Use of the harness | When it runs | Where its answer goes |
|---|---|---|
| A [harness run](02-actors-and-words.md#evaluation-and-supervision) | During evaluation, when the script calls `ravel::run_harness` | Back to the script, as data for its choice |
| The [managed process](02-actors-and-words.md#the-managed-process-and-its-result) | After evaluation, when the script selects the harness | To the caller, as the result of the session |

The managed process is the program that Ravel starts and supervises for the
session. Ravel starts it only after the evaluation ends and Ravel reaps the
worker.

## Run the examples of this chapter

Every complete example in this chapter uses fake programs. A **fake agent** is
a small shell script that stands where a harness stands. Most fake agents here
run `ravel end-session` with the arguments that the configuration script gave
them. Thus, the result of an example shows you what the script chose. The fake
harnesses for an LLM call print a fixed answer. No example needs a model
account or a real harness.

Each example has a name and a directory of its own. Put these directories in
one workspace directory, `~/ravel-chapter-4`, in your home directory. Why a
new directory for each example? Several examples use the same
file names, such as `config.rhai` and `bin/fake-agent`, with different
contents. Some commands also write files, such as `.ravel-choice`, that
change the result of the next command.

The first example is `fixed-route`. Its configuration script selects one fake
agent for every request. The fake sends the caller's prompt back as the
result. When you finish the steps, the directory holds these paths:

```text
~/ravel-chapter-4/
└── fixed-route/
    ├── config.rhai        you save it
    ├── bin/               you make it
    │   └── fake-agent     you save it
    ├── home/              the first command line makes it
    └── tmp/               the first command line makes it
```

Each path has one purpose in the example:

| Path in `fixed-route` | Who makes it | Purpose |
|---|---|---|
| `config.rhai` | You | The configuration script. The option `--config config.rhai` selects it. |
| `bin/fake-agent` | You | The fake agent that the configuration script selects. It sends its first argument as the result. |
| `home/` | The first command line | An empty home directory for Ravel. Ravel finds no settings file there, so the default settings apply. |
| `tmp/` | The first command line | A private temporary directory. Ravel makes the result channel of the session here. |

Do these steps in a shell where `ravel --version` succeeds:

1. Make the example directory and its `bin` subdirectory, then `cd` into the
   example directory. The fragment after these steps gives the two commands.
2. Save the two files of the complete example at their labelled paths. Each
   label is a path in `fixed-route`.
3. Type each command line of the example without its first `$ `, which
   stands for the prompt of your shell.
4. Run `echo "$?"` immediately after the second line to see its exit status.
   Compare the output and the status with the example.

Fragment: make the directory of `fixed-route` and `cd` into it. This block
omits the two files, which you save next. The book checker makes its own
directory for each example and does not run this block.

```sh
mkdir -p ~/ravel-chapter-4/fixed-route/bin
cd ~/ravel-chapter-4/fixed-route
```

The `~` stands for your home directory, so these commands work from any
current directory.

Complete example: pass the caller's prompt to a fake agent.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "fixed-route-1";

fn select(request, context) {
    #{
        status: "selected",
        program: "fake-agent",
        args: [request.prompt],
        provider: "example",
        model: "fake",
        effort: "low",
        reason: "one fixed route",
    }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "Explain this change."
```

**Standard output, without a final newline**

```text
Explain this change.
```

**Exit status**

```text
0
```

In your terminal, standard error also shows the routing notice and can show
reports of cleanup operations. The book makes no claim about standard error
in this example.

What did the two command lines do? The first line prepares the directory. The
second line runs the session. These are their parts:

| Part of the command | What it does in `fixed-route` |
|---|---|
| `mkdir -p home tmp` | Makes `home` and `tmp`. With `-p`, `mkdir` gives no error when they exist already. |
| `chmod +x bin/*` | Makes each file in `bin`, here only `fake-agent`, executable |
| `env -i` | Runs the rest of the line with an empty environment. Your own variables, such as a `RAVEL_EXIT_FILE` of an enclosing session, do not enter the example. |
| `HOME="$PWD/home"` and `TMPDIR="$PWD/tmp"` | Give Ravel the private directories. Your shell expands `$PWD` before `env` runs, so each value is an absolute path. |
| `PATH="$PWD/bin:$PATH"` | Puts `bin` first on the search path. Your shell expands `$PATH` before `env -i` clears the environment, so your installed `ravel` stays on the path. |
| `ravel start-session` | Asks Ravel for one session |
| `--config config.rhai` | Selects the configuration script of the example, relative to the current directory |
| `--kind question` | Gives the caller's kind. This configuration script does not examine it. |
| `--prompt "Explain this change."` | Gives the caller's prompt. The script passes it to the fake as its first argument. |

The private home and the explicit `--config` keep your own settings and
configuration script out of the example.

What happened in the session? Follow the request from the caller to the
result:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | The caller | Runs `ravel start-session` with the kind `question` and the prompt `Explain this change.` |
| 2 | The configuration script | `select` returns a selection with the program `fake-agent` and one argument, the prompt |
| 3 | The Ravel CLI | Writes the routing notice `ravel: starting provider example, model fake, effort low for kind "question"` on standard error, then starts `fake-agent` |
| 4 | `fake-agent` | Runs `ravel end-session -- "$1"`, which sends its first argument as the result |
| 5 | The Ravel CLI | Stops `fake-agent` and the rest of its process group, then completes cleanup |
| 6 | The Ravel CLI | Writes `Explain this change.` on standard output. This text is the result of `ravel start-session`. |

A process group is the operating-system unit that holds the program and the
descendants that remain in its group. Ravel stops each process in that group
before it returns the result.

The `--` in `ravel end-session -- "$1"` ends the options of `end-session`.
Thus, a result that starts with `-` stays a result and does not become an
option. Ravel writes the result unchanged and adds no newline. This is why the
label says “without a final newline”, and why your next shell prompt can start
on the same line as the result.

Each later example of this chapter uses the same four steps in a new
directory under `~/ravel-chapter-4`. For example, step 1 of the next example,
`refusals`, is `mkdir -p ~/ravel-chapter-4/refusals/bin`, then
`cd ~/ravel-chapter-4/refusals`. Before its complete example, a paragraph
names the directory and says how the example is different from `fixed-route`.
An example can have more files, more command lines or different options. The
first line, `mkdir -p home tmp && chmod +x bin/*`, appears only in the first
command block of each example. A later block uses the directories that the
first block made.

The book checker runs the same files and commands with the built Ravel CLI.
For each example, it makes a new directory, saves the labelled files and makes
the files in `bin` executable. Then it runs the command blocks in order and
compares each output with the output that the book shows. You do the
preparation by hand, in the steps above.

## Define a configuration script and read the request

The `config.rhai` of `fixed-route` has two `export const` lines and one
function. What must a configuration script contain, and where does Ravel find
it? Ravel looks in one place. The default script is
`$HOME/.config/ravel/config.rhai`. `HOME` must be absolute for that default.

`--config PATH` selects another file, relative to the caller's working directory.
Ravel searches no parent directory and tries no fallback script.
Thus, a missing script is an error.
It is not a silent change of route.

The file must be readable UTF-8, regular, and at most 1 MiB.
A symlink identifies its canonical real file.

A configuration script is a Rhai module. A module is one file of Rhai code,
which can export constants for other code to read. Your script exports two
constants and defines a function:

| Declaration | Purpose |
|---|---|
| `export const SCHEMA_VERSION = 4;` | Name the version of the interface between Ravel and the script that this file uses. |
| `export const VERSION = "fixed-route-1";` | Name this revision of your script. The JSON routing notice shows it. |
| `fn select(request, context)` | Return a selection or a [refusal](02-actors-and-words.md#evaluation-and-supervision). |
| `fn load_context(request)` | Optionally assemble selection context before `select`. |

`SCHEMA_VERSION` is independent of the Ravel release and of the version of
the context document. `VERSION` is a nonblank string that you change when your
script changes. The functions need the parameter counts shown above.
Top-level statements run once before the functions.

The `select` function of `fixed-route` used this Rhai:

- `#{ ... }` makes a map, which is a set of named fields.
- `request.prompt` reads the field `prompt` of the map `request`.
- `[request.prompt]` makes an array with one element.
- The last expression of a function, without a semicolon, is its return value.
  `return value;` returns earlier.

What can the script read about the request? The request is a map with these
fields. Optional inputs have the value `()` when they are absent. In Rhai, `()`
means “no value”, and it becomes `null` in JSON.

| Field | Value |
|---|---|
| `schemaVersion` | The integer `4`. |
| `kind` | The caller's [kind](02-actors-and-words.md#caller-and-owner) token, from `--kind`. |
| `originalPrompt` | The caller's prompt, unchanged. |
| `prompt` | The prompt to give to the harness. With skill delivery disabled, it holds the same bytes as `originalPrompt`. With delivery enabled, it holds the discovery instructions, then the caller's prompt in one delimited block. |
| `cwd` | The caller's working directory. |
| `params` | A map of strings from `--param NAME=VALUE`. |
| `taskFile` | The absolute `--task-file` path, or `()`. Ravel does not read it. |
| `taskId` | The `--task-id` string, or `()`. |
| `context` | The caller's checked context document, or `()`. |
| `skills` | `()` with skill delivery disabled. Otherwise a map with `descriptorPath`, `sources`, `directories` and `mandatory`. |
| `limits` | The effective bounds that the script can read. |

Put `request.prompt` into the arguments of the harness. Read
`request.originalPrompt` when the script examines the task, for example when
it asks a model to classify the task.

The owner can enable skill delivery in the settings. Then `request.prompt`
also holds the skill directories and the discovery instructions, and the
script needs no change for them. A harness that reads only some directories
needs a read grant for the skill snapshots. Make that grant from
`request.skills`, for example one argument for each `snapshotRoot`. The
shipped example `harness-table.rhai` does this for its lead harness.

Excerpt from `crates/ravel/examples/harness-table.rhai`: the function that
lists the snapshot roots. The function `harness_args` puts each root after
`--add-dir`.

```rhai
fn skill_reads(skills) {
    if skills == () { return []; }
    let reads = [];
    for source in skills.sources { reads.push(source.snapshotRoot); }
    reads
}
```

[Skill delivery at launch](06-reference.md#skill-delivery-at-launch) gives the
parts of the prepared prompt and the members of `request.skills`.

The second argument, `context`, is the delivered
[selection context](#supply-selection-context): JSON data for the choice. A
loader can make it different from `request.context`. Neither value gives the
script authority to end the session.

Ravel checks undefined variables when it compiles the script.
It also refuses an absent map property when the script reads it.
Check `request.params.contains("name")` before you read an optional parameter.
The documented request fields themselves are always present.

## Select a command or refuse

Your configuration script gives one of two answers to each request. A selection tells Ravel
which program to start. A refusal prevents the start of the final managed
process. Why refuse?
You may want a request that does not fit your routes to stop with a clear
message, and not go to the wrong harness.

A selection is the map that `select` returns when it chooses a program. Its
`status` field is `"selected"`. A selected map holds seven fields, and Ravel
needs each of them. Ravel refuses an unknown field.

| Field | What the script supplies |
|---|---|
| `status` | The literal `"selected"`. |
| `program` | A nonblank program name or path, without NUL. |
| `args` | An array of strings after the program. Empty strings are valid, but NUL is not. |
| `provider` | A nonblank owner label for the model origin. |
| `model` | A nonblank owner label for the model. |
| `effort` | A nonblank owner label for the effort. |
| `reason` | A nonblank explanation of the choice. |

A bare program name resolves through the caller's `PATH`.
A relative path with `/` resolves from the caller's working directory.
Each member of `args` is one argument.
Ravel performs no word splitting, shell expansion or template substitution.
A prompt with spaces therefore stays one argument when the script puts it there.

`program` and `args` are the command that Ravel starts. `provider`, `model`,
`effort` and `reason` are labels. A label describes the choice of the script,
but it does not configure the program.
For example, `model: "haiku"` records a label, whereas
`args: ["--model", "haiku"]` gives a model option to a program.
Your script sets both, and Ravel does not compare them.

Why have labels, then? The caller usually wants to know what the script chose,
but the caller does not read your script. Ravel tells the caller in the routing
notice. Before it starts the program, the Ravel CLI writes one line on the
caller's standard error. For the first example of this chapter,
`fixed-route`, that line was:

Fragment: the routing notice of `fixed-route`, on standard error. This block
omits the command, the reports of cleanup operations and the result on
standard output.

```text
ravel: starting provider example, model fake, effort low for kind "question"
```

With `--json`, a JSON document replaces this line after the supervision
attempt. That document also gives the `reason`.
[Chapter 5, “Read notices separately from results”](05-running-sessions.md#read-notices-separately-from-results),
teaches both forms.

A refusal is a map with `status: "refused"` and three more strings. Each
string is nonblank:

| Field | What the script supplies | Who reads it |
|---|---|---|
| `code` | A short token that names the reason, such as `unknown_kind` | A caller program that acts on the reason |
| `message` | A sentence that says what is wrong | The person who reads standard error |
| `remedy` | A sentence that says what to change | The person who reads standard error |

After a refusal, Ravel reports `config_refused` with exit status 3.
Your `code` appears separately as `configCode` in the JSON failure document.
No managed process starts, and Ravel tries no other route.
The caller receives no result.

The next example is `refusals`. Make its directory as for `fixed-route`:
`~/ravel-chapter-4/refusals`, with a `bin` subdirectory. It has the same two
file paths. `bin/fake-agent` has the same contents, but `config.rhai` is
different. The example has three command blocks. Run them in order:

- The first block makes `home` and `tmp`, then asks with the kind `question`.
- The second block asks with the kind `unknown`.
- The third block asks with the kind `question` and adds `--param model=other`.

Complete example: accept one kind and refuse an unknown kind or a forbidden
parameter.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "checked-route-1";

fn refuse(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

fn select(request, context) {
    if request.kind != "question" {
        return refuse("unknown_kind", "This configuration script accepts question only.",
                      "Use --kind question.");
    }
    if request.params.contains("model") {
        return refuse("owner_model", "The owner chooses the model.",
                      "Remove the model parameter.");
    }
    #{ status: "selected", program: "fake-agent", args: [request.prompt],
       provider: "example", model: "fake", effort: "low", reason: "question route" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
hello
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind unknown --prompt "hello"
```

**Standard output**

```text
```

**Standard error**

```text
ravel: refused (config_refused, stage selection): the configuration script <scratch>/work/config.rhai refused the selection: This configuration script accepts question only.
  input: --kind unknown
  source: <scratch>/work/config.rhai
  config code: unknown_kind
  remedy: Use --kind question.
```

**Exit status**

```text
3
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --param model=other --prompt "hello"
```

**Standard output**

```text
```

**Standard error**

```text
ravel: refused (config_refused, stage selection): the configuration script <scratch>/work/config.rhai refused the selection: The owner chooses the model.
  input: --kind question
  source: <scratch>/work/config.rhai
  config code: owner_model
  remedy: Remove the model parameter.
```

**Exit status**

```text
3
```

How do you read a refusal? The first command succeeded, and the two refused
commands wrote nothing on standard output. Each refusal wrote these lines on
standard error. The values come from the third command:

| Line | What it tells you |
|---|---|
| `ravel: refused (config_refused, stage selection)` | Ravel's code for the outcome, and the stage in which it occurred. The script refused during selection. |
| `the configuration script … refused the selection: The owner chooses the model.` | The file of the configuration script, then the script's `message` |
| `input: --kind question` | The kind of the request |
| `source: <scratch>/work/config.rhai` | The file of the configuration script. `<scratch>/work` stands for the example's directory. In your shell, the line shows the absolute path of your `refusals` directory. |
| `config code: owner_model` | The script's own `code` |
| `remedy: Remove the model parameter.` | The script's `remedy`, which tells the caller what to change |

The exit status 3 tells a caller program that a refusal occurred, without a
read of the text. With `--json`, the same fields are in the JSON failure
document, and the script's code is in `configCode`.

`if` chooses a branch. `!=` compares values for inequality.
`fn refuse(...)` is an ordinary helper function, with no special Ravel role.
Rhai functions have no parameter type declarations here.
Ravel checks the value when it crosses the boundary from the script.

A return of `()` gives `selection_abstained`.
An uncaught error in `select` gives `selection_threw` with a source location.
A wrong field or type gives `selection_malformed`.
After any of these outcomes, Ravel starts no managed process.

<!-- tc: depart synonym-rotation because validate names the end-to-end test of a script, while check names a single diagnostic inspection -->
## Validate a configuration script with fake agents

How do you know that your configuration script does what you want before a
real harness runs? You test it with fake agents. Ravel has no separate command
that type-checks a script or evaluates it without the start of a program.
`start-session`
evaluates the script and then runs the selected program. Thus, each test of a
script is a real session.
A fake agent makes that session cheap and safe, because it only sends back its
arguments.

For each route, test the arguments and the result.
For each refusal, test the diagnostic and absence of a managed process.
Include absent optional inputs, unknown kinds and malformed local data.
The examples in this chapter show success and failure at that same boundary.

You may also want to see values while the script runs. `print(text)` writes to
the script's standard output, and `debug(text)` writes to the script's standard
error. Ravel captures both streams of the Rhai worker. Then it writes the
captured text on its own standard error, before the routing notice.
Each line starts with `config stdout:` or `config stderr:`.

These lines are diagnostics for you. They are not the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result) on
Ravel's standard output, which Ravel does not interpret. The managed process
sends that result with `end-session`. With `--json`, the JSON routing notice
holds the captured text in its `diagnostics` member.

The next example is `print-and-parse`, in `~/ravel-chapter-4/print-and-parse`
with a `bin` subdirectory. It has a third file, `broken.rhai`, beside
`config.rhai`. `broken.rhai` is a second configuration script with a syntax
error. `bin/fake-agent` is the same as in `fixed-route`. The second command
block selects `broken.rhai` with `--config broken.rhai`.

Complete example: print a value from the script, then find a syntax error.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "print-1";

fn select(request, context) {
    print("kind is " + request.kind);
    #{ status: "selected", program: "fake-agent", args: [request.prompt],
       provider: "example", model: "fake", effort: "low", reason: "print demo" }
}
```

**`broken.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "broken-1";

fn select(request, context) {
    #{ status: "selected", program: "fake-agent" args: [] }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
hello
```

**Standard error includes**

```text
config stdout: kind is question
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config broken.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error**

```text
ravel: refused (config_parse_failed, stage load): Expecting ',' to separate the items of this object map literal
  source: <scratch>/work/broken.rhai
  location: 5:50
  remedy: fix the syntax error in <scratch>/work/broken.rhai, at the line and column the location names
```

**Exit status**

```text
3
```

`+` joins two strings. The first command wrote the line of `print` on standard
error, and the result `hello` on standard output. The second script has no
comma after `"fake-agent"`. Ravel found the error when it compiled the file, in
the stage `load`. `location: 5:50` names line 5, column 50 of `broken.rhai`.
Ravel started no program.

The table says where to look for each outcome that a test can show:

| Observation | What to examine |
|---|---|
| A parse failure | The source file, line and column in the diagnostic. |
| `config_invalid` | The exports, their types and the function parameter counts. |
| `selection_threw` | The property or expression at the reported location. |
| `selection_malformed` | The shape and field types of the returned map. |
| `config_refused` | The `configCode`, message and remedy of your script. |
| `selection_timeout` | The work before the selection and its whole deadline. |

`debug(text)` also names the file, the line and the position of the call.
Use `--json` when a program, not a person, reads the routing notice and the
diagnostics.

## Put each kind and its route in one table

As your configuration script gets more kinds, you may want to see each kind
and its route in one place. A table in the script does this. It also makes a
review of your choices easy, because each route is one line.
Ravel ships an example configuration script, `static.rhai`, that has one such
table. You can start from it.
Its program, provider and model are placeholders that you replace.

Excerpt from `crates/ravel/examples/static.rhai`: the example's kinds and efforts.

```rhai
export const ROUTES = #{
    question: agent("low"),
    bugfix: agent("medium"),
    feature: agent("high"),
    migration: agent("xhigh"),
    architecture: agent("xhigh"),
};
```

These kinds and efforts belong to this example.
Ravel defines neither list. A kind is the caller's token, and only your script
gives it a meaning.
The helper `select_route` refuses an unknown kind with `incomplete_mapping`.
When a route matches, it returns a command. The helper adds `status` and `reason`.

A smaller table can store strings directly.
`const` declares a value that the script does not reassign.
A function reads a top-level constant through `global::`, as the next example
shows. `let` declares a local variable that the script can change.

The next example is `route-table`, in `~/ravel-chapter-4/route-table` with a
`bin` subdirectory. It has the same two file paths as `fixed-route`, but both
files are different. This `bin/fake-agent` sends its first two arguments,
joined by `|`. The three command blocks ask with the kinds `question`,
`feature` and `other`.

Complete example: route two kinds through a local table.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "table-1";
const EFFORTS = #{ question: "low", feature: "high" };

fn select(request, context) {
    if !global::EFFORTS.contains(request.kind) {
        return #{ status: "refused", code: "unknown_kind",
                  message: "The table has no route for this kind.",
                  remedy: "Use question or feature." };
    }
    let effort = global::EFFORTS[request.kind];
    #{ status: "selected", program: "fake-agent", args: [effort, request.prompt],
       provider: "example", model: "fake", effort: effort, reason: "table lookup" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1|$2"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
low|hello
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind feature --prompt "hello"
```

**Standard output, without a final newline**

```text
high|hello
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind other --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
config_refused
```

**Exit status**

```text
3
```

The `!` operator means logical negation.
`table[key]` selects a property whose name comes from a variable.
The earlier `contains` check prevents an absent property error.
Rhai functions do not capture the variables around their definitions.
Pass local values as parameters, or use a top-level constant through `global::`.

## Import other Rhai files

When a configuration script grows, you may want to keep the route table or
some helper functions in a separate file. You may also want two scripts to
share the same helpers. An import does this. It gives a name to another module, which is
another Rhai file.

`import "routes" as routes;` reads `routes.rhai` beside the file that imports it.
An absolute import uses that path.
Ravel appends `.rhai` when the path has no extension.
For a symlinked script, imports start from the real file's directory.

Imports and measured reads use different bases. An import starts from the
directory of the file that imports it. A measured read starts from the caller's
directory. Thus, the same relative name can find two different files:

| Operation | Base for a relative path |
|---|---|
| `import "routes" as routes;` | The directory of the file that imports it. |
| `ravel::read_text("choice.txt")` | The caller's `request.cwd`. |
| `ravel::read_json("choice.json")` | The caller's `request.cwd`. |
| `--config config.rhai` | The caller's working directory. |

Each distinct imported module loads once per evaluation.
Ravel measures the entry and each module with its canonical path, byte count
and SHA-256 digest. The JSON routing notice reports those measurements in
`config.modules`. Thus, the notice shows exactly which bytes of which files
made the choice.

A missing module, import cycle or 65th distinct imported module gives
`config_import_failed`.
Ravel embeds no Rhai module of its own. Imports name files, which can be
copies of the shipped examples.

The next example is `imports`, in `~/ravel-chapter-4/imports` with a `bin`
subdirectory. It has a third file, `routes.rhai`, beside `config.rhai`.
`routes.rhai` is the module that `config.rhai` imports. `bin/fake-agent` is
the same as in `route-table`. The command is the same as in `fixed-route`,
with the prompt `hello`.

Complete example: call an imported function and use its exported constant.

**`config.rhai`**

```rhai
import "routes" as routes;
export const SCHEMA_VERSION = 4;
export const VERSION = "import-1";

fn select(request, context) {
    routes::choose(request, routes::EFFORT)
}
```

**`routes.rhai`**

```rhai
export const EFFORT = "medium";

fn choose(request, effort) {
    #{ status: "selected", program: "fake-agent", args: [effort, request.prompt],
       provider: "example", model: "fake", effort: effort, reason: "local module" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1|$2"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
medium|hello
```

**Exit status**

```text
0
```

## Let a repository choose an arrangement

You may want one repository to get more careful work than another. For
example, a repository with a production service can ask for a higher effort.
But you still want to decide which arrangements exist. A checkout choice does
this. The repository names one arrangement in a file, and your configuration
script accepts only the names that you offer.

The shipped `sample.rhai` reads `.ravel-choice` from `request.cwd`, the
caller's working directory.
That filename is a rule of the sample, not a Ravel configuration search rule.

The sample imports an adjacent copy of `static.rhai`.
An absent choice file uses `default`.
A file with exactly `careful` changes the selected effort to `high` and rebuilds
the arguments. It leaves an unknown kind as a refusal.

| Choice file | Sample behaviour |
|---|---|
| Absent | Use `default`. |
| `default` | Use the route table of `static.rhai` unchanged. |
| `careful` | Use the route table of `static.rhai` with effort `high`. |
| Empty or more than one offered name | Refuse with `one_choice`. |
| An unoffered name | Refuse with `choice_unoffered`. |
| An unreadable file or a directory | Refuse with `choice_unreadable`. |

The sample implements this behaviour with `ravel::read_text` and ordinary
Rhai string operations. No special choice-file API is necessary.
The next example uses a smaller choice rule so you can test every branch.

The next example is `checkout-choice`, in `~/ravel-chapter-4/checkout-choice`
with a `bin` subdirectory. It has the same two file paths as `fixed-route`.
`bin/fake-agent` has the same contents, but `config.rhai` is different. The
example directory stands for the caller's repository. Do not save a
`.ravel-choice` file:

- The first command block runs without `.ravel-choice`.
- The second block first writes `careful` to `.ravel-choice` with `printf`.
- The third block replaces that text with `other`.

Run the three blocks in order, because each block uses the file that the
block before it left.

Complete example: accept `careful`, use a default when absent, and refuse any
other text.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "choice-1";

fn select(request, context) {
    let choice = "default";
    try {
        choice = ravel::read_text(".ravel-choice", 4096).text;
        choice.trim();
    } catch (error) {
        if error.code != "source_missing" { throw error; }
    }
    if choice != "default" && choice != "careful" {
        return #{ status: "refused", code: "choice_unoffered",
                  message: "The choice is not offered.",
                  remedy: "Write default or careful in .ravel-choice." };
    }
    let effort = if choice == "careful" { "high" } else { "low" };
    #{ status: "selected", program: "fake-agent", args: [effort],
       provider: "example", model: "fake", effort: effort, reason: "checkout choice" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
low
```

**Exit status**

```text
0
```

**Command**

```console
$ printf 'careful\n' > .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
high
```

**Exit status**

```text
0
```

**Command**

```console
$ printf 'other\n' > .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
config_refused
```

**Exit status**

```text
3
```

What happened in the three commands? Without `.ravel-choice`, the read threw
`source_missing`. The script caught it and kept `default`, so the effort was
`low`. With `careful` in the file, the effort was `high`. With `other` in the
file, the script refused, and the caller received no result.

The script used this Rhai:

- `try` runs a block whose ordinary error can reach `catch (error)`.
- `throw error` passes an error onwards.
- A measured read throws a map with `code` and `message`, and a `name` where
  applicable. The script catches only the absent file here.
- `trim()` removes spaces and line ends at the start and the end of the string.
  It changes the string in place.

A successful read returns `text` and `source`.
`source` holds the canonical `name`, the byte count and `sha256`.
The JSON routing notice records the read in `reads`, with the evaluation
phase: `load` for the top level, `context` for `load_context` or `selection`
for `select`.

The context budget is your limit on the bytes of selection context, and
[“Set bounds and environment grants in the settings”](#set-bounds-and-environment-grants-in-the-settings)
explains it. The default read bound is the smaller of 65,536 bytes and the
context budget. The optional second argument changes it, up to the context
budget. The example gives 4096 bytes. A choice is one short word, so a small
bound is enough.

| Read failure | Can the script catch it? |
|---|---|
| `source_missing`, `source_not_regular`, `source_unreadable`, `source_not_utf8` | Yes. |
| `source_invalid_json` from `read_json` | Yes. |
| `source_too_large`, `too_many_sources` | No. They end evaluation. |
| A malformed host request | No. It ends evaluation as `host_request_invalid`. |

## Supply selection context

You may want facts about the task to change the choice. For example, a large
change can go to a higher effort, or a task with a security label can go to a
particular model. Selection context carries such facts to `select`.

The book uses the word “context” for three different things. This section is
about the second:

| Name | What it is | Who uses it |
|---|---|---|
| Session-result context | The path in `RAVEL_EXIT_FILE`, through which `end-session` sends the result | The managed process. The configuration script never receives it. |
| Selection context | A JSON document of data for the choice | `select`, as its second argument |
| The caller's working directory | The directory of the request, in `request.cwd` | The script, for measured reads and relative program paths |

Selection context is data for the decision, not authority to execute a command.
The caller can supply a JSON context document with `--context PATH`.
Without a loader, `select` receives that document with its measured source,
or `()` when no document exists.

Why write a loader? You may want facts that the caller does not send, such as
a file in the caller's repository. A `load_context(request)` function can
assemble a different document. Ravel calls it before `select`.

The loader can read the caller's input at `request.context` and read local sources.
Ravel checks its return value before it calls `select`.
A loader that returns `()` gives `context_invalid`.
A loader can also return a refusal, in which case `select` never runs.

| Document field | Purpose |
|---|---|
| `schemaVersion` | Required integer `1`. |
| `summary` | Optional text for the configuration script. |
| `facts` | Optional map of JSON values. |
| `assessments` | Optional map of judgements, each with `by` and `value`. |
| `sources` | Optional attributed source records, each with `name` and a digest or version. |
| `acceptanceCriteria` | Optional array of strings that state the caller's criteria. |

Ravel adds `measured` to the delivered document.
The caller and loader do not supply that field themselves.
It lists the caller's context file first, then the reads made by the loader.
A read in `select` appears in the routing notice but changes no delivered context.

Attributed `sources` describe claims. `measured` records the bytes Ravel read.
Neither measurement establishes that a claim is true.

The next example is `selection-context`, in
`~/ravel-chapter-4/selection-context` with a `bin` subdirectory. It has a third
file, `task.json`, beside `config.rhai`. `task.json` holds the task data that
the loader reads from the caller's working directory. This `bin/fake-agent`
sends its first three arguments, joined by `|`. The command uses the kind
`review`.

Complete example: read task data, build selection context, and use it in the
selection.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "context-1";

fn load_context(request) {
    let task = ravel::read_json("task.json");
    #{ schemaVersion: 1, summary: task.value.title,
       facts: #{ effort: task.value.effort } }
}

fn select(request, context) {
    #{ status: "selected", program: "fake-agent",
       args: [context.summary, context.facts.effort,
              context.measured.len().to_string()],
       provider: "example", model: "fake", effort: context.facts.effort,
       reason: "task facts" }
}
```

**`task.json`**

```json
{"title":"Review the change","effort":"high"}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1|$2|$3"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind review --prompt "hello"
```

**Standard output, without a final newline**

```text
Review the change|high|1
```

**Exit status**

```text
0
```

Follow the data from the file to the result:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | `load_context` | Reads `task.json` with `ravel::read_json` and builds a document with a `summary` and one fact |
| 2 | The Ravel CLI | Checks the document and adds `measured`, with one record for `task.json` |
| 3 | `select` | Puts the summary, the effort and the count of measured records in `args` |
| 4 | `fake-agent` | Sends `Review the change|high|1` with `end-session` |

`read_json` returns `value` and `source`.
The loader chooses which values enter the document.
`.len()` counts the measured records, and `.to_string()` makes a string for
`args`. Ravel refuses an integer in that array, because each argument of a
program is a string.

The context budget applies to the caller's file and to the delivered JSON,
with `measured`. The default is 262,144 bytes.
At most 256 measured sources can contribute to one evaluation.
A file counts when its bytes arrive, even if the bytes are not valid JSON or UTF-8.
An absent file does not consume a source.

## Check JSON with the data helpers

Your configuration script reads JSON from files, and later from models. You
may want to be sure that this JSON has the shape that you expect before the
script acts on it. A model can give an answer with a wrong field or a value
that you did not offer. The data helpers let the script parse, check and
write JSON strictly.

Ravel supplies strict JSON operations in the `ravel` namespace.
They run inside the Rhai worker and send no request to the Ravel CLI.
Use them for data that crosses between the script and another program.
The Rhai built-in `parse_json` accepts syntax that strict JSON rejects.
Its `to_json` can produce string escapes that are not JSON.

| Helper | Use |
|---|---|
| `ravel::json_parse(text)` | Parse strict JSON into a Rhai value. |
| `ravel::json_encode(value)` | Encode JSON with sorted object keys and no whitespace. |
| `ravel::json_check(value, schema)` | Check a JSON Schema subset and return the value unchanged. |
| `ravel::uuid_v4()` | Generate a random version-4 UUID string. |
| `ravel::path_resolve(base, path)` | Join a path to an absolute base and normalise `.` and `..`. |
| `ravel::path_parent(path)` | Return the parent of an absolute path. |

The path helpers read no file and resolve no symlink.
An absolute second path replaces the base in `path_resolve`.
A relative base or relative `path_parent` input throws `path_not_absolute`.
Use a measured read when you also need to establish what a file contains.

Values at the boundary are JSON values: null, booleans, numbers, strings,
arrays and maps. Rhai's `()` represents null.
Characters, function pointers, ranges and other non-JSON types do not cross.
Ravel also refuses non-finite numbers. Arrays and maps nest at most 100 levels.
A JSON integer outside the signed 64-bit range becomes a float.

The next example is `data-helpers`, in `~/ravel-chapter-4/data-helpers` with a
`bin` subdirectory. It has the same two file paths as `fixed-route`.
`bin/fake-agent` has the same contents, but `config.rhai` is different. The
command is the same as in `fixed-route`, with the prompt `hello`.

Complete example: check data, handle a failed check, and encode the result.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "data-1";

fn select(request, context) {
    let schema = #{ type: "object",
                    properties: #{ effort: #{ type: "string", enum: ["low", "high"] } },
                    required: ["effort"], additionalProperties: false };
    let good = ravel::json_check(ravel::json_parse("{\"effort\":\"high\"}"), schema);
    let failure = "none";
    try { ravel::json_check(#{ effort: "other" }, schema); }
    catch (error) { failure = error.code; }
    let values = [];
    for value in [good.effort, failure] { values.push(value); }
    #{ status: "selected", program: "fake-agent", args: [ravel::json_encode(values)],
       provider: "example", model: "fake", effort: good.effort, reason: "checked data" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
["high","json_mismatch"]
```

**Exit status**

```text
0
```

What did the script do? It parsed `{"effort":"high"}` and checked it against
the schema, which passed. Then it checked `#{ effort: "other" }`, which failed
with the code `json_mismatch`. The script caught that code and sent both values
to the fake agent as one JSON array.

`for value in array` visits each element.
`push` adds an element to the array.
The schema requires a string whose value is `low` or `high`.
It also rejects properties other than `effort`.
A schema checks shape and permitted values. The script still decides their meaning.

The supported schema keywords are `type`, `properties`, `required`,
`additionalProperties`, `items`, `enum`, `title` and `description`.
The last two are descriptive only.
`additionalProperties` is a boolean, and `enum` holds scalar JSON values.
Unknown keywords and schema depth beyond 32 levels throw `schema_invalid`.
A mismatch throws `json_mismatch`. Its message names the location and can include
property names, but omits property values.

`json_parse` throws `json_invalid` for invalid JSON or excessive depth.
`json_encode` and `json_check` throw `json_unencodable` for a non-JSON value.
Ravel disables `eval`, so a model's answer cannot become code through that function.
Keep model answers as data that the script checks and interprets.

## Set bounds and environment grants in the settings

You may want to give a slow configuration script more time, or let the script
read an environment variable of the caller. You may also want to name the
harnesses that the script can run. Your settings do each of these things. They
are the second part of your configuration, in a JSON file next to the default
script.

Ravel reads the settings from `$HOME/.config/ravel/settings.json`. It reads
this file also when `--config` selects another script. Thus, a script that you
copy to another place does not carry its own settings.
An absent file sets nothing. Each key is optional, and Ravel refuses unknown keys.

| Key | Meaning and range |
|---|---|
| `timeoutMs` | Whole evaluation deadline, 1,000–600,000 ms. Default: 30,000. |
| `contextBytes` | Context budget, 1–8,388,608 bytes. Default: 262,144. |
| `configEnv` | Additional caller environment names that `ravel::env` can read. |
| `harnesses` | Named programs available to `ravel::run_harness`. |
| `budgets` | An object whose optional `harnessRuns` is an integer from 0 to 16. Default: 0. |

The caller can also change two bounds for one session.
`--timeout-ms` and `--context-bytes` replace the settings of those bounds.
Ravel checks the settings first, so a flag cannot override an invalid file.
`--config-env NAME` adds a grant to those in `configEnv`.
Duplicate grants count once.

`request.limits` holds `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes`, `diagnosticsBytes` and `harnessRuns`.
These values describe the effective bounds. A change to the map cannot raise them.
`ravel::remaining()` reports what remains of the evaluation: time in `ms`,
`harnessRuns` and `sources`.

The whole deadline covers module loads, the context loader, selection and all
host calls. It does not bound the final managed process.
At expiry, Ravel reports `selection_timeout`, exit status 124.
Cancellation or failed harness cleanup can take precedence.
Cleanup takes additional time.
Other fixed bounds include 1 MiB worker messages and 262,144 bytes of
diagnostics from the script.

| Bound reached | Effect |
|---|---|
| Whole deadline | End evaluation and start no managed process. |
| Source, context, message or diagnostics bound | End evaluation and start no managed process. |
| Engine operation, depth or data bound | End evaluation with `script_limit`. |
| Worker heap bound of 1 GiB | End evaluation with `memory_limit`. |
| Harness-run budget | End evaluation with `harness_budget_exhausted`. |
| One harness run's deadline or output bound | Return a status to the script if the whole evaluation can continue. |

The script cannot catch an evaluation bound with `try`.
This rule also applies inside a closure passed to an array or map helper.
A deadline or output bound of one harness run is different: it returns data,
as the next section explains.

Why would a configuration script read an environment variable? The caller can
set a variable, for example a name for the effort that it wants, and the
script can use it in the choice.

But the
[Rhai worker](02-actors-and-words.md#evaluation-and-supervision) has
an empty environment. Thus, the script asks the Ravel CLI: `ravel::env` is a
host capability that returns the caller's value of one variable.
It permits `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*` and explicit grants.
An unset permitted name returns `()`.
An ungranted name throws `env_not_granted`, even if it is unset.
A value outside UTF-8 throws `env_not_utf8`.

The next example is `settings-grants`, in `~/ravel-chapter-4/settings-grants`
with a `bin` subdirectory. It has a third file, `settings.json`, at the top
of the example directory. This `bin/fake-agent` sends its first three
arguments, joined by `|`. The command block has three lines:

- The first line is the same as in `fixed-route`.
- The second line copies `settings.json` into `home/.config/ravel/`. Ravel
  reads the settings only from that path in the private home.
- The third line sets `BOOK_EFFORT` and `BOOK_HIDDEN` after `env -i`, and
  adds `--timeout-ms 10000`.

Complete example: grant one variable and observe a CLI bound replacement.
The command installs settings only in the example's private home.

**`settings.json`**

```json
{"timeoutMs":5000,"contextBytes":4096,"configEnv":["BOOK_EFFORT"]}
```

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "settings-1";

fn select(request, context) {
    let effort = ravel::env("BOOK_EFFORT");
    let hidden = "none";
    try { ravel::env("BOOK_HIDDEN"); }
    catch (error) { hidden = error.code; }
    #{ status: "selected", program: "fake-agent",
       args: [effort, request.limits.selectionMs.to_string(), hidden],
       provider: "example", model: "fake", effort: effort, reason: "settings" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1|$2|$3"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" BOOK_EFFORT=high BOOK_HIDDEN=unused PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --timeout-ms 10000 --prompt "hello"
```

**Standard output, without a final newline**

```text
high|10000|env_not_granted
```

**Exit status**

```text
0
```

What happened in this example? The settings granted `BOOK_EFFORT`, so
`ravel::env` returned `high`. The caller also set `BOOK_HIDDEN`, but the
settings did not grant it. Thus, `ravel::env` threw `env_not_granted`, and the
script caught the code. The caller's `--timeout-ms 10000` replaced the
`timeoutMs` of 5000 in the settings, so `request.limits.selectionMs` was
`10000`.

Environment grants control this helper, not the environment of an unconfined
managed process or harness run. Those programs receive the caller's environment,
with the control variables of an enclosing session or driver removed or replaced.
Ravel refuses grants for its reserved variables and dynamic-loader variables.
The [reference](06-reference.md) lists the reserved classes.

A measured read can reach files that the user can read.
These grants therefore do not create a filesystem sandbox for the script.
You control the configuration script and choose which data it reads.

## Run a harness during evaluation

You may want an LLM to choose the best harness, model, effort and prompt
additions for a kind and a prompt. You can do this with a harness run in your
configuration script. A harness run is one run of a program from your
`harnesses` settings, during evaluation. The script starts it with the host
capability `ravel::run_harness`. Then the script reads the output of the run
as data and makes its own choice.

How does the answer get back to the script? Follow one harness run:

| Order | Who acts | What happens |
|---|---|---|
| 1 | The configuration script | Calls `ravel::run_harness` with the name of a settings entry, the arguments and the text for standard input |
| 2 | The Rhai worker | Sends the request to the Ravel CLI and waits |
| 3 | The Ravel CLI | Finds the entry's program, checks the run budget and starts the program in a new POSIX session with its own result channel |
| 4 | The harness run | Reads its standard input, writes its answer on standard output and exits |
| 5 | The Ravel CLI | Stops the processes that remain in the group of the run, completes cleanup and sends a map back to the worker |
| 6 | The configuration script | Reads `status`, `exitCode` and `stdout` from the map, then continues |

The harness run returns its answer to the script before Ravel starts the final
managed process.

![The Rhai worker requests a claude harness run from the Ravel CLI and waits. The CLI checks the budget and starts the run with its own result channel. The run reads standard input, writes its answer and exits. After cleanup the CLI returns a map to the worker. The configuration script selects, the CLI reaps the worker, writes the routing notice and starts the managed process.](figures/evaluation-harness.svg)

*Figure: The evaluation harness run and the managed process are two separate programs that Ravel starts.*

[Open the evaluation harness sequence at full size](figures/evaluation-harness.svg).

A [result channel](02-actors-and-words.md#the-managed-process-and-its-result)
is a private temporary directory with a path for one result file. The path
goes to the program in `RAVEL_EXIT_FILE`. A harness run gets a result channel
of its own. Thus, a run that uses `end-session` sends its value to the script,
not to the session.

The answer goes back to the script, not to the caller. The caller sees the
script's selection in the routing notice and receives the final session's
result after cleanup. The JSON routing notice also records each harness run,
but not its prompt or its answer.

Fragment: the `harnessRuns` member of the JSON routing notice, from a real run
of the next section's Claude Code example with `--json`. This block omits the
other members of the document. The value of `elapsedMs` changes in each run.

```json
{"harnessRuns":[{"elapsedMs":314,"exitCode":0,"harness":"claude","sentResult":false,"signal":null,"status":"exited"}]}
```

The record names the settings entry `claude`. The run exited with status 0
after 314 milliseconds, and no signal stopped it. `sentResult: false` says that
the run did not use `end-session`. It wrote its answer on standard output.

A harness entry fixes the program that a host call can run.
Its name has 1–64 lowercase letters, digits or dashes. Its first character is a letter
or digit. The entry has a required `program` and optional `capabilities` labels.
Ravel gives those labels no meaning.

Fragment: settings for a harness run. This block omits the configuration
script and the program.

```json
{
  "harnesses": {
    "classifier": {"program":"my-classifier","capabilities":["classify"]}
  },
  "budgets": {"harnessRuns":1}
}
```

`ravel::harnesses()` reports the entries in name order and runs no program.
Each map has `name`, `program`, `available`, `resolved`, `reason` and
`capabilities`. An available program has a resolved path and `reason: ()`.
An unavailable one has `resolved: ()` and a reason of `not_found` or
`not_executable`. Discovery reports availability, not a recommendation or live model list.

`ravel::run_harness` accepts this request:

| Field | Meaning |
|---|---|
| `harness` | Required name of a settings entry. |
| `args` | Required array of argument strings, without NUL. |
| `stdin` | Optional text for standard input. Without it, input is at end of file. |
| `timeoutMs` | Optional positive integer in milliseconds, capped by the time left for evaluation. |

The call is synchronous: the script waits until the run ends. Only one host
call runs at a time.

The run starts in `request.cwd`, without a terminal, in a new POSIX session.
A POSIX session is the operating system's group of process groups, which
`setsid` makes. It is not a Ravel session.
It uses the environment and [confinement](02-actors-and-words.md#trust-and-confinement) rules of a managed process.
Confinement is the optional restriction of the filesystem access and the
environment of a program, which the caller asks for with `--confine`.
`--confine` therefore also confines harness runs.

The script supplies every argument and chooses where the prompt goes.
Ravel builds no argument that is specific to a harness.

The returned map describes the completed run:

| Field | Value |
|---|---|
| `status` | `exited`, `signalled`, `timed_out`, `output_limit` or `unavailable`. |
| `exitCode` | An integer or `()`. |
| `signal` | A signal name or `()`. |
| `stdout`, `stderr` | Captured text. Invalid UTF-8 becomes the replacement character. |
| `result` | The value sent by the run with `end-session`, or `()`. |
| `elapsedMs` | Elapsed milliseconds. |
| `error` | `()`, or a map with `code` and `message` for an unavailable run. |

A run can finish by natural exit and return standard output to the script.
It does not have to call `end-session`.
If it does, that result belongs only to this run.
It cannot complete the outer session.
The final managed process later receives a different
[result channel](02-actors-and-words.md#the-managed-process-and-its-result).

An unavailable run reports `unknown_harness`, `harness_not_found`,
`harness_not_executable` or `harness_launch_failed` and consumes no run budget.
Each run that starts consumes one.
Each output stream has a 1 MiB bound.
A per-run timeout or output overflow stops the run and returns its status.
A cleanup failure ends evaluation as `harness_cleanup_failed`, exit status 5.

Can a harness run start a Ravel session of its own? No. Ravel puts the
marker `RAVEL_CONFIG_RUN` in the environment of each harness run, and each
child of the run inherits it. `start-session` finds the marker and refuses as
`config_reentry_refused`, before it reads the settings or a configuration
script. Thus, a harness run cannot start a second evaluation inside the first.

Do you need the route of another kind during evaluation? Call that route's
function in your script directly. The final managed process never receives the
marker, so a session that the final harness nests stays allowed.

The marker does not protect against code that removes it.
A descendant that leaves the process group is outside group cleanup.

## Ask a model through Claude Code

Do you want an LLM in Claude Code to help your configuration script choose? This section
gives the recipe. An
[LLM call](02-actors-and-words.md#evaluation-and-supervision) is a
harness run that asks a model a question. Claude Code runs the call under your
own sign-in, so the call can use your subscription. Ravel stores no model
credential.

The script reads the answer and decides what it means. A model can give an
unexpected answer, so the script asks for structured output. Structured output
is an answer in JSON, in a shape that a JSON Schema describes. The script then
checks the answer against the same schema with `ravel::json_check`. The fake
below uses the same arguments as a real call, but it returns a fixed answer.

The recipe uses Claude Code 2.1.310.
The author checked the installed help on 2026-10-26.
The repository's real smoke call also succeeded with that version.
Its arguments and response field are the basis of this example.

| Argument | Purpose |
|---|---|
| `-p` | Print an answer and exit. |
| `--model haiku` | Select the model for this call. |
| `--tools ""` | Give the call no tools. |
| `--no-session-persistence` | Do not retain the session in local history. |
| `--output-format json` | Return a JSON document. |
| `--json-schema TEXT` | Request an answer of this shape, in `structured_output`. |

The prompt goes to `stdin`, not into `args`, as in the recorded real call.
Thus, the arguments stay the same for each prompt.
The script checks the run status and exit code before it parses standard output.
It then checks `structured_output` against the same schema.
The fixed answer permits only `yes` or `no`.
It never supplies a program name or arbitrary command argument.

The next example is `claude-question`, in `~/ravel-chapter-4/claude-question`
with a `bin` subdirectory. It has four files:

| Path in `claude-question` | Purpose |
|---|---|
| `settings.json` | Names the harness entry `claude` and permits one harness run. The first command block copies it into the private home. |
| `config.rhai` | Asks the harness run a question, checks the answer and selects `fake-agent` |
| `bin/claude` | A fake Claude Code. `bin` comes first on the search path, so a real Claude Code does not run. |
| `bin/fake-agent` | The same fake agent as in `fixed-route` |

The fake `claude` writes two files when it runs: `call.args` and
`call.prompt`. The first command block also writes the result to a file,
`result`. Then it prints the three files. The second command block first makes
an empty file, `invalid-answer`, with `touch`. That file makes the fake give
an answer that the schema does not permit.

Complete example: ask through a fake Claude Code and refuse an invalid answer.

**`settings.json`**

```json
{"harnesses":{"claude":{"program":"claude"}},"budgets":{"harnessRuns":1}}
```

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "claude-question-1";

fn refuse() {
    #{ status: "refused", code: "no_answer", message: "The call gave no valid answer.",
       remedy: "Check the harness and its output." }
}

fn select(request, context) {
    let schema = #{ type: "object",
                    properties: #{ prime: #{ type: "string", enum: ["yes", "no"] } },
                    required: ["prime"], additionalProperties: false };
    let run = ravel::run_harness(#{
        harness: "claude",
        args: ["-p", "--model", "haiku", "--tools", "", "--no-session-persistence",
               "--output-format", "json", "--json-schema", ravel::json_encode(schema)],
        stdin: "Is 17 a prime number? Answer with the JSON object only.",
        timeoutMs: 20000,
    });
    if run.status != "exited" || run.exitCode != 0 { return refuse(); }
    let answer = ();
    try {
        let document = ravel::json_parse(run.stdout);
        if type_of(document) != "map" || !document.contains("structured_output") {
            return refuse();
        }
        answer = ravel::json_check(document.structured_output, schema);
    } catch (error) { return refuse(); }
    let result = if answer.prime == "yes" { "prime" } else { "not-prime" };
    #{ status: "selected", program: "fake-agent", args: [result],
       provider: "example", model: "fake", effort: "low", reason: "checked model answer" }
}
```

**`bin/claude`**

```sh
#!/bin/sh
printf '%s\n' "$@" > call.args
cat > call.prompt
if test -f invalid-answer; then
    printf '%s\n' '{"structured_output":{"prime":"perhaps"}}'
else
    printf '%s\n' '{"structured_output":{"prime":"yes"}}'
fi
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello" > result && cat call.args && cat call.prompt && printf '\n' && cat result
```

**Standard output, without a final newline**

```text
-p
--model
haiku
--tools

--no-session-persistence
--output-format
json
--json-schema
{"additionalProperties":false,"properties":{"prime":{"enum":["yes","no"],"type":"string"}},"required":["prime"],"type":"object"}
Is 17 a prime number? Answer with the JSON object only.
prime
```

**Exit status**

```text
0
```

**Command**

```console
$ touch invalid-answer
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
config_refused
```

**Exit status**

```text
3
```

Follow the first command block from the script to the result:

| Order | Who acts | What happens in this example |
|---|---|---|
| 1 | The configuration script | Calls `ravel::run_harness` for the entry `claude`, with the arguments in the table above and the question on `stdin` |
| 2 | The fake `claude` | Writes its arguments to `call.args` and its standard input to `call.prompt`. Then it prints `{"structured_output":{"prime":"yes"}}`. |
| 3 | The Ravel CLI | Gives the script a map with `status: "exited"`, `exitCode: 0` and that line in `stdout` |
| 4 | The configuration script | Parses `stdout`, checks `structured_output` against the schema and selects `fake-agent` with the argument `prime` |
| 5 | `fake-agent` | Sends `prime` with `end-session`, and `prime` becomes the result of `ravel start-session` |

The first command block then prints `call.args`, `call.prompt` and the result. Thus, the
output shows each argument that a real Claude Code would receive. In the
second command, the fake answers `perhaps`. The schema permits only `yes` or
`no`, so the script refuses.

`type_of` reports the Rhai type name.
The map check and `contains` test protect the read of `structured_output`.
The schema rejects `perhaps`, and the script returns its own refusal.
A different script could choose a fixed fallback after an unusable answer.
It still cannot catch an exhausted
[evaluation budget](02-actors-and-words.md#evaluation-and-supervision).
An evaluation budget is a limit that you set in the settings for one
evaluation, such as the number of harness runs in `budgets.harnessRuns`.

The shipped `llm.rhai` asks Claude Code to classify the caller's prompt. It
sends `request.originalPrompt` to the classifier.
It limits the answer to keys from the route table of `static.rhai`.
After an unusable answer, it uses the caller's kind instead.
It imports `static.rhai`, so both files must be present.

## Ask a model through Codex

Do you use Codex and not Claude Code? The recipe is almost the same. The
script asks for structured output, reads the answer from standard output and
checks it with the same schema.

One difference matters.
Codex accepts its structured-output schema as a file path, not as text.
Keep that file where the run can read it.
The script cannot write the schema file itself.
The example places it in the caller's working directory and reads the same
file with `ravel::read_json`.

The recipe uses Codex CLI 0.164.0.
The author checked the installed help on 2026-10-26.
The repository's real smoke call succeeded with that version and these arguments.
The final message on standard output is the JSON answer.
This recipe does not use `--json`, which selects JSON Lines events instead.

| Argument | Purpose |
|---|---|
| `exec` | Run noninteractively. |
| `--ephemeral` | Do not retain the session in local history. |
| `--skip-git-repo-check` | Permit the call outside a Git repository. |
| `--sandbox read-only` | Select the harness's read-only sandbox. |
| `-c 'model_reasoning_effort="low"'` | Set the effort for this call. |
| `--output-schema PATH` | Read the answer schema from a file. |
| `-` | Read the prompt from standard input. |

The next example is `codex-question`, in `~/ravel-chapter-4/codex-question`
with a `bin` subdirectory. It has the shape of `claude-question`, with two
differences:

- `settings.json` names the harness entry `codex`, and `bin/codex` is a fake
  Codex in place of `bin/claude`.
- A fifth file, `answer-schema.json`, holds the schema of the answer. The
  configuration script reads it, and the fake Codex receives its path.

The second command block makes an empty file, `failed-call`, with `touch`.
That file makes the fake Codex fail. In the output of the first block,
`<scratch>/work` stands for the example's directory. In your shell, the line
shows the absolute path of your `codex-question` directory.

Complete example: ask through a fake Codex, check the schema and inspect every
argument.

**`settings.json`**

```json
{"harnesses":{"codex":{"program":"codex"}},"budgets":{"harnessRuns":1}}
```

**`answer-schema.json`**

```json
{"type":"object","properties":{"prime":{"type":"string","enum":["yes","no"]}},"required":["prime"],"additionalProperties":false}
```

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "codex-question-1";

fn refuse() {
    #{ status: "refused", code: "no_answer", message: "The call gave no valid answer.",
       remedy: "Check the harness and its output." }
}

fn select(request, context) {
    let schema = ravel::read_json("answer-schema.json").value;
    let run = ravel::run_harness(#{
        harness: "codex",
        args: ["exec", "--ephemeral", "--skip-git-repo-check", "--sandbox", "read-only",
               "-c", "model_reasoning_effort=\"low\"", "--output-schema",
               ravel::path_resolve(request.cwd, "answer-schema.json"), "-"],
        stdin: "Is 17 a prime number? Answer with the JSON object only.",
        timeoutMs: 20000,
    });
    if run.status != "exited" || run.exitCode != 0 { return refuse(); }
    let answer = ();
    try { answer = ravel::json_check(ravel::json_parse(run.stdout), schema); }
    catch (error) { return refuse(); }
    let result = if answer.prime == "yes" { "prime" } else { "not-prime" };
    #{ status: "selected", program: "fake-agent", args: [result],
       provider: "example", model: "fake", effort: "low", reason: "checked model answer" }
}
```

**`bin/codex`**

```sh
#!/bin/sh
printf '%s\n' "$@" > call.args
cat > call.prompt
if test -f failed-call; then
    printf '%s\n' 'The fake call failed.' >&2
    exit 1
fi
printf '%s\n' '{"prime":"yes"}'
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello" > result && cat call.args && cat call.prompt && printf '\n' && cat result
```

**Standard output, without a final newline**

```text
exec
--ephemeral
--skip-git-repo-check
--sandbox
read-only
-c
model_reasoning_effort="low"
--output-schema
<scratch>/work/answer-schema.json
-
Is 17 a prime number? Answer with the JSON object only.
prime
```

**Exit status**

```text
0
```

**Command**

```console
$ touch failed-call
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
config_refused
```

**Exit status**

```text
3
```

In the first command block, the fake `codex` printed `{"prime":"yes"}`. The script
checked that answer against the schema from `answer-schema.json`, and
`fake-agent` sent `prime` as the result. The output also shows the full path of
the schema file. The script made that path from `request.cwd`, so Codex can
find the file when the caller runs the example from its directory.

In the second command block, the fake failed.
The failed run returns `status: "exited"` and `exitCode: 1` to the script.
The script then refuses. Ravel itself assigns no meaning to the harness's text.
A retry would need another run in the budget and time before the whole deadline.

The fake examples prove the arguments, prompt placement and the checks of each answer.
They do not prove that a real harness accepts the arguments. Two opt-in tasks
in a Ravel checkout prove the recipes with real calls:

| Task | What it proves | What it needs |
|---|---|---|
| `task harness:smoke:claude` | The Claude Code recipe, with a real call | An installed Claude Code with the owner's sign-in. Task and the Rust toolchain, because the task builds Ravel from the checkout. The call spends subscription usage. |
| `task harness:smoke:codex` | The Codex recipe, with a real call | An installed Codex with the owner's sign-in. Task and the Rust toolchain, because the task builds Ravel from the checkout. The call spends subscription usage. |

Run the appropriate task in the root directory of the checkout after a
harness upgrade.
[Chapter 10, “Prepare a checkout”](10-develop-ravel.md#prepare-a-checkout),
gives the tools that a checkout needs.

Those tasks use a temporary configuration and links to the harness's sign-in
store. They do not need edits to your own settings.
They run unconfined. Confinement can deny writes to a harness's sign-in
store, and Ravel does not widen confinement for authentication.
A schema beside the configuration script is also unsuitable for a confined
harness run, because confinement refuses grants that overlap the directory of
the script.

The harness owns its authentication.
See the [Claude Code setup documentation](https://code.claude.com/docs/en/setup)
and [Codex authentication documentation](https://learn.chatgpt.com/docs/auth)
for account setup. This chapter assumes an installed, signed-in harness for real calls.
Ravel reports no token usage. Its notice lists run metadata, not the prompt or answer.
A script that copies an answer into diagnostics or delivered context exposes it there.

## Select Claude Code or Codex for the caller's work

The harness runs above answer a question for the configuration script.
Usually, you also want a harness to do the caller's work, such as the review
of chapter 1. Then the harness is the managed process: the program that Ravel
starts and supervises for the session. An interactive harness waits for more
input when its LLM completes the task. Thus, the caller's prompt tells the LLM
to run `ravel end-session RESULT`, and that command ends the session.

A final interactive harness is the managed process, not a harness run.
Select its `program` and `args` directly.
This selection needs no `harnesses` entry or harness-run budget.

The caller's prompt gives the complete task. In the review of chapter 1, the
prompt named the two files to review. It gave two permitted results,
`approved` and `changes-needed`, and the condition for each result. It told
the LLM to send the result with `ravel end-session`. The orchestrator then
acted on each result.

These values are the caller's convention. Ravel assigns
them no meaning, and it returns the value unchanged after cleanup.

Your script passes that prompt unchanged if its route uses `request.prompt`.
Ravel does not add the instruction to end the session.

How does `ravel end-session` find the session? Ravel puts the path of the
session's result file in the `RAVEL_EXIT_FILE` environment variable of the
harness. The harness runs each command of the LLM in a child process, for
example a shell. Each child inherits `RAVEL_EXIT_FILE` from its parent unless
the parent changes it. Thus, `ravel end-session` finds the path in its own
environment.
[Chapter 1, “How `end-session` finds its session”](01-what-ravel-is-for.md#how-end-session-finds-its-session),
shows the paths of a real nested run.

Follow the review with a real Claude Code, from the selection to the result:

| Order | Who acts | What happens in the review |
|---|---|---|
| 1 | The configuration script | Selects `claude` with the caller's prompt as its last argument |
| 2 | The Ravel CLI | Writes the routing notice, puts the path of the result file in `RAVEL_EXIT_FILE` and starts `claude` |
| 3 | The LLM in Claude Code | Reviews the two files, finds a problem and writes `review-notes.md`. Then it asks Claude Code to run `ravel end-session changes-needed`. |
| 4 | `ravel end-session` | Finds the path in `RAVEL_EXIT_FILE`, writes `changes-needed` there and asks Ravel to end the session |
| 5 | The Ravel CLI | Stops Claude Code and the rest of its process group, then completes cleanup |
| 6 | The Ravel CLI | Writes `changes-needed` on standard output. It is the result of `ravel start-session`. |
| 7 | The orchestrator | Reads `review-notes.md`, then requests a session of the kind `fix` |

| Intended use | Action of the configuration script | How it completes |
|---|---|---|
| Ask a question before selection | Call `ravel::run_harness` with noninteractive arguments. | Read its output, status or run result. |
| Run an interactive LLM session | Return a selected map for the harness. | A program in the session calls `end-session`. |

The noninteractive recipes above are for the first row.
The next example uses interactive arguments from the same installed versions.
The author checked those arguments against their help on 2026-10-26.
Both routes use the harness's configured model default.
Only the Claude Code route sets an explicit effort.

`--` separates the prompt from options, even when its first character is `-`.
Codex's `--no-daemon` makes Codex run without its shared background server.
Ravel supervises the selected process group. A process that leaves that group
is outside its cleanup.

The next example is `interactive-routes`, in
`~/ravel-chapter-4/interactive-routes` with a `bin` subdirectory. It has three
files: `config.rhai`, a fake Claude Code at `bin/claude` and a fake Codex at
`bin/codex`. `bin` comes first on the search path, so neither real harness
runs. Each fake sends all its arguments as the result, one per line, each in
`<` and `>`.

The first command block is a `for` loop over two kinds, `claude-demo` and
`codex-demo`. Type it as one command over five lines. Your shell shows a
continuation prompt for the second to fifth lines. The second block asks with
the kind `other`.

Complete example: pass an instruction to end the session to two fake interactive
harnesses.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "interactive-1";

fn select(request, context) {
    let program = "";
    let args = [];
    let effort = "configured-default";
    if request.kind == "claude-demo" {
        program = "claude";
        effort = "medium";
        args = ["--effort", effort, "--", request.prompt];
    } else if request.kind == "codex-demo" {
        program = "codex";
        args = ["--no-daemon", "--no-alt-screen", "--sandbox", "workspace-write",
                "--ask-for-approval", "on-request", "--", request.prompt];
    } else {
        return #{ status: "refused", code: "unsupported_kind",
                  message: "No interactive route matches this kind.",
                  remedy: "Use claude-demo or codex-demo." };
    }
    #{ status: "selected", program: program, args: args,
       provider: "agent-configured", model: "configured-default",
       effort: effort, reason: "attended harness route" }
}
```

**`bin/claude`**

```sh
#!/bin/sh
set -eu
result=$(printf '<%s>\n' "$@")
ravel end-session -- "$result"
```

**`bin/codex`**

```sh
#!/bin/sh
set -eu
result=$(printf '<%s>\n' "$@")
ravel end-session -- "$result"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/*
$ for kind in claude-demo codex-demo; do
    printf '%s\n' "$kind"
    env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind "$kind" --prompt 'Review the change. If it is correct, run ravel end-session approved. Otherwise, run ravel end-session changes-needed.' || break
    printf '\n'
  done
```

**Standard output**

```text
claude-demo
<--effort>
<medium>
<-->
<Review the change. If it is correct, run ravel end-session approved. Otherwise, run ravel end-session changes-needed.>
codex-demo
<--no-daemon>
<--no-alt-screen>
<--sandbox>
<workspace-write>
<--ask-for-approval>
<on-request>
<-->
<Review the change. If it is correct, run ravel end-session approved. Otherwise, run ravel end-session changes-needed.>
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config config.rhai --kind other --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
config_refused
```

**Exit status**

```text
3
```

The fakes return their arguments. They do not act on the prompt.
This check establishes prompt delivery, not whether an LLM obeys the instruction.
Real interactive harnesses need a foreground terminal.
Ravel can lend that terminal to the managed process and recover it afterwards.

The harness needs permission to run `ravel` and write its inherited result path.
Keep `ravel` on its `PATH`, or name the installed absolute path in the prompt.
The harness's own sandbox and approvals are independent of Ravel's `--confine`.
A natural exit without `end-session`, even exit status 0, returns no result to the caller.

## Copy a configuration script and its files

You may want to move a configuration script to another account, or start from
one of the example scripts that Ravel ships. A script can be more than one
file. It can import modules, read data files and need settings entries.

Copy a configuration script with its imported modules and data files.
Preserve relative import paths, and check paths relative to `request.cwd` separately.
Copy only the settings entries and environment grants that the new script needs.
The script does not carry its own settings file through `--config`.

The shipped examples are under `share/ravel/examples/` in a Ravel 2.0.0 installation.
In a checkout, their source is `crates/ravel/examples/`.
The examples are editable copies, not embedded imports.

| Example | Files and purpose |
|---|---|
| `static.rhai` | A fixed route table with placeholder commands and labels. |
| `sample.rhai` | A checkout choice. It also needs `static.rhai`. |
| `llm.rhai` | An LLM-assisted choice through Claude Code. It needs `static.rhai` and harness settings. |

Before real use, replace placeholder commands and test every route with fake
agents. Check that the script refuses unknown kinds as intended.
Check that absent data and unsuccessful harness runs produce the outcome you chose.
Then put the script at `$HOME/.config/ravel/config.rhai`, or select it with
`--config`.

## Review your configuration script before real use

In short, the caller asks with a kind and a prompt. Ravel evaluates your Rhai
configuration script in the Rhai worker, a second copy of the `ravel`
executable. The script can read measured files, read granted environment
variables and run harnesses through `ravel::` functions.
Each of these host capabilities asks the Ravel CLI to do the work, within the
limits of your settings. Then `select` returns a selection or a refusal.

After a selection, Ravel reaps the worker and starts the selected program as
the managed process. Ravel writes the routing notice on the caller's standard
error. The text notice comes before the start of the program, and the JSON
notice follows the supervision attempt. The program sends a result with
`ravel end-session`, as the caller's prompt tells it. After cleanup, that
result becomes the result of `ravel start-session`, unchanged. The caller
gives the result its meaning.

Before you use a configuration script with real harnesses, test each of these
points with fake agents:

| Point to test | What a good script does |
|---|---|
| Each route | Starts the program with the arguments that you expect, and the fake agent sends them back |
| Each unknown kind | Refuses with a `code`, a `message` and a `remedy` that the caller can act on |
| Absent optional inputs | Gives the outcome that you chose, without an error from an absent map property |
| Absent or malformed data files | Refuses or uses a default, as you chose |
| Each harness run | Checks the status, the exit code and the answer before the script uses the answer |
| An unusable answer from a model | Refuses or uses a fixed fallback, as you chose |
| The interactive routes | Pass the caller's prompt, with its instruction to run `ravel end-session` |
