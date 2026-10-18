# jobs-signals-and-the-terminal-k176

## Goal

Read the whole of chapter 9,
`docs/book/src/09-jobs-signals-and-the-terminal.md`, after copy-edit k175,
and own its figures and their adjacent prose under `docs/book/contract.md`,
`docs/book/figures.md` and `docs/book/ste.md`. Put the escalation timing
figure on the page, draw the terminal lending and recovery sequence, and
reuse chapter 5's two cancellation sequences where this chapter needs them.

## Context

- The node brief's `## Handed forward` holds both art handoffs in full. Read
  them first, then k158's “Figure handoff to art”. In short:
  - **`escalation-timing`** exists from the scaffold
    (`book-figure-timing-k156`), but no chapter page shows it. Check it
    against the final text of “Follow the escalation after a result” and put
    it after the escalation table. Bounds to check: grace 2 seconds from
    observation, kill grace 5 seconds from TERM, 500-millisecond poll until
    KILL, 10-millisecond poll after KILL, a two-second retry bound after the
    first KILL, two final group KILL attempts 20 milliseconds apart before the
    reap, and a group query of up to one second after terminal recovery. The
    figure must not suggest a seven-second session deadline, a new grace for
    descendants after the child exits, or a wall-clock guarantee.
  - **New figure: terminal lending and recovery sequence.** Put it in “Lend
    and recover the terminal”, near the ordinary foreground-launch table. The
    node brief lists the participants, the initial condition, the child
    sequence, the successful-spawn path, the holder branches and the
    failed-spawn/unknown-status branch with its two conditions. The runner is
    inside the Ravel CLI, so do not draw a separate runner process. The
    private attempted-group pipe is data, not a participant.
  - **Chapter 5's `selection-cancellation` and `supervision-cancellation`.**
    “Handle cancellation” links to them. Decide whether the page embeds them
    or keeps the link. Confined cancellation sends KILL at once, and neither
    figure shows that case.
- **Signal names.** Copy-edit k175 set the rule in `docs/book/ste.md`
  (“Signal names”): INT, TERM, HUP and KILL without the prefix SIG, other
  signals with their full names. Chapter 9 now follows it. The escalation
  figure already uses the short names. The two chapter 5 figures say
  “SIGINT (the example below)”, and their alt text in chapter 5 says SIGINT.
  If chapter 9 embeds them, decide their text under that rule. The mixed forms
  in chapters 5 and 6 belong to the whole-book proof (book brief table).
- Copy-edit preserved these distinctions, and each caption and alt text must
  keep them too: stop attempts versus confirmed absence, group signals before
  the reap versus the signal-0 query after it, only `ESRCH` confirms absence
  (success and `EPERM` both mean that the group is present), `Started` is
  spawn-reported success and not proof of exec, and no unconditional terminal
  takeover. Use “return no result to the caller” (R13).
- The tables in the chapter carry the facts without the figures. A figure
  adds the order and the branches.

## Done when

- The escalation figure is on the page, checked against the text, with a
  caption and alt text in STE. The terminal sequence exists in the existing
  sequence module, with a caption and alt text in STE. The cancellation reuse
  is decided.
- `task docs:figures`, the book build, strict STE and strict prose breaks on
  chapter 9, the examples and the excerpts pass. Each figure is inspected at
  the 750-pixel column, as `docs/book/figures.md` states.
- The node brief's art handoffs are cleared. Last act: add the `proof` leaf
  with the bare stem `jobs-signals-and-the-terminal`, unless a live later
  sibling already holds that stage.

## Notes

Dense small figure text across the book stays with whole-book art k101. This
leaf does not claim a reader result or a new runtime measurement.

## Decisions (running log)

- Use the agreed Typst/CeTZ renderer and existing timing and sequence modules.
  The chapter and node handoff already settle the design and authorise this art
  work. No renderer, runtime or document-structure decision is reopened.
- Separate the timing figure into escalation, early exit and common cleanup
  panels. The scaffold equates KILL with exit and promises a maximum poll delay.
  Replace those claims with attempts, observed exit and nominal poll intervals.
  Show both final group KILL attempts before reap and the later signal-0 query.
- Keep chapter 5's cancellation sequences linked. This is an editorial judgement:
  chapter 9's table supplies the phase comparison and immediate confined KILL,
  while the linked figures give the examples' sequence. Repeating them here
  adds no relation and could suggest that they cover confined cancellation.
  No medium limitation or contract prohibition caused this choice.
- Draw terminal acquisition and the two recovery paths in one sequence, with
  the runner inside the CLI. Explicit conditions and alternative section labels
  keep successful spawn, normal recovery and uncertain status distinct.
- The one fresh-context reviewer found one actionable qualification: the first
  terminal key made exec a predecessor of every failed spawn. Add an explicit
  early-failure entry to B before the setup steps, and correct the key. This
  changes no recovery rule or diagram machinery and needs no second review.
  The reviewer found no escalation or caption defect. This is a bounded read
  against the chapter, not a new runtime observation.
- Strict STE found “Scheduling” in the timing key. Replace it with “The
  scheduler”; do not extend the shared word list for a figure sentence.
- The complete chapter read found that its existing tables carry the other
  comparisons, mappings and operation order. Retain them with their adjacent
  explanations. No runtime or earlier-stage prose correction is required.
- Preserve the terminal sequence's conditions and key despite its SVG size.
  It exceeds the earlier per-figure benchmark. The book brief hands the exact
  size to the existing whole-book art k101, beside its set-size and dense-text
  work. No claim of compliance with those size benchmarks follows.

## Validation

- `task docs:figures`: thirteen figures match their sources and layout records.
- `task docs:book:build`: passed without warnings.
- Strict `docs:ste` on chapter 9 and both changed figure layout records:
  zero problems. Strict `docs:breaks` on chapter 9: no run over 450 words.
- `task docs:examples`: fourteen passed, none failed or skipped, six labelled
  fragments not run, no page errors. `task docs:excerpts`: eight passed,
  none failed, no page errors.
- SHA-256 lists before and after the checks match for 3,484 tracked files
  outside `.grove/`. This covers the chapters, figures, modules, scripts,
  source, fixtures and manifests. Scratch lists are under `target/art-k176/`.
  The subsequent changes are the viewing note and Grove handoff records.
- Chrome displayed both complete figures and their adjacent tables at the
  750-pixel column. Full-size views were also inspected. `docs/book/figures.md`
  gives dimensions, sizes and viewing limits. No reader or runtime result follows.
- Graph project `Users-antony-Development-Ravel.simplify-and-componentize`
  was ready at bootstrap. Tier 2 coverage reported no issue for the document
  and Typst paths, with matching metadata. The Typst symbol search returned
  no rows, so both existing modules and figure sources were read directly.
  SVGs are excluded by design; direct renders and browser views supplied their
  checks. No graph completeness or runtime claim follows.

The art handoffs in node k157 are cleared. The proof successor keeps the node
open. No ADR decision changed and no parent cascade is due.
