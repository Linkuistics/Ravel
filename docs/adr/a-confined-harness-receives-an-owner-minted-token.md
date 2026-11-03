# A confined harness receives an owner-minted token

A confined Claude Code cannot reach its sign-in in the Keychain. Ravel hands it
a token instead. The owner mints the token with `claude setup-token` and names
its file in the settings. Ravel reads that file before the launch and sends the
token through an inherited socket. Ravel never opens the sign-in store of a
harness. The [token handoff specification](../specs/token-handoff.md) states the
rules.

The trade-off is the owner's work against the reach of a confined agent and
Anthropic's terms. The owner mints a token each year and keeps its date in the
settings. In return, no grant of a confined launch reaches the token file, and
the handoff adds no Keychain or sign-in-store grant. A confined agent can read
a sign-in store only through a grant that the caller names. The handoff gives a
confined session nothing that can rotate the owner's own sign-in. The route is
the same on macOS and Linux.

The design rejected these routes:

- **Ravel reads the sign-in of Claude Code.** The owner chose this route first,
  on 2026-11-03. Ravel would read the Keychain item, or `.credentials.json` on
  Linux, and pass on only the access token. Anthropic's terms forbid developers
  to collect, store or intermediate Claude.ai credentials or session tokens
  ([legal and compliance](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use)).
  The route also ties Ravel to a private format of Claude Code. A run also ends
  when a short-lived access token expires.
- **A Keychain grant in the profile.** Claude Code signs in under a profile
  that adds two Mach services and reads of `~/Library/Keychains`. A confined
  agent could then ask for any Keychain item that the user can read.
- **A runtime read of the token file.** The owner's wrapper would read a
  `--runtime-read` grant and export `CLAUDE_CODE_OAUTH_TOKEN`. Ravel would
  hold nothing. But every confined process could read the one-year token and
  send it over the open network. Each caller would also need the grant.
- **A plaintext sign-in file inside the grants.** Claude Code keeps
  `.credentials.json` beside its Keychain store. A copy holds the refresh
  token, so a confined refresh would rotate the owner's sign-in.
- **A pipe.** Claude Code opens a pipe by the path `/dev/fd/N`, and the
  confinement profile reports that path as absent.
- **The token in an environment variable.** Every descendant inherits the
  variable, so the agent could print the token.

Reconsider when a harness offers a sign-in service that a sandbox can admit for
one item only. Reconsider also when Anthropic's terms or the token variables of
Claude Code change.
