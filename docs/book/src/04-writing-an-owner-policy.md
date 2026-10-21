# Write an owner policy

An [owner policy](02-actors-and-words.md#caller-and-owner) receives a request
from a [caller](02-actors-and-words.md#caller-and-owner) and returns a command
or a [refusal](02-actors-and-words.md#policy-evaluation-and-supervision).
Start with chapter 3's fake agent. Then add decisions that you can test:
which [kinds](02-actors-and-words.md#caller-and-owner) to accept, which
arguments to pass, and what to tell a caller when no route applies.

Use the Ravel that you installed in
[chapter 3](03-a-complete-session.md#install-ravel-and-check-its-version).
Put each complete example below in its own empty directory. Create the `bin`
subdirectory first. Then save the files of the example at the labelled paths.

The examples create private `home` and `tmp` directories. They clear the
environment of the subprocess and put the fake programs first on `PATH`. They
use real policy evaluation. They do not connect to a model service or read
your usual owner configuration.

## Define a policy and read the request

<!-- tc: depart synonym-rotation because TypeScript field checks and runtime policy validation occur at different boundaries -->
The entry file exports a plain object with the name `policy`. Import
`definePolicy` from the literal module name `ravel/sdk`. Give that object to
`definePolicy` as its argument. The helper returns the object unchanged. It
gives TypeScript sufficient information to check your fields and
to find the parameter types of `select`. Ravel still validates the policy at
runtime, when it evaluates the policy.

The policy object can have these four fields. They identify the policy
interface, give a label to your revision and supply the functions that Ravel
calls:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`. It identifies the policy shape. |
| `version` | A nonblank label that you maintain, such as `book-routing-1`. It is independent of Ravel's own version. |
| `select` | A function that returns a selection or a refusal, directly or through a promise. |
| `loadContext` | An optional function that assembles the [policy context](02-actors-and-words.md#three-different-kinds-of-information) before selection. [Supply policy context](#supply-policy-context) describes it. |

Ravel refuses a policy object that has an unknown field. It reads only the
named export `policy`. If an entry has only a default export, Ravel refuses
it as an invalid policy.

The simplest `select` uses only its first parameter, `request`. Ravel also
gives `select` the delivered policy context, which can be `undefined`, and a
host. The host has `diagnostic(text)` and an
`AbortSignal` at `signal`. With the host, asynchronous work can monitor the
selection deadline, and the policy can write diagnostic text. The host of
`select` has no operations that read files. Only the host of the
policy-context loader has them.

The request is frozen data. This table gives its fields. Most fields are
caller input. `schemaVersion` identifies the request shape, and `limits`
holds Ravel's effective bounds:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`. |
| `kind` | The caller's nonempty token. Only your policy defines which tokens it accepts. |
| `prompt` | The unchanged prompt, also when it is an empty string or contains newlines. Ravel adds no instructions to end the [session](02-actors-and-words.md#caller-and-owner). |
| `cwd` | The caller's working directory, not the directory of the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision). |
| `params` | The caller's `--param NAME=VALUE` pairs, as strings by name. It is an empty object when the caller supplied no pair. |
| `taskFile` | An optional absolute path from `--task-file`. Ravel does not read the file for you. |
| `taskId` | An optional caller identifier from `--task-id`. |
| `context` | An optional validated version-1 document from `--context`. |
| `limits` | The effective bounds for this invocation. |

If an optional field is absent, the caller did not supply it. `limits` has
these six fields:

- `selectionMs`: the maximum time for the whole selection
- `contextBytes`: the maximum bytes of the delivered policy context
- `sourceBytes`: the default maximum bytes for one host read
- `sources`: the maximum number of measured sources
- `messageBytes`: the maximum bytes of one protocol message
- `diagnosticsBytes`: the maximum bytes of all worker output together

Your policy can read these values. If the policy changes a value, the bound
that Ravel enforces does not increase.
[Set bounds and grant environment names](#set-bounds-and-grant-environment-names)
gives the settings and their ranges.

Fragment: only the signature of the `select` method. This block omits the
policy object and the policy code in the method.

```ts
select(request, context, host) {
  host.diagnostic(`Selecting a command for ${JSON.stringify(request.kind)}`);
  // Return a selected command or a refusal here.
}
```

To use a private test file, pass `--config policy.ts`. Ravel resolves that
path against the caller's working directory. It follows the path to a
readable regular file and imports the real path. Put relative helpers beside
that real file.

Without `--config`, the entry is `$HOME/.config/ravel/policy.ts`. Ravel does
not search a repository for a policy. With each of the two entries, the
settings come from `$HOME/.config/ravel/settings.json`. Thus, the examples
supply an empty private `HOME` and also `--config`.

## Select a command or refuse

A selection describes the process that Ravel starts. `program` is one of
these:

- an absolute path
- a relative path that contains a separator, which Ravel resolves against the
  caller's working directory
- a name that Ravel finds on the caller's `PATH`

Each string in `args` becomes one argument after the program. Ravel does no
shell expansion or word splitting. A shell runs only when the selected
command needs one. The fake in this section needs one: its shebang line names
`/bin/sh`.

A selection has these fields. Some give the command that Ravel executes. The
others give the labels and the explanation that Ravel reports:

| Selection field | Meaning |
|---|---|
| `status` | The literal `"selected"`. |
| `program` | The path or the name of the executable. |
| `args` | The argument strings, in order. The array and its strings can be empty. |
| `provider`, `model`, `effort` | Owner labels that the routing diagnostics report. They do not configure a service or change the arguments. |
| `reason` | Your explanation of this decision. The routing diagnostics report it verbatim. |

Each string field is nonblank, except the entries in `args`. The program and
the arguments cannot contain a NUL. Ravel validates the shape of the
selection. It does not examine whether the command is sensible or whether it
passes the prompt. Your policy and its tests own those decisions.

A refusal gives a reason that the caller can identify and an action that the
caller can take. It has these fields:

| Refusal field | Meaning |
|---|---|
| `status` | The literal `"refused"`. |
| `code` | Your nonblank code that identifies the reason. |
| `message` | A nonblank explanation of the problem. |
| `remedy` | A nonblank action that the caller or the owner can take. |

<!-- tc: depart synonym-rotation because launch refers to a process, while start also names the beginning of a deadline -->
Ravel reports your refusal as `policy_refused`, together with your code
(`policyCode` in JSON diagnostics). Then it exits with status 3 and launches
nothing. It does not try a different route. If `select` returns nothing,
returns malformed data or throws an error, Ravel also launches nothing. But
then the caller does not get the message and the remedy of your refusal. If
evaluation does not finish before the selection deadline, the outcome is
`selection_timeout`, which is different from an explicit policy refusal.

The table below gives the steps of evaluation, in order, on the path to a
launch. A returned selection does not let Ravel skip the worker cleanup. If
evaluation stops at an earlier step, it also needs worker cleanup, but no
managed launch can follow.

| Step | Work | Condition to continue |
|---|---|---|
| 1 | Ravel imports the owner policy. | The import succeeds, and the named `policy` export has the accepted shape. |
| 2 | Ravel assembles the [policy context](#supply-policy-context). It calls `loadContext` if it exists. | The caller data and the delivered document obey their shape and size rules. If the loader refuses, evaluation stops here, and Ravel does not call `select`. Without a loader or a caller document, selection continues with `undefined`. |
| 3 | Ravel calls `select` with the request and the delivered policy context. | The returned value is a valid selection. An explicit refusal becomes `policy_refused`. Malformed data or a thrown error also prevents the launch. |
| 4 | Ravel reaps the policy worker and removes its private directory before the managed launch. | Ravel confirms the reap and the removal. If the cleanup fails, a returned selection is not sufficient for a launch. <!-- tc: depart synonym-rotation because confirmation establishes completed reap and removal rather than merely attempting a check --> |

One deadline, `selectionMs`, starts when Ravel spawns the worker. It covers
the import, the assembly of the policy context and `select`. It does not
start again for each step. When it expires, evaluation stops with
`selection_timeout` (exit status 124), and Ravel launches nothing. A refusal
has exit status 3. Ravel can refuse an invalid caller document before the
loader runs.

The evaluation table above gives only the order from the policy to the
launch. Chapter 3's
[outcome view](03-a-complete-session.md#recognise-the-other-outcomes) also
gives the outcomes of cancellation and supervision.

The policy below extends the example of chapter 3. It accepts only the kind
`demo`, and it refuses an empty prompt for that kind. Ravel accepts an empty
prompt, but this policy requires a prompt. The order of the two checks is
intentional: the policy refuses an unsupported kind before it examines the
prompt.

Complete example: validate the accepted route and the two explicit refusals
of one small policy.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-routing-1",
  select(request) {
    if (request.kind !== "demo") {
      return {
        status: "refused",
        code: "unsupported_kind",
        message: "This policy accepts only demo.",
        remedy: "Pass --kind demo or add a route in policy.ts.",
      };
    }
    if (request.prompt === "") {
      return {
        status: "refused",
        code: "prompt_required",
        message: "The demo route needs a prompt.",
        remedy: "Supply a nonempty --prompt or --prompt-file.",
      };
    }
    return {
      status: "selected",
      program: "fake-agent",
      args: [request.prompt],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "The demo route uses the local fake.",
    };
  },
});
```

**`bin/fake-agent`**

```sh
#!/bin/sh
set -eu
printf 'fake-agent received: %s\n' "$1"
ravel end-session -- "finished"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Print this prompt, then call ravel end-session -- "finished".'
```

**Standard output, without a final newline**

```text
finished
```

**Standard error includes**

```text
ravel: starting provider local, model fake, effort none for kind "demo"
fake-agent received: Print this prompt, then call ravel end-session -- "finished".
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind unknown --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: unsupported_kind
  remedy: Pass --kind demo or add a route in policy.ts.
```

**Exit status**

```text
3
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt ''
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: prompt_required
  remedy: Supply a nonempty --prompt or --prompt-file.
```

**Exit status**

```text
3
```

For the successful command, Ravel writes the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
`finished` unchanged to standard output, with no final newline. The fake's
output goes to the caller's standard error. Thus, standard output contains
only the result. Each refusal returns no result to the caller. The standard-error excerpts check stable diagnostic lines.
Other operational reports can also occur on standard error.

When you edit the policy, keep a test case for each route and each refusal.
A fake can record the full argument vector and send a known value as the
result. Thus, you can check the choice of command, and you do not pay for a
model. Also, no agent acts on the prompt. `start-session` launches the selected
command. Ravel has no verb that examines a policy without a launch,
so this validation is necessary.

`task docs:examples` runs the examples in this book. It uses a private `PATH`
that contains only the fakes of the example, the built Ravel and the system
utilities. It also clears the environment, uses a temporary `HOME` and
`TMPDIR`, and gives each command a limit of 60 seconds.

## Typecheck against the public declarations

A typecheck finds misspelt fields, missing fields and wrong return types
before a session runs. It does not execute the policy. It does not prove that
each kind has a route. Use it together with the fake-program checks above.

<!-- tc: depart synonym-rotation because fetch refers to tool downloads, while get also describes receiving policy data -->
The recipes below need a source checkout as a development tool. The
installed Ravel alone can run your policy. For the checkout, use Rust 1.85
or later, curl, tar, Task, ShellCheck and cargo-nextest. Run `task setup` in
the checkout root to fetch its tools and install its development dependencies.
[Chapter 11, “Develop Ravel”](11-develop-ravel.md), gives the development workflow.

The checkout generates the public declarations when it builds the worker.
Use those declarations for your policy. Do not use the implementation modules
of the worker. In the root of the checkout, run `task worker` to generate the
declarations. Then create a temporary configuration that includes the
absolute path of your policy. The recipe maps the two embedded module names,
because the later examples also import the starter table.

Fragment: the typecheck in a checkout. Replace the policy path. This block
omits the policy and the development setup above. Run it in
the root of the checkout. It runs in a subshell, so its cleanup trap has an
effect only in that subshell.

```sh
(
  set -eu
  task worker
  export RAVEL_BOOK_ROOT="$PWD"
  export RAVEL_BOOK_POLICY="/absolute/path/to/policy.ts"
  check_dir=$(mktemp -d)
  trap 'rm -rf "$check_dir"' EXIT
  target/tools/bun -e '
    const root = process.env.RAVEL_BOOK_ROOT;
    console.log(JSON.stringify({
      extends: `${root}/crates/ravel/worker/tsconfig.json`,
      compilerOptions: {
        types: ["bun"],
        typeRoots: [`${root}/crates/ravel/worker/node_modules/@types`],
        paths: {
          "ravel/sdk": [`${root}/target/libexec/ravel/sdk/index.d.ts`],
          "ravel/examples/*": [`${root}/target/libexec/ravel/examples/*.d.ts`]
        }
      },
      include: [process.env.RAVEL_BOOK_POLICY]
    }, null, 2));
  ' > "$check_dir/tsconfig.json"
  target/tools/bun crates/ravel/worker/node_modules/typescript/bin/tsc \
    -p "$check_dir/tsconfig.json"
)
```

The recipe uses `JSON.stringify` to write the file. Thus, spaces and other
characters in the paths stay correct. Through `extends`, the checker uses
the strict options of the repository. `typeRoots` finds the checkout's Bun
declarations, although the configuration is in a different directory. A
successful check prints no TypeScript diagnostics and exits with status 0.

The directory that `brew --prefix ravel` prints also contains
`libexec/ravel/sdk/index.d.ts` and `libexec/ravel/examples/static.d.ts`,
together with TypeScript sources that you can edit. The installation does not
contain a development type checker. You can check
a policy against that installed interface with a separate TypeScript
installation. The following recipe uses the checkout's pinned checker only as
that development tool. Its module mappings point to the installed
declarations. To run a policy, an installation needs neither that checker
nor a separate installation of Bun.

Fragment: the typecheck against the installed declarations, with the
checkout's checker. Replace the two absolute paths. This block omits the
installation and the policy. Run it in the root of the checkout, after you
install the development dependencies of the checkout.

```sh
(
  set -eu
  export RAVEL_BOOK_PREFIX="/absolute/path/to/installation"
  export RAVEL_BOOK_POLICY="/absolute/path/to/policy.ts"
  check_dir=$(mktemp -d)
  trap 'rm -rf "$check_dir"' EXIT
  target/tools/bun -e '
    const prefix = process.env.RAVEL_BOOK_PREFIX;
    console.log(JSON.stringify({
      compilerOptions: {
        strict: true,
        noEmit: true,
        target: "esnext",
        module: "preserve",
        moduleResolution: "bundler",
        allowImportingTsExtensions: true,
        verbatimModuleSyntax: true,
        noUncheckedIndexedAccess: true,
        noImplicitOverride: true,
        exactOptionalPropertyTypes: true,
        types: [],
        paths: {
          "ravel/sdk": [`${prefix}/libexec/ravel/sdk/index.d.ts`],
          "ravel/examples/*": [`${prefix}/libexec/ravel/examples/*.d.ts`]
        }
      },
      include: [process.env.RAVEL_BOOK_POLICY]
    }, null, 2));
  ' > "$check_dir/tsconfig.json"
  target/tools/bun crates/ravel/worker/node_modules/typescript/bin/tsc \
    -p "$check_dir/tsconfig.json"
)
```

The policies in the sections above need no global types for Bun.
Thus, the standalone configuration uses `types: []`. A policy that uses the
APIs of a specific runtime also needs the development declarations for that
runtime. A typecheck does not replace validation at runtime. The installed
worker still checks the object and the selection that it actually receives.

## Use the starter route table

The embedded module `ravel/examples/static` supplies an example that you can
copy and edit. The module's `Route` type is a function from a request
to a `Command`. A
`Command` is a selection without `status` and `reason`.
`selectRoute(table, request)` finds the table's own entry, not an inherited
property, whose name is exactly the kind. It calls that route and adds the selected status and a reason that
names the route. For an unknown kind, it returns a refusal with the policy
code `incomplete_mapping`, and no other entry is a default.

This table gives the effort argument that each route of the module passes.
These kind names and effort values are owner conventions:

| Kind | Effort passed to the wrapper |
|---|---|
| `question` | `low` |
| `bugfix` | `medium` |
| `feature` | `high` |
| `migration` | `xhigh` |
| `architecture` | `xhigh` |

Each entry calls `agent(effort)`. This function makes a command that runs
`my-agent-wrapper` with the arguments `--model your-model --effort EFFORT`.
The unchanged prompt follows them as one argument. The provider label of the
command is `your-provider`. These names are placeholders. They are not
integrations with installed agents.

The module's exported `policy` is a complete policy that uses this table. To
try the table, re-export that `policy` from your own entry file. Then, to edit
your own table, copy `examples/static.ts` from the installation's
`libexec/ravel` directory. A change to
your local copy does not change the embedded import.

The fake below sends the arguments that it receives as a multiline result.
Thus, the transcript shows the order of the arguments, the labels that are
arguments and the prompt as one argument. The loop checks the status of each
session before it starts the next session.

Complete example: test each entry of the supplied table, and its refusal of
an unknown kind, through a fake wrapper.

**`policy.ts`**

```ts
export { policy } from "ravel/examples/static";
```

**`bin/my-agent-wrapper`**

```sh
#!/bin/sh
set -eu
result=$(printf '<%s>\n' "$@")
ravel end-session -- "$result"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/my-agent-wrapper
$ for kind in question bugfix feature migration architecture; do
    printf '%s\n' "$kind"
    env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind "$kind" --config policy.ts --prompt 'Report the arguments, then call ravel end-session with that report.' || exit "$?"
    printf '\n'
  done
```

**Standard output**

```text
question
<--model>
<your-model>
<--effort>
<low>
<Report the arguments, then call ravel end-session with that report.>
bugfix
<--model>
<your-model>
<--effort>
<medium>
<Report the arguments, then call ravel end-session with that report.>
feature
<--model>
<your-model>
<--effort>
<high>
<Report the arguments, then call ravel end-session with that report.>
migration
<--model>
<your-model>
<--effort>
<xhigh>
<Report the arguments, then call ravel end-session with that report.>
architecture
<--model>
<your-model>
<--effort>
<xhigh>
<Report the arguments, then call ravel end-session with that report.>
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind unknown --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: incomplete_mapping
  remedy: add a route for this kind in your copy of the policy
```

**Exit status**

```text
3
```

## Offer a checkout choice

A policy can let a checkout choose from arrangements that the owner wrote
before. The SDK's `readChoice(request.cwd, offered)` reads `.ravel-choice` in
that directory. It splits the contents of the file at whitespace and returns
the names as an array. If the file does not exist, it returns `undefined`,
so the policy supplies the default. It does not search parent directories. For an empty file, it returns an empty array, not `undefined`.

The helper accepts a regular UTF-8 file of 4 KiB or less. For a file that
exists but that the helper does not accept, it returns a refusal with the
code `choice_unreadable`:

- a file that it cannot read
- a file that is too large
- a file that is not valid UTF-8
- a file that is not a regular file

For a name that is not in `offered`, it returns a refusal with the code
`choice_unoffered`. Return each refusal of `readChoice` from `select`.

The exported constant `CHOICE_FILE` holds the file name. The helper does an
ordinary filesystem read, not a measured read of policy context. If the
selection must explain the choice, put the applied choice in its `reason`.

Nothing reads this file unless your policy calls the helper. The file can
choose offered names, but it cannot add a program or arguments. Your code
decides how many names the policy accepts and what each name means.

The sample that you can edit is `crates/ravel/worker/sample/policy.ts` in the
checkout, and `libexec/ravel/examples/sample.ts` in an installation. The
example below uses the decisions of the sample:

- Without a choice file, the choice is `default`.
- The policy accepts exactly one name.
- `careful` sets the selected effort and its argument to `high`.

The `careful` value replaces the effort of each route. It is not a minimum.
Thus, it also replaces the table's `xhigh` for `migration` and
`architecture`. If you want a different arrangement, edit the names and the
behaviour together.

Complete example: validate the two arrangements of the sample on all five
routes. Then validate its four refusals: an unknown kind, the wrong number of
names, a name that it does not offer and an unreadable file.

**`policy.ts`**

```ts
import { definePolicy, readChoice } from "ravel/sdk";
import { selectRoute, routes } from "ravel/examples/static";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "owner-choice-1",
  select(request) {
    const choice = readChoice(request.cwd, ["default", "careful"]);
    if (choice !== undefined && "status" in choice) return choice;
    const names = choice ?? ["default"];
    if (names.length !== 1) {
      return {
        status: "refused",
        code: "one_choice",
        message: "choose one arrangement",
        remedy: "write default or careful in .ravel-choice",
      };
    }
    const selected = selectRoute(routes, request);
    if (selected.status === "refused" || names[0] !== "careful") return selected;
    return {
      ...selected,
      effort: "high",
      args: ["--model", selected.model, "--effort", "high", request.prompt],
      reason: "careful choice",
    };
  },
});
```

**`bin/my-agent-wrapper`**

```sh
#!/bin/sh
set -eu
[ "$#" -eq 5 ]
[ "$1" = '--model' ]
[ "$2" = 'your-model' ]
[ "$3" = '--effort' ]
[ "$5" = 'Report the effort, then call ravel end-session with that effort.' ]
ravel end-session -- "$4"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/my-agent-wrapper
$ for choice in absent default careful; do
    if [ "$choice" = absent ]; then
      rm -f .ravel-choice
    else
      printf '%s\n' "$choice" > .ravel-choice
    fi
    for kind in question bugfix feature migration architecture; do
      printf '%s %s: ' "$choice" "$kind"
      env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind "$kind" --config policy.ts --prompt 'Report the effort, then call ravel end-session with that effort.' || exit "$?"
      printf '\n'
    done
  done
```

**Standard output**

```text
absent question: low
absent bugfix: medium
absent feature: high
absent migration: xhigh
absent architecture: xhigh
default question: low
default bugfix: medium
default feature: high
default migration: xhigh
default architecture: xhigh
careful question: high
careful bugfix: high
careful feature: high
careful migration: high
careful architecture: high
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind unknown --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: incomplete_mapping
  remedy: add a route for this kind in your copy of the policy
```

**Exit status**

```text
3
```

**Command**

```console
$ printf 'default careful\n' > .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind question --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: one_choice
  remedy: write default or careful in .ravel-choice
```

**Exit status**

```text
3
```

**Command**

```console
$ printf '' > .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind question --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: one_choice
  remedy: write default or careful in .ravel-choice
```

**Exit status**

```text
3
```

**Command**

```console
$ printf 'turbo\n' > .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind question --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: choice_unoffered
  remedy: write names from default, careful in <scratch>/work/.ravel-choice, separated by whitespace, or delete it
```

**Exit status**

```text
3
```

**Command**

```console
$ rm .ravel-choice && mkdir .ravel-choice
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind question --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: choice_unreadable
  remedy: write names from default, careful in <scratch>/work/.ravel-choice, separated by whitespace, or delete it
```

**Exit status**

```text
3
```

The prefix `<scratch>/work` stands for the temporary working directory that
the book's checker uses. Your diagnostic gives your example directory. The last
case makes `.ravel-choice` a directory. Thus, it tests the refusal of a file
that is not a regular file, and the test does not depend on permissions.
Remove that directory with `rmdir` before you write a choice file again. The
other conditions for an unreadable file also give `choice_unreadable`, but
this example does not show each filesystem failure that can cause that code.

## Supply policy context

A caller can supply a JSON document with `--context PATH`. The path is
relative to the caller's working directory, and it names a readable regular
file. The document's `schemaVersion` is `1`, independently of the policy's
version-2 interface. The document holds facts and attributed judgements. Your
policy can use them to choose from the commands that it defines. The document
does not give permission to run a command that its facts name.

The document has the fields in the table below. The choice of the executable
stays in owner code.

| Field | Accepted data |
|---|---|
| `schemaVersion` | Required: the number `1`. |
| `summary` | An optional string. It can be empty. |
| `acceptanceCriteria` | An optional array of strings. |
| `facts` | An optional object with JSON data as its values. |
| `assessments` | An optional object with values of the form `{ by, value }`: a nonblank author name and JSON data. |
| `sources` | An optional array of 256 source records or fewer. Each record has a nonblank `name`. It identifies the source with `sha256`, with a nonblank `version`, or with both. An optional `bytes` is a nonnegative whole number. A digest is 64 lowercase hexadecimal digits. |

An absent field means that the value is unknown. For example, an absent
`facts` is different from a known empty object. Ravel refuses each unknown
top-level field, for example `program`, `args`, `argv`, `command`, `select`,
`loadContext`, `env`, `environment`, `exec`, `shell` and `script`. Nested JSON facts stay data, also when a key has one of those
names. The caller and the loader do not supply the `measured` field. Ravel
adds it.

Ravel reserves one JSON object shape, also in facts and in assessment values:
an object whose only key is `$ravel`. Ravel uses that shape to mark values
that JSON cannot hold. It refuses that shape as `context_invalid`. Use a
different key for your own data.

Without `loadContext`, `select` receives the caller's document with Ravel's
`measured` field attached. If the caller supplied no document, `select` receives
`undefined`. With a loader, `select` receives the return value of the loader
instead. The loader receives `request.context`. Thus, it can explicitly keep
or replace the caller's data. Ravel does not merge the two documents for you.

The loader can be asynchronous. It shares the selection deadline with the
policy import and `select`.

Find the row for your caller input and your policy. A loader replaces the
document that `select` receives, unless the code of the loader keeps the
caller's data. Each row assumes that the documents are valid and that the
loader, if the policy has one, returns a document:

| Caller document? | Policy has `loadContext`? | Policy context that `select` receives |
|---|---|---|
| No | No | `undefined`. |
| Yes | No | The caller's document, with Ravel's `measured` list attached. |
| No | Yes | The document that the loader returns, with `measured` attached. The loader receives no `request.context`. |
| Yes | Yes | The document that the loader returns, with `measured` attached. The loader receives the caller's document at `request.context`, and it decides what to keep. Ravel does not merge the two documents automatically. |

Choose a synchronous host read for the value that your loader needs. Each
read returns a source record that identifies the bytes that it read:

| Operation | Return value |
|---|---|
| `host.readText(path, maxBytes?)` | `{ text, source }`. It decodes the file as UTF-8. |
| `host.readJson(path, maxBytes?)` | `{ value, source }`. It decodes the file as UTF-8 and parses the JSON. Validate that JSON value for the needs of your policy. |

The two reads resolve relative paths against `request.cwd`. They accept only
a regular file. They return a `source` with the canonical `name`, the actual
`bytes` and the `sha256` of the file. The default read limit is 65,536 bytes,
or the policy-context budget if the budget is smaller. A positive whole
number in `maxBytes` sets the limit of one read. That limit can be as large
as the policy-context budget.

These read problems cause a refusal, even if the loader catches the exception
that the read throws:

- a read that is larger than its limit
- a read that asks for a limit larger than the budget
- a read that makes the number of measured sources larger than its limit

A missing file, invalid UTF-8 or invalid JSON causes an exception. If the loader
does not catch that exception, the refusal identifies the source of the read
that failed. A loader that needs valid content can return its own
refusal, as `select` can. Then Ravel does not call `select`.

Ravel attaches a `measured` array to the frozen document that it delivers.
Its records have `name`, `bytes`, `sha256` and `via` (`"--context"`,
`"readText"` or `"readJson"`). The caller's document comes first. The
successful host reads follow, in order, and each repeated read of a file
makes its own record. The limit is 256 measured records, and the record of
the caller's document, if there is one, counts toward it. The document's
own `sources` array has a separate limit of 256 records.

The complete delivered JSON, with `measured`, cannot be larger than the
policy-context budget. Thus, a small input file can give a delivered
document that is too large when Ravel attaches its measurement.

These fields distinguish claims about sources from Ravel's own measurements:

| Field | Who supplies it | What it identifies |
|---|---|---|
| `sources` | The caller or the loader | The stated origin of a claim, by digest, version or both. |
| `assessments[name].by` | The caller or the loader | The stated author of an assessment. |
| `measured` | Ravel | The bytes read from the caller document and through the host, with each path, size, digest and method. |

None of these fields proves who the author is or that the claim is true.
Trusted owner code can also read files or the network directly, but
`measured` does not record those reads.

Keep the policy decisions in owner code. Use outside information to choose a
route that the policy defines. Do not use outside text as a program or as a
template of arguments to execute.

The example below keeps the caller's data and adds a local readiness fact and
a summary. The loader checks the JSON readiness file before it reads the text
file. The fake reports the summary and the measurement methods. Thus, the two
successful commands show the change when the caller supplies a document.

Complete example: assemble the policy context and validate its necessary
fact. Then test the refusals for invalid caller data, for a source that the
loader cannot read and for a read that is too large.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-context-1",
  loadContext(request, host) {
    const readiness = host.readJson("ready.json");
    const value = readiness.value;
    if (value === null || typeof value !== "object" || Array.isArray(value)
        || !("ready" in value) || value.ready !== true) {
      return {
        status: "refused",
        code: "not_ready",
        message: "The local readiness file does not say ready: true.",
        remedy: "Set ready to true in ready.json when the work is ready.",
      };
    }
    const brief = host.readText("brief.txt", 128);
    return {
      ...request.context,
      schemaVersion: 1,
      summary: brief.text.trim(),
      facts: { ...request.context?.facts, ready: true },
      sources: [...(request.context?.sources ?? []), readiness.source, brief.source],
    };
  },
  select(request, context) {
    return {
      status: "selected",
      program: "fake-agent",
      args: [context?.summary ?? "", context?.measured.map((s) => s.via).join(",") ?? ""],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: `Local readiness accepted for ${request.kind}.`,
    };
  },
});
```

**`caller.json`**

```json
{
  "schemaVersion": 1,
  "acceptanceCriteria": ["Keep the public interface unchanged."],
  "assessments": { "scope": { "by": "caller", "value": "small" } },
  "sources": [{ "name": "work-brief", "version": "revision-1" }]
}
```

**`ready.json`**

```json
{"ready": true}
```

**`brief.txt`**

```text
Check the interface.
```

**`bin/fake-agent`**

```sh
#!/bin/sh
set -eu
[ "$#" -eq 2 ]
result=$(printf '%s\n%s' "$1" "$2")
ravel end-session -- "$result"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Report the supplied summary and source methods.'
```

**Standard output, without a final newline**

```text
Check the interface.
readJson,readText
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --context caller.json --prompt 'Report the supplied summary and source methods.'
```

**Standard output, without a final newline**

```text
Check the interface.
--context,readJson,readText
```

**Exit status**

```text
0
```

**Command**

```console
$ printf '{"ready":false}\n' > ready.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: not_ready
  remedy: Set ready to true in ready.json when the work is ready.
```

**Exit status**

```text
3
```

**Command**

```console
$ printf '{"ready":true}\n' > ready.json && rm brief.txt
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  source: <scratch>/work/brief.txt
  location: policy.loadContext
```

**Exit status**

```text
3
```

**Command**

```console
$ printf '%129s' x > brief.txt
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (source_too_large, stage context): loadContext in <scratch>/work/policy.ts read <scratch>/work/brief.txt, which holds more than its limit of 128 bytes (129 bytes or more)
```

**Exit status**

```text
3
```

**Command**

```console
$ printf '{"schemaVersion":1,"program":"fake-agent"}\n' > caller.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --context caller.json --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (context_invalid, stage context): `program` would be an executable field, and a context is data: it supplies no program, argument, environment or policy
```

**Exit status**

```text
3
```

In the last command, Ravel refuses the caller's invalid document before the
loader runs. Thus, the loader does not read the text file that is larger
than its limit, from the command before it. These examples test the one
accepted route of the policy, its explicit refusal and some typical Ravel
refusals.

The table below gives all the diagnostic codes for policy context. With each
of these codes, Ravel exits with status 3 and launches nothing. Use the code
to find the input, the loader behaviour or the bound that needs a change:

| Code | What to correct |
|---|---|
| `context_unreadable` | Ravel cannot read the `--context` path as a regular file. |
| `context_invalid` | The document is not valid JSON or has the wrong shape. The same code applies when the loader returns invalid or unserialisable data or no document, or supplies `measured` itself. |
| `unsupported_version` | A nonnegative integer that fits in an unsigned 64-bit value names a policy-context schema version other than 1. A negative, fractional or wrong-type version gives `context_invalid`. |
| `context_loader_failed` | The loader throws an exception or rejects its promise. The cause is not a known source-read failure or a breach of a bound. |
| `context_loader_unsettled` | The loader's promise stays unsettled, and no work that can resolve or reject it remains. |
| `message_too_large` | A failure report of the loader is larger than the fixed 1 MiB protocol-message bound. Make the error that it reports shorter. <!-- tc: depart synonym-rotation because fixed describes an immutable bound and does not mean correcting an error --> |
| `context_source_unreadable` | A required host read fails, also for invalid UTF-8 or invalid JSON, and the loader does not catch the failure. |
| `source_too_large` | A host read is larger than its limit, or it asks for a limit above the policy-context budget. |
| `too_many_sources` | The `measured` list or the document's attributed `sources` has more than 256 entries. |
| `context_too_large` | The caller document or the delivered document is larger than the policy-context budget. |
| `policy_refused` | The loader returns a well-formed refusal. Its own code, message and remedy explain the requirement. |

If the loader's promise is unsettled and work that can settle it continues
until the deadline, the outcome is `selection_timeout`, not `context_loader_unsettled`.
Invalid host arguments can cause a loader failure. If
the loader returns a malformed refusal, Ravel treats it as invalid policy
context. [Chapter 6, “Reference”](06-reference.md), gives the complete
lookup entries. They include the failures that also occur in policy
selection.

## Set bounds and grant environment names

The owner settings are in `$HOME/.config/ravel/settings.json`, also when
`--config` chooses a different policy. The file is optional. If the file is
absent or holds an empty JSON object, Ravel uses the defaults. If `HOME` is
missing or relative, Ravel reads no settings file. The default policy entry
still needs an absolute `HOME`. Thus, in that case, give an explicit entry
with `--config`.

Use this table to choose an owner default. It also shows whether a caller
replaces that value or adds to it:

| Key | Value and default | Caller adjustment |
|---|---|---|
| `timeoutMs` | Whole-number milliseconds, 1,000–600,000 inclusive. The default is 30,000. | `--timeout-ms` replaces it. |
| `contextBytes` | Whole-number bytes, 1–8,388,608 inclusive. The default is 262,144 (256 KiB). | `--context-bytes` replaces it. |
| `policyEnv` | An array of environment-variable names. The default has no extra grants. | Each `--policy-env NAME` adds one name. |

Ravel checks all the settings before it applies the CLI values. A valid CLI
replacement does not correct an invalid setting. These problems give
`settings_invalid` (exit status 2):

- a settings file that Ravel cannot read, or that is not a regular file
- invalid JSON
- a document that is not an object
- a `policyEnv` value that is not an array of strings
- an unknown key

An invalid numeric bound or a malformed grant name gives `malformed_input`
(exit status 2).

The obsolete key `stateDir` is an unknown key. Remove it. Do not point it to
an old store. Ravel opens no storage.

A numeric CLI bound accepts only plain decimal digits in the range, with no
sign, fraction, exponent or whitespace. Ravel never clamps a value.

The request gives six effective limits. The first two settings change the
other limits only as this table shows. They do not limit the time for which
the [managed process](02-actors-and-words.md#the-managed-process-and-its-result)
runs.

| Request field | Effective bound | Source of the value |
|---|---|---|
| `selectionMs` | The policy import, the loader and the selection together, from the worker spawn. | `timeoutMs`, then a `--timeout-ms` that replaces it. Otherwise 30,000 ms. |
| `contextBytes` | The caller document and the delivered policy-context JSON, with the measured records. | `contextBytes`, then a `--context-bytes` that replaces it. Otherwise 262,144 bytes. |
| `sourceBytes` | The default bytes for one host read. | The smaller of 65,536 and `contextBytes`. A read can set `maxBytes` from 1 through `contextBytes`. |
| `sources` | The measured records, with the caller document. Separately, the attributed source records. | Always 256 for each array. |
| `messageBytes` | A policy snapshot, a selection result or a failure report in the private protocol. | Always 1,048,576 bytes (1 MiB). The policy-context document has its own budget. |
| `diagnosticsBytes` | The worker's standard output and standard error together. | Always 262,144 bytes (256 KiB). |

Ravel refuses data that is too large. It does not truncate the data to make
it fit. An overflow of policy context or of a host read gives the codes
above. A policy snapshot, a selection result or a failure report that is
larger than its protocol bound gives `message_too_large` (exit status 3). Too
much worker output gives `output_limit` (exit status 3). Ravel then captures
only the first 262,144 diagnostic bytes, discards the remainder while it
stops the worker and does not accept the selection.

If the selection deadline expires, the outcome is `selection_timeout` (exit
status 124). A caller can increase an adjustable bound only in its hard
range. The policy cannot increase a bound with a change to the request data.

The policy worker receives the caller's `HOME`, `PATH`, `TMPDIR`, `LANG` and
`LC_*` variables, if they are present. Grant more names only when the policy
code needs their values. An example is a network request while the policy
chooses a route. A grant is a name, never `NAME=value`. Its value comes from
the environment of the
[Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision).
Repeated grants of one name give one grant.

A name that is not set stays unset in the worker. Grant diagnostics report
the names and whether each name is set. They never report values. But your
own policy can disclose a value if it prints that value.

The following exclusions are case-sensitive. They apply to settings and to
CLI grants. An excluded name gives `excluded_grant` (exit status 2).

| Name or prefix pattern | Reason for the exclusion |
|---|---|
| `BUN_*` | Runtime startup and options, for example preloads and runtime mode. |
| `NODE_OPTIONS`, `NODE_PATH`, `NODE_PRESERVE_SYMLINKS` | Runtime startup and module resolution. |
| `NODE_CHANNEL_*` | Runtime IPC descriptors and channel options. |
| `LD_*`, `DYLD_*` | Dynamic-loader inputs. |
| `RAVEL_*` | Ravel's own controls. The [session-result context](02-actors-and-words.md#three-different-kinds-of-information) is one of them. |
| `HARNESS_DISPATCH_*` | Legacy dispatch controls. |

Ravel itself sets `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` in the worker. An
owner grant cannot replace it. The exclusions match these exact names and
prefixes, not each name that starts with `NODE` or `BUN`. A grant name is
nonempty UTF-8 and contains no `=`.

The policy worker's environment is different from the managed process's
environment. An unconfined managed process inherits the caller's
environment. Ravel removes Ravel's own private and obsolete controls from
that environment and supplies the session-result context. The worker does not
need a grant of a credential only because the managed process uses that
credential. [Confinement](02-actors-and-words.md#trust-and-confinement)
applies its own environment rules to the managed process.
[Chapter 5, “Run sessions”](05-running-sessions.md#confine-the-managed-process),
describes these rules.

Compare the columns before you grant a name. A credential that only an
unconfined managed process uses does not need a grant to the policy worker:

| Environment rule | Policy worker | Unconfined managed process |
|---|---|---|
| Initial values | Only the caller's `HOME`, `PATH`, `TMPDIR`, `LANG` and `LC_*`, if they are present. | The caller's environment, without Ravel's private and obsolete controls. |
| Additional grants | `policyEnv` and `--policy-env` add permitted names with their values from the caller. Unset names stay unset. The exclusions above apply. | These grants do not filter or add to this environment. |
| Values that Ravel supplies | `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. No session-result context. | A new session-result context in `RAVEL_EXIT_FILE` for this launch. |

This comparison gives the environments that Ravel supplies at launch. It
does not limit what trusted policy code can read after the launch. Chapter
2's [process-authority view](02-actors-and-words.md#the-managed-process-and-its-result)
shows why the policy worker and the managed process have different
permissions. Confinement changes the rules in the managed-process column. It
does not confine the policy worker.

The next policy reports only whether two invented grant names are present.
Its output also shows how a lower policy-context budget decreases the
default read limit. The fake sends that report unchanged as the result.

Complete example: observe the settings, the CLI replacements and the
additional grants. Then observe the refusals of an obsolete key and of a
forbidden grant.

**`home/.config/ravel/settings.json`**

```json
{"timeoutMs": 5000, "contextBytes": 1024, "policyEnv": ["BOOK_FIRST"]}
```

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-settings-1",
  select(request) {
    const limits = request.limits;
    const report = [limits.selectionMs, limits.contextBytes, limits.sourceBytes,
      limits.sources, limits.messageBytes, limits.diagnosticsBytes,
      process.env.BOOK_FIRST !== undefined, process.env.BOOK_SECOND !== undefined];
    return {
      status: "selected",
      program: "fake-agent",
      args: [JSON.stringify(report)],
      provider: "local",
      model: "fake",
      effort: "none",
      reason: "Report effective bounds and grant presence.",
    };
  },
});
```

**`bin/fake-agent`**

```sh
#!/bin/sh
set -eu
[ "$#" -eq 1 ]
ravel end-session -- "$1"
```

**Command**

```console
$ mkdir -p tmp && chmod +x bin/fake-agent && env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" BOOK_FIRST=example BOOK_SECOND=example ravel start-session --kind demo --config policy.ts --prompt 'Report effective bounds and grant presence.'
```

**Standard output, without a final newline**

```text
[5000,1024,1024,256,1048576,262144,true,false]
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" BOOK_FIRST=example BOOK_SECOND=example ravel start-session --kind demo --config policy.ts --timeout-ms 6000 --context-bytes 2048 --policy-env BOOK_SECOND --prompt 'Report effective bounds and grant presence.'
```

**Standard output, without a final newline**

```text
[6000,2048,2048,256,1048576,262144,true,true]
```

**Exit status**

```text
0
```

**Command**

```console
$ rm home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" BOOK_FIRST=example BOOK_SECOND=example ravel start-session --kind demo --config policy.ts --prompt 'Report effective bounds and grant presence.'
```

**Standard output, without a final newline**

```text
[30000,262144,65536,256,1048576,262144,false,false]
```

**Exit status**

```text
0
```

**Command**

```console
$ printf '{"stateDir":"old-store"}\n' > home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (settings_invalid, stage cli): the owner settings file <scratch>/work/home/.config/ravel/settings.json holds the unknown key "stateDir"
```

**Exit status**

```text
2
```

**Command**

```console
$ rm home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind demo --config policy.ts --policy-env NODE_OPTIONS --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (excluded_grant, stage cli): --policy-env NODE_OPTIONS is never granted to the policy worker: NODE_OPTIONS, NODE_PATH and NODE_PRESERVE_SYMLINKS change what code loads, and from where
```

**Exit status**

```text
2
```

## Resolve policy imports

Use the literal import names `ravel/sdk` and `ravel/examples/static` for the
embedded modules. They resolve to the worker's embedded copies, also when a
package beside your policy supplies the same names. Ravel reserves only those
two exact names. `ravel/anything-else` is an ordinary package lookup. A
change to the installed SDK source or to a copy of the static table does not
change the embedded copies. To use a local copy, import it by its file path.

This table gives the location from which each reference resolves. A relative
import and a relative host read do not always refer to the same directory:

| Reference | Resolution base or destination |
|---|---|
| Literal `ravel/sdk` or `ravel/examples/static` import | The worker's embedded module, also beside a package with the same name. |
| Relative import such as `./routes.ts` | The directory of the file that contains the import. For a symlinked policy, the directory of its real file. |
| Ordinary package import | Bun's package resolver, from the location of the file that contains the import. |
| Package `imports` alias with an embedded name as its target | An ordinary package lookup. It does not reach the embedded module. |
| Import without a file location | The worker's process directory. That directory is `/` when owner code starts. |
| Relative `host.readText` or `host.readJson` path | The caller's `request.cwd`. |

A relative import resolves in the directory of the file that contains it. If
your entry is a symlink, the location of its real file applies. Keep a
helper beside the policy. Import it with a relative path, for example
`import { route } from "./routes.ts"`. The agent example below uses that
layout. A relative host read is different: it resolves against the caller's
`request.cwd`, as the policy-context example showed.

Bun's package resolver resolves ordinary dependencies from the location of
the file that imports them. The nearest `package.json` that Bun can read as a
package supplies the `imports` map, the package's own name and its `exports`.
Bun skips a `package.json` file that it cannot parse or that is not a
package. Resolution
through the `main` and `exports` fields of a package stays enabled. Install
the dependencies yourself, because the standalone worker does not install a
missing dependency automatically.

An alias in a package's `imports` map with the target `ravel/sdk` is still a
package lookup. It never reaches the embedded module. Thus, it can load a
package that supplies the same name instead. When you need the embedded
module, use the literal `ravel/sdk` import. The same rule applies to
`ravel/examples/static`.

The compiled worker controls which configuration files Bun loads
automatically. These controls apply at runtime:

| Configuration | Autoloading in the worker | Effect on your policy |
|---|---|---|
| dotenv files | Disabled | Bun does not load environment values from these files. |
| bunfig | Disabled | Bun does not load this configuration automatically. |
| `tsconfig.json` | Disabled | A path alias can help your editor, but that alias alone does not make a runtime import work. |
| `package.json` | Enabled | The package behaviour above, with `main` and `exports`, stays available. |

The Ravel CLI clears the runtime-injection variables and disables Bun's
runtime transpiler cache. Then it starts the worker in a private empty
directory. Before the worker imports owner code, it changes its directory to
`/`. File imports still resolve from their files. Imports without a file
location, such as a virtual module, resolve from that process directory.

The worker changes its directory to `/`. Thus, the lookups for imports
without a file location do not search the parent directories of the
temporary directory. But the resolver can
still find `/package.json` and `/node_modules`. Owner code can change its
directory or import arbitrary code itself.

If policy code creates a native Bun `Worker`, that `Worker` starts a
different runtime instance. The worker disables dotenv autoloading. Thus,
that instance does not load the dotenv files that it finds. The controls in
this section change how the worker starts and resolves modules. They do not confine trusted
owner code. The policy is never in a managed-process confinement.

## Route to Claude Code and Codex

When the checks with fakes confirm your policy decisions, a route can name an
installed agent program. Claude Code and Codex can run under the
subscription of their user, instead of with paid API access. The
[Claude Code authentication guide](https://code.claude.com/docs/en/authentication)
and [Codex authentication guide](https://learn.chatgpt.com/docs/auth)
describe those options, as checked on 18 October 2026.

Ravel treats the selected program as an ordinary executable. The program
does not have to be a
[harness](02-actors-and-words.md#the-managed-process-and-its-result), and it
does not have to receive the prompt. Any program in the session can send the
result with `end-session`. These routes select harnesses and pass the
caller's prompt to them. Ravel does not configure an account or supply a prompt
convention.

For installation and sign-in, follow the
[Claude Code quickstart](https://code.claude.com/docs/en/quickstart) or the
[Codex CLI documentation](https://learn.chatgpt.com/docs/codex/cli). This
chapter does not explain agent installation, accounts or credentials.

On 2026-10-16, the flags below agreed with the `--version` and `--help`
output of the installed programs. The programs were Claude Code 2.1.301 and
codex-cli 0.163.0. When you change a route, check the help of your installed
program again. The check of 2026-10-16 records which flags the programs
support. It is not a test that runs a real agent.

The argument column shows which agent settings the route actually changes.
The Ravel labels alone do not set those options:

| Program | Arguments this example uses | Meaning in the checked help |
|---|---|---|
| `claude` | `--effort medium`, then the prompt after `--` | Sets the effort and supplies the initial prompt. The usual CLI is interactive. `-p`/`--print` selects noninteractive output instead. |
| `codex` | `--no-daemon --no-alt-screen --sandbox workspace-write --ask-for-approval on-request`, then the prompt after `--` | Runs without the shared background server and keeps the terminal scrollback. Selects the workspace-write sandbox and lets the agent request approvals. Without a subcommand, the interactive CLI starts. |

The `--` separator causes the agent to read the prompt as a positional
argument, also when the prompt looks like an option. These routes use the
configured model defaults of the agents. The Ravel label `configured-default`
describes that decision. It does not claim a specific
resolved model. Put an explicit model choice in the agent's arguments, and
also in each Ravel label that reports it.

Ravel's `effort` label does not set the effort of the agent. The Claude route
passes a real flag. The Codex route uses the effort that Codex has in its
configuration.

<!-- tc: depart synonym-rotation because display is the terminal presentation, not a verb for showing evidence -->
The two routes start interactive agents for attended work, and each agent
needs a terminal. Run a real interactive agent from a foreground terminal.
Ravel can then
[lend the terminal](03-a-complete-session.md#launch-the-managed-job) to its
managed process. `--no-alt-screen`
changes the display of Codex. It does not change the need for a terminal.

`--no-daemon` makes this example's Codex invocation run without a shared
background server. Do not assume that Ravel supervises a process that leaves
the managed process's group. [Chapter 9, “Jobs, signals and the terminal”](09-jobs-signals-and-the-terminal.md),
explains how Ravel lends and recovers the terminal, and the process-group
boundary.

The automated example below uses fake programs without a terminal. It checks
only that the arguments arrive.

The caller's prompt contains the instructions to end the session. They give
the `ravel end-session` command and the exact opaque result to send. The
real agent needs permission to execute that command. It also needs
permission to write the path in its inherited session-result context. The
agent's own sandbox and approval settings are independent of Ravel's
settings. If the agent exits before it sends a result, the session returns
no result to the caller, also when the exit status is 0.

Put `ravel` on the agent's `PATH`, or write the installed absolute path of
`ravel` in the instructions to end the session. Ravel adds none of these
instructions itself.

Complete example: validate the two agent routes and the refusal of an
unknown kind. Fake executables return the argument vector that they receive.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";
import { route } from "./routes.ts";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-agents-1",
  select: route,
});
```

**`routes.ts`**

```ts
import type { SelectionRequest, SelectionResult } from "ravel/sdk";

export function route(request: SelectionRequest): SelectionResult {
  if (request.kind === "claude-demo") {
    return {
      status: "selected",
      program: "claude",
      args: ["--effort", "medium", "--", request.prompt],
      provider: "agent-configured",
      model: "configured-default",
      effort: "medium",
      reason: "Use the attended Claude Code route.",
    };
  }
  if (request.kind === "codex-demo") {
    return {
      status: "selected",
      program: "codex",
      args: ["--no-daemon", "--no-alt-screen", "--sandbox", "workspace-write",
        "--ask-for-approval", "on-request", "--", request.prompt],
      provider: "agent-configured",
      model: "configured-default",
      effort: "configured-default",
      reason: "Use the attended Codex route.",
    };
  }
  return {
    status: "refused",
    code: "unsupported_kind",
    message: "No agent route matches this kind.",
    remedy: "Use claude-demo or codex-demo, or add an owner route.",
  };
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
$ mkdir -p home tmp && chmod +x bin/claude bin/codex
$ for kind in claude-demo codex-demo; do
    printf '%s\n' "$kind"
    env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind "$kind" --config policy.ts --prompt 'Read the project and report one improvement. When you finish, run ravel end-session -- "reviewed" as your last action. Then do nothing else.' || exit "$?"
    printf '\n'
  done
```

**Standard output**

```text
claude-demo
<--effort>
<medium>
<-->
<Read the project and report one improvement. When you finish, run ravel end-session -- "reviewed" as your last action. Then do nothing else.>
codex-demo
<--no-daemon>
<--no-alt-screen>
<--sandbox>
<workspace-write>
<--ask-for-approval>
<on-request>
<-->
<Read the project and report one improvement. When you finish, run ravel end-session -- "reviewed" as your last action. Then do nothing else.>
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind unknown --config policy.ts --prompt 'Do nothing.'
```

**Standard output**

```text
```

**Standard error includes**

```text
  policy code: unsupported_kind
  remedy: Use claude-demo or codex-demo, or add an owner route.
```

**Exit status**

```text
3
```

The fakes intentionally send their arguments as the result. They do not act
on the prompt. The transcript checks that the caller's instructions to end
the session arrive unchanged. It does not show that an agent obeys
them, or that its sandbox lets it complete the session.

Before real use,
check that the agent can run the Ravel CLI with your chosen permissions.
Also check that it can write the result path. Keep checks with fakes for each new route.

## Copy a policy from another configuration

A copy of a policy is a configuration change. You can validate it before you
use it for real work. Keep the source and the destination available while
you reconcile them. Ravel has no init or migration verb, and it imports no
stored history.

Compare these files before you copy. Each row identifies what the
destination needs to preserve or change:

| Files | What to reconcile |
|---|---|
| Policy entry | Kinds, selected arguments, labels, owner version and refusals. |
| Relative helpers and dependencies | Imports, file locations and the packages that the helpers need. |
| `.ravel-choice` in each checkout | The names that the destination policy offers, if it calls `readChoice`. |
| `settings.json` | Supported bounds and environment grants. Remove obsolete keys such as `stateDir`. |

1. Compare the source directory with `$HOME/.config/ravel`. Keep the two
   directories. Keep all your edits in the destination. Before you copy, identify
   the entry, the relative helpers, the dependencies and the settings. A
   policy can need more than one file.
2. Copy only the missing files. Do not overwrite a file that already exists
   in the destination. Reconcile each file that is different in the source
   and the destination. Do this in the destination. Do not change the
   source. The older installation
   and its configuration can stay installed, independently of the
   destination.
3. Change the SDK imports to the literal `ravel/sdk`. Keep each relative
   helper beside the file that imports it. Install the dependencies of the
   helpers where the resolver can find them. Caller-specific helpers stay
   files that the caller owns, because Ravel embeds no caller adapter.
   Reconcile the imports of each helper too.
4. Keep the intended kind mappings, selected arguments, labels, owner version
   and refusals. If the policy uses `readChoice`, reconcile the offered
   arrangements with the `.ravel-choice` of each checkout. A copy of the
   helper does not make Ravel read that file automatically.
5. Reconcile `timeoutMs`, `contextBytes` and `policyEnv` with the
   destination's settings. Keep the supported bounds and grants. Remove
   obsolete storage settings, such as `stateDir`. Do not copy records,
   databases, snapshots or history. None of them is an input to this
   configuration.
6. Typecheck the destination against its installed declarations with the
   recipe above. Then validate each route and each refusal with fakes. Use a
   private `PATH`, a cleared environment, a temporary `HOME` and `TMPDIR`,
   and a time limit. Put the intended settings in
   `.config/ravel/settings.json` under that temporary `HOME`. `--config`
   alone does not move the settings beside your test policy. In a test copy,
   use fake paths for absolute selected programs, because a private `PATH`
   cannot intercept them.
7. Compare the captured argument vectors and the exact results with the
   intended policy decisions. Check the loader refusals and the choice
   combinations, and also the unknown kinds. When those checks pass, use the
   reconciled destination for real sessions. Give those sessions the caller's
   instructions to end the session and the chosen agent's own permissions.
