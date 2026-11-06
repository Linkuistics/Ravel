# Skill sources are local directories

Ravel reads each skill source from a local directory. The owner configures
that directory, or a package installs it in the default skill directory. Ravel
does not fetch, install or update a source. Skill delivery runs
no VCS command and makes no network request. A tool outside Ravel puts each
checkout in place and chooses its revision.

The
[skill discovery specification](../specs/skill-discovery.md#settings) owns the
settings record and the refusal of every other source type. Its section
[sources outside Ravel](../specs/skill-discovery.md#sources-outside-ravel)
owns the capture of a checkout.

The owner decided that the installation of skills and their updates are not
part of Ravel. Ravel delivers the guidance that is on disk for one session. A
tool outside Ravel decides when that content changes. The same checkouts then
also serve harness sessions that Ravel did not launch.

The decision binds because Git acquisition is a large surface, and Ravel would
have to own all of it. Ravel implemented online exact Git acquisition on its
unreleased feature line. One review then found six defects. In four of them,
Git configuration, a locator or a server changed what the fetch did.

In one, global settings that allowed the `ext` protocol and rewrote a locator
ran an `ext::` helper. In another, a local `upload-pack` read none of the
settings that Ravel passed. The other two defects showed a complete URL in a
refusal and gave a wrong remedy. Each finding needed its own defence, and the
list had no clear end. The
[design evidence](../skill-discovery-design-evidence.md#withdrawal-of-git-acquisition)
records the six findings.

## Consequences

Ravel needs no Git executable, so its installation and its Homebrew formula
declare no Git. Delivery needs no network. Ravel captures the current files of
a checkout, with uncommitted changes. It records a resource digest, not a
revision, so the owner's tool keeps the record of which revision each checkout
holds. A missing checkout refuses as `skill_source_missing`, and the owner's
tool restores it.

## Considered options

- **Exact Git acquisition with an offline cache.** Ravel implemented it, and
  the owner withdrew it before any release. It made Ravel responsible for
  transport, credentials, Git configuration and the server side of a local
  fetch. Reopen only if the owner gives source installation back to Ravel.
  Ravel must then run a fetch whose configuration, helpers and hooks it
  controls completely.
- **A source type that Ravel skips.** The design rejected it. Every configured
  source is required, and a skipped source would deliver an incomplete
  guidance set with no refusal. Nothing reopens this option while sources stay
  required.
- **A dedicated failure code for a Git record.** The design rejected it. No
  release accepted a Git source, so no published settings file holds one.
  `settings_invalid` at `skills.sources[N].type` names the value and gives the
  checkout remedy. Reopen if a released Ravel withdraws a source type that
  owners used.
- **No `type` field.** The design rejected it. With one value the field gives
  no choice. It does give a former Git record a precise location and remedy,
  and it keeps the settings, descriptor and request shapes of the local stage.
  Reopen at the next incompatible change of the `skills` schema, if local
  directories are still the only type.
