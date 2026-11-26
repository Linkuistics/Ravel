---
name: simplify-project
description: Contract-preserving project simplification — consolidate current-state documentation, remove proven-dead code and artifacts, shrink dependency and directory surface, and reconcile every consumer. Use when a mature repository has accumulated duplicate docs, historical plans, stale experiments, compatibility scaffolding, or structure that no longer earns its maintenance cost.
harnesses: [any]
---

# Simplify Project

Reduce the repository's **maintained present** without obscuring a behaviour
change. The goal is fewer maintenance obligations and clearer ownership, not a
large negative line count. Version control already holds the past.

The contract is wider than runtime logic: commands, help text, configuration,
persisted formats, installer paths, package contents, release mechanics, embedded
content, and documented integration points are all observable behaviour.

## Establish the preservation ledger

Define the allowed change before proposing deletions. Record the surfaces that
exist in this project — public commands, flags, output, errors and exit status;
configuration, defaults, environment variables, persisted formats and supported
migrations; packages, binaries, workspace membership, dependencies, installers and
release artifacts; embedded or generated content, automation, examples and
downstream integrations — each with its consumers, whether it is preserved or
changed, the evidence, the destination, and how the claim is verified.

| Surface or artifact | Owner / consumers | Preserve or change | Evidence | Action / destination | Verification |
|---|---|---|---|---|---|
| `tool --help` | CLI users, docs | preserve | captured output | keep grammar | snapshot comparison |

Run the existing checks and capture representative public outputs **before**
editing, recording pre-existing failures separately. A green post-change suite
proves current correctness; only a before/after comparison supports an equivalence
claim.

If simplification requires an observable change, put it in a separate ledger and a
separate change, named as compatibility work rather than hidden inside cleanup.

## Map ownership and reachability

Inventory top-level products, entry points, packages, scripts, documentation,
local or generated state, and release paths. Prefer structural graph or language
tools for code reachability; use text search for paths, links, configuration,
manifests, comments, and non-code consumers. Check downstream repositories and
published paths where the project exposes them.

**No search depth or deprecation duration proves an external consumer absent.**
Set the exit criterion from the project's support policy and an authorized
contract owner — do not assume the requester holds that authority. Ask them when
their answer would change a removal's disposition; **Defer** is for where the
authority or the evidence is genuinely unavailable, not for an unasked question. Replace an old
implementation with a thin compatibility shim only when the shim preserves its
complete public interface; otherwise retain it.

Assign each subject one canonical owner. A useful documentation default —
orientation and installation in the root README; commands and workflows in a usage
guide; settings and local state in a configuration guide; current design and
binding rationale in an architecture guide or focused ADR; recurring operator
procedures in a release or maintenance runbook; component-specific operation
beside the owning component. Adapt the set; do not create empty categories.
Organize by reader intent and ownership, never by the chronology in which facts
were discovered.

## Classify with evidence

Give every candidate one disposition:

| Disposition | Evidence required |
|---|---|
| **Keep** | Current contract, live migration/adapter, unique binding rationale, operational procedure, licence, or provenance. |
| **Fold** | A current fact is duplicated or fragmented; select one owner and migrate every unique fact and stable anchor. |
| **Move** | The artifact has a clear component owner; reconcile internal and external path consumers. |
| **Delete** | No supported consumer remains, durable conclusions already have a home, and VCS is the only reason to retain the narrative. |
| **Defer** | Reachability, compatibility policy, ownership, or legal provenance remains uncertain. |

Use the deletion test from `codebase-design` for modules and abstractions, and
`decision-records` when pruning ADRs.

Typical delete candidates: completed plans, superseded specs, task logs, research
whose durable findings were promoted, stale generated state, one-off experiments,
unreachable packages, and their dependency closure. A compatibility adapter is not
stale merely because it is old — retain it while a supported input, persisted
format, or downstream consumer still crosses that seam.

## Design the target state

Before deleting, write down the target product, package, binary and dependency
set; the canonical owner of each retained fact; an old-to-new path and anchor map;
the evidence for each removal; and every approved observable difference.

Prefer fewer, deeper documents and modules over one giant document or a prettier
folder tree. Consolidation succeeds when a reader or caller has fewer surfaces to
learn while the necessary behaviour and rationale remain available.

## Execute in coherent slices

1. Create or reshape the canonical owners, migrating unique current facts,
   invariants, failure modes, provenance, and stable anchors.
2. Reconcile inbound references: source comments, tests, help, READMEs, manifests,
   automation, packaging, catalogues, release instructions.
3. Move component-owned material beside its owner — separately from any semantic
   rewrite, when that makes review clearer.
4. Delete superseded artifacts. Remove a dead executable or package as one
   complete maintenance obligation: source, workspace/build entries, dependencies,
   tests, packaging, lockfile residue.
5. Update the changelog and any user-facing migration or deprecation guidance.

Keep each slice reviewable and reversible through the repository's native VCS, and
run the relevant checks after each rather than deferring all feedback to the end.

## Prove the result

- Re-run formatting, linting, tests, builds, and repository-specific checks.
- Compare the ledger's public outputs, schemas, defaults, package metadata, binary
  list, installed tree, and release artifacts before and after.
- Exercise installers, migration paths, packaging, and release dry-runs in
  isolated temporary environments — never test destructive install behaviour
  against the operator's real home or live configuration.
- Search for every removed name, path, slug, anchor, package, and dependency,
  classifying intentional history-only hits such as changelog entries.
- Check documentation links, anchors, navigation, ownership statements, and
  contradictions between canonical guides. Every link can resolve while the prose
  still names an owner, a package or an install path that no longer exists.
- Inspect the final diff and VCS status for unrelated files and tool-generated
  residue.

Complete when every contract difference is approved, every deletion has evidence,
every retained fact has one canonical owner, live references resolve, supported
migrations still work, and the repository has fewer concepts to maintain. Lines and
files removed are supporting metrics, never the acceptance test.

## Common mistakes

| Mistake | Correction |
|---|---|
| Treating "no runtime logic changed" as equivalence | Include help, paths, packaging, embedded content, and operations in the contract. |
| Keeping historical artifacts "for context" | Promote current lessons; let VCS hold the narrative. |
| Deleting before consolidation | Establish canonical owners and migrate unique facts first. |
| Trusting no in-repo callers | Check release, automation, documentation, and downstream consumers. |
| Optimizing line count | Optimize maintenance obligations and interface depth. |
| Removing compatibility because it looks old | Require support-policy and consumer evidence. |
| Running only post-change tests | Capture baselines and compare observable surfaces. |
