# rhai-policy-k5

**Integrates:** rhai-policy-k4

## Goal

Triage the design review and integrate the supported corrections into the
Ravel 2.0.0 policy design before planning consumes it.

## Context

- Find the review commit by its stable handle, `rhai-policy-k4`, and read its
  findings, coverage assessment, source evidence and verification limits.
- The reviewed producer is `rhai-policy-k2`. The requirements settled in
  `plan-k1` and issue 2 remain the contract.
- The design is `docs/specs/policy-evaluation.md`, the ADR set,
  `CONTEXT.md` and `docs/design/rhai-policy/`.

## Done when

- Every review finding has a reasoned disposition.
- Supported corrections are integrated, and the spec, ADRs, glossary and views
  give a coherent design for the planning leaf.
- Verification evidence and its limits are recorded under the integration
  skill's procedure.

## Notes

The review is evidence to triage, not a prescribed fix list. Read its committed
artifact through the handle above. Do not treat its conclusions as requirements.

## Decisions (running log)

**Evidence for the triage.** The findings come from the review commit
`e94fd132da16`. This session checked each cited source again, in Ravel at the
reviewed commit and in the pinned crates. The downloaded Rig archives have the
SHA-256 values that the review records. Tokio 1.52.1 came from the local Cargo
registry. No test, build or product command ran, because no code changes.

**F1 is a real issue, and the spec gets the mechanism.** A read moves off the
runtime thread to a reader thread. A stop does not wait for a read in progress.
On the path to a launch no read is outstanding, because the worker sends its
result only after the reply to its last host call. The rejected alternative is
reads in the worker. That reverses the decision that the CLI performs every host
capability, so it is a redesign.

**F1 has two stated limits.** Ravel bounds its own waiting, as the native
contract does, and not a call that the kernel holds. The agreed seam cannot make
the kernel hold a read. A nonblocking open already removes the stalls that a
test can make, which are a FIFO and a file lease. A FIFO case with a firing
control proves the open. No seam test proves that a stop never waits for a
read, and the spec says so without a second seam.

**F2 is a real issue, and the LLM adapter gets a bounded transport.** Ravel
implements the HTTP client interface of Rig on its own reqwest client. A
successful body has a limit of 8 MiB, and a body beyond it fails the call as
`output_limit`. Ravel reads at most 64 KiB of a failure body and keeps its
status, so a large 429 page stays `rate_limited`. The limit counts the bytes
as they arrive, whatever the declared length says.

**F3 is a real issue, and the credential names take precedence.** A
`--policy-env` flag that names a credential refuses as `excluded_grant`,
exit 2, as the settings grant does. An `apiKeyEnv` that names a base readable
name refuses as `settings_invalid`. Two more paths reach a credential: a
measured read of a process environment under `/proc` on Linux, and the output
of an unconfined harness run. Reads are not confined by path, so the spec now
limits the claim to what Ravel itself serves, and it states both paths. A
denylist under `/proc` was rejected, because files and other processes of the
same user stay readable.

**F4 is a real issue, and cancellation wins again.** The stop keeps its first
cause. After cleanup, Ravel joins cancellation as the completion contract
requires. A signal then wins over the deadline, and the first consumed signal
stays the signal. The report keeps the deadline as the original failure. The
stopping view and a new scenario show the interleaving.

**Two named-doubt observations become clarifications.** The spec now writes
`\u00xx` in lowercase. Then `json_encode` writes each Rhai string as
`JSON.stringify` writes it, and Grove's port can join encoded members. The
spec also states that Rig rewrites a schema before it sends it, and that an
endpoint can refuse a schema that the subset admits. The other doubts need no
change. The spec states schema 2 of the notice, and no named caller reads it.

**The CLI enforces the launch-path rule.** The F1 correction relies on the rule
that no host call is open when the worker sends its result. A message from the
worker during a host call now breaks the protocol, and the evaluation refuses
as `worker_failed`. The rule is therefore checked, not assumed.

**No further review follows this integration.** Each change applies a finding
that this session verified against source, and none rethinks the design. The
session spent no in-session reviewer. The planning leaf consumes the changed
spec, and the new scenarios bind the tests at the seam.

**Verification.** The controlled-English checker reports no hard finding in
each prose file that this session changed. Mermaid 12.0.0 parsed both edited
views, and a deliberately broken copy failed as a control. All six views
rendered in a browser. A sweep for the old wording found no stale copy outside
the finished review and the 1.x book. A positive control found the new wording.

**The limits of the verification.** No product code or test ran. The `/proc`
statement rests on Linux procfs behaviour, and this session did not run it on
Linux.
