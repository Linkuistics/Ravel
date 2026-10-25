# llm-calls-through-harnesses-k59

**Reviews:** llm-calls-through-harnesses-k58

## Goal

Read the design change of `llm-calls-through-harnesses-k58` adversarially, and report findings without fixes. The change moves every LLM call to a harness run of Claude Code or Codex. It removes `ravel::llm`, the `endpoints` settings key, the LLM budgets and Rig, and it keeps the schema check as `ravel::json_check`.

## Context

- The producer's commit holds the change. Find it with `jj log` and the handle. It touches `docs/specs/policy-evaluation.md`, the ADR set, `CONTEXT.md`, `docs/design/rhai-policy/` and the briefs.
- The running log of `llm-calls-through-harnesses-k58` holds the owner's decisions. Do not reopen a settled decision. Report a consequence that the decision did not see.
- The owner's amendment to GitHub issue 2 is a comment on the issue. Read it with `gh issue view 2 --comments`.
- `task docs:rhai-policy` serves the changed views.

## Done when

- The findings answer each doubt below, with evidence.
  - Does the spec meet each acceptance criterion of issue 2, as the owner amended it?
  - Is every rule that named `ravel::llm`, an LLM endpoint, Rig or an LLM budget gone or rewritten? The summary layers count too. They are the Solution, the tables of host capabilities, budgets and errors, and the notice. They are also the requirements, "Out of scope", the glossary and the design views.
  - A Codex call with structured output needs a schema file that the owner keeps. Is that workable, and does the spec say enough about it?
  - The harness-run budget defaults to 0 and allows 16. Do those values still fit, now that every LLM call is a harness run?
  - Can a harness use its sign-in inside confinement? The spec says that it depends on the harness and the platform. Is that statement honest enough, and does a test need it?
  - `ravel::json_check` throws `schema_invalid` and does not end the evaluation. Does that weaken a requirement?
  - The Credentials section now says that Ravel trusts the owner policy. Does another part of the spec contradict that, such as the leak probes or the hostile classes?
  - Does each claim about Claude Code and Codex match the cited documentation and the `--help` output of the installed versions?
- If findings are worth acting on, this session cuts the `integrate-review-design` leaf.

## Notes

- Make no real call to a model, and never read, print or record a credential.

## Decisions (running log)

**Review boundary and evidence.** The reviewed producer is
`cbfeff303be50fa1abf231c66991e970ef8c5fe4`, immediately before this session's
empty change. The owner's amendment is issue 2 comment
<https://github.com/Linkuistics/Ravel/issues/2#issuecomment-6397905744>.
This is an inspection-only review. No test, build, lint, format or real model
call runs, and no credential is read. The repository has `Taskfile.yml`.
The graph CLI refused `list_projects` because a pre-coordination or unverified
generation is active. Source checks therefore use exact file reads and text
searches; no graph generation or index-coverage claim is available.

**The host seam and the harness flags fit the amendment.** The installed
versions are Claude Code 2.1.310 and Codex CLI 0.164.0, as the producer recorded.
Their help and the fetched official documentation support the stated
noninteractive and structured-output flags. A static schema file is workable
for Codex. The owner must keep it in the subset that `json_check` accepts and
pass a path that resolves to that file from the run's working directory.

**A credential requirement contradicts the accepted trust boundary.** The
Credentials section explicitly permits a harness to return an ungranted caller
variable in its output. The normative requirement instead prohibits every host
function from returning one. This is a contract correction, not a proposal to
reopen the owner's decision to trust the policy.

**The harness-run budget remains an accepted owner choice.** The original
design log records the owner's choice to disable harness runs until settings
enable them. Moving LLM calls to that primitive makes the opt-in apply to LLM
calls too. A default of 0 and a maximum of 16 still permit the amended use;
there is no evidence that the maximum needs changing. Examples and smoke
tasks must set a positive budget.

**The output-limit contract needs one meaning.** The new harness-output
scenario returns `output_limit` to the script, as the run result table and the
budget table require. The Solution and Resource exhaustion requirement instead
say that every output-bound breach ends the evaluation. The design views repeat
both meanings. This needs a correction before a test can choose its expected
outcome.

**The re-entry escape clause overstates the outer bounds.** The spec says that
a run can clear the marker and that the budget and deadline still bound that
work. The new ADR and rejected-nesting view explain why an inner managed process
in another group can survive the outer stop. The cooperative marker is the
owner's chosen rule; the review will report the false guarantee without asking
for another recursion design.

**The review has an adjacent integration.** The findings and all eight named
doubts are recorded below. `llm-calls-through-harnesses-k61` integrates this
review before `llm-harness-replan-k60`. That session owns triage and any design
corrections. This review changes no design, production or test source.

