# plan-k1

## Goal

Establish what GitHub issue 2 asks Ravel to build, in the owner's own words. The
issue replaces the TypeScript owner policy with embedded Rhai scripts. It also
gives those scripts generic LLM access through Rig.

This session settles the open requirements with the owner and records each
decision. It then hands the workstream to the next session.

## Context

- GitHub issue 2 is the owner's statement of the work. Read it with
  `gh issue view 2`.
- The issue inspected commit `20df884`. The main line moved on to v1.1.0. Each
  file that the issue cites still exists, except `docs/owner-configuration.md`.
  The book in `docs/book/` documents owner configuration now, chiefly in
  chapters 4 and 6.
- The ADR `policy-evaluation-precedes-the-launch` rejected an embedded smaller
  engine. It also keeps owner policy out of the process that supervises the
  session. The issue reopens both points.
- `CONTEXT.md` defines **Owner policy** as TypeScript configuration and
  **Policy worker** as a separate process. This work changes both definitions.
- Grove ships TypeScript policy helpers in its own repository, under
  `assets/ravel-policy/`. They import `ravel/sdk`. They use measured task-file
  reads, UUID generation, canonical JSON and a wrapper that launches through
  Bash.
- The owner's personal policy reads a jj workspace file through `node:fs`. That
  read is outside the measured context reads.
- Rig has no 1.0 release yet. Its newest stable release is 0.45.0, from
  2026-10-19. Its OpenAI and Anthropic clients accept a custom endpoint. Its
  `rig-cassette` crate records and replays provider HTTP traffic.

## Done when

- The running log records each requirement that the owner settled.
- The owner confirmed a shared understanding before the commit.
- The root brief states the goal and the completion conditions of the
  workstream.
- The next leaves exist, and each body names its input.

## Notes

- The issue's sections "Required architecture" and "Acceptance criteria" bind
  as the owner wrote them. Its "Design questions to resolve" belong to the
  design session. The migration question is the exception, and this session
  puts it to the owner.

## Decisions (running log)

**Migration mode: a clean cutover in Ravel 2.0.0.** No release runs both policy
formats. Main keeps working during the transition, with the TypeScript path
beside the new Rhai path. Only 2.0.0 ships Rhai, and the same release removes
Bun and TypeScript. A leftover `policy.ts` refuses with a remedy that names the
migration guide. Main makes no release until 2.0.0, which waits until Rhai
covers what the known policies do.

The owner chose this over a compatibility period in 1.x releases. The known
TypeScript policies all belong to the owner, and coexistence would keep the
costs of the Bun worker.

**Grove's helpers: a Grove issue, with generic parity in Ravel.** A GitHub issue
in `Linkuistics/grove` carries the port of `assets/ravel-policy/` to Rhai. Ravel
stays independent of Grove. Its requirements list the generic capabilities that
the helpers need, and its own tests prove each one without Grove code. These are
the capabilities:

- A measured read of a task file, within its limits.
- JSON parsing and canonical JSON encoding.
- String matching, enough to parse a handle and a marker line.
- A random UUID, version 4.
- One policy that wraps another and changes its selection.
- A module import by path.

The owner chose this over a port in both repositories, and over Rhai ports of
the helpers as Ravel fixtures. The helpers are optional, and the owner's
personal policy uses none of them. This session drafts the Grove issue and
files it only after the owner approves the text.

**Providers in the first release: Anthropic, OpenAI and OpenAI-compatible
endpoints.** Ravel 2.0.0 supports the Anthropic and OpenAI providers through
Rig. It also supports any OpenAI-compatible endpoint that the owner configures
by URL. The owner configures each endpoint outside the script. A script thus
cannot send a credential to an endpoint that the owner did not configure. The
design session decides where that configuration lives.

The owner chose this over every provider that Rig ships, and over Anthropic and
OpenAI without a custom endpoint. Each exposed provider adds tests and
documentation, and each Rig upgrade must test it again.

**Test seams: one command-line seam, agreed with the owner.** Every test of this
work runs the real `ravel start-session` against a sandbox home that holds the
Rhai policy. Two kinds of fake stand behind that seam:

- Fake harness scripts on a private `PATH`, as the existing tests use.
- A deterministic local HTTP server that plays scripted responses or
  `rig-cassette` recordings. The sandbox configuration points each supported
  provider at it. The real Rig client runs with a dummy key and no network.

Opt-in Taskfile smoke tasks make one real provider call and one real harness
run with real credentials. The default test run never includes them. The owner
chose this over a fake model inside the binary, and over a second, in-process
seam.

**The owner confirmed a shared understanding.** The owner accepted the items
below, which this session proposed because the issue already implies them. The
owner also added the release and the local upgrade.

**Callers do not change.** The `start-session` flags, the policy context schema
and the result channel stay as they are. Only owner configuration changes.
Grove's driver therefore needs no change for Ravel 2.0.0.

**The migration guide translates three policies and adds one example.** It
translates the shipped static route table and the shipped sample policy. It
also translates a policy with the shape of the owner's personal policy: a
harness table and the grant of a jj workspace's main repository. A new example
shows an LLM-assisted policy.

**The workstream includes the release and the local upgrade.** The owner added
this at confirmation. The workstream ends after three steps. They are the Ravel
2.0.0 release, an upgrade of the owner's installation through Homebrew, and the
conversion of the owner's local configuration to Rhai. This session derived two
consequences from the addition:

- The upgrade runs inside a Grove session that Ravel 1.1.0 launched. The driver
  resolved the versioned 1.1.0 binary once, and that session ends through it.
  `brew upgrade` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP`
  is set. The upgrade must therefore keep 1.1.0 until the driver restarts.
- Sessions under the 1.1.0 driver read `policy.ts`, so both policy files exist
  until the driver restarts. This refines the cutover decision. When the Rhai
  policy file exists, 2.0.0 uses it and ignores `policy.ts`. When only
  `policy.ts` exists, 2.0.0 refuses with the migration remedy. The design
  session names the Rhai policy file.

**The next leaves: design, then planning.** A `design` leaf establishes how, as
a spec and a reworked ADR set. A `planning` leaf then cuts the design into
increments. No research leaf precedes the design. The design session verifies
Rhai and Rig against their source at pinned versions, and it can add a research
leaf if the evidence is thin. No review of this leaf follows, because the owner
confirmed it directly.

**"Provider" needs one meaning.** After this work, the word names the owner's
model-origin label in a selection. It also names the LLM service that Rig
calls. The design session settles the term in `CONTEXT.md`.

**A verb that evaluates a policy without a launch stays out of scope.** With LLM
calls in a policy, `start-session` is the only way to exercise it, and that
launches the selected program. Such a verb would help owners. But it adds a
third verb, and issue 2 does not ask for one. The root brief records it under
"On the horizon".

**This session filed the Grove issue.** It is
<https://github.com/Linkuistics/grove/issues/24>, with the text that the owner
approved.
