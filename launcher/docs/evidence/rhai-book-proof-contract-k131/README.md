# Reference obligations for the final book proof

This map supports `rhai-book-inside-k130`. It records reference obligations, not book compliance.
The final proof must read all eleven chapters freshly. Earlier partial reads do not count.

## Reference subjects and limits

The eight repository references below were read in full.
`references-before.sha256` and `references-after.sha256` identify their exact contents.
The references did not change during this check.

| Reference | Purpose |
|---|---|
| [Book contract](../../book/contract.md) | Reader tasks, complete inventories, examples, figures, style and proof evidence |
| [STE decisions](../../book/ste.md) | Shared words, signal names and technical exceptions |
| [Documentation lessons](../../documentation-lessons.md) | Owner feedback and lessons L01–L86 |
| [Rhai decision](../../adr/configuration-script-uses-rhai.md) | Language choice, migration and refusal behaviour |
| [Evaluation decision](../../adr/configuration-evaluation-precedes-the-launch.md) | Separate worker and parent authority |
| [Model decision](../../adr/a-configuration-script-asks-a-model-through-a-harness-run.md) | Harness transport and owner recipes |
| [Supervisor decision](../../adr/ravel-supervises-the-session.md) | Result authority and caller responsibilities |
| [Job decision](../../adr/the-launched-child-is-a-job.md) | Process groups, terminal authority and cleanup limits |

Graph coverage reported `no_recorded_issue` for these paths on 14 November 2026.
The index generation was `2026-11-13T17:05:39Z`. This report does not prove completeness.
Direct reference reads supply the evidence here. This child makes no verified structural code claim.

The owner PDF at `/Users/antony/Downloads/ASD-STE100_ISSUE9.pdf` could not be read.
Checksum and bounded read attempts stalled without content or a digest.
The final proof must consult applicable Part 1 rules and check the expected digest below.
It must preserve the PDF privately. Neither the PDF nor extracted text belongs in the repository.

```text
Expected PDF SHA-256: d1f4ea9e7cd6e46b47aa9057209f99e78c0e9cfc4e27a5b07895b05c1a166431
```

## Concrete proof obligations

The contract supplies the full criteria. This table makes the cross-document checks explicit.

| Subject | Final proof check |
|---|---|
| Reader path | Present each topic with a concrete task, inputs, workspace and observable result. Explain prerequisites before commands. |
| Local explanation | Explain terms at each chapter entry. Name the actor, action, place and reason. Repeat explanations where entry paths need them. |
| Configuration | Distinguish the configuration script, returned settings, Rhai worker and selection context. Preserve exact option and field names. |
| Worker | Explain deadline enforcement, capability bridges and failure isolation. Show worker reap and private-directory removal before managed launch. |
| Source path | Trace the final cancellation decision through named functions and exact excerpts. Byte presence alone does not prove a call path. |
| Source inventories | Reconcile CLI, configuration, lifecycle, component and workflow inventories against current subjects. Coverage does not establish comprehension. |
| Commands | Give complete commands before their parts table. Name required files, their purpose and the reader's current directory. |
| Examples | Execute every complete example through the repository seam. Compare output and status exactly. Label excerpts and illustrative fragments. |
| Migration | Demonstrate refusal for a lone `policy.ts`. Explain `config.rhai` precedence without implying a JavaScript fallback. |
| Figures | Reconcile installed files, source components, process authority, session paths, cancellation and release delivery with current source. |
| Figure evidence | Check reproducibility and chapter-width readability separately. State which local, published, GitHub and raw surfaces were viewed. |
| Style | Apply Issue 9 writing rules and shared technical exceptions. Use British spelling and descriptive headings. Preserve meaning when shortening prose. |
| Limits | Keep instructions within 20 words and descriptions within 25. Keep paragraphs within six sentences and prose runs within 450 words. |
| Links | Check destinations and inbound link text. Built-book links must not depend on checkout-only ADR paths. |
| Evidence | Freeze subjects before checks. Compare each digest afterwards. Demonstrate dirty controls and report environment limits. |
| Owner | Preserve actual reader words, progress and locations. Automated checks cannot establish novice comprehension. |

## All documentation lessons

Each range below has a concrete check. The full lesson text remains authoritative.

