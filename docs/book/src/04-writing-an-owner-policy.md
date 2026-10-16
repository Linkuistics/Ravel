# Writing an owner policy

An [owner policy](02-actors-and-words.md#caller-and-owner) turns a request
from a [caller](02-actors-and-words.md#caller-and-owner) into a command or a
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

Use these four fields to identify the policy interface, label your revision
and supply the functions Ravel calls:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`, identifying the policy shape. |
| `version` | A nonblank label you maintain, such as `book-routing-1`; it is independent of the executable's version. |
| `select` | A function returning a selection or refusal, directly or through a promise. |
| `loadContext` | An optional function assembling the policy context before selection; introduced later in this chapter. |

Unknown fields refuse. Ravel reads only the named export `policy`; an entry with
only a default export is rejected as an invalid policy. The simplest `select`
uses just its first parameter, `request`. Its full call also receives the
delivered
[policy context](02-actors-and-words.md#three-different-kinds-of-information),
which can be `undefined`, and a host with `diagnostic(text)` and an `AbortSignal` at
`signal`. The host lets asynchronous work observe the selection deadline and
lets policy write diagnostic text. It gives `select` no file-read operations;
those belong to the context loader's host.

The request is frozen data. Use this table to find the caller input your
decision needs and distinguish it from Ravel's effective bounds:

| Field | Meaning |
|---|---|
| `schemaVersion` | The literal number `2`. |
| `kind` | The caller's nonempty token; only your policy defines which tokens it accepts. |
| `prompt` | The prompt unchanged, including an empty string or newlines. Ravel adds no ending instructions. |
| `cwd` | The caller's working directory, not the [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision)'s directory. |
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

Use the selection fields to separate the command Ravel executes from the
labels and explanation it reports:

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

A refusal needs both an identifiable reason and an action the caller can take;
the fields below carry those parts together:

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
refusal with its message and remedy. Exceeding the selection deadline has its
own outcome, `selection_timeout`, distinct from an explicit policy refusal.

Read this table from top to bottom along the accepted path: returning a
selection does not bypass worker cleanup. Stopped evaluation also requires
cleanup, but cannot lead to a managed launch.

| Order | Work | What must hold to continue |
|---|---|---|
| 1 | Import the owner policy. | The import succeeds and the named `policy` export has the accepted shape. |
| 2 | Assemble [policy context](#supply-policy-context), calling `loadContext` if present. | Caller data and the delivered document meet their shape and size rules. A loader refusal stops here; `select` is not called. Without a loader or caller document, selection proceeds with `undefined`. |
| 3 | Call `select` with the request and delivered policy context. | The returned value is a valid selection. An explicit refusal becomes `policy_refused`; malformed data or a thrown error also prevents launch. |
| 4 | Settle the policy worker before admitting the managed launch. | Ravel confirms worker reap and private-directory removal; a returned selection alone cannot authorise launch after failed cleanup. |

One deadline, `selectionMs`, runs from worker spawn across import, policy-context
assembly and `select`; it does not restart for each row. Expiry stops evaluation
with `selection_timeout` (exit 124), distinct from refusal (exit 3), and launches
nothing. An invalid caller document can refuse before the loader runs. This
table shows the policy-to-launch order; chapter 3's
[outcome view](03-a-complete-session.md#recognise-the-other-outcomes) includes
cancellation and supervision outcomes.

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
before a [session](02-actors-and-words.md#caller-and-owner) runs. It does not
execute the policy or prove that every kind has a route. Combine it with the
fake-program checks above.

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

Generating the file with `JSON.stringify` preserves spaces and other
characters in the paths. The
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

The policies introduced so far need no Bun-specific global types, so the
standalone configuration uses `types: []`. A policy using runtime-specific APIs
needs the corresponding development declarations too. Typechecking does not
replace runtime validation: the installed worker still checks the object and
selection it actually receives.

## Use the starter route table

The embedded module `ravel/examples/static` supplies an editable starting
point. Its `Route` type is a function from a request to a `Command`: a selection
without `status` and `reason`. `selectRoute(table, request)` finds an own entry
whose name exactly equals the kind, calls that route and adds the selected
status and a reason naming the route. An unknown kind returns a refusal with
the policy code `incomplete_mapping`; no other entry acts as a default.

Use this mapping to predict the effort argument each shipped route passes;
these kind names and effort choices are owner conventions:

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
These names are placeholders, not installed agent integrations. The module's
exported `policy` wraps this table as a complete policy. You can re-export it
to try it,
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
in that directory and returns the names it lists, split on whitespace, as an
array. With no file it
returns `undefined`, so the policy supplies the default. It does not search
parent directories. An empty file returns an empty array, not `undefined`.

The helper accepts a regular UTF-8 file of at most 4 KiB. An unreadable,
oversized, invalid UTF-8 or non-regular file returns a refusal with the code
`choice_unreadable`; a name outside `offered` returns one with
`choice_unoffered`. Pass that refusal back from
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

## Supply policy context

A caller can supply a JSON document with `--context PATH`. The path is relative
to the caller's working directory and names a readable regular file. Its
`schemaVersion` is `1`, independently of the policy's version-2 interface.
The document carries facts and attributed judgements that your policy can use
to choose among commands it defines. It does not grant permission to run a
command named by those facts.

Use these fields to carry facts and attributed judgements while keeping
executable choices in owner code:

| Field | Accepted data |
|---|---|
| `schemaVersion` | Required: the number `1`. |
| `summary` | Optional string; it can be empty. |
| `acceptanceCriteria` | Optional array of strings. |
| `facts` | Optional object whose values are JSON data. |
| `assessments` | Optional object whose values are `{ by, value }`: a nonblank author name and JSON data. |
| `sources` | Optional array of at most 256 source records. Each has a nonblank `name` and identifies the source with `sha256`, a nonblank `version`, or both. An optional `bytes` is a nonnegative whole number. A digest is 64 lowercase hexadecimal digits. |

Absent means unknown. For example, an absent `facts` differs from a known empty
object. Unknown top-level fields refuse, including `program`, `args`, `argv`,
`command`, `select`, `loadContext`, `env`, `environment`, `exec`, `shell` and
`script`. Nested JSON facts remain data even if a key has one of those names.
Neither the caller nor the loader supplies the `measured` field: Ravel adds it.
One JSON object shape is reserved even inside facts or assessment values: an
object whose only key is `$ravel`. Ravel uses that shape to mark values JSON
cannot carry and refuses it as `context_invalid`; choose another key for your
own data.

Without `loadContext`, `select` receives the caller's document with that
measurement attached, or `undefined` when the caller supplied none. With a
loader, `select` receives what the loader returns instead. The loader receives
`request.context`, so it can explicitly retain or replace the caller's data;
Ravel does not merge the two documents for you. The loader can be asynchronous
and shares the selection deadline with policy import and `select`.

Choose the row matching your caller input and policy: a loader replaces the
document delivered to `select`, unless its own code retains the caller's data.
These rows assume the documents pass validation and the loader returns a document.

| Caller document? | Policy has `loadContext`? | Policy context delivered to `select` |
|---|---|---|
| No | No | `undefined`. |
| Yes | No | The caller's document, with Ravel's `measured` ledger attached. |
| No | Yes | The loader's returned document, with `measured` attached. The loader sees no `request.context`. |
| Yes | Yes | The loader's returned document, with `measured` attached. The loader sees the caller's document at `request.context` and decides what to retain; there is no automatic merge. |

Choose the synchronous host read by the value your loader needs; both return
a source record identifying the bytes read:

| Operation | Return value |
|---|---|
| `host.readText(path, maxBytes?)` | `{ text, source }`, decoding the file as UTF-8. |
| `host.readJson(path, maxBytes?)` | `{ value, source }`, decoding UTF-8 and parsing JSON. Validate the resulting JSON for your policy's own needs. |

Both resolve relative paths against `request.cwd`, require a regular file and
return a `source` with its canonical `name`, actual `bytes` and `sha256`.
The default read limit is 65,536 bytes, reduced to the context budget if that is
smaller. A positive whole-number `maxBytes` can set a read's limit up to the
context budget. A read exceeding its limit, requesting more than the budget,
or exceeding the measured-source count refuses even if the loader catches its
exception. A missing file, bad UTF-8 or invalid JSON throws; if that error
escapes the loader, the refusal identifies the unreadable source. A loader
that requires valid content can return its own refusal, just as `select` can;
then `select` is not called.

Ravel attaches a `measured` array to the frozen delivered document. Its records
have `name`, `bytes`, `sha256` and `via` (`"--context"`, `"readText"` or
`"readJson"`). The caller's document is first, then successful host reads in
order, including repeated reads of the same file. The limit is 256 measured
records, counting the caller's document. The document's own `sources` array
has a separate 256-record limit. The entire delivered JSON, including
`measured`, has to fit the context budget; a small input file can still produce
an oversized delivered document once its measurement is attached.

A `sources` record or an assessment's `by` attributes a claim. A host measurement
identifies the bytes read. Neither authenticates the author or proves the claim
true. Trusted owner code can also read files or the network directly, but those
reads are outside this measurement ledger. Keep routing authority in owner
code: use outside information to choose a defined route, rather than treating
outside text as a program or an argument template to execute.

This example retains caller data and adds a local readiness fact and summary.
The JSON readiness file is checked before the text is read. The fake reports
the summary and the measurement methods, so the two successful commands show
what changes when the caller supplies a document.

Complete example: assemble policy context, validate its required fact, and
exercise caller-data, unreadable-source and read-bound refusals.

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

The final command refuses the caller's invalid document before running the
loader, so the oversized text file from the preceding command is not read.
These examples exercise the policy's one accepted route and explicit refusal,
plus representative Ravel refusals. The complete policy-context diagnostic
vocabulary is below; all these codes exit 3 and launch nothing.

Use the code to locate the input, loader behaviour or bound that needs changing:

| Code | What to correct |
|---|---|
| `context_unreadable` | The `--context` path cannot be read as a regular file. |
| `context_invalid` | The document is not valid JSON or has the wrong shape; the loader returns invalid or unserialisable data or no document, or supplies `measured` itself. |
| `unsupported_version` | A nonnegative integer representable as an unsigned 64-bit value names a context schema version other than 1. Negative, fractional or wrong-type versions are `context_invalid`. |
| `context_loader_failed` | The loader throws or rejects for a reason other than a recognised source-read failure or bound breach. |
| `context_loader_unsettled` | The loader's promise remains pending after no work is left that could settle it. |
| `context_source_unreadable` | A required host read fails, including bad UTF-8 or JSON, and that failure escapes the loader. |
| `source_too_large` | A host read exceeds its limit or asks for a limit above the context budget. |
| `too_many_sources` | Either the measured ledger or the document's attributed `sources` exceeds 256 entries. |
| `context_too_large` | The caller document or the delivered document exceeds the context budget. |
| `policy_refused` | The loader returns a well-formed refusal; its own code, message and remedy explain the requirement. |

A promise that keeps work alive until the deadline has the `selection_timeout`
outcome instead. Invalid host arguments can produce a loader failure; a malformed
refusal returned by the loader is invalid context. Chapter 6, Reference, supplies
the complete lookup entries, including failures shared with policy selection.

## Set bounds and grant environment names

Owner settings live at `$HOME/.config/ravel/settings.json`, even when
`--config` chooses another policy. The file is optional; an absent file or an
empty JSON object selects the defaults. If `HOME` is missing or relative, no
settings file is read; the default policy entry still needs an absolute `HOME`,
so use an explicit entry in that case.

Use this table to choose an owner default and see whether a caller replaces
that value or adds to it:

| Key | Value and default | Caller adjustment |
|---|---|---|
| `timeoutMs` | Whole-number milliseconds, 1,000–600,000 inclusive; default 30,000. | `--timeout-ms` replaces it. |
| `contextBytes` | Whole-number bytes, 1–8,388,608 inclusive; default 262,144 (256 KiB). | `--context-bytes` replaces it. |
| `policyEnv` | Array of environment-variable names; default no extra grants. | Each `--policy-env NAME` adds one name. |

Settings are checked as a whole before overrides are applied. A valid CLI
replacement does not rescue an invalid setting. An unreadable or non-regular
settings file, invalid JSON, a non-object document, a `policyEnv` value other
than an array of strings, or an unknown key
produces `settings_invalid` (exit 2). An invalid numeric bound or malformed
grant name produces `malformed_input` (exit 2). Unknown keys include obsolete
`stateDir`; remove it rather than pointing it at an old store. No storage is
opened. CLI numeric bounds accept only plain decimal digits within the range,
with no sign, fraction, exponent or whitespace; values are never clamped.

The request reports six effective limits. The first two settings affect the
others only as shown here; they do not bound the
[managed process](02-actors-and-words.md#the-managed-process-and-its-result)'s
running time.

| Request field | Effective bound | How it is set |
|---|---|---|
| `selectionMs` | Policy import, loader and selection together, timed from worker spawn. | `timeoutMs`, then a replacing `--timeout-ms`; otherwise 30,000 ms. |
| `contextBytes` | Caller document and delivered policy-context JSON, including measured records. | `contextBytes`, then a replacing `--context-bytes`; otherwise 262,144 bytes. |
| `sourceBytes` | Default bytes for one host read. | The smaller of 65,536 and `contextBytes`; a read can set `maxBytes` from 1 through `contextBytes`. |
| `sources` | Measured records, including the caller document; separately, attributed source records. | Fixed at 256 for each array. |
| `messageBytes` | A policy snapshot or selection result in the private protocol. | Fixed at 1,048,576 bytes (1 MiB). The context has its own budget. |
| `diagnosticsBytes` | Worker standard output and standard error combined. | Fixed at 262,144 bytes (256 KiB). |

Overflow refuses rather than truncating data to fit. A context or read overflow
uses the codes above; a policy snapshot or selection exceeding its protocol
bound uses `message_too_large`, and excessive worker output uses
`output_limit` (both exit 3). For excessive output Ravel retains only the first
262,144 diagnostic bytes and discards the rest while stopping the worker; this
does not admit the selection. Exceeding the selection deadline is
`selection_timeout` (exit 124). A caller can increase an adjustable bound only
within its hard range; the policy cannot raise a bound by changing request data.

The policy worker receives the caller's `HOME`, `PATH`, `TMPDIR`, `LANG` and
`LC_*` variables when present. Grant additional names only when policy code
needs their values, for example to make a network request while choosing a
route. The grant is a name, never `NAME=value`; its value comes from the
[native front](02-actors-and-words.md#policy-evaluation-and-supervision)'s
environment. Duplicate names collapse to one grant. A name that is unset stays
unset. Grant diagnostics report names and whether they are set, never values;
your own policy can still disclose a value if it prints one.

The following exclusions are case-sensitive and apply to both settings and
CLI grants. An excluded name produces `excluded_grant` (exit 2).

| Name or prefix pattern | Why it is excluded |
|---|---|
| `BUN_*` | Runtime startup and options, including preloads and runtime mode. |
| `NODE_OPTIONS`, `NODE_PATH`, `NODE_PRESERVE_SYMLINKS` | Runtime startup and module resolution. |
| `NODE_CHANNEL_*` | Runtime IPC descriptors and channel options. |
| `LD_*`, `DYLD_*` | Dynamic-loader inputs. |
| `RAVEL_*` | Ravel's own controls, including the [session-result context](02-actors-and-words.md#three-different-kinds-of-information). |
| `HARNESS_DISPATCH_*` | Legacy dispatch controls. |

Ravel itself sets `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` in the worker; an owner
grant cannot replace it. The exclusions match these exact names and prefixes,
not every name beginning with `NODE` or `BUN`. A grant name must be nonempty
UTF-8 and contain no `=`. The policy worker's environment is distinct from the
managed process's: an unconfined managed process inherits the caller's
environment after Ravel removes its private and obsolete controls and supplies
the session-result context. The worker needs no grant of a credential merely
because the managed process uses that credential.
[Confinement](02-actors-and-words.md#trust-and-confinement) applies its own
environment rules to the managed process, taught in chapter 5, Running
sessions.

Compare the columns before granting a name: a credential needed only by an
unconfined managed process does not need a policy-worker grant.

| Environment rule | Policy worker | Unconfined managed process |
|---|---|---|
| Starting values | Only the caller's `HOME`, `PATH`, `TMPDIR`, `LANG` and `LC_*`, when present. | The caller's environment, with Ravel's private and obsolete controls removed. |
| Additional grants | `policyEnv` and `--policy-env` add permitted names with their caller values; unset names stay unset. The exclusions above apply. | These grants do not filter or add to this environment. |
| Ravel-supplied values | `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; no session-result context. | A fresh session-result context in `RAVEL_EXIT_FILE` for this launch. |

This comparison describes the environments Ravel supplies at launch, not a
restriction on what trusted policy code can read afterwards. Chapter 2's
[process-authority view](02-actors-and-words.md#the-managed-process-and-its-result)
shows why the policy worker and managed process hold different authority.
Confinement changes the managed-process column's rules; it does not confine
the policy worker.

The next policy reports only whether two invented grant names are present.
Its output also shows how a lowered context budget reduces the default read
limit. The fake publishes that report exactly.

Complete example: observe settings, CLI replacements and additional grants,
then reject an obsolete key and a forbidden grant.

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
embedded modules. They resolve to the worker's embedded copies even when a
package beside your policy provides those same names. Ravel reserves
only those two exact names; `ravel/anything-else` is an ordinary package lookup.
Editing installed SDK source or a copied static table does not change the
embedded copies. Import a local copy by its file path when you intend to use it.

Use this mapping to choose the location a name is resolved from; a relative
import and a relative host read need not refer to the same directory.

| Reference | Resolution base or destination |
|---|---|
| Literal `ravel/sdk` or `ravel/examples/static` import | The worker's embedded module, even beside a package with the same name. |
| Relative import such as `./routes.ts` | The importing file's directory; a symlinked policy uses its real file. |
| Ordinary package import | Bun's package resolver, starting from the importing file's location. |
| Package `imports` alias targeting an embedded name | An ordinary package lookup; it does not reach the embedded module. |
| Import without a file location | The worker's process directory, initially `/` when owner code starts. |
| Relative `host.readText` or `host.readJson` path | The caller's `request.cwd`. |

Relative imports resolve beside the importing file. If your entry is a symlink,
its real file determines that location. Keep a helper beside the policy and
write, for example, `import { route } from "./routes.ts"`. The agent example
below exercises that layout. A relative host read is different: it resolves
against the caller's `request.cwd`, as the context example showed.

Ordinary dependencies resolve from their importing file's location through Bun's
package resolver. The nearest `package.json` that reads as a package supplies
the `imports` map and the package's own name and `exports`; an unparseable or
non-package file is passed over. Resolution through a package's `main` and
`exports` fields remains enabled. Install dependencies yourself: the standalone
worker does not install a missing dependency automatically.

An alias in a package's `imports` map whose target is `ravel/sdk` is still a
package lookup. It never reaches the embedded module, so it can load a package
that supplies the same name instead. Use the literal `ravel/sdk` import when you
need that embedded identity. The same qualification applies to
`ravel/examples/static`.

The compiled worker disables dotenv, bunfig and tsconfig autoloading. A
`tsconfig.json` path alias that helps your editor therefore does not by itself
make a runtime import work. `package.json` autoloading stays enabled for the
package behaviour above. The native front clears runtime-injection variables and
disables Bun's runtime transpiler cache before starting the worker in a private
empty directory. Before importing owner code, the worker changes its directory
to `/`. File imports still resolve from their files; imports without a file
location, such as a virtual module, resolve from that process directory.

Changing to `/` keeps the temporary directory's ancestors out of those no-file
lookups;
it leaves `/package.json` and `/node_modules` within the resolver's reach.
Owner code can change directory or import arbitrary code itself. A native Bun
`Worker` created by policy code starts another runtime instance; the disabled
dotenv switch prevents that instance from loading ambient dotenv files.
These controls govern startup and resolution, not confinement of trusted owner
code. The policy still runs outside any managed-process confinement.

## Route to Claude Code and Codex

Once the fake checks establish your routing, a route can name an installed
agent program. Ravel treats it as an executable: it neither configures an
account nor supplies a prompt convention. Follow the
[Claude Code quickstart](https://code.claude.com/docs/en/quickstart) or
[Codex CLI documentation](https://learn.chatgpt.com/docs/codex/cli) for
installation and sign-in. Agent installation, accounts and credentials are
outside this chapter.

The flags below were checked on 2026-10-16 against the installed programs'
own `--version` and `--help`: Claude Code 2.1.301 and codex-cli 0.163.0.
Recheck your installed help when changing a route. This check records flag
support; it is not a real-agent execution test.

Use the argument column to see which agent settings the route actually
changes; the Ravel labels alone do not set those options.

| Program | Arguments this example uses | Meaning in the checked help |
|---|---|---|
| `claude` | `--effort medium`, then the prompt after `--` | Sets effort and supplies the initial prompt. The normal CLI is interactive; `-p`/`--print` selects noninteractive output instead. |
| `codex` | `--no-daemon --no-alt-screen --sandbox workspace-write --ask-for-approval on-request`, then the prompt after `--` | Runs without the shared background server, preserves terminal scrollback, selects the workspace-write sandbox and lets the agent request approvals. Omitting a subcommand selects the interactive CLI. |

The `--` separator keeps an option-looking prompt positional. These routes use
the agents' configured model defaults; the Ravel label `configured-default`
describes that decision, rather than claiming a particular resolved model.
An explicit model choice belongs in the agent's arguments as well as any Ravel
label you use to report it. Ravel's `effort` label does not set agent effort:
the Claude route passes a real flag, while the Codex route leaves it configured.

Both routes start interactive agents for attended work, and each needs a
terminal. Run a real interactive agent from a foreground terminal so Ravel can
lend the terminal to its managed process. `--no-alt-screen` changes Codex's
display, not whether it needs a terminal. `--no-daemon` keeps this example's
Codex invocation out of a shared background server; do not assume a process that
escapes the managed process's group is supervised by Ravel. Chapter 9, Jobs,
signals and the terminal, explains terminal lending, recovery and the
process-group boundary. The automated example below uses fake programs without a
terminal and establishes argument delivery only.

The caller's prompt carries the ending instructions: the `ravel end-session`
command and the exact opaque result to publish. The real agent needs permission
to execute that command and to write the path in its inherited session-result
context; its own sandbox and approval settings are independent of Ravel's.
If the agent exits without publishing, even with status 0, Ravel reports no
result. Put `ravel` on the agent's `PATH`, or write its installed absolute path
in the ending instructions. Ravel inserts none of these instructions itself.

Complete example: validate both agent routes and the unknown-kind refusal,
using fake executables that return the received argument vector.

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
    env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --kind "$kind" --config policy.ts --prompt 'Read the project and report one improvement. When done, run ravel end-session -- "reviewed" as your last action, then do nothing else.' || exit "$?"
    printf '\n'
  done
```

**Standard output**

```text
claude-demo
<--effort>
<medium>
<-->
<Read the project and report one improvement. When done, run ravel end-session -- "reviewed" as your last action, then do nothing else.>
codex-demo
<--no-daemon>
<--no-alt-screen>
<--sandbox>
<workspace-write>
<--ask-for-approval>
<on-request>
<-->
<Read the project and report one improvement. When done, run ravel end-session -- "reviewed" as your last action, then do nothing else.>
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

The fakes deliberately publish their arguments instead of acting on the prompt.
The transcript checks that the caller's ending instructions arrive unchanged; it
does not show an agent obeying them or prove its sandbox permits completion.
Before real use, check the agent's access to the Ravel executable and result
path under your chosen permissions, and retain fake checks for every new route.

## Copy an existing policy

A policy copy is a configuration change you can validate before using it for
real work. Keep the source and destination available while reconciling them.
Ravel has no init or migration verb and imports no stored history.

1. Compare the source directory with `$HOME/.config/ravel`. Preserve both
   directories and any destination edits. Identify the entry, relative helpers,
   dependencies and settings before copying; a policy may need more than one
   file.
2. Copy only missing files, without overwriting existing destination files.
   Reconcile differing files explicitly in the destination, keeping the source
   intact. The older installation and its configuration can stay installed,
   independently of the destination.
3. Change SDK imports to the literal `ravel/sdk`. Keep relative helpers beside
   their importing files and install their dependencies where the resolver can
   find them. Caller-specific helpers remain caller-owned files: Ravel embeds
   no caller adapter. Reconcile each helper's imports too.
4. Preserve the intended kind mappings, selected arguments, labels, owner
   version and refusals. If the policy uses `readChoice`, reconcile the offered
   arrangements with each checkout's `.ravel-choice`; copying the helper does
   not make Ravel read that file automatically.
5. Reconcile `timeoutMs`, `contextBytes` and `policyEnv` with the destination's
   settings. Keep supported bounds and grants, and remove obsolete storage
   settings such as `stateDir`. Copy no records, databases, snapshots or history;
   none is an input to this configuration.
6. Typecheck the destination against its installed declarations using the recipe
   above. Then validate every route and refusal with fakes on a private `PATH`,
   a cleared environment, temporary `HOME` and `TMPDIR`, and bounded execution.
   Reproduce the intended settings under that temporary `HOME`: `--config`
   alone does not move settings beside your test policy. Replace any absolute
   selected program paths with fake paths in a test copy; a private `PATH`
   cannot intercept an absolute executable.
7. Compare the captured argument vectors and exact results with the intended
   routing. Check loader refusals and choice combinations as well as unknown
   kinds. Once those checks pass, use the reconciled destination for real
   sessions with the caller's ending instructions and the chosen agent's own
   permissions.
