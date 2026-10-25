# rhai-policy-k4

**Reviews:** rhai-policy-k2

## Goal

Read the Ravel 2.0.0 policy design adversarially, and report findings without
fixes. The design is the spec `docs/specs/policy-evaluation.md`, the ADR set in
`docs/adr/`, the glossary changes in `CONTEXT.md` and the views in
`docs/design/rhai-policy/`.

## Context

- Find the commit of `rhai-policy-k2` by its handle, and read its diff.
- GitHub issue 2 binds through its "Required architecture" and "Acceptance
  criteria". Its "Design questions to resolve" are what the design must answer.
- The running log of `plan-k1` holds the requirements that the owner settled.
  The running log of `rhai-policy-k2` holds the design decisions, and the owner
  chose three of them directly.
- The spec names Rhai 1.27.0 and Rig 0.45.0. Check a claim against the crate
  source from crates.io at that version, not against memory.

## Done when

- Each finding names the artifact, the section and the defect, with evidence.
- The review checks each design question and each acceptance criterion of issue
  2 against the spec or an ADR.

## Notes

These doubts are the producer's own. Read each one against the source.

- The spec claims that killing the worker never orphans host work. Check every
  host capability, module reads included, for a path that a kill leaves open.
- On macOS the worker re-executes the canonical path of the binary. Check
  whether a Homebrew upgrade during a session can break that path or replace
  the build. Then check that the identity check covers each case.
- Check that the engine's strict property mode and optional fields that hold
  `()` work together in Rhai 1.27.0.
- The schema subset may not meet the strict rules of a provider, such as
  required properties and `additionalProperties: false`. Decide whether such a
  call fails clearly.
- The routing notice moves to schema version 2. Decide whether that contradicts
  "callers do not change", or whether no caller reads the notice.
- Check the token accounting when an endpoint reports no usage. Also check how
  the deadline of a call composes with the whole deadline.
- `json_encode` sorts keys, but Grove's provider receipt has a fixed member
  order. Check that the spec gives Grove's port a workable path.
- The ADR set should be a minimum coherent set. Check that no record restates
  the spec, and that every citation resolves.

## Decisions (running log)

**Review evidence.** The reviewed producer is commit
`ca9486be72a28a997b4152dbec9298f5c1eb7c93`. Its predecessor's glossary conversion
is `01654277d4f6`. The checkout started clean. This is an inspection-only review:
no test, build, lint, format or product command runs. The repository has a
`Taskfile.yml`. The graph CLI refused `list_projects` because a pre-coordination
or unverified generation is active, so source discovery and verification use
direct file reads. No graph generation or coverage claim is available.

**Two authority and resource findings are established.** The specification's
text-output bound applies after Rig's unbounded response collection in the CLI.
Its credential exclusion also covers settings grants but misses command-line
grants and the automatically readable environment names. Both need design
corrections before implementation planning.

**The cancellation contract needs two corrections.** Moving file reads into the
CLI leaves their cancellation and shutdown ownership unspecified. A synchronous
read stalls the sole runtime thread; an ordinary Tokio file read instead leaves
blocking work that cannot be aborted. Separately, latching a deadline ahead of a
later signal contradicts the existing rule that cancellation wins during
settlement. These are findings about the proposed guarantees, not claims that an
implementation of the new design has failed.

**The review is ready for integration.** Each required architecture item, design
question and acceptance criterion of issue 2 has a recorded design assessment.
The named doubts have source-based dispositions. F1–F4 warrant an
`integrate-review-design` leaf immediately before planning. That session owns
triage; this review changes no design or implementation artifact.

## Findings

### F1 — P1: File reads have no cancellation mechanism compatible with the bridge

**Artifact and sections:** `docs/specs/policy-evaluation.md:245`, "The bridge
between the script and the CLI"; `:290`, "Measured reads"; `:103`, module
imports; and `:606`, "Cancellation and deadlines".

