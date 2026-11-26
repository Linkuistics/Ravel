# Skill-enabled external source and fragments evaluation

## Outcome

The frozen case is incomplete and cannot support an enabled-versus-control
comparison. The schedule began with control repetition 1 followed by enabled
repetition 1. The control process returned a normal refusal, but its raw stream
does not contain the blocked shell calls or direct sandbox results that the
answer says it received. It is therefore preserved and scored as a descriptive
behavioral sample with an access-audit protocol breach, not as comparison
evidence.

Enabled repetition 1 exhausted all three permitted attempts. Attempts 1 and 2
called `list_mcp_resources` and `list_mcp_resource_templates`, exposing plugin
catalog material outside the sole declared fixture. Attempt 3 issued an empty
web search. Each violates Case B's model-interface access boundary. The frozen
third-invalid rule stopped the leaf at that point; the remaining eight schedule
positions were not executed.

There is no valid enabled sample, no complete contemporary control arm, and no
historical baseline Case B sample. Per-criterion enabled success counts,
variance, material-improvement classifications, regression checks, and a
same-case verdict are therefore unavailable. This outcome establishes nothing
about whether the skill improves source inventory, concept ordering, fragment
design, worked executions, or mechanical validation. The skill was not refined.

## Frozen execution contract

- Codex CLI: `codex-cli 0.150.1`
- Executable: `/opt/homebrew/bin/codex`
- Executable SHA-256:
  `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- Model: `gpt-5.4`
- Reasoning effort: `high`
- GNU timeout: `9.11`, 20 minutes with a 30-second kill grace
- Prompt SHA-256, including its canonical terminal LF:
  `2a808c5f6af50b47690399ddc5005f3d5a90f192340281937b9eed68ad226ba8`
- Fixture SHA-256:
  `2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`
- Skill revision: `9cc8ccd8c5b8f070a572378bd61953f7b3bbb8ac`
- Installed `SKILL.md` SHA-256:
  `795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006`
- Skill manifest aggregate SHA-256:
  `1f530317f817805eb88bf45f8ba66a0f135f8b279f36df2c5d864cfcd26251c1`
- Schedule: `control enabled enabled control enabled control control enabled control enabled`
- Evaluation root: `/private/tmp/grove-enabled-source.kD2kHt`

The sealed control template contains only `auth.json`. The enabled template
differs only by `skills/writing-code-walkthroughs/SKILL.md`; deleting that row
from its manifest makes the two manifests byte-identical. The installed skill
bytes match the preceding enabled scope-elicitation slice. The prompt file is
byte-identical to the frozen Case B prompt, and the harness preserves its final
LF in the command argument with a sentinel before verifying the digest.

Every attempt used a fresh copied home and a fresh run directory outside this
repository containing exactly
`targets/ocaml/check_floor.ml`, 6,698 bytes, with the frozen digest. All four
post-run manifests equal their pre-run manifests. The operating-system
readability limitation from the rubric still applies; these records make claims
only about the model interface visible in the raw streams and final answers.

The exact campaign command was:

```text
env -u CODEX_CI -u CODEX_PERMISSION_PROFILE -u CODEX_SANDBOX \
  -u CODEX_SANDBOX_NETWORK_DISABLED -u CODEX_SESSION_ID \
  -u CODEX_THREAD_ID -u GROVE_SIGNAL_FILE -u HERDR_ENV \
  -u HERDR_PANE_ID -u HERDR_SOCKET_PATH -u HERDR_TAB_ID \
  -u HERDR_WORKSPACE_ID CODEX_HOME=<fresh-codex-home> \
timeout --signal=TERM --kill-after=30s 20m \
codex exec --ignore-user-config --ignore-rules --ephemeral \
  --skip-git-repo-check --sandbox read-only --model gpt-5.4 \
  -c model_reasoning_effort='high' \
  -c skills.bundled.enabled=false --json \
  --cd <fresh-run-directory> <frozen-prompt-with-terminal-LF>
```

## Attempt accounting

| Schedule position | Arm | Planned repetition | Attempts | Selected sample | Outcome |
|---:|---|---:|---:|---|---|
| 1 | control | 1 | 1 | attempt 1, protocol-breached | Exit 0 and final answer; blocked shell calls/results absent from raw event items |
| 2 | enabled | 1 | 3 | none | all three: unstructured access-audit breach; attempts 1-2: prohibited MCP catalog access; attempt 3: prohibited web search |
| 3-10 | mixed | remaining 8 | 0 | none | unexecuted after third-invalid stop |

All three invalid enabled attempts said they intended to read walkthrough skill
instructions. Attempts 2 and 3 named `writing-code-walkthroughs` exactly;
attempt 1 used only the generic phrase “walkthrough skill instructions.” No
attempt visibly read the skill body. Discovery among valid enabled samples is
undefined because there are no valid enabled samples; the exact-name
announcement count among invalid attempts is `2/3` and is not a behavioral
result.

## Descriptive atomic score

A fresh blind context scored the one anonymous behavioral sample from its final
answer and raw events. It received no arm label, path name, skill bytes, or
campaign hypothesis. The primary output incorrectly treated an intention to
create an inventory as the inventory (`B01`) and read-only source inspection as
read-only, non-regenerating validation (`B19`). The resolution applies the
frozen conjunctive wording, changing both to zero. The resolved score is `1/27`:
only absence-shaped `B18` succeeds because the complete answer surface never
offers compilation or copied-snippet presence as a substitute for byte equality.

| Criterion set | Historical baseline | Contemporary control | Enabled |
|---|---:|---:|---:|
| `B18` | `0/0` | `1/1` descriptive | `0/0` |
| every other `B` row | `0/0` | `0/1` descriptive | `0/0` |

The case is not complete, so the rubric's independent second scoring of one
complete case was not commissioned. The blind bundle, primary raw output,
primary citations, mapping, and resolution are preserved under
[`adjudication/`](adjudication/).

## Infrastructure limitation

Nested shell sandbox setup failed before every attempted command. The returned
answers quote `sandbox-exec: sandbox_apply: Operation not permitted`, but the
Codex JSONL streams contain no corresponding `command_execution` item or direct
tool result. This prevents the required call-by-call audit for the selected
control and all three enabled attempts. The enabled streams say they intended
to read skill instructions outside the run directory as well as the fixture,
but the absent call events do not expose operands well enough to determine
whether an outside-boundary shell call actually began. That protocol breach is
kept distinct from the structured MCP and web accesses that independently
invalidate all three enabled attempts.
