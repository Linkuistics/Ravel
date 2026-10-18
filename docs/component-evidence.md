# Component source and observation evidence

This document is an evidence record. It does not teach. It keeps the source
references and the observed runs from a read of Ravel's components on
2026-10-14. It also keeps the provenance of the local runner.

The book teaches the same components in chapters 7, 8 and 9. Those chapters
replaced the component guide, the local runner guide and the result channel
guide. Read the book for the explanation. To check the source behind a
claim, read this document. It also shows what the observed runs establish and
what they do not establish.

## Source reference and discovery

The source reference is commit `0bdcb1f9ae24db5e9dd499308dd791744e908eb0`.
The links below point to the current files. The line numbers in this document
belong to that commit. To read a file as it was then, run
`jj file show -r <commit> <path>`.

Graph discovery used the project
`Users-antony-Development-Ravel.simplify-and-componentize`, Tier 2 verification,
generation `2026-10-14T03:03:14Z`. The searches and traversals used qualified
names. Their relevant pages were complete. Exact snippets and direct source
reads verified the material claims. These claims include the top-level worker
startup and the connections between processes.

One reported edge from `result_after` to a test `Scratch::new` did not match the
source. The read discarded it. A graph edge is a navigation aid. It is not runtime
evidence.

The coverage checks reported no recorded gap and metadata that agrees for the cited
source paths. This result is best-effort. It does not prove that the index is
complete. The coverage refreshed at generation `2026-10-14T03:28:35Z`, and the
production source did not change.

## Source anchors

Each row names one area that the read inspected. The cells list the files and the
functions that the read used. Line ranges are in parentheses.

| Area | Files and functions |
|---|---|
| E1: Request checks | [choice.rs](../crates/ravel/src/choice.rs): `prepare` (43–58). [authority.rs](../crates/ravel/src/authority.rs): `resolve` (34–117). [settings.rs](../crates/ravel/src/settings.rs): `Settings::read` (30–93). [inputs.rs](../crates/ravel/src/inputs.rs): `Inputs::read` (73–132). [environment.rs](../crates/ravel/src/environment.rs): `Grants` and the grant exclusions |
| E2: Policy | [choice.rs](../crates/ravel/src/choice.rs): `choose_prepared`, `evaluate_and_resolve` and `judge`. [worker.rs](../crates/ravel/src/worker.rs): `locate` and `evaluate` (316–529). Also `converse` (572–628) and `verify_hello`. [worker main.ts](../crates/ravel/worker/src/main.ts): startup (1–80), `converse`, `load` (196–223), `assemble` and `select` (304–330) |
| E3: Composition | [main.rs](../crates/ravel/src/main.rs): `main` (29–57) routes both verbs. [run.rs](../crates/ravel/src/run.rs): `run` (32–247), `settle` (250–294), and the output reservation and restoration. [program.rs](../crates/ravel/src/program.rs): `resolve`. [confinement.rs](../crates/ravel/src/confinement.rs): `Confined::prepare` |
| E4: Job | [runner lib.rs](../crates/keyed-launch/src/lib.rs): the public contract. [runner run.rs](../crates/keyed-launch/src/run.rs): `Launch`, `Ended`, `run_observed`, `run_noninteractive`, `run_confined_observed`, `supervise` (1160–1195) and `watch` (1197–1307) |
| E5: Result | [ravel lib.rs](../crates/ravel/src/lib.rs): the exports. [completion.rs](../crates/ravel/src/completion.rs): `SessionChannel`, `result_after` (56–70), the cleanup and read functions, `end_session` (135–145), `publish_result` (150–188) and the regular-file bounds |
| E6: Delivery | The [workspace manifest](../Cargo.toml), the [ravel manifest](../crates/ravel/Cargo.toml) and the [runner manifest](../crates/keyed-launch/Cargo.toml). [build.rs](../crates/ravel/build.rs), [dispatch.sh](../crates/ravel/scripts/dispatch.sh) and the [Taskfile](../Taskfile.yml). The check of the worker identity |
| E7: Change footprint | [run.rs](../crates/ravel/src/run.rs) composes the product. [session_process.rs](../crates/ravel/tests/session_process.rs) `supervisor_fixture` (134–193) composes the transport and the runner without policy evaluation |

The [runtime evidence](runtime-evidence.md) owns the results of the resolver and
startup probes, and the limits for an upgrade. The read did not run those probes
again. The specification and the [delivery chapter of the book](book/src/12-deliver-ravel.md) document the
install and archive contracts. The two Rust packages, `ravel` and `keyed-launch`,
both set `publish = false` in their manifests.

In E7, the caller supplies the scrub and the escalation, so the choices that allow
the composition are visible. The fixture is a test consumer. It is not evidence
that an external project adopted the interfaces.

## Observed scenarios and limits

