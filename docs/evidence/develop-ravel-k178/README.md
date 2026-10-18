# Chapter 11 part B: command and document checks

These records support the second draft part of `11-develop-ravel.md` on
18 October 2026. The starting parent is `d72aaf29`, the part A draft.
The host is macOS 27.0.1 (26A434), arm64, with Rust 1.99.1 (Homebrew),
Task 3.54.0 and nextest 0.9.148. Existing checkout tools, dependencies and
build caches were available. This was not a clean-machine setup trial.

## Commands and results

`results.json` records each command, its exit status, elapsed seconds and log.
All sixteen commands returned status 0. Their logs are beside this note.
The installation and smoke wrapper logs name nested worker logs, which this
directory also preserves as `install-worker.log` and `smoke-worker.log`.
`probes-worker.log` preserves the probe build details.

| Check | Observation |
|---|---|
| `task install PREFIX=/tmp/ravel-k178-checks/install-prefix` | Release pair and direct/symlink installed checks passed at the disposable prefix |
| `task smoke` | The same checks passed in its own temporary prefix |
| `task probes` | Four test-only worker builds passed |
| Book build and figures | Build passed. Thirteen figure SVG/layout pairs matched their sources |
| Complete examples | Fourteen passed, zero failed, zero skipped. Sixteen fragments did not run |
| Source excerpts | Eight passed, zero failed, zero page errors |
| Strict STE and prose breaks | No reported problem in 26 text/figure files or ten chapters, respectively |
| Figure and example controls | Both tasks passed |
| Example mutants | All 29 removed-safeguard copies failed their assigned controls |
| Excerpt controls and mutants | Controls passed. All 24 mutants failed their assigned controls |
| STE and prose-break controls | Both tasks passed their ordinary controls and rule mutants |
| Book integrity | Offline fetch, cache and refusal controls passed |

`serve.log` and `serve-result.txt` record `task docs:book PORT=18778`.
The chapter returned HTTP 200 and contained both boundary headings of part B.
The controller stopped only its own server process group after the request.
This check did not inspect a browser view or exercise a change-triggered rebuild.

`figures-update-scratch.log` records `task docs:figures:update` in the existing
`/tmp/ravel-k160-checkout` scratch workspace. All 26 SVG/layout output files
matched this checkout byte for byte. `figures-update-result.txt` records that
comparison. No primary-checkout figure was rewritten.

## Source freeze and limits

`subjects-before.json` and `subjects-after.json` contain identical SHA-256 maps
for 264 files. The set is the tracked working tree except `.grove/` and
`docs/evidence/`. It includes the chapter, book configuration, figures, prose
decisions, scripts, script fixtures, Rust/TypeScript source, manifests, lockfiles
and Cargo configuration. The command sequence had one writer and stopped on
any nonzero status. No measured source changed during it.

The manifests exclude generated outputs, installed trial files, tool binaries,
host caches, worker dependencies and external services. They are not a digest
of the whole machine. Preliminary install, smoke and probe trials also passed
before this recorded sequence; only the sequence above has before/after maps.

After that sequence, the chapter gained one first-use policy-context link.
`final-document-checks.log` records the fresh build and strict prose checks.
The final source digest record identifies that chapter version. No command or
code fragment changed after the complete example and excerpt runs.

`inventory.txt` records all 57 public names from a fresh `task --list`.
Each appears exactly once in the chapter's final inventory, with a destination.
A removed-name control failed the comparison. Part A remains byte-for-byte
unchanged. The final link check records its missing-file and missing-anchor
controls in `final-document-checks.log`.

No personal installation, policy or real model was a trial subject. No native
campaign, Linux measurement, release cut or remote delivery ran. These checks
do not add platform support, qualify the minimum Rust version or establish
reader comprehension. Part A's failed full check and unchanged successful
rerun remain in `../develop-ravel-k160/` with their original limits.
