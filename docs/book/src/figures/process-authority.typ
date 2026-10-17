// Processes and authority — C4 container view. The only editable source of this
// figure: entities, the boundary, relations and the key. Positions are placed by
// hand; sizes come from _c4.typ, which returns the layout, and the drawing from
// _draw.typ.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("caller", "external", [caller], [«external_system»], cx: 520, top: 8, w: 300,
    desc: [An orchestrator or person: requests work and interprets the opaque result]),
  ent("cli", "ravel", [Ravel CLI], [«container»], cx: 520, top: 250, w: 270,
    tech: [Rust process: ravel start-session],
    desc: [Checks the selection and decides the final outcome. The runner executes inside this process and owns launch and supervision.]),
  ent("worker", "ravel", [policy worker], [«container»], cx: 90, cy-of: "cli", w: 230,
    tech: [Separate process; compiled Bun],
    desc: [Evaluates trusted owner policy. No session-result context.]),
  ent("managed", "external", [managed process], [«external_system»], cx: 970, cy-of: "cli", w: 230,
    desc: [Selected direct child: a harness running an LLM session, or another program]),
  ent("channel", "temporary", [result channel], [«temporary» «container»], cx: 520, top: 500, w: 270,
    tech: [Temporary directory \ and completion file],
    desc: [Carries one opaque result; no persistent history], db: true),
)

#let bounds = (boundary("ravel", [Ravel software], ("worker", "cli", "channel"), tag: [System]),)

#let rels = (
  rel("caller", "cli", [Requests a session], tech: [kind and prompt], out: ("bottom", -70), into: ("top", -70), side: "left", t: 0.3),
  rel("cli", "caller", [Writes opaque result only after successful cleanup], tech: [standard output], role: "result",
    out: ("top", 70), into: ("bottom", 70), w: 170, t: 0.7),
  rel("worker", "cli", [Policy response], tech: [selection or refusal; separate socket],
    out: ("right", -26), into: ("left", -26), w: 150),
  rel("cli", "worker", [1\. Evaluates request], tech: [bounded request; restricted environment],
    out: ("left", 30), into: ("right", 30), side: "below", w: 150),
  rel("cli", "managed", [2\. Launches], tech: [selected command; session-result context], w: 150, dx: 14),
  rel("managed", "channel", [Publishes result via end-session], role: "result", out: "bottom", into: "right", seg: 0, w: 120, dy: -20),
  rel("channel", "cli", [Supplies result], tech: [validate and read], role: "result"),
)

#render(c4([Processes and authority — C4 container view], ents, bounds, rels, (
  [Blue: Ravel processes. Grey: external participants. Gold: temporary channel.],
  [Green arrows carry the opaque result.],
  [The LLM asks its harness to run end-session.],
  [The managed process's children inherit its session-result context.],
  [Publication asks Ravel to stop; return waits for successful cleanup.],
  [1 precedes 2: the worker and managed process are not concurrent phases.],
  [Before 2: worker ended and reaped; its private directory removed.],
  [A valid selection alone is insufficient for launch.],
  [The runner is library code inside the Ravel CLI, not another process.],
  [The enclosing outline is software membership, not a sandbox.],
  [Boundaries do not authenticate same-user code or contain detached children.],
)))
