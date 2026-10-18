# Fixture page: the shapes an example may take

An example set in a blockquote, as a chapter 6 note would.

> Complete example: a note holding an example, with a claimed line that itself
> starts with a quote marker.
>
> **Command**
>
> ```console
> $ printf '> quoted\n'
> ```
>
> **Standard output**
>
> ```text
> > quoted
> ```
>
> **Exit status**
>
> ```text
> 0
> ```

An example fenced with tildes.

Complete example: tilde fences.

**Command**

~~~console
$ printf 'tilde\n'
~~~

**Standard output**

~~~text
tilde
~~~

**Exit status**

~~~text
0
~~~

An example whose claimed output contains a three-backtick line, so its fence is
four backticks.

Complete example: a longer fence.

**Command**

````console
$ printf '```\n'
````

**Standard output**

````text
```
````

**Exit status**

````text
0
````

Fragment: the policy's first line only, without the export.

```ts
import { definePolicy } from "ravel/sdk";
```

A block in a language that is never an example needs no label.

```text
fn main() {}
```
