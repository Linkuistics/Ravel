---
name: grove-integrate-review-requirements
description: The `integrate-review-requirements` session kind — triage one `review-requirements` leaf's findings and apply the real ones. Use when a grove mandate names this skill, or when running an `integrate-review-requirements` session in a grove working tree.
harnesses: [any]
---

# integrate-review-requirements

**Load the `grove` skill now**, and read `references/integrate-review.md`
in it. `grove` is the bare skill name. If your harness requires a plugin
namespace, use the namespace under which `grove` is actually installed. Do not
assume one. If you cannot find or read the `grove` skill, follow the launch
prompt's rule for a missing kind skill.

The five `integrate-review-*` kinds are one family with one procedure, and that
file is it: the goal, the shared triage, where the findings are read from, what
each of the five may change, and what is too large for this session. This skill
states none of it a second time. Everything else — the constraints, the
bootstrap, and the execute, decompose, retire and commit procedures — is the
spine's.

The review you are integrating is the `review-requirements` leaf named by handle
in your task body.
