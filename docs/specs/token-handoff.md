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
  owns the checks and the read of a token file, the socket, the refusal
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
end. The selected program inherits the other end, and the declared variable
holds its descriptor number. Claude Code reads the token to the end of the
stream during its initialisation.

No confined process can read the file, because Ravel refuses every grant that
overlaps it. The confinement profile gains no grant for this feature. The
owner's own sign-in stays in its store, and the handoff cannot refresh or
rotate it.

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

### Protected token files

Every declared token file is a protected path of every confined launch, as the
configuration directory and the settings file are. A grant that overlaps a
declared file refuses with the code that an overlap with the settings file
receives. This covers writable grants, runtime reads, system reads and skill
directories.

The check uses the canonical path of the file, or its prospective path when
the file is absent. It reads no content, and it runs before the worker
launches.

### Checks and the read

The Ravel CLI checks each named token file after the selection and before it
allocates the result channel. For a harness run, it checks the file before it
launches the run. It checks these properties in this order:

1. The current date is before `expires`.
2. The path resolves to a regular file. Ravel admits a symbolic link by its
   real file.
3. The file belongs to the effective user of the Ravel CLI.
4. The mode of the file grants no permission to the group or to others.
5. The file holds 1 to 4,096 bytes.

Ravel then reads the whole file once. It does not check the content further.
Claude Code reads up to the first newline and removes the surrounding white
space.

A failed check refuses as `token_handoff_unavailable`, stage `handoff`, exit 3.
The message names the handoff, the path and the failed property. It never
holds content of the file.

| Failed property | Remedy |
| --- | --- |
| Expiry | Mint a new token with the harness, replace the file and set a later `expires` date |
| Regular file | Save the token in the named file, or correct `file` in the settings |
| Owner or mode | Make the file belong to you, with mode `0600` |
| Size | Save only the token in the file |

A refusal launches nothing. Ravel does not try again without the handoff,
without confinement or with another credential. A harness run that fails a
check is `unavailable` with the same code, and the script interprets it.

### Delivery through a socket

For each delivered handoff, Ravel creates a connected pair of Unix stream
sockets. It writes the token bytes into one end, shuts down that end for
writing and closes it. The token has at most 4,096 bytes, so the write ends
before the launch on both platforms.

The selected program inherits the other end through the confinement backend.
The descriptor number is 3 or more. The declared variable holds that number in
decimal, and Ravel adds the variable to the confined environment. Each handoff
has its own socket and its own variable.

Ravel keeps the token bytes only from the read to the write. It writes them to
no notice, diagnostic, refusal, request, log or file, and it sends them to no
worker.

The descriptor is a socket because of the reader in Claude Code 2.1.317:

- It reads a socket directly, to the end of the stream. Then it removes the
  variable from its environment.
- It opens any other descriptor by the path `/dev/fd/N`. It reads the
  descriptor directly only after `EACCES` or `EPERM`.
- Ravel's Seatbelt profile reports `/dev/fd/N` as absent, with `ENOENT`.

A pipe therefore leaves a confined Claude Code without a sign-in. A probe
under a copy of the profile measured the two reads with dummy text. The read
by path failed for a pipe and for a socket. The direct read passed for both.

The token is not in an environment variable, because every descendant inherits
the environment. An agent could then print the token.

### What a confined process can read

- **The token file.** No confined process can read it, because Ravel refuses
  every grant that overlaps it.
- **The Keychain and the sign-in stores.** No confined process can read them.
  The profile gains no Mach service and no Keychain read for this feature.
- **The socket.** Claude Code reads it to the end of the stream during its
  initialisation, before the agent acts. A descendant that inherits the descriptor then reads
  only the end of the stream.
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
token, so a confined Claude Code cannot refresh it. Nothing that a confined
session does can rotate or revoke the owner's own sign-in.

### macOS and Linux

The handoff is the same on both platforms. The token file is an ordinary file.
Ravel opens no Keychain item on macOS and no `.credentials.json` on Linux.
Seatbelt and bubblewrap both pass an inherited descriptor to the program that
they launch. Claude Code reads a socket directly on both platforms.

The probe measured Seatbelt only. The Linux fixture of the implementation
measures bubblewrap before this feature can pass.

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
| The overlap code of the settings file | As for the settings file | As for the settings file | A grant overlaps a declared token file |
| `selection_malformed` | `selection` | 3 | The selection names an undeclared handoff, or repeats a name |
| `token_handoff_unavailable` | `handoff` | 3 | A named token file failed a check |

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
- **THEN** a fake harness reads the exact bytes of the file and then the end
  of the stream, from the descriptor that its variable names
- **AND** the descriptor is a socket
- **AND** an open of the token file inside the launch fails

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

- **WHEN** the date of `expires` is today or earlier
- **THEN** the launch refuses as `token_handoff_unavailable`, exit 3
- **AND** Ravel creates no result channel and launches no managed process

#### Scenario: A file open to others

- **WHEN** the token file has mode `0644`
- **THEN** the launch refuses as `token_handoff_unavailable`
- **AND** the remedy asks for mode `0600`

### Requirement: The token stays out of Ravel's output

Ravel SHALL write no byte of a token file to its stdout, its stderr, a notice,
a refusal, a request, the worker or a log.

#### Scenario: Leak probe

- **WHEN** the token file holds a sentinel and a confined launch delivers it
- **AND** another launch refuses because the file is open to others
- **THEN** the sentinel is in no output of Ravel, no notice, no failure
  document and no request that the script receives

### Requirement: Protected token files

Ravel SHALL refuse a confined launch when a grant overlaps a declared token
file.

#### Scenario: A token file in the working directory

- **WHEN** a declared token file is in the working directory
- **THEN** every confined launch refuses before the worker launches, as for an
  overlap with the settings file

### Requirement: Harness runs

A confined harness run SHALL receive the handoffs that its request names,
under the same rules.

#### Scenario: A run that names an absent file

- **WHEN** a confined harness run names a handoff whose file is absent
- **THEN** the run is `unavailable` with `token_handoff_unavailable`
- **AND** it uses no budget, and the script continues

## Test seams

- **Ravel's public boundary, with fixtures.** `start-session` and
  `ravel::run_harness` cover every requirement above. Each test has a
  temporary home, a fake harness and a dummy token file. The fake harness
  reports its variable, the type of its descriptor, the bytes that it reads and
  an attempt to open the token file. These tests run in `task check` on macOS
  and in the Linux lane under bubblewrap. They need no owner credential.
- **The real profile, without a model call.** This check is opt-in. The human
  approves it before Ravel reads the owner's token. The selected program runs
  `claude auth status` under Ravel's own `--confine`, in a sandbox home with
  its own settings. With the handoff, Claude Code reports a sign-in. The
  control, whose selection names no handoff, reports no sign-in. The record
  holds no byte of the token.

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
