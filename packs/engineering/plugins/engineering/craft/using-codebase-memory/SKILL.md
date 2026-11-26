---
name: using-codebase-memory
description: Query a codebase knowledge graph from any shell via the codebase-memory-mcp CLI — search_graph, query_graph (Cypher), trace_path, get_code_snippet — and the silent failure modes that make its output lie. Use when finding symbols by pattern, tracing callers or callees, assessing the blast radius of a change, hunting dead code or high fan-in, composing several graph queries into one answer, or working in a harness that has no MCP.
harnesses: [any]
---

# Using codebase-memory from the shell

`codebase-memory-mcp` indexes a codebase into a knowledge graph and serves
fourteen tools two ways — over MCP, and over a CLI. The tool set is identical;
only the calling convention differs. The CLI reaches every harness that has a
shell, and lets one question become one script instead of a round-trip per call.

> Behaviour verified against `codebase-memory-mcp 0.8.1`. Re-check the stream and
> truncation rules against a newer build before trusting a script to them.

**Read [`references/reading-the-graph.md`](references/reading-the-graph.md) before
treating any output as an answer about the codebase.** The failure modes that
corrupt an *answer* are all silent — a truncated page, an ignored filter, an
unresolved name and a capped traversal all return well-formed JSON and exit `0`,
and each one reads exactly like a smaller true answer. (A call that fails outright
is loud: empty stdout, an error on stderr, exit `1` — guard for it as below.) That file carries the fourteen tools, the
search modes and which of them silently override the others, the two truncation
caps, the Cypher rules, and how to compose several calls into one script.

## Which surface to use

If the MCP graph tools are available to you, prefer them for single queries —
typed arguments, no shell quoting. Use the CLI when composing, batching, or when
those tools are not available.

In Claude Code, MCP tools are [deferred by default][toolsearch] — absent from the
base tool list until loaded, so an absent `mcp__codebase-memory-mcp__*` tool means
"not yet loaded", not "server not running". Load what you need first:

    ToolSearch("select:mcp__codebase-memory-mcp__search_graph,mcp__codebase-memory-mcp__trace_path")

Deferral is the default, not a guarantee: `ENABLE_TOOL_SEARCH=false`, an
unsupported endpoint, or a server marked `alwaysLoad` all put the tools in the
base list instead, where they are simply callable.

[toolsearch]: https://code.claude.com/docs/en/mcp#scale-with-mcp-tool-search

## Invoking the CLI

    codebase-memory-mcp cli <tool> '<json>'

There is no per-tool help — `cli --help` is read as a tool name and fails.
Parameter names live in the MCP tool schemas; where those are reachable
(`ToolSearch` in Claude Code) read them there.

**Skip the `--json` flag.** It wraps the payload in an MCP envelope
(`.content[0].text`, a JSON *string* needing a second parse) and returns exit `0`
even on failure — the failure signal moves inside the envelope, to
`"isError": true`, where a shell `if` will not see it. Plain `cli` gives clean
stdout and an honest exit status.

**Pass `project` on every call.** The CLI does not infer it from the working
directory; running inside an indexed repo without it still fails. Names are
path-derived — list them with `codebase-memory-mcp cli list_projects`. The two
exceptions: `list_projects` takes no arguments, and `index_repository` takes
`repo_path`.

**Build the argument with `jq -n` whenever it interpolates anything.** A JSON
argument that does not parse is silently replaced by an empty argument set, and
you are then told about the *missing* `project` — the same message an unknown
project produces. So "project not found" reads as *either* an unindexed project
*or* a quoting mistake.

**Guard every call.** On failure stdout is empty, the error goes to stderr, and
exit is `1` — so a bare `| jq` sees nothing and reports `jq`'s own exit `0`. In a
loop that turns a failed call into one fewer row of a plausible-looking answer.
Two lines fix it for a whole script:

```bash
set -o pipefail

cm() {  # cm <tool> <json>  — JSON on stdout, or a loud failure and exit 1
  local out err; err=$(mktemp)
  if ! out=$(codebase-memory-mcp cli "$@" 2>"$err"); then
    printf 'codebase-memory-mcp %s failed:\n' "$1" >&2; cat "$err" >&2
    rm -f "$err"; return 1
  fi
  rm -f "$err"; printf '%s' "$out"
}
```

Read that stderr rather than discarding it: for an unknown project it carries
`hint` and the full `available_projects` list. Reach for a blanket `2>/dev/null`
only after a call is known to work.

## When the graph has no answer

An unindexed project is not an empty graph. If `list_projects` does not name the
repo you are in, either index it —

```bash
codebase-memory-mcp cli index_repository "$(jq -nc --arg r "$PWD" '{repo_path:$r, mode:"fast"}')" \
  | jq -c '{project, status, nodes, edges, excluded: .excluded.dirs}'
```

— or fall back to Grep, and say which you did. `fast` skips similarity and
semantic edges and filters the file set; where it dropped directories it names
them under `excluded`, so check them before concluding a symbol is absent. The key
is **absent entirely** when nothing was dropped, so `.excluded.dirs` yielding
`null` means "nothing excluded", not "field missing". `full` and `moderate` index
more and cost more.

Reporting "no results found" from an index that was never built is the failure
mode this skill exists to prevent.
