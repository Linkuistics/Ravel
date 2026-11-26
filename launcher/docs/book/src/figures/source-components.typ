// Source responsibilities — C4 component view. The only editable source of this
// figure: entities, the boundary, relations and the key. Positions are placed by
// hand; sizes come from _c4.typ, which returns the layout, and the drawing from
// _draw.typ.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("front", "ravel", [Request checks and final outcome], [«component»], cx: 130, top: 0, w: 260,
    tech: [Rust: crates/ravel/src \ Reader thread and host runtime],
    desc: [Prepares skills (crates/ravel/src/skills), then controls host calls. Checks the configuration script and its response. Decides the final outcome.]),
  ent("worker", "ravel", [Rhai worker], [«container»], cx: 740, cy-of: "front", w: 250,
    tech: [Separate process: same bin/ravel],
    desc: [crates/ravel/src/rhai_worker \ Evaluates configuration script and requests host capabilities.]),
  ent("runner", "ravel", [runner], [«component»], cx: 130, top: 390, w: 260,
    tech: [Rust: crates/keyed-launch],
    desc: [Linked library code. Supervises harness runs and the final managed job inside the Ravel CLI.]),
  ent("harness", "external", [evaluation harness], [«external_system»], cx: 740, cy-of: "runner", w: 250,
    desc: [Program from settings. Returns data to the configuration script. Own result channel.]),
  ent("managed", "external", [final managed process], [«external_system»], cx: 740, top: 740, w: 250,
    desc: [Program that configuration script selects]),
)

#let bounds = (boundary("native", [Ravel CLI — one Rust process \ (bin/ravel)], ("front", "runner"), tag: [Container]),)

#let rels = (
  rel("front", "worker", [Evaluates configuration script], tech: [request, host calls and replies on private socket], w: 170, dx: 10),
  rel("front", "runner", [Uses for each launch], tech: [Rust library interface], w: 110),
  rel("runner", "harness", [Supervises harness run], tech: [during evaluation], w: 170, dx: 10),
  rel("runner", "managed", [Launches after evaluation], tech: [selected program and arguments], out: "bottom", into: "left", seg: 1, side: "above", w: 170),
)

#render(c4([Source responsibilities — C4 component view], ents, bounds, rels, (
  [Blue: Ravel code or process. Grey: external programs.],
  [Dashed outline: components inside the Ravel CLI, not a sandbox.],
  [C4 components here are internal code. Ravel supports no separate component.],
  [The runner has no separate process or installed executable.],
  [Arrows name use or supervision, not a complete chronology.],
  [Skill preparation precedes the worker. The reader thread serves the host runtime.] ,
  [Before managed launch: the worker ends. Ravel reaps it and removes its private directory.],
  [Every host call ends first. Each harness run completes its required cleanup.],
  [Harness-run data returns to the configuration script. It cannot complete the final session.],
  [The worker never receives the session-result context.],
  [Two Rust packages: crates/ravel and crates/keyed-launch.],
  [The CLI and Rhai worker are separate processes of the same installed executable.],
)))
