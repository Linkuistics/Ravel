# Only symbols decide acceptance

A step carries attributes as KDL properties, such as a file reference with the type annotation `(url-reference)` (https://kdl.dev/spec/). Both views show the attributes, but the tracer never reads them. Only the symbols of the steps decide whether a grammar accepts a sequence. Attribute declarations can come later without a break, because they would only add checks.

## Trade-off

The tracer cannot reject a step for a missing or wrong attribute, such as an exercise step without its file. In return, the notation needs no attribute language, and the tracer stays one Earley parser over symbols. Process-mining tools also keep attributes off the model. Inductive Visual Miner shows them in a panel beside the graph (https://leemans.ch/publications/ivm%20ProM%206.10.pdf).

## Rejected alternative

Attribute declarations for each symbol, and data conditions on gateways as in BPMN. Both would make acceptance depend on data that the grammar view does not show.

Source: the human's decision of 2026-11-17 and the agreement of 2026-11-18 in `docs/milestones/grammar-workbench.md`, and the process-mining survey.
