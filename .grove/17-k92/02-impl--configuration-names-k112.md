# configuration-names-k112


## Goal
Rename environment grants to `--config-env` and `configEnv`, and the evaluation
marker to `RAVEL_CONFIG_RUN`, through every current producer and consumer.



## Context
Follows default discovery in `configuration-names-k111`. The parent owns the
complete contract. Inspect CLI parsing, settings, environment, harness requests,
worker/front and confinement consumers before changing names.

## Done when
- Both flag and settings grants work through the real CLI fake-harness seam.
- `--policy-env` and `policyEnv` refuse rather than alias; old-input test cases
  remain exact. Tests cover duplicate/reserved grants and unchanged authority.
- `RAVEL_CONFIG_RUN` causes re-entry refusal before evaluation; it is propagated
  only to harness runs and scrubbed where required. Stop order remains unchanged.
- Update current documentation, examples, smoke scripts and checks that consume
  these names. Keep exact excerpts and examples executable.
- Relevant tests, `task check`, `task test`, examples and excerpts pass.

## Notes
Do not change the installed driver, its ending command or live settings. JSON
output names and remaining artifact filenames have later children. Record graph
availability and test/platform limits.
