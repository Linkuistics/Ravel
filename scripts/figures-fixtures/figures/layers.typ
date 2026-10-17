// Fixture figure: two boxes and an arrow, drawn through the fixture module.
#import "_draw.typ": *
#show: setup

#canvas({
  framed(0, 0, 120, 40, [Caller starts])
  framed(200, 0, 120, 40, [Ravel runs])
  arrow(120, 20, 200, 20)
})
