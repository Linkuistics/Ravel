# Owner settings name the LLM endpoints

An LLM endpoint exists only in the owner's `settings.json`, with its API type,
its base URL and the environment variable of its credential. A policy names an
endpoint in an LLM call. It never supplies a URL or a credential, and no host
capability gives it one. Only the Ravel CLI reads the credential, at the time of the call, and
it sends the credential only to that base URL.

The HTTP client of Ravel ignores ambient proxies, follows no redirect and makes
no retry. Ravel 2.0.0 offers three API types: Anthropic, OpenAI and
OpenAI-compatible endpoints. The
[policy evaluation specification](../specs/policy-evaluation.md) states the
entry fields, the URL rule and the credential rules.

The trade-off is containment against convenience. A script cannot reach a model
service that the owner did not configure, and it cannot leak a key to one. That
holds even when the script puts the output of a model into a request. The price
is a settings entry before a policy can make its first call, and no API type
beyond the three.

The owner chose the three types over every provider that Rig ships. Each type
adds tests and documentation, and each Rig upgrade must test it again.

The design rejected an HTTP capability in the script, with a URL and headers of
its own. That would make the script own credentials and endpoints, which issue
2 assigns to Ravel. It also rejected the environment constructors of Rig, which
read `*_API_KEY` and `*_BASE_URL` variables. An ambient variable could then
redirect a credential. When an owner needs another service, add an API type,
never a URL in a script.
