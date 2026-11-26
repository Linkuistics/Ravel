---
name: grove-impl
description: The `impl` session kind — produce code, docs, or tests that ship, verifying framework decisions against the source rather than memory. Use when a grove mandate names this skill, or when running an `impl` session in a grove working tree.
harnesses: [any]
---

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     addyosmani/agent-skills@13e43f23 (skills/source-driven-development)
     — MIT licensed; see LICENSES/addyosmani-agent-skills.LICENSE. -->

# impl

**Load the `grove` skill now.** It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. `grove` is the bare skill name. If your
harness requires a plugin namespace, use the namespace under which `grove` is
actually installed. Do not assume one. If you cannot find or read the `grove`
skill, follow the launch prompt's rule for a missing kind skill. What follows is
`impl`'s, and is stated nowhere else.

**impl** (AFK) — produces code, docs, or tests. The deliverable is an artifact
that ships.

## Verify a framework decision against the source, not your memory

Where correctness depends on the **version** of a framework or library, training
data is not evidence: an API you remember may have been deprecated two releases
ago. Version-invariant logic, renames and plumbing are exempt; for the rest —

- **Read the manifest first.** `Cargo.toml`, `package.json`, `pyproject.toml`,
  `go.mod` — whatever pins the version. The version decides which pattern is
  correct, so guessing it defeats the exercise.
- **Fetch the official source.** The project's own docs or changelog for that
  version — Context7 (`resolve-library-id` → `query-docs`) or your harness's
  web-fetch tool. Official docs outrank Stack Overflow, blog posts and training
  data. This is reading, not running: constraint 2 is satisfied.
- **Cite at the decision site.** A one-line comment carrying the source URL,
  beside the non-obvious call, so the next reader can check it without you. A
  citation that lives only in the chat evaporates; one in the code is
  walk-away-able.
- **Flag what you could not verify.** "No official source found; based on
  training data, verify before relying on it" beats false confidence, and the
  absence is itself a finding.

**When the decision is also hard to reverse, cite the source *and* have a fresh
context try to break it.** That is one of the occasions the leaf-wide review
allowance in `references/execute.md` exists for; the citation says what the API
is, the doubt pass says whether the call built on it survives an adversarial
read.
