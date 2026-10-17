// Fixture figure: two boxes and an arrow, as a layout record drawn through the
// fixture module.
#import "_draw.typ": render

#render((
  w: 320,
  h: 40,
  elements: (
    (id: "caller", shape: "box", role: "ravel", x0: 0, y0: 0, x1: 120, y1: 40, text: "Caller starts"),
    (id: "ravel", shape: "box", role: "ravel", x0: 200, y0: 0, x1: 320, y1: 40, text: "Ravel runs"),
    (id: "start", shape: "line", role: "relation", pts: ((120, 20), (200, 20)), dashed: false, arrow: true),
  ),
))
