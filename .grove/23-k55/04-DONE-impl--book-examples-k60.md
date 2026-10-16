# book-examples-k60

## Goal

Make "every complete example runs" a checked fact: a Taskfile task extracts each
complete example from the book's Markdown, runs it through the seam Ravel's CLI
tests use, and compares its output exactly, with controls seen to fail.

## Context

- Contract, "Examples policy": `docs/book/contract.md`. The built `ravel`, real
  TypeScript policy evaluation, fake agent programs on a private PATH, a cleared
  environment and a temporary HOME and TMPDIR; no real agent or model; every
  example introduced by a label beginning "Complete example" or "Fragment"; a
  route to a real agent program is validated through a fake executable of the
  same name.
- The seam to reuse is `Fixture` in `crates/ravel/tests/cli.rs` (`env_clear`,
  temporary `home`, `work` and `tmp`, `PATH=/usr/bin:/bin`, the built front,
  `/bin/sh` fixtures that call `end-session`). Its `routed_result_is_exact_*`
  and `refused_policy_never_launches_*` tests are the model for what an example
  asserts.
- Examples appear in chapters as literal Markdown fenced blocks, because the
  book is also read on GitHub and in an editor, where an mdBook `{{#include}}`
  does not run. So the Markdown is the source, and the checker reads the
  Markdown.
- Chapters 3, 4 and 5 are the first consumers; none exists yet, so this leaf
  proves the checker on a fixture page that is not in the book.

## Done when

- A convention for marking a complete example in Markdown is fixed and
  recorded: how its files (policy, fake agents, prompt, arguments) and its
  expected stdout, stderr and exit status are identified in the Markdown so a
  reader sees ordinary labelled code blocks and the checker finds the same
  blocks. Anything the checker needs that a reader should not see goes in HTML
  comments, which render in neither mdBook nor GitHub.
- `task docs:examples` builds what it needs (the existing front and worker), runs
  every complete example under `docs/book/src/`, and compares stdout, exit
  status and the stderr text the example claims, byte for byte. Output that is
  not stable (temporary paths, PIDs, timing) has a stated normalisation rule
  that is visible to the reader or a block the example may omit; nothing is
  compared loosely by default.
- An example needing a platform feature (confinement needs `sandbox-exec` or
  `bwrap`) reports a skip with its reason, never a pass, and the summary line
  counts passed, failed and skipped separately.
- Controls seen to fail: a changed expected stdout, a changed exit status, an
  example block with no expected output, a missing fake program, a stale
  `ravel` build and a Markdown page with an unlabelled runnable block each make
  the task fail. A page with one passing example passes. Each control is run
  and its failure shown here.
- The implementation language is chosen and recorded with the reason. The
  seam's neighbours are Rust nextest cases in `crates/ravel/tests`; a shell
  script under `scripts/` with a `task` wrapper is the alternative. Whichever is
  chosen, `task check` is not made to depend on the book's chapters (a Part I
  edit must not turn `task check` red unless the owner asks for that), or the
  decision to include it is recorded as the owner's.
- Confirm the checker never reads `~/.config/ravel`, a real agent or a real
  model, by running it with `HOME` pointing at an empty directory and a PATH
  that holds only fakes, and by showing the examples still pass.

## Notes

- Also settle, and record for chapter 5, how the checker treats an example that
  must be cancelled by a signal during selection or supervision. If the seam
  cannot run it deterministically without a PTY, the answer is "label it a
  Fragment and say what it omits", and the convention says so.
- The source-excerpt check (exact walkthrough excerpts) serves chapter 8 and
  waits for the second half; it is not this leaf's.
- One in-session reviewer is allowed by the spine's rule, and this is the leaf
  where it is best spent: whether a control would have failed against the
  broken instrument.

## Decisions (running log)

- **Implementation language: a shell script, not nextest (decided 2026-10-16).**
  `scripts/examples.sh` with its POSIX-awk extractor
  `scripts/examples-extract.awk`, wrapped by `task docs:examples`, which builds
  first (`task build`, so the front and worker are current). Reasons, recorded
  in `docs/book/examples.md`: a nextest case is part of `task test` and `task
  check`, so a Part I edit would turn `task check` red unless the cases were
  ignored by default; examples are discovered from Markdown at run time, which a
  compiled case list cannot name; and `figures.sh` and `book.sh` beside it are
  the same shape. The cost is that the seam is written twice (`Fixture` in
  `cli.rs`, and `examples.sh`), so a control reads `cli.rs` and fails when its
  environment (`env_clear`, `HOME`, `TMPDIR`, `PATH=/usr/bin:/bin`) drifts, seen
  to fire against a drifted copy. `task check` is unchanged and does not run the
  check; the new scripts pass its shellcheck command. Not an owner decision: the
  leaf allowed either language.
