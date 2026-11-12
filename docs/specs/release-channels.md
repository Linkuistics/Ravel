# Release channels for a caller transition

Grove's coordinated-release specification names the one release owner for the
2.0.0 transition and owns the order between products. The focused design review
of this document and its integration are complete. The release tasks carry the
channel as "Release interface" states. `release:promote` implements "Promotion
without another release".

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

Homebrew links a requested versioned keg-only formula into the shared prefix
when neither that formula nor its unversioned formula is installed. The first
installation of `ravel@2` therefore uses `brew install --skip-link`. An
upgrade keeps the link state of the installed keg.

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

- `scripts/release-channel.sh` owns the mapping and the admission rules. The
  other scripts read no environment value for the channel. Each script of the
  cut that writes or reads one formula takes `--channel` as an argument. Three
  scripts are exceptions:
  - `release-publish.sh --check-tap` takes no channel, because it reads the
    tap state alone.
  - `release-formula.sh` takes an optional channel as its second positional
    argument. It renders and tests both formulae on every run. Without a
    channel it keeps both formulae, and with one it keeps only that formula.
  - `release-promote.sh` takes no channel. A promotion always reads
    `ravel@2.rb` and writes `ravel.rb` and `ravel@2.rb`.
- The task requires `CHANNEL` for every version. Thus omission refuses at the
  first cut of 2.0.0 and at every later cut.
- `versioned-2` publishes only major 2. It refuses after `Formula/ravel.rb`
  has major 2. `unversioned` refuses a major-2 target until
  `Formula/ravel.rb` has major 2.
- `scripts/release-formula.sh` renders both classes from one template and
  tests each through Homebrew's install and test methods. The test fails when
  the two texts differ beyond the class line and the `keg_only` line.
- `scripts/release-install.sh` installs with cleanup suppressed. It installs
  `versioned-2` for the first time with `--skip-link`. It verifies
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
change nothing else in it. Publish that one commit through jj to the tap's
`origin`, the remote that the task fetched and verified. `ravel@2` stays
installable at the promoted version. Verify the unversioned installation with
cleanup suppressed.

On retry, the task verifies and retains a matching existing formula, commit or
installation. Report mismatches. Do not overwrite them to make a retry pass.

Consumer readiness is explicit input from the coordinating release owner.
Ravel's runtime acquires no Grove version table or dependency. "Consumer
readiness receipt" below gives the receipt's shape. Grove finish invokes this
already prepared task only after its compatible release and installed
verification succeed.

### Promotion interface

Run the promotion from Ravel's default colocated workspace:

```sh
RAVEL_TAP_DIR=/Users/antony/Development/Delivery/homebrew-taps task release:promote VERSION=2.0.0
```

`READINESS=FILE` names the readiness receipt. Its default is
`target/release-evidence/v2.0.0.readiness.json`. `EVIDENCE=FILE` names the
cut's release evidence. Its default is `target/release-evidence/v2.0.0.json`,
which `scripts/release-evidence.sh` wrote during the cut. The task holds the
release lock, and `scripts/release-promote.sh` does this work in order:

1. Verify the evidence and the receipt. Both are local files, so a missing
   receipt refuses before any network read. Each must be exactly one JSON
   object. An empty file refuses.
2. Verify that the public tag has the cut's commit and that the GitHub Release
   is complete.
3. Download the three assets. Compare each SHA-256 with the evidence.
4. Verify that the cut's book commit last changed `v2.0.0/` on the Pages
   branch. The root index must list the release.
5. Fetch the tap. The receipt's consumer formula must have the receipt's
   consumer version there.
6. Render both classes from the downloaded assets with the template at the
   tag's commit. Add the deprecation to the versioned rendering. Test the
   three files through Homebrew with `release-formula.sh --test`.
7. Bring the tap to the promotion as the table below states.
8. Install and verify unversioned `ravel`.
9. Write `target/release-evidence/v2.0.0.promotion.json`.

The deprecation is one line after the `keg_only` line, which is the place
that Homebrew's component order gives it:

```ruby
  deprecate! date: "<promotion date>", because: :versioned_formula, replacement_formula: "ravel"
```

Homebrew 7.0.13 requires the date. Homebrew marks the formula deprecated
only when the date is not after its local date. The date is the date of the
tag's commit at UTC−12. No time zone is behind UTC−12, so that date is never
in the future after the tag, also west of UTC. The date depends only on the
tag, so each retry renders the same bytes. `scripts/release-channel.sh` owns
the line, its date and its insertion.

