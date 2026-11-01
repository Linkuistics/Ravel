// A configuration script with the shape of a working owner's: a table from the caller's kind
// to a harness, and the grant of the main repository to a harness that runs in
// a secondary jj workspace. The names are invented. Two programs share the work
// by role: a lead writes and integrates, and a reviewer checks.
//
// WHERE IT RUNS. It reads no parameter and names no session. A secondary jj
// workspace keeps its store in the main repository, and its `.jj/repo` is a
// file that names that store; there both harnesses are given the main
// repository with `--add-dir`. In a primary workspace, or anywhere else,
// nothing more is granted. Test the routing with temporary fake agents on a
// private PATH: start-session launches the selected program.

export const SCHEMA_VERSION = 4;
export const VERSION = "harness-table 3";

/// A command line, and the labels it is recorded under. `style` names the
/// layout of its arguments, which `harness_args` builds.
fn lead_agent(model, effort) {
    #{ provider: "provider-a", model: model, effort: effort, program: "lead-agent", style: "lead" }
}

fn review_agent(model, effort) {
    #{ provider: "provider-b", model: model, effort: effort, program: "review-agent", style: "review" }
}

/// The arguments of `harness` for `prompt`. `grants` are the directories beyond
/// the cwd that the harness may write.
fn harness_args(harness, prompt, grants) {
    let args = [];
    if harness.style == "review" {
        args += ["--model", harness.model, "-c", `reasoning_effort=${harness.effort}`,
                 "-c", "permissions=full-access", "--ask-for-approval", "never"];
        for grant in grants { args += ["--add-dir", grant]; }
    } else {
        for grant in grants { args += ["--add-dir", grant]; }
        args += ["--model", harness.model, "--effort", harness.effort];
    }
    args.push(prompt);
    args
}

/// The roles. A role is a model of one program at an effort that the table gives.
fn writer(effort) { lead_agent("model-writer", effort) }
fn lead(effort) { lead_agent("model-lead", effort) }
fn reviewer(effort) { review_agent("model-review", effort) }

/// The harness for `kind`, or `()` when the table has none.
fn select_harness(kind) {
    switch kind {
        "scope" => lead_agent("model-deep", "max"),
        "scope-check" => review_agent("model-deep", "max"),
        "scope-fold" => lead_agent("model-deep", "max"),

        "design" => lead_agent("model-deep", "xhigh"),
        "design-check" => review_agent("model-deep", "xhigh"),
        "design-fold" => lead_agent("model-deep", "xhigh"),

        "build" => lead("high"),
        "build-check" => reviewer("high"),
        "build-fold" => lead("high"),

        "survey-a" => lead_agent("model-deep", "xhigh"),
        "survey-b" => review_agent("model-deep", "xhigh"),

        "draft" => writer("high"),
        "polish" => lead_agent("model-deep", "high"),
        "release-notes" => lead("medium"),

        _ => (),
    }
}

fn refused(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

/// The main repository that a secondary jj workspace at `cwd` keeps its store
/// in, as a list of one grant, or an empty list. A secondary workspace's
/// `.jj/repo` is a file that holds the store's path, relative to `.jj/`, and
/// the store is `<main>/.jj/repo`. The grant is `<main>` rather than the store
/// alone because a colocated repository's git objects sit beside the store, in
/// `<main>/.git`. A primary workspace's `.jj/repo` is the store itself, a
/// directory inside the cwd, and a directory with no `.jj` is no workspace:
/// neither needs a grant. A file that cannot be read, or that is empty, gives
/// a refusal map.
fn main_repository(cwd) {
    let file = ravel::path_resolve(cwd, ".jj/repo");
    let store = "";
    try {
        store = ravel::read_text(file).text;
    } catch (error) {
        if error.code == "source_missing" || error.code == "source_not_regular" { return []; }
        return unreadable_store(file, error.message);
    }
    store.trim();
    if store == "" { return unreadable_store(file, "it is empty"); }
    let jj_dir = ravel::path_resolve(cwd, ".jj");
    [ravel::path_parent(ravel::path_parent(ravel::path_resolve(jj_dir, store)))]
}

fn unreadable_store(file, why) {
    refused(
        "jj_store_unreadable",
        `${file} names no jj store: ${why}`,
        `run from a jj workspace whose ${file} names its store, or from a directory with no .jj`
    )
}

fn select(request, context) {
    let harness = select_harness(request.kind);
    if harness == () {
        return refused(
            "incomplete_mapping",
            `no mapping for kind ${ravel::json_encode(request.kind)}`,
            "add a mapping for this kind in your configuration script"
        );
    }

    let grants = main_repository(request.cwd);
    if type_of(grants) == "map" { return grants; }

    #{
        status: "selected",
        program: harness.program,
        args: harness_args(harness, request.prompt, grants),
        provider: harness.provider,
        model: harness.model,
        effort: harness.effort,
        reason: `routes ${ravel::json_encode(request.kind)} to ${harness.program} with model ${harness.model} at effort ${harness.effort}`,
    }
}
