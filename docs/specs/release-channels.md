# Release channels for a caller transition

The owner acknowledged that the coordinating Grove workstream alone remains
live through cutover. It owns the combined 2.0.0 release and owner upgrade. This
design awaits focused review and implementation. Existing release tasks still
publish unversioned `ravel` and must change before using this route.

## One release, two formula channels

One product release has one source commit, tag, book and three verified
archives. A formula channel selects how those same artifacts reach Homebrew. It
selects no different build or archive layout.

Ravel 2.0.0 combines Rhai and complete directory delivery. Publish it
first as keg-only `ravel@2`, leaving unversioned `ravel` at 1.1.0. Grove's
coordinated-release specification owns when its consumers permit promotion.

| Channel | Formula | Installed verification |
| --- | --- | --- |
| Unversioned | `ravel.rb`, class `Ravel` | Its selected prefix and `bin/ravel`. |
| Major 2 | `ravel@2.rb`, class `RavelAT2`, `keg_only :versioned_formula` | Versioned prefix and `bin/ravel`, never the default PATH command. |

Follow Homebrew's [versioned-formula convention](https://docs.brew.sh/Versions).
The versioned install writes no competing shared-prefix command and changes
neither unversioned formula nor installation. Its formula test uses the real
installed layout, private settings and a fake harness.

## Release interface

Extend existing tasks with one explicit channel carried through preparation,
rendering, publication, installation and verification. The new task input is
`CHANNEL=versioned-2` or `CHANNEL=unversioned`. Reject unknown values before
mutation. For coordinated 2.0.0, require explicit versioned selection: omission
must not publish unversioned Ravel. The task owns formula/class/prefix mapping.
Callers do not coordinate three independent arguments.

Preparation distinguishes the baseline channel from the publication target.
The first 2.0.0 cut verifies baseline 1.1.0 against unversioned `ravel`, then
publishes `ravel@2`. Later 2.x cuts verify their baseline against `ravel@2`,
even while unversioned remains 1.1.0. Preserve incomplete-baseline refusal.

Resume modes retain the selection. `--tap-only` must not silently default to
unversioned. Verify class, keg-only behaviour, destination and prefix together.
Record channel, source/tag, asset digests, book commit, tap commit and installed
result in the release task's evidence. No second release-state database is needed.

Keep the existing order. Verify the tagged book and archives, and smoke all
targets. Push source/tag, publish GitHub assets, and publish and verify the
book. Then publish the selected formula. Verify installed behaviour and the live
book.

Keep the release lock and the clean-tap verification. The acknowledged
coordinating owner prevents a competing workstream from cutting another release.

## Promotion without another release

Provide a separate repeatable task for an already complete release:
`release:promote VERSION=2.0.0`. It cuts no version, rebuilds no
archives, creates no tag/release and rewrites no book. Before tap mutation,
verify public tag, exact asset digests, book identity, versioned formula and
consumer readiness evidence. Missing evidence refuses with a remedy.

Render and test the unversioned class from those same verified assets. Then
publish only the unversioned formula through jj. Retain `ravel@2`. Verify the
unversioned installation with cleanup suppressed.

On retry, the task verifies and retains a matching existing formula, commit or
installation. Report mismatches. Do not overwrite them to make a retry pass.

Consumer readiness is explicit input from the coordinating release owner.
Ravel's runtime acquires no Grove version table or dependency. Release-tool
implementation settles the receipt shape and plumbing, with actionable errors
and deterministic fixtures. Grove finish invokes this already prepared task
only after its compatible release and installed verification succeed.

## Recovery and existing processes

The [release chapter](../book/src/11-deliver-ravel.md#recover-a-partial-release-without-another-bump)
owns exact recovery commands. Extend them to retain the selected channel before
using this route. Once a cut exists, recover it instead of repeating
`release:major`. Reconcile source/tag and preserve public identities. Upload
only missing verified assets, and resume the book before the tap. Finish
existing tap commits, and retry only failed installation/live verification.

Promotion has a separate boundary. A published formula with failed installation
needs installation recovery. An unpushed tap commit needs its push. Neither
condition warrants another Ravel release.

Retain active callers' full original prefixes. For this transition that includes
the original 1.1.0 keg and ending path. Apply `HOMEBREW_NO_INSTALL_CLEANUP=1`
to every package mutation, and do no explicit cleanup of active readers. The
[Homebrew manual](https://docs.brew.sh/Manpage#environment) describes automatic
cleanup suppression. It does not protect against explicit uninstall or cleanup.

The operator owns activation separately. Prefixes can coexist while settings
under one HOME remain incompatible. Private formula tests do not change that.

## Verification

Extend release-lifecycle fixtures and actual formula install/test methods.
Cover baseline versus target channel, resume exits, wrong-channel refusal,
unchanged unversioned formula during versioned delivery, unchanged archives
during promotion, idempotent retries and preserved old prefixes. Full release
still needs current Rust, book, archive/platform and public installed
verification. Schema reconciliation and real directory acceptance are release
inputs, not claims that release fixtures establish.
