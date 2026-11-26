---
name: grove-review-requirements
description: The `review-requirements` session kind — a fresh-context, inspection-only adversarial read of one requirements artifact, producing findings and no fixes. Use when a grove mandate names this skill, or when running a `review-requirements` session in a grove working tree.
harnesses: [any]
---

# review-requirements

**Load the `grove` skill now**, and read `references/review.md` in it.
`grove` is the bare skill name. If your harness requires a plugin namespace, use
the namespace under which `grove` is actually installed. Do not assume one. If
you cannot find or read the `grove` skill, follow the launch prompt's rule for a
missing kind skill.

The five `review-*` kinds are one family with one procedure, and that file is
it: the goal, the deliverable, what a review may not do, and what each of the
five reads looks for. This skill states none of it a second time. Everything
else — the constraints, the bootstrap, and the execute, decompose, retire and
commit procedures — is the spine's.

Your artifact is the requirements artifact produced by the `requirements` leaf this review
sits beside.
