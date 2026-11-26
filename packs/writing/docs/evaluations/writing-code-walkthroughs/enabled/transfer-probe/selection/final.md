**Frozen Selection**

Project and upstream URL: `junegunn/fzf`  
Upstream: <https://github.com/junegunn/fzf>

Immutable commit: `15f64c492a08f0840b81540c7d1de35737448086`  
Release anchor: `v0.74.3`

License: MIT

Exact production-source paths:
- `bin/fzf-tmux`

Authoritative raw URLs:
- <https://raw.githubusercontent.com/junegunn/fzf/15f64c492a08f0840b81540c7d1de35737448086/bin/fzf-tmux>

Total line count:
- `bin/fzf-tmux`: 256 total lines
- GitHub file metadata reports `233 loc`, which keeps it comfortably under the 350-line ceiling even under a stricter nonblank-style budget

Why this subject satisfies the bounds:
- It is a real shipped production utility in a mature open-source project.
- It has a concrete observable purpose: launch `fzf` inside a tmux split or popup while preserving stdin/stdout behavior.
- The complete implementation fits in a single text source file.
- It supports a worked execution with at least three causal stages:
  1. Resolve environment and dependencies (`fzf`, terminal size, tmux version, mode).
  2. Parse layout/options and derive popup vs split configuration.
  3. Set up temp files/FIFOs, launch the child process, proxy I/O, and clean up tmux state.
- The control flow is nontrivial: multiple option families, tmux-version gating, tty vs piped stdin handling, popup vs split branches, traps, cleanup, and fallback behavior.

Byte-exact user prompt for both arms:
```text
You are preparing an authoring plan for a complete-source Markdown code walkthrough, not the walkthrough itself.

Audience: experienced software engineers who are comfortable with Unix shells and terminal tooling but have not read this utility’s implementation before.

The supplied source inventory in this fresh run directory is complete and authoritative for the utility under review:
- bin/fzf-tmux

Use read-only inspection only inside this fresh run directory. Do not fetch, browse, or assume any other source material.

Produce an authoring plan that includes:
1. The exact source inventory you will treat as the full subject.
2. A concept-oriented reader sequence for the walkthrough.
3. A fragment ledger that enumerates the source fragments you would quote or paraphrase, with explicit insertion relationships showing where each fragment would appear relative to earlier fragments.
4. One low-resolution worked execution that traces a representative run through the utility at a coarse level.
5. Local exposition decisions and cross-reference decisions for each major section of the planned walkthrough.
6. Mechanical assurance checks that the walkthrough will cover the complete authoritative inventory, plus an independent-review assurance pass you would apply before considering the walkthrough ready.

Do not write the walkthrough itself.
```

Sources used:
- Repo and license: <https://github.com/junegunn/fzf>
- Release and commit SHA: <https://github.com/junegunn/fzf/releases>
- File metadata and source path: <https://github.com/junegunn/fzf/blob/master/bin/fzf-tmux>
