# Fixture page

This page is a control for `scripts/examples.sh`. It is not part of the book.

## A launched session

Complete example: route every request to a fake `claude`, which publishes
`continue`.

**`policy.ts`**

```ts
import { definePolicy } from "ravel/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "book-example",
  select(request) {
    return {
      status: "selected",
      program: "claude",
      args: ["-p", request.prompt],
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
pwd >&2
ravel end-session -- "continue"
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config policy.ts --prompt "Review the change."
```

**Standard output, without a final newline**

```text
continue
```

**Standard error includes**

```text
ravel: starting provider fixture, model fixture, effort fixture for kind "check"
<scratch>/work
```

**Exit status**

```text
0
```

## A refusal

Complete example: a policy that refuses every request, and the refusal text.

**`policy.ts`**

```ts
export const policy = {
  schemaVersion: 2,
  version: "book-example",
  select() {
    return {
      status: "refused",
      code: "no_route",
      message: "no route for this kind",
      remedy: "choose a kind the policy routes",
    };
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

**Standard error**

```text
ravel: refused (policy_refused, stage selection): the policy <scratch>/work/policy.ts refused the selection: no route for this kind
  input: --kind check
  source: <scratch>/work/policy.ts
  policy code: no_route
  remedy: choose a kind the policy routes
```

**Exit status**

```text
3
```

## Plain shell and a fragment

Complete example: a command that prints two lines, with a final newline.

**Command**

```console
$ printf 'one\ntwo\n'
```

**Standard output**

```text
one
two
```

**Exit status**

```text
0
```

Fragment: the selection function alone, without the import or the policy
export.

```ts
select(request) {
  return { status: "refused", code: "no_route", message: "", remedy: "" };
}
```
