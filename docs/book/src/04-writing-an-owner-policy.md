# Writing an owner policy

An [owner policy](02-actors-and-words.md#caller-and-owner) turns a caller's
request into a command or a
[refusal](02-actors-and-words.md#policy-evaluation-and-supervision). Start with
chapter 3's fake agent and add decisions you can test: which
[kinds](02-actors-and-words.md#caller-and-owner) to accept, which arguments to
pass, and what to tell a caller when no route applies.

Use the checkout build installed in
[chapter 3](03-a-complete-session.md#obtain-the-build-used-by-this-chapter).
Each complete example below belongs in its own empty directory. Save its files
at the labelled paths, creating `bin` first. The examples create private `home`
and `tmp` directories, clear the subprocess environment and put fake programs
first on `PATH`. They run real policy evaluation without contacting a model
service or reading your normal owner configuration.

## Define a policy and read the request

The entry file exports a plain object named `policy`. Import `definePolicy`
from the literal module name `ravel/sdk` and wrap that object with it. The
helper returns the object unchanged; it gives TypeScript enough information
to check your fields and infer the parameters of `select`. Runtime validation
still happens when Ravel evaluates the policy.

The policy has four possible fields:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`, identifying the policy shape. |
| `version` | A nonblank label you maintain, such as `book-routing-1`; it is independent of the executable's version. |
| `select` | A function returning a selection or refusal, directly or through a promise. |
| `loadContext` | An optional function assembling the policy context before selection; introduced later in this chapter. |

Unknown fields refuse. The named export matters: a default export alone is
not this interface. The simplest `select` uses just its first parameter,
`request`. Its full call also receives the delivered
[policy context](02-actors-and-words.md#three-different-kinds-of-information),
which can be `undefined`, and a host with `diagnostic(text)` and an
`AbortSignal` at `signal`. The host lets asynchronous work observe the selection
deadline and lets policy write diagnostic text. It gives `select` no file-read
operations; those belong to the context loader's host.

The request is frozen data. It contains these fields, whether your policy uses
them or not:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`. |
| `kind` | The caller's nonempty token; only your policy defines which tokens it accepts. |
| `prompt` | The prompt unchanged, including an empty string or newlines. Ravel adds no ending instructions. |
| `cwd` | The caller's working directory, not the worker's directory. |
| `params` | The caller's `--param NAME=VALUE` pairs as strings by name; an empty object when none were supplied. |
| `taskFile` | Optional absolute path from `--task-file`. Ravel does not read it for you. |
| `taskId` | Optional caller identifier from `--task-id`. |
| `context` | Optional validated version-1 document from `--context`. |
| `limits` | The effective bounds for this invocation. |

An absent optional field means the caller did not supply it. `limits` names
`selectionMs`, `contextBytes`, `sourceBytes`, `sources`, `messageBytes` and
`diagnosticsBytes`: the whole-selection time, delivered-context bytes, default
bytes per host read, measured-source count, protocol-message bytes and combined
worker-output bytes. These are values to read; changing a policy-side value
cannot raise the enforced bound. Settings and their ranges are introduced later.

Fragment: the `select` method signature only, omitting the policy object and
its routing body.

```ts
select(request, context, host) {
  host.diagnostic(`Selecting a command for ${JSON.stringify(request.kind)}`);
  // Return a selected command or a refusal here.
}
```

For a private test file, pass `--config policy.ts`. Ravel resolves that path
against the caller's working directory, follows it to a readable regular file
and imports the real path. Relative helpers belong beside that real file.
Without `--config`, the entry is `$HOME/.config/ravel/policy.ts`; there is no
repository search. Settings still come from
`$HOME/.config/ravel/settings.json` with either entry, which is why the examples
supply an empty private `HOME` as well as `--config`.

## Select a command or refuse

A selection describes the actual process to start. `program` is an absolute
path, a relative path containing a separator, or a name searched on the caller's
`PATH`. Relative paths use the caller's working directory. Every string in
`args` becomes one argument after the program; Ravel does no shell expansion
or word splitting. A shell runs only when the selected command needs one, as
the fake's shebang does here.

| Selection field | Meaning |
|---|---|
| `status` | The literal `"selected"`. |
| `program` | The executable path or name. |
| `args` | The argument strings, in order; the array and its strings may be empty. |
| `provider`, `model`, `effort` | Owner labels reported in routing diagnostics; they do not configure a service or change arguments. |
| `reason` | Your explanation of this routing decision, reported verbatim in routing diagnostics. |

Every string field other than the entries in `args` is nonblank. Neither the
program nor an argument can contain a NUL. Ravel validates the selection's
shape, not whether its command is sensible or whether it passes the prompt.
Those decisions belong to your policy and its tests.

A refusal gives the caller something they can act on:

| Refusal field | Meaning |
|---|---|
| `status` | The literal `"refused"`. |
| `code` | Your nonblank code identifying the reason. |
| `message` | A nonblank explanation of what is wrong. |
| `remedy` | A nonblank action the caller or owner can take. |

Ravel reports your refusal as `policy_refused`, with your code alongside it
(`policyCode` in JSON diagnostics), exits with status 3 and launches nothing.
It does not fall back to another route. Returning nothing, returning malformed
data or throwing also prevents launch, but does not express your intended
refusal with its message and remedy. A selection deadline has its own outcome,
distinct from an explicit policy refusal.

This policy extends chapter 3's fake. It accepts only `demo` and refuses an
empty prompt for that kind. Empty prompts are legal Ravel input; this policy
chooses to require one. The order is deliberate: an unsupported kind refuses
before the prompt is examined.

Complete example: validate the accepted route and both explicit refusals of
one small policy.

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

The successful command returns the
[opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
`finished`, with no final newline. The fake's output reaches the caller's
standard error, leaving standard output for the result. Each refusal returns
no result. The standard-error excerpts check stable diagnostic lines; they do
not exclude additional operational reports.

Keep a case for every route and refusal when you edit the policy. A fake can
record the full argument vector and publish a known value, so you can check
routing without paying for a model or letting an agent act on the prompt.
`start-session` really launches the selected command; Ravel has no inspection
verb that substitutes for this validation. The examples in this book run under
`task docs:examples`, which uses a private PATH containing only the example's
fakes, the built Ravel and system utilities, with a cleared environment,
temporary HOME and TMPDIR, and a 60-second bound per command.

## Typecheck against the public declarations

Typechecking catches misspelt fields, missing fields and wrong return types
before a session runs. It does not execute the policy or prove that every kind
has a route. Combine it with the fake-program checks above.

The checkout generates the public declarations when it builds the worker.
Use those declarations for your policy, rather than reaching into worker
implementation modules. From the checkout root, generate them with `task
worker`, then create a temporary configuration that includes your policy's
absolute path. The recipe maps both embedded module names because the later
examples also import the starter table.

Fragment: checkout typechecking setup; replace the policy path. This block
omits the policy itself and the checkout prerequisite setup from chapter 3.
Run it from the checkout root, in a subshell so its cleanup trap stays local.

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

The JSON generator preserves spaces and other characters in the paths. The
checker inherits the repository's strict options, while `typeRoots` locates
the checkout's Bun declarations even though the configuration lives elsewhere.
A successful check prints no TypeScript diagnostics and exits 0.

An installation also ships `libexec/ravel/sdk/index.d.ts` and
`libexec/ravel/examples/static.d.ts`, alongside editable TypeScript sources.
It does not ship a development type checker. You can check a policy against
that installed interface using a separate TypeScript installation. The
following recipe uses the checkout's pinned checker solely as that development
tool; its module mappings point at the installed declarations. Normal installed
execution needs neither that checker nor a separately installed Bun runtime.

Fragment: installed-declaration typechecking with the checkout's checker;
replace the two absolute paths. This block omits installation and the policy.
Run it from the checkout root after its development dependencies are installed.

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

The policies on this page need no Bun-specific global types, so the standalone
configuration uses `types: []`. A policy using runtime-specific APIs needs the
corresponding development declarations too. Keep runtime validation: the
installed worker still checks the object and selection it actually receives.

## Use the starter route table

The embedded module `ravel/examples/static` supplies an editable starting
point. Its `Route` type is a function from a request to a `Command`: a selection
without `status` and `reason`. `selectRoute(table, request)` finds an own entry
whose name exactly equals the kind, calls that route and adds the selected
status and a reason naming the route. An unknown kind returns the policy code
`incomplete_mapping`; no other entry acts as a default.

The shipped `routes` table contains these owner conventions:

| Kind | Effort passed to the wrapper |
|---|---|
| `question` | `low` |
| `bugfix` | `medium` |
| `feature` | `high` |
| `migration` | `xhigh` |
| `architecture` | `xhigh` |

Each entry calls `agent(effort)`, which builds a command named
`my-agent-wrapper` with `--model your-model --effort EFFORT` followed by the
unchanged prompt as one argument. Its provider label is `your-provider`.
These names are placeholders, not installed agent integrations. The supplied
`policy` exports this table as a whole policy. You can re-export it to try it,
then copy `examples/static.ts` from `libexec/ravel` to edit your own table.
Changes to a local copy do not alter the embedded import.

The fake below publishes its received arguments as a multiline result. That
makes argument order, labels used as arguments, and the unsplit prompt visible
in the transcript. The loop checks the status of each session before starting
the next one.

Complete example: exercise every shipped table entry and its unknown-kind
refusal through a fake wrapper.

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

A policy can let a checkout choose among arrangements the owner has already
written. The SDK's `readChoice(request.cwd, offered)` reads `.ravel-choice`
in that directory and returns whitespace-separated names. With no file it
returns `undefined`, so the policy supplies the default. It does not search
parent directories. An empty file returns an empty array, not `undefined`.

The helper accepts a regular UTF-8 file of at most 4 KiB. An unreadable,
oversized, invalid UTF-8 or non-regular file returns `choice_unreadable`; a name
outside `offered` returns `choice_unoffered`. Pass that refusal back from
`select`. The exported constant `CHOICE_FILE` holds the filename. The helper
performs an ordinary filesystem read, not a measured policy-context read;
record the applied choice in the selection's reason if you need it explained.

Nothing reads this file unless your policy calls the helper. The file can
choose offered names, but cannot introduce a program or arguments. Your code
decides how many names are allowed and what each means.

The editable sample is `crates/ravel/worker/sample/policy.ts` in the checkout,
and `libexec/ravel/examples/sample.ts` in an installation. The example below
uses its routing logic: absent choice means `default`, exactly one name is
required, and `careful` sets the selected effort and its argument to `high`.
That is a fixed override, not a minimum: it also replaces the table's `xhigh`
for `migration` and `architecture`. Edit the names and behaviour together if
that is not the arrangement you want.

Complete example: validate both sample arrangements across all five routes,
then its kind, choice-count, unoffered-name and unreadable-file refusals.

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

The `<scratch>/work` prefix represents the temporary working directory used by
the book's checker; your diagnostic names your example directory. The last
case makes `.ravel-choice` a directory, so it exercises the non-regular-file
refusal without depending on permissions. Remove that directory with `rmdir`
before restoring a choice file. The other unreadable-file conditions share
`choice_unreadable`; this example demonstrates that code, not every filesystem
failure that can produce it.