The CLI now performs every file read, including module loading. The bridge
promises that its one runtime thread keeps watching cancellation, that all
runtime tasks finish before launch, and that teardown has a finite bound. The
stop procedure accounts for HTTP and harness work but never for a read in
progress. A regular-file check and a byte limit do not make an open, metadata,
canonicalisation or read operation interruptible. A stalled filesystem can
therefore keep an evaluation alive after its worker is killed.

Doing this work with `std::fs` on the current-thread runtime prevents the timer
and signal work from running. Merely substituting `tokio::fs` does not discharge
the obligation: Tokio uses blocking tasks, which continue after their future is
dropped, and runtime destruction waits for them. A shutdown timeout leaves the
work alive instead. The pinned Rhai resolver cannot solve this because the CLI,
not Rhai, owns the actual read.

**Evidence:** Tokio 1.52.1 source, `src/fs/mod.rs:9`,
`src/task/blocking.rs:107` and `src/runtime/runtime.rs:30`, explicitly documents
these properties. The existing native contract at
`docs/specs/result-job-completion.md:330` distinguishes bounded polling from
kernel stalls and permits specified shielded reader residue; the new spec
neither carries that qualification into this path nor chooses another owner.

**Smallest useful correction:** Specify the read execution and stop mechanism,
including imports, descriptor ownership, bounded observation and what may
remain at CLI exit. Reconcile the "all tasks finished" promise with that choice
and the existing native contract. Add a CLI-seam discriminator for cancellation
while a host read is outstanding. Do not claim that dropping a blocking future
stops the underlying operation.

### F2 — P1: The supervisor buffers unbounded HTTP bodies before the output check

**Artifact and sections:** `docs/specs/policy-evaluation.md:360`, "LLM calls";
`:544`, the HTTP client rules; and `:568`, "Evaluation budgets and their
composition".

The only answer-size limit is 1 MiB of decoded text. There is no response-body
limit before Rig decodes the response, nor a limit for error bodies or ignored
metadata. In `rig-reqwest` 0.45.0, `into_response` collects a successful body with
`response.bytes().await`. `non_success_status_error` collects an error body with
`response.text().await`. Both collections are unbounded. A compatible endpoint
can return an arbitrarily large JSON field or error page, with little or no
answer text. The CLI can exhaust memory before it applies the text bound. The
worker's counting allocator protects a different process.

**Evidence:** `rig-reqwest` 0.45.0, `src/lib.rs:194–228`; the spec puts this
client in the CLI and bounds only the worker heap. Rig's later decoding also
retains the raw provider document in `CompletionResponse.raw`
(`rig-core` 0.45.0, `src/completion/request.rs:309`).

**Smallest useful correction:** Put a bounded transport adapter at the existing
LLM module boundary. Specify limits for successful and failed HTTP bodies,
including chunked bodies without a trustworthy Content-Length, before buffering
or JSON decoding. Define the resulting call failure and require fake-endpoint
cases with oversized success metadata and error bodies. Keep the independent
decoded-text limit.

### F3 — P1: Command-line environment grants bypass the endpoint credential exclusion

**Artifact and sections:** `docs/specs/policy-evaluation.md:333`, "Data helpers";
`:526`, credential rules; and `:962`, "Credential boundary".

`env` explicitly reads names granted by either `policyEnv` or `--policy-env`,
plus the base names. The credential rule rejects only a collision with a
`policyEnv` settings grant. With an endpoint using `apiKeyEnv: "MODEL_KEY"`, the
otherwise valid invocation `--policy-env MODEL_KEY` therefore lets
`ravel::env("MODEL_KEY")` return the credential to the worker. This violates
the promised exclusion and makes the leak-probe scenario fail. The same conflict
exists if an endpoint names a base variable such as `LC_MODEL_KEY`.

**Evidence:** The two spec rules above admit this path directly. Existing
`crates/ravel/src/environment.rs:22–36` merges settings grants and flag grants;
`:76–78` defines the automatically readable names. Neither path is covered by
the new settings-only exclusion.

