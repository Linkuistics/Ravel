# Skill discovery

Ravel resolves the skill sources that the owner configures. It adds their
directory paths and loading instructions to the prompt before the
configuration script runs. The agent finds the applicable skills in those
directories and reads them. Ravel does not resolve a skill that a caller names.
The caller still supplies its task, its conventions and its exact ending
instructions.

## Status and ownership

This specification states the directory-delivery contract of the combined
Ravel 2.0.0. It is the contract of the `skill-directory-delivery` line. Ravel
`main` does not hold this feature. Implementation and acceptance on that line
remain ahead. The [design evidence](../skill-discovery-design-evidence.md)
records the source observations, the exact base of the line and the handoff.

Each rule has one owner:

- This specification owns the `skills` settings object, source acquisition,
  the retained resources and the directory descriptor. It also owns the
  prepared prompt, `ravel skills resolve`, `RAVEL_SKILL_DIRECTORIES`, the skill
  bounds and failures, the skill read grants and the acceptance cases of this
  feature.
- The [configuration evaluation specification](configuration-evaluation.md)
  owns the request map and the settings file as a whole. It also owns the host
  capabilities, the Rhai worker, its frames and the evaluation bounds. It lists the schema-4
  fields and links here for their meaning.
- The [session execution specification](session-execution.md) owns the
  managed process, the result channel and confinement in general.

