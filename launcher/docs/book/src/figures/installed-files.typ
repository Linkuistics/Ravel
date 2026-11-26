// Installed files — UML deployment view. The only editable
// source of this figure: the enclosures with what they hold and
// the key. Nothing here has a coordinate: _deployment.typ measures the text and
// places every box, and the drawing is _draw.typ's.
#import "_deployment.typ": *
#import "_draw.typ": render

#let tree = node("host", [Host],
  dir("prefix", [PREFIX (installation directory)], flow: "row", gap: 70,
    artifact("front", "ravel", [bin/ravel \ Ravel CLI \ with linked runner \ and Rhai engine], limit: 210),
    dir("share", [share/ravel],
      artifact("record", "external", [installation.json \ Homebrew only], limit: 210),
      dir("examples-dir", [examples],
        artifact("examples", "external", [static.rhai \ sample.rhai \ llm.rhai], limit: 210),
      ),
      dir("notices-dir", [notices],
        artifact("notices", "external", [THIRD-PARTY-NOTICES.md], limit: 210),
      ),
    ),
  ),
  dir("skill-keg", [Skill source keg],
      artifact("payload", "external", [libexec/skill-sources/ID/ \ Source payload], limit: 210),
      artifact("alias", "external", [share/ravel/skills/ID \ Alias to payload], limit: 210),
    ),
  artifact("shared", "external", [Shared prefix: share/ravel/skills/ID \ Link to keg alias], limit: 210),
)

#render(deployment([Installed files — UML deployment view], tree, (), (
  [Blue: the executable. Grey: other files. Outlines group directories.],
  [The file group lists separate files relative to share/ravel/examples.],
  [The CLI starts the same bin/ravel as a separate Rhai worker and checks its identity.],
  [A skill source is installed by its own formula, not by Ravel.],
  [The shared prefix link resolves through the keg alias to the source payload.],
  [The worker is a process role, not another installed executable.],
  [The owner copies and edits the examples. sample.rhai and llm.rhai import static.rhai.],
  [Installed execution needs no separate language runtime or source checkout.],
  [THIRD-PARTY-NOTICES.md lists notices for Rust dependencies.],
  [Release archives additionally carry README.md and LICENSE at their root.],
)))
