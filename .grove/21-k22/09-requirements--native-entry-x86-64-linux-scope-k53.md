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

