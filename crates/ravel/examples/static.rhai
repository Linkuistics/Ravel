// The starter route table: a kind maps to one command, exactly.
export const SCHEMA_VERSION = 3;
export const VERSION = "ravel/examples/static 3";

/// One harness and model at `effort`, with the prompt as its last argument.
fn agent(effort) {
    let model = "your-model";
    |request| #{
        program: "my-agent-wrapper",
        args: ["--model", model, "--effort", effort, request.prompt],
        provider: "your-provider",
        model: model,
        effort: effort,
    }
}

export const ROUTES = #{
    question: agent("low"),
    bugfix: agent("medium"),
    feature: agent("high"),
    migration: agent("xhigh"),
    architecture: agent("xhigh"),
};

fn select_route(routes, request) {
    let kind = ravel::json_encode(request.kind);
    if !routes.contains(request.kind) {
        return #{
            status: "refused",
            code: "incomplete_mapping",
            message: `the routes name no command for kind ${kind}`,
            remedy: "add a route for this kind in your copy of the policy",
        };
    }
    let command = routes[request.kind].call(request);
    command.status = "selected";
    command.reason = `routes[${kind}] gives ${command.program} with model ${command.model} at effort ${command.effort}`;
    command
}

fn select(request, context) {
    select_route(global::ROUTES, request)
}
