# native-entry-x86-64-linux-scope-k53

## Goal

Decide how native x86_64 Linux is to be treated: provide a host the campaign can
use, or accept user-mode emulation as the stated limit. This is a human choice;
the agent's work is to put it so that it can be answered in one reply.

## Context

The development evidence is in `docs/native-entry-development-evidence.md`,
"x86_64 under user-mode emulation". In short, the campaign was run for
x86_64-unknown-linux-gnu under QEMU user-mode emulation on an aarch64 Linux
kernel, with the pinned QEMU and the guest base the release smoke documents. It
builds and runs, and 146 of 153 matrix rows reconcile to the Linux expectations.
k52's immutable emulated run reproduced those numbers exactly (146 reconciled, 7
limited with reason and control, 74 protected identities that cannot run); the
receipts are under `docs/evidence/native-entry-linux-campaign/
x86_64-unknown-linux-gnu-emulated/` and the argument is in the evidence note's
k52 section.
It **cannot** qualify an x86-64 kernel, for reasons that are properties of the
emulator and are measured:

- every emulated process has a second host thread, so `unshare(CLONE_NEWUSER)`
  is EINVAL: the product's bubblewrap confinement cannot run at all, and 74 of
  the plain protected suite's 338 identities fail for that reason;
- SA_NOCLDWAIT is not honoured (the same C source gives ECHILD natively and the
  child's status under emulation), so the E04 rows cannot discriminate;
- E07's later-wins discriminator conformed under emulation, cause not isolated
  (the emulator and the architecture cannot be separated without a native run);
- a process's argv as `ps` reads it is the emulator's.

## The choice

1. **Provide an x86_64 Linux host the campaign can use** (a machine, a VM, a CI
   runner or a cloud instance with Docker or just rustup, Task, nextest and
   bubblewrap). Native qualification then costs one run of
   `scripts/native-linux/measure.sh` on it, plus a short session to record it,
   with no expected code change. *Recommended.*
2. **Accept emulation as the stated limit.** The documents say x86_64 Linux is
   built and run under emulation and is not qualified; the 74 identities and the
   7 rows stay recorded as emulation limits, and "supported" is claimed for
   aarch64 Linux and Darwin only.
3. A hybrid, such as a CI-only x86_64 qualification run with no local host, is a
   variant of 1 and needs the same answer to "who provides it".

## Recommendation and evidence

Option 1. The emulator cannot reproduce three independent mechanisms the
product's claims rest on (namespaces, SA_NOCLDWAIT, signal sequencing), so no
amount of further emulated running can turn into a statement about an x86-64
kernel, and the one row family it disagrees on (E07) cannot even be attributed.
Native aarch64 Linux shows those mechanisms are not exotic: all of them are
exercised and pass there. The cost of native x86_64 is a host, not engineering.

Accepting emulation (option 2) is a legitimate scope decision; what it must not
become is a qualification claim. Either way the documents already say so.

## Done when

- The human's answer is recorded verbatim in this file's decision log, with what
  it means: for option 1, who provides what and by when it is expected; for
  option 2, the exact support statement to carry into the spec and design.
- If option 1, a precise next leaf is cut (an `impl` run leaf that executes
  `measure.sh` natively and records it); if option 2, the documents' wording is
  reconciled by a short leaf. This leaf does neither.

## Notes

No narrowing of the 153-row matrix follows from either answer. Returning-host,
independent adoption, K4/k6, G2-G5/R-W and every root obligation are unaffected.

## Decisions (running log)

- Before asking, the framing was checked against the tree and corrected in what
  was put to the human. x86_64-unknown-linux-gnu is a shipped target: the public
  v1.0.0 release (2026-10-14) carries `ravel-v1.0.0-x86_64-unknown-linux-gnu.tar.xz`
  and `docs/RELEASING.md` lists it among the supported targets, smoke-tested only
  under emulation. "No expected code change" holds for product source,
  `measure.sh`, `scripts/native-campaign.sh` and the plan (a native run selects
  `platforms.linux`, the same expectations as aarch64; nothing there keys on the
  architecture), but not for the host launcher: `scripts/native-linux.sh` routes
  amd64 only through the emulated helper, so any native x86_64 Docker host needs
  a launcher route chosen from the Docker server's architecture, with a control
  (the launcher is a digested subject). None of this workstream's 45 commits, nor
  the incumbent `0515419edd29`, is on GitHub; `main` has never carried `.grove/`.
  QEMU 11.1.2 is installed here, so a full-system x86-64 guest was added as a
  route the task did not list. Four routes were put, GitHub x86_64 runner
  recommended.
- The human answered verbatim **"USe testanyware"**. Measured before recording a
  meaning: testanyware 2.1.0's only Linux golden is
  `testanyware-golden-linux-24.04` on the tart backend; a clone reports
  `aarch64`, kernel 6.17.0-19-generic, Ubuntu 24.04.4 LTS, no Rosetta mount
  (probe output kept in the session scratchpad, not the tree). No testanyware
  command takes an architecture, its Windows golden is ARM64, and its own
  sources say the same: `qemu_profile.rs` boots aarch64 guests under HVF on
  macOS and x86_64 guests only on an x86_64 Linux host with KVM, and
  `linux-host-harness.rs` records its own x86_64 runtime as "unverified on this
  host — closable later only with a real x86_64 box or TCG emulation"
  (TestAnyware ADR-0009). So the answer cannot by itself provide an x86-64
  kernel on this Mac; it is not yet a settled decision, and a clarification was
  put to the human.
- Clarification put: testanyware cannot start an x86_64 guest here; which route
  should the qualification take (add x86_64 to testanyware, recommended; a QEMU
  VM recipe in Ravel; a GitHub x86_64 runner; or accept emulation as the limit)?
  The human answered verbatim **"Add x86_64 to testanyware (Recommended)"**, then
  added, verbatim: **"We should use QEMU in that case, after cross-building.
  HOWEVER - we can leave this for now as a GH issue on this repo."** and **"And a
  GH issue in testanyware"**.
- **Settled.** Native x86-64 qualification is deferred out of this grove, not
  rejected, and is tracked by two public GitHub issues filed in this session:
  [Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1) (qualify
  native entry on a real x86-64 kernel) and
  [Linkuistics/TestAnyware#1](https://github.com/Linkuistics/TestAnyware/issues/1)
  (run x86_64 Linux guests on Apple Silicon hosts), cross-linked. *Who provides
  what:* TestAnyware gains an x86_64 Linux guest under QEMU full-system emulation
  (`qemu-system-x86_64`, TCG) on macOS hosts; Ravel cross-builds its x86_64
  binaries on the host and runs the unchanged frozen campaign and R01 in that
  guest, recording the result as a real x86-64 kernel on an emulated CPU, never
  as x86-64 hardware. *By when:* not scheduled; "for now" it waits on the
  TestAnyware issue and is not a precondition of this grove or the book. Under
  `references/retire.md` this is work that is not this tree's now, so it goes to
  issues rather than leaves, and no x86_64 run leaf is cut. *Support statement
  until then:* x86_64-unknown-linux-gnu is built, linted at Rust 1.85.0 and run
  only under QEMU user-mode emulation (146 of 153 rows reconciled, 7
  emulation-limited, 74 protected identities unable to run); nothing about an
  x86-64 kernel is qualified; its release archive keeps shipping as v1.0.0's
  does; native qualification is tracked in Ravel#1. Native-entry conformance is
  measured with immutable receipts on aarch64-apple-darwin and
  aarch64-unknown-linux-gnu only. No row of the 153-row matrix and no protected
  identity is narrowed. The deferral is easy to reverse, so it fails the ADR
  test; it lives here, in the issues and in the documents.
- The documents still name this leaf's handle as the open choice, so the short
  reconciliation leaf the Done-when foresees for a non-run answer is cut inside
  k22: an `impl` leaf that carries the statement above into the spec, the design
  and the evidence note, and links Ravel#1. This leaf edits none of them.