**Smallest useful correction:** Give endpoint credential names precedence over
every policy-readable environment path. Define whether a flag collision
refuses or `env` denies it; reject credential names that conflict with mandatory
base reads if those reads must stay available. Cover settings, flags and base
names separately at the agreed CLI seam.

### F4 — P2: A deadline latch suppresses cancellation received during cleanup

**Artifact and sections:** `docs/specs/policy-evaluation.md:606`, "Cancellation
and deadlines"; and
`docs/design/rhai-policy/stopping-sequence.mmd:8`.

The stop procedure latches either the deadline or a signal and says a later
signal does not replace the cause. If the deadline passes and TERM arrives
while a harness group is being stopped, this rule returns `selection_timeout`
and 124 instead of propagating TERM. The existing completion contract requires
applicable cancellation to win over an earlier operational failure, including
a selection timeout; only an already consumed cancellation is protected from
replacement by another signal. The new wording conflates these two rules.

**Evidence:** `docs/specs/result-job-completion.md:21–25`, `:126–130` and
`:209–215` require cancellation precedence. Existing
`crates/ravel/src/worker.rs:643–650` checks cancellation after teardown even
when the conversation already failed.

**Smallest useful correction:** Preserve the initiating timeout as diagnostic
evidence, preserve the first consumed signal among signals, and perform the
existing final cancellation join after cleanup. Update the sequence view and
include the timeout-then-signal interleaving in the CLI-seam plan.

## Coverage against issue 2

