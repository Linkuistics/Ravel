# Token handoff

A confined harness cannot reach the sign-in store of its harness. A token
handoff gives it a token that the owner minted for that use. Ravel reads the
token from a file that the settings name. Then it sends the token to the
selected program through a socket. Ravel never opens the sign-in store of a
harness.

## Status and ownership

This specification states the token handoff of the combined Ravel 2.0.0. It is
part of the contract of the `skill-directory-delivery` line. Ravel `main` does
not hold it, and no release contains it.

Each rule has one owner:

- This specification owns the `tokenHandoffs` settings object and the
  `tokenHandoffs` field of a selection and of a harness-run request. It also
  owns the protected paths, the checks and the read of a token file. It owns
  the socket, the runner's descriptor list, the refusal
  `token_handoff_unavailable` and the acceptance cases of this feature.
- The [configuration evaluation specification](configuration-evaluation.md)
  owns the settings file as a whole, the selection map, the harness-run
  request, the routing notice and the [credentials](configuration-evaluation.md#credentials)
  rule. It lists the new fields and links here for their meaning.
- The [session execution specification](session-execution.md) owns the
  managed process and confinement in general.

The decision record
[a confined harness receives an owner-minted token](../adr/a-confined-harness-receives-an-owner-minted-token.md)
records the choice of this route and the routes that the design rejected. The
[visual views](../design/token-handoff/index.html#discussion) draw the route
and the launch.

## Problem

Ravel's confinement denies the Mach services of the Security framework and the
reads of the owner's Keychain files. Claude Code 2.1.317 on macOS keeps its
claude.ai sign-in in the Keychain item `Claude Code-credentials`. A confined
Claude Code therefore reports "Not logged in" and makes no model request. Grove's confined trials measured this result on 2026-11-03.

A profile that admits the Keychain would let a confined agent ask for any
Keychain item that the user can read. The owner rejected that change.

Anthropic's terms forbid developers to collect, store or intermediate Claude.ai
credentials or session tokens
([legal and compliance](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use)).
Ravel therefore does not read the sign-in of Claude Code and pass it on.

## Solution

Anthropic documents `claude setup-token` for scripts and for other places
without a browser sign-in
([authentication](https://code.claude.com/docs/en/authentication#generate-a-long-lived-token)).
The command runs Anthropic's own sign-in flow and prints a token with a
lifetime of one year. The token can make model requests only. Claude Code reads
it from `CLAUDE_CODE_OAUTH_TOKEN`, or from the descriptor that
`CLAUDE_CODE_OAUTH_TOKEN_FILE_DESCRIPTOR` names.

The owner mints such a token once and saves it in a file that only the owner
can read. The settings declare a token handoff: the file, the variable that
receives the descriptor number and the expiry date. The configuration script
names the handoff in each selection or harness run that needs it.

For a confined launch, the Ravel CLI checks and reads the file before the
launch. It writes the token into one end of a new socket pair and closes that
end. The runner keeps the other end open across the launch, and the selected
program inherits it. The declared variable holds its descriptor number. Claude
Code reads the token during its initialisation and closes the descriptor.

No grant of a confined launch reaches the file, because Ravel refuses every
grant that overlaps it. Skill capture refuses a source that holds the file, so
no snapshot copies it. The confinement profile gains no grant for this
feature. The owner's own sign-in stays in its store, and the handoff cannot
refresh or rotate it.

## Decisions

### The owner's token

The owner runs `claude setup-token` in a terminal, outside any session. The
owner saves the printed token in a regular file with mode `0600`, for example
beside the settings file. The book gives the recipe.

Ravel does not mint, renew, revoke or store a token. It never runs
`claude setup-token`. A token belongs to the owner's account, and its use bills
the owner's subscription, as the owner's own sign-in does.

Ravel gives a token no meaning. A token handoff names a file and a variable,
and it holds nothing that is specific to a harness. The variable of Claude Code
is an example in the settings and in the book.

### Settings

The settings file gains an optional key `tokenHandoffs`. It is a map from a
handoff name to a record. A name has the grammar of a harness name: 1 to 64
lowercase letters, digits and dashes, with a letter or a digit first.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | String | Required. The token file, as an absolute UTF-8 path, with no tilde or environment expansion |
| `descriptorVariable` | String | Required. The environment name that receives the descriptor number, such as `CLAUDE_CODE_OAUTH_TOKEN_FILE_DESCRIPTOR` |
| `expires` | String | Required. A date in the form `YYYY-MM-DD`. The token is unusable from 00:00 UTC on that date |

A missing field, an unknown field or a malformed value refuses as
`settings_invalid`, stage `cli`, exit 2. The `location` names the value, such
as `tokenHandoffs.claude-code.expires`.

`descriptorVariable` holds ASCII letters, digits and underscores, with a letter
or an underscore first. A name in a reserved class of
[settings and environment grants](configuration-evaluation.md#settings-and-environment-grants)
refuses: `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*`. A name of the
confined environment also refuses: `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG`,
`LC_*`, `TMPDIR`, `TMP` and `TEMP`. Two handoffs with the same
`descriptorVariable` refuse.

The owner sets `expires` at most one year after the date of minting. An earlier
date gives an earlier refusal. Ravel cannot read the expiry of an opaque token,
so the owner's date is the only expiry that Ravel checks.

Ravel checks the shape of each record when it reads the settings. It does not
open a token file at that point.

### Naming a handoff

The selected map gains an optional field `tokenHandoffs`. It is an array of
handoff names. An absent field and an empty array name no handoff. A name that
the settings do not declare refuses as `selection_malformed`, exit 3. So does a
name that the array repeats. The location names the entry, such as
`result.tokenHandoffs[0]`.

The request of `ravel::run_harness` gains the same optional field. A harness
run that names an undeclared handoff is `unavailable`, with the error code
`token_handoff_unavailable`. It uses no budget, as the other `unavailable` runs
do not.

The configuration script decides which launch receives a token. For example, a
selection of Codex names no handoff for Claude Code. The settings decide where
the token is and when it expires, and the script cannot change them.

### Confined launches only

Ravel delivers a named handoff only to a confined managed process or a confined
harness run. Without `--confine`, Ravel opens no token file and sets no
variable. The harness then uses its own sign-in, as before.

This rule keeps an unconfined Claude Code on the owner's own sign-in. Claude
Code ranks a token in either of its token variables above the sign-in of
`/login`. The token can also make model requests only.

An unconfined launch still protects every declared token file from skill
capture, as the next section states.

### Protected token files

Each declared token file has two protected paths. The first is its declared
path, with its directory in canonical form. The second is the real path that
the declared path resolves to, or its prospective path when the file is
absent. When the declared path is a symbolic link whose target is absent, the
real path is the prospective path of that target. A link at the declared path
is therefore protected, as is its target. Ravel protects every declared file,
also when no selection names it.

Each launch resolves the protected paths of each declared file once, before
its first check. The source check, the confinement check and the read of that
launch use that one resolution. Each harness run resolves them once for its own
confinement check and its read.

These paths join the protected paths of two checks that Ravel has now. Each check
refuses with the code that an overlap with the settings file receives in that
check. Ravel has no single overlap code. Each check also refuses a declared
file that exists with more than one link, with the same code. Its other name
can lie inside a grant or a source.

| Check | What it covers | When | Refusal |
| --- | --- | --- | --- |
| Confinement grants | The writable grants, runtime reads and system reads of a confined launch | Before the worker launches | `confinement_overlap`, stage `cli`, exit 2 |
| Skill sources and their grants | Each skill source of `skills resolve` and of every launch, with or without `--confine`. Each descriptor directory and snapshot tree that a confined launch grants | Before capture, and before the worker launches | `skill_access_unavailable`, stage `skills`, exit 3 |

A source that holds a declared token file therefore refuses before Ravel
copies any of its content into the cache. A snapshot from a capture before
the settings declared the file stays in the cache. No launch grants that
snapshot, because every launch captures its sources again and refuses first.
The owner removes it, as the
[skill discovery specification](skill-discovery.md#sources-and-retained-resources)
states.

These checks resolve paths and examine link counts, and they read no content.
The read examines the link count again. A confined process cannot make a hard
link to the token file. Under a copy of Ravel's Seatbelt profile, `ln` from a
dummy file outside the grants failed with `EPERM`, also through the
`/System/Volumes/Data` path. Bubblewrap does not mount the file.

The runner grants each path in the form that Ravel checked, as the
[session execution specification](session-execution.md#confinement) states. A
grant whose path now resolves to another path, such as the token file, gives
no grant, and the launch refuses.

Two read grants stay outside the overlap check: the selected program and
Ravel's own executable. The configuration script selects the program, and
Ravel trusts it.

Ravel protects no symbolic link between the declared path and the real path.
A process that can change such a link before a launch changes the file that
Ravel protects and reads. The owner therefore keeps each link of a declared
path outside every grant.

### Checks and the read

The Ravel CLI checks each named token file after the selection and before it
allocates the result channel. For a harness run, it checks the file before it
launches the run. Each delivery runs these steps, in this order:

1. The current date is before `expires`, in UTC.
2. Ravel resolves the declared path again. Its real path must be the real path
   of the resolution of the same launch. A harness run has a resolution of its
   own. The declared path can be a symbolic link.
3. Ravel opens that real path to read it. The open follows no link in the
   last component, and it does not block.
4. The open file is a regular file.
5. The open file belongs to the effective user of the Ravel CLI.
6. The mode of the open file grants no permission to the group or to others.
7. The open file has exactly one link.
8. The open file holds 1 to 4,096 bytes.
9. Ravel reads the open file to its end, with as many reads as that needs.
   It stops after 4,097 bytes. The read must give 1 to 4,096 bytes.

Steps 4 to 8 examine the open descriptor, not the path, and step 9 reads from
the same descriptor. The file that Ravel checks is therefore the file that it
reads. Step 2 binds that file to the protected real path. Step 7 refuses a
second name that appeared after the protection. Step 9 refuses a file that
grew after step 8. A short read does not end step 9 early.

An interval stays open between step 2 and step 3. In it, a process can
replace a directory above the real path. No confined process can do that,
because a grant over such a directory overlaps a protected path. Ravel trusts
the processes outside confinement, because they can read the token file
already. Such a replacement can make the launch refuse, or make Ravel read
another file. It cannot give a confined process a grant over the token file.

The owner can replace the token file by a rename at any time. The real path
stays the same, so a delivery reads either the old file or the new one.

Ravel does not check the content further. Claude Code reads up to the first
newline and removes the surrounding white space.

A failed check or read refuses as `token_handoff_unavailable`, stage
`handoff`, exit 3. The message names the handoff, the path and the failed
property. It never holds content of the file. A failed open reports as the
regular-file property, with the error of the system.

| Failed property | Remedy |
| --- | --- |
| Expiry | Mint a new token with the harness, replace the file and set a later `expires` date |
| Changed path | Keep the declared path and its links unchanged during a launch, then launch again |
| Regular file | Save the token in the named file, or correct `file` in the settings |
| Owner or mode | Make the file belong to you, with mode `0600` |
| Link count | Keep one name for the token file. Copy it, and do not make a link to it |
| Size | Save only the token in the file |

A refusal launches nothing. Ravel does not try again without the handoff,
without confinement or with another credential. A harness run that fails a
check is `unavailable` with the same code, and the script interprets it.

### Delivery through a socket

For each delivery, Ravel creates a connected pair of Unix stream sockets. It
creates the pair directly before it gives the launch request to the runner.
Both ends are close-on-exec in the Ravel CLI, so no other child inherits them.
Ravel writes the token bytes into one end, shuts down that end for writing
and closes it. The token has at most 4,096 bytes, so the write ends before the
launch on both platforms. A failure before the runner receives the request
closes both ends, and the token goes with them.

The selected program inherits the other end, as the
[runner's descriptor list](#the-runners-descriptor-list) states. The
descriptor number is 3 or more. The declared variable holds that number in
decimal, and Ravel adds the variable to the confined environment. Each handoff
has its own socket and its own variable. Each harness run receives new
sockets.

Ravel keeps the token bytes only from the read to the write.
[The token and output](#the-token-and-output) states what Ravel writes.

The descriptor is a socket because of the reader in Claude Code 2.1.317:

- It reads a socket directly, up to the end of the stream or the first
  newline, and closes the descriptor. Then it removes the variable from its
  environment.
- It opens any other descriptor by the path `/dev/fd/N`. It reads the
  descriptor directly only after `EACCES` or `EPERM`.
- Ravel's Seatbelt profile reports `/dev/fd/N` as absent, with `ENOENT`.

A pipe therefore leaves a confined Claude Code without a sign-in. A probe
under a copy of the profile measured the two reads with dummy text. The read
by path failed for a pipe and for a socket. The direct read passed for both.

The token is not in an environment variable, because every descendant inherits
the environment. An agent could then print the token.

### The runner's descriptor list

The runner of `keyed-launch` marks every inherited descriptor above standard
error close-on-exec in each detached launch. A confined managed launch and a
harness run are detached launches. Without an exception, the runner would
close the socket when it launches the confinement backend. A change in the Ravel
CLI alone is not sufficient, because the runner sets close-on-exec again in
the child.

A launch request therefore carries an explicit list of socket descriptors, and
the runner owns that list:

- In the child, the runner first marks every descriptor above standard error
  close-on-exec, as before. Then it clears close-on-exec on each listed
  descriptor, at its number in the Ravel CLI. No other descriptor above
  standard error stays open across the exec.
- The confinement backend passes the listed descriptors to the selected
  program.
- The runner receives the other ends with the request. It closes its
  copies in the Ravel CLI after the spawn, when the spawn succeeds and when it
  fails. A failed spawn discards the token with the socket.
- Only a confined managed launch and a confined harness run carry a list. The
  list holds only the other ends of the delivered handoffs.

The list is a narrow addition to the runner seam that Ravel has now. It needs no
launcher that knows about credentials.

### The token and output

Ravel itself writes no token byte to its stdout, its stderr, a notice, a
diagnostic, a refusal, a request, a log or a file. Its handling of a handoff
sends no token byte to the worker.

That promise covers what Ravel adds while it handles a token. It does not
cover a measured read of the token file that the configuration script makes.
The [credentials](configuration-evaluation.md#credentials) rule keeps such a
read unchanged. The promise also does not cover what a trusted program
returns. The selected program and a harness run can read the socket and print
the token:

- The output of the managed process goes to the caller's standard error. Its
  result goes to Ravel's stdout.
- Ravel returns the `stdout` and `stderr` of a harness run to the
  configuration script.

Ravel removes no credential from that output, as the
[credential boundary](configuration-evaluation.md#requirement-credential-boundary)
states. It trusts the configuration script and the programs that the owner
names. An echo of the token is therefore outside the secrecy of the handoff.
The owner names a handoff only for a program that does not print the token.

### What a confined process can read

- **The token file.** No writable root, runtime read, system read, skill
  directory or snapshot tree of a confined launch reaches it. Ravel refuses
  each one that overlaps a protected path of the file, and the runner grants
  each path in its checked form. No snapshot holds a copy, because capture
  refuses a source that holds the file. A file with a second name refuses, and
  a confined process cannot make one. The selected program and Ravel's own
  executable are the two read grants outside this check.
- **The Keychain and the sign-in stores.** This feature opens no sign-in store
  and adds no grant for one. The profile gains no Mach service and no
  Keychain read for this feature. A confined process reads a file only
  through a grant of its launch. A grant that the caller names can still
  give access to a sign-in file. Examples are the runtime read of `auth.json` that
  Codex uses, and a granted `.credentials.json` on Linux. The complete grant
  set decides those reads, not this feature.
- **The socket.** Claude Code reads it during its initialisation and then
  closes the descriptor. A descendant that Claude Code launches later receives
  no such descriptor. A static reading of the 2.1.317 executable gives this
  order, and no run measured it. A hook or an MCP server that Claude Code
  launches before its read would inherit the unread socket. A confined agent
  can write project hooks into a writable working directory for a later
  launch.
- **The token.** Claude Code holds the token in memory and sends it with each
  model request. Ravel does not protect the token from a process that can read
  the memory of Claude Code. The platform owns that protection.

The rule for the socket holds only when the selected program reads the socket
first. A wrapper that sets variables and then executes Claude Code keeps it. A
wrapper that runs other programs before Claude Code gives them the unread
socket. The owner names a handoff only for a program that reads it first.

### Lifetime

A run cannot outlast its token. A token of `claude setup-token` lasts one year
from minting, and the owner can revoke it sooner. Ravel refuses a launch from
the `expires` date. A run that is active when the token expires or loses its
validity fails inside the harness, with the message of the harness. Ravel
cannot see that failure.

The owner's own sign-in is independent of the token. The token has no refresh
token, so a confined Claude Code cannot refresh it. The handoff gives a
confined session nothing that can rotate or revoke the owner's own sign-in. A
grant that gives access to a sign-in store is outside this feature, as
[what a confined process can read](#what-a-confined-process-can-read) states.

### macOS and Linux

The handoff is the same on both platforms. The token file is an ordinary file.
Ravel opens no Keychain item on macOS and no `.credentials.json` on Linux.
Seatbelt and bubblewrap both pass an inherited descriptor to the program that
they launch, when the runner keeps that descriptor open. Claude Code reads a
socket directly on both platforms.

The probe measured Seatbelt only. The bubblewrap source passes inherited
descriptors to its payload, but no run measured that. The Linux fixture of
the implementation measures bubblewrap before this feature can pass.

### Diagnostics

The routing notice gains `tokenHandoffs`, the names that Ravel delivered to
the managed process, in their order. It is an empty array when Ravel
delivered none, also without `--confine`. Each entry of `harnessRuns` gains the
same field for its run. The notice keeps schema version 2. It never holds the
path or the content of a token file.

### Refusals

| Code | Stage | Exit | Meaning |
| --- | --- | --- | --- |
| `settings_invalid` | `cli` | 2 | A `tokenHandoffs` record is malformed |
| `confinement_overlap` | `cli` | 2 | A writable grant, runtime read or system read overlaps a protected path of a token file, or a declared token file has a second link |
| `skill_access_unavailable` | `skills` | 3 | A skill source, descriptor directory or snapshot tree overlaps a protected path of a token file, or a declared token file has a second link |
| `supervision_failed` | `exec` | 126 | A checked grant of the managed launch now resolves to another path. A harness run returns `harness_launch_failed` |
| `selection_malformed` | `selection` | 3 | The selection names an undeclared handoff, or repeats a name |
| `token_handoff_unavailable` | `handoff` | 3 | A named token file failed a check or the read |

A harness run reports `token_handoff_unavailable` as data, with the status
`unavailable`.

## Requirements

### Requirement: Delivery under confinement

Ravel SHALL deliver each named token handoff to a confined managed process
through an inherited socket. It SHALL set the declared variable to the
descriptor number of that socket.

#### Scenario: A confined launch

- **WHEN** the settings declare the handoff `claude-code` with a valid file
- **AND** the selection names `claude-code`
- **AND** the caller passes `--confine`
- **THEN** a fake harness reads bytes with the SHA-256 digest of the file,
  from the descriptor that its variable names
- **AND** the fake harness then reads the end of the stream
- **AND** the descriptor is a socket
- **AND** the harness does not hold a sentinel descriptor that the caller of
  Ravel leaves open without close-on-exec
- **AND** an open of the token file inside the launch fails, also through the
  `/System/Volumes/Data` path on macOS
- **AND** a hard link of the token file into the working directory fails

#### Scenario: An unconfined launch

- **WHEN** the same selection runs without `--confine`
- **THEN** the harness receives no added descriptor and no declared variable
- **AND** Ravel opens no token file

#### Scenario: A selection that names no handoff

- **WHEN** a confined selection omits `tokenHandoffs`
- **THEN** the harness receives no added descriptor and no declared variable

### Requirement: No unconfined fallback

Ravel SHALL refuse a confined launch whose named token file fails a check, and
it SHALL launch nothing.

#### Scenario: An expired token

- **WHEN** the current date in UTC is the date of `expires` or later
- **THEN** the launch refuses as `token_handoff_unavailable`, exit 3
- **AND** Ravel creates no result channel and launches no managed process

#### Scenario: A file open to others

- **WHEN** the token file has mode `0644`
- **THEN** the launch refuses as `token_handoff_unavailable`
- **AND** the remedy asks for mode `0600`

#### Scenario: A link retargeted after the protection

- **WHEN** the declared path is a link to a valid file outside every grant
- **AND** fault injection retargets the link after the protection, to a valid
  file in the working directory
- **THEN** the launch refuses as `token_handoff_unavailable`, with the
  changed-path property
- **AND** Ravel opens neither file and launches no managed process

#### Scenario: A file replaced after its checks

- **WHEN** fault injection replaces the token file by a rename after the
  checks of the open file
- **THEN** the fake harness reads bytes with the digest of the checked file

#### Scenario: A file that grows after its size check

- **WHEN** fault injection appends bytes after the size check, and the file
  then holds more than 4,096 bytes
- **THEN** the launch refuses as `token_handoff_unavailable`, with the size
  property
- **AND** Ravel launches no managed process

#### Scenario: A second link after the protection

- **WHEN** fault injection adds a second hard link to the token file after the
  protection
- **THEN** the launch refuses as `token_handoff_unavailable`, with the
  link-count property

### Requirement: The token stays out of Ravel's own output

Ravel SHALL add no byte of a token file to its stdout, its stderr, a notice, a
refusal, a request, the worker or a log. Output that the selected program or a
harness run gives is outside this requirement.

#### Scenario: Leak probe

- **WHEN** the token file holds a sentinel
- **AND** a confined launch and a confined harness run deliver it to a fake
  harness that reports only a digest
- **AND** another launch refuses because the file is open to others
- **THEN** the sentinel is in no output of Ravel, no notice, no failure
  document, no request and no harness-run result that the script receives

### Requirement: Protected token files

Ravel SHALL refuse a confined launch when a grant overlaps a protected path of
a declared token file.

#### Scenario: A token file in the working directory

- **WHEN** a declared token file is in the working directory
- **THEN** every confined launch refuses as `confinement_overlap`, exit 2,
  before the worker launches

#### Scenario: A token link in the working directory

- **WHEN** a declared path is a link in the working directory to a file
  outside it
- **THEN** every confined launch refuses as `confinement_overlap`, exit 2,
  before the worker launches

#### Scenario: A token file with a second link

- **WHEN** a declared token file has a second hard link
- **AND** no selection names its handoff
- **THEN** a confined launch without skill delivery refuses as
  `confinement_overlap`, exit 2, before the worker launches

#### Scenario: A dangling token link

- **WHEN** a declared path is a link whose target is absent
- **THEN** the protection checks the prospective path of that target
- **AND** a confined launch that names the handoff refuses as
  `token_handoff_unavailable`, with the regular-file property

#### Scenario: A runtime read retargeted after its check

- **WHEN** a confined launch grants a runtime read of a regular file
- **AND** fault injection replaces that file with a link to a declared token
  file, after the check and before the spawn
- **THEN** the launch refuses as `supervision_failed`, exit 126
- **AND** the managed process does not run

### Requirement: Token files and skill sources

Ravel SHALL refuse a skill source that overlaps a protected path of a declared
token file, before it captures that source.

#### Scenario: A token file inside a skill source

- **WHEN** a declared dummy token file lies inside a skill source
- **AND** no selection names its handoff
- **THEN** `skills resolve`, an unconfined launch and a confined launch each
  refuse as `skill_access_unavailable`, exit 3
- **AND** the cache holds no copy of the dummy token

### Requirement: Harness runs

A confined harness run SHALL receive the handoffs that its request names,
under the same rules.

#### Scenario: A run that names an absent file

- **WHEN** a confined harness run names a handoff whose file is absent
- **THEN** the run is `unavailable` with `token_handoff_unavailable`
- **AND** it uses no budget, and the script continues

## Test seams

- **Ravel's public boundary, with fixtures.** `start-session`,
  `skills resolve` and `ravel::run_harness` cover every requirement above.
  Each test has a temporary home, a fake harness and a dummy token file. The
  fake harness reports its variable and the type of its descriptor. It
  reports the SHA-256 digest of the bytes that it reads, and whether the
  stream ended. It also reports the state of a sentinel descriptor, and the
  results of an open and a hard link of the token file. It never prints the
  bytes. Fault injection behind internal seams changes the file or a grant
  between the protection and the read or the spawn. These tests run in
  `task check` on macOS and in the Linux lane under bubblewrap. They need no
  owner credential.
- **The real profile, without a model call.** This check is opt-in. The human
  approves it before Ravel reads the owner's token. The selected program runs
  `claude auth status` under Ravel's own `--confine`, in a sandbox home with
  its own settings. With the handoff, Claude Code reports a sign-in. The
  control, whose selection names no handoff, reports no sign-in. The record
  holds no byte of the token.

### Verification limits

- **Bubblewrap.** No run measured that bubblewrap passes a listed descriptor.
  The Linux fixture measures it.
- **The reader of Claude Code.** The socket branch, the close of the
  descriptor and the order of the read come from a static reading of the
  minified 2.1.317 executable. The real-profile check measures the sign-in.
  It does not measure the order of the read against hooks and MCP servers,
  and the record names that gap unless a run measures it.
- **The probes.** The hard-link and `/System/Volumes/Data` results and the
  descriptor reads used a copy of the Seatbelt profile with dummy files. The
  public fixtures measure Ravel's own launch.

## Out of scope

- A read of the sign-in store of a harness, on any platform. Anthropic's terms
  forbid it, as the decision record states.
- Minting, renewal and revocation of a token. The owner does them with the
  harness.
- Detection of expiry or revocation from the token itself. A token is opaque to
  Ravel.
- A token for an unconfined launch. The harness uses its own sign-in there.
- A Keychain grant in the confinement profile. The owner rejected it.
- The sign-in of Codex. It keeps its route: a runtime read of `auth.json`.