## Findings

### F1 — P2: The credential requirement prohibits an explicitly permitted result

**Location:** `docs/specs/policy-evaluation.md:1106–1108`, "Requirement:
Credential boundary", against `:628–653`, "Credentials". The Solution at
`:36–38` also says that the worker holds no credential.

**Counterexample:** An unconfined harness prints the value of the caller's
`ANTHROPIC_API_KEY`, which the owner has not granted to `ravel::env`.
`run_harness` returns that text to the worker. Lines 631–633 and 651–653
explicitly permit this. The new SHALL statement instead prohibits *every host
function* from returning a caller variable outside the readable names and
grants. Both contracts cannot hold for this run. The permitted measured read
of a sign-in file at lines 642–647 also contradicts the unconditional claim
that the worker holds no credential.

The leak probe at lines 1112–1116 narrows the old test by excluding harness
runs, but it still permits file reads and says that the policy prints every
reachable value. On Linux, a measured read of the CLI's `/proc` environment is
the very exposure that the Credentials section accepts. The probe also does
not state that the key is ungranted, although line 483 permits an explicit grant.

**Consequence:** An implementer following the SHALL statement could add
redaction or block harness output, reversing the owner's trust decision. An
implementer following the Credentials section cannot satisfy that statement
or the unqualified leak probe.

**Smallest useful correction:** Scope the environment restriction to the
`env` accessor. Describe the absence of Ravel-owned model authentication
separately from credential bytes that a trusted policy deliberately obtains.
Make the probe about an ungranted `env` lookup, without other credential
sources, and reconcile the Solution and "Out of scope" wording at line 1195.
Keep the accepted file-read and harness-output behavior. Use dummy sentinel
values at the existing CLI seam to distinguish denied access through `env`
from permitted data returned by a harness; no real credential is needed.

### F2 — P2: Output exhaustion has incompatible evaluation outcomes

**Location:** `docs/specs/policy-evaluation.md:51–54`, "Solution", and
`:1066–1073`, "Requirement: Resource exhaustion", against `:681–682` and
`:1030–1034`. The caption at `docs/design/rhai-policy/diagrams.json:93`
repeats the unconditional rule, while `outcomes.mmd:7` returns `output_limit`
as data.

**Counterexample:** Give an evaluation two harness runs. The first fake harness
writes 2 MiB to stdout, is stopped and cleaned up, and the script then selects
a deterministic route. The newly added Output bound scenario requires
`run_harness` to return `output_limit`; its result table and budget table agree.
The script can then continue. The Resource exhaustion requirement instead says
that *each output bound* ends the evaluation and cannot be caught, and the
Solution says that every bound breach ends it.

This inconsistency existed in the general bounds summary before the change.
It is material to this design because all LLM answers now use harness output,
and the producer added an explicit acceptance case for the recoverable result.
It also affects per-run deadlines: `timed_out` is data when the whole deadline
has not passed, rather than an evaluation stop.

**Consequence:** A test suite or implementation derived from the summary would
terminate a policy that the detailed contract allows to recover. A suite
derived from the new scenario would violate the SHALL statement. The views
offer both expectations to the owner.

**Smallest useful correction:** Distinguish limits that stop one harness run
from limits that end the evaluation. Keep that distinction consistent in the
Solution, Resource exhaustion requirement and design captions. State the
continuation outcome in the output-limit scenario and cover it at the same
CLI seam. This does not require another host interface or a token budget.

### F3 — P2: The re-entry escape clause promises a bound the runner cannot enforce

**Location:** `docs/specs/policy-evaluation.md:808–813`, "Re-entry and
completion authority", against
`docs/adr/a-policy-asks-a-model-through-a-harness-run.md:41–45` and
`docs/design/rhai-policy/rejected-nested-session.mmd:10–19`.

**Counterexample:** An unconfined harness run starts a second Ravel with
`RAVEL_POLICY_RUN` removed from its environment. The second Ravel admits a
policy and launches a managed process in a separate process group. The outer
budget counts only the original harness run. Its deadline stops that group's
processes; it does not reach the second managed group. The inner supervisor's
cleanup can still be in progress when the outer kill-grace expires. The new ADR
and rejected-alternative view identify exactly this survival path. A 1.x
Ravel that ignores the marker has the same limitation.

The spec nevertheless says that after an escape "the budget and the deadline
still bound the work". They bound the directly supervised run, not every
invocation or process that the escaped selection can create.

**Consequence:** The limitation that motivated rejection of nested sessions is
presented as safely bounded in the active interface contract. A caller can
infer that all local model work has ended after the outer timeout when the
documented process-group scope cannot establish that.