The obligation is [GitHub issue 2](https://github.com/Linkuistics/Ravel/issues/2),
read with `gh issue view 2 --repo Linkuistics/Ravel`, together with the settled
requirements in `plan-k1`. "Specified" below means that the design answers the
obligation. It does not mean that the unimplemented 2.0.0 behaviour has passed
tests. Coordinates are in `docs/specs/policy-evaluation.md` at the reviewed
producer commit.

| Required architecture | Design coverage | Assessment |
|---|---|---|
| Embedded Rhai; deterministic routes; script-owned decisions and caller meanings | Policy module, :117; request and selection, :142; deterministic scenario, :812 | Specified. The CLI validates data and executes effects; it makes no semantic route or fallback choice. |
| Generic LLM access, neutral bounded discovery/context, separate named harness invocation | Host capabilities, :264; discovery, :338; LLM calls, :358; harness runs, :448 | Specified interfaces. F1 and F2 leave execution bounds incomplete. |
| Host owns clients, credentials, runtime, processes, limits, logs and cancellation | Worker and bridge, :205–262; credentials, :500; budgets and cancellation, :568–630 | Partial: F1–F4 affect these guarantees. |
| Direct invocation, re-entry rules, invocation budgets, separate completion authority | Harness runs, :448; budgets, :568; re-entry and completion, :687 | Specified. LLM calls use HTTP and harness runs use the runner; neither invokes selection. The environment-clearing escape and group-cleanup limits are disclosed. |

| Design question | Answer in the design | Assessment |
|---|---|---|
| Synchronous Rhai with asynchronous work and isolation | Separate worker, synchronous socket bridge, CLI current-thread runtime, exact Rhai/Rig pins, :54 and :205–262 | The process decision has source support. F1 leaves blocking host work without a complete lifecycle. |
| Budgets, cancellation, memory/output, concurrency, tokens and composition | Fixed engine limits and counting allocator, :219; budget table and debit rules, :568; ordered stop, :606 | Defaults and composition are explicit. F1/F2/F4 require corrections. Remote work already accepted by an endpoint is explicitly outside local cancellation. |
| Typed boundaries, history, structured output, usage, status, final validation | Request/selection and conversions, :142–203; LLM request/response, :358–446; harness request/result, :448–498 | Specified. Scripts retain history; no streaming, tools or harness session-handle interpretation. Rig's schema sanitation is discussed below. |
| Defaults, abstention, refusal and distinct failures, with no fallback | :169–183; capability failure tables, :264–286 and :394–420; errors, :655 | Specified. Operational outcomes are script data; breaches terminate evaluation. F4 concerns final failure precedence. |
| Entry/version/discovery, migration mapping, cutover and delivery | :82–140; installation, :710; migration, :731; upgrade checks, :783 | Specified. Main temporarily supports both paths, but only the final 2.0.0 release ships Rhai. |

| Acceptance criterion | Design coverage | Assessment |
|---|---|---|
| Deterministic harness/model/effort selection without an LLM | Route-table requirement and scenario, :812–821 | Specified, with no endpoint required. |
| Generic Rig call, script-built messages and interpretation, validated selection/refusal, no harness required | :358–446 and :823–839 | Specified; no specialised selection API. |
| Bounded, neutral discovery/context and retained caller provenance | :290–356 and :841–849 | Data shapes and measurements are specified; read lifecycle remains F1. |
| Tests prove bypass, rejected re-entry and separate completion authority | :687–708 and :851–866 | Scenarios are specified at the real CLI seam. The marker is cooperative, as disclosed; it is not an authentication boundary against same-user code. |
| Typed conversions, missing/unknown/invalid data, defaults, refusals and failures | :142–203, :394–420, :483–498, :655–685 and :868–923; fake endpoint cases, :792 | Planned coverage exists for these classes. This review did not execute it. |
| Deadlines/cancellation, resource bounds, descendants, credentials and logs/results | :568–653 and :925–973 | Partial: F1–F4 identify gaps in the promised behaviour or its discriminators. |
| Credential-free deterministic fakes and separate opt-in real-provider/real-harness smoke checks | Rig upgrades, :783–808; test seams, :998–1015 | Specified at the owner-agreed CLI seam, with a real Rig client and local endpoint. |
| Equivalent representative migrations and matching installation/documentation | Installed layout and migration, :710–781; migration equivalence, :975–985 | Specified: static, sample and owner-shaped policies, plus an LLM example; comparison fixtures freeze before the TypeScript path is removed. |

The internal LLM adapter owns the provider-specific mapping and presents one
checked request/response interface. That is an appropriate substitution point:
the local HTTP endpoint substitutes the remote service while the real adapter
runs. The worker socket serves a concrete isolation requirement. The design adds
no unsupported plug-in layer or host-owned selection policy.

## Disposition of the producer's named doubts

| Doubt | Inspection result |
|---|---|
| Killing the worker leaves no host work, including module reads | F1. Moving all effects into the CLI is insufficient without defining how blocking reads stop and settle. F2 separately concerns the CLI's memory. |
| macOS canonical re-execution during Homebrew changes | The spec explicitly refuses a missing image as `worker_missing` and another build as `worker_identity_mismatch` (:205–214). Removal can break re-execution, but the design promises refusal, not survival of arbitrary installation mutation. `plan-k1` separately requires retaining the 1.1.0 binary until the running driver restarts. No additional finding. |
| Strict absent-property errors with optional `()` values | Supported. In Rhai 1.27.0, `src/eval/indexing.rs:120–128` returns an existing property's value before testing the absent-property option. An explicitly present unit value therefore remains readable. `src/api/options.rs:243–272` exposes the required strict settings. |
| JSON Schema subset versus provider strict rules | Rig sanitises the schema before sending it: `rig-core` 0.45.0, `src/providers/internal/schema.rs:26–60`, makes every property required and inserts missing `additionalProperties: false`. OpenAI's `structured_output_schema` calls it and Responses sends `strict: true` (`src/providers/openai/mod.rs:71`, `src/providers/openai/responses_api/mod.rs:510–517`); Anthropic calls it too (`src/providers/anthropic/completion.rs:306–310`). Some locally admitted schemas, such as an explicit `additionalProperties: true`, can still be refused by a provider. A 400 follows the specified nonretryable `provider_error` path with its status/message; it does not select a fallback. The raw body remains inspectable in Rig's error type. No claim that the whole local subset is accepted by every remote provider is established. |
| Notice schema 2 versus unchanged callers | Grove's named caller path constructs public flags in `crates/grove-loop/src/session_command.rs:60–64` and reads only exact stdout bytes in `src/session_result.rs:80–108`. Stderr is displayed or captured as a transcript (`src/ravel_job.rs:215–228`). It does not depend on notice schema 1 along that path. The explicit flag/context/result compatibility commitment remains satisfied; JSON notice consumers must recognise the documented version change. |
| Missing token usage and call/whole deadlines | Rig retains missing counters as `Option<u64>` (`rig-core` 0.45.0, `src/completion/request.rs:679–702`), so the conservative effective-`maxTokens` debit at :596–597 is implementable. :593–595 distinguishes a recoverable per-call timeout from the whole deadline. F4 concerns a later signal during settlement. |
| Sorted JSON members versus Grove receipts | :316–318 explicitly leaves fixed member order to the script, which can concatenate encoded keys/values and omit a missing optional taskId. Grove's current helper validates exact `JSON.stringify` bytes (`assets/ravel-policy/provenance.ts:24–39`), so the port must preserve its string-escape convention too; Ravel's map encoding alone is not that receipt encoding. The permitted Rhai string operations allow the port to own those bytes. No new host receipt API is needed. |
| Minimal coherent ADR set and citations | The five records cited by the root brief own distinct decisions: language/cutover, evaluation isolation, endpoint authority, session supervision and job supervision. The first three point to the spec for interface detail. The rewritten worker record replaces the old Bun rationale in place; it does not leave a superseding record. The relative targets and heading anchors in these five ADRs resolve on inspection. No material ADR-set finding. |

The Rhai source also supports the process-isolation rationale: its
`src/packages/lang_core.rs:138–166` calls `std::thread::sleep`, and
`src/packages/string_more.rs:1376–1392` does not advance when padding is empty.
`src/types/error.rs:320–381` makes engine limits uncatchable;
`src/func/script.rs:416–420` passes system exceptions through function calls.
The named-function environment retains imported modules
(`src/func/script.rs:238–268` and `:365–378`), which supports the design's
avoidance of the reported post-script closure regression. These are source
observations, not newly measured conformance.

## Source provenance and verification limits

Pinned dependency sources came from the published crates.io archives. The
following SHA-256 values identify the inspected downloads; source coordinates
above refer to files inside them.

| Archive | SHA-256 |
|---|---|
| [rhai 1.27.0](https://static.crates.io/crates/rhai/rhai-1.27.0.crate) | `c3925b784c0bee6542637e425ed7511a63257bf11b7ecf51ffd845e2d24d9163` |
| [rig-core 0.45.0](https://static.crates.io/crates/rig-core/rig-core-0.45.0.crate) | `0d07ae8e85415d83f620eb4f519d03126227178ac67971b08368c087ce8f9052` |
| [rig-http 0.45.0](https://static.crates.io/crates/rig-http/rig-http-0.45.0.crate) | `817c761f9949151fd099ec78e2fbacf7174f24c066ced84c9d28807a6b9d4cee` |
| [rig-reqwest 0.45.0](https://static.crates.io/crates/rig-reqwest/rig-reqwest-0.45.0.crate) | `c9f13e51c8cbe597ab4ed79f4a1211dab83ac3d9ceaa3826d69b5c770274b9fa` |
| [rig-cassette 0.45.0](https://static.crates.io/crates/rig-cassette/rig-cassette-0.45.0.crate) | `ee092a5c4462f99f718b1c874f0a14f8b71598cb239044f6461c5db190f9933f` |

Tokio 1.52.1 was read from the local Cargo registry, rather than asserted as a
new design pin. Rig's `connect` accepts a custom `HttpClientExt`
(`rig-core`, `src/client/macros.rs:29–37`; `rig-http`,
`src/http_client/mod.rs:194–213`), so F2's proposed transport seam exists in
the pinned API. The [stable Rust channel manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml),
read during review, reports Rust 1.99.1, matching the design's chosen floor.

All design Markdown, the six Mermaid sources, their manifest and the viewer
source were read. The viewer was not rendered. No executable behaviour, archive
target, TLS build or personal configuration was exercised or changed. The
integration owns triage, every correction and any subsequent verification.
