# Fixture page: a platform feature

Complete example: a command whose claim is wrong on purpose. It must be
skipped, never passed, where confinement is missing, and fail where it runs.

<!-- example: requires confinement -->

**Command**

```console
$ printf 'ran'
```

**Standard output, without a final newline**

```text
not what the command prints
```

**Exit status**

```text
0
```