**Smallest useful correction:** Preserve the selected no-re-entry design and
its marker check. State that it is a cooperative rule, that stripping the
marker or using an older binary escapes it, and that the outer budget and
deadline guarantee extends only to the supervised job's cleanup scope. Qualify
the "every process" wording in the cancellation requirement at lines
1038–1040 by the same scope. Reconcile the rejected view and the active prose;
no new recursion mechanism is requested.

## Coverage against amended issue 2

These are assessments of the design at the producer commit, not claims that
the unimplemented harness capability or smoke tasks have passed.

| Acceptance criterion | Design evidence and assessment |
|---|---|
| 1. Deterministic Rhai selection | Policy module and selection contract, lines 132–200, and Route table scenario, 915–924. Supported; no harness entry or run is required. |
| 2. Generic LLM interaction through a configured harness | `run_harness`, 508–615; policy-owned arguments, parsing, interpretation and retries; selection and context-summary scenarios, 926–957. Supported. The host adds no selection oracle or harness-specific parser. |
| 3. Bounded, neutral discovery and context provenance | Measured reads, 375–399; discovery, 488–506; limits, 670–702. Configured labels replace endpoint model lists. No live model discovery or recommendation is promised. Supported. |
| 4. Direct calls, prohibited re-entry and completion separation | 799–824 and scenarios 975–990. Ordinary inherited-marker refusal and separate channels are specified. F3 corrects the overstated guarantee after the marker is bypassed. The 1.x session spec already describes inherited context as isolation rather than authentication against same-user code. |
| 5. Typed conversions, policy and execution failures | Conversion, 202–221; host and run contracts, 350–373 and 508–558; data helpers, 401–458; errors, 768–797; scenarios, 945–957 and 992–1034. Harness authentication, network, rate-limit and unsupported-option failures are exit status/output that the policy interprets, as the amendment permits. Final selection validation remains mandatory. F2 leaves output-limit continuation ambiguous. |
| 6. Deadlines, cancellation, exhaustion, cleanup and credential/log boundaries | Bridge, 297–348; budgets and stop, 670–741; diagnostics, 743–766; scenarios, 1036–1122. The supervised scope and credential wording need F1–F3. The kernel-read limitation is explicit at 1164–1168. |
| 7. Credential-free fakes and opt-in real harness smoke tests | Test seams, 1147–1162; Upgrades, 900–911. One sandbox-home CLI seam uses real workers/runners and fake harness scripts. Separate opt-in Claude Code and Codex calls use the owner's sign-in, unconfined. No network fixture or Rig cassette remains required. |
| 8. Equivalent migration and matching delivery documentation | Entry precedence, 94–130; delivery, 826–845; migration, 847–898; scenarios, 1124–1145. Supported as a release obligation. The following planning leaf owns reconciliation of the remaining live work orders. |

The required architecture still leaves branching, prompts, interpretation,
retries and fallback choices in the policy. Ravel owns the worker, process
creation, limits, cancellation and result-channel lifetime. The synchronous
worker/private-socket bridge keeps the CLI available for I/O and stopping; the
design names the abandonable reader thread for filesystem calls. No additional
test seam or public adapter is needed by this change. Removing the LLM adapter
concentrates execution in the existing runner; parsing two harness output
formats moves to owner policy and is a visible accepted cost.

The remaining design questions have answers: fixed engine/heap limits and
composed run/whole deadlines; explicit JSON conversions and run result fields;
policy-owned failure interpretation; clean-cutover entry rules and migration;
no streaming or Ravel token accounting. Rig API, wire mapping and transport
retry questions have no subject after the owner's amendment. Harness-internal
retries are part of the bounded run, rather than extra host invocations.

## Disposition of the task's named doubts

