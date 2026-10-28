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
      dir("examples-dir", [examples],
        artifact("examples", "external", [static.rhai \ sample.rhai \ llm.rhai], limit: 210),
      ),
      dir("notices-dir", [notices],
        artifact("notices", "external", [THIRD-PARTY-NOTICES.md], limit: 210),
      ),
    ),
  ),
)

#render(deployment([Installed files — UML deployment view], tree, (), (
  [Blue: the executable. Grey: supporting files. Outlines group directories.],
  [The file group lists separate files relative to share/ravel/examples.],
  [The CLI starts the same bin/ravel as a separate Rhai worker and checks its identity.],
  [The worker is a process role, not another installed executable.],
  [The owner copies and edits the examples. sample.rhai and llm.rhai import static.rhai.],
  [Installed execution needs no separate language runtime or source checkout.],
  [THIRD-PARTY-NOTICES.md lists notices for Rust dependencies.],
  [Release archives additionally carry README.md and LICENSE at their root.],
)))
