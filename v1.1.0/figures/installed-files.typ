// Installed pair and other files — UML deployment view. The only editable
// source of this figure: the enclosures with what they hold, the dependency and
// the key. Nothing here has a coordinate: _deployment.typ measures the text and
// places every box, and the drawing is _draw.typ's.
#import "_deployment.typ": *
#import "_draw.typ": render

#let tree = node("host", [Host],
  dir("prefix", [PREFIX (installation directory)], flow: "row", gap: 70,
    artifact("front", "ravel", [bin/ravel \ Ravel CLI \ with linked runner]),
    dir("libexec", [libexec/ravel (directory)],
      artifact("worker", "ravel", [ravel-policy \ policy worker \ with Bun, SDK and static example embedded], limit: 210),
      artifact("sdk", "external", [sdk/index.ts \ sdk/index.d.ts]),
      artifact("examples", "external", [examples/static.ts \ examples/static.d.ts \ examples/sample.ts]),
      artifact("notices", "external", [notices/NOTICES.md \ notices/bun-LICENSE.md]),
    ),
  ),
)

#render(deployment([Installed pair and other files — UML deployment view], tree, (dep("front", "worker"),), (
  [Blue: executables. Grey: other files. Outlines group directories.],
  [A file group lists separate files relative to libexec/ravel.],
  [Dashed arrow: dependency on worker discovery and a matched pair, not a process timeline.],
  [The CLI uses its own canonical real path to find the worker and checks build identity.],
  [Changes to adjacent sources do not replace embedded modules.],
  [Declarations support type checks. The owner copies and edits sample.ts.],
  [Installed execution needs no host Bun, Node.js or source checkout.],
  [Release archives additionally carry README.md and LICENSE at their root.],
)))
