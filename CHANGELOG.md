# Changelog

## Unreleased

- Reword the `ravel end-session` help text: the LLM sends the result, and Ravel
  keeps the first result that it receives. The behavior does not change.

## v1.0.0

- Declare the two-verb public CLI stable: `ravel start-session` and
  `ravel end-session RESULT`, with opaque caller-owned results, owner policy
  routing and confinement behaving exactly as released in 0.1.0.
- Commit to semantic versioning for that CLI: compatible changes arrive in minor
  and patch releases; incompatible changes require a new major version.
- Ship the same matching front and private worker, generic SDK/examples and
  runtime notices as 0.1.0, with no source or behavior change.

## v0.1.0

- Introduce the independent Ravel session process manager with `start-session`
  and `end-session RESULT`, preserving opaque results and owner policy routing.
- Ship a matching front and private worker, generic SDK/examples and runtime
  notices without a Grove or host TypeScript runtime dependency.
- Add independent release tasks, exact three-target archives, installed-layout
  smoke checks, Linux glibc/CPU-floor controls and a tested Homebrew formula.
