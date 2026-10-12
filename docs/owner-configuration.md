# Owner configuration

`start-session` defaults to `~/.config/ravel/policy.ts`. A sibling
`settings.json` may set `timeoutMs`, `contextBytes` and `policyEnv`; each is
optional. An absent settings file keeps the 30,000 ms selection bound, 262,144
byte context budget and no extra policy environment grants. The default source
read is 65,536 bytes, with 256 sources, a 1,048,576 byte protocol message and
262,144 diagnostic bytes. CLI flags can replace the adjustable bounds and add
grants. See the [session CLI](session-cli.md) and
[executable specification](specs/session-execution.md).

The policy imports `ravel/sdk`. Its version string is the owner's label, not
the executable version. `readChoice` uses `.ravel-choice` only when the policy
calls it; Ravel does not choose a route from that file on its own. Caller kinds,
provider/model labels and command arguments belong to the owner policy.

## Copying an existing policy

Compare both configuration directories before copying. Preserve destination
edits and source files; copy missing files without overwriting existing ones,
then reconcile any differences explicitly. Relocate SDK imports to `ravel/sdk`
and keep relative helpers beside their importing policy. Caller-specific helpers
remain the caller's files; Ravel embeds no caller adapter. Preserve selection
choices, supported bounds and grants. Remove obsolete storage settings such as
`stateDir` from the destination: they refuse as unknown keys. Copy no records,
databases, snapshots or history. The old tool and its configuration can remain
installed independently.

`start-session` launches the selected program. Ravel has no inspect or init verb.
Validate through a private PATH containing fake executables for every program
the policy can select, with a cleared subprocess environment, temporary session
channels and bounded execution. Fake agents can capture arguments and invoke
`end-session` to return an exact caller value. Never use real agent executables
for this check. Check refusals as well as successful routes.

For development typechecking, run `task worker` to generate SDK declarations.
Create a temporary tsconfig extending `crates/ravel/worker/tsconfig.json`, include
the owner policy's absolute path, map `ravel/sdk` to
`target/libexec/ravel/sdk/index.d.ts`, and set `typeRoots` to the absolute
`crates/ravel/worker/node_modules/@types` directory. Then run the pinned checker:

```sh
target/tools/bun crates/ravel/worker/node_modules/typescript/bin/tsc -p /path/to/temporary/tsconfig.json
```

This uses the generated public declarations and the repository's strict options.
The installed worker embeds the SDK and runtime; the development checker is not
needed for normal installed execution.

## Configuration handoff on 2026-10-12

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
