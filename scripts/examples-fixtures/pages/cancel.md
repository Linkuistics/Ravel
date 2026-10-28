# Fixture page: cancellation by a signal

Complete example: the agent cancels the session by sending its parent, the
front, SIGINT.

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "book-example";
fn select(request, context) {
    #{
      status: "selected",
      program: "claude",
      args: [],
      provider: "fixture",
      model: "fixture",
      effort: "fixture",
      reason: "route every request to the fake claude",
    }
}
```

**`bin/claude`**

```sh
#!/bin/sh
kill -INT "$PPID"
sleep 5
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.rhai --prompt "Review the change."
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (session_cancelled, stage exec): session selected by <scratch>/work/policy.rhai was cancelled by SIGINT
  signal: SIGINT, re-raised once this is reported
```

**Exit status**

```text
130
```

Complete example: the policy requests a fake harness that cancels selection
by sending the front SIGINT. Only the first line of the refusal is claimed,
because teardown details further down vary from run to run.

**`settings.json`**

```json
{"harnesses":{"cancel":{"program":"cancel-selection"}},"budgets":{"harnessRuns":1}}
```

**`policy.rhai`**

```rhai
export const SCHEMA_VERSION = 3;
export const VERSION = "book-example";
fn select(request, context) {
    ravel::run_harness(#{ harness: "cancel", args: [] });
    #{ status: "refused", code: "unreachable", message: "unreachable", remedy: "none" }
}
```

**`bin/cancel-selection`**

```sh
#!/bin/sh
kill -INT "$PPID"
while :; do sleep 1; done
```

**Command**

```console
$ mkdir -p "$HOME/.config/ravel" && cp settings.json "$HOME/.config/ravel/settings.json" && PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.rhai --prompt "Review the change."
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (selection_cancelled, stage evaluation): selection with the configuration script <scratch>/work/policy.rhai was cancelled by SIGINT: worker teardown was attempted and nothing was launched
```

**Exit status**

```text
130
```
