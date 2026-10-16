# Configuration handoff, 2026-10-12

This is a dated record of one copy of the owner's policy: what was copied, what
changed and what the caller-owned choices were on that day. It describes the
owner's machine-local files. It is not Ravel's registry or defaults, it does not
describe how to configure Ravel (the book does, in chapters 4 and 6), and later
edits to the owner's policy are not tracked here.

The owner's source directory contained only `policy.ts`; there were no explicit
settings or helper files. The destination did not exist. The copy retains every
route, command argument, refusal and the owner version label `harness-dispatch 2`.
The SDK import now names `ravel/sdk`; comments name Ravel and replace the obsolete
inspection example. Both files retain mode 0644. No settings file was invented,
so supported bounds and environment grants keep their matching defaults.

The caller-owned policy maps 24 kinds. Its current choices are:

| Caller role | Program / model | Effort |
|---|---|---|
| Requirements, integration of requirements | codex / gpt-6.1-sol | max |
| Review of requirements | claude / claude-opus-5-5 | max |
| Design, integration of design | codex / gpt-6.1-sol | xhigh |
| Review of design | claude / claude-opus-5-5 | xhigh |
| Planning, implementation and their integrations | codex / gpt-6.1-sol | high |
| Review of planning or implementation | claude / claude-sonnet-5-5 | high |
| Prototype and its integration | codex / gpt-6.1-sol | medium |
| Review of prototype | claude / claude-sonnet-5-5 | medium |
| Research A, combined research | codex / gpt-6.1-sol | xhigh |
| Research B | claude / claude-opus-5-5 | xhigh |
| Draft, art | codex / gpt-6-astra | high |
| Copy edit | claude / claude-opus-5-5 | high |
| Proof | codex / gpt-6.1-sol | medium |
| Finish, release notes | claude / claude-sonnet-5-5 | medium |

These are the owner's current caller conventions, not Ravel's registry or
defaults. Unmapped kinds refuse. The policy does not use a choice file. It grants
the main repository through `--add-dir` when a secondary jj workspace's
`.jj/repo` file names its store; primary workspaces and ordinary directories
require no extra directory grant. Its Codex arguments retain the owner's full
access and approvals-off settings. Ravel does not interpret those arguments.

The source policy SHA-256 is
`59a54bfae00295a887ab87283e7ba0a794c456a437fb142ff19a6a3d87e2275d`;
the copied policy SHA-256 is
`7ca627d265fb8b3d81a1f7f710aa42298eac7aa6369fec92e126f608efd3196e`.
These identify this handoff, not future owner edits. Personal policy code stays
outside the project. Only configuration was copied; the old store was neither
opened nor imported, and the old installed front remains available to its live
driver. Packaging/publication is a separate handoff.
