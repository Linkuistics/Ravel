# The configuration script uses Rhai

Ravel 2.0.0 accepts only a configuration script in Rhai. The default entry is
`config.rhai`, a module that defines `select` and optionally `load_context`.
Rhai is a scripting language that a Rust program embeds. It has no file,
network, process or environment access of its own. Every effect therefore goes
through a host capability that Ravel controls, and Ravel ships no JavaScript
runtime.

Ravel 2.0.0 is a clean cutover, and no release runs both formats. A lone
`policy.ts` refuses with a remedy that names the Rhai configuration script chapter. When both
files exist, `config.rhai` wins. The owner chose the cutover over a
compatibility period in 1.x. All known TypeScript configuration scripts are the owner's, and
coexistence keeps the costs of the Bun worker. The
[configuration evaluation specification](../specs/configuration-evaluation.md) states the
entry rules and the historical migration map.

The trade-off is a small embedded language against a typed one with packages.
TypeScript under Bun gave owners a type checker, npm packages and ordinary I/O.
It also made every release carry Bun, its notices, its startup and resolver
controls and its checks on each target. Most configuration scripts are small route tables,
and they need none of that.

Rhai costs the type checker and the package ecosystem. A mistake in a configuration script
shows when the configuration script runs, with a line and a column. Ravel moves undefined
variables and absent map properties to errors, which recovers part of the
checks that types gave.

Keeping TypeScript was the rejected alternative, with or without a
compatibility period. GitHub issue 2 asks for the change, and the owner
chose the cutover in the requirements session. Reconsider only if owners
need a type checker or packages more than a small sandboxed runtime.
