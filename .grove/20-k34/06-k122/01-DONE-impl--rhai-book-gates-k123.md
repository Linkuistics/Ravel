# rhai-book-gates-k123


## Goal

Record reproducible evidence from the book's executable gates and their controls
against the current promoted-release text, for the final whole-book proof.



## Context

The enclosing proof brief and editorial node retain the whole-document scope.
Historical proof evidence is in `docs/evidence/ravel-book-k311/README.md`.
Use the current Taskfile and `machine-environment` guidance. `task setup` has
completed in this session. Do not change frozen historical evidence.

## Done when

- Book build, examples, excerpts, figures, STE and prose-break checks pass.
- The existing checker controls and mutants run, showing deliberately wrong
  subjects refused before clean results are credited.
- An evidence record identifies exact commands, input digests before and after,
  observed results and the limits of these checks.
- A final whole-document `proof` leaf is cut under this node, carrying all
  remaining proof and Read 2 obligations.

## Notes

The initial read covered chapters 1–3 and chapter 4 through line 420, with no
settled correction. It is not a whole-book proof. The final proof starts a fresh
whole-document read rather than inheriting a partial-read completion claim.

## Decisions (running log)

2026-11-14: Use the existing executable gates and controls without modifying
their instruments. Record the input set before running them. Retain `out.err`,
which predates this session, outside the task commit. No prose correction has
been settled in the initial read.

2026-11-14: Record current gate evidence in
`docs/evidence/rhai-book-gates-k123/README.md`. The book build, 27 examples,
22 excerpts, 21 figures, STE check and prose-break check passed. The controls
passed, including all 30 example, 24 excerpt, 12 STE and 17 prose-break mutants.
Both retained command statuses are 0. A repeated control run supplied the exit
status that the first launch did not retain.

2026-11-14: All 354 declared subjects have unchanged membership and SHA-256
digests after the checks. The set excludes system libraries, environment
variables and the compiled debug binary. Record these limits rather than claim
complete environment capture. No book or instrument edit was needed. Cut a
whole-document proof next; do not close the enclosing proof node.
