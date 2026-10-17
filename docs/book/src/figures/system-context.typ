// Where Ravel sits — C4 system context. The only editable source of this figure:
// entities, relations and the key. Positions are placed by hand; sizes come from
// _c4.typ, which returns the layout, and the drawing from _draw.typ.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("owner", "owner", [owner], [«person»], cx: 640, top: 0, w: 280,
    desc: [Controls the account's owner policy and settings]),
  ent("ravel", "ravel", [Ravel], [«system»], cx: 640, top: 165, w: 300,
    desc: [Routes one request, supervises the selected program and decides the final outcome]),
  ent("caller", "external", [caller], [«external_system»], cx: 120, cy-of: "ravel", w: 230,
    desc: [An orchestrator or person: supplies the prompt and interprets the result]),
  ent("managed", "external", [managed process], [«external_system»], cx: 640, top: 405, w: 280,
    desc: [Harness that runs an LLM session, or another selected program]),
)

#let rels = (
  rel("owner", "ravel", [Configures command selection], tech: [owner policy and settings]),
  rel("caller", "ravel", [Requests a session], tech: [start-session: kind and prompt], out: ("right", -30), into: ("left", -30)),
  rel("ravel", "caller", [Returns opaque result after successful cleanup], tech: [standard output], role: "result",
    out: ("left", 30), into: ("right", 30), side: "below", w: 200),
  rel("ravel", "managed", [Launches and supervises], tech: [selected command], out: ("bottom", -70), into: ("top", -70), side: "left"),
  rel("managed", "ravel", [Sends opaque result and asks Ravel to stop], tech: [end-session], role: "result",
    out: ("top", 70), into: ("bottom", 70), w: 190),
)

#render(c4([Where Ravel sits — C4 system context], ents, (), rels, (
  [Blue: Ravel. Grey: caller and managed process. Slate: human owner.],
  [Green arrows carry the opaque result.],
  [The LLM asks its harness to run end-session when its task is done.],
  [Ravel stops the group members that remain and confirms cleanup before it returns.],
  [This view hides Ravel's processes. Boxes are not security boundaries.],
)))