- **The convention** is in `docs/book/examples.md` (the contract's "Examples
  policy" points to it, as it does for figures). A complete example is a
  paragraph beginning `Complete example`, then label paragraphs (one bold line)
  each directly before a fenced block: a file as ``**`path`**``, `**Command**`
  (`$ `-prefixed lines, run under `/bin/sh -c`), `**Standard output**` (or `…,
  without a final newline`, for an opaque result), `**Standard error**` (or
  without a final newline, or `includes`), `**Exit status**`. The command is
  what the reader sees, so the example writes `PATH="$PWD/bin:$PATH" ravel …`
  itself and the checker's PATH is only `<scratch>/ravel:/usr/bin:/bin`. The
  only text the checker needs that a reader should not see is the one-line
  comment `<!-- example: requires NAME -->`.
- **Newline is a visible label, not a rule.** A fenced block always ends in a
  newline, and an opaque result has none, so `Standard output` claims block +
  `\n`, and `…, without a final newline` claims the block as written. An empty
  block claims an empty stream. Nothing is stripped on either side.
- **One normalisation:** every occurrence of the example's scratch directory in
  stdout and stderr becomes `<scratch>`, physical path first because macOS's
  `/var/folders/…` is a substring of `/private/var/folders/…`. A control shows a
  raw `/tmp/work` in a claim does not match. Standard error with no block claims
  nothing; there is no loose comparison otherwise.
- **Cancellation by signal (for chapter 5): not a PTY problem.** A fake agent
  that runs `kill -INT "$PPID"` cancels the supervised session
  deterministically (status 130, `session_cancelled`; SIGTERM 143, SIGHUP 129),
  and a policy that runs `process.kill(process.ppid, "SIGINT")` cancels the
  selection (status 130, `selection_cancelled`). Both are complete examples; a
  five-run control passes. Two things are not claimed: the front's `stop attempt
  N target -PGID` lines (a process-group id, and under this session's tool
  sandbox also `Operation not permitted`), so those examples use `Standard error
  includes`; and the worker's reap detail in a selection cancellation, which
  alternated between `signal: 15 (SIGTERM)` and `exit status: 0` across three
  runs. A cancellation that needs a terminal (typed Ctrl-C, job-control stop,
  closed terminal) is a `Fragment` that says the terminal is what it omits.
  The cause of the `Operation not permitted` was not established: the run
  outside the tool sandbox that would have shown it was refused by the session's
  permission classifier and not retried.
- **Stale build is refused by digest, before anything runs.** `build.rs` embeds
  the digest of the worker source in the front and `dispatch.sh` in the worker;
  `dispatch.sh build-id` prints the checkout's current one, and the checker
  refuses a front or a worker that does not contain it, naming which. This
  covers worker-source staleness. Rust-source staleness of the front is the
  build step's: `task docs:examples` always builds first, and the script run on
  its own is told to (`no built ravel`, `stale build`). The product also refuses
  a mismatched pair when it runs, so the checker's refusal is earlier and
  better named rather than the only one (with the front's check removed the
  control still fails, on the product's own worker refusal).
- **Platform skips:** `requires confinement` is present when `sandbox-exec` (macOS)
  or `bwrap` (Linux) is. The summary counts passed, failed and skipped apart and
  prints the reason; an unknown requirement fails the page. `EXAMPLES_ASSUME_MISSING`
  is the controls' seam for a host that has the feature.
- **The reviewer (the leaf's one allowance, spent).** A fresh context was given
  the three files, the fixtures, the seam and the Done-when, without my
  conclusions, and asked for false passes and vacuous controls. It found, each
  reproduced by it or by me, and each fixed with a control seen to fail:
  - *Hidden blocks.* A fence opened on a list-item line paired wrongly and let a
    later unlabelled `sh` block through; only ten languages counted as runnable
    (so bare fences, `shell-session`, `Bash`, `fish` and indented `$ ` transcripts
    went unseen). Now fail closed: outside an example a block must be on the list
    of never-an-example languages (case-insensitive) or it is an error, a bare
    fence is an error, a list-item fence and an indented `$ ` block are errors.
  - *A claimed `> hi` inside a quoted block lost its marker* (every `>` was
    stripped, not the opener's depth). Now exactly the opener's depth.
  - *CRLF* failed closed with a misleading message; now named. `{4,}` in a regex
    (not in mawk 1.3.4) replaced; not run under mawk, which I did not have.
  - *Silent overwrite:* two `Standard output` or `Exit status` blocks, or an exact
    stderr then `includes`, kept only the last. Now errors. An empty `includes`
    matched anything; now an error.
  - *Order:* with ten or more Command blocks the glob ran `run-10` before
    `run-2`. Now numeric, with an eleven-command control.
  - *Injection:* a `'` in a file label reached `system()` at extraction. Labels
    are now letters, digits and `. _ - /` only; a control puts `$(touch …)` in a
    label and shows nothing runs.
  - *A claimed 142 excused a real timeout.* The runner now forks the command into
    its own process group, kills the group at the limit and leaves a marker the
    comparison reads, so no claim excuses a timeout.
  - *Stale-build check covered only the worker's TypeScript.* It now also refuses
    when any Rust, Cargo manifest or lock file is newer than the front, or any
    worker TypeScript, tsconfig, package manifest, lock or `dispatch.sh` is newer
    than the worker. A touched file refuses (the safe direction).
  - *Mutants that survived the first controls* (HOME or TMPDIR dropped from
    `env -i`, stdin inherited, a fragment label covering every following block,
    prose no longer clearing a label, blockquote, tilde and long-fence support
    removed) each got a control (the seam's own `HOME TMPDIR PATH` printed from
    inside an example, a piped stdin, `shapes.md`, the fragment and prose pages).
  - *Weak controls corrected:* the newline control's second check was loose; the
    "fakes only" run now really narrows the caller's `PATH` to the fakes plus
    `/usr/bin:/bin`; the mutant mode now runs each named control against the
    unmutated checker first, so none kills a mutant by already failing.
  - *Documentation:* the note said a `requires` comment goes directly after the
    label (it is accepted anywhere in the example) and that the language tag is
    never read (it is read outside examples); both corrected. The normaliser is
    not one-to-one, and the note now says so. A Command block's status is its last
    command's, and the note now says so.
  - *Not changed, with the reason:* the default target (`docs/book/src`) and the
    Taskfile wiring have no control and are vacuously green until chapter 3
    lands (`task docs:examples` run once: 0 passed, 0 failed). The platform
    branches of `requires confinement` are exercised only through
    `EXAMPLES_ASSUME_MISSING`, because this host has the feature. The reviewer's
    own environment failed the cancellation control (a SIGINT sent to the parent
    did not arrive there); it passes here five runs in a row, and the cause there
    was not diagnosed. Chapter 5's cancellation examples depend on that behaviour
    and fail, not skip, on a host that forbids it.
- **Controls, each seen to fire** (`task docs:examples:test`, 41 passes): the
  Done-when's six (a changed expected stdout, a changed exit status, a command with
  no stdout block, a fake program the page does not supply with a `claude` that
  records itself on the caller's PATH not running, a front and separately a worker
  with another digest and an older pair, a runnable block with no label), a page
  with one passing example, and the further refusals named above and in
  `docs/book/examples.md`. The unchanged fixture pages pass.
- **Mutants** (`task docs:examples:mutants`): 26 safeguards removed one at a time
  from a copy of the checker, the control named for each failing against it:
  26 of 26 killed. The first run killed 23 of 26 and showed the three survivors
  were a control that accepted either of two messages (a bare fence), a mutant
  paired with a control that fails for the opposite reason (quote depth), and a
  control whose path contained `//` and so was refused for another reason (label
  injection); each was repaired.
- **Isolation (Done-when's last item):** every example runs under `env -i` with an
  empty `HOME`. The control also runs the checker with `HOME` holding an invalid
  `~/.config/ravel/settings.json` (read, every session would refuse as
  `settings_invalid`) and a PATH leading with `claude`, `codex` and `ravel` that
  write a marker file: the fixture examples pass and the marker is absent. It runs
  again with an empty `HOME` and a PATH of two fakes plus `/usr/bin:/bin` only, and
  passes.