| Lessons | Check |
|---|---|
| L01–L11 | Follow the reader's task. Check prerequisites, concrete cases, local terms, actors, mechanisms, goals, result meaning and useful repetition. |
| L12–L18 | Preserve meaning under style edits. Check consistent words, scoped guarantees, exceptions and the distinction between design, delivery and evidence. |
| L19–L25 | Derive inventories from source. Date real-agent flags. Execute visible examples privately. Keep excerpts exact and handoffs green. Check installed versions. |
| L26–L34 | Give each figure a reader question. Check useful prose breaks, shared style, semantics, pins, chapter width and alternatives. State viewing limits. |
| L35–L42 | Demonstrate dirty controls. Enumerate subjects before checks. Freeze inputs. Explain failures. Check links and checker scope. Apply reader patterns throughout. |
| L43–L51 | Separate owner words from interpretation. Explain tradeoffs and causal limits. Preserve handoffs, task scope, lessons, rights and fake-agent boundaries. |
| L52–L65 | Name real inputs and caller tasks. Teach concrete cases before abstractions. Check JSON purpose, headings, commands, renames, fragments and inbound links. |
| L66 | Distinguish a reason to stop from incomplete work. Preserve remaining work in precise leaves. |
| L67 | Expose setup and state. Hidden runner help and model checks cannot substitute for a novice tutorial. |
| L68 | Distinguish shell commands from interactive session exit actions. |
| L69 | Check whether each qualifier implies an unsupported complementary claim. |
| L70 | Label commands that cannot run before installation. |
| L71 | Distinguish a narrated scenario from a real run. Name the differing inputs. |
| L72 | Derive component lists again after component removal. |
| L73 | Demonstrate different process observations. Explain CLI arguments versus the worker process name. |
| L74 | Check built-book access to ADR references without checkout-relative links. |
| L75 | Isolate default skill sources through private settings. Empty HOME and `env -i` alone do not isolate `installation.json`. |
| L76 | Trace named functions from `main` through worker supervision. Check actual call relationships beside exact excerpts. |
| L77 | Match runtime questions to demonstrated output. Otherwise label the answer as source-derived. |
| L78 | Compare subjects against the last thorough revision. Account for newly added launch modes. |
| L79 | Demonstrate cancellation after a deadline with paired runs. Explain status 143, `selection_cancelled` and original `selection_timeout` versus status 124. |
| L80 | Unset ambient `HOMEBREW_NO_INSTALL_CLEANUP` before the mutant suite so controls can fail. |
| L81 | Check event order expressed through words such as “yet” and “until”. |
| L82 | Reconcile waiting tasks with delivered proof before another draft. |
| L83 | Check external transitions before present-tense claims. Reconcile the promoted unversioned formula and deprecated `ravel@2`. |
| L84 | Keep test placeholders outside the real home when an earlier shell prefix does not cover later commands. |
| L85 | Explain each step from current state. Do not imply broad trust from one Homebrew check. |
| L86 | Check the fixed 2.0.0 installation and PATH remedy. Explain what `command -v ravel` establishes. |

## ADR mechanisms and limits

The Rhai decision requires a clean 2.0.0 cutover. Explain the loss of JavaScript ecosystem tools alongside the simpler installation.
A lone `policy.ts` refuses. A present `config.rhai` wins.

The parent supervises a separate worker in the same binary. The parent owns deadlines and capability effects.
An evaluation thread cannot contain stack or heap failure. A second installed binary would require version pairing.
The worker is reaped before managed launch. All host calls must settle before that launch.

`run_harness` uses owner recipes and existing sign-ins. Ravel does not supply a model service or parse arbitrary harness arguments.
Explain process cost and budget limits. Reject nested work that cancellation cannot reach through the selected process group.

Ravel remains the supervisor. The caller supplies the task, ending instructions and interpretation of the opaque result.
Explain first-writer behaviour, empty and multiline results, and natural exit without a result.
Same-user processes can submit the wrong first result. The result path does not authenticate a sender.

Managed jobs use their defined process groups and terminal authority. Captured harnesses use a separate POSIX session without a terminal.
The Rhai worker is not a managed runner job. Explain signal restoration and checked terminal handback.
State escaped-descendant and uncatchable-death limits. Do not extend native platform evidence beyond its actual scope.

## Evidence and final handoff

The final proof must reconcile these existing artifacts by subject digest:

- [Gate evidence k123](../rhai-book-gates-k123/README.md).
- [CLI inventory k125](../rhai-book-cli-inventory-k125/README.md).
- [Configuration inventory k126](../rhai-book-configuration-inventory-k126/README.md).
- [Lifecycle inventory k127](../rhai-book-lifecycle-inventory-k127/README.md).
- [Component inventory k128](../rhai-book-components-inventory-k128/README.md).
- [Workflow inventory k129](../rhai-book-workflows-inventory-k129/README.md).

Changed subjects require fresh checks. Frozen historical `ravel-book-k311` evidence must remain unchanged.
Use `mise exec -- task setup` before development checks that require setup.
The Taskfile defines example, excerpt, figure, STE, prose-break and book-build checks plus their controls.
A report-only checker exit does not establish a strict pass. Record exact commands and observed failures for controls.

This child changes no book, source or checker subject. It does not rerun those suites or claim that they pass now.
Its own checks cover reference digests, artifact links and prose. Native arm64 macOS evidence does not establish Linux or real-harness behaviour.

Older lesson entries describe historical states. The current enclosing briefs establish Read 1 completion and delivered release 2.0.0.
Keep those historical statements dated. Do not silently treat them as current requirements or erase their lessons.
Real-agent flags recorded on 16 October 2026 remain dated observations. New current claims require fresh verification.

The final proof must clear or explicitly assign every Handed forward item. It must reconcile settled parent criteria before closing nodes.

After final proof, create owner Read 2 as a requirements leaf outside the editorial node.
The owner must explain the separate worker and two Rust packages, trace final cancellation, and build and test the checkout.
Record the owner's words, progress, locations and unreached material. Stop the read at an early comprehension failure.
A model proxy cannot complete Read 2.

## Artifact checks

The artifact link check resolves each relative Markdown destination from this directory.
All 14 destinations passed. A missing destination control failed the same check.
The digest check compares all eight reference subjects individually.
All eight references matched. A changed expected digest control failed the same comparison.

The technical-communication checker covers this artifact and the task prose.
These checks establish artifact integrity only. They do not establish book compliance.
