# Ravel 2.0.0 book proof

Grove leaf `ravel-book-inside-k311` completed this proof on 12 November 2026.
The starting Ravel commit was `7d9526e9`, the completed art pass.
The work changes documentation only. It neither cuts nor promotes a release.

## Reading and corrections

One independent reviewer read all eleven chapters, the whole contract and the
word decisions. That read included code, tables, captions and alternatives.
The producing session checked the handed-forward cases, source behaviour,
installed examples, links and each reported correction.

The reviewer checked every chapter against every Read 1 pattern in contract
section 2. These patterns cover concrete-first examples, technology
introductions, local terms, actors and outputs, and mechanisms. They also
cover figures with real examples, reader effects and goals, actions through
results, restatement and conversational address.

| Chapter | Concrete teaching path | Result of the proof |
|---|---|---|
| 1 | A code review from request through the caller's next action | Distinguished original and prepared prompts in summaries |
| 2 | The review's participants, authority and inherited paths | Reconciled the first-session file inventory |
| 3 | The first session, its files, output and cleanup | Selected the versioned formula, declared skill settings and qualified terminal recovery |
| 4 | Fake routes, refusals, imports, data and harness calls | Declared settings for installed examples and corrected prompt, settings, reply-check and permission summaries |
| 5 | Caller data, outcomes, nested sessions and cancellation | Reconciled example setup, environment changes, skill outcomes and terminal recovery |
| 6 | Self-contained lookup entries with examples and teaching links | Corrected the formula prefix, settings-key count, descriptor retention and skill diagnostic/status coverage |
| 7 | The parts made visible by two fake programs | Removed the absolute worker-deadline claim and reconciled explicit skill settings |
| 8 | The same session through the current source | Reconciled the trace's disabled skill input with the example |
| 9 | Escalation, evaluation deadlines and terminal ownership | Declared explicit skill settings for each executable example |
| 10 | A concrete change, its tests and checkout workflow | Defined local terms and moved timing detail to the existing evidence record |
| 11 | A versioned-first release, promotion and recovery | Defined local terms and replaced the duplicate check log with its scope and evidence link |

All independent findings were valid and actionable. None required another
review context or a correction run. The example-input corrections have an executable
comparison, and the remaining edits correct summaries, definitions and evidence
placement. No figure or source excerpt changed.

The contract's stale detached-script-child limit now names descendants of
evaluation harness runs. The Rhai worker itself starts no process. Its stale
first-release statement now distinguishes the existing 1.1.0 book from a
later release's copy. These edits update facts and preserve the owner's
structure, emphasis and reader checkpoints.

Historical TypeScript-worker removal statements, checkout-only Bun tools and
skill-source payload paths under `libexec` remain valid. The read found no
current 2.0.0 claim that requires the retired worker, SDK or runtime.
The release placeholders remain in the chapters for the release tooling.

## Verification

`inputs.json` gives SHA-256 digests for 288 subjects under `docs/book`,
`scripts` and `crates`, plus the Taskfile, Cargo manifests/lockfile,
`rust-toolchain.toml` and `.cargo/config.toml`. These subjects did not change
between the start and end of both final runs.

| Check | Observed result | Record |
|---|---|---|
| `mise exec -- task docs:book:build` | No error or warning | `checks.log` |
| `task docs:examples` | 27 passed, none failed | `checks.log` |
| `task docs:excerpts` | 22 passed, none failed | `checks.log` |
| `task docs:figures` | 21 match their sources | `checks.log` |
| `task docs:ste` | No problems in 35 files | `checks.log` |
| `task docs:breaks` | No long run in eleven chapters | `checks.log` |
| Installed-prefix `task docs:examples` | The same 27 cases passed, compared item by item | `installed-examples.log` |
| Rendered chapter links | 1,173 local references valid | `links.json` |
| Linked repository files | All 16 distinct targets exist in the checkout | `links.json` |
| External destinations | All 17 respond successfully, with redirects recorded | `links.json` |

The public CLI help of the root, `start-session`, `end-session` and
`skills resolve` supplied the flag inventory. Every flag appears in chapter 6.
The link instrument accepted a known chapter anchor and rejected planted
missing-page, missing-anchor and missing-figure targets before the inventory.
An initial instrument run included the generated 404 page as a chapter. The
final run restricts the scope to the eleven numbered chapters.

### Installed example control

A private prefix held a byte-for-byte copy of `target/debug/ravel` and a
real `share/ravel/installation.json`. The record named a directory containing
a valid `proof-fixture` source, with a `skills` root and guidance file.
`RAVEL_FRONT` selected that executable for the existing example task.
No real Homebrew keg or owner settings changed.

Before correction, this valid installed fixture exposed eight failures in
chapters 4 and 5, while 19 examples passed. The normal checkout check passed.
`installed-before.log.gz` retains that counterexample. An earlier fixture
without a `skills` or `craft` root was invalid and supplies no example evidence.

Every session example now explicitly names an empty skill-source array in
its private settings. The retired-format example refuses before skill
resolution, so it needs no such setting. The final installed run and the
checkout run agree case by case, not only in their totals.

## Viewing and limits

The session examined the raw Markdown and generated local HTML. Firefox in an
isolated Linux TestAnyware guest displayed chapter 3's revised first-session
file tree and settings table without clipping. This is a targeted browser
inspection, not another whole-book art pass or a reader-comprehension trial.
The prior art pass inspected the figures. The session stopped the guest afterwards.

The external check reached the existing Pages site. It does not inspect the
unreleased 2.0.0 book on Pages. The link check resolved `blob/main` files against the local checkout,
because this change has not landed on public `main`.
The GitHub-rendered view of this revision and the released 2.0.0 Pages copy
remain publication checks.

The graph was ready at generation `2026-11-02T05:30:19Z`, in fast mode.
Coverage excludes `docs` and `scripts` and reports changed metadata for
`crates/ravel/src/skills/launch.rs`. Its snippet was stale. Current source
reads, rather than that snippet, establish source preparation before worker
evaluation. Graph cleanliness supplies no completeness claim here.

This proof does not replace the owner's Read 1 or Read 2. Both remain with
the paused Rhai workstream. The session changed no paused task file and retired no owner checkpoint.
It claims no new reader-comprehension result.
