# deliver-ravel-k183

## Goal

Read and copy-edit all of chapter 12, `docs/book/src/12-deliver-ravel.md`, after
both draft parts, under `docs/book/contract.md` and `docs/book/ste.md`.
Own sentences, terms and consistency. Preserve the settled teaching order and
technical facts, and route earlier-stage defects through the family procedure.

## Context

- Read the document node and both draft logs, k162 and k182. They carry the
  source checks, delivery-task inventory and guide destinations for fold-in.
- Use plain language for identity, archive validation, library/CPU floors,
  release preparation and partial recovery. Keep measured routes separate
  from minimum-OS or native x86-64 support claims.
- The shared term is “the book on GitHub Pages”. The release's book version
  appears once in chapter 3. Chapter 12 teaches the replacement and guards.
- Part A passed strict STE and prose-break checks, but those mechanical checks
  do not discharge your whole-document prose charter. Part B adds the release
  and recovery sections before the final dated checks section.
- The node's Handed forward list belongs to art: reuse the chapter 6 installed
  view and draw the release activity view. Do not absorb art's work here.
- The two draft parts are a planned division of one document, not correction
  re-runs. Chapter 10 remains unwritten. Guides stay until their fold-in leaf.

## Done when

- The whole chapter meets its prose contract, with consistent terms and clear
  instructions. The book build, strict STE, strict prose breaks and complete
  examples pass, along with excerpts if part B adds any.
- Closed handoffs are cleared. Other defects have their correct stage owners.
- As the last work act, add the bare-stem `deliver-ravel` art leaf under this
  node, unless a live later sibling already holds it. Give it the concrete
  figure needs from the complete chapter and the node brief.

## Decisions (running log)

- 2026-10-18: The prose contract is section 2 of `docs/book/contract.md`, with
  `docs/book/ste.md`. The precedent is the proved chapter 11, which links
  each glossary term to chapter 2 at first use. Strict STE and strict breaks
  passed on the draft text before any edit. Those checks do not cover term
  links, figures of speech, condition order or term collisions.
- Term links: link the Ravel CLI, policy worker, owner policy, session,
  session-result context, opaque result, harness and runner to the place
  the book teaches each one, at first use, as chapter 11 does. Keep the
  chapter 7 link for the installed pair.
- One word for one concept: “version cut”, not “bump”, in running text. The
  settled recovery heading keeps “bump”, so the recovery opening names the
  equivalence one time. The CLI/worker “pair” keeps that word alone. The two
  glibc probes become “the two probes”. “Manifest check” names the validator
  everywhere. “Smoke” stays a noun or task name, not a verb.
- Architecture spellings: prose uses arm64 and x86-64. The target names use
  `aarch64` and `x86_64`, and the build section says so one time. The support
  statement keeps the contract's words, which use the target spelling.
- Term risk: `worker_identity_mismatch` is a worker failure, not a refusal.
  Remove “identity refusal”. “Measured context” becomes measured policy
  context and `--param` values, as `installed-smoke.sh` shows.
- jj is not in the assumed knowledge, and chapter 11 does not introduce it.
  Explain the jj words that the reader needs (colocated workspace, change ID,
  description, bookmark) in one short passage at their first use.
- Mechanisms in place of figures of speech: the kernel 6.7 floor comes from
  the private `binfmt_misc` instance per user namespace; the ELF check matches
  each archive executable to the emulator registration; the release lock goes
  away through Task's `defer`; “lies ahead” becomes the checked fact that tag
  `v1.0.0` has no `docs/book`.
- Contract question set (node brief, R17): the chapter answered what a
  checkout shows, what the cut writes and what each guard refuses. It did not
  give the recovery for each guard. Add it: correct chapter 3 to one version
  line after a `ready` refusal, and use the local-cut row after `released`.
- Production words: “part A”, “part B” and “this draft” are workstream terms
  that the reader does not know. The dated checks name the two check sets by
  order and date instead. The historical sentence “no longer 0.0.0” goes,
  under the contract's history exclusion. The script's 0.0.0 cases stay.
- Synonym pairs that the writing hook found in the draft text are closed:
  “check” replaces validate, verify and confirm. “Correct” replaces repair.
  “Run” replaces “launch” for the harness. No departure marker was needed.
- No in-session reviewer was spent. Each changed fact was read directly from
  its source: `Taskfile.yml` (`release` lock, `defer`, workspace check),
  `scripts/release-book-version.sh`, `release.toml`, `scripts/release-smoke-qemu.sh`
  (lines 18–30 and the ELF match), `installed-smoke.sh`, `scripts/check.sh`,
  `crates/keyed-launch/Cargo.toml` and `git ls-tree v1.0.0 docs/`.

## Final validation

The whole chapter was read after the edits. The settled headings and their
order are unchanged. No section moved, and no technical fact changed except
the added mechanisms, each checked above. No structural defect was found, so
no draft re-run is due. The node's Handed forward entries belong to art and
proof. This stage closed none of them, so none was cleared.

Fresh checks on the final text passed: `task docs:book:build`, strict STE on
chapter 12 and `SUMMARY.md`, strict prose breaks, `task docs:examples` (14
passed, 0 failed, 0 skipped, 26 fragments not run) and `task docs:excerpts`
(8 passed). The chapter has no excerpt of its own. A scratch copy with one
semicolon made the strict STE check report one problem. All 17 local link
targets of the chapter resolve to ids in the built HTML. A control link to a
missing anchor failed. No browser, GitHub or Pages view ran. Art and proof
keep those viewing obligations.
