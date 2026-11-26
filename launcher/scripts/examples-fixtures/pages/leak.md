# Fixture page: the environment is cleared

Complete example: a variable set in the caller's environment does not reach
the example, and neither does anything else the caller holds.

**Command**

```console
$ if [ -z "${LEAK_CANARY+x}" ]; then echo clean; else echo leaked; fi
```

**Standard output**

```text
clean
```

**Exit status**

```text
0
```

Complete example: the seam's own environment, as the tests' `Fixture` sets it,
and nothing it does not set.

**Command**

```console
$ echo "$HOME $TMPDIR $PATH"
```

**Standard output**

```text
<scratch>/home <scratch>/tmp <scratch>/ravel:/usr/bin:/bin
```

**Exit status**

```text
0
```

Complete example: standard input is empty, not the caller's.

**Command**

```console
$ cat
```

**Standard output**

```text
```

**Exit status**

```text
0
```
