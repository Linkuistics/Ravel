# Test-isolation guards and legacy-name exceptions

This note records the test-isolation guards that keep tests from acting as the
enclosing development session, and the legacy `GROVE_*` and `HARNESS_DISPATCH_*`
spellings that those guards deliberately keep. Ravel's authority boundaries and
their limits are in the book, in
[chapter 2](book/src/02-actors-and-words.md#trust-and-confinement) and
[chapter 5](book/src/05-running-sessions.md#understand-the-operational-limits).

Tests must never act as the enclosing development session. Cargo's
[forced-empty controls](../.cargo/config.toml) cover old launch/signal/dispatch
controls and current/obsolete Ravel spellings. Empty values disable authority;
they do not redirect it to another path. Runner subprocess helpers independently
scrub the same names when invoked directly. Front/hostile command fixtures use
env_clear plus temporary HOME/TMPDIR and fixture PATH; installed smoke uses env -i.
No test inherits personal owner policy or a real completion path.

The legacy GROVE_* and HARNESS_DISPATCH_* spellings in those guards are intentional
extraction safety exceptions, not product dependencies. Caller-specific paths in
source provenance and the retired project document are historical attribution.
Ravel neither loads nor interprets those projects or their task trees.
[The caller-reference inventory](caller-references.md) classifies active
references, including dotfiles.
