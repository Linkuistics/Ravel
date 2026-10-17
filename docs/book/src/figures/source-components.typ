// Source responsibilities — C4 component view. The only editable source of this
// figure: entities, the boundary, relations and the key. Positions are placed by
// hand; sizes come from _c4.typ, which returns the layout, and the drawing from
// _draw.typ.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("front", "ravel", [Request checks and final outcome], [«component»], cx: 130, top: 0, w: 260,
    tech: [Rust: crates/ravel/src],
    desc: [Checks owner policy and its response. Decides the final outcome.]),
  ent("worker", "ravel", [policy worker], [«container»], cx: 740, cy-of: "front", w: 300,
    tech: [Separate process: ravel-policy],
    desc: [crates/ravel/worker/src \ Bun compiles this worker with worker/sdk and worker/examples/static.ts embedded. The worker evaluates trusted owner policy.]),
  ent("runner", "ravel", [runner], [«component»], cx: 130, top: 330, w: 260,
    tech: [Rust: crates/keyed-launch],
    desc: [Linked library code. Launches and supervises the managed job inside the Ravel CLI.]),
  ent("managed", "external", [managed process], [«external_system»], cx: 740, cy-of: "runner", w: 300,
    desc: [Program that owner policy selects]),
)

#let bounds = (boundary("native", [Ravel CLI — one Rust process (bin/ravel)], ("front", "runner"), tag: [Container]),)

#let rels = (
  rel("front", "worker", [Evaluates owner policy], tech: [private request and response protocol], w: 300),
  rel("front", "runner", [Uses after worker cleanup], tech: [Rust library interface], w: 110),
  rel("runner", "managed", [Launches and supervises], tech: [selected program and arguments], w: 300),
)

#render(c4([Source responsibilities — C4 component view], ents, bounds, rels, (
  [Blue: Ravel code or process. Grey: selected program.],
  [Dashed outline: components inside the Ravel CLI, not a sandbox.],
  [The runner has no separate process or installed executable.],
  [Arrows name use or supervision, not a complete chronology.],
  [Before managed launch: the worker ends. Ravel reaps it and removes its private directory.],
  [The worker never receives the session-result context.],
  [Two Rust packages: crates/ravel and crates/keyed-launch.],
  [Worker paths in its box are relative to crates/ravel.],
)))
