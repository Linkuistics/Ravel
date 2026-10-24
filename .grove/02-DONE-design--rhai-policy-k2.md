# rhai-policy-k2

## Goal

Establish how Ravel 2.0.0 evaluates Rhai owner policies with Rig-backed LLM
access. Deliver a spec and a reworked ADR set that the owner can agree before
planning cuts the work.

## Context

- The "Design questions to resolve" in issue 2 are this session's work. Its
  "Required architecture" and "Acceptance criteria" bind as written.
- The running log of `plan-k1` holds what the owner settled. Synthesise the
  spec from it, and do not interview the owner again.
- This work contradicts the ADR `policy-evaluation-precedes-the-launch`.
  Rework the ADR set in place, as `ADR-FORMAT.md` directs.
- The owner's personal policy, `~/.config/ravel/policy.ts`, has the shape that
  the migration guide must translate. Read it, and leave it unchanged.
- Grove's helpers in `/Users/antony/Development/grove/assets/ravel-policy/`
  show why each capability in the running log exists.
- Chapters 4 and 6 of the book document today's owner-facing contract.

## Done when

- A spec in `docs/specs/` describes how Ravel evaluates a Rhai policy. It
  covers the host capabilities, the budgets, cancellation, the typed
  boundaries, the errors and the migration.
- The spec's "Test seams" section records the seams that the owner agreed.
- Each design question in issue 2 has an answer in the spec or in an ADR.
- The ADR set is a minimum coherent set for the new design.
- The spec names the pinned Rhai and Rig versions. This session verified each
  API that the design relies on against their source at those versions.
- `CONTEXT.md` gives "provider" one meaning, and it defines each new term.

## Notes

- If you cut a `review-design` leaf, insert it before the planning leaf with
  `grove-llm leaf-insert`. Planning must read a reviewed design.
- The isolation question needs particular care. Today a separate process
  evaluates the policy, and Ravel can kill that process at the deadline. Do not
  assume that a thread keeps the same guarantees. Issue 2 says this too.
- Rig released seven minor versions between June and October 2026. State in the
  design how the tests prove a Rig upgrade, because upgrades come often.
- The running log also fixes which policy file 2.0.0 uses when both exist. The
  local upgrade depends on that rule.

## Decisions (running log)

**Rust: the newest stable release, and edition 2024.** The owner said "we
should use latest possible rust". Rig 0.45.0 needs Rust 1.97.1 and edition
2024, and Ravel's floor is 1.85 with edition 2021. The design does not stop at
Rig's floor. The workspace `rust-version` becomes the newest stable Rust, which
is 1.99.1 (2026-10-13) at design time, and both crates move to edition 2024.
A later Rust release may raise the floor again through the same upgrade
checks.

**Isolation: a separate policy worker, which is Ravel's own binary.** The
owner chose this over a Rhai thread inside the Ravel CLI. Ravel re-executes its
own binary in an internal worker role. The worker only computes. The Ravel CLI
performs every host capability for it over the existing private socket: reads,
environment, Rig calls and harness runs. The CLI kills the worker at the
deadline or on cancellation.

The evidence is in the Rhai 1.27.0 source. `sleep` blocks the thread for a time
that the script chooses. `pad` with an empty padding string loops forever. The
unreleased 1.27.1 fixes that loop upstream. The size limits bound each value but
not the heap.

On a thread, memory exhaustion or a stack overflow aborts the supervisor
without cleanup. In a worker, they end only the worker.

**"Provider" keeps one meaning: the owner's model-origin label.** The owner
chose this. `Selected.provider`, the routing notice and Grove's receipts keep
the word. The owner-configured LLM service is an **LLM endpoint**: settings key
`endpoints`, request field `endpoint`. No caller-visible data changes.

**Harness runs are off until the owner enables them.** The owner chose this.
The harness-run budget defaults to 0. A policy can run only a harness that
settings name. Each run gets the session's confinement, so Ravel confines it
when the caller passes `--confine`.

**Pins: Rhai 1.27.0 and Rig 0.45.0.** Rhai 1.27.0 has the `compact_str`
feature, which keeps the unmaintained MPL-2.0 `smartstring` code out of the
build. It also counts operations inside native callbacks correctly. Its known
regression (rhaiscript/rhai#1193) loses imports in a closure that the host calls
after the script ends. Ravel calls named script functions and holds no
closures, so a test guards the case instead.

Ravel pins `rig-core`, `rig-http` and `rig-reqwest` exactly at 0.45.0.
`rig-cassette` 0.45.0 is a development dependency only. Ravel builds its own
`reqwest` 0.13 client.

**Entry and precedence.** The default entry is `~/.config/ravel/policy.rhai`.
When it exists, Ravel ignores `policy.ts`. When only `policy.ts` exists, Ravel
refuses with `policy_format_retired` and names the migration guide. The entry is
a Rhai module that exports `SCHEMA_VERSION = 3` and `VERSION`, and defines
`select(request, context)` and optionally `load_context(request)`.

**Configuration home: `settings.json`.** The owner's settings file gains three
keys: `endpoints`, `harnesses` and `budgets`. Credentials stay in the
environment. An endpoint names the variable, and only the Ravel CLI reads it.

**Outcomes of a host call are data, and breaches of a bound end the
evaluation.** A failed LLM call or harness run returns a result map that the
script interprets. Some events end the evaluation as an uncatchable Rhai error:
an exhausted budget, the deadline, cancellation, a source bound and a malformed
host request. A host latch refuses even if the script swallows the error.

**Network hygiene.** Ravel ignores ambient proxies, follows no redirects and
makes no transport retries. A base URL is HTTPS, or HTTP to a loopback address.

**Ravel checks structured output itself.** Rig 0.45.0 drops `output_schema`
with only a warning on a dialect without structured output. Ravel therefore
checks every answer against a documented schema subset. A schema sent
to an endpoint without the capability fails as `unsupported_capability`.

**Re-entry.** Each harness run carries a reserved marker variable.
`start-session` refuses under that marker with `policy_reentry_refused`.

**A review-design leaf precedes planning.** The owner chose this after reading
the views. The spec covers the whole 2.0.0 release, so a fresh-context reviewer
reads the spec and the ADR set before planning cuts the work.
