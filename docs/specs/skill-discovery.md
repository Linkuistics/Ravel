# Skill discovery

Ravel resolves the skill sources that the owner configures, or the sources that
packages install in its default skill directory. It adds their directory paths
and loading instructions to the prompt before the configuration script runs.
The agent finds the applicable skills in those directories and reads them. Ravel does not resolve a skill that a caller names.
The caller still supplies its task, its conventions and its exact ending
instructions.

## Status and ownership

This specification states the directory-delivery contract of the combined
Ravel 2.0.0. It is the contract of the `skill-directory-delivery` line. Ravel
`main` does not hold this feature, and no release contains it.

Skill sources are local directories. Ravel reads them, and it does not fetch,
install or update them, as
[skill sources are local directories](../adr/skill-sources-are-local-directories.md)
records.

The line implements this contract, with the refusal of a source `type` other
than `local` in [settings](#settings). The
[default skill directory](#default-skill-directory) joined the contract before
the 2.0.0 cut, and the line implements it. The formula template writes the
installation record, and its test reads it. The channel tooling that renders
`ravel@2` from that template is ahead, as
[release channels](release-channels.md) states.

The [actual harness trials](#actual-harness-trials) are still ahead.
The [design evidence](../skill-discovery-design-evidence.md)
records the source observations, the exact base of the line and the handoff of
each stage.

Each rule has one owner:

- This specification owns the `skills` settings object, the default skill
  directory and its installation record, source resolution, the retained
  resources and the directory descriptor. It also owns the
  prepared prompt, `ravel skills resolve`, `RAVEL_SKILL_DIRECTORIES`, the skill
  bounds and failures, the read access to skill resources and the acceptance
  cases of this feature.
- The [configuration evaluation specification](configuration-evaluation.md)
  owns the request map and the settings file as a whole. It also owns the host
  capabilities, the Rhai worker, its frames and the evaluation bounds. It lists the schema-4
  fields and links here for their meaning.
- The [session execution specification](session-execution.md) owns the
  managed process, the result channel and the refusal of the removed
  confinement flags.

Five decision records bind this feature:

- [Skill preparation precedes evaluation](../adr/skill-preparation-precedes-evaluation.md)
  records the boundary and the schema change.
- [Skill sources are local directories](../adr/skill-sources-are-local-directories.md)
  records why Ravel installs no source.
- [Installed skill sources are the default](../adr/installed-skill-sources-are-the-default.md)
  records why Ravel reads the default skill directory without settings.
- [Skill resources outlive sessions](../adr/skill-resources-outlive-sessions.md)
  records retention.
- [Ravel supervises the session](../adr/ravel-supervises-the-session.md)
  records the inherited authority of delegates.

The [visual views](../design/skill-discovery/index.html#discussion) show the
flow.

## Ownership and order

The Ravel CLI resolves the sources, checks the resources and publishes the
snapshots and the descriptor. It prepares the prompt. The Rhai worker receives a complete request. It finds no skill and
reads no source. The configuration script still chooses the harness, the model
and the command. It puts `request.prompt` into the arguments of that harness,
as before.

No helper function is necessary. Ravel does not inspect or change the selected
argument vector. A script can ignore the prepared prompt. It then gives up
automatic delivery, as a script that ignores the prompt today gives up the
task.

`start-session` runs these steps in this order:

1. It checks for `RAVEL_CONFIG_RUN`, as the
   [re-entry rule](configuration-evaluation.md#re-entry-and-completion-authority)
   requires.
2. It reads the settings and the caller inputs, and it finds the entry of the
   configuration script.
3. It finds the source set, resolves the skill sources and prepares the
   prompt. A disabled configuration does nothing more in this step.
4. It launches the Rhai worker. The whole evaluation deadline runs from this
   launch. The worker imports the entry, then runs `load_context` and
   `select`.
5. It allocates the result channel, writes the routing notice and launches the
   managed process.

A failure in step 3 prevents every later step. Step 3 creates no result channel
and launches no worker. The order in
[configuration evaluation precedes the launch](../adr/configuration-evaluation-precedes-the-launch.md)
still binds.

| Boundary | Guarantee | Decisions hidden from its consumer |
| --- | --- | --- |
| Source resolver | A complete, checked and immutable resource set, or a refusal | Local capture, staging and publication |
| Prompt preparation | The original task, intact, with the directories and loading instructions | Path rendering, the block delimiters and byte accounting |
| Request | The original prompt, the prepared prompt and the descriptor fields | Source I/O and cache coordination |
| Managed launch and harness runs | The exact published resources, at stable absolute paths that the account of the owner can read | The cache layout |
| Harness delegation | The same directories and readable resources. Ending restrictions depend on the adapter | Delegate construction and its measured isolation |

Ravel gives no meaning to the domain of a skill, the kind of a caller, provider
receipts, task trees or result tokens. Skill text is guidance that the owner
selected. It is not configuration authority, and it grants no access.

## Settings

The settings file `$HOME/.config/ravel/settings.json` gains an optional
`skills` key. It sits beside `timeoutMs`, `contextBytes`, `configEnv`,
`harnesses` and `budgets`. The file stays at that path, also with `--config`.
Ravel has no workspace search, no environment override and no marketplace
lookup. A configuration script holds no skill configuration.

Source content stays separate from the protected configuration. A source is
trusted because the owner configured it, not because a task names it.
[Skill preparation precedes evaluation](../adr/skill-preparation-precedes-evaluation.md)
records why skills share the settings file, and why use of two majors at once
isolates `HOME`.

Ravel 2.0.0 refuses a settings file that repeats a member name in any object.
The refusal is `settings_invalid`, stage `cli`, exit 2. This rule applies to
every key, not only to `skills`.

`skills` accepts exactly `schemaVersion` and the optional `sources`,
`mandatory` and `resolutionTimeoutMs`. `schemaVersion` is the integer 1.
`sources` and `mandatory` are arrays. An unknown key, a duplicate source ID or
a malformed value refuses as `settings_invalid`, exit 2. Its `location` names
the value, such as `skills.sources[2].path`. The other settings keys keep
their meanings.

**The source set** is the set of sources that one resolution captures.
`sources` decides it:

- Without `sources`, the [default skill directory](#default-skill-directory)
  supplies the source set. That includes a settings file without `skills`,
  and an absent settings file.
- With `sources`, the array names exactly the source set. Ravel then reads no
  default skill directory and no installation record.
- `"sources": []` therefore turns the default off.

`resolutionTimeoutMs` is an integer from 1,000 to 3,600,000, and its default
is 120,000. It bounds one complete resolution: capture, cache checks and
publication. It is independent of `timeoutMs`, which bounds evaluation. No
flag overrides it, and no value removes the bound.

Diagnostics report the effective deadline. On a timeout they report the active
source and operation. An owner raises the bound for valid sources on slow
storage.

Ravel examines the deadline and the cancellation latch between operations. It
examines them at each entry, at each block of a copy and at each entry of a
check or a seal. It cannot interrupt one system call that storage holds. The
refusal then comes after that call returns.

Each source record has these fields. An unknown field refuses.

| Field | Meaning |
| --- | --- |
| `id` | A unique owner token of 1 to 64 lowercase ASCII letters and digits, in groups that single hyphens separate. Its first character is a letter |
| `type` | `local`, the only source type |
| `path` | The source directory. An absolute UTF-8 directory path, with no tilde or environment expansion |
| `roots` | A nonempty array of normalised search directories, relative to the source directory. `.` is permitted |

Within a source record, Ravel reads `type` before the other fields. A missing
`type`, or a value other than the string `local`, refuses as
`settings_invalid`, stage `cli`, exit 2. The `location` is
`skills.sources[N].type`, and Ravel checks no other field of that record. The
repeated-member rule above still applies to the whole file first.

The remedy states that `type` accepts only `local`, so it also covers a
misspelt value. It then tells the owner to make any checkout of a repository
with a tool outside Ravel. The owner names the directory of that checkout in a
`local` record with `path`.

A record with `type: "git"` receives this refusal, as every other value does.
No release of Ravel accepted a Git source. The refusal comes before capture,
before cache work and before the worker, at both public boundaries.
[Sources outside Ravel](#sources-outside-ravel) states what the owner's tool
does.

Each root contains no absolute path, empty segment, `..`, backslash or
trailing separator. A repeated identical root in one source refuses as a
configuration mistake. Overlapping roots are valid. Each root must
exist as a directory in the captured source, and an empty directory is valid.

Ravel does not search for `SKILL.md`. It does not parse frontmatter, names,
descriptions or pack manifests. Roots choose which directories Ravel shows the
agent. The whole source directory supplies the supporting resources. Ravel
keeps the order of sources and roots for presentation. It never uses that order
to choose between skills.

`mandatory` is an ordered array of records with exactly `source` and `path`.
`source` names the ID of a source in the source set. `path` is a normalised
file path, relative to that source, with the grammar of a root. `.` is not a
file.

Each
reference must reach a readable UTF-8 regular file in the captured source. An
admitted symlink inside the source can lead to it. The file can be outside the
search roots, but never outside its source.

Two references to the same file refuse. A repeated identical reference refuses
as `settings_invalid`, exit 2. Two other references can reach one retained file
through an admitted link. Two sources with identical contents also share one
snapshot. Such references refuse as `skill_mandatory_invalid`, exit 3, at the
later reference.

Ravel translates each reference to an absolute path in the snapshot. It does not resolve a skill name, and it does
not interpret the file. The agent remains responsible for the supporting reads
that a mandatory file requires. No Grove or personal guidance is a default.

With `sources`, an unknown `source` ID refuses as `settings_invalid`, exit 2,
before capture. Without `sources`, an ID that the default skill directory does
not hold refuses as `skill_source_missing`, exit 3, before capture. Its remedy
is to install the package that supplies the source, or to name the source in
`sources`. A known source with a missing, unreadable, nonregular or non-UTF-8
mandatory file refuses as `skill_mandatory_invalid`, exit 3. An absent or
empty `mandatory` array is valid.

Delivery is disabled when the source set and `mandatory` are both empty:

- `"sources": []` with no mandatory reference disables delivery.
- Without `sources`, a default skill directory that supplies no source
  disables delivery.
- Without an absolute `HOME`, Ravel reads no settings and no default skill
  directory, so delivery is disabled.

A disabled resolution creates no cache and adds no prompt text. A nonempty
`mandatory` array with `"sources": []` refuses as `settings_invalid`. An
installation without installed skill sources has delivery disabled, and native
harness skills work without it.

Every source of the source set is required. A missing local directory or
configured root refuses as `skill_source_missing`. The refusal names the
source ID and the path. Its remedy is to restore the directory or to change the
settings deliberately. Ravel never creates an empty source, disables a source,
substitutes native skills or changes settings.

An existing empty root is not a missing root. A new machine restores its
sources outside the failing session. Alternatively, the owner removes their
configuration before a bootstrap launch. No option of one session skips the
owner's guidance.

## Default skill directory

A package manager can be unable to write into `HOME`. Homebrew is the case,
because its sandbox denies those writes. A package can still install skill
sources beside Ravel. The **default skill directory** is the one directory
where such packages expose their sources. Ravel reads it when the settings name
no `sources`.
[Installed skill sources are the default](../adr/installed-skill-sources-are-the-default.md)
records why.

Each package keeps its sources intact, as
[packaging a default source](#packaging-a-default-source) states.

### The installation record

Ravel finds the default skill directory through its **installation record**,
the file `share/ravel/installation.json` in the prefix of the installation.
The prefix is the parent of the directory that holds the canonical path of the
running executable. A Homebrew keg is such a prefix, because its executable is
`bin/ravel`.

The record is a JSON object with exactly two members:

| Member | Contract |
| --- | --- |
| `schemaVersion` | The integer 1 |
| `skillDirectory` | The absolute UTF-8 path of the default skill directory. It holds no empty segment, `.`, `..`, trailing separator, tilde or control character |

Each Homebrew formula of Ravel writes the record at installation. Its
`skillDirectory` is `HOMEBREW_PREFIX/share/ravel/skills`, with the prefix of
that Homebrew installation. The keg-only `ravel@2` therefore names the same
directory as the unversioned `ravel`. An installation on Linux names the
directory under its own prefix. The formula records the prefix, so a Cellar
inside Homebrew's repository changes nothing.

The release archives hold no record, because only the installation knows its
prefix. `scripts/install.sh` writes no record either. An installation without a
record has no default skill directory. A build that runs from `target`, a
copied executable and an extracted archive are such installations. Their
settings name the source set, or delivery is disabled.

Ravel reads the record only when the settings name no `sources`. An absent
record is valid. A record that is unreadable, not a regular file, not UTF-8 or
not such an object refuses as `skill_directory_invalid`. A repeated member
refuses the same way.

### Entries and roots

The entries of the default skill directory are its sources. Ravel lists the
directory at each resolution, and it keeps no list between resolutions. An
absent directory supplies no source and refuses nothing. That is the state
before any package installs a source.

| Entry | Result |
| --- | --- |
| A name whose first character is `.` | Ignored. Operating systems and file managers write such entries, such as `.DS_Store` |
| A directory with a name of the source-ID grammar and at least one root | A source. Its ID is the entry name |
| A link whose chain ends at such a directory | A source, as above |
| A directory or a link target that holds neither root, such as an empty directory | Refuses |
| A name outside the source-ID grammar | Refuses |
| A regular file, or another object that is not a directory | Refuses |
| A dangling or cyclic link | Refuses |

Each refusal in this table is `skill_directory_invalid`, stage `skills`, exit
3. It names the entry. Its remedy is to remove the entry, or to reinstall the
package that installed it. `brew cleanup --prune-prefix` removes a dangling
link of Homebrew.

The roots of a default source are `skills` and `craft`, each where it exists as
a directory in the source, in that order. Ravel looks for no other root. The
users of a package with another layout name its source in `sources`.

The source path of a default source is the path of its entry, such as
`/opt/homebrew/share/ravel/skills/grove-core`. Capture applies every rule of
[sources and retained resources](#sources-and-retained-resources) to that path.
Capture canonicalises the source root, so `originPath` names the directory that
the link reaches. The protected paths of
[read access and delegation](#read-access-and-delegation) apply too.

Default sources follow the byte order of their IDs, in the descriptor and in
the preamble. That order chooses no skill. The [bounds](#bounds) count the
default sources and their roots. A refusal about a default source names its
entry path in place of a settings location.

### Packaging a default source

A package installs each source as a private payload. It exposes the payload
through a relative directory symlink, the **source alias**. The default skill
directory then holds a link to the alias, not a copy of the source. A Homebrew
formula uses this layout in its keg:

| Path in the keg | Content |
| --- | --- |
| `libexec/skill-sources/ID/` | The payload: the source directory, with its executable bits and its links |
| `share/ravel/skills/ID` | The source alias: the relative directory symlink `../../../libexec/skill-sources/ID` |

Homebrew links the alias into `HOMEBREW_PREFIX/share/ravel/skills/ID`. That
entry is a link whose chain ends at a source, a row of the entry table.
Capture canonicalises the source root, so `originPath` names the payload in
the versioned keg.

A real source directory under `share` does not stay intact. Homebrew first
links a directory that one keg holds as a single directory link. A second
keg can link into the same directory. `Keg#resolve_any_conflicts` then
replaces the link with a real directory. It links the displaced subtree again
with the `:mkpath` strategy, which descends into each real directory.

The source of the first keg thus becomes a tree of links to its individual
files. Each such link leaves the source boundary, so Ravel refuses the whole
resolution as `skill_resource_invalid`. One such package therefore blocks the
sources of every other package.

`Keg#link_dir` handles a symlink as a file, even when the symlink names a
directory. Homebrew therefore links each source alias as one entry under both
strategies, and it never descends into a payload. Both methods are in
[`keg.rb` at `3ccfa1d07143`](https://github.com/Homebrew/brew/blob/3ccfa1d07143/Library/Homebrew/keg.rb).
The [packaging record](../skill-discovery-design-evidence.md#packaging-record-of-a-default-source)
gives the measurement on the accepted build.

- Homebrew's cleaner prunes nothing in `libexec`. It sets file modes only
  under `bin`, `sbin` and `lib`. A payload therefore keeps its files and their
  modes. A dangling alias is an installation defect, and its entry refuses.
- Two formulae that install one source ID conflict at that entry. Homebrew
  then leaves the whole later keg unlinked, with its other sources and its
  executables. The owner resolves the ownership before the next launch. An
  overwrite that conceals the duplicate is not a remedy.
- The `examples`, `notices` and `installation.json` of Ravel's own formula are
  in `share/ravel`, outside the default skill directory. They are not sources
  and need no alias.
- Another package manager follows the same rule when it merges the
  directories of several packages.

This rule binds packages, not Ravel. The entry table and the source boundary
do not change. Ravel does not inspect how a package installed an entry.

### Changes under running sessions

A package manager changes the directory while sessions run. Homebrew unlinks
the earlier keg of a formula before it installs the new keg. It links the new
keg after the installation. During that interval, the sources of that formula
are absent from the directory.

- A resolution in that interval delivers the other sources and refuses nothing.
- A mandatory reference to an absent source refuses as `skill_source_missing`.
- A source that disappears after the listing and before its capture refuses as
  `skill_source_missing`.
- A rerun after the upgrade captures the new keg.

A change of the directory changes no existing snapshot, as
[local freshness](#local-freshness) states.

The default skill directory belongs to the installation of Ravel, not to a
`HOME`. A separate `HOME` gives separate settings and a separate cache, but the
same default sources. A `HOME` that must not receive them sets
`"sources": []`, or it names its own sources. A test can also run a copied
executable without a record.
[Configuration readers](#configuration-readers) states the effect on a `HOME`
that Ravel 1.1 also reads.

## Agent discovery and native installations

The agent lists the `SKILL.md` files beneath the supplied directories. It reads
their names and trigger descriptions. It opens applicable guidance before the
work that the guidance governs. After it finds a skill, the agent uses the
directory of that skill as the base of its resources. It does not search the
resources of a skill for nested skills, unless the pack tells it to.

Overlapping roots can show one physical path twice, and the agent can read it
once. A missing requested skill, a malformed description or an ambiguous name
is a problem of agent discovery. It is not a rule of source admission. Ravel
has no skill identity, name grammar, duplicate-name rule, lookup interface or
harness filter. Pack conformance can check these content properties separately.

An explicit loading mechanism names a concrete native tool, a command or a file
path. A skill name with a verb such as "load" is a name request. A name with a
plugin namespace, such as `grove:grove-design`, is also a name request. It does
not select a native loader. Ravel never interprets or translates a name. The
preamble gives the agent these rules:

| Request | Agent behaviour after the mandatory reads |
| --- | --- |
| A skill name, with or without a plugin namespace | Search the supplied directories first. Use native discovery when no supplied directory holds a match |
| A concrete native tool or command | Use that mechanism first. When it reports no native match, say so. Then search the supplied directories for the same guidance before you report it missing. When it reports another error, report that error in a visible reply to the human. Give that reply as its own step, before dependent work and before any ending command call |
| A concrete file path | Read that path. When the read fails, report the failure before dependent work. Do not use another location |
| An express restriction to one mechanism, such as "use only this native tool" | Obey the restriction. When that mechanism cannot load the guidance, report the limit in a visible reply to the human. Give that reply as its own step, before dependent work and before any ending command call. Do not fall back |

The preamble also states one rule about skill metadata. When the metadata of a
skill sets `disable-model-invocation: true`, the agent loads that skill only
when the caller names it, by name or by path. The agent never loads it because
its description matches the work. Ravel itself reads no metadata, so the agent
applies this rule.

The agent reports a native loader error other than "no match" before
dependent work and before any ending command call. The error is not permission
to go around that loader. A report in these rules is a visible reply that the
agent writes to the human. A tool result alone is not a report. Reasoning alone
is not a report either, because the human may not see it.

In a headless run, the visible text before the ending command is the only
report that the human sees. The report of a loader error or of a blocked
restriction is therefore its own step, before the ending command call.

Two different matching files in the supplied
directories need an explicit reference from the caller or the pack. Without
one, the agent reports the ambiguity before dependent work. Source order
chooses no winner.

A directory copy and a native copy of the same guidance are
not both loaded only because both exist. None of these rules replaces the
mandatory reads.

Ravel neither lists nor changes native installations. Directory guidance
cannot withdraw native text that a harness added already, and it cannot disable
native hooks. Trials record that limit. When native startup guidance
conflicts, the owner isolates or reconfigures the harness. The native-only
route stays valid when delivery is disabled. A source with duplicate skill
names or malformed optional metadata still resolves when its files satisfy
this contract.

## Sources and retained resources

A resolved source is a retained immutable copy of the whole configured source
directory. It is not a pointer into a mutable checkout. Ravel excludes the
reserved `.git` and `.jj` entries at every level, with their administrative
files. It includes every other regular file and directory, whatever an ignore
file says. A link that reaches a reserved entry refuses. That includes a
different spelling that a case-insensitive source filesystem resolves to it.

A pack chooses a narrow source directory. A project root with build
output is usually a bad source. Ravel runs no pack script, hook, submodule
update or Git LFS filter.

Ravel admits a relative symlink only when its complete chain of targets ends
inside the source boundary, with no cycle. The snapshot keeps the link. An
absolute, dangling, cyclic or escaping link refuses. Every nonregular
filesystem object refuses too. A source path can itself be a
symlink. Ravel canonicalises the source root once, then applies the boundary
to every captured entry.

An admitted link must also work in the written tree on the cache filesystem.
A link that the source filesystem resolves through another spelling of a name,
and the cache filesystem does not, refuses as `skill_resource_invalid`.

Each path must be exact UTF-8, with no NUL and no control character. Capture
copies bytes, so a hard link in a mutable source never backs a snapshot.

The cache filesystem can fail to keep two distinct source paths or their exact
spellings. Ravel then refuses as `skill_resource_invalid` before publication,
and it names the paths. That includes a collision of case or of Unicode
normalisation. Ravel never writes one entry over another, and it never
normalises a name. It checks the written tree against the source manifest,
including every directory segment. A name or a link target in the written tree
that is not exact UTF-8 matches no manifest entry.

A relative reference to a supporting resource resolves against the directory
of its skill. That includes a `../` reference that stays inside the source. The
snapshot keeps the layout, so the same reference works there. A pack-relative
reference outside the source is not supported, and the pack author must repair
it.

For a reference to another skill, the agent finds that skill through the
supplied directories or through the native route that the caller named. It then
uses the directory of that skill. It must not assume a sibling checkout. Ravel
does not parse every reference in prose, and it does not make external links
available. It guarantees reads of the captured resources. It grants no
execution or network access because a skill recommends a tool.

### Local freshness

Each independent resolution reads the current bytes of a local source, with
uncommitted changes, and checks the resulting resources. It does not reuse a
local snapshot because directory metadata or a VCS revision is unchanged. After
it checks a new capture, it can reuse a published snapshot with the same
content digest. Later edits to the local source, or its removal, do not change
an existing snapshot. A later resolution still refuses when the configured
source no longer exists.

The owner keeps a local source quiet during capture. Capture detects an
observable change of entry, type, identity, size or content during its scan. It
then refuses as `skill_source_changed`, with a remedy to try again. This is not
an atomic snapshot against every concurrent writer.

Each published manifest and descriptor describes exactly the bytes that Ravel
copied. It never pairs paths of the live tree with a different capture. Digests
before and after the capture, and controlled concurrent edits, test this
boundary. A successful retry does not prove atomicity against every writer.

### Sources outside Ravel

Ravel reads a local source directory. It does not fetch, install or update
the directory. Skill delivery runs no VCS command and makes no network
request, so it needs no Git executable and no network.
[Skill sources are local directories](../adr/skill-sources-are-local-directories.md)
records why.

A tool outside Ravel puts each source directory in place and chooses its
contents. A package manager is such a tool. A Homebrew formula exposes its
sources in the [default skill directory](#default-skill-directory) through
[source aliases](#packaging-a-default-source).

A checkout of a Git or jj repository is an ordinary local source.
Ravel captures its current files, with uncommitted changes, and leaves out the
reserved `.git` and `.jj` entries. Ravel reads no VCS revision and records
none. The descriptor identifies a capture by its resource digest and its
origin path.

Ravel does not initialise or update a submodule. A submodule directory in a
checkout holds ordinary files, and capture includes them without the reserved
entry of the submodule. Ravel captures a Git LFS pointer file as its literal
bytes, not as the payload that it describes. Ravel neither detects nor hydrates
such pointers, and its read guarantee covers only the captured file. A pack
that needs an LFS payload materialises it in the source directory.

When a source directory is missing, resolution refuses as
`skill_source_missing`. The owner's tool restores the directory outside the
failing session.

### Publication, concurrency and cleanup

The cache root is `$HOME/.cache/ravel/skills/v1`. It does not depend on the
working directory, `TMPDIR` or the location of the configuration script. The
settings and the cache are private to the account. A cache root that a replaced
or unsafe symlink reaches refuses as `skill_cache_unavailable`. Ravel does not
walk a substituted cache tree. Cache writes use held directory identities and
staging directories that only the owner can open.

Before each publication, the cache paths must still reach the held root and
each of its held directories. The staging name must still hold the staging
directory that Ravel wrote and checked, before and after the rename. A
replacement refuses as `skill_cache_unavailable`.

Ravel materialises and checks each snapshot privately. The identity of a
snapshot is a SHA-256 digest of a versioned, sorted and length-delimited
manifest. The manifest holds the relative paths, the entry types, the
normalised executable bits, the symlink targets and the content digests of the
regular files.

A snapshot keeps the difference between executable and nonexecutable files.
It removes write permission, set-ID bits and ownership. A snapshot cannot
acquire the protected configuration or a result channel through an overlapping
source boundary. Ravel does not scan prose for secrets, so the owner keeps
private content out of source directories.

Ravel publishes the complete source directory and its manifest with an atomic
operation that does not replace an existing object. A reader sees no object or
a complete checked object. An existing object with the same name wins, whether
a concurrent publisher or an earlier resolution wrote it. Ravel reads the
winner back in full before it removes its private copy, and it reuses the
winner only when the content agrees.

On Linux the operation is the `renameat2` system call with `RENAME_NOREPLACE`,
which Linux added in 3.15. On macOS it is `renameatx_np` with `RENAME_EXCL`. A
kernel or a cache filesystem without the operation refuses as
`skill_cache_unavailable`. A plain rename can replace an empty directory, so
Ravel does not use one in its place.

A winner whose content differs is corrupt, and the refusal is
`skill_cache_corrupt`. The remedy is to stop the readers of the object, then to
move it out of the cache or remove it, and to resolve again. Ravel does not
accept the object, and it does not repair it under its readers.

Ravel publishes the descriptor only after every source and every configured
path passes its checks. A failed launch can leave useful source objects. It
never leaves a partial descriptor, and it never launches with a partial source
set. No mutable `latest` pointer is part of a request.

[Skill resources outlive sessions](../adr/skill-resources-outlive-sessions.md)
owns the lifetime trade-off. Snapshots and descriptors stay after a refusal,
after a session or delegate exits, after a settings change and after an
upgrade. A later cache format uses a new directory and keeps the older paths.
Ordinary cleanup removes only the private staging objects of the resolver that
made them, and it reports each cleanup failure. It removes the contents through
the held staging directory. It removes the staging name only while that name
holds the same directory.

Ravel has **no automatic collector and no prune command**. Manual removal
requires that no affected reader runs. Removal of staging also requires that no
resolver of that staging runs. Removal gives up the guarantees of existing
readers. The next resolution captures the present source and publishes it
again. It never substitutes another object of the same source ID.

When storage is full, the refusal is
`skill_cache_unavailable` with this remedy. Read-only permissions prevent
accidents. They do not prevent deliberate changes by the same user.

## Directory descriptor and loading contract

One immutable `directories.json` records the prepared directory set. It holds
`schemaVersion: 1`, `sources`, `directories` and `mandatory`, and no skill
entries. The descriptor, the request and `skills resolve --json` use the same
source record. Every value of a source record is a string.

| Source-record field | Contract |
| --- | --- |
| `id` | The configured source ID |
| `type` | `local` |
| `resourceDigest` | The lowercase SHA-256 digest of the snapshot manifest, 64 hexadecimal digits |
| `snapshotRoot` | The absolute path of the retained snapshot directory |
| `originPath` | The canonical absolute source directory at capture |

A directory record holds `source` and the absolute `path`. A mandatory record
holds `source` and the absolute file `path`, in configured order. Directory
records follow the order of sources and roots. A repeated absolute path appears
once, at its first position. Every path points into a complete retained
snapshot.

Source provenance stays per configured source, also when two sources share
stored content.

The identity of the descriptor is the SHA-256 of its versioned canonical JSON
bytes. Its retained directory has the name of that digest. The descriptor holds
no identity or path of its own. It holds no caller task, run identity,
session-result context, credential or configuration environment.

The descriptor records resource locations for recovery. It is not a skill
catalogue. Ravel generates no `entry.md`, no trigger list and no name index.
The manifest format and the cache layout stay internal. The public contract is
the schema and stable readable absolute paths.

### The preamble

The preamble holds the search directories, the ordered mandatory file paths,
the descriptor path and the obligations below. Each path is a JSON-quoted UTF-8
literal. It is not a shell snippet or markup to evaluate. The preamble tells the
agent:

1. Read the mandatory files in order, with their required supporting reads.
   Do this before optional discovery and before task actions. The descriptor
   need not be read first, because the preamble supplies the same paths.
2. Find and load relevant skills by the rules of
   [agent discovery](#agent-discovery-and-native-installations). That includes
   the skills that the caller names. Read supporting resources relative to the
   skill directory when they are necessary. Do not read every skill in advance.
3. Report a missing or unreadable resource, or an ambiguous reference, before
   dependent work. Do not use another revision. Do not claim that a supplied
   directory proves that guidance exists or that the agent read it.
4. Keep the descriptor path in a continuation or compaction summary. A
   re-entry is a compaction or a fresh context. A resume that keeps the whole
   earlier conversation is not a re-entry and needs no re-read. The guidance
   stays in that context. On re-entry, recover the same directories
   and mandatory paths. Then read the mandatory and relevant guidance again
   before work continues. Do not acquire newer sources during a session.
5. Give each delegate the descriptor path, these obligations and read access
   to the resources. Omit the ending instructions of the parent, and forbid the
   delegate to send the result of the parent. Each delegate reads the mandatory
   guidance first, also when the parent read it already. An omitted instruction
   does not remove inherited process authority, as
   [read access and delegation](#read-access-and-delegation) states.

Ravel delivers instructions. It does not observe what an LLM reads, and it does
not control compaction.

### `RAVEL_SKILL_DIRECTORIES`

When delivery is enabled, Ravel sets `RAVEL_SKILL_DIRECTORIES` to the absolute
descriptor path in each managed-process environment that it builds. Those are
the environment of the managed process and the environment of each harness run
of the evaluation. The value replaces a stale inherited value.
When delivery is disabled, Ravel removes the name from those environments.

The name is absent from the Rhai worker. It grants no read permission and no
ending authority. Ordinary children inherit
it. A nested `start-session` changes it only for its own managed process. An
adapter whose delegates do not inherit the environment forwards the descriptor
path itself.

The name is in the reserved `RAVEL_*` class of the
[settings and environment grants](configuration-evaluation.md#settings-and-environment-grants).
No `configEnv` grant can name it, so `ravel::env` never reads it. A script
reads `request.skills` instead.

Compaction can lose the prompt path but keep the recovery instruction. The
agent can then read the variable and reach the same descriptor. When both path
channels are unavailable, the agent reports missing discovery context. It does
not guess a latest path or resolve again.

Ravel has no durable instruction channel into a harness. Compaction can also
remove the whole preamble, with its instruction to read the variable. The
retained variable alone does not prove recovery in that case. Actual harness
trials measure both cases. A fixture proves only delivery and readability.

### `ravel skills resolve`

`ravel skills resolve` is a **source resolution** command. It evaluates no
configuration script. It takes no skill name, task, kind, prompt, source
override or ending token. It reads the settings and runs the same capture and
checks as a launch. It imports no configuration script, creates no result
channel and launches no harness. It is permitted inside a harness run, because
it selects nothing.

On success, its default output is the preamble that a launch would add, with a
final newline. With delivery disabled, its output is empty and its status is 0.
`--json` writes one object with `schemaVersion: 1` and `status`. The status is
`resolved` or `disabled`. A resolved object also holds `descriptorPath`,
`sources`, `directories`, `mandatory` and `instructions`, which is the same
preamble. A disabled object holds only `schemaVersion` and `status`.

A failure goes to standard error with a nonzero status, never as empty success.
With `--json`, a failure is a failure document with the stage, the stable code,
the source or location where it applies, the message and the remedy.

| Status | Meaning |
| --- | --- |
| 0 | Resolved, or disabled |
| 2 | A command-line or settings problem |
| 3 | A skill refusal |
| 5 | An internal failure, such as `skill_cleanup_failed` |
| 124 | The resolution deadline passed |
| 128 plus the signal number | INT, TERM or HUP cancelled the resolution |

An independent harness can receive that preamble explicitly. The command cannot
change the environment of its caller, and it installs no hook. Each run captures
current local bytes. A failure can leave complete useful cache objects. It
prints no preamble and no partial JSON.

The help text shows the plain and JSON forms and the disabled result. It
explains the cache side effect. Source resolution is optional setup. It is never
a prerequisite of a caller such as Grove.

## Request and configuration script

The script and request schema is **4**. Schema 4 adds `originalPrompt` and
`skills`, and it changes `prompt` to the text prepared for delivery. The
[configuration evaluation specification](configuration-evaluation.md#request-context-and-selection)
owns the request map. This section owns the meaning of these three fields.

| Field | Contract |
| --- | --- |
| `originalPrompt` | The exact UTF-8 bytes of `--prompt` or `--prompt-file`, including empty text and every newline |
| `prompt` | The prepared launch text. With delivery disabled, it equals `originalPrompt` byte for byte |
| `skills` | `()` with delivery disabled. Otherwise a map with exactly `descriptorPath`, `sources`, `directories` and `mandatory`, as in the descriptor. It holds no prompt offsets |

Every other field keeps its schema-3 meaning, and `limits` does not change.

### The prepared prompt

With delivery enabled, `prompt` is the preamble, then the label line
`Caller task`, then one delimited block that holds the original bytes. The
opening line is `BEGIN RAVEL CALLER TASK <nonce>`. After the original bytes,
Ravel writes one newline and the line `END RAVEL CALLER TASK <nonce>`. That
newline belongs to the wrapper, whether or not the original bytes end with a
newline.

The nonce is 32 lowercase hexadecimal digits from the random source of the
operating system. Ravel checks that the caller bytes do not contain the nonce,
and it chooses again when they do.

Ravel inserts the original bytes once, contiguously. It adds no trimming,
indentation, escape, newline conversion or template expansion, and it changes
no byte. The label, the delimiters and the separators are outside the caller
byte range. Preparation records that range internally and checks it. An empty
caller task gives an empty block, and the prompt still supplies discovery.

Ravel does not recognise or parse ending commands, so every command in the
caller block stays exact. The label and the delimiters belong to the launch
wrapper. They are not part of the preamble that `skills resolve` prints.

Within the delivered prompt, the preamble sets exactly one precedence rule.
Mandatory reads come before caller-block actions. That includes a request of
the caller to load a skill "first". A caller qualifies such an instruction with
"after any mandatory guidance".

This order stays subject to harness instructions of higher priority. It does
not raise the preamble to system authority. It gives configured content no
other priority over the intent of the caller, and the task stays the caller's.

### Evaluation order and reads

The Ravel CLI prepares the request before it launches the worker. `load_context`
and `select` each receive a copy of the same request value. A change that
`load_context` makes to its copy therefore does not reach `select`. Top-level
module code receives no request.

A script that analyses the task reads `request.originalPrompt`. A script puts
`request.prompt` into the arguments of the managed harness. A script that asks
a model a question in a harness run chooses the prompt of that run. It passes
`originalPrompt` when the run must not receive skill guidance.

The descriptor is request data from Ravel. It is not measured selection
context, and it is not session-result authority. The selection-context schema
stays 1. Ravel adds no skill text to the context of the caller, and it makes no
measured source for it.

Ravel adds no host function for skills. A script that reads the descriptor or a
snapshot file uses `ravel::read_text` or `ravel::read_json`. That read is an
ordinary measured read, with its read bound, its source count and its notice
record. A read in `load_context` enters the measured list of the delivered
context. The descriptor can be larger than the default read bound, and
`request.skills` already holds its content. The shapes of a selection and a
refusal do not change.

### Diagnostics

The JSON routing notice gains the member `skills`. It is `null` with delivery
disabled. Otherwise it is an object with these members:

- `descriptorPath` and `resolutionMs`.
- `defaultDirectory`: the absolute default skill directory when it supplied the
  source set, otherwise `null`.
- `sources`, with one record for each source. A record holds `id`, `type`,
  `resourceDigest` and `disposition`. The disposition is `captured`.
- `originalPrompt` and `prompt`. Each holds `bytes` and `sha256`.

No diagnostic shows a prompt, a skill body, a credential or an environment
value. The text routing notice does not change. Successful standard output
remains only the opaque result. Diagnostics do not become a persistent record
of a session.

## Bounds

All byte bounds count UTF-8 or raw file bytes, not characters. The skill bounds
are public refusal boundaries, not truncation rules. The byte and count bounds
are constant, and the owner sets the resolution deadline.

| Subject | Bound |
| --- | --- |
| Sources, configured or default | 16 |
| Search directories / mandatory files | 256 / 256 |
| One mandatory file | 65,536 bytes |
| One regular supporting resource | 16 MiB |
| One source / all sources, in regular-file bytes | 128 MiB / 512 MiB |
| Entries in one source, with directories and links | 20,000 |
| Descriptor / preamble | 1 MiB each |
| Original prompt / prepared prompt | 1 MiB each |
| One resolution, with capture, checks and publication | `resolutionTimeoutMs`: 120,000 ms by default, from 1,000 to 3,600,000 |

A breach of a byte or count bound refuses as `skill_limit_exceeded`. The
refusal reports the actual value, the bound and the subject. A value equal to
a bound passes. The search-directory bound counts the roots of all sources,
before the descriptor joins repeated paths. Ravel checks the three counts
before capture, after it lists the default skill directory.

Capture stops at the first entry or file that passes a bound. A refusal of the
entries or the bytes of a source therefore reports the count at that point as
"at least" that value. The bytes of all sources count each configured source,
also two that name one directory.

Ravel checks the descriptor and the preamble before it publishes the
descriptor. For the prepared prompt, it reports both prompt byte counts. It
suggests a smaller task with references to supporting files. Ravel never
truncates caller instructions, omits a configured directory, omits mandatory
guidance or disables delivery to meet a bound.

The prepared prompt travels to the worker in the evaluate frame. It returns in
the script result when the script puts it into `args`.
[Frame bounds](configuration-evaluation.md#frame-bounds) owns both bounds.
The 1 MiB prompt bound does not promise that a selection with that prompt fits
the script-result bound or the argument limit of the platform.

## Failures

| Code | Stage | Exit | Meaning and remedy |
| --- | --- | --- | --- |
| `settings_invalid` | `cli` | 2 | A malformed `skills` object, a source `type` other than `local`, a mandatory source ID that `sources` does not name or a repeated member name. Repair the settings at the reported `location`. A source `type` is `local`. Make any checkout of a repository with a tool outside Ravel |
| `skill_source_missing` | `skills` | 3 | A local source directory or a configured root does not exist. Or a mandatory reference names a source that the default skill directory does not hold. Restore or install the source, or change the settings deliberately |
| `skill_directory_invalid` | `skills` | 3 | The installation record is invalid, or an entry of the default skill directory is not a source. Remove or repair the entry, or reinstall the package that installed it |
| `skill_source_changed` | `skills` | 3 | A local source changed during capture. Keep it quiet and try again |
| `skill_mandatory_invalid` | `skills` | 3 | A mandatory file is missing, unreadable, nonregular or not UTF-8, or two references reach one retained file |
| `skill_resource_invalid` | `skills` | 3 | A link, an entry or a path that the snapshot cannot keep exactly |
| `skill_cache_corrupt` | `skills` | 3 | A cached object fails its checks. Stop its readers, move it out of the cache or remove it, then resolve again |
| `skill_cache_unavailable` | `skills` | 3 | Ravel cannot use the cache root, because of an unsafe link, a permission or full storage |
| `skill_limit_exceeded` | `skills` | 3 | A count or byte bound of this specification |
| `skill_access_unavailable` | `skills` | 3 | A source directory overlaps a protected path: the configuration directory, the settings file, the skill cache or the result channel of an enclosing session. Move the source outside each of them |
| `skill_resolution_timeout` | `skills` | 124 | The resolution deadline passed. Raise `resolutionTimeoutMs`, or address the latency of storage |
| `skill_resolution_cancelled` | `skills` | 128 plus the signal number | INT, TERM or HUP arrived during resolution |
| `skill_cleanup_failed` | `skills` | 5 | Ravel cannot remove its staging |

The failure document gains the stage `skills`. It covers the capture, checks
and publication of skill sources and the prepared prompt. A settings problem keeps the stage `cli`.

Each refusal names the exact source or path that its remedy addresses. No
remedy offers another snapshot or directory as a substitute for a configured
source. A timeout reports the effective deadline and the active operation.

`skill_access_unavailable` does not claim to detect a later failure of the
agent to read. No failure in this table gives a successful result. Each
refusal launches no worker and no managed process.

After a consumed INT, TERM or HUP during resolution, Ravel removes the private
staging. The refusal is `skill_resolution_cancelled`, and Ravel raises the same
signal again.
A consumed signal wins over a resolution refusal, as in the
[completion contract](result-job-completion.md#outcome-and-diagnostic-order).
The report then keeps that refusal as the original failure.

## Read access and delegation

The managed process and each harness run can read the descriptor and every
captured resource, also outside the working directory. Ravel publishes them at
stable absolute paths that the account of the owner can read. Each snapshot is
read-only by its file modes. Ravel applies no operating-system confinement, as
[Ravel does not confine a session](../adr/ravel-does-not-confine-a-session.md)
records. The harness sandbox decides what the agent can read and write.

A harness sandbox that limits reads must admit the descriptor directory and the
snapshot roots. The configuration script builds that access from
`request.skills`, for example one `--add-dir` argument of Claude Code for each
`snapshotRoot`. Codex in `--sandbox workspace-write` reads every path, so it
needs no addition. Ravel does not inspect the arguments of a harness, and it
cannot detect a sandbox that denies the reads. The agent then reports an
unreadable resource, as the [preamble](#the-preamble) requires.

Ravel refuses a source boundary that overlaps the configuration directory, the
settings file or a known session-result channel. That includes an overlap
through a canonical alias. The inherited `RAVEL_EXIT_FILE` of an enclosing
session is a known channel. Ravel checks each source before capture.

`skills resolve` protects the default configuration directory
`$HOME/.config/ravel`. A launch also checks each source against the directory
of its actual `--config` entry, before capture. Every launch captures its
sources again.

A source that holds the cache root refuses too, also before the cache exists.
Ravel checks the real path that the opening of the cache would reach. A source
that held the cache would capture its own staging.

These checks keep the configuration of the owner and the result channels out of
published skill content. Agents read that content, and it outlives the session.
The checks cover the explicit overlap of a source with a protected path. They
do not cover secret text that an owner puts inside a source. Each refusal in
this section is `skill_access_unavailable`.

Ravel does not check the working directory or the harness sandbox against the
cache. A process that can write inside the cache can change a snapshot after it
changes the file modes. The cache does not authenticate against code of the
same user, as [publication, concurrency and
cleanup](#publication-concurrency-and-cleanup) states. A later resolution of
the same content then refuses as `skill_cache_corrupt`.

The harness and its ordinary children read the same paths, under the same
harness sandbox. A delegate can run in a separate sandbox, a remote worker,
another account or another filesystem namespace. Its adapter then forwards the
same descriptor and directory set, maps the same absolute paths and gives the
delegate read access to them.

When it cannot, it reports unsupported delegation before the delegate acts. A
copy of the task alone does not satisfy this contract. Nor does a descriptor
whose paths do not resolve. Ravel claims nothing for an adapter that nobody
tested.

The descriptor and the resource trees hold no result path or capability. Ravel
gives its managed harness a result context of its own. It does not intercept
the child creation of the harness. An ordinary delegate can inherit
`RAVEL_EXIT_FILE` and access to the result directory. The loading instructions
forbid that delegate to use them, but Ravel does not withdraw the inherited
authority.

[Ravel supervises the session](../adr/ravel-supervises-the-session.md) records
this residue and its reopen condition. The first-writer rule prevents the
replacement of a result. It does not prevent an early or wrong first result.

An adapter can control the environment and the filesystem access of a
delegate. Such an adapter removes `RAVEL_EXIT_FILE` and the completion context
of the caller. It withholds the launch descriptors and the result directory of
the parent.

It forwards the descriptor path and the resource reads separately,
with `RAVEL_SKILL_DIRECTORIES` where the delegate supports it. A trial measures
that isolation for that adapter. A fixture adapter is no evidence for a
delegate inside the harness process. Neither scrubbing nor instructions
authenticate against other code of the same user.

An explicit nested `ravel start-session` in a managed process is a new session.
It resolves again when it can read its own settings and configuration script
and write its cache. It replaces the result context only for its own managed
process. The parent keeps its directories and its channel.

A harness sandbox can deny the reads or the cache writes of a nested launch.
The nested launch then refuses with the code of the failed read or write, such
as `skill_cache_unavailable`. A harness delegate can reuse the retained
descriptor without a nested launch. A `start-session` inside a harness run
still refuses as `config_reentry_refused`.

## Compatibility and transition

This feature changes the promise that `request.prompt` is the caller's prompt,
unchanged, and that Ravel adds no instructions. It ships inside the combined
Ravel 2.0.0, with script and request schema 4.
[Skill preparation precedes evaluation](../adr/skill-preparation-precedes-evaluation.md)
records why.

### Schemas and identities

- Ravel 2.0.0 reads `SCHEMA_VERSION` 4 only. A script that exports 3 refuses as
  `unsupported_version`, exit 3, also with delivery disabled. The remedy names
  both changes: export 4, and read `request.originalPrompt` wherever the script
  analyses the task. Schema 3 was never published, and Ravel does not
  reinterpret it.
- With delivery disabled, a schema-4 script receives both prompt fields and
  `skills` is `()`. The launched prompt equals the caller's prompt byte for
  byte.
- The worker protocol stays 1. The Ravel CLI and the worker are one executable.
  The handshake already pairs the package version, the build identity and the
  Rhai version. A change to the request code changes the build identity, so a
  mismatched pair refuses as `worker_identity_mismatch`. The protocol number names the
  frame grammar and the message types, which do not change.
- The selection-context schema stays 1. The routing notice and the failure
  document keep `schemaVersion` 2. The `skills` member and the `skills` stage
  are part of their first published 2.0.0 shape.
- `end-session`, the result protocol, exact ending commands and open kind tokens
  do not change. `end-session` reads no settings and no configuration script.

### Configuration readers

| Reader | Configuration file | Settings keys it accepts | Refusals |
| --- | --- | --- | --- |
| Ravel 1.1.0 `start-session` | `policy.ts` | `timeoutMs`, `contextBytes`, `policyEnv` | Any other key, such as `skills`, `configEnv`, `harnesses` or `budgets`, refuses as `settings_invalid`, stage `cli`, exit 2, before evaluation |
| Ravel 2.0.0 `start-session` | `config.rhai`, with `SCHEMA_VERSION` 4 | `timeoutMs`, `contextBytes`, `configEnv`, `harnesses`, `budgets`, `skills` | `policyEnv`, any other key and a repeated member name refuse as `settings_invalid`, exit 2. Schema 3 refuses as `unsupported_version`, exit 3. A lone `policy.ts` refuses as `config_format_retired`, exit 3 |
| Ravel 2.0.0 `skills resolve` | None | The keys of 2.0.0 `start-session` | The settings refusals of 2.0.0 `start-session` |
| `end-session` of either release | None | None | None from settings |

The two configuration files coexist under one `HOME`, but the settings file
does not. A settings file that both releases accept holds only `timeoutMs` and
`contextBytes`. Different executable prefixes do not isolate it. Nor does
`--config`, because both releases read the settings from `HOME`.

Use of 1.1 and of 2.0 with delivery at the same time therefore needs a separate
`HOME` for 2.0. The alternative is shared settings without 2.0 keys until
no 1.1 reader remains. Ravel never solves a major-version refusal by an
upgrade that nobody directed. A running session keeps the canonical ending
executable of its launch and its resource paths through upgrades, source moves
and settings changes.

A shared `HOME` also cannot hold `"sources": []`, because Ravel 1.1 refuses the
`skills` key. A 2.0 launch there delivers each source of the default skill
directory. After packages install sources, Ravel 2.0 without delivery beside a
1.1 reader therefore needs a separate `HOME` with `"sources": []`.

### Caller migration

A caller migrates its configuration script in four steps:

1. Export `SCHEMA_VERSION` 4.
2. Pass `request.prompt` to the managed harness, as before.
3. Read `request.originalPrompt` wherever the script analyses the task, for
   example in an LLM call that classifies it.
4. Build the read access of each harness sandbox from `request.skills`, for
   example one `--add-dir` argument of Claude Code for each `snapshotRoot`.

No helper for each script exists. The installed examples, the book and the
release notes migrate together. A caller that checks the major version of Ravel
admits 2.0.0 explicitly before it uses this route. Ravel 2.0.0 is necessary for
directory delivery. It is not necessary for native skill availability. Neither
release resolves a skill that a caller names.

For acceptance, install 2.0 with a separate prefix and select its executable
explicitly. Existing callers keep 1.1. Publication does not authorise a switch
of the shared installation.

First make compatible callers and configuration
available. Then switch each launch environment and activate its skill settings
together. A new settings key makes an older release refuse, so activate it
only after the compatible pair is installed. Installed skill sources activate
delivery at the first 2.0 launch, with no settings change. Grove's
coordinated-release design owns the order between the products.

Ravel ships no skill. The default skill directory holds only what packages
install. This feature adds no native installer, skill execution engine,
caller-kind registry or provider integration. It adds no task parser,
result-token interpreter or persistent session database. The retained content
cache is the one new persistence surface. The feature manages no marketplace,
fetches no source, removes no native hook, runs no source code and edits no
settings.

## Test seams and acceptance

Use the public `ravel skills resolve` and `start-session` boundaries. Supply
temporary `HOME`, working, cache and configuration directories and local
source fixtures. A fake managed program reads the delivered prompt, the
descriptor and the resources. It sends an opaque result through its own
channel.

Configuration-script fixtures record what `load_context` and `select` receive.
Fault injection stays behind internal seams. Tests also use real cache
filesystems.

On macOS, `task smoke:pair OLD=<1.1 executable> NEW=<prefix>` runs the
installed-pair part of "Version transition".

| Case | Observable acceptance at the public seam |
| --- | --- |
| Disabled delivery | A missing settings file and an absent key do no cache work in three cases: no record, no default skill directory and an empty one. `"sources": []` with no mandatory reference does no cache work beside a populated default skill directory. Empty, newline and Unicode prompts stay exact. `skills` is `()`, and `prompt` equals `originalPrompt`. Native-only Grove sessions need no directory setup |
| Default skill directory | A copied executable has a record that names a fixture directory, with Homebrew-shaped relative links and real directories. With no settings, it delivers each source with the roots that exist, in byte order of the IDs. A dot entry is ignored. Each refusing row of the entry table refuses as `skill_directory_invalid`, and no worker launches. A malformed record refuses the same way. Settings that name `sources` launch beside a malformed record, because Ravel does not read it. A mandatory reference to an absent default source refuses as `skill_source_missing`. So does a source that a controlled hook removes after the listing. The routing notice names the directory |
| Installation record | The installed test of each formula channel reads the record that the formula wrote. It names `HOMEBREW_PREFIX/share/ravel/skills`. With a fixture source in that directory, the keg-only `ravel@2` resolves it with no settings. A Linux installation passes the same test under its own prefix |
| Directory delivery | Local roots outside the working directory appear as absolute retained paths in the preamble. The order of sources, roots and mandatory files agrees with the descriptor and the request |
| Filesystem versus skill semantics | Empty roots, duplicate skill names, invalid optional frontmatter and unusual directory names resolve and launch. A fixture agent reports missing or ambiguous guidance, and Ravel chooses no skill |
| Settings and bootstrap | A mandatory source ID that `sources` does not name refuses as `settings_invalid`, exit 2, before capture. So do a repeated member name, a malformed path and an unknown field. A missing or non-UTF-8 mandatory file refuses as `skill_mandatory_invalid`, exit 3. A missing source or root refuses as `skill_source_missing`. No worker launches |
| Local sources only | A record with `type: "git"` and valid Git fields refuses as `settings_invalid`, exit 2, at `skills.sources[N].type`, with the checkout remedy. That happens at both public boundaries, with no cache object and no worker. A spy `git` first on `PATH` sees no call while a checkout with uncommitted changes resolves and launches, and the snapshot holds those changes |
| Relative resources | The managed program reads sibling and parent-relative files from the whole source. Admitted links work. Escaping, dangling and cyclic links and special files refuse |
| Path identity | Paths that differ only in case or normalisation refuse when the cache filesystem cannot keep them. A filesystem that keeps them keeps both. No entry is written over another or normalised |
| LFS pointer bytes | A pointer file in a local checkout is captured byte for byte, with no contact with LFS storage. A materialised payload is captured as ordinary bytes |
| Local freshness | A content change with no VCS change gives a new snapshot. Old readers keep old bytes. Controlled concurrent edits refuse with no partial descriptor |
| Corrupt object | A tampered snapshot refuses as `skill_cache_corrupt`, with its remedy. No worker or job launches, and the object stays as it was |
| Concurrent publication | Publishers behind a barrier and an existing reader see no partial object. The losing publisher checks the winner. A killed publisher cannot publish a partial descriptor. The rename refuses an existing empty directory. A platform without that rename refuses as `skill_cache_unavailable` and leaves no object |
| Lifetime | Independent and detached readers keep their directories and resources after the parent exits, the source goes and the settings change. Ordinary managed descendants are reaped with the group. After a quiet manual removal, the next resolution publishes the present source again and uses no neighbour object |
| Request and evaluation | `load_context` and `select` receive the same request, and a change to the copy in `load_context` does not reach `select`. Top-level code sees no request. The caller byte range is exact for non-ASCII text, every line ending and text that resembles a delimiter. `skills` has no offset fields. The label and delimiters occur only in the wrapper. A quoted path cannot become prompt structure |
| Host reads | A read of the descriptor with `ravel::read_text` is a measured read in the notice. `ravel::env("RAVEL_SKILL_DIRECTORIES")` throws `env_not_granted`, and a grant of that name refuses as `excluded_grant` |
| Harness runs | A fake harness in a harness run sees this session's descriptor in `RAVEL_SKILL_DIRECTORIES` when delivery is enabled, and no variable when it is disabled. It reads the resources outside its working directory |
| Bounds and frames | Enabled delivery with an empty task still bootstraps. Each count and byte bound refuses at its limit and passes below it, with controls that remove the bound. The three tests of [frame bounds](configuration-evaluation.md#frame-bounds) pass where their platform admits them |
| Slow valid source | A controlled delay times out with the effective deadline and the settings remedy. A larger valid deadline succeeds with every check. A malformed deadline refuses |
| Cancellation | INT, TERM or HUP during a controlled capture refuses as `skill_resolution_cancelled`, exit 128 plus the signal number. No staging stays, and Ravel raises the same signal again |
| Read access | A child and its descendants read resources outside the working directory. A write to a snapshot file fails by its file mode, and a snapshot directory admits no new entry. A source that overlaps a protected path, also through an alias, refuses as `skill_access_unavailable`, exit 3, before capture |
| Removed confinement flags | `--confine` and `--runtime-read`, alone, together, in the `=` form and with `--json`, refuse as `confinement_withdrawn`, stage `cli`, exit 2. No settings read, cache work, worker or result channel follows. `--prompt --confine` launches with that text as the prompt |
| Delegate authority | Disposable trials tell inherited context that can send an early result from an adapter whose scrubbed delegate cannot. First-writer behaviour has its own check. Each trial names its adapter |
| Directory recovery | With the instructions retained, the prompt path is lost and the agent recovers the same descriptor through `RAVEL_SKILL_DIRECTORIES`. Stale values are replaced, and disabled delivery removes the name. A nested child alone receives a new value. Loss of both path channels gives a report of missing context |
| Nested sessions | A nested resolution sees current sources and returns only its own result. The paths and channel of the parent stay intact |
| Version transition | Schema 3 refuses with and without sources. Schema 4 works with and without sources. A mismatched pair refuses. Ravel 1.1.0 refuses shared settings that hold `skills`, exit 2, and separate prefixes cannot prevent it. Ravel 2.0.0 refuses `policyEnv`. An isolated `HOME` permits 1.1 native-only and 2.0 directory launches at the same time. In a shared `HOME` without `skills`, a 2.0 launch delivers the default sources. Existing ending commands stay usable |
| Standalone resolution | No configuration script, channel or harness. The plain preamble and the JSON describe the same directories as a launch. Disabled and failed outputs differ. Retries keep only complete useful objects |

### Actual harness trials

Actual Claude Code and Codex trials are separate acceptance evidence. They run
in a sandbox home, which the
[configuration evaluation specification](configuration-evaluation.md#sandbox-home-for-real-harness-runs)
defines. The sandbox isolates native skill roots and leaves the owner's
installations unchanged.

A task activates a craft skill by its activity, without naming it. A trace
shows the listing or the description inspection, then the read of the skill's
absolute path, **before** the governed action. The trials observe the
mandatory order, a supporting-resource read, compaction and re-entry, and one
delegate. They include missing guidance, ambiguous directory matches and a
stale native copy.

The trials exercise every row of the
[agent discovery rules](#agent-discovery-and-native-installations). They use a
bare name and a name with a plugin namespace with native copies absent. They use
a native loader with a match and without one, an express native-only
restriction and a concrete file path. Each observation matches its row, and
Ravel chooses no skill. A skill with `disable-model-invocation: true` stays
unread until the caller names it.

The caller gives a "first" instruction qualified by "after any mandatory
guidance". The trial observes the mandatory reads before that caller action.

For compaction, a trial first loses the path while the instructions survive.
It then loses the whole preamble, with its recovery instruction. In that case
the environment variable stays available, and the trial gives no new reminder.
The trial observes whether the agent recovers the descriptor and reads the
mandatory guidance again before it acts. A failure is an explicit acceptance
gap. A prompted retry is not unprompted recovery, and the retained variable is
no evidence of success.

Each trial records the harness, model and version, and the native isolation.
It records the provenance of the selected directories, the delegate mechanism,
the sandbox mode of the harness and the order of reads and actions. It records native startup
text that the harness added already. It records the inherited ending context of
each real delegate and whether the delegate obeyed the instruction not to end.

Result attempts belong only in disposable trials. A fixture program cannot
prove LLM compliance. A prompt transcript cannot prove permissions. An unavailable
compaction or delegate mechanism is an explicit gap in the measurements. Grove checks its native-only compatibility separately from these
trials.

## Out of scope

- A skill catalogue, a name resolver or a lookup command.
- Automatic configuration of harness sessions that the owner opens
  independently.
- A garbage collector or a prune command.
- Management of native installations, hooks or marketplaces.
- A host function for skills.
- Fetch, installation or update of a source, and every VCS command.
- Git LFS hydration and submodule updates.
- A source override for one launch.
- Roots of a default source other than `skills` and `craft`, and a descriptor
  that would name them.
- A list of required default sources, and explicit sources beside the default
  ones.
- Operating-system confinement of a session.
  [Ravel does not confine a session](../adr/ravel-does-not-confine-a-session.md)
  records why.
