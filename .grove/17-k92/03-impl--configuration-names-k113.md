# configuration-names-k113


## Goal
Use configuration names for public diagnostics and JSON output, preserving
behaviour and frozen pre-cutover observations.



## Context
Follows `configuration-names-k112`. Inspect refusals, notices, selection handling,
worker/front/host capabilities, comparison normalization and all output consumers.

## Done when
- Output fields are `config`, `configEnvironment`, `configCode`; Ravel-owned codes
  and locations use `config_` and `config.`. Prefixes are `config stdout:` and
  `config stderr:`. Messages and remedies use configuration terms.
- Failure documents use schema version 2. Routing notices remain 2 and script
  schema remains 3. Reconcile producers, consumers, examples and schema statements.
- Frozen pre-cutover fixtures are unchanged. Explicit tested comparison
  translation accounts for renamed fields/messages without hiding behaviour
  changes; the existing mutation control still fires.
- Text/JSON refusals and notices have CLI coverage. Preserve exact opaque results,
  caller-owned values, cancellation and cleanup behaviour.
- Current consumers and executable examples agree; `task check`, `task test`,
  complete examples, checker controls and excerpts pass. Record platform limits.

## Notes
The next child owns remaining source/document filename and prose reconciliation.
Do not rename arbitrary owner-supplied codes or results. The parent contract is
the completion checklist; use direct source fallback if the graph stays unavailable.
