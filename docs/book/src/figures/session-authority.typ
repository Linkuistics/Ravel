// Process authority during the chapter 7 example — C4 container view.
#import "_c4.typ": *
#import "_draw.typ": render

#let ents = (
  ent("cli", "ravel", [Ravel CLI], [«container»], cx: 350, top: 0, w: 270,
    desc: [Coordinates both runs and their separate result channels.]),
  ent("worker", "ravel", [Rhai worker], [«container»], cx: 100, top: 330, w: 200,
    desc: [Evaluates the configuration. No session-result authority.]),
  ent("advisor", "external", [fake-advisor], [«container»], cx: 350, top: 330, w: 200,
    desc: [Harness run. Sends advice through its own result channel.]),
  ent("agent", "external", [fake-agent], [«container»], cx: 600, top: 330, w: 200,
    desc: [Managed process. Sends finished through the session result channel.]),
)
#let rels = (
  rel("cli", "worker", [Starts and reaps], w: 110),
  rel("cli", "advisor", [Supervises during selection], w: 130),
  rel("cli", "agent", [Launches after worker cleanup], w: 130),
)
#render(c4([Processes and authority in ravel-parts — C4 container view], ents, (), rels, (
  [Blue: Ravel processes. Grey: programs in the example.],
  [The CLI starts and supervises the harness run and the final managed process.],
  [The two result channels are separate and exist at different stages.],
  [The worker cannot send the final session result.],
)))
