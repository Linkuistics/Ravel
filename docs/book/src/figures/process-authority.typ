// Processes and authority — C4 container view. The only editable source of this
// figure: entities, the boundary, relations and the key. Positions are placed by
// hand; sizes come from _c4.typ, which returns the layout, and the drawing from
// _draw.typ.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("caller", "external", [caller], [«external_system»], cx: 350, top: 8, w: 300,
    desc: [An orchestrator or person: requests work and interprets the opaque result]),
  ent("cli", "ravel", [Ravel CLI], [«container»], cx: 350, top: 280, w: 270,
    tech: [Rust process: ravel start-session],
    desc: [Checks the selection and decides the final outcome. The runner executes inside this process and owns launch and supervision.]),
  ent("worker", "ravel", [policy worker], [«container»], cx: 80, top: 600, w: 210,
    tech: [Separate process: compiled Bun],
    desc: [Evaluates trusted owner policy. No session-result context.]),
  ent("managed", "external", [managed process], [«external_system»], cx: 350, top: 950, w: 250,
    desc: [Selected direct child: a harness that runs an LLM session, or another program]),
  ent("channel", "temporary", [result channel], [«temporary» «container»], cx: 620, top: 600, w: 230,
    tech: [Temporary directory \ and completion file],
    desc: [Carries one opaque result. No persistent history.], db: true),
)

#let bounds = (boundary("ravel", [Ravel software], ("worker", "cli", "channel"), tag: [System]),)

#let rels = (
  rel("caller", "cli", [Requests a session], tech: [kind and prompt], out: ("bottom", -70), into: ("top", -70), side: "left", t: 0.3),
  rel("cli", "caller", [Writes opaque result only after successful cleanup], tech: [standard output], role: "result",
    out: ("top", 70), into: ("bottom", 70), w: 170, t: 0.7),
  rel("worker", "cli", [Policy response], tech: [selection or refusal on a separate socket],
    out: ("top", 30), into: ("left", 30), seg: 0, side: "right", w: 150),
  rel("cli", "worker", [1\. Evaluates request], tech: [bounded request and restricted environment],
    out: ("left", -30), into: ("top", -30), seg: 0, side: "above", w: 150),
  rel("cli", "managed", [2\. Launches], tech: [selected command and session-result context], out: "bottom", into: "top", w: 120, side: "left"),
  rel("managed", "channel", [Sends result with end-session], role: "result", out: "right", into: "bottom", seg: 1, w: 120, side: "left"),
  rel("channel", "cli", [Supplies result], tech: [validate and read], role: "result", out: "top", into: "right", seg: 0, w: 120, side: "left"),
)

#render(c4([Processes and authority — C4 container view], ents, bounds, rels, (
  [Blue: Ravel processes. Grey: external participants. Gold: temporary channel.],
  [Green arrows carry the opaque result.],
  [The LLM asks its harness to run end-session.],
  [The managed process's children inherit its session-result context.],
  [The sent result asks Ravel to stop. Ravel returns only after successful cleanup.],
  [1 precedes 2: the worker and managed process are not concurrent phases.],
  [Before 2: the worker ends. Ravel reaps it and removes its private directory.],
  [A valid selection alone is insufficient for launch.],
  [The runner is library code inside the Ravel CLI, not another process.],
  [The outer outline is software membership, not a sandbox.],
  [Boundaries do not authenticate same-user code or contain detached children.],
)))
