# Reading the graph without being lied to

The calling mechanics — which surface, the `project` parameter, `jq -n` argument
building, and the `cm` guard wrapper — are in `using-codebase-memory`'s
`SKILL.md`. This file is what stands between a well-formed response and a wrong
answer. Examples assume `P` holds the project name and `cm` is defined.

Behaviour verified against `codebase-memory-mcp 0.8.1`.

## The fourteen tools

| Tool | For |
|---|---|
| `list_projects` | what is indexed (no arguments) |
| `index_repository` | index a repo — `repo_path`, optional `mode` |
| `index_status` | `{project, nodes, edges, status}` |
| `detect_changes` | files changed since the index, and the symbols they touch |
| `delete_project` | drop an index |
| `search_graph` | find symbols — the workhorse |
| `search_code` | text search over the corpus (results under `total_results`) |
| `query_graph` | Cypher, with aggregation and `ORDER BY` |
| `trace_path` | callers, callees, data flow, cross-service |
| `get_code_snippet` | a symbol's `source` plus its metrics, by `qualified_name` |
| `get_graph_schema` | node labels, their properties, and edge types with counts |
| `get_architecture` | layers, clusters, entry points, routes, hotspots |
| `manage_adr` | decision records held in the graph |
| `ingest_traces` | accepts runtime traces — see below before relying on it |

`ingest_traces` does **not** currently change the graph. It accepts a payload and
says so: `{"status":"accepted", "traces_received":0, "note": "Runtime edge
creation from traces not yet implemented"}`. Read the `note`; `status: "accepted"`
alone reads like success.

`index_repository`'s `mode` is `full` (the default), `moderate`, `fast`, or
`cross-repo-intelligence` — the last also **requires** `target_projects` (use
`["*"]` for all). An unrecognised mode is not rejected: it silently routes to
`full`, the slowest one, so a typo costs time rather than raising an error.

## `query` is exclusive; everything else composes

`search_graph` selects rows three ways — `query` (BM25 full-text, splits
camelCase — best for natural-language discovery), `name_pattern` / `qn_pattern`
(regex over the name or qualified name), and `semantic_query` (an **array** of
keywords, vector search) — narrowed by `label`, `file_pattern` and `min_degree` /
`max_degree`.

**`query` overrides every other one of those, silently.** Send it alongside any of
them and the rest are ignored — no error, no warning, just unfiltered results you
will read as filtered:

```bash
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, query:"wait socket", limit:5}')" \
  | jq -r '.total'                                                          # 257
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, query:"wait socket", label:"NoSuchLabel", limit:5}')" \
  | jq -r '.total'                                                          # 257 — label ignored
```

An impossible `name_pattern`, a nonexistent `label`, a `file_pattern` matching
nothing, `min_degree:10000` — every one returns the same 257 rows. Treat `query`
as a whole search, not as one clause of one: use it alone, or drop it and express
the constraint in the modes that *do* compose. Everything except `query`
intersects as you would expect.

## Results are truncated by default

`search_graph` returns at most `limit` rows, **default 200**, and says so only in
two fields you have to look at: `total` is the full match count, `has_more` is
true when there are more.

```bash
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, label:"Function", min_degree:10}')" \
  | jq -c '{total, rows: (.results|length), has_more}'
# {"total":2460,"rows":200,"has_more":true}
```

Sorting or aggregating those 200 rows client-side answers a question about an
arbitrary 8% of the matches, not about the codebase. Page with `offset` until
`has_more` is false, or push the whole question into `query_graph`.

`trace_path` truncates too, and worse: its response carries no `total` and no
`has_more`, and the cap is **hard at 100** — `limit` and `max_results` are both
ignored. A symbol whose true caller count is 123 returns 100, and nothing in the
payload says so. Read 100 as "100 or more, and this tool cannot tell you which";
when the exact set matters, count it in Cypher.

## Degree filters are not directional

`min_degree` gates on **total** degree, `in_degree + out_degree`. Adding
`relationship` and `direction` does not make it directional — those narrow the
candidate set by whether a node participates in that edge type at all, which at
`min_degree:10` on a 23k-node graph removed 2 rows out of 2460.

```bash
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, label:"Function", min_degree:10, limit:5}')" \
  | jq -r '.results[] | "in=\(.in_degree) out=\(.out_degree) \(.name)"'
# in=0 out=11 above_pane_sets_autoscroll_up   <- in_degree 0, past a min_degree of 10
```

Exactly half the rows that filter returned on the fixture had `in_degree: 0`. So
"find high fan-in functions" is not a `search_graph` call; it is a Cypher query.

