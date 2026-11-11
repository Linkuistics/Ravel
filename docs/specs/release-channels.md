# Release channels for a caller transition

Grove's coordinated-release specification names the one release owner for the
2.0.0 transition and owns the order between products. The focused design review
of this document and its integration are complete. The release tasks carry the
channel as "Release interface" states. The promotion task remains ahead.

## One release, two formula channels

One product release has one source commit, tag, book and three verified
archives. A formula channel selects how those same artifacts reach Homebrew. It
selects no different build or archive layout.

Ravel 2.0.0 combines Rhai and complete directory delivery. Publish it
first as keg-only `ravel@2`, leaving unversioned `ravel` at 1.1.0. Grove's
coordinated-release specification owns when its consumers permit promotion.
[A caller transition publishes a versioned formula first](../adr/a-caller-transition-publishes-a-versioned-formula-first.md)
records the reason and the rejected alternatives.

| Channel | Formula | Installed verification |
| --- | --- | --- |
| Unversioned | `ravel.rb`, class `Ravel` | Its selected prefix and `bin/ravel`. |
| Major 2 | `ravel@2.rb`, class `RavelAT2`, `keg_only :versioned_formula` | Versioned prefix and `bin/ravel`, never the default PATH command. |

Each formula writes the
[installation record](skill-discovery.md#the-installation-record) into its keg
at installation, and its installed verification reads that record.

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
publishes `ravel@2`. Until promotion, a later 2.x cut verifies its baseline
against `ravel@2` and publishes `ravel@2` again. Unversioned `ravel` stays at
1.1.0 meanwhile. Preserve incomplete-baseline refusal.

After promotion, unversioned `ravel` is the only publication channel. A later
cut selects `CHANNEL=unversioned`, verifies its baseline against unversioned
`ravel` and publishes there. The task refuses `CHANNEL=versioned-2` once
unversioned `ravel` has major 2. `ravel@2` stays at the promoted version as a
transition formula and receives no later release.

Resume modes retain the selection. `--tap-only` must not silently default to
unversioned. Verify class, keg-only behaviour, destination and prefix together.
Record channel, source/tag, asset digests, book commit, tap commit and installed
result in the release task's evidence. No second release-state database is needed.

The implementation keeps these rules in the following places:

- `scripts/release-channel.sh` owns the mapping and the admission rules. Every
  script that writes or reads a formula takes `--channel` as an argument and
  reads no environment value for it. Only `release-publish.sh --check-tap`
  takes no channel, because it reads the tap state alone.
- The task requires `CHANNEL` for every version. Thus omission refuses at the
  first cut of 2.0.0 and at every later cut.
- `versioned-2` publishes only major 2. It refuses after `Formula/ravel.rb`
  has major 2. `unversioned` refuses a major-2 target until
  `Formula/ravel.rb` has major 2.
- `scripts/release-formula.sh` renders both classes from one template and
  tests each through Homebrew's install and test methods. The test fails when
  the two texts differ beyond the class line and the `keg_only` line.
- `scripts/release-install.sh` installs with cleanup suppressed. It verifies
  the keg's `bin/ravel` and installation record, the keg-only state and the
  opt prefix. For `versioned-2`, it also verifies that unversioned `ravel` is
  unchanged.
- `scripts/release-evidence.sh` writes `target/release-evidence/vX.Y.Z.json`
  before the book's `live` step.

Keep the existing order. Verify the tagged book and archives, and smoke all
targets. Push source/tag, publish GitHub assets, and publish and verify the
book. Then publish the selected formula. Verify installed behaviour and the live
book.

Keep the release lock and the clean-tap verification. The lock does not
prevent a later cut by another operator. Grove's coordinated-release
specification names the one release owner from the first cut through promotion.

## Promotion without another release

Provide a separate repeatable task for an already complete release:
`release:promote VERSION=2.0.0`. It cuts no version, rebuilds no
archives, creates no tag/release and rewrites no book. Before tap mutation,
verify public tag, exact asset digests, book identity, versioned formula and
consumer readiness evidence. Missing evidence refuses with a remedy.

Promotion also moves every default-channel user to Rhai configuration. Ravel
2.0.0 refuses a HOME that holds only `policy.ts` with `config_format_retired`
and exit 3, as [the configuration script uses Rhai](../adr/configuration-script-uses-rhai.md)
records. The tagged 2.0.0 changelog must name that format change, the refusal
and book chapter 4, which the refusal's remedy cites.

Render and test the unversioned class from those same verified assets. In the
same tap commit, mark `ravel@2` deprecated in favour of unversioned `ravel`, and
change nothing else in it. Publish that one commit through jj. `ravel@2` stays
installable at the promoted version. Verify the unversioned installation with
cleanup suppressed.

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

That variable protects only the mutations that set it. Any other install,
upgrade or reinstall can run Homebrew's periodic full cleanup, which removes old
versions. After the unversioned upgrade, the 1.1.0 keg is such an old version.
From that upgrade until its last reader ends, add `ravel` to
`HOMEBREW_NO_CLEANUP_FORMULAE` and keep the existing entries. Set it where every
Homebrew invocation on the machine reads it, such as Homebrew's user `brew.env`.

The operator owns activation separately. Prefixes can coexist while settings
under one HOME remain incompatible. Private formula tests do not change that.

## Verification

Extend release-lifecycle fixtures and actual formula install/test methods.
Cover baseline versus target channel, resume exits, wrong-channel refusal,
`versioned-2` refusal after promotion, and unchanged unversioned formula during
versioned delivery. Cover unchanged archives and the `ravel@2` deprecation
during promotion, idempotent retries and preserved old prefixes. Full release
still needs current Rust, book, archive/platform and public installed
verification. Schema reconciliation and real directory acceptance are release
inputs, not claims that release fixtures establish.
