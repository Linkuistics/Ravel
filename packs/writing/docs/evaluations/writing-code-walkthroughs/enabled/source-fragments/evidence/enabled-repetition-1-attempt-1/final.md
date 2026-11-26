I can’t produce the exact authoring plan you asked for from this run, because the environment prevented any shell inspection at all, including read-only commands against `targets/ocaml/check_floor.ml`.

I attempted read-only shell access and got the same pre-execution failure each time: `sandbox-exec: sandbox_apply: Operation not permitted`. That means I could not read the authoritative source snapshot, so I cannot truthfully return:
- the exact source inventory and exclusions
- exact fragment names and line ranges
- a worked execution grounded in the production code
- the exact pre-publication mechanical checks for that file

I did not inspect any other project file.

To continue, I need one of these:
1. The contents of `targets/ocaml/check_floor.ml` pasted here.
2. A rerun in an environment where read-only shell commands like `sed -n`, `nl -ba`, or `cat` are actually permitted.

With the file contents available, I can return the full plan in the exact structure you requested.
