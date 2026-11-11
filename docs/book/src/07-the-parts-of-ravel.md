# The parts of Ravel

Do you want to change Ravel, or find the part that caused a failure? Then you
need to know where each part of Ravel lives and why it is a separate part.
This chapter gives you that map. It starts with one session that makes each
part visible in its output. Then it names the parts, gives their source
locations and explains how they work together.

In [chapter 3](03-a-complete-session.md#run-the-first-session) you ran the
first session in `~/ravel-first-session`. Its configuration script selected
`fake-agent`, and the caller received `finished`. That session used every part
of Ravel, but its output showed only the routing notice and the result. The
example of this chapter adds one harness run. Its fake programs then write down
who started them and what they received.

These are the words that this chapter uses. Each row gives the value that the
word has in the example of this chapter.

<!-- tc: depart synonym-rotation because launch names the runner's launch of a job, while start names other process creation -->

| Word | What it names | In the example |
|---|---|---|
| [Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) | The `ravel` command-line program. It checks the request, runs the programs and decides the outcome. | The process `ravel start-session --kind demo …` |
| [Configuration script](02-actors-and-words.md#caller-and-owner) | The owner's Rhai file that selects a program or refuses | `config.rhai` |
| [Rhai worker](02-actors-and-words.md#evaluation-and-supervision) | A second copy of the `ravel` executable that evaluates the configuration script | It runs `select` in `config.rhai` |
| [Host capability](02-actors-and-words.md#evaluation-and-supervision) | An operation that the script asks the Ravel CLI to do, because the worker cannot do it | `ravel::run_harness` |
| [Harness run](02-actors-and-words.md#evaluation-and-supervision) | One run of a program from the `harnesses` settings, during evaluation. Its output returns to the script. | `fake-advisor`, which prints `short` |
| [Managed process](02-actors-and-words.md#the-managed-process-and-its-result) | The program that the script selects. It does the caller's work. | `fake-agent` |
| [Result channel](02-actors-and-words.md#the-managed-process-and-its-result) | A private temporary directory with one path for a result file. The path goes to a program in `RAVEL_EXIT_FILE`. | One for `fake-advisor`, one for `fake-agent` |
| [Runner](02-actors-and-words.md#evaluation-and-supervision) | The Rust library code in the Ravel CLI that launches and supervises a job | It runs both fakes |

## Watch the parts in one session

The example of this chapter is `ravel-parts`. Its directory is
`~/ravel-parts`. It has the same shape as the first session of chapter 3. It
adds a settings file that names one harness, and the fake harness itself.

When you finish the steps, the directory holds these paths:

```text
~/ravel-parts/
├── settings.json       you save it
├── config.rhai         you save it
├── bin/                you make it
│   ├── fake-advisor    you save it
│   └── fake-agent      you save it
├── home/               the first command makes it
├── tmp/                the first command makes it
└── …                   the fakes and the command write five files
```

Each path has one purpose:

| Path | Who makes it | Purpose |
|---|---|---|
| `settings.json` | You | Names the harness entry `advisor`, with the program `fake-advisor`, and permits one harness run. The first command copies it into the private home. |
| `config.rhai` | You | Asks the harness run for advice. Then it selects `fake-agent` and gives it the advice as its argument. |
| `bin/fake-advisor` | You | The fake harness. It writes `advisor.txt` and `advisor.path`, prints `short` and sends the result `advice`. |
| `bin/fake-agent` | You | The fake agent. It writes `agent.txt` and `agent.path`, then sends the result `finished`. |
| `home/` | The first command | An empty home directory. The first command puts the settings file at `home/.config/ravel/settings.json`. |
| `tmp/` | The first command | A private temporary directory. Ravel makes each result channel here. |
| `result` | The second command | The standard output of `ravel start-session`, which is the result of the session |

Do these steps in the shell where `ravel --version` succeeded in chapter 3:

1. Make the example directory and its `bin` subdirectory, then `cd` into the
   example directory. The fragment after these steps gives the two commands.
2. Save the four files of the complete example at their labelled paths.
3. Type each command of the complete example without its first `$ `.
4. Compare the output and the exit status with the example.

Fragment: make the example directory and `cd` into it. This block omits the
four files. The book checker makes its own directory for each example and does
not run this block.

```sh
mkdir -p ~/ravel-parts/bin
cd ~/ravel-parts
```

Complete example: a configuration script asks a fake harness for advice, then
selects a fake agent. Both fakes write down who started them.

**`settings.json`**

```json
{"harnesses":{"advisor":{"program":"fake-advisor"}},"budgets":{"harnessRuns":1}}
```

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "parts";

fn select(request, context) {
    let run = ravel::run_harness(#{ harness: "advisor", args: [] });
    print(`The advisor run ended: ${run.status}, result ${run.result}.`);
    #{
        status: "selected",
        program: "fake-agent",
        args: [run.stdout],
        provider: "local",
        model: "fake",
        effort: "none",
        reason: "Show each part of Ravel.",
    }
}
```

**`bin/fake-advisor`**

```sh
#!/bin/sh
{
  echo "advisor: parent is $(ps -o args= -p "$PPID" | cut -d ' ' -f 1-2)"
  echo "advisor: RAVEL_CONFIG_RUN is ${RAVEL_CONFIG_RUN-absent}"
} > advisor.txt
printf '%s\n' "$RAVEL_EXIT_FILE" > advisor.path
printf 'short'
ravel end-session -- "advice"
```

**`bin/fake-agent`**

```sh
#!/bin/sh
{
  echo "agent: parent is $(ps -o args= -p "$PPID" | cut -d ' ' -f 1-2)"
  echo "agent: RAVEL_CONFIG_RUN is ${RAVEL_CONFIG_RUN-absent}"
  echo "agent: argument is $1"
} > agent.txt
printf '%s\n' "$RAVEL_EXIT_FILE" > agent.path
ravel end-session -- "finished"
```

**Command**

```console
$ mkdir -p home/.config/ravel tmp && chmod +x bin/* && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config config.rhai --prompt 'Show the parts of Ravel.' > result
```

**Standard output**

```text
```

**Standard error includes**

```text
config stdout: The advisor run ended: exited, result advice.
ravel: starting provider local, model fake, effort none for kind "demo"
```

**Exit status**

```text
0
```

**Command**

```console
$ cat advisor.txt agent.txt && cat result && printf '\n'
$ test "$(cat advisor.path)" != "$(cat agent.path)" && echo "The two result paths differ."
$ rmdir tmp && echo "The directory tmp is empty again."
```

**Standard output**

```text
advisor: parent is ravel start-session
advisor: RAVEL_CONFIG_RUN is 1
agent: parent is ravel start-session
agent: RAVEL_CONFIG_RUN is absent
agent: argument is short
finished
The two result paths differ.
The directory tmp is empty again.
```

**Exit status**

```text
0
```

The first command block differs from chapter 3's command in two parts. Its
first line also copies `settings.json` into the private home, because Ravel
reads settings only from `$HOME/.config/ravel/settings.json`. Its second line
sends the result to the file `result`, so that the next block can show it beside
the other files. On macOS, standard error can also show `ravel: stop attempt`
lines. [Chapter 3](03-a-complete-session.md#run-the-first-session) explains
these operation reports.

The second command block only reads what the session left. `ps -o args=`
prints the arguments of a process, and `$PPID` is the process ID of the fake's
parent. `cut` keeps the first two words. `rmdir` removes a directory only when
it is empty, so its success shows that Ravel removed both result channels.

Each line of the output comes from one part of Ravel:

| Output | What it shows | Part of Ravel |
|---|---|---|
| `config stdout: The advisor run ended: exited, result advice.` | The script ran, called a host capability and received the run as data. Ravel shows the script's `print` output on its own standard error. | The Rhai worker and the host bridge |
| `advisor: parent is ravel start-session` | The Ravel CLI started the harness run. The worker asked for the run but did not start it. | The host bridge and the runner |
| `advisor: RAVEL_CONFIG_RUN is 1` | A harness run receives a marker. A `start-session` inside the run would refuse. | The harness-run code of the Ravel CLI |
| `ravel: starting provider local, …` | The Ravel CLI checked the selection and wrote the routing notice, after the worker ended | The Ravel CLI |
| `agent: parent is ravel start-session` | The Ravel CLI launched the managed process too | The runner |
| `agent: RAVEL_CONFIG_RUN is absent` | The managed process receives no marker, so it can start a nested session | The Ravel CLI |
| `agent: argument is short` | The advice went from the harness run, through the script, into the arguments of the managed process | The host bridge and the Ravel CLI |
| `finished` | The result of the managed process reached the caller after cleanup | The result channel |
| `The two result paths differ.` | Each program received its own result channel | The result channel |
| `The directory tmp is empty again.` | Ravel removed both result channels | The result channel and the Ravel CLI |

Thus, the Ravel CLI starts every program in a session. The Rhai worker starts
none. It computes, and it asks the Ravel CLI for anything else. The rest of
this chapter explains each part and why it is separate.

## Find each part in the source

The source of Ravel is one Cargo workspace with two Rust packages: `ravel` and
`keyed-launch`. A package is not always a process. The runner is a package,
but it runs inside the Ravel CLI. The Rhai worker is no package of its own,
but it is a separate process. The table gives each part with its source
location and the process where it runs. Paths are relative to the root of a
checkout.

| Part | Source location | Where it runs | In the example |
|---|---|---|---|
| Ravel CLI | `crates/ravel/src/main.rs`, `run.rs`, `choice.rs` and the other modules of the executable | The `ravel start-session` process | Checks the request, writes the routing notice and returns `finished` |
| Rhai worker | `crates/ravel/src/rhai_worker/`, mainly `role.rs`, `engine.rs` and `bridge.rs` | A second `ravel` process, during evaluation only | Runs `select` and calls `ravel::run_harness` |
| Host bridge, CLI side | `crates/ravel/src/rhai_worker/front.rs`, `serve.rs`, `host.rs`, `reader.rs` and `harness.rs` | Inside the Ravel CLI, during evaluation only | Receives the request for the `advisor` run and starts it |
| Skill preparation | `crates/ravel/src/skills/` | Inside the Ravel CLI, before the worker starts | Does nothing, because no skill source is configured |
| Runner | `crates/keyed-launch/src/` | Inside the Ravel CLI | Runs `fake-advisor`, then `fake-agent` |
| Result channel | `crates/ravel/src/completion.rs`, exported by `crates/ravel/src/lib.rs` | Inside the Ravel CLI and inside each `ravel end-session` process | Carries `advice` to the script and `finished` to the caller |

The figure shows the same parts and their relations. The dashed outline is the
Ravel CLI process.

![Source responsibilities: crates/ravel checks requests, controls host calls and decides the final outcome. crates/keyed-launch supplies the runner inside that CLI process. A separate Rhai worker uses the same binary to evaluate the configuration script and request host capabilities. The runner supervises each evaluation harness run, whose data returns to the configuration script. Every host call and worker cleanup ends before the runner launches the final managed process.](figures/source-components.svg)

[Open the source-responsibility view at full size](figures/source-components.svg).

The runner is library code, not a service. `ravel end-session` is another run
of the same `ravel` executable. It is not a request to a daemon that stays
alive. [Chapter 2](02-actors-and-words.md#the-managed-process-and-its-result)
shows the same processes in its process-and-authority figure.

## The Ravel CLI coordinates the session

The Ravel CLI is the only part that knows every condition for a successful
session. It connects the configuration script's choice to the supervision of
the job, and it decides what the caller receives.

`main` in `crates/ravel/src/main.rs` chooses the work of the process. First it
compares its own `argv[0]` with `ravel-rhai-config-worker`. On a match, the
process is the Rhai worker, and `main` gives control to the worker code. The
worker section below explains this name. Otherwise `main` reads the command
line and runs one of three verbs:

| Verb | Function that `main` calls | What it does |
|---|---|---|
| `start-session` | `run::run` in `crates/ravel/src/run.rs` | Routes the request and supervises one session |
| `end-session RESULT` | `end_session` in `crates/ravel/src/completion.rs` | Sends one result to the enclosing session |
| `skills resolve` | `skills::command::run` in `crates/ravel/src/skills/command.rs` | Captures the skill sources and prints the instructions that a prompt would receive, without a session |

`start-session` does its work in a fixed order. Each step needs the step
before it. The last column gives the step in the example.

| Step | What the Ravel CLI does | Where | In the example |
|---|---|---|---|
| 1 | Refuses when `RAVEL_CONFIG_RUN` is in its environment | `run::run` | The caller's environment has no marker |
| 2 | Reads the settings and the caller inputs, then finds the configuration script | `choice::prepare` | Reads `home/.config/ravel/settings.json` and finds `config.rhai` |
| 3 | Resolves the skill sources and prepares the prompt | `skills::launch::prepare` | Delivery is off, so the prompt stays unchanged |
| 4 | Starts the Rhai worker, answers its host calls, checks its reply and reaps it. Then it finds the selected program on `PATH`. | `choice::choose_prepared` | Starts `fake-advisor` for the script, receives the selection of `fake-agent` and finds `bin/fake-agent` |
| 5 | Makes the result channel, writes the routing notice and launches the managed process | `run::run` | Launches `fake-agent` with its own `RAVEL_EXIT_FILE` |
| 6 | Reads the result after cleanup, decides the outcome and writes the result | `run::run` | Writes `finished` and exits with status 0 |

A failure stops the order at its step. For example, a skill refusal in step 3
starts no worker and makes no result channel.

The Ravel CLI also keeps the caller's environment, terminal and standard
output while the worker runs. It reserves standard output for the result.
The output of the managed process and the diagnostics of Ravel go to standard
error. Thus, a configuration script cannot write the result of the session.
In the example, the script's `print` output appears on standard error with the
prefix `config stdout:`.

The `ravel` package has a library target and an executable target. The library
exports `SessionChannel`, `publish_result`, `end_session` and the result
constants. The executable uses this library and `keyed-launch`. This division
lets the result-channel tests use a channel with the runner, with no
configuration script.

Neither the worker nor the runner knows every condition for success. For
example, the runner can report a reaped child while the Ravel CLI still has a
temporary directory to remove. Thus, a change to the final output or to the
order of failures belongs in the coordination code of the Ravel CLI, mainly in
`run.rs`.

The table gives each decision one owner:

| Decision | Owner | What crosses the boundary | In the example |
|---|---|---|---|
| Which program to select | Configuration script | A command or a [refusal](02-actors-and-words.md#evaluation-and-supervision) | `fake-agent` with the argument `short` |
| Whether the script's reply is valid | Ravel CLI | A checked command, or no launch | The reply has every required field |
| How to launch and stop a job | Runner | Launch options from the Ravel CLI | Two jobs, each with its own channel |
| Whether to return the result | Ravel CLI | Job observations and cleanup reports | Both jobs ended, and cleanup succeeded |
| What the result means | Caller | The result, unchanged | `finished` |

## The Rhai worker evaluates the script in its own process

Why does Ravel run the configuration script in a separate process? Because the
Ravel CLI must stay in control while the script runs, whatever the script
does. The decision record “Configuration evaluation precedes the launch”
gives the reasons. Its file in the repository is
`docs/adr/configuration-evaluation-precedes-the-launch.md`. The table restates
the reasons in your terms:

| Problem that a thread inside the Ravel CLI would have | What the separate process gives |
|---|---|
| A thread cannot stop a Rhai function that does not yield. Rhai 1.27.0 has two: `sleep`, and `pad` when its fill string is empty. | The Ravel CLI kills the worker at the deadline, whatever the script runs |
| Rhai limits the size of each value, but not the heap. A memory exhaustion or a stack overflow on a thread stops the whole Ravel CLI without cleanup. | Such a failure stops only the worker. The Ravel CLI reports it and launches nothing. |
| Host capabilities inside the worker would make the worker the parent of each harness run | The Ravel CLI does each host capability, so a kill of the worker leaves no harness run without a parent |

The price is a request and reply protocol for every host call, and the code
that starts, watches and reaps the worker. The record says when to reopen the
decision: when Rhai can interrupt every native function and Ravel can limit the
heap of one thread.

How does the Ravel CLI start the worker? It starts its own executable again,
with `argv[0]` set to `ravel-rhai-config-worker`. On Linux it executes
`/proc/self/exe`, which names the image that runs now. On macOS it executes the
canonical path of its own executable.

The name `ravel-rhai-config-worker` is
an internal process identity. It is not an installed file or a command for you
to run. `crates/ravel/src/rhai_worker/front.rs` holds this launch, and
`crates/ravel/src/worker.rs` supervises the process.

The worker receives very little from the Ravel CLI:

| The worker has | The worker does not have |
|---|---|
| A private empty directory with mode 0700, as its working directory | The caller's environment. Its environment is empty. |
| Standard input from `/dev/null` | `RAVEL_EXIT_FILE`, so it cannot end a session |
| A private socket at file descriptor 3 | `RAVEL_SKILL_DIRECTORIES`, or any other inherited descriptor |
| Standard output and standard error, which the Ravel CLI reads with a size limit | Access to files, processes or the network. The Rhai engine has no such functions. |

The first message on the socket is the worker's `hello`. It holds the protocol
number 1, the package version, the build identity and the Rhai version. The
Ravel CLI compares these values with its own before it sends the request.

A different build identity refuses as `worker_identity_mismatch`. A worker that
cannot start refuses as `worker_missing`. The identity shows that the worker
is the same build as the Ravel CLI. It does not prove that the installation
belongs to its owner.

The worker runs the script on one thread with a fixed set of engine limits.
Chapter 6 lists them. An allocator in
`crates/ravel/src/rhai_worker/heap.rs` counts each live byte of the worker
and limits its heap to 1 GiB.
When an allocation would pass that limit, the worker reports `memory_limit`
and exits.

After it sends its reply, the worker exits. The Ravel CLI reaps it and removes
its private directory. Only then can the managed process launch. If the reap
or the removal fails, nothing launches. Thus, no code of the configuration
script runs while the managed process runs.

Ravel 1.x ran TypeScript configuration scripts in a Bun worker. Ravel 2.0.0
removed that worker. Thus, the installation has no JavaScript runtime, no SDK
and no declaration file. It has no separate worker executable under `libexec`
either.

The decision record “The configuration script uses Rhai”, in
`docs/adr/configuration-script-uses-rhai.md`, gives the reason. Ravel chose a small embedded language with no effects of its
own, in place of a typed language with packages. [Chapter 4](04-write-a-configuration-script.md)
teaches the language.

## The host bridge carries each request to the Ravel CLI

The worker cannot read a file or start a program. So how did the example's
script run `fake-advisor`? It asked the Ravel CLI through the **host bridge**:
the code on both sides of the private socket that carries host calls and their
replies. Follow the `advisor` call:

| Order | Who acts | Where | What happens in the example |
|---|---|---|---|
| 1 | The configuration script | `config.rhai` | Calls `ravel::run_harness(#{ harness: "advisor", args: [] })` |
| 2 | The Rhai worker | `rhai_worker/bridge.rs` | Sends a `host` frame with the call `run_harness`, then waits. The script sees an ordinary call that returns a value. |
| 3 | The Ravel CLI, runtime thread | `rhai_worker/serve.rs` | Receives the frame and gives the call to the reader thread. It continues to watch the deadline, cancellation, the output limit and the worker. |
| 4 | The Ravel CLI, reader thread | `rhai_worker/reader.rs`, `harness.rs` | Finds the program `fake-advisor` for the entry `advisor`, makes a result channel and runs the program through the runner |
| 5 | The runner | `crates/keyed-launch/src/run.rs`, `run_captured` | Supervises `fake-advisor` in a new POSIX session, captures its output and stops its process group after it exits |
| 6 | The Ravel CLI | `rhai_worker/harness.rs`, `host.rs` | Removes the run's result channel and sends a `reply` frame with the run's map |
| 7 | The Rhai worker | `rhai_worker/bridge.rs` | Gives the map to the script. `run.stdout` is `short`, and `run.result` is `advice`. |

Chapter 4 shows the same sequence for a fake Claude Code. The figure is
general: its harness is named `claude`, but its steps are the steps of the
table.

![The Rhai worker requests a claude harness run from the Ravel CLI and waits. The CLI checks the budget and starts the run with its own result channel. The run reads standard input, writes its answer and exits. After cleanup the CLI returns a map to the worker. The configuration script selects, the CLI reaps the worker, writes the routing notice and starts the managed process.](figures/evaluation-harness.svg)

[Open the evaluation harness sequence at full size](figures/evaluation-harness.svg).

Why are there two threads in the Ravel CLI during evaluation? The Ravel CLI
serves the socket from a Tokio runtime on one thread. Tokio is the Rust library
for asynchronous input and output. That runtime thread never waits for a file
or a program. A separate **reader thread** does all the file work and every
harness run.

Thus, a slow read or a long harness run never stops the code that watches the
deadline and cancellation. The reader is a plain thread, not a task that
`spawn_blocking` starts. Tokio cannot stop such a task while it runs. Also, the
end of a runtime waits for each such task.

Each message on the socket is one frame: a length, then a JSON document. The
module comment of `crates/ravel/src/rhai_worker/mod.rs` lists every frame and
its fields:

| Frame | From | What it carries | In the example |
|---|---|---|---|
| `hello` | Worker | Its identity | Sent first |
| `evaluate` | Ravel CLI | The entry's source, the request and the limits | The source of `config.rhai` |
| `host` | Worker | One host call and where the script made it | The `advisor` run |
| `reply` | Ravel CLI | The value of that call, or an error that the script can catch | The run's map |
| `context` | Worker | What `load_context` returned | Not sent: the script has no `load_context` |
| `delivered` | Ravel CLI | The selection context that `select` receives | Not sent |
| `selection` | Worker | The script's `VERSION` and what `select` returned | `parts` and the selection of `fake-agent` |
| `failure` | Worker | A failure of the script, with its location | Not sent |

The Ravel CLI reads the entry file itself and sends its source in the
`evaluate` frame. The worker reads no file, also for its own script. An
`import` in the script is a host call too. The Ravel CLI reads the module and
sends its source back.

The worker uses seven host calls. Other `ravel::` functions are data helpers,
which run inside the worker and send no frame:

| Host call | Rhai function | What the Ravel CLI does |
|---|---|---|
| `read_text`, `read_json` | `ravel::read_text`, `ravel::read_json` | Reads and measures a file, within the limits of the settings |
| `import` | An `import` statement | Resolves and reads a module |
| `env` | `ravel::env` | Reads a variable of the caller's environment that the settings grant |
| `harnesses` | `ravel::harnesses` | Reports the harness entries of the settings and whether each program is available |
| `remaining` | `ravel::remaining` | Reports what the evaluation still has of its budgets and deadline |
| `run_harness` | `ravel::run_harness` | Runs a harness entry through the runner and returns the outcome as a map |

An LLM call is a harness run. The script runs Claude Code or Codex
noninteractively and reads the answer from the run's output. Ravel has no model
adapter, holds no model credential and opens no network connection.

The script is synchronous, so only one host call runs at a time. The worker
sends its reply only after the reply to its last host call. Thus, no host call
is in progress when evaluation ends. The Ravel CLI then shuts down the runtime,
and no task of it remains when the managed process launches.

## The configuration module defines the interface

The configuration script is a Rhai module. Its interface to Ravel is four
exports and the `ravel` namespace:

| Export or function | Purpose | Limit |
|---|---|---|
| `SCHEMA_VERSION` | Names the request and selection shape. Ravel 2.0.0 accepts only 4. | Another value refuses |
| `VERSION` | Names the script's own version in diagnostics | A string that is not empty |
| `select(request, context)` | Selects a command or refuses | `config::judge_result` checks the returned map before any launch |
| `load_context(request)` | Optional. Assembles selection context | Selection context gives no result authority |
| `ravel::` host calls | Ask the Ravel CLI for an effect | The settings limit each one |
| `ravel::` data helpers, such as `ravel::json_check` | Compute in the worker | They give no access to files, processes or the network |

The default entry is `$HOME/.config/ravel/config.rhai`. The option `--config`
names another Rhai file. The settings stay at their path in either case.

Where does a change belong? A new route usually belongs in the configuration
script alone, with no change to Ravel. A new host capability crosses every
part of the host bridge:

- the Rhai function and its frame in `rhai_worker/bridge.rs`,
- the answer of the Ravel CLI in `rhai_worker/host.rs` or `serve.rs`,
- the conversion and checks of its data in `rhai_worker/convert.rs`,
- its limits and its section of the specification `configuration-evaluation.md`.

## The skill directories reach the program before the worker starts

Do you want each agent to find the owner's skills? A **skill** is a directory
of guidance files for an agent, such as a `SKILL.md` and the files that it
names. The owner names skill sources in the `skills` settings. Ravel copies
them, tells the agent where they are and leaves the choice of skills to the
agent.

The Ravel CLI does this work in step 3, before the worker starts:

1. It finds the source set. Without `skills.sources`, it reads the default
   skill directory that the record of its installation names.
2. It copies each source into its cache, `$HOME/.cache/ravel/skills/v1`, and
   checks the copy.
3. It writes one descriptor, `directories.json`, with the directories and the
   mandatory files.
4. It prepares the prompt: a preamble with the directories, the mandatory files
   and the descriptor path, then the caller's prompt.

The prepared prompt and the descriptor path then reach each part in a
different way:

| Who | The prepared prompt | `RAVEL_SKILL_DIRECTORIES`, the descriptor path |
|---|---|---|
| The configuration script, in the worker | In `request.prompt`. The caller's prompt stays in `request.originalPrompt`. | Not in its environment. The script reads `request.skills`. |
| Each harness run | Only when the script passes it | In its environment |
| The managed process | When the script passes it, as before, usually in an argument | In its environment |

In the example, the settings have no `skills` key. The checkout build of the
book's check has no installation record, so delivery is off. An installation
from Homebrew has a record. If its default skill directory holds a source, the
prompt of your run has a preamble. The output of the example is the same in
both cases, because no fake prints the prompt.

Why does the Ravel CLI prepare skills, and not the script? The decision record
“Skill preparation precedes evaluation”, in
`docs/adr/skill-preparation-precedes-evaluation.md`, gives the reason. One owner gives the same behaviour to every script, and the
script still builds its own command. A helper that each script calls would
make delivery depend on each script. The source of this step is in
`crates/ravel/src/skills/`. `launch.rs` holds the step, and the other modules
copy, check, cache and list the sources.

## The runner owns the local job mechanics

`keyed-launch` holds the code that launches and supervises a job: the process
group, the escalation from TERM to KILL, the reap and the terminal recovery.
A **job** is the launched program and the other processes in its process
group. `Argv` holds a program and its arguments as separate words. `Launch`
adds the working directory, the environment changes, an optional completion
channel, the entry signal state and the escalation times. The runner starts the
program directly. A shell runs only when the command selects one.

The runner does not choose a program, and it gives no meaning to provider or
model labels. It does not read a result either. Its completion channel tells it
only that a file appeared. Its Rust result is `Ended`, or a `LaunchError`. It is
never the [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
that the caller receives.

The Ravel CLI uses two launch interfaces of the runner, one for each kind of
job in the example:

| Runner interface | Information or action | In the example |
|---|---|---|
| `run_observed_with_start_check` | Supervises the managed process. It reports spawn and reap. After spawn, the Ravel CLI restores its own signal mask. | Runs `fake-agent` |
| `run_captured`, with `CapturedLaunch` | Supervises a harness run in a new POSIX session. It captures standard output and standard error with a limit, and it installs no signal handler. | Runs `fake-advisor` |
| `StopRequest` | A flag that the Ravel CLI sets to stop a captured job at a deadline or on cancellation | Not set: the run ended by itself |
| `Channel` | A fresh completion path. The appearance of its file starts the escalation. | One for each fake |
| `EntrySignals` | The signal state that the managed process starts with | Recorded when the Ravel CLI starts |
| `Ended` | The child status, the cause of its end, whether the file appeared, the group state and the operation reports | Two values, one for each fake |

The runner also has `run`, `run_observed` and `run_noninteractive`. The
production code of Ravel does not call them. A test of
`crates/ravel/src/cancellation.rs` calls `run_noninteractive`.

Why is the runner a separate package? You can test its job mechanics without
the Rhai worker or a configuration script. Its only production dependency is
`libc`, which supplies the operating-system calls. A correction to terminal
recovery can stay inside the runner while the configuration interface stays
the same. [Chapter 9](09-jobs-signals-and-the-terminal.md) explains the
mechanics and their limits.

The Ravel CLI is the runner's consumer. It supplies the environment rules, the
completion channel and the escalation times: a 2-second grace, then up to
5 seconds after TERM. It checks `Ended.group` before it accepts a result, and it
reports the operation reports that `Ended` carries. The runner can change
process-wide signal state, and its return does not restore that state.

Ravel supports no separate component. Its runner serves the Ravel CLI, which
supervises one session and then exits. Another program runs the Ravel CLI as
a child process and leaves supervision to it. The private packages do not give
a contract for a [persistent runner](01-what-ravel-is-for.md#decide-whether-ravel-fits).

## The result channel carries the value

The [result channel](02-actors-and-words.md#the-managed-process-and-its-result)
adds contents to the runner's file-appearance mechanism. In the example, Ravel
made two channels: one for `fake-advisor` and one for `fake-agent`. Each is a
`SessionChannel`. It owns a private temporary directory, the completion path
and an open directory handle for the read.

`end_session` finds the destination in `RAVEL_EXIT_FILE`. `publish_result`
writes a complete temporary file, then adds the completion name with a hard
link. A hard link cannot replace an earlier result. Thus, Ravel keeps the
first result.

The runner sees the file appear. The result-channel code reads
the contents and rejects invalid files. An empty string is a sent result. An
absent file is no result.

A harness run's result goes to the script, as `run.result`. In the example,
that is `advice`. The managed process's result goes to the caller, as the
output of `start-session`. In the example, that is `finished`.

The table shows why no part alone can decide that a session succeeded:

| Part | What it knows | What it cannot decide alone |
|---|---|---|
| `end-session` | The destination it inherited and whether the send succeeded | Whether supervision and cleanup succeeded |
| Runner | The file appeared, and the child, group and terminal observations | Whether the file holds a valid Ravel result |
| `SessionChannel` | The file contents and the `Ended` that it receives | Whether that `Ended` belongs to its own launch |
| Ravel CLI | Which channel belongs to which launch, and the other cleanup and signal reports | What the result means to the caller |

The Ravel CLI pairs each channel with its launch. The Rust type of
`SessionChannel::result_after` accepts an `Ended` from any launch. `run.rs`
gives it the outcome of the launch that received that channel. Then it closes
the channel and checks the other cleanup results. A readable file alone does
not show success.

Thus, a change to result storage affects the result channel, the runner's
observation and the coordination of the Ravel CLI. It needs no change to a
configuration script. The file format stays private, so neither scripts nor
callers depend on it. [Chapter 8](08-follow-a-session-through-the-code.md)
follows this order and the failure paths.

## The program contract stays loose

The configuration script selects a program, Ravel runs it, and a program in the
session sends one value with `end-session`. Ravel expects a
[harness](02-actors-and-words.md#the-managed-process-and-its-result) in normal
use. But it needs no model-provider interface from the selected program. The
script decides whether the program receives the prompt.

The code shows this separation. `config::judge_result` in
`crates/ravel/src/config.rs` checks the shape of the selection. Then `run.rs`
makes an `Argv` from the selected program and argument words. Ravel adds no
prompt argument.

In the example, `fake-agent` received `short` and no prompt.
`end_session` accepts text and uses the path that it inherited. The final write
returns that text unchanged after successful cleanup.

The table shows which decisions can change while the interfaces between the
parts stay the same:

| Change | Part that owns the decision | Boundary that stays the same |
|---|---|---|
| Select a different harness or an ordinary program | Configuration script | The runner still receives a program and argument words |
| Pass the prompt in a different way | Configuration script | Ravel still gives the prompt to the script in `request.prompt` |
| Give a result a new meaning | Caller | Ravel still carries one opaque value |

The program still needs to be executable. A successful session still needs a
sent result and successful cleanup. A program that exits with status 0 and no
result does not complete that contract.

## The installed executable supplies both roles

An installation has one executable, `bin/ravel`. The Ravel CLI finds its own
executable to start the Rhai worker. Thus, a symlink to `bin/ravel` uses the
same binary for both roles. Ravel does not look for the worker on `PATH`.

The `hello` of the worker holds its build identity. The Ravel CLI compares it
with its own before it sends the request. Thus, a worker of another build
refuses. The check finds a mismatch. It does not authenticate the installation
against its owner. A change to the Rhai worker needs one new build of
`bin/ravel`.

The installation also has `share/ravel/examples/`, with three example scripts,
and `share/ravel/notices/THIRD-PARTY-NOTICES.md`, the notices of the Rust
dependencies. A Homebrew installation adds the installation record
`share/ravel/installation.json`, which names the default skill directory.
The [installed layout in chapter 6](06-reference.md#installed-layout) lists
each file, and [chapter 11](11-deliver-ravel.md) explains delivery.

To summarise the chapter: the Ravel CLI starts every program, decides every
outcome and writes the only result. The Rhai worker only computes, and it asks
the Ravel CLI for each effect through the host bridge. The runner supervises
each job inside the Ravel CLI. Each job receives its own result channel.

These source boundaries do not give compatibility with every owner helper or a
contract for another program that embeds the parts. The
[test record in chapter 1](01-what-ravel-is-for.md#see-what-we-tested-and-what-remains-unchecked)
separates measured platforms from the others. Chapter 9 gives the process and
terminal guarantees within those limits.
