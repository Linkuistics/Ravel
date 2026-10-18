# book-first-half-k147

## Goal

Proof chapters 1–6 as a set after the read-1 correction run for the owner's
findings R10–R17.

## Context

- The owner's findings and decisions are in `book-read-one-k70`'s log. The
  contract changed these sections: “The intended domain”, “Emphasis”, the
  section plan, “The reader's Ravel”, “Breaks in prose” and “Figure style”.
- Check these points across the set:
  - R12 and R13 wherever their concepts occur. When no chapter uses the old
    words, consider their addition to the checked table in
    `docs/book/ste.md`.
  - R14 and R15 agree in chapters 1, 2 and 4 and in the README.
  - R16 agrees in chapters 2, 5 and 6.
  - R17: each link to the installation in chapter 3 resolves. The version
    placeholder occurs one time and agrees with `book-release-version-k145`.
  - R11: the prose check passes on all six chapters.
  - R10: every figure uses the crisp style.
- This is the third proof run on the set. The owner authorised it on
  2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:book:build`, `task docs:figures`, `task docs:examples`,
  `task docs:ste -- --strict`, the strict prose check and `task check` pass
  on subjects that have the same digests before and after.
- Every local link destination resolves. Controls with a broken anchor and a
  missing file fail.
- Every figure loads in the served book. A control image with no file fails.
- No defect in the first half goes forward without an owner.

## Notes

Art k144 keeps the nine layout records unchanged, as its mandate requires.
This also keeps short labels such as “exact result”, “No result bytes” and
“suppress”. The chapter 3 and 5 captions and alt text now use “unchanged”
and “return no result to the caller”. Read the labels with those alternatives.
The figure mechanics and dated view are in `docs/book/figures.md`.

## Decisions (running log)

- 2026-10-18: Read chapters 1–6 as one set against the contract, glossary,
  shared word decisions, both specifications and the three ADRs. R12–R17
  agree in the chapter prose, tables, captions and README. Keep the figure
  labels that k144 was required to preserve, with their prose alternatives.
- 2026-10-18: Keep R12 and R13 as phrase decisions in `docs/book/ste.md`.
  Do not add “exact” or “give” to its checked word table. Both words have
  valid uses elsewhere in these chapters. The current checker matches single
  words, so phrase rows would silently check nothing. This proof checks the
  result phrases directly, with planted controls. No checker change is needed.
- 2026-10-18: Close the opening unit's R16 handoff: chapters 5 and 6 carry
  the race explanation, first-result rule and shutdown schedule. Close the
  chapter 3 installation and version handoffs in the book brief after fresh
  validation. The second-half list retains its named owners. No correction
  run or architectural decision is needed.

## Whole-set proof, 2026-10-18

Read chapters 1–6, their tables, example explanations, captions and alt
text as one document. Read the README, glossary, contents, all nine figure
layout texts, the contract, shared word decisions, both specifications and
all three ADRs. Consulted Part 1 of the owner's STE PDF outside the
repository, with the SHA-256 that the contract names. The dictionary is not
the word authority. No chapter correction was needed.

R10 uses the crisp renderer, and all nine SVGs match it. The drawing module
accepts only the crisp style. The retained short labels agree with their
surrounding explanations and the alternatives in chapters 3 and 5. The
small text in dense figures remains with whole-book art k101, as k144
records. R11's new tables carry useful comparisons and sequences. The strict
prose check finds no run above 450 words in the six chapters.

R12 and R13 agree across the prose, tables, captions and alt text. The old
phrases are absent from the chapters, README, glossary and contents. The
same search matches both planted old phrases and the normative session
specification, where the earlier vocabulary is intentional. Valid uses of
“exact” identify literal names, precise arguments or comparison expectations.
R14's subscription reason and R15's loose program contract agree in chapters
1, 2 and 4 and the README. The book names no price or plan. R16 agrees in
chapters 2, 5 and 6: the first result requests shutdown, and another call
races it. The first value stays even if the later call succeeds. The
2-second grace, TERM, up to 5 seconds and KILL schedule agrees with the
runner. The waits stop early when the child exits, and group cleanup comes
before reap.

R17's Homebrew route and its inbound links agree across chapters 3–6.
Chapter 3 has one version line and one placeholder. The release replacement
matches that line with `exactly = 1`, and the ready guard accepts it. The
post-cut guard reads the tag, as k145 records. Its isolated release controls
are k145's evidence, not a release performed by this proof. Chapter 12's
teaching has its existing owner in the second-half handoff list.

Re-enumerated the current help, SDK and static exports, archive manifest,
diagnostic literals, settings, bounds and environment rules. Chapter 6 has
entries for all 15 start options, 19 SDK exports, six static exports and
11 archive paths, plus both verbs, RESULT and help/version requests. All
49 diagnostic codes have rows. Other snake-case literals are classified:
`owner_declined` and `no_reviewer` are fixture policy codes, `prompt_file`
is a clap conflict ID, `remove_dir_all` is an operation, `spawn_missing`
and `spawn_other` are private controls, `session_name` is a fixture parameter
and `bun_options` checks case-sensitive grants. No missing entry was found.
The initial inventory script required a whole code span and falsely missed
three entries with arguments. The corrected run matches identifiers inside
code spans and rejects a deliberately absent identifier. Its results above
come from that corrected run.

Compared the subjects item by item with k132's final manifest: no product
file under `crates/` changed. Fresh completion and run snippets confirm the
first-result rule, unchanged final output and cleanup-before-return order.
The learning, caller and policy-author routes retain their prerequisites.
The chapter titles agree with SUMMARY. Future chapters stay draft entries
or plain-text mentions. Inventories 4 and 5 are not claimed here.

Tier 2 graph verification used project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-18T07:22:48Z`. Searches were complete at their stated page limits.
Coverage checks report matching metadata and no recorded gap in the checked
source and document paths. SVG files are deliberately excluded. Their
layout text was read directly, all nine SVGs were regenerated and compared,
and Chrome checked their loads. Coverage is a best-effort signal, not proof
of completeness.