Three decision records bind this feature.
[Skill preparation precedes evaluation](../adr/skill-preparation-precedes-evaluation.md)
records the boundary and the schema change.
[Skill resources outlive sessions](../adr/skill-resources-outlive-sessions.md)
records retention.
[Ravel supervises the session](../adr/ravel-supervises-the-session.md) records
the inherited authority of delegates. The
[visual views](../design/skill-discovery/index.html#discussion) show the flow.

## Ownership and order

The Ravel CLI resolves the sources, checks the resources and publishes the
snapshots and the descriptor. It prepares the prompt and builds the read
grants. The Rhai worker receives a complete request. It finds no skill and
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
3. It resolves the skill sources and prepares the prompt. A disabled
   configuration does nothing in this step.
4. It prepares confinement, with the same resource set.
5. It launches the Rhai worker. The whole evaluation deadline runs from this
   launch. The worker imports the entry, then runs `load_context` and
   `select`.
6. It allocates the result channel, writes the routing notice and launches the
   managed process.

A failure in step 3 prevents every later step. Step 3 creates no result channel
and launches no worker. The order in
[configuration evaluation precedes the launch](../adr/configuration-evaluation-precedes-the-launch.md)
still binds.

| Boundary | Guarantee | Decisions hidden from its consumer |
| --- | --- | --- |
| Source resolver | A complete, checked and immutable resource set, or a refusal | Local capture, exact Git acquisition, staging and publication |
| Prompt preparation | The original task, intact, with the directories and loading instructions | Path rendering, the block delimiters and byte accounting |
| Request | The original prompt, the prepared prompt and the descriptor fields | Source I/O and cache coordination |
| Managed launch and harness runs | Read access to the exact published resources under their confinement | Read grants that are specific to a backend |
| Harness delegation | The same directories and readable resources. Ending restrictions depend on the adapter | Delegate construction and its measured isolation |

Ravel gives no meaning to the domain of a skill, the kind of a caller, provider
receipts, task trees or result tokens. Skill text is guidance that the owner
selected. It is not configuration authority, and it cannot weaken confinement.

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

`skills` accepts exactly `schemaVersion`, `sources`, `mandatory` and the
optional `resolutionTimeoutMs`. `schemaVersion` is the integer 1. `sources`
and `mandatory` are required arrays. An unknown key, a duplicate source ID or
a malformed value refuses as `settings_invalid`, exit 2. Its `location` names
the value, such as `skills.sources[2].revision`. The other settings keys keep
their meanings.

`resolutionTimeoutMs` is an integer from 1,000 to 3,600,000, and its default
is 120,000. It bounds one complete resolution: capture, cache checks, transport
and publication. It is independent of `timeoutMs`, which bounds evaluation. No
flag overrides it, and no value removes the bound.

Diagnostics report the
effective deadline. On a timeout they report the active source and operation.
An owner raises the bound for valid sources on slow storage or a slow transport.

Ravel examines the deadline and the cancellation latch between operations: at
each entry, at each block of a copy and at each entry of a check or a seal. It
cannot interrupt one system call that storage holds. The refusal then comes
after that call returns.

Each source record has these fields. An unknown field refuses.

| Field | Meaning |
| --- | --- |
| `id` | A unique owner token of 1 to 64 lowercase ASCII letters and digits, in groups that single hyphens separate. Its first character is a letter |
| `type` | `local` or `git` |
| `path` | Local only. An absolute UTF-8 directory path, with no tilde or environment expansion |
| `url` | Git only. An HTTPS, SSH or `file://` URL, or the absolute path of a local repository |
| `revision` | Git only. A full lowercase commit object ID of 40 or 64 hexadecimal digits, in the object format of the repository. A ref, a tag, an abbreviated ID and a revision expression refuse |
| `subdir` | Git only. A normalised source directory, relative to the repository root. `.` means the root |
| `roots` | A nonempty array of normalised search directories, relative to the source directory. `.` is permitted |

`subdir` and each root contain no absolute path, empty segment, `..`,
backslash or trailing separator. A repeated identical root in one source
refuses as a configuration mistake. Overlapping roots are valid. Each root must
exist as a directory in the captured source, and an empty directory is valid.

Ravel does not search for `SKILL.md`. It does not parse frontmatter, names,
descriptions or pack manifests. Roots choose which directories Ravel shows the
agent. The whole source directory supplies the supporting resources. Ravel
keeps the order of sources and roots for presentation. It never uses that order
to choose between skills.

`mandatory` is an ordered array of records with exactly `source` and `path`.
`source` names a configured source ID. `path` is a normalised file path,
relative to that source, with the grammar of a root. `.` is not a file.

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

An unknown `source` ID refuses as `settings_invalid`, exit 2, before
acquisition. A known source with a missing, unreadable, nonregular or non-UTF-8
mandatory file refuses as `skill_mandatory_invalid`, exit 3. An empty
`mandatory` array is valid.

Delivery is disabled when the settings file is absent or has no `skills` key.
It is also disabled when `sources` and `mandatory` are both empty. Without an
absolute `HOME`, Ravel reads no settings, so delivery is disabled. A disabled
resolution creates no cache, makes no network request and adds no prompt text.
A nonempty `mandatory` array with no sources refuses as `settings_invalid`. A
fresh installation has delivery disabled, and native harness skills work
without it.

Every configured source is required. A missing local directory or configured
root refuses as `skill_source_missing`. The refusal names the source ID and the
path. Its remedy is to restore the directory or to change the settings
deliberately. Ravel never creates an empty source, disables a source,
substitutes native skills or changes settings.

An existing empty root is not a missing root. A new machine restores its
sources outside the failing session. Alternatively, the owner removes their
configuration before a bootstrap launch. No option of one session skips the
owner's guidance.

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
| A concrete native tool or command | Use that mechanism first. When it reports no native match, say so. Then search the supplied directories for the same guidance before you report it missing |
| A concrete file path | Read that path. When the read fails, report the failure before dependent work. Do not use another location |
| An express restriction to one mechanism, such as "use only this native tool" | Obey the restriction. When that mechanism cannot load the guidance, report the limit before dependent work. Do not fall back |

The preamble also states one rule about skill metadata. When the metadata of a
skill sets `disable-model-invocation: true`, the agent loads that skill only
when the caller names it, by name or by path. The agent never loads it because
its description matches the work. Ravel itself reads no metadata, so the agent
applies this rule.

A native loader error other than "no match" is a report. It is not permission
to go around that loader. Two different matching files in the supplied
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
absolute, dangling, cyclic or escaping link refuses. A Git submodule entry and
every nonregular filesystem object refuse too. A source path can itself be a
symlink. Ravel canonicalises the source root once, then applies the boundary
to every captured entry.

Each path must be exact UTF-8, with no NUL and no control character. Capture
copies bytes, so a hard link in a mutable source never backs a snapshot.

The cache filesystem can fail to keep two distinct source paths or their exact
spellings. Ravel then refuses as `skill_resource_invalid` before publication,
and it names the paths. That includes a collision of case or of Unicode
normalisation. Ravel never writes one entry over another, and it never
normalises a name. It checks the written tree against the source manifest,
including every directory segment.

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

### Exact Git acquisition and offline reuse

The Git acquisition key is a versioned tuple. It holds the exact repository
locator of the owner, the object format, the full commit ID and `subdir`. Two
spellings of one remote are two acquisition identities. Ravel infers no
equivalence between hosts, mirrors, transports or credential contexts. The
source ID labels the settings and the diagnostics. It is not a skill identity,
and it is not part of the key.

Ravel refuses a locator that embeds a password, a credential in a query or
fragment, or a relative local path. It uses the ordinary noninteractive
transport authentication of the owner. No credential enters a manifest or a
diagnostic. A transport child receives no session-result context and no caller
completion descriptor. When authentication needs interactive input, it fails at
once. A source fetch never uses the caller's terminal.

Ravel first looks in the cache for a complete checked resolution of the exact
key. When it finds one, it checks the manifest and the content. It then reuses
the resolution with **no network operation**, not even a freshness check. A
cache hit needs no Git executable and no source repository. A changed pin is a
different key, so it cannot change an existing reader.

A corrupt cache entry refuses as `skill_cache_corrupt`. Its remedy is to
quarantine or remove the entry, then fetch it again. Ravel does not accept the
entry, and it does not repair it under its readers.

On a miss, Ravel acquires the commit in isolated temporary Git storage. It
checks that the exact object is a commit, not a tag that peels to another
object. It checks that `subdir` is a tree in that commit. A fetch can take
advertised refs to reach a commit that the remote does not advertise. Ravel
still extracts only the configured full commit ID. It never falls back to a
branch tip, a near ancestor, an earlier cache entry, the default branch or
another source with the same ID.

Some failures refuse as `skill_revision_unavailable`: a failed transport, a
missing Git executable, an unreachable commit or a missing `subdir`. The
refusal names the source and the required revision. The remedies are to
restore access, to make that commit available, to seed its exact cache while
online, or to change the pin deliberately.

Acquisition runs no Git hook or filter of a working tree. Ravel extracts the
tracked object contents directly. It refuses a gitlink, and it fetches no
submodule. Ravel captures a Git LFS pointer file as its literal bytes, not as
the payload that it describes. Ravel neither detects nor hydrates such
pointers, and its read guarantee covers only the captured file. A pack that
needs an LFS payload materialises it in a local source or stores an ordinary
Git blob.

Ravel bounds network work. It stops and reaps acquisition children at the
deadline and on cancellation. A cache miss needs Git on the `PATH` of the Ravel
CLI. The installation documentation and the installed checks state and test
that requirement. The pinned stage decides whether the Homebrew formula
declares Git. Ravel needs no other runtime.

### Publication, concurrency and cleanup

The cache root is `$HOME/.cache/ravel/skills/v1`. It does not depend on the
working directory, `TMPDIR` or the location of the configuration script. The
settings and the cache are private to the account. A cache root that a replaced
or unsafe symlink reaches refuses as `skill_cache_unavailable`. Ravel does not
walk a substituted cache tree. Cache writes use held directory identities and
staging directories that only the owner can open.

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
a complete checked object. When two publishers race, the loser checks the
winner before it removes its private copy. A winner whose content differs is
corruption.

Ravel publishes the descriptor only after every source and every configured
path passes its checks. A failed launch can leave useful source objects. It
never leaves a partial descriptor, and it never launches with a partial source
set. No mutable `latest` pointer is part of a request.

[Skill resources outlive sessions](../adr/skill-resources-outlive-sessions.md)
owns the lifetime trade-off. Snapshots and descriptors stay after a refusal,
after a session or delegate exits, after a settings change and after an
upgrade. A later cache format uses a new directory and keeps the older paths.
Ordinary cleanup removes only the private staging objects of the resolver that
made them, and it reports each cleanup failure.

Ravel has **no automatic collector and no prune command**. Manual removal
requires that no affected reader runs. Removal of staging also requires that no
resolver of that staging runs. Removal gives up offline reuse and the
guarantees of existing readers.

When storage is full, the refusal is
`skill_cache_unavailable` with this remedy. Read-only permissions prevent
accidents. They do not prevent deliberate changes by the same user.

## Directory descriptor and loading contract

One immutable `directories.json` records the prepared directory set. It holds
`schemaVersion: 1`, `sources`, `directories` and `mandatory`, and no skill
entries. The descriptor, the request and `skills resolve --json` use the same
source record. Every value of a source record is a string. A field that applies
only to the other type is absent.

| Source-record field | Contract |
| --- | --- |
| `id` | The configured source ID |
| `type` | `local` or `git` |
| `resourceDigest` | The lowercase SHA-256 digest of the snapshot manifest, 64 hexadecimal digits |
| `snapshotRoot` | The absolute path of the retained snapshot directory |
| `originPath` | Local only. The canonical absolute source directory at capture |
| `url` | Git only. The configured locator, free of credentials, that the acquisition key uses |
| `revision` | Git only. The exact full commit object ID |
| `subdir` | Git only. The configured source directory in the repository |

A directory record holds `source` and the absolute `path`. A mandatory record
holds `source` and the absolute file `path`, in configured order. Directory
records follow the order of sources and roots. A repeated absolute path appears
once, at its first position. Every path points into a complete retained
snapshot.

Source provenance stays per configured source, also when two sources share
stored content. A later launch can therefore check the original protected
boundaries again.

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
4. Keep the descriptor path in a continuation or compaction summary. On
   re-entry, recover the same directories and mandatory paths. Then read the
   mandatory and relevant guidance again before work continues. Do not acquire
   newer sources during a session.
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
of the evaluation, confined or not. The value replaces a stale inherited value.
When delivery is disabled, Ravel removes the name from those environments.

The name is absent from the Rhai worker and from acquisition children. It
grants no read permission and no ending authority. Ordinary children inherit
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

`ravel skills resolve` is a **source acquisition** command. It evaluates no
configuration script. It takes no skill name, task, kind, prompt, source
override or ending token. It reads the settings and runs the same acquisition
and checks as a launch. It imports no configuration script, creates no result
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
current local bytes and reuses exact checked Git pins. A failure can leave
complete useful cache objects. It prints no preamble and no partial JSON.

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
- `sources`, with one record for each source. A record holds `id`, `type`,
  `resourceDigest`, `revision` for a Git source, and `disposition`. The
  disposition is `captured` for a local source, `reused` for a Git cache hit
  and `acquired` for a Git fetch.
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
| Configured sources | 16 |
| Search directories / mandatory files | 256 / 256 |
| One mandatory file | 65,536 bytes |
| One regular supporting resource | 16 MiB |
| One source / all sources, in regular-file bytes | 128 MiB / 512 MiB |
| Entries in one source, with directories and links | 20,000 |
| Descriptor / preamble | 1 MiB each |
| Original prompt / prepared prompt | 1 MiB each |
| One resolution, with capture, checks, transport and publication | `resolutionTimeoutMs`: 120,000 ms by default, from 1,000 to 3,600,000 |

A breach of a byte or count bound refuses as `skill_limit_exceeded`. The
refusal reports the actual value, the bound and the subject. A value equal to
a bound passes. The search-directory bound counts the configured roots of all
sources, before the descriptor joins repeated paths. Ravel checks the three
configured counts before acquisition. Capture stops at the first entry or file
that passes a bound, so a refusal of the entries or the bytes of a source
reports the count at that point as "at least" that value. The bytes of all
sources count each configured source, also two that name one directory. Ravel
checks the descriptor and the preamble before it publishes the descriptor. For the prepared
prompt, it reports both prompt byte counts. It suggests a smaller task with
references to supporting files. Ravel never truncates caller instructions,
omits a configured directory, omits mandatory guidance or disables delivery to
meet a bound.

The prepared prompt travels to the worker in the evaluate frame. It returns in
the script result when the script puts it into `args`.
[Frame bounds](configuration-evaluation.md#frame-bounds) owns both bounds.
The 1 MiB prompt bound does not promise that a selection with that prompt fits
the script-result bound or the argument limit of the platform.

## Failures

| Code | Stage | Exit | Meaning and remedy |
| --- | --- | --- | --- |
| `settings_invalid` | `cli` | 2 | A malformed `skills` object, an unknown mandatory source ID or a repeated member name. Repair the settings at the reported `location` |
| `skill_source_missing` | `skills` | 3 | A local source directory or a configured root does not exist. Restore it, or change the settings deliberately |
| `skill_source_changed` | `skills` | 3 | A local source changed during capture. Keep it quiet and try again |
| `skill_revision_unavailable` | `skills` | 3 | Git cannot supply the exact commit or `subdir`. Restore access, make the commit available, seed the cache online or change the pin |
| `skill_mandatory_invalid` | `skills` | 3 | A mandatory file is missing, unreadable, nonregular or not UTF-8, or two references reach one retained file |
| `skill_resource_invalid` | `skills` | 3 | A link, an entry or a path that the snapshot cannot keep exactly |
| `skill_cache_corrupt` | `skills` | 3 | A cached object fails its checks. Quarantine or remove it, then fetch it again |
| `skill_cache_unavailable` | `skills` | 3 | Ravel cannot use the cache root, because of an unsafe link, a permission or full storage |
| `skill_limit_exceeded` | `skills` | 3 | A count or byte bound of this specification |
| `skill_access_unavailable` | `skills` | 3 | Ravel cannot set up the required reads, because of a protected or writable overlap, or a grant that the backend cannot give |
| `skill_resolution_timeout` | `skills` | 124 | The resolution deadline passed. Raise `resolutionTimeoutMs`, or address the latency of storage or transport |
| `skill_resolution_cancelled` | `skills` | 128 plus the signal number | INT, TERM or HUP arrived during resolution |
| `skill_cleanup_failed` | `skills` | 5 | Ravel cannot remove its staging or reap an acquisition child |

The failure document gains the stage `skills`. It covers the acquisition,
checks and publication of skill sources, the prepared prompt and the read
grants of the resources. A settings problem keeps the stage `cli`.

Each refusal names the exact revision or path that its remedy addresses. No
remedy recommends a revision that the owner did not configure. A timeout
reports the effective deadline and the active operation. It does not report an
unavailable revision only because time ran out. Ravel bounds Git diagnostic
output to 256 KiB and redacts it before it shows it.

`skill_access_unavailable` does not claim to detect a later failure of the
agent to read. No failure in this table gives a successful result. Each
refusal launches no worker and no managed process.

A consumed INT, TERM or HUP during resolution stops the acquisition children
with TERM, then KILL, and reaps them. Ravel removes the private staging. The
refusal is `skill_resolution_cancelled`, and Ravel raises the same signal again.
A consumed signal wins over a resolution refusal, as in the
[completion contract](result-job-completion.md#outcome-and-diagnostic-order).
The report then keeps that refusal as the original failure.

## Read access and delegation

The managed process and each harness run can read the descriptor and every
captured resource, also outside the working directory. With `--confine`, Ravel
grants the exact published descriptor directory and snapshot trees read-only
to both. It grants neither the private staging area of the cache, nor the
acquisition storage, nor the mutable source directories.

This grant set is internal. The public `--runtime-read` flag keeps its meaning
of one regular file. Ravel never grants the whole home directory or the whole
cache as a shortcut. The backend keeps the traversal and read behaviour of
admitted links inside a source, with no write access to their targets.

Ravel refuses a source boundary or a snapshot grant that overlaps the
configuration directory, the settings file or a known session-result channel.
That includes an overlap through a canonical alias. `skills resolve` protects
the default configuration directory `$HOME/.config/ravel`. A launch also checks
its actual `--config` directory against the origin record of each snapshot
that it would reuse. A copy of a protected file in the cache cannot pass a
grant that the original would fail.

A source that holds the cache root refuses too, also before the cache exists.
Ravel checks the real path that the opening of the cache would reach.

These checks cover the explicit overlap of sources and grants. They do not
cover secret text that an owner puts inside a source. Ravel also refuses a
writable grant that overlaps a published snapshot or the descriptor. Then
confinement cannot make an immutable object writable through the working
directory. That check covers the whole cache root, because a writable staging
area could change a publication in progress. Before the launch, Ravel lists
each granted directory inside the backend. Each refusal in this section is
`skill_access_unavailable`, with no unconfined fallback.

The harness and its ordinary children inherit the read grant from their
confinement. A delegate can run in a separate sandbox, a remote worker, another
account or another filesystem namespace. Its adapter then forwards the same
descriptor and directory set, maps the same absolute paths and grants the reads.

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

An adapter can control the environment and the filesystem grants of a
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

A nested launch inside a confined parent does not escape the sandbox of the
parent. The nested launch refuses when its configuration script, the
transport or cache writes are unavailable. On macOS the settings file is
visible and unreadable, so the nested launch refuses as `settings_invalid`.
On Linux the namespace holds no settings file. The nested launch then has
delivery disabled, and it runs when it can read its configuration script. Ravel does not open the
protected configuration directory to permit it. A harness delegate can reuse
the retained descriptor without a nested launch. A privileged broker for nested
launches is outside scope. A `start-session` inside a harness run still refuses
as `config_reentry_refused`.

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

### Caller migration

A caller migrates its configuration script in four steps:

1. Export `SCHEMA_VERSION` 4.
2. Pass `request.prompt` to the managed harness, as before.
3. Read `request.originalPrompt` wherever the script analyses the task, for
   example in an LLM call that classifies it.
4. Build each read grant of a harness from `request.skills`, for example one
   argument for each `snapshotRoot`.

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
only after the compatible pair is installed. Grove's coordinated-release design
owns the order between the products.

This feature adds no default skill corpus, native installer, skill execution
engine, caller-kind registry or provider integration. It adds no task parser,
result-token interpreter or persistent session database. The retained content
cache is the one new persistence surface. The feature manages no marketplace,
removes no native hook, runs no source code and edits no settings.

## Test seams and acceptance

Use the public `ravel skills resolve` and `start-session` boundaries. Supply
temporary `HOME`, working, cache and configuration directories and local Git
fixtures. A fake managed program reads the delivered prompt, the descriptor and
the resources. It sends an opaque result through its own channel.

Configuration-script fixtures record what `load_context` and `select` receive.
Fault injection stays behind internal seams. Tests also use real cache
filesystems and real confinement backends.

| Case | Observable acceptance at the public seam |
| --- | --- |
| Disabled delivery | A missing settings file, an absent key and empty arrays do no cache or network work. Empty, newline and Unicode prompts stay exact. `skills` is `()`, and `prompt` equals `originalPrompt`. Native-only Grove sessions need no directory setup |
| Directory delivery | Local roots outside the working directory appear as absolute retained paths in the preamble. The order of sources, roots and mandatory files agrees with the descriptor and the request |
| Filesystem versus skill semantics | Empty roots, duplicate skill names, invalid optional frontmatter and unusual directory names resolve and launch. A fixture agent reports missing or ambiguous guidance, and Ravel chooses no skill |
| Settings and bootstrap | An unknown mandatory source ID refuses as `settings_invalid`, exit 2, before acquisition. So do a repeated member name, a malformed path and an unknown field. A missing or non-UTF-8 mandatory file refuses as `skill_mandatory_invalid`, exit 3. A missing source or root refuses as `skill_source_missing`. No worker launches |
| Relative resources | The managed program reads sibling and parent-relative files from the whole source. Admitted links work. Escaping, dangling and cyclic links, special files and gitlinks refuse |
| Path identity | Paths that differ only in case or normalisation refuse when the cache filesystem cannot keep them. A filesystem that keeps them keeps both. No entry is written over another or normalised |
| Git LFS object bytes | A committed pointer is captured byte for byte, with no contact with LFS storage. A materialised local payload is captured as ordinary bytes |
| Local freshness | A content change with no VCS change gives a new snapshot. Old readers keep old bytes. Controlled concurrent edits refuse with no partial descriptor |
| Exact Git | With two commits and a moved tag, Ravel extracts only the full pin. Refs, abbreviations and tag objects refuse |
| Offline reuse | Warm the exact cache, then remove remote access and Git from `PATH`. Resolution and launch succeed, and a transport spy sees zero calls. A cold absent pin cannot use a cached neighbour |
| Missing or corrupt revision | A missing commit or `subdir`, a bad object type, a timeout and a tampered snapshot have distinct remedies. None of them launches a worker or a job |
| Concurrent publication | Publishers behind a barrier and an existing reader see no partial object. The losing publisher checks the winner. A killed publisher cannot publish a partial descriptor |
| Lifetime | Independent and detached readers keep their directories and resources after the parent exits, the source goes and the settings change. Ordinary managed descendants are reaped with the group. Quiet manual removal loses offline reuse |
| Request and evaluation | `load_context` and `select` receive the same request, and a change to the copy in `load_context` does not reach `select`. Top-level code sees no request. The caller byte range is exact for non-ASCII text, every line ending and text that resembles a delimiter. `skills` has no offset fields. The label and delimiters occur only in the wrapper. A quoted path cannot become prompt structure |
| Host reads | A read of the descriptor with `ravel::read_text` is a measured read in the notice. `ravel::env("RAVEL_SKILL_DIRECTORIES")` throws `env_not_granted`, and a grant of that name refuses as `excluded_grant` |
| Harness runs | A fake harness in a harness run sees this session's descriptor in `RAVEL_SKILL_DIRECTORIES` when delivery is enabled, and no variable when it is disabled. Under `--confine` it reads the resources |
| Bounds and frames | Enabled delivery with an empty task still bootstraps. Each count and byte bound refuses at its limit and passes below it, with controls that remove the bound. The three tests of [frame bounds](configuration-evaluation.md#frame-bounds) pass where their platform admits them |
| Slow valid source | A controlled delay times out with the effective deadline and the settings remedy. A larger valid deadline succeeds with every check. A malformed deadline refuses |
| Cancellation | TERM during a controlled transport refuses as `skill_resolution_cancelled`, exit 143. The transport child is reaped, and no staging stays |
| Real confinement | A child and its descendants read resources outside the working directory. They cannot write them or read the protected configuration. Grant failures, protected aliases and writable overlap refuse as `skill_access_unavailable`, exit 3, with no fallback, on macOS and Linux |
| Delegate authority | Disposable trials tell inherited context that can send an early result from an adapter whose scrubbed delegate cannot. First-writer behaviour has its own check. Each trial names its adapter |
| Directory recovery | With the instructions retained, the prompt path is lost and the agent recovers the same descriptor through `RAVEL_SKILL_DIRECTORIES`. Stale values are replaced, and disabled delivery removes the name. A nested child alone receives a new value. Loss of both path channels gives a report of missing context |
| Nested sessions | A nested resolution sees current sources and returns only its own result. The paths and channel of the parent stay intact. Confinement can refuse unavailable settings or cache writes |
| Version transition | Schema 3 refuses with and without sources. Schema 4 works with and without sources. A mismatched pair refuses. Ravel 1.1.0 refuses shared settings that hold `skills`, exit 2, and separate prefixes cannot prevent it. Ravel 2.0.0 refuses `policyEnv`. An isolated `HOME` permits 1.1 native-only and 2.0 directory launches at the same time. Existing ending commands stay usable |
| Standalone acquisition | No configuration script, channel or harness. The plain preamble and the JSON describe the same directories as a launch. Disabled and failed outputs differ. Retries keep only complete useful objects |

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
the confinement mode and the order of reads and actions. It records native startup
text that the harness added already. It records the inherited ending context of
each real delegate and whether the delegate obeyed the instruction not to end.

Result attempts belong only in disposable trials. A fetch fixture cannot prove
LLM compliance. A prompt transcript cannot prove permissions. An unavailable
compaction, delegate or confinement mechanism is an explicit gap in the
measurements. Grove checks its native-only compatibility separately from these
trials.

## Out of scope

- A skill catalogue, a name resolver or a lookup command.
- Automatic configuration of harness sessions that the owner opens
  independently.
- A garbage collector or a prune command.
- Management of native installations, hooks or marketplaces.
- A host function for skills.
- Git LFS hydration and submodules.
- A source override for one launch.
