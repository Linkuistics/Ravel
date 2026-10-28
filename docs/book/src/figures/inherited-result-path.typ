// Process tree for the nested example, using C4-style objects and relations.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("caller", "external", [caller], [«process»], cx: 350, top: 0, w: 270, desc: [RAVEL_EXIT_FILE: none]),
  ent("outer", "ravel", [outer start-session], [«process»], cx: 350, top: 180, w: 270, desc: [RAVEL_EXIT_FILE: none]),
  ent("worker", "ravel", [policy worker], [«process»], cx: 30, top: 400, w: 240, desc: [RAVEL_EXIT_FILE: none]),
  ent("harness", "external", [outer harness], [«process»], cx: 350, top: 400, w: 270, desc: [RAVEL_EXIT_FILE: outer path]),
  ent("shell", "external", [child shell], [«process»], cx: 350, top: 620, w: 270, desc: [RAVEL_EXIT_FILE: outer path]),
  ent("end", "external", [end-session], [«process»], cx: 100, top: 850, w: 240, desc: [RAVEL_EXIT_FILE: outer path. Writes result there]),
  ent("inner", "ravel", [inner start-session], [«process»], cx: 600, top: 850, w: 260, desc: [RAVEL_EXIT_FILE: outer path]),
  ent("agent", "external", [inner harness], [«process»], cx: 600, top: 1080, w: 260, desc: [RAVEL_EXIT_FILE: inner path]),
  ent("innerend", "external", [inner end-session], [«process»], cx: 600, top: 1300, w: 260, desc: [RAVEL_EXIT_FILE: inner path. Writes result there]),
)
#let rels = (
  rel("caller", "outer", [starts]),
  rel("outer", "worker", [starts without path], out: "left", into: "top", w: 90),
  rel("outer", "harness", [starts with outer path]),
  rel("harness", "shell", [starts with inherited path]),
  rel("shell", "end", [starts with inherited path], out: "left", into: "top", w: 90),
  rel("shell", "inner", [starts with inherited path], out: "right", into: "top", w: 90),
  rel("inner", "agent", [starts with new inner path]),
  rel("agent", "innerend", [starts with inherited path]),
)
#render(c4([How end-session finds its session — process tree], ents, (), rels, (
  [Blue: session supervisors and worker. Grey: caller and managed descendants.],
  [Outer path: /private/tmp/ravel-session-mlcN86/signal-c7d5…],
  [Inner path: /private/tmp/ravel-session-QpztGn/signal-b643…],
  [Children inherit the path unless their parent changes the environment.],
  [An evaluation harness run gets its own path. Its result returns to the policy.],
  [The inner launch changes its child's environment, not the outer harness's.],
)))
