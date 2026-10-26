# Write an owner policy

An [owner policy](02-actors-and-words.md#caller-and-owner) chooses the program for a
[session](02-actors-and-words.md#caller-and-owner).
It can use a fixed table, read local data, or ask a model before it decides.
You write it in Rhai, a small language that Ravel embeds.
This chapter teaches the Rhai that the examples need.

The policy has no direct filesystem, network or process access.
A [host capability](02-actors-and-words.md#policy-evaluation-and-supervision) is an operation that the policy asks Ravel to perform.
Ravel supplies those operations in the `ravel` namespace.
They are available at the module's top level, in `load_context` and in `select`.

A [harness](02-actors-and-words.md#the-managed-process-and-its-result) is a program such as Claude Code or Codex.
The policy can select it as the [managed process](02-actors-and-words.md#the-managed-process-and-its-result), or run it during evaluation.
These are different uses. A [harness run](02-actors-and-words.md#policy-evaluation-and-supervision) during evaluation returns data to the policy.
The final selection starts only after evaluation ends.

Every complete example here uses fake programs. None needs a model account.

Use a fresh directory for each example. Create the listed files there.
The commands make the fake programs executable and create local `home` and `tmp`
directories. Each Ravel command uses a cleared environment and those directories.
Your personal policy and settings do not enter these examples.

The book checker runs the same examples through the
[CLI test environment](03-a-complete-session.md), with the built
[Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision).

## Define a policy and read the request

The default policy is `$HOME/.config/ravel/policy.rhai`.
`HOME` must be absolute for that default.
`--config PATH` selects another file, relative to the [caller's](02-actors-and-words.md#caller-and-owner) working directory.
Ravel searches no parent directory and tries no fallback policy.
The file must be readable UTF-8, regular, and at most 1 MiB.
A symlink identifies its canonical real file.

The module exports two constants and defines a function:

| Declaration | Purpose |
|---|---|
| `export const SCHEMA_VERSION = 3;` | Select the policy interface version. |
| `export const VERSION = "my-policy-1";` | Give the policy an owner-maintained version for the notice. |
| `fn select(request, context)` | Return a selection or a [refusal](02-actors-and-words.md#policy-evaluation-and-supervision). |
| `fn load_context(request)` | Optionally assemble policy context before `select`. |

`SCHEMA_VERSION` is independent of the Ravel release and the policy context
schema. `VERSION` is a nonblank string that you change when your policy changes.
The functions need the parameter counts shown above.
Top-level statements run once before the functions.

Complete example: pass the caller's prompt to a fake agent.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "first-policy-1";

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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "Explain this change."
```

**Standard output, without a final newline**

```text
Explain this change.
```

**Exit status**

```text
0
```

`#{ ... }` constructs a map. `request.prompt` reads a property of a map.
`[request.prompt]` constructs an array with one element.
A function's final expression, without a semicolon, is its return value.
`return value;` returns earlier.

The request has these fields. Optional inputs have the value `()` when absent.
That value means no value in Rhai and becomes `null` in JSON.

| Field | Value |
|---|---|
| `schemaVersion` | The integer `3`. |
| `kind` | The caller's [kind](02-actors-and-words.md#caller-and-owner) token, from `--kind`. |
| `prompt` | The caller's prompt, unchanged. |
| `cwd` | The caller's working directory. |
| `params` | A map of strings from `--param NAME=VALUE`. |
| `taskFile` | The absolute `--task-file` path, or `()`. Ravel does not read it. |
| `taskId` | The `--task-id` string, or `()`. |
| `context` | The caller's checked policy context document, or `()`. |
| `limits` | The effective bounds that the policy can read. |

The second argument, `context`, is the delivered
[policy context](#supply-policy-context). A loader can make it different from
`request.context`. Neither value gives the policy authority to end the session.

Ravel checks undefined variables when it compiles the policy.
It also refuses an absent map property when the policy reads it.
Check `request.params.contains("name")` before you read an optional parameter.
The documented request fields themselves are always present.

## Select a command or refuse

The selected map has seven fields, all required.
Ravel refuses an unknown field.

| Field | What the policy supplies |
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
A prompt with spaces therefore stays one argument when the policy puts it there.

The labels do not configure the program.
For example, `model: "haiku"` records a label, whereas
`args: ["--model", "haiku"]` gives a model option to a program.
The policy owns both, and Ravel does not compare them.

A refused map has `status: "refused"`, plus nonblank `code`, `message` and
`remedy` strings. Ravel reports `policy_refused`, exit status 3.
The owner's `code` appears separately as `policyCode`.
No managed process starts, and Ravel tries no other route.

Complete example: accept one kind and refuse an unknown kind or a forbidden
parameter.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "checked-route-1";

fn refuse(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

fn select(request, context) {
    if request.kind != "question" {
        return refuse("unknown_kind", "This policy accepts question only.",
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind unknown --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
```

**Exit status**

```text
3
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --param model=other --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
```

**Exit status**

```text
3
```

`if` chooses a branch. `!=` compares values for inequality.
`fn refuse(...)` is an ordinary helper function, with no special Ravel role.
Rhai functions have no parameter type declarations here.
Ravel checks the value when it crosses the boundary from the policy.

A return of `()` gives `selection_abstained`.
An uncaught error in `select` gives `selection_threw` with a source location.
A wrong field or type gives `selection_malformed`.
After any of these outcomes, Ravel starts no managed process.

<!-- tc: depart synonym-rotation because validate names the end-to-end policy exercise, while check names a single diagnostic inspection -->
## Validate a policy with fake agents

Ravel has no separate policy typecheck command or evaluation-only command.
`start-session` evaluates the policy and runs the selected program.
Use fake programs while you develop it.

For each route, test the arguments and the result.
For each refusal, test the diagnostic and absence of a managed process.
Include absent optional inputs, unknown kinds and malformed local data.
The examples in this chapter show success and failure at that same boundary.

| Observation | What to examine |
|---|---|
| A parse failure | The source file, line and column in the diagnostic. |
| `policy_invalid` | The exports, their types and the function parameter counts. |
| `selection_threw` | The property or expression at the reported location. |
| `selection_malformed` | The shape and field types of the returned map. |
| `policy_refused` | The owner's `policyCode`, message and remedy. |
| `selection_timeout` | The work before the selection and its whole deadline. |

`print(text)` writes to the policy's captured standard output.
`debug(text)` writes to the policy's captured standard error, with source information.
These streams are diagnostics. They are not the [opaque result](02-actors-and-words.md#the-managed-process-and-its-result) on Ravel's standard output.
The final managed process sends that result with `end-session`.
Use `--json` when you need the structured routing notice on standard error.

## Use the starter route table

A table makes the policy's choices visible together.
The shipped `static.rhai` provides one such table.
Its program, provider and model are placeholders that the owner replaces.

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
Ravel defines neither list.
The helper `select_route` refuses an unknown kind with `incomplete_mapping`.
When a route matches, it returns a command. The helper adds `status` and `reason`.

A smaller table can store strings directly.
`const` declares a value that the policy does not reassign.
A function reads a top-level constant through `global::`, as the next example
shows. `let` declares a local variable that the policy can change.

Complete example: route two kinds through a local table.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind feature --prompt "hello"
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind other --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
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

## Resolve policy imports

An import gives a name to another module.
`import "routes" as routes;` reads `routes.rhai` beside the file that imports it.
An absolute import uses that path.
Ravel appends `.rhai` when the path has no extension.
For a symlinked policy, imports start from the real file's directory.

Imports and measured reads use different bases:

| Operation | Base for a relative path |
|---|---|
| `import "routes" as routes;` | The directory of the file that imports it. |
| `ravel::read_text("choice.txt")` | The caller's `request.cwd`. |
| `ravel::read_json("choice.json")` | The caller's `request.cwd`. |
| `--config policy.rhai` | The caller's working directory. |

Each distinct imported module loads once per evaluation.
Ravel measures the entry and each module with its canonical path, byte count
and SHA-256 digest. The routing notice reports those measurements.
A missing module, import cycle or 65th distinct imported module gives
`policy_import_failed`.
Ravel embeds no policy module. Imports name files, which can be copies of the
shipped examples.

Complete example: call an imported function and use its exported constant.

**`policy.rhai`**

```rhai
import "routes" as routes;
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
medium|hello
```

**Exit status**

```text
0
```

## Offer a checkout choice

A policy can let a checkout choose among arrangements that the owner offers.
The shipped `sample.rhai` reads `.ravel-choice` from `request.cwd`.
That filename is a rule of the sample, not a Ravel configuration search rule.

The sample imports an adjacent copy of `static.rhai`.
An absent choice file uses `default`.
A file with exactly `careful` changes the selected effort to `high` and rebuilds
the arguments. It leaves an unknown kind as a refusal.

| Choice file | Sample behaviour |
|---|---|
| Absent | Use `default`. |
| `default` | Use the starter table unchanged. |
| `careful` | Use the starter table with effort `high`. |
| Empty or more than one offered name | Refuse with `one_choice`. |
| An unoffered name | Refuse with `choice_unoffered`. |
| An unreadable file or a directory | Refuse with `choice_unreadable`. |

The sample implements this behaviour with `ravel::read_text` and ordinary
Rhai string operations. No special choice-file API is necessary.
The next example uses a smaller choice rule so you can test every branch.

Complete example: accept `careful`, use a default when absent, and refuse any
other text.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
```

**Exit status**

```text
3
```

`try` runs a block whose ordinary error can reach `catch (error)`.
`throw error` passes an error onwards.
A measured read throws a map with `code` and `message`, and a `name` where
applicable. The policy catches only the absent file here.
`trim()` changes the string in place.

A successful read returns `text` and `source`.
`source` holds the canonical `name`, the byte count and `sha256`.
Ravel records the read in its notice, with the evaluation phase.
The default read bound is the smaller of 65,536 bytes and the context budget.
The optional second argument changes it, up to the context budget.

| Read failure | Policy can catch it? |
|---|---|
| `source_missing`, `source_not_regular`, `source_unreadable`, `source_not_utf8` | Yes. |
| `source_invalid_json` from `read_json` | Yes. |
| `source_too_large`, `too_many_sources` | No. They end evaluation. |
| A malformed host request | No. It ends evaluation as `host_request_invalid`. |

## Supply policy context

Policy context is data for the decision, not authority to execute a command.
The caller can supply a version-1 JSON document with `--context PATH`.
Without a loader, `select` receives that document with its measured source,
or `()` when no document exists.

A `load_context(request)` function can assemble a different document.
It can read the caller's input at `request.context` and read local sources.
Ravel checks its return value before it calls `select`.
A loader that returns `()` gives `context_invalid`.
A loader can also return a refusal, in which case `select` never runs.

| Document field | Purpose |
|---|---|
| `schemaVersion` | Required integer `1`. |
| `summary` | Optional text for the policy. |
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

Complete example: read task data, build policy context, and use it in the
selection.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind review --prompt "hello"
```

**Standard output, without a final newline**

```text
Review the change|high|1
```

**Exit status**

```text
0
```

`read_json` returns `value` and `source`.
The loader chooses which values enter the document.
`.len()` counts the measured records, and `.to_string()` makes a string for
`args`. Ravel refuses an integer in that array.

The context budget applies to the caller's file and to the delivered JSON,
with `measured`. The default is 262,144 bytes.
At most 256 measured sources can contribute to one evaluation.
A file counts when its bytes arrive, even if the bytes are not valid JSON or UTF-8.
An absent file does not consume a source.

## Use the data helpers

Ravel supplies strict JSON operations in the `ravel` namespace.
Use them for data that crosses between the policy and another program.
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

Complete example: check data, handle a failed check, and encode the result.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
```

**Standard output, without a final newline**

```text
["high","json_mismatch"]
```

**Exit status**

```text
0
```

`for value in array` visits each element.
`push` adds an element to the array.
The schema requires a string whose value is `low` or `high`.
It also rejects properties other than `effort`.
A schema checks shape and permitted values. The policy still decides their meaning.

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
Keep model answers as data that the policy checks and interprets.

## Set bounds and grant environment names

Ravel reads settings from `$HOME/.config/ravel/settings.json`, also with `--config`.
An absent file sets nothing. Each key is optional, and Ravel refuses unknown keys.

| Key | Meaning and range |
|---|---|
| `timeoutMs` | Whole evaluation deadline, 1,000–600,000 ms. Default: 30,000. |
| `contextBytes` | Context budget, 1–8,388,608 bytes. Default: 262,144. |
| `policyEnv` | Additional caller environment names that `ravel::env` can read. |
| `harnesses` | Named programs available to `ravel::run_harness`. |
| `budgets` | An object whose optional `harnessRuns` is an integer from 0 to 16. Default: 0. |

`--timeout-ms` and `--context-bytes` replace the settings of those bounds.
Ravel checks the settings first, so a flag cannot override an invalid file.
`--policy-env NAME` adds a grant to those in `policyEnv`.
Duplicate grants count once.

`request.limits` holds `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes`, `diagnosticsBytes` and `harnessRuns`.
These values describe the effective bounds. A change to the map cannot raise them.
`ravel::remaining()` reports what is left: time in `ms`, `harnessRuns` and
`sources`.

The whole deadline covers module loads, the context loader, selection and all
host calls. It does not bound the final managed process.
At expiry, Ravel reports `selection_timeout`, exit status 124.
Cancellation or failed harness cleanup can take precedence.
Cleanup takes additional time.
Other fixed bounds include 1 MiB worker messages and 262,144 bytes of policy
diagnostics.

| Bound reached | Effect |
|---|---|
| Whole deadline | End evaluation and start no managed process. |
| Source, context, message or diagnostics bound | End evaluation and start no managed process. |
| Engine operation, depth or data bound | End evaluation with `script_limit`. |
| Worker heap bound of 1 GiB | End evaluation with `memory_limit`. |
| Harness-run budget | End evaluation with `harness_budget_exhausted`. |
| One harness run's deadline or output bound | Return a status to the policy if the whole evaluation can continue. |

The policy cannot catch an evaluation bound with `try`.
This rule also applies inside a closure passed to an array or map helper.
A deadline or output bound of one harness run is different: it returns data,
as the next section explains.

The [policy worker](02-actors-and-words.md#policy-evaluation-and-supervision) has an empty environment.
`ravel::env` asks the Ravel CLI for the caller's value.
It permits `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*` and explicit grants.
An unset permitted name returns `()`.
An ungranted name throws `env_not_granted`, even if it is unset.
A value outside UTF-8 throws `env_not_utf8`.

Complete example: grant one variable and observe a CLI bound replacement.
The command installs settings only in the example's temporary home.

**`settings.json`**

```json
{"timeoutMs":5000,"contextBytes":4096,"policyEnv":["BOOK_EFFORT"]}
```

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "settings-1";

fn select(request, context) {
    let effort = ravel::env("BOOK_EFFORT");
    let hidden = "none";
    try { ravel::env("BOOK_HIDDEN"); }
    catch (error) { hidden = error.code; }
    #{ status: "selected", program: "fake-agent",
       args: [effort, request.limits.selectionMs.to_string(), hidden],
       provider: "example", model: "fake", effort: effort, reason: "owner settings" }
}
```

**`bin/fake-agent`**

```sh
#!/bin/sh
ravel end-session -- "$1|$2|$3"
```

**Command**

```console
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" BOOK_EFFORT=high BOOK_HIDDEN=unused PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --timeout-ms 10000 --prompt "hello"
```

**Standard output, without a final newline**

```text
high|10000|env_not_granted
```

**Exit status**

```text
0
```

Environment grants control this helper, not the environment of an unconfined
managed process or harness run. Those programs receive the caller's environment,
with the control variables of an enclosing session or driver removed or replaced.
Ravel refuses grants for its reserved variables and dynamic-loader variables.
The [reference](06-reference.md) lists the reserved classes.

A measured read can reach files that the user can read.
These grants therefore do not create a filesystem sandbox for the policy.
The owner controls the policy and chooses which data it reads.

## Run a harness during evaluation

A harness entry fixes the program that a host call can run.
Its name has 1–64 lowercase letters, digits or dashes. Its first character is a letter
or digit. The entry has a required `program` and optional `capabilities` labels.
Ravel gives those labels no meaning.

Fragment: settings for a harness run. This block omits the policy and program.

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

The call is synchronous. Only one host call runs at a time.
It runs in `request.cwd`, without a terminal, in a new POSIX session.
It uses the environment and [confinement](02-actors-and-words.md#trust-and-confinement) rules of a managed process.
`--confine` therefore also confines harness runs.

The policy supplies every argument and chooses where the prompt goes.
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

A run can finish by natural exit and return standard output to the policy.
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

A harness run receives `RAVEL_POLICY_RUN`.
`start-session` refuses inside it as `policy_reentry_refused`.
Call a policy helper directly if you need another selection decision.
The marker does not protect against code that removes it.
A descendant that leaves the process group is outside group cleanup.

## Ask a model through Claude Code

An [LLM call](02-actors-and-words.md#policy-evaluation-and-supervision) is a harness run that asks a model a question.
Claude Code uses its own sign-in. Ravel stores no model credential.
The policy reads the response and decides what it means.
The fake below uses the same arguments but returns a fixed answer.

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

The prompt goes to `stdin`, not into `args`.
The policy checks the run status and exit code before it parses standard output.
It then checks `structured_output` against the same schema.
The fixed answer permits only `yes` or `no`.
It never supplies a program name or arbitrary command argument.

Complete example: ask through a fake Claude Code and refuse an invalid answer.

**`settings.json`**

```json
{"harnesses":{"claude":{"program":"claude"}},"budgets":{"harnessRuns":1}}
```

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello" > result && cat call.args && cat call.prompt && printf '\n' && cat result
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
```

**Exit status**

```text
3
```

`type_of` reports the Rhai type name.
The map check and `contains` test protect the read of `structured_output`.
The schema rejects `perhaps`, and the policy returns its own refusal.
A different policy could choose a fixed fallback after an unusable answer.
It still cannot catch an exhausted
[evaluation budget](02-actors-and-words.md#policy-evaluation-and-supervision).

The shipped `llm.rhai` asks Claude Code to classify the caller's prompt.
It limits the answer to keys from the starter route table.
After an unusable answer, it uses the caller's kind instead.
It imports `static.rhai`, so both files must be present.

## Ask a model through Codex

Codex accepts its structured-output schema as a file path.
Keep that file where the run can read it.
The policy cannot write the schema file itself.
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

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ mkdir -p home/.config/ravel && cp settings.json home/.config/ravel/settings.json
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello" > result && cat call.args && cat call.prompt && printf '\n' && cat result
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
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind question --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
```

**Exit status**

```text
3
```

The failed run returns `status: "exited"` and `exitCode: 1` to the policy.
The policy then refuses. Ravel itself assigns no meaning to the harness's text.
A retry would need another run in the budget and time before the whole deadline.

The fake examples prove the arguments, prompt placement and the checks of each answer.
The opt-in checkout tasks `task harness:smoke:claude` and
`task harness:smoke:codex` prove the recipes with real calls.
Each spends subscription usage and needs the owner's sign-in.
Run the appropriate task after a harness upgrade.

Those tasks use a temporary configuration and links to the harness's sign-in
store. They do not need edits to the owner's live settings.
They run unconfined. Confinement can deny writes to a harness's sign-in
store, and Ravel does not widen confinement for authentication.
A schema beside the policy is also unsuitable for a confined harness run,
because confinement refuses grants that overlap the policy directory.

The harness owns its authentication.
See the [Claude Code setup documentation](https://code.claude.com/docs/en/setup)
and [Codex authentication documentation](https://learn.chatgpt.com/docs/auth)
for account setup. This chapter assumes an installed, signed-in harness for real calls.
Ravel reports no token usage. Its notice lists run metadata, not the prompt or answer.
A policy that copies an answer into diagnostics or delivered context exposes it there.

## Route to Claude Code and Codex

A final interactive harness is the managed process, not a harness run.
Select its `program` and `args` directly.
This selection needs no `harnesses` entry or harness-run budget.
The caller's prompt tells the LLM when and how to send its opaque result.

For example, the prompt can instruct the LLM to run
`ravel end-session -- "continue"` as its last action.
The caller decides what `continue` means.
The policy passes that prompt unchanged if its route uses `request.prompt`.
Ravel does not add the instruction.

| Intended use | Policy action | How it completes |
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

Complete example: pass an instruction to end the session to two fake interactive
harnesses.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
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
$ mkdir -p home tmp && chmod +x bin/* || exit "$?"
$ for kind in claude-demo codex-demo; do
    printf '%s\n' "$kind"
    env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind "$kind" --prompt 'Review the change. Then run ravel end-session -- "reviewed" as your last action.' || exit "$?"
    printf '\n'
  done
```

**Standard output**

```text
claude-demo
<--effort>
<medium>
<-->
<Review the change. Then run ravel end-session -- "reviewed" as your last action.>
codex-demo
<--no-daemon>
<--no-alt-screen>
<--sandbox>
<workspace-write>
<--ask-for-approval>
<on-request>
<-->
<Review the change. Then run ravel end-session -- "reviewed" as your last action.>
```

**Exit status**

```text
0
```

**Command**

```console
$ env -i HOME="$PWD/home" TMPDIR="$PWD/tmp" PATH="$PWD/bin:$PATH" ravel start-session --config policy.rhai --kind other --prompt "hello"
```

**Standard output**

```text
```

**Standard error includes**

```text
policy_refused
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

## Copy a policy from another configuration

Copy a Rhai policy with its imported modules and data files.
Preserve relative import paths, and check paths relative to `request.cwd` separately.
Copy only the settings entries and environment grants that the new policy needs.
The policy does not carry its own settings file through `--config`.

The shipped examples are under `share/ravel/examples/` in a Ravel 2.0.0 installation.
In a checkout, their source is `crates/ravel/examples/`.
The examples are editable copies, not embedded imports.

| Example | Files and purpose |
|---|---|
| `static.rhai` | A fixed route table with placeholder commands and labels. |
| `sample.rhai` | A checkout choice. It also needs `static.rhai`. |
| `llm.rhai` | An LLM-assisted choice through Claude Code. It needs `static.rhai` and harness settings. |

Before real use, replace placeholder commands and test every route with fake
agents. Check that the policy refuses unknown kinds as intended.
Check that absent data and unsuccessful harness runs produce the outcome you chose.
Then place the policy at your personal entry or select it with `--config`.
