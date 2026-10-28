// An configuration script to copy and edit. It adds one choice to the starter table:
// a `.ravel-choice` file in the caller's directory picks the `default` route
// or the `careful` one. Copy `static.rhai` beside this file, because the
// import below names it by its path.

import "static" as starter;

export const SCHEMA_VERSION = 3;
export const VERSION = "owner-choice-1";

const CHOICE_FILE = ".ravel-choice";
const OFFERED = ["default", "careful"];

/// The names in the choice file of `directory`, separated by whitespace, or `()`
/// when there is no such file. A file that cannot be read, and a name that the
/// configuration script does not offer, give a refusal map.
fn read_choice(directory) {
    let file = ravel::path_resolve(directory, global::CHOICE_FILE);
    let text = "";
    try {
        // A read is a measured source, so Ravel lists it in the notice.
        text = ravel::read_text(file, 4096).text;
    } catch (error) {
        if error.code == "source_missing" { return (); }
        let why = if error.code == "source_not_regular" {
            "is not a regular file"
        } else {
            `cannot be read: ${error.message}`
        };
        return choice_refusal(file, "choice_unreadable", why);
    }
    let names = text.split().filter(|name| name != "");
    for name in names {
        if !global::OFFERED.contains(name) {
            return choice_refusal(
                file,
                "choice_unoffered",
                `names ${ravel::json_encode(name)}, which this policy does not offer; it offers ${offered_text()}`
            );
        }
    }
    names
}

/// The names that the configuration script offers, as one text.
fn offered_text() {
    let text = "";
    for name in global::OFFERED {
        if text != "" { text += ", "; }
        text += name;
    }
    text
}

fn choice_refusal(file, code, message) {
    #{
        status: "refused",
        code: code,
        message: `the choice file ${file} ${message}`,
        remedy: `write names from ${offered_text()} in ${file}, separated by whitespace, or delete it`,
    }
}

fn select(request, context) {
    let choice = read_choice(request.cwd);
    if type_of(choice) == "map" { return choice; }
    let names = if choice == () { ["default"] } else { choice };
    if names.len() != 1 {
        return #{
            status: "refused",
            code: "one_choice",
            message: "choose one arrangement",
            remedy: "write default or careful in .ravel-choice",
        };
    }
    let selected = starter::select_route(starter::ROUTES, request);
    if selected.status == "refused" || names[0] != "careful" { return selected; }
    selected.effort = "high";
    selected.args = ["--model", selected.model, "--effort", "high", request.prompt];
    selected.reason = "careful choice";
    selected
}
