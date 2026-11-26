# Skill preparation precedes evaluation

The Ravel CLI resolves the skill sources that the owner configures, and it
prepares the prompt before the Rhai worker launches. The prepared prompt
carries directory locations and mandatory file reads. The agent finds and
chooses the skills. The request holds the exact original prompt beside the
prepared prompt. The [skill discovery specification](../specs/skill-discovery.md)
owns the fields, the loading rules and the acceptance boundary.

This changes the promise that `request.prompt` is the caller's prompt,
unchanged. The feature therefore ships in a major release, with script and
request schema 4. A schema-3 script refuses, also with delivery disabled.
Otherwise a later settings edit would change what an unchanged script receives.
The separate original field keeps task-only consumers working. It costs a
migration and duplicate bytes in the request.

Preparation in the Ravel CLI gives the shared behaviour one owner, and it keeps
the command construction of the configuration script intact. Reopen the
source-selection boundary only for a concrete caller that needs a separately
authorised source set, controlled by the configuration script.

## Considered options

- **A helper that each configuration script calls.** The design rejected it,
  because automatic delivery would depend on every script remembering the
  helper. Nothing reopens this option while delivery must be automatic.
- **Text appended after selection.** The design rejected it, because Ravel
  would have to interpret the command that the script selected.
- **Schema 3 kept while delivery is disabled.** The design rejected it. The
  request shape would then depend on the settings, and a settings edit would
  change script semantics silently. Schema 3 was never published, so the
  refusal costs no released script. Reopen if a published schema-3 script must
  keep working.
- **A generated catalogue of skill names.** The design rejected it, because it
  couples source delivery to pack metadata, namespaces and duplicate names.
  Directory delivery leaves those content judgements with the agent and pack
  conformance. It costs enumeration by the agent and its handling of
  ambiguity. Reopen a metadata index only when measured discovery failures
  require it and the owner changes this boundary. Source resolution alone is
  not that evidence.
- **Mandatory skill names instead of file references.** The design rejected
  them, because they restore the metadata and ambiguity rules that directory
  delivery leaves with the agent. File references make the settings depend on
  the pack layout, so a moved file needs a settings repair. Reopen only for a
  requirement of stable names that do not depend on the layout.
- **Inline paths without a descriptor.** The design rejected them, because a
  reader must recover the same source view after it loses the prompt. The
  retained descriptor adds a public schema and a cache object. It does not keep
  the instruction to recover it after compaction. Reopen if readers can
  recover the same directory set reliably without it.
- **A separate settings file for skills.** The design rejected it, because it
  adds a second settings document and a second check boundary to every
  preparation. One file costs mixed-major use. Ravel 1.1 refuses the `skills`
  key, also with an explicit `--config`. Separate executable prefixes therefore
  do not permit delivery in a shared `HOME`. Use of both majors isolates `HOME`,
  or keeps `skills` out of shared settings until no 1.1 reader remains. Reopen
  if delivery in one `HOME` with both majors becomes a requirement.
