# A configuration script asks a model through a harness run

A configuration script asks a model a question with `ravel::run_harness`. The harness is
Claude Code or Codex, which the owner names in settings. The harness signs in
with the owner's own subscription. Ravel never reads that sign-in, and it never
calls a model service itself. It builds no argument that is specific to a
harness, and it parses no output of a harness. The
[configuration evaluation specification](../specs/configuration-evaluation.md#llm-calls-through-a-harness)
states the rules.

A confined harness run can receive a token that the owner minted, as
[a confined harness receives an owner-minted token](a-confined-harness-receives-an-owner-minted-token.md)
records.

The trade-off is the sign-ins that the owner already uses against the weight of
a harness process. A subscription sign-in works only in its own harness.
Anthropic permits the sign-in of a Claude subscription only in the unmodified
Claude Code program. Its terms forbid developers to collect, store or
intermediate a sign-in or a session token
([legal and compliance](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use)).
The Codex documentation tells a general API caller to use a Platform API key
([Codex authentication](https://learn.chatgpt.com/docs/auth)).

A harness run uses those sign-ins as they are, and it needs no API key. The
release also loses Rig, its HTTP client and its TLS stack. Rig released seven
minor versions from June to October 2026. Each one needed a proof against a real
exchange, and a real exchange needed an API key that the owner does not use.

The price is a harness process for every LLM call. It costs seconds of start-up
and the context of the harness, where an HTTP request costs one round trip.
Ravel cannot count the tokens of a call, and it cannot check an answer in the
transport. The script reads the usage from the output, and `ravel::json_check`
checks the shape of a structured answer. Each subscription has rate limits of
its own, which Ravel cannot see.

The design rejected four other shapes:

- `ravel::llm` through Rig, with Anthropic, OpenAI and OpenAI-compatible
  endpoints in settings. GitHub issue 2 asked for it, and Ravel built it. The
  owner then removed it and amended the issue, because the owner's sign-ins
  cannot fill its credential slot.
- A fourth endpoint type whose transport is a harness. Ravel would then build
  the arguments of each harness and parse its output, and a harness release
  would change Ravel.
- A nested `start-session` inside a harness run, with a depth bound in place of
  the re-entry refusal. Each level would have budgets of its own. The outer stop
  cannot reach the process group of the nested managed process, and both levels
  use a kill-grace of 5 seconds. The `ravel` on `PATH` can also be another
  version.
- A host call that evaluates the configuration script again for a nested request. It keeps one
  supervisor, but it is the most new code. The configuration script already holds its own
  route table, so it can call its route function directly.

Reconsider when an owner needs a model service that has no harness. Reconsider
also when the start-up of a harness makes an LLM-assisted selection too slow for
its deadline.