## Fresh checks and limits

- `task docs:book:build`: passed with no warning or error.
- `task docs:figures`: nine figures match their sources and layout records
  with Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37.
- `task docs:examples`: 13 passed, zero failures, skips or page errors.
  Six labelled fragments did not run.
- `task docs:ste -- --strict`: zero problems in 18 files. The STE controls
  and mutation controls passed through `task docs:ste:test`.
- `task docs:breaks -- --strict`: zero long runs in six chapters. Its
  controls and mutation controls passed through `task docs:breaks:test`.
- `task check`: format, script lint, worker and probe builds, runtime
  integrity, types, Clippy and the Rust suite passed. The suite ran 339
  tests: all passed, with 46 skipped.
- The local link audit checked 429 destinations across chapters 1–6,
  README, CONTEXT and SUMMARY. All resolve. The installation anchor passed.
  A broken anchor, missing page and missing image were rejected by the same
  resolver. Future draft entries have no destination to check.
- A fresh `task docs:book PORT=8784` served the book. Chrome at 1227 by
  919 pixels loaded all nine figures at the 750-pixel column. All were
  complete with nonzero natural dimensions and nonempty alt text. Each
  figure has a visible image and mdBook's hidden zoom copy. Chapter 4 has
  no separate figure. On the same server, the real control image loaded and
  the absent image had zero natural width and height.
- Visual samples: chapter 3's installation and version section in Chrome,
  the final-outcome SVG at full size, and chapter 3's opening, installation
  instructions and first example in VS Code's light Markdown preview, in
  Restricted Mode. Raw Markdown supplied the full text read. No complete
  new visual inspection of every figure, other theme or narrow viewport is
  claimed. GitHub rendering and GitHub Pages remain unchecked for this
  unpublished book. Real-agent behaviour, terminal interaction, native
  x86-64 qualification and new-reader comprehension remain separate unknowns.
- The before/after SHA-256 manifests agree for all 3,524 subjects: root
  files, `docs/`, `crates/` without `node_modules`, `scripts/` and the
  pinned tools under `target/tools/`. Build outputs and dependency
  installations are outside this scope. The compact sorted manifest digest
  is `6722bdb49daf9bc6e2d59ed0f7bd9b49915a5849cb803d6048910ea4e1db5743`.
  Logs, manifests and per-item inventories and links are in
  `target/book-proof-k147`.

No unresolved first-half handoff remains. The book node stays open for the
second half, whole-book art and proof, and read 2. Their existing owners
remain in the book brief. No live review names this producer, so no Creator
line is due. No in-session reviewer was used, and no ADR decision changed.
