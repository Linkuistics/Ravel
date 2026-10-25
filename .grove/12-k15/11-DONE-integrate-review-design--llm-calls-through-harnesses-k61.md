# llm-calls-through-harnesses-k61

**Integrates:** llm-calls-through-harnesses-k59

## Goal

Triage the design review and integrate supported corrections before planning
consumes the design of LLM calls through harnesses.

## Context

- Find the review commit by its stable handle,
  `llm-calls-through-harnesses-k59`, and read its findings, coverage assessment,
  source evidence and verification limits.
- The reviewed producer is `llm-calls-through-harnesses-k58`. Its running log
  and the owner's amendment to issue 2 remain the contract.
- The design is `docs/specs/policy-evaluation.md`, the current ADR set,
  `CONTEXT.md` and `docs/design/rhai-policy/`.
- `llm-harness-replan-k60` follows this integration and consumes the settled
  design.

## Done when

- Every review finding has a reasoned disposition.
- Supported corrections are integrated, and the spec, ADRs, glossary and views
  give planning one coherent contract.
- Verification evidence and its limits are recorded under the integration
  skill's procedure.

## Notes

The review is evidence to triage, not a prescribed fix list. Read its committed
artifact through the handle above. Its conclusions do not become requirements
merely because this task integrates them. Preserve the owner's settled choices.

## Decisions (running log)

**Review boundary.** The findings are in the commit of
`llm-calls-through-harnesses-k59`, `2374855`. The design files are unchanged
since the reviewed producer `cbfeff3`, so the line numbers of the review hold.
Each finding was checked against the text of the spec, the ADRs, the glossary
and the views. No owner decision is reopened.

**F1 is a contract stated unclearly.** The SHALL statement of "Credential
boundary" forbids every host function to return a caller variable. Its own
scenario "A harness finds its own sign-in" requires `run_harness` to return
one, and "Credentials" accepts that exposure. On Linux a measured read of
`/proc` also returns one. The leak probe says "every value that it can reach",
which includes such a read. The probe also does not say that no grant names the
key.

The Solution says that the worker holds no credential, but a trusted policy
can receive one. "Out of scope" still lists "a credential in a script", a
residue of the endpoint design. The fix scopes the restriction to `ravel::env`
and keeps the trust decision. Both scenarios now use one ungranted sentinel.

**F2 is a contract stated unclearly.** The Solution and "Resource exhaustion"
say that every bound breach ends the evaluation. The run result table, the
budget table and the scenario "Output bound" return `timed_out` and
`output_limit` to the script. The fix names the deadline and the output bound
of one harness run as the two bounds that stop only the run.

The Solution, the host table, the composition rules, both requirements and the
views now agree. A new scenario covers the deadline of one run, and "Output
bound" states that the policy goes on. The spec also says that the run status
`output_limit` differs from the refusal `output_limit` of the diagnostics bound.

**F3 is a contract stated unclearly.** The escape clause says that the budget and
the deadline still bound the work after a run escapes the marker. They bound the
process group of the run only. The managed process of an escaped
`start-session` is a job in a group of its own. The ADR
`the-launched-child-is-a-job` already puts a process that leaves its group
outside group cleanup.

The fix states that the refusal is cooperative and names the scope of the outer
stop. It also scopes "reap every process" and renames "Deadlines and
cancellation stop everything", which no live brief cites. The re-entry design
stays as the owner chose it.

**The integration found a residue that the review missed.** `outcomes.mmd`
replied with the statuses `ok` and `failed`, which belonged to `ravel::llm`. No
host function has them now. The view now replies with a value, a thrown map or a
run status.

**The Codex schema note of the review's doubt table is a contract stated
unclearly.** The spec gives "beside the policy" as the place of a schema file.
The confinement contract refuses a grant that overlaps the policy directory, as
`session-execution` and the confinement code state. The spec now says that only
an unconfined run can read a schema file there. The review also notes that the
owner must not change the file between the two reads. That is an accepted
trade-off of an owner-kept file, and the spec does not state it.

**The other named doubts stand as the review assessed them.** The harness-run
budget of 0 with a maximum of 16 is the owner's choice. The catchable
`schema_invalid` weakens no requirement. The harness facts match the cited
documentation and the installed help. The caveat about a sign-in inside
confinement is honest, and the smoke tasks run unconfined. No ADR changes,
because no finding reaches a recorded decision.

**The planning leaf receives a pointer.** `llm-harness-replan-k60` now names
the new and changed scenarios, so that it gives each one a live leaf.

**Verification and its limits.** A sweep looked for each overclaim of the
findings. It read the spec, the glossary, the five ADRs, the manifest, the views
README and the eight Mermaid sources. It came back clean on the tree. The same sweep
against the producer `cbfeff3` found every old phrase, which proves that the
search can see them.

A listing of every sentence about a breach, a credential, a
caller variable or an escape was then read line by line. That reading found two
false summaries of this session, and the session fixed them.

The prose checker reports no hard finding in the spec, the views README and
this file. `diagrams.json` loads as JSON, and the viewer accepts each id that it
names. Mermaid 12.0.0 in the viewer parsed both edited sources. A broken control
source failed the same parse. No test, build or real model call ran, because
this session changes only the design. The new scenarios are untested until the
leaves that planning cuts implement them.