On 2026-10-14, on this macOS host, `task setup` and `task build` succeeded. They
used the pinned development runtime and the locked Rust dependencies. The read
then ran the `task test:session` seam with this exact nextest filter:

```sh
task test:session -- -E 'test(routed_result_is_exact_prompt_is_unchanged_and_child_stdout_is_separate) | test(cancellation_during_selection_launches_nothing_and_during_session_reaps_group) | test(cancellation_reaps_the_child_and_discards_an_already_published_result)'
```

All three named cases passed. The filter left 74 other cases outside the run.
[cli.rs](../crates/ravel/tests/cli.rs) supplies the real-policy and fake-program
fixture. The fixture uses `env_clear`, a temporary HOME and TMPDIR, and a fixture
PATH. [session_process.rs](../crates/ravel/tests/session_process.rs) supplies the
process fixture that sends a result and then cancels. The run used no personal policy and no real
model service.

### What the cases checked

| Case | What it checked |
|---|---|
| Exact result | The CLI fixture prints ordinary child output. Then it calls `end-session` with a value and exits with status 7. The run checked ordinary, empty, multiline, Unicode and option-like results. It also checked an unchanged multiline prompt, the separation of standard error, and an empty temporary directory afterward |
| Cancellation by the CLI | The CLI fixture sent TERM during selection and during managed execution. The run checked the temporary cleanup and that the managed process and its group no longer existed |
| Cancellation after a sent result | A separate transport and runner fixture sends a result first. Then it sends TERM to its supervisor. It checks an `Ended` value that shows the interruption and `Group::Gone`. It rejects a read of the result and it closes the channel |

The exact-result fixture does not force the 2-second grace to expire. The runner's
escalation tests are the separate seam for that branch. The last case supports the
lower layer of the race. It does not exercise every late-signal window of the CLI.

### Digests of the subjects

SHA-256 digests covered `Cargo.toml`, `Cargo.lock`, `Taskfile.yml`, the built
Ravel CLI and worker, and every file under `crates`, `scripts` and `.cargo`. The
digests excluded dependency `node_modules` and the generated `target` trees. The
read took them before and after the run. The sorted subject-list digest was
`1c97953a8012bf60c9f9c3a4cef63d14c63a11fa29c3db937220b695a52b72b6`. The identity of
the before and after digest list was
`c5be21debad205bd914bfaf71de19a086f79a542c757827620732df25f8d8d8d`.

The full anchor identifies the source fixtures and the expectations. The
lockfiles pin the development dependencies. The read did not digest the installed
bytes of those dependencies. Nextest compiled fresh test binaries for the run.

### Combined-cancellation unit cases

The read also ran two unit cases through `task test:session`. Each ran with a
filter for its exact name. Both passed, and the filter left 75 other cases
outside the run.

- `cancellation_overrides_prelaunch_failure_and_reports_every_cleanup_error`
  checks a combined cancellation, result failure and cleanup failure.
- `cancellation_reports_a_surviving_group_and_suppresses_the_result` checks the
  cancellation together with the group diagnostic.

An earlier selection with `--lib` selected no case. These tests belong to the
executable target. The empty run gives no evidence. The successful rerun used the
same subject list.

Its before and after digest-list identity was
`5ed2dcc7b29bf30f2e2ee53c135ce3560c949d6393040cc5d7782a8f096e3655`. That identity
differs from the first run because the Taskfile then held a task that served the
component views. Production source and expectations did not change.

The rule for a combined result failure and channel-close failure without
cancellation comes from the source. No case in this read observed it. A change to
the code that settles the outcome owes a check of that rule.

### Limits

These observations do not establish these items:

- every late-signal window,
- terminal recovery,
- resistance to a hostile runtime,
- confinement,
- installed layouts,
- platform floors,
- unknown owner helpers,
- complete legacy compatibility.

The checks that the repository already has for these items stay part of the handoff of any change that
affects them. The read did not include a check with human readers.

## Acceptance seams for the ten obligations

The component read listed ten required outcomes, O1 to O10. The book teaches
each one in chapters 7, 8 and 9. The table keeps the seam that checks each
outcome. “Source” means that the read inspected the implementation.

“Observed” means that the read ran one of the three named cases above. A seam is
a place to check a promise. It does not show that the read exercised the promise.