| Tap state on the remote | Behaviour |
| --- | --- |
| `ravel.rb` and `ravel@2.rb` hold the promotion | Keep both. |
| Local `main` is one promotion commit ahead of the remote | Push that commit. |
| `ravel@2.rb` is the published versioned formula, and `ravel.rb` has a major below 2 | Make one commit, and push it through jj. |
| Any other state | Report the difference, and change nothing. |

The tap must be a jj repository with a clean working copy. The task fetches
and pushes the remote `origin` by name. A `git.push` setting of jj does not
select another remote.

Installation runs
`scripts/release-install.sh --channel unversioned --version 2.0.0`. It
upgrades with `HOMEBREW_NO_INSTALL_CLEANUP=1` and verifies the linked keg,
the shared command and the installation record. Before the mutation it
records each earlier `ravel` and `ravel@2` keg: each path, its type, a link's
target, and a file's executable bit and SHA-256. It refuses when any of them
changed afterwards. The 1.1.0 keg's `bin/ravel` runs
`libexec/ravel/ravel-policy` and reads its SDK. Thus `bin/ravel` alone does
not prove that a legacy launch still works.

Homebrew's own `INSTALL_RECEIPT.json` must stay, but its content can change,
because Homebrew rewrites it and Ravel does not read it. A repeated upgrade of
a current formula changes nothing, so a matching installation stays.

The promotion evidence records the tag commit, the asset digests and the book.
It records the receipt's digest and consumer, and the tap commit. It records
both formula digests, the deprecation date, the installed keg and every
retained keg.

### Consumer readiness receipt

The coordinating release owner writes the receipt after the compatible
consumer release and its installed verification. For the 2.0.0 transition,
that owner is Grove's finish session. The receipt is one JSON object:

| Field | Value |
| --- | --- |
| `schemaVersion` | `1`. |
| `kind` | `"ravel-promotion-readiness"`. |
| `ravel.version` | The promoted version, such as `"2.0.0"`. |
| `ravel.tagCommit` | The tag commit from the cut's release evidence. |
| `ravel.assets` | The three `name` and `sha256` entries from the cut's release evidence. |
| `consumer.name` | The consumer, such as `"grove"`. |
| `consumer.version` | The compatible consumer release, as `X.Y.Z`. |
| `consumer.formula` | The consumer's formula file in the same tap, such as `"grove.rb"`. Its `version` line must equal `consumer.version`. |
| `consumer.verifiedAt` | The UTC time of the installed verification, such as `"2026-11-20T09:30:00Z"`. |
| `consumer.verifiedRoutes` | A list of the route names that the consumer verified. Ravel does not interpret them. |

[The example receipt](release-channels-readiness.example.json) has
placeholder identities. The lifecycle fixtures bind that example to a fixture
cut, so its shape stays valid. Ravel reads no consumer version table. The
receipt and the consumer formula in the tap are its only consumer inputs.

This command writes the receipt from the cut's evidence, in Ravel's default
workspace:

```sh
jq --arg version 22.9.0 --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  '{schemaVersion: 1, kind: "ravel-promotion-readiness",
    ravel: {version: "2.0.0", tagCommit, assets},
    consumer: {name: "grove", version: $version, formula: "grove.rb", verifiedAt: $at,
      verifiedRoutes: ["native-1.1", "native-2.0-delivery-disabled", "directory-2.0"]}}' \
  target/release-evidence/v2.0.0.json >target/release-evidence/v2.0.0.readiness.json
```

Replace `22.9.0` with the compatible Grove release that finish published and
verified.

## Recovery and existing processes

The [release chapter](../book/src/11-deliver-ravel.md#recover-a-partial-release-without-another-bump)
owns exact recovery commands. Extend them to retain the selected channel before
using this route. Once a cut exists, recover it instead of repeating
`release:major`. Reconcile source/tag and preserve public identities. Upload
only missing verified assets, and resume the book before the tap. Finish
existing tap commits, and retry only failed installation/live verification.

Promotion has a separate boundary. A published formula with failed installation
needs installation recovery. An unpushed tap commit needs its push. Neither
condition warrants another Ravel release. Run `task release:promote
VERSION=2.0.0` again for both. It keeps the published formulae and pushes a
local promotion commit.

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
versioned delivery. Cover a versioned install on a prefix without either
formula. Cover unchanged archives and the `ravel@2` deprecation
during promotion, idempotent retries and preserved old prefixes.
`scripts/release-channel.test.sh` holds these fixtures and their mutants.
`task release:formula` loads the deprecated `ravel@2.rb` through Homebrew on
every run.

Full release still needs current Rust, book, archive/platform and public
installed verification. Schema reconciliation and real directory acceptance are release
inputs, not claims that release fixtures establish.