`in_degree` also counts **every** edge type, not just `CALLS` — `USAGE`,
`DEFINES`, `DECORATES` and a dozen more all contribute (`get_graph_schema` lists
them with counts). A symbol with `in_degree: 40` whose inbound edges are all
`USAGE` has no callers at all, and `trace_path` will correctly say so.

## A bare name may not resolve

`trace_path` takes `function_name`, and a name shared by several symbols resolves
to none of them — you get an empty `callers` array and exit `0`, which reads
exactly like "nothing calls this". Check `total` from `search_graph` before
trusting a bare name, and pass the `qualified_name` instead; `trace_path` accepts
one, and on the fixture the same query then returned 67 callers instead of 0.

Two more `trace_path` defaults shape the answer. `include_tests` is **false**, so
callers in test files are dropped — and on a test-heavy symbol that is most of
them. One fixture symbol reports **1** caller by default and **100** with
`include_tests: true`; nothing but the flag distinguishes "barely used" from "used
everywhere in the suite". The other: the response key follows the direction —
`callers` for `inbound`, `callees` for `outbound`.

## Push aggregation into Cypher

`query_graph` runs Cypher server-side, which is where any question involving
counting, sorting or a global top-N belongs — over the whole graph rather than a
truncated page:

```bash
cm query_graph "$(jq -nc --arg p "$P" '{project:$p, query:
  "MATCH (f:Function)<-[:CALLS]-() RETURN f.qualified_name AS qn, count(*) AS fan_in ORDER BY fan_in DESC LIMIT 5"}')" \
  | jq -r '.rows[] | @tsv'
```

Group by `qualified_name`, not `name` — the same ambiguity that defeats
`trace_path` silently inflates a `count(*)` grouped on the bare name, summing
eight unrelated `wait_for_socket`s into one row. The response is columnar:
`{columns, rows, total}`, where `rows` is an array of arrays in `columns` order
and `total` is the row count.

### Filter with `WHERE`, never an inline property map

Pinning a node with a property map in the pattern — `(f:Function
{qualified_name: '…'})` — **silently truncates any traversal off it to 10 rows**:

```bash
cy() { cm query_graph "$(jq -nc --arg p "$P" --arg q "$1" '{project:$p, query:$q}')"; }

cy "MATCH (f:Function {qualified_name: '$QN'})<-[:CALLS]-() RETURN count(*) AS n" | jq -c '.rows'
# [["10"]]    <- capped, and nothing in the response says so
cy "MATCH (f:Function)<-[:CALLS]-() WHERE f.qualified_name = '$QN' RETURN count(*) AS n" | jq -c '.rows'
# [["123"]]   <- the true count
```

Every high-fan-in symbol on the fixture returns exactly `10` in the first form.
There is no `has_more`, and `total` counts the rows *returned*, so the response
looks complete. Worse, the two forms **agree** whenever the true result is under
10 — so this passes every small test case and is wrong on every large one. The cap
is on the traversal, not the match, but that narrowness is not worth remembering:
always write `WHERE`.

## Compose across tools, not within one

Cypher covers everything one query can express. Bash earns its place for the shape
Cypher cannot reach: **fanning a second tool over the first tool's results**, with
each intermediate staying in the shell instead of transiting the conversation. A
loop is also where the silent-failure trap does the most damage — a failed call
contributes nothing to stdout, so the loop iterates fewer times and the output
looks like a smaller true answer. Use the `cm` wrapper from `SKILL.md`, never a
blanket `2>/dev/null`.

Disambiguate, then trace — the two-step that turns a name into real callers:

```bash
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, label:"Function", name_pattern:"^wait_for_socket$"}')" \
  | jq -r '.results[] | select(.in_degree > 0) | .qualified_name' \
  | while read -r qn; do
      cm trace_path \
        "$(jq -nc --arg p "$P" --arg f "$qn" '{project:$p, function_name:$f, direction:"inbound", depth:1}')" \
      | jq -c --arg qn "$qn" '{symbol:$qn, callers:(.callers|length)}'
    done
```

Rank in Cypher, then pull each hit's source — one script instead of a round-trip
per symbol:

```bash
cm query_graph "$(jq -nc --arg p "$P" '{project:$p, query:
  "MATCH (f:Function)<-[:CALLS]-() RETURN f.qualified_name AS qn, count(*) AS fan_in ORDER BY fan_in DESC LIMIT 5"}')" \
  | jq -r '.rows[][0]' \
  | while read -r qn; do
      cm get_code_snippet "$(jq -nc --arg p "$P" --arg q "$qn" '{project:$p, qualified_name:$q}')" \
      | jq -c '{name, file_path, lines, complexity}'
    done
```
