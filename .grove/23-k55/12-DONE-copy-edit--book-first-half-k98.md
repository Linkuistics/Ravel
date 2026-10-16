# book-first-half-k98


## Goal

Rewrite chapters 1–6 as one set under the amended prose contract in
`docs/book/contract.md`. The owner wants plain language for a 16-year-old
software engineer and specifically found “front” unfamiliar and “admits”
unnecessarily technical. This is a wording pass, with technical meaning and
the existing programming prerequisites preserved.



## Context

- Read `book-first-half-k67` for the set proof, source inventories, edits and
  checks. This pass applies a new owner requirement, rather than reopening
  the earlier units' correction runs.
- The settled term is **Ravel CLI**, first defined as the `ravel` command-line
  program. The owner suggested “cli” after considering “Ravel executable”.
  `CONTEXT.md` and the contract carry the current term. Do not preserve the
  superseded choice merely because it appears in an earlier log.
- Scope: all six `docs/book/src/0*.md` chapters as one document, their local
  links and the glossary where wording needs to agree. Review sentences,
  headings, tables and examples' surrounding explanation. Use “checks”,
  “accepts”, “allows”, “starts” or another exact ordinary verb for the actual
  action. Do not treat this as two global substitutions: simplify other
  specialist phrasing, including unexplained “admission” and “settlement”.
- Assume the reader knows software engineering, the contract's programming
  prerequisites and the diagram types. Keep necessary Ravel/OS concepts
  explained where needed; simplify without talking down or losing conditions.
- Literal CLI output, identifiers, exact examples and normative source
  terminology remain exact. If a heading changes, reconcile its inbound
  links. Report figure-owned wording to art, which is already queued next.

## Done when

- The whole set meets the revised prose contract, with Ravel CLI used
  consistently and first explained/linked where needed. Every specialised
  word earns its place and explanation. Record representative wording choices
  and any deliberate literal/source exceptions.
- Technical distinctions proved in k67 survive: worker-before-launch,
  publication versus successful delivery, refusal/failure/cancellation,
  policy context versus session-result context, limits and evidence scope.
- Build and complete-example checks pass on final unchanged inputs; changed
  anchors resolve. Any figure label/caption/alt-text changes art owns are
  written into its live body or the book brief, then cleared by art.

## Notes

Art `book-first-half-k99` and proof `book-first-half-k100` already follow this
leaf before guide migration. Cut no duplicate successor. If wording reveals
a missing explanation or a substantive technical defect requiring draft,
use the editorial correction procedure with the precise case; do not conceal
it with an easier synonym.

## Decisions (running log)

- Charter split with art `book-first-half-k99`: this pass rewrites running
  prose, headings, table cells, example labels and the explanations that
  follow a figure. Terminology is this charter's everywhere in prose, so the
  figure key paragraphs get the term and verb fixes too, but not their
  restructuring: removing the generic notation tutorials they hold is art's.
  Image alt text, italic figure captions and SVG/PlantUML labels are art's;
  their stale wording is listed in art's body rather than edited here.
- Vocabulary: "native front"/"front" become **Ravel CLI**, defined in
  chapter 2 as the `ravel` command-line program and linked at first use on
  each page. "Admit"/"admission" become the actual action ("checks",
  "accepts", "allows", "starts"); "settle"/"settlement" become "decide the
  final outcome" or "finish", naming the step where it matters. Literal
  diagnostic codes (`selection_unsettled`, `context_loader_unsettled`),
  file names (`settled-exit-status.svg`) and source identifiers stay exact.
