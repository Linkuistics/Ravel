# Fixture page

This page is a control for `scripts/examples.sh`. It is not part of the book.

## A launched session

Complete example: route every request to a fake `claude`, which publishes
`continue`.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "book-example";
fn select(request, context) {
    #{
      status: "selected",
      program: "claude",
      args: ["-p", request.prompt],
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
pwd >&2
ravel end-session -- "continue"
```

**Command**

```console
$ PATH="$PWD/bin:$PATH" ravel start-session --kind check --config config.rhai --prompt "Review the change."
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

Complete example: a configuration script that refuses every request, and the refusal text.

**`config.rhai`**

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "book-example";
fn select(request, context) {
    #{
      status: "refused",
      code: "no_route",
      message: "no route for this kind",
      remedy: "choose a kind the policy routes",
    }
}
```

**Command**

```console
$ ravel start-session --kind check --config config.rhai --prompt "Review the change."
```

**Standard output**

```text
```

**Standard error**

```text
ravel: refused (config_refused, stage selection): the configuration script <scratch>/work/config.rhai refused the selection: no route for this kind
  input: --kind check
  source: <scratch>/work/config.rhai
  config code: no_route
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

Fragment: the selection function alone, without the exported constants.

```rhai
fn select(request, context) {
  #{ status: "refused", code: "no_route", message: "", remedy: "" }
}
```
