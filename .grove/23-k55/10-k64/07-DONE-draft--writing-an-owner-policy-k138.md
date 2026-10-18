# writing-an-owner-policy-k138

## Goal

Draft chapter 4 again for the owner's read-1 findings R17 (the link to the
installation in chapter 3) and R11 (runs of prose that are longer than the
limit).

## Context

- The owner's words are in `book-read-one-k70`'s log, “Owner findings
  R10–R17”.
- **R17.** Line 11 at `1ef02398` links to
  `#obtain-the-build-used-by-this-chapter` in chapter 3. Draft
  `a-complete-session-k136` replaces that section. Use the new anchor that the
  brief of node `a-complete-session-k63` gives. Where the text says “the
  checkout build”, say that the reader uses the Ravel that they installed.
- **R11.** With code blocks as breaks, chapter 4 has runs of 549 words (lines
  1477–1538, “Route to Claude Code and Codex”), 487 words (lines 1714–1764,
  “Copy a policy from another configuration”) and 466 words (lines 892–942,
  in “Supply policy context”). Measure with the check from
  `book-prose-breaks-check-k133`. Add tables where items compare on the same
  attributes. Write each figure need under `## Handed forward` in this node's
  brief, with its reader question, for set art `book-first-half-k144`.
- Chapter 1's new text gives the reason for a harness (R14) and the loose
  contract (R15). The routes to Claude Code and Codex must agree with it.
  Name no price or plan.
- This is the first draft re-run on this document. Set art and set proof
  serve all six chapters, so this node gets no art or proof leaf.

## Done when

- Every link to the installation in chapter 3 works. A search with a control
  shows it.
- The prose check finds no run that is longer than the limit in chapter 4.
- This node's brief lists each figure need.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. Its reader is a
  “16-year-old software engineer with no background in Ravel”. Its ordered
  section plan makes chapter 4 teach the reader to “write, typecheck and
  validate a policy with fake agents”. Its Emphasis and Exclusions sections
  require fake validation and authority boundaries, and exclude agent setup
  and personal configuration. The document node gives the section order
  within chapter 4. This correction keeps that order.
- R17 uses chapter 3's `#install-ravel-and-check-its-version` anchor, now
  recorded in the book brief after k137 closed the chapter 3 node. The
  typecheck recipe also needs its obsolete chapter 3 prerequisites pointer
  removed. It states the checkout prerequisites locally, verified against
  README Develop, and names chapter 11 for the later development workflow.
- R11 uses three page tables: attribution versus measured reads, runtime
  autoloading controls, and the files to reconcile before a policy copy.
  These answer comparison questions without a new diagram. The initial
  strict prose check failed on the expected three runs: 463, 542 and 473
  words. This is the failing control for the correction.
- The route introduction will state subscription access and the loose
  program contract. The official Claude Code authentication and Codex
  authentication pages were opened on 2026-10-18 and confirm subscription
  and API access as separate options. No price or plan is named. The
  example arguments and their dated 2026-10-16 help evidence stay intact.
- Copy-edit k139 is already the next live stage. No new stage leaf is
  needed. Set art k144 and set proof k147 remain the shared later stages.

## Verification

- Read the whole chapter and retained its section order and reader outcome.
  The three comparison tables answer questions within the existing sections.
  The document brief carries the table questions and the decision that no
  new diagram is needed, for set art k144.
- Checked the changed technical claims against
  `docs/specs/session-execution.md`, the SDK's source and assessment shapes
  in `crates/ravel/worker/sdk/index.ts`, `Session` in
  `crates/ravel/worker/src/host.ts`, and `assemble` in
  `crates/ravel/worker/src/main.ts`. The last two show measured host reads
  and attachment of the ledger. The autoloading table matches
  `SHIPPED_SWITCHES` in `crates/ravel/scripts/dispatch.sh` and the policy
  evaluation ADR. The migration table retains the existing procedure's
  file responsibilities. The R17 target is the actual chapter 3 heading.
- Tier 2 graph project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T06:27:27Z`, was ready. Coverage checks on the evidence paths
  found no recorded issue and matching metadata. Narrow searches found
  `Session` and `assemble`; the one-hop trace of `assemble` found its
  caller and host/encoding functions without truncation. Broader context
  name searches were insufficient, so direct source reads supplied the
  exact fields and build switches. No graph-completeness claim follows.
- Subscription access was checked on 18 October 2026 against the
  [Claude Code authentication guide](https://code.claude.com/docs/en/authentication)
  and the [Codex authentication guide](https://learn.chatgpt.com/docs/auth).
  The chapter links both beside the statement. No agent flags changed.
- `task docs:breaks -- --strict docs/book/src/04-writing-an-owner-policy.md`
  passes: no run exceeds 450 words. The initial run failed on the three
  passages named above. `task docs:ste -- --strict` on the same chapter
  passes with no problems after two issues in the new text were corrected.
- `task docs:book:build` passes without a warning. `task docs:examples`
  passes: 13 complete examples, no failures, no skips and no page errors.
  Six labelled fragments are not executed by that task.
- All 20 local links extracted from chapter 4 resolve to existing files
  and, where specified, IDs in the built HTML. Deliberately missing file
  and anchor targets both fail that check. The exact old installation
  anchor is absent from chapter 4. The same `rg -nF` search finds it in the
  parent chapter and in chapter 5, whose correction belongs to k140. The
  new anchor is present in chapter 4 and resolves in chapter 3's HTML.
- An exact comparison with `@-` confirms that all 109 fenced blocks and
  all section headings are unchanged. The example check therefore runs the
  same policies, fake programs and expected output under the new prose.
- Before and after the final checks, SHA-256 digests match for the 193
  enumerated source/tool inputs: `docs/book/`, `scripts/`, `crates/`, the
  Cargo manifest and lockfile, `Taskfile.yml`, `CONTEXT.md`, `README.md`,
  and the pinned `target/tools/bun` and `target/tools/mdbook` executables.
  No chapter or check input changed during those checks.

No in-session reviewer was used. No architectural decision changed, so the
ADR set needs no edit. Copy-edit k139 remains live, so this retirement closes
no parent node. These checks establish source agreement, rendered links and
fake-program behaviour. They do not establish real-agent behaviour, terminal
interaction, a visual read or new-reader comprehension.