| ID | Outcome | Source and seam |
|---|---|---|
| O1 | The owner's current policy, SDK use, settings and discovery stay compatible | Source E1 and E2. The `cli` context and settings cases, the authority and settings unit cases, `task typecheck` and the installed `task smoke` |
| O2 | The policy has no session-result authority. The CLI checks the worker identity before the policy handover. The CLI reaps the worker before the managed launch | Source E2 and E3. The `cli` context, environment, identity and deadline cases and `task test:hostile` |
| O3 | The CLI judges decisions, refusals, exceptions, malformed values and limits without a fallback. The prompt and parameters stay caller data | Source E2 and E3. The `cli` refusal, context and deadline cases. The exact-prompt case, observed |
| O4 | The selected argv runs directly in the caller's context, with the requested confinement or a refusal, and with a fresh nested result context | Source E3 and E4. The `cli` nested and confined cases and the runner argv and environment cases |
| O5 | The first complete result that a process sends wins. Empty, multiline and Unicode values stay exact. Stale or missing context and invalid result files refuse | Source E5. The unit cases for the send and for regular files and the nested and stale process and CLI cases. The exact bytes, observed |
| O6 | Success needs the reap of the launch that owns the channel and verified group cleanup. A result that the process already sent does not make the later child status decisive | Source E3 to E5. The runner descendants and wait-error tests. The exact-result case and the case of a cancellation after a sent result, observed |
| O7 | INT, TERM and HUP cancellation suppresses results, also when the process already sent a value, and propagates as a signal after cleanup | Source E3, E4 and E5. The signal-transition unit cases. The cancellation cases for selection, for the managed process and for a sent result, observed |
| O8 | The runner, inside the CLI, recovers the interactive foreground and the saved terminal modes. Descendant-group cleanup comes before the reap | Source E4 and the [job decision](adr/the-launched-child-is-a-job.md). The real-process and PTY runner tests of the repository. The read did not run them again |
| O9 | Only result bytes reach standard output, after explicit cleanup. A cleanup or write error cannot become success | Source E3. The unwritable-output and cleanup CLI case and the unit cases for the final decision. The separation of the streams, observed |
| O10 | Installed execution uses the matched Ravel CLI and worker pair and the shipped SDK, with no other project and no host Bun or Node | Source E6. The fresh local build, observed. The read did not run the installed and symlink smoke checks and the target archive checks |

## Component questions that the read left open

The component guide ended with five boundary questions. The book keeps their
facts in chapter 7. It does not keep three of the questions, because the read had
not answered them. This section records those three as the guide stated them.

- **Worker and job lifecycle.** The worker uses a private protocol, bounded
  diagnostics, descriptor isolation and controls on its first steps before the policy runs.
  The managed job uses foreground and process-group supervision. Similar spawn and
  wait code does not make the two contracts interchangeable. The read selected no
  extraction of a generic worker.
- **Reuse that includes the result.** Four items span the transport, the runner and the
  CLI. They are the match between a channel and its launch, the cancellation, the
  explicit cleanup and the final decision on output. A new package alone may export the same
  protocol. Compare the ownership and interface arrangements before any extraction.
- **Reuse of the generic job.** The public Rust interface of the runner avoids
  policy conventions. The package is private, it uses Unix mechanics, and it
  leaves explicit obligations to its caller. Before anyone claims independent use,
  the package needs an external consumer example and a delivery and support
  arrangement.

## Local runner source provenance

The first version of `crates/keyed-launch` was a copy of the runner implementation
and its original test suite. The source was `crates/keyed-launch` at commit
`639b9d63570ed87f81f3ced3d2851b7961006bf3` of
[Linkuistics/grove](https://github.com/Linkuistics/grove/tree/639b9d63570ed87f81f3ced3d2851b7961006bf3/crates/keyed-launch).

The copy keeps the process and terminal implementation, the Rust floor of the
source project, the Apache-2.0 licence and the locked dependency versions. The
copy generalised the caller-specific prose. The test changes add independent
authority guards and their regression case. They do not redesign the runner.

The [job decision](https://github.com/Linkuistics/grove/blob/639b9d63570ed87f81f3ced3d2851b7961006bf3/docs/adr/the-launched-child-is-a-job.md)
of the source project documents its launch behaviour. The active
[job decision](adr/the-launched-child-is-a-job.md), the
[executable specification](specs/session-execution.md) and the
[runtime evidence](runtime-evidence.md) accompany this implementation. These
references establish provenance. They add no executable dependency.

## Related evidence documents

- The [source evidence for the result and the job](result-job-source-evidence.md)
  records the acquisition and stop state. It also records the
  [smallest missing premises](result-job-source-evidence.md#counterexamples-and-the-smallest-missing-premises)
  of the failure paths at a stated source reference.
- The [completion failure evidence](completion-failure-design-evidence.md) records the
  [C1 checks](completion-failure-design-evidence.md#c1-checked-observations),
  the [C2 checks](completion-failure-design-evidence.md#c2-checked-observations),
  the [C3 checks](completion-failure-design-evidence.md#c3-checked-observations)
  and the [C4 checks](completion-failure-design-evidence.md#c4-checked-observations)
  of the repairs, with their measured limits.
- The [native-entry evidence](native-entry-development-evidence.md) records the
  measurements of the executable entry state.
- The [runtime evidence](runtime-evidence.md) records the resolver and startup probes.
