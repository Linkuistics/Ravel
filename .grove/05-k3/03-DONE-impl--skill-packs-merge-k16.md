# skill-packs-merge-k16

## Goal

Bring RequirementsSkills, EngineeringSkills, ResearchSkills and WritingSkills
into the Ravel repository, each as one snapshot, on the terms of
`merge-terms-k15`. Then delete their local checkouts.

## Context

- The running log of `merge-terms-k15` holds the terms. They cover the
  snapshot, the layout, the mise rule, the root `check` and the deletion of
  the local checkouts.
- `product-layout-k34` runs first, so the root `Taskfile.yml` and `mise.toml`
  exist.

## Warnings

- Warning: Do not delete a pack checkout before you show that GitHub holds its
  work. A deleted checkout cannot be restored.

## Done when

- Each pack is in `packs/<name>/` as one commit that holds the files of its
  trunk. The names are `requirements`, `engineering`, `research` and
  `writing`. Each commit message names the source repository and the commit.
- `packs/mise.toml` declares every program that the pack tasks call. The
  plugin check of each pack no longer skips when Claude Code is absent.
- The root `check` runs `mise exec -C packs/<name> -- task check` for each
  pack, and each check passes. The launcher's check still passes.
- The root `README.md` lists the four packs, each with its directory.
- The four local checkouts under `~/Development/AgenticOrchestration/` are
  gone.
- No change goes into or from Grove or Ravel-Strategies, and PersonalSkills
  stays as it is.

## Notes

- A pack checkout holds nothing that GitHub lacks when its `main` equals
  `main@origin`, its working copy is empty and no other change holds work. Its
  ignored files are regenerable: Grove checkouts in `.cache/`, a `ruff` cache
  and jj metadata. On 2026-11-26 all four checkouts met that test.
- Each pack Taskfile builds its paths from `{{.ROOT_DIR}}`. A separate run in
  the pack's directory keeps those paths right. So leave each Taskfile as it
  is, apart from the skip in its plugin check.
- Each pack check fetches its own pinned Grove commit from GitHub, and each
  pack keeps its pin.
- If the four packs do not fit one session, finish whole packs, and leave the
  rest in a live sibling. Each merged pack is a working state.
- Check that the merged packs hold no owner configuration or secrets.

## Decisions (running log)

**Each snapshot is exact.** `git archive` of each source trunk wrote the files
into `packs/<name>/`, and each pack took one commit. The Git tree of each
`packs/<name>/` equals the tree of its source trunk:

| Pack | Source commit | Tree |
| --- | --- | --- |
| `requirements` | `efaeeef917d89383902142d99c227db1df14f888` | `07cc422266ba4966aaca23cc2eb204a503fe5aa3` |
| `engineering` | `55aff4acbd58c9d27837d3496bb97b08b216b258` | `85ea290a8014431d8e3786fd3cc322240ec30ba9` |
| `research` | `67cb6c856d93d8102e3dbc2d29519fd39cfb365d` | `111891bf9a78d808875e06fd84d9fa0612f47651` |
| `writing` | `c15934c379fb9ac6e065bb540d28f68001016d21` | `6a295d08cadfac18e0276ee2933bf422fcaec298` |

No snapshot commit touches a path outside its own pack. A control compared the
research snapshot with the engineering trunk, and it reported a difference.

**The packs hold no secrets and no owner configuration.** A scan looked for
key formats, private keys, configuration files, home paths and email
addresses. A planted token showed that the key search can find a hit. The scan
found no key and no configuration file. It found two kinds of personal data:

- The owner's email is the author in six plugin manifests.
- Evaluation evidence in `packs/writing/docs/evaluations/` names two local
  checkout paths under the owner's home.

The four source repositories are public on GitHub, so GitHub publishes both
already. The snapshots stay unchanged.

**`packs/mise.toml` declares the programs of every pack task.** The tasks
and their scripts call `task`, `git`, `bash`, `claude`, `python3`,
`shellcheck`, `pi` and `d2`. The pins match the launcher's where both
declare a tool. The utilities that macOS ships in `/bin` and `/usr/bin` stay
outside, as in `launcher/mise.toml`. Grove's conformance runner calls only
`bash` and those utilities.

**Pi comes from mise at version 0.84.2.** The owner's machine ran Pi 0.84.2
from a global npm package. mise's `pi` entry installs the same release as a
binary with a checksum. The newer 1.x releases stay out until the Pi checks
pass against them. The Pi load check and `check:hook-writes` loaded the
extension with this binary.

**The Pi skip stays in its two scripts.** `scripts/pi-extension-load.test.sh`
and `scripts/hook-writes.py` still skip when Pi is absent. This leaf changes
only the plugin check, as its notes direct. In a run through mise, Pi is
always present, so neither script skips. The writing README says so.

**The pack READMEs describe the new check.** Each README said that the
plugin check skips without Claude Code, and the removal of the skip made that
false. Each README now runs `mise exec -- task check` and names
`packs/mise.toml`. Other sentences about the source repository wait for the
landing and the shipping leaves.

**The root check passes.** `mise exec -- task check` at the root ran the
launcher's check and the check of each pack, with exit status 0. The
working-copy commit was the same before and after the run. Each plugin check
ran `claude plugin validate`, and each conformance run reported that every
assertion holds. A control ran the research plugin check without `claude` on
the path, and it failed with exit status 127.

**The four local checkouts are gone.** Just before each deletion, the session
fetched the checkout and tested it again. In each checkout, `main` equalled
`main@origin`, and the working copy was empty. No other change held work, and
the checkout had one workspace and no stash. Then the session deleted
`RequirementsSkills`, `EngineeringSkills`, `ResearchSkills` and
`WritingSkills` under `~/Development/AgenticOrchestration/`. The owner's Ravel
settings name none of them.

**No adversarial review of this leaf.** This is the session's judgement, not
the owner's decision. The snapshots are byte-identical to their sources, and
the tree hashes prove it. The other changes are a tool list, the removal of
one skip and a list of directories. The root check and its controls test
them, and the landing leaf runs the root check again.
