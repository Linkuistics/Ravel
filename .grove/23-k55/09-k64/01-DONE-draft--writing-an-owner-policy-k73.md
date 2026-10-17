# writing-an-owner-policy-k73

## Goal

Draft **part A** of chapter 4, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/04-writing-an-owner-policy.md`,
linked from `docs/book/src/SUMMARY.md`: the anatomy of a policy, its
typechecking, and validating every route and refusal with fake agents. The
chapter's headings for part B are not written here.

## Context

- Chapters 1–3 are proved before this runs; the chapter reuses chapter 3's
  example as its starting point and extends it.
- Settle the chapter's heading structure for the whole of chapter 4 now, so
  part B fills sections rather than reshaping the file, and write down in
  this leaf's log which headings part B owns.

## Done when

- Part A's sections exist, build and pass `task docs:examples`; each has a
  worked complete example or an honest fragment.
- Last act: `leaf-add` a `draft` leaf (bare stem `writing-an-owner-policy`)
  whose body states part B's scope from the node brief, the headings reserved
  for it and anything part A learnt that changes it; then `leaf-add` the
  `copy-edit` leaf if no live later sibling already holds it.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`: “The reader is a
  software engineer with no background in Ravel's context”; its ordered section
  plan gives chapter 4 the write/typecheck/validate outcome; “Validating with fake
  agents” is an emphasis and installing/configuring agents is excluded.
- Part A's ordered headings are “Define a policy and read the request”, “Select
  a command or refuse”, “Typecheck against the public declarations”, “Use the
  starter route table”, and “Offer a checkout choice”. Each moves from the
  chapter 3 fake toward an owner-editable policy with checked branches.
- Part B owns the following subsequent headings, reserved here rather than
  emitted empty in the chapter: “Supply policy context”, “Set bounds and grant
  environment names”, “Resolve policy imports”, “Route to Claude Code and
  Codex”, and “Copy an existing policy”. It fills these in that order without
  reshaping part A. Configuration lookup appears where `--config` first needs it;
  settings, context and resolver details remain part B's.
- Use the shipped static table and sample through embedded imports,
  with fake `my-agent-wrapper` validation. Explain that the sample's “careful”
  choice sets effort to `high` even for routes whose default is `xhigh`.
  Generated declaration checks cover both `ravel/sdk` and
  `ravel/examples/static`; normal execution needs no development checker.

## Verification

- Structure: the first five headings follow the teaching progression recorded
  above. Part B's five headings and the guide's sentence-group destinations are
  carried in the document-node brief. No part B heading or empty placeholder
  was emitted in the chapter. SUMMARY now links the chapter.
- Technical truth: enumerated the policy, request, limits, selection and refusal
  fields from `crates/ravel/worker/sdk/index.ts`; read `readChoice`, the full
  static table and sample, the worker tsconfig and declaration-generation
  script. Checked the prose against session-execution's authority/request
  sections and the three ADRs. The graph was used at Tier 2; coverage for the
  source and document paths reported no recorded issue and matching metadata,
  generation `2026-10-16T04:18:13Z`. That is best-effort coverage, not proof of
  completeness. The readChoice trace contained a suspicious unrelated Rust
  `Channel.path` edge; full exact source reads, not that edge, ground the claim.
- `task docs:examples`: 4 passed, 0 failed, 0 skipped, 0 page errors. Three are
  this chapter's examples (the first policy, static table and choice policy);
  the fourth is chapter 3. Four fragments across the book are not run by that
  checker. Both typecheck fragments were separately executed as below.
- Both printed typecheck recipes were extracted from Markdown and run for all
  three chapter policies: six successful checks. The installed mappings used
  `target/book-install/libexec/ravel`, whose SDK and static declarations were
  byte-equal to the generated checkout declarations. Both recipes rejected an
  otherwise identical policy with numeric `program`, reporting number not
  assignable to string. The paths alone were substituted in the printed recipes.
- Final `task docs:book:build` passed without warnings. Read-only
  `git diff --check` passed (`jj diff --check` is not a supported jj option).
  Before/after SHA-256 records were equal for the chapter, SUMMARY, SDK, static
  table, sample, worker tsconfig, examples runner and extractor. The final
  examples run followed completion of all typecheck/rebuild work.
- One bounded fresh-context reviewer inspected validation/typechecking
  instructions against the reader and examples contracts, not copy-edit or
  art. Claim under test: the instructions let an owner exercise the stated
  checks before real-agent execution. The reviewer received the artifact and
  contract, not this conclusion; it returned no concrete findings, so there
  were no findings to classify or fixes to re-review. The one-review allowance
  is spent. Executable checks establish routing/output and declaration
  compatibility, not reader comprehension, terminal interaction or real agents.
- The installed-prefix recipe transparently uses the checkout's pinned compiler
  as a development tool; no claim that the installation bundles one. The dated
  personal configuration handoff is not reproduced. No production code, ADR,
  toolchain pin, or settings changed. No parent closes: part B and the remaining
  editorial stages are still required.