| Doubt | Assessment |
|---|---|
| Meet the amended issue | The matrix above identifies the supported contracts and the wording corrections needed before implementation. The owner's removal of Rig and choice against re-entry are accepted. |
| Remove or rewrite the old rules, including summaries | The whole policy-evaluation spec, glossary, current ADR set, diagram manifest and Mermaid sources were read. The host table, request limits, `remaining`, budget/error tables, notice fields, requirements and scope now describe harness calls. Residual old terms in the current design identify removed dependencies or rejected alternatives; they do not authorise endpoints. F1–F3 identify remaining incompatible summary and requirement clauses. |
| Owner-kept Codex schema file | Workable unconfined. The run's working directory is `request.cwd` (529), so an owner can pass an absolute schema path and read that path through `read_json`. A schema using only the documented subset can be supplied to Codex and checked locally. The file must remain the same between the policy read and Codex's read; the design provides no atomic snapshot across those reads. This is an owner-file stability assumption. Under confinement a schema beside the policy cannot receive a runtime-read grant, because grants cannot overlap the protected policy directory. A confined recipe would need a schema outside that directory and an explicit grant or an admitted working-directory path, in addition to workable sign-in access. |
| Default 0 and maximum 16 runs | Accepted. The owner chose opt-in in `rhai-policy-k2`, lines 77–80. LLM examples and smoke fixtures must configure a positive budget. Sequential calls and fallbacks fit within 16; no stated requirement needs more. A run counts when launched, not once per internal model request. |
| Sign-in inside confinement | The caveat is honest as a lack of guarantee, and the smoke tasks deliberately prove only unconfined use. `HOME` does not itself grant reads of the sign-in store: the existing confinement contract denies other file contents and only admits explicit runtime files, writable roots and platform reads. Even an admitted store may need a write for refresh. The design promises no wider grant or unconfined fallback. Keep the confinement tests using dummy store files and an explicit read grant; real sign-in testing is opt-in. No claim that a real confined sign-in works is established here. |
| Catchable `schema_invalid` | Supported. It is validation data handled by a pure helper, like `json_parse`, rather than an authority-bearing host request. A catch can recover, but final selection validation and engine/heap/deadline limits remain mandatory. Neither the amendment nor the surviving requirements requires an invalid schema to terminate evaluation. The `rhai-llm-calls` brief explicitly requires the unsupported-schema case at the CLI seam. |
| Trusted policy versus leak and hostile probes | Trust does not remove resource limits, disabled `eval`, worker identity checks or diagnostic/result separation. Those protect against runaway behavior and untrusted data. The normative credential restriction and leak probe do need F1; intentional credential access is already accepted. The retained Bun hostile-runtime evidence describes the 1.x worker and explicitly disclaims a sandbox for admitted policy code. It is not evidence for a stronger Rhai credential guarantee. |
| Harness documentation and installed help | The recorded versions and all four rows of the harness fact table are supported by the inspected help and fetched official pages. The table distinguishes Claude's JSON document and `structured_output` from Codex's JSONL events and schema-file argument. Claude's `--bare` excludes the subscription sign-in. No real run was made. |

## ADR set, evidence and limits

The new ADR replaces the endpoint-settings ADR rather than adding a live
superseding record. It owns the sign-in versus process-cost trade-off and names
the rejected alternatives and reconsideration conditions. The worker ADR owns
isolation and pre-launch ordering; the Rhai ADR owns language and migration;
the session and job ADRs own supervision and cleanup. Their responsibilities
remain distinct, and no further ADR is supported by this review. Durable
citations in the inspected design point to the replacement. Old task records
describe the old endpoint design as historical context.

External sources were fetched on 2026-10-26 local time:

- [Claude Code programmatic runs](https://code.claude.com/docs/en/headless):
  noninteractive behavior, output fields, missing structured output and bare
  mode authentication. Installed `claude --help` independently exposes `-p`,
  JSON output, inline schema and the bare-mode authentication restriction.
- [Claude Code credential rules](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use):
  the distinction between credential intermediation and an end user's sign-in
  to an unmodified Claude Code binary. This review accepts the owner's choice
  of that route and makes no broader legal determination.
- [Codex noninteractive mode](https://learn.chatgpt.com/docs/non-interactive-mode):
  final stdout, JSONL events and usage, schema files and reuse of saved auth.
  Installed `codex exec --help` independently exposes the JSONL and schema-file
  flags and stdin behavior.
- [Codex authentication](https://learn.chatgpt.com/docs/auth): saved CLI auth,
  possible file or OS credential storage, refresh during use and Platform API
  keys for general API calls.

Source inspection also covered the existing confinement contract in
`docs/specs/session-execution.md:188–196`,
`docs/book/src/09-jobs-signals-and-the-terminal.md:272–310` and the grant
construction in `crates/keyed-launch/src/confinement.rs:238–320` and
`crates/ravel/src/confinement.rs:94–122`. These grant listed roots/files
rather than the whole home directory and reject overlap with the policy
directory. These reads support the stated limitation; they do not prove any
particular harness can authenticate in a sandbox.

The old-token search was bounded to the current policy spec, glossary, ADRs,
views and the two active brief-chain documents. Its positive matches include
the replacement ADR's rejected endpoint alternative and the removal brief's
work order. Each was read in context. The viewer obtains its prose from
`diagrams.json`; no rendered view or product behavior was tested. Later live
briefs are deliberately assigned to `llm-harness-replan-k60`, which must search
them again. No repository-wide absence or runtime-conformance claim is made.

All findings are document-derived counterexamples. No reproduction, test,
build, lint or format command ran. The real harness-run implementation and
post-fix verification belong to later sessions.
