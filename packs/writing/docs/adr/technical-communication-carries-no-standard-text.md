# technical-communication carries no text from the standard

The `technical-communication` skill is modelled on ASD-STE100 (Issue 9), and it
carries none of the standard's text. It holds no rule wording, no explanatory
text, no example and no dictionary from the standard. It claims no conformance.
Its name and its description never use the standard's full trademarked title.
The skill describes itself only as modelled on ASD-STE100.

This binds because publication is a one-way door. The skill ships from a public
Apache-2.0 repository. Grove's Homebrew keg carries it, Ravel delivers it,
and the keg's marketplace files list its hooks plugins. Grove published it
earlier from its own public repository, through its Claude Code marketplace
and its manual installer. The plugin snapshot in each Grove binary up to 22.8
still holds it.

The standard's copyright notice forbids reproduction or publication, in whole
or in part, without written authority from its owner. Only eight listed groups
of users are free of that limit. Text in a public commit cannot be taken back
from those routes or from the history.

## Consequences

- Vocabulary control is approximate. Without the dictionary, the skill states
  principles for choosing words, and the checker enforces only the rules that
  need no dictionary.
- Each rule is restated from its idea, and each example is invented. The spec
  [`technical-communication`](../specs/technical-communication.md#restatement-discipline)
  holds the procedure, and the sweep that checks the repository before a
  release.
- The checker comes from an MIT project, `danyuchn/asd-ste100-skill` at commit
  `3944283`, under its own notice. Its messages name this skill's rules. They
  do not speak in the standard's name or cite its numbering.

## Considered options

- **A private skill that carries the standard's material.** Rejected: every route that
  ships the plugin is public, and one skill cannot be held back from its
  plugin's routes. Reopen if the skill moves to a private repository with no
  public distribution.
- **Wait for written authority from the standard's owner.** Rejected: there is no timeline, and
  the human's use is an experiment that needs the skill now. Reopen if the owner gives
  written authority to redistribute named parts. Then only those parts change.
- **Install or fork `danyuchn/asd-ste100-skill`.** Rejected: it carries the
  trademark in its name, it presents two rule passages as quotations, and it
  covers only part of the standard. Its checker is vendored, and its text is not.
  Nothing reopens the fork.
