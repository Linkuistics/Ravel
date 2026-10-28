// An LLM-assisted selection. The configuration script asks Claude Code which kind of work the
// prompt describes, and then routes that kind through the starter table. When
// the call gives no usable answer, the caller's own kind decides.
//
// It needs a harness entry and a budget in settings.json:
//
//     { "harnesses": { "claude": { "program": "claude" } },
//       "budgets": { "harnessRuns": 1 } }
//
// The harness signs in with your own Claude Code sign-in, and Ravel holds no
// credential. The default budget is 0 runs, so without the setting the call
// ends the evaluation. Copy `static.rhai` beside this file, because the import
// below names it by its path.
//
// The model only picks a key of the table, and the schema check admits no
// other text. Its answer never reaches `program` or `args`.

import "static" as starter;

export const SCHEMA_VERSION = 3;
export const VERSION = "llm-triage-1";

/// The kinds that the table routes, in name order.
fn kinds() {
    let names = starter::ROUTES.keys();
    names.sort();
    names
}

/// The shape of the answer: one property, and one of the table's kinds.
fn answer_schema() {
    #{
        type: "object",
        properties: #{ kind: #{ type: "string", enum: kinds() } },
        required: ["kind"],
        additionalProperties: false,
    }
}

/// The kind that Claude Code names for the prompt, or `()` when the call gives
/// no usable answer. A status other than `exited`, an exit code other than 0,
/// text that is no JSON, a document with no `structured_output` and an answer
/// that breaks the schema all give `()`. The reason goes to the diagnostics.
/// `json_parse` and `json_check` throw a map with a `code`, and a `try` catches
/// it.
fn ask(request) {
    let schema = answer_schema();
    let run = ravel::run_harness(#{
        harness: "claude",
        // -p prints the answer and exits. --tools "" gives the call no tool.
        // --json-schema asks for structured output, which the JSON document
        // holds in `structured_output`.
        args: ["-p", "--model", "haiku", "--tools", "", "--no-session-persistence",
               "--output-format", "json", "--json-schema", ravel::json_encode(schema)],
        stdin: "Which kind of work does this request describe? Answer with the JSON object only.\n\n" + request.prompt,
        timeoutMs: 20000,
    });
    if run.status != "exited" || run.exitCode != 0 {
        debug(`claude gave no answer: status ${run.status}, exit code ${run.exitCode}`);
        return ();
    }
    let answer = ();
    try {
        let document = ravel::json_parse(run.stdout);
        if type_of(document) != "map" || !document.contains("structured_output") {
            debug("claude gave no structured_output");
            return ();
        }
        answer = ravel::json_check(document.structured_output, schema);
    } catch (error) {
        debug(`claude gave an unusable answer: ${error.code}`);
        return ();
    }
    answer.kind
}

fn select(request, context) {
    let asked = ask(request);
    let routed = request;
    let how = "the caller's kind";
    if asked != () {
        routed.kind = asked;
        how = "claude's answer";
    }
    let selected = starter::select_route(starter::ROUTES, routed);
    if selected.status == "selected" {
        selected.reason = `${how}: ${selected.reason}`;
    }
    selected
}
