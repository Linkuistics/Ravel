# Existing owner configuration: installed check

On 14 November 2026, the installed Ravel 2.0.0 passed all 72 checks of the
owner's existing Rhai routing table: 24 named kinds in each of three working
directories. This establishes the current behavior. It does not establish
equivalence to the original TypeScript configuration, which was absent from
the live configuration directory when this session began.

The original task expected to create and seek approval for a candidate before
copying it into the live directory. That directory already held `config.rhai`
and `settings.json`, with no `policy.ts`. This session made no live change.
The owner approved closing the historical conversion task on these
current-behavior checks and waived the unavailable old/new comparison. This
acceptance does not establish historical file preservation or continued 1.1.0
compatibility with the current shared settings.

## Subjects and method

- Binary: `/opt/homebrew/Cellar/ravel/2.0.0/bin/ravel`.
- Older binary: `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`.
- Copied configuration: `/Users/antony/.config/ravel`.
- Rhai source SHA-256:
  `dae11e833f51a9a568446c64c436f3b6e88bd57c6cd52c70ccb5defd687a398a`.
- Settings present: mandatory skill guidance under `skills`.

[verify.py](verify.py) is the one-off check, and [results.json](results.json)
holds each result, the captured skill-source digests, and the before/after
digests of the live configuration and both installed kegs. The personal Rhai
source is not copied into this repository.

The sessions used a private home containing a copy of the real configuration,
a PATH containing fake `bili`, `codex` and `claude` programs, and a macOS
sandbox denying reads and writes beneath the real home and denying network
access. A control confirmed that reading the real home was refused. Ravel's
installed default skill directory remained available outside that home.

The fake `bili` recorded its complete argument vector and skill descriptor,
then called the installed `ravel end-session verified`. It did not invoke a
real model harness. The checks compared all routing arguments against the
explicit expected table and compared the caller block byte for byte, including
its Unicode, newline and shell metacharacters. Ravel's skill preamble and
caller-block markers were checked separately. All sessions reported the same
skill-source digests.

## Results

| Case | Checks | Result |
|---|---:|---|
| Plain working directory | 24 | Exact routing arguments; no extra directory grant. |
| Primary jj repository, `.jj/repo` a directory | 24 | Exact routing arguments; no extra directory grant. |
| Secondary jj workspace, relative `.jj/repo` pointer | 24 | Exact routing arguments, including `--add-dir` for the main repository. |
| Unlisted kind | 1 | Exit 3, `incomplete_mapping`; no fake launched. |
| 1.1.0 with the copied current settings | 1 | Exit 2, `settings_invalid`; no fake launched. |
| Configuration and both installed kegs | 3 digests | Identical before and after the run. |

The wrapper returned exactly `verified` in every successful case. The JSON
diagnostics also reported two unsuccessful process-group kill attempts per
session, with `Operation not permitted (os error 1)`. Those diagnostics are
retained per row. These checks cover selection and argument delivery; they
make no claim about descendant-cleanup behavior.

The 1.1.0 refusal confirms that the original shared-settings coexistence plan
does not describe the current configuration. It provides no evidence about
what the missing original TypeScript selected. The expected routing table in
this check was derived from the existing Rhai file and its explicitly reviewed
model/effort choices, so this is an execution check rather than an independent
reconstruction of the old policy.

## Remaining owner choices

The driver that launched this session uses the versioned 2.0.0 command.
There is no `policy.ts` in the live directory to remove. Removal or preservation
of that file before this session cannot be established by this check.

Both installed versions remain intact. The owner can choose when to run
`brew cleanup ravel` to remove the older keg. The new optional `harnesses`,
`budgets` and `configEnv` settings can be considered for a 2.0.0-only setup;
the current Rhai file supplies its own harness commands and needs none of
those settings to pass these checks. Any remaining 1.1.0 consumer would first
need settings that it accepts.

## Repeating this one-off check

On this machine, with these installed subjects still present:

```sh
mkdir -p /tmp/ravel-owner-k39
mise exec -- python3 docs/evidence/owner-policy-conversion-k39/verify.py
```

The check writes a fresh `/tmp/ravel-owner-k39/results.json`, leaving this
archived result intact. It intentionally fails if the named kind set, selected
model or effort, or argument contract has changed. It uses only Python's
standard library. Ruff and Pyright 1.1.414 in strict mode checked the archived
script successfully. No product source changed, so the product test suite
was not rerun.