- Representative wording choices: "Admit the request and evaluate the
  policy" became "Check the request and evaluate the policy" (anchor changed;
  its chapter 6 inbound links updated, no other document links it);
  "admits the policy file" became "checks"; "admit the managed launch" became
  "allow"; "admits only a worker with its own build identity" became
  "accepts"; "Refusal by the policy or of its admission" became "Refusal by
  the policy, or Ravel's refusal of the policy", matching CONTEXT.md's
  "file, behaviour or data". "Settlement" became the step where "Ravel
  decides the final outcome"; "settles its result" in chapter 1 became
  "reports". "Authenticate" became "is not a security check/defence against";
  "measured ledger" became "the `measured` list"; "unsettled promise" became
  "a promise that can never resolve or reject"; "Unknown fields refuse"
  became "Ravel refuses unknown fields"; "establish success" became "prove";
  "scrub set" became "removed names"; "session-result authority" became
  "receives no session-result context" or "permission to publish a result".
- The superseded "Ravel executable" survived once in chapter 4 and became
  "the Ravel CLI". "Ravel CLI" is linked to chapter 2's definition at its
  first use on every other page; chapter 1 also says what it is in place.
- Deliberate literal exceptions: diagnostic codes `selection_unsettled`,
  `context_loader_unsettled`; the stage name `authority`; the figure file
  `settled-exit-status.svg` and the "process-authority" figure name;
  "launcher CLI" and "persistent runner" (glossary terms); all code blocks,
  CLI output and example transcripts are byte-unchanged.
- Chapter 5's terminal-recovery limit row was restated from
  `docs/specs/result-job-completion.md` §"Terminal recovery after failed
  exec": the two premises are no foreground change between check and
  handback, and no unrelated reuse of the recorded group ID. The old
  "foreground-authority and identity conditions" named them only abstractly.
- Figure-owned wording (alt text, Figure 3.1's caption, figure source labels,
  generic notation tutorials in key paragraphs) is handed to art k99 in its
  body under "Handed from copy-edit".
- The one in-session reviewer (adversarial meaning-preservation read of the
  whole prose diff against both specifications) is spent. Classification:
  valid and applied — chapter 1's "without calling `end-session`" (a
  descendant can publish and a call can fail; now "no result was
  published"); the reserved-grant sentence that read as "no names removed";
  "native-entry" restated too narrowly (now signal settings inherited,
  cancellation, cleanup and terminal recovery); "each of Ravel's steps
  completed" overclaimed beside retained failed-stop reports; worker
  environment "names" (it receives values); "Ravel failed"; "cannot make
  Ravel run a program"; `sources` "says who"; dropped "real" executable;
  narrowed same-user "Ravel's files"; overstated persistent-host and
  chapter 10 limits; SIGKILL "before" the decision; inclusive cancellation
  decision point; "or thread" in the recovery premise; a residual
  "establish"; an unsupported "can find the path". Visible trade-off — the
  refusal wording follows CONTEXT.md's "file, behaviour or data"; the
  status 3 `authority`-stage refusals `home_unset` and `cwd_unavailable` stay
  exact in chapter 6's code table. No noise. No literal, code or anchor
  defect was found; a second review was not needed, since the fixes are
  local restatements checked against the cited spec text.

## Checks and their limits

Taken after the last edit, with a SHA-256 digest over every file under
`docs/book`, `docs/specs`, `docs/adr`, `crates` (excluding `node_modules`),
`scripts`, `CONTEXT.md` and `Taskfile.yml` identical before and after:

- `task docs:book:build` built without warnings. Every local link and image
  destination in chapters 1–6 resolves against the built HTML, including the
  renamed chapter 3 anchor; restoring the old anchor in chapter 6 was seen to
  fail the same check before the final run.
- `task docs:examples`: 13 passed, 0 failed, 0 skipped; 6 labelled fragments
  not run; 0 page errors. No code block or transcript changed.
- `task docs:figures`: all nine figures match their sources. No figure or
  style file changed here; their wording is art's (handed forward above).

Not checked here: rendered viewing in any environment (proof k100's), the
figure wording, and any reader's comprehension. Product source is untouched,
so `task check` was not rerun. The prose still contains "front", "admit" and
"settle" only inside alt text and the Figure 3.1 caption, which art owns.
