# Ravel does not confine a session

Ravel 2.0.0 applies no operating-system confinement. The managed process and
each harness run of a configuration script receive the caller's environment,
less the launch controls. The harness's own sandbox is the only boundary of
either. The configuration script selects that sandbox through the arguments of
the harness. A 2.0 front refuses `--confine` and `--runtime-read`, and Ravel
1.1 keeps them.

The [session execution specification](../specs/session-execution.md#no-confinement)
owns the refusal and its remedy. The
[skill discovery specification](../specs/skill-discovery.md#read-access-and-delegation)
owns the read access to skill resources.

## The trade-off

Confinement put an outer boundary around a harness that already has an inner
one. The measurements of 2026-11-03 showed that the outer boundary replaced
the inner boundary instead of adding to it:

- Claude Code 2.1.317 has no sign-in under the macOS profile, because the
  profile denies the Keychain. It made no model request.
- The sandbox of Codex 0.166.0 cannot run inside the macOS profile. A
  confined Codex completed its session only with `--sandbox
  danger-full-access`, so the profile of Ravel was its only sandbox.
- On Linux, bubblewrap resolves each bind source again after Ravel checks it.
  A process of the same user can replace a granted path in that interval. The
  repair needs bubblewrap 0.10.0, which Ubuntu 24.04 and Debian 12 do not ship.

Grove's `docs/common-loading-acceptance-record.md` holds the measurements, in
its sections "Confined harness repair", "Token handoff integrated design
review" and "Token handoff withdrawal". The human decided the removal on
2026-11-03, in Grove's `stage-evidence-reconciliation-k125`.

The price is the guarantees that only confinement gave. Ravel no longer stops
a managed process from reading the owner's configuration, a mutable skill
source or another project. It no longer limits the writes of that process. The
harness sandbox now sets those limits. An owner who selects a harness with its
sandbox off has no boundary at all.

Two protections stay, because they govern what Ravel publishes, not what a
sandbox denies. A skill source that overlaps a protected path refuses before
capture. A published snapshot is read-only by its file modes.

## Rejected alternatives

- **Widen the macOS profile for the Keychain.** The sign-in of Claude Code
  needs two Mach services and reads of the Keychain files. That grant opens
  the owner's whole login keychain to a confined process.
- **Send a confined Claude Code a token through a socket.** The owner would
  mint a token for Ravel to carry. The human withdrew this design in
  `ravel-token-handoff-k141`, in favour of the harness sandboxes.
- **Keep `--confine` as an option of 2.0.0.** No supported harness works under
  it with its own sandbox and its sign-in. A removal after the 2.0.0 cut needs
  a 3.0.0.
- **Accept `--confine` and ignore it.** A caller that asks for a boundary
  would get a launch without it.

Reopen this decision when a supported harness can keep its own sandbox and
its sign-in inside an outer boundary. Reopen it also when a caller needs a
boundary that no harness sandbox gives.
