# Fixture page: cancellation by a signal

Complete example: the agent cancels the session by sending its parent, the
front, SIGINT.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-example",
  select() {
    return {
      status: "selected",
      program: "claude",
      args: [],
      provider: "fixture",
      model: "fixture",
      effort: "fixture",
      reason: "route every request to the fake claude",
    };
  },
});
```

**`bin/claude`**

```sh
#!/bin/sh
kill -INT "$PPID"
sleep 5
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.ts --prompt "Review the change."
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (session_cancelled, stage exec): session selected by <scratch>/work/policy.ts was cancelled by SIGINT
  signal: SIGINT, re-raised once this is reported
```

**Exit status**

```text
130
```

Complete example: the policy cancels the selection by sending the front
SIGINT. Only the first line of the refusal is claimed, because the worker's
reap detail further down varies from run to run.

**`policy.ts`**

```ts
export const policy = {
  schemaVersion: 2,
  version: "book-example",
  select() {
    process.kill(process.ppid, "SIGINT");
    return { status: "refused", code: "unreachable", message: "unreachable", remedy: "none" };
  },
};
```

**Command**

```console
$ ravel start-session --kind check --config policy.ts --prompt "Review the change."
```

**Standard output**

```text
```

**Standard error includes**

```text
ravel: refused (selection_cancelled, stage evaluation): selection with the policy <scratch>/work/policy.ts was cancelled by SIGINT: worker teardown was attempted and nothing was launched
```

**Exit status**

```text
130
```
