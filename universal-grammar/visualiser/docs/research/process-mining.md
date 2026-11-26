# Process mining and conformance checking for the grammar workbench

This report answers Topic 7 of the research brief. The survey date is 17 November 2026.

Status: complete for this pass. Section 9 lists the searches that found nothing. Cells marked "not checked" in Section 8 still need a source.

The survey assumes these decisions from after the brief:

- The grammar is EBNF with bounded repeats, such as `x{2,4}`, and every terminal is a whole symbol.
- KDL serialises the grammar, the traces and every derived artifact.
- A trace returns parse trees, and each loop gets one node with one child for each pass.
- The grammar view shows the position, the next options and the current pass of each loop.
- The output view shows a partial parse tree and folds the completed passes.
- Several tracer markers mean alternative readings of one sequence, and the grammar has no parallel operator.
- The tracer is one Earley parser over the position automaton of each rule.

The topic maps the terms of process mining onto the workbench as follows:

- A case is an episode.
- An activity is a move label.
- A trace is the move sequence of one episode.
- A process model is a grammar.

Under this mapping, conformance checking asks whether an episode fits the grammar and where it departs. Process discovery asks which grammar a set of episodes suggests. The fallback to beat is a set of overlays on the grammar diagrams. The overlays are markers for the current state, one highlighted path for one trace, counts for many traces and an alignment for a rejected trace.

## 1. Findings that change decisions

- Q3: A process tree matches an EBNF rule without self-embedding. Choice, sequence and the silent leaf map directly, but the loop is unbounded and has a redo part ([Leemans 2013](https://dfahland.win.tue.nl/publications/LeemansFA_2013_blockstructured.pdf)). No process-mining notation found has a bounded repeat. Recursion appears only in one 2018 extension ([SANER 2018](https://arxiv.org/pdf/1710.09323)).
- Q3: The standard alignment cost is 1 for a log move or a model move and 0 for a synchronous move. No replacement move exists ([p1299](https://www.vdaalst.com/publications/p1299.pdf)). The automata survey counts a replacement as one edit. The tracer needs one written cost model, and alignment costs keep pm4py usable as an oracle.
- Q3: Optimal alignments are often not unique. The IvM manual says that the implementations "deterministically choose one option" by internal ordering ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). Two optimal alignments of one trace give precision 0.8 and 0.67 ([Syring](https://arxiv.org/pdf/1909.02393)). This supports several markers with a cap.
- Q4: The Petri-net token game is the precedent for the next options. WoPeD replays a trace through clicks on enabled transitions ([WoPeD tutorial](http://conformancechecking.win.tue.nl/wp-content/uploads/WoPeD.pdf)). The process-mining tools animate many cases and do not step one case.
- Q4: IvM draws deviations on the unchanged model. A log move is a red dashed self-loop, and a model move is a red dashed edge that bypasses the activity ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). This is the fallback to beat for a rejected trace.
- Q4: Directly-follows graphs show false loops under concurrency, invisible gaps after frequency filters and conditional average times ([DFG paper](https://www.vdaalst.com/publications/p1101.pdf)). Two tools gave "two completely different DFGs" for one log. Counts belong on the grammar diagram, and each count must state what it counts.
- Q4: Tools shorten long loops in their variant views. The Celonis variant count cuts each run of one activity to two occurrences by default ([Celonis FAQ](https://help.celonis.com/pql47/en/faq---process-query-language)). A ProM plug-in replaces a repeated subsequence with one abstract activity ([Bose 2011](https://ceur-ws.org/Vol-734/PaperDemo05.pdf)). This supports the fold of completed passes.
- Q2: The case views of Disco and IvM put event attributes in a table or a side panel, not on the graph ([Disco](https://fluxicon.com/book/read/casesview/)). OCEL 2.0 links an event to objects with a qualifier, such as "creates" ([OCEL 2.0](https://arxiv.org/pdf/2403.01975)). An attached file can be an object that steps reference.
- Q3: Multi-perspective conformance costs data and moves in one search. Its authors show that a search that aligns the moves first can mislead ([Mannhardt 2016](https://pure.tue.nl/ws/files/4020941/377779004839577.pdf)). If attribute guards enter the grammar, the tracer must cost them in the same search.
- Q5: pm4py is AGPL-3.0-or-later, and its vendor says that network use requires publication of the source ([PIS](https://processintelligence.solutions/pm4py)). The Rust crate `process_mining` is MIT OR Apache-2.0 and reads XES and OCEL ([crates.io](https://crates.io/crates/process_mining)). pm4py fits only as an external test oracle (inference, not legal advice).

## 2. Q1: Purpose

Process mining has two halves that match the workbench. Conformance checking compares one model with recorded traces. Discovery builds a model from recorded traces. The tracer is a conformance checker for one trace, and a candidate-grammar feature would be a discoverer.

The two fields differ in their unit of analysis. Process mining diagnoses a real process from a log of many cases. Its views aggregate cases, and its animations replay many cases against a clock ([Disco paper](https://ceur-ws.org/Vol-940/paper8.pdf)). The workbench explains a grammar through single traces that a user steps. No examined process-mining tool steps one trace move by move against a model (inference from the sources in Section 10).

The early literature saw both uses. Cook and Wolf, who introduced the term "process discovery", found their methods "useful in visualizing the data collected on a process" ([Cook and Wolf](http://www.doc.ic.ac.uk/~alw/doc/papers/tosem0798.pdf)). Their industrial study compared 159 recorded executions with a formal model. It counted matched, missed and extra events, which later became synchronous moves, model moves and log moves (inference).

### 2.1 Discovery as a precedent for candidate grammars

Cook and Wolf built finite-state machines from the event data of software processes ([Cook and Wolf](http://www.doc.ic.ac.uk/~alw/doc/papers/tosem0798.pdf)). They compared a neural network, the k-tail method of Biermann and Feldman and a new Markov method. They concluded "that the Ktail and Markov methods show the most promise". Their k-tail version needed extra steps to delete "overly complex constructs" from its models.

The industrial study shows what discovery leaves to people. The documented process model "did not capture even half of the behavior" in the executions ([Cook and Wolf](http://www.doc.ic.ac.uk/~alw/doc/papers/tosem0798.pdf)). The Markov method gave model fragments, and a process engineer merged, deleted and modified states by hand. The authors conclude that "the discovery techniques alone will not automatically create a completed process model".

Later benchmarks found that discovery methods disagree. Augusto et al. compared seven methods on 24 real logs with nine measures ([Augusto 2019](https://arxiv.org/abs/1705.02288)). Fodina found "only eight sound models", although its models usually fitted the logs well. Inductive Miner scored a fitness above 0.90 on 20 logs, but it "did not stand out for its precision". Split Miner stood out for its F-score.

Buijs, van Dongen and van der Aalst found that most algorithms consider at most two of four quality dimensions ([Buijs 2012](https://doi.org/10.1007/978-3-642-33606-5_19)). The dimensions are replay fitness, precision, generalisation and simplicity. They conclude that the last three matter only "if the replay fitness is acceptable". The flower model shows why, because it accepts every sequence and so explains nothing ([Leemans 2013](https://dfahland.win.tue.nl/publications/LeemansFA_2013_blockstructured.pdf)).

The fallback to beat is a hand-written grammar with no discovery. A departure needs users who hold many episodes and no grammar. Then Inductive Miner is the obvious candidate, because its process trees are sound and fit the log by construction. Its candidates still need a precision check and edits by hand before a user accepts them (inference from the benchmark and from Cook and Wolf).

## 3. Q2: Notation for trace files

XES is the IEEE standard for event logs. IEEE 1849-2023 superseded the 2016 edition and is active ([IEEE SA](https://standards.ieee.org/ieee/1849/10907/)). A log holds traces, and a trace is a sequence of events. Standard extensions define keys such as `concept:name`, `lifecycle:transition`, `org:resource` and `time:timestamp` ([XES](https://www.xes-standard.org/)).

OCEL 2.0 drops the single case notion. Each event relates to one or more objects, and each relation carries a qualifier, such as "creates" ([OCEL 2.0](https://arxiv.org/pdf/2403.01975)). Objects can relate to other objects, and object attribute values carry timestamps. A 2024 critique notes that object-to-object relations have no time ([critique](https://arxiv.org/pdf/2405.12709)). It also notes that an attribute change does not name the event that caused it.

A successor is in progress. The IEEE Task Force on Process Mining develops an Object-Centric Event Data (OCED) standard for IEEE certification ([TFPM 2025](https://tf-pm.org/newsletter/newsletter-stream-17-08-2025/recent-updates-and-next-steps-for-the-oced-standard)). Its 2024 white paper states that it does not formulate a standard ([white paper](https://arxiv.org/pdf/2410.14495)). The latest update found, from December 2025, reports continued development ([TFPM 18](https://tf-pm.org/newsletter/newsletter-stream-18-12-2025)).

The fallback to beat is a KDL trace with one node for each step. The move label is the node argument, and attributes are properties. This mirrors an XES trace (inference). A departure to the OCEL form pays when several steps reference one file, or when a file changes between steps. Then the file becomes a node of its own, and each step references it with a qualifier.

Both standards pass the walk-away check. XES is XML with a public schema, and OCEL 2.0 offers JSON, XML and SQLite forms. The Rust crate `process_mining` reads both formats ([Rust4PM](https://ceur-ws.org/Vol-3758/paper-16.pdf)).

## 4. Q3: Semantics

### 4.1 Process trees and EBNF rules

A process tree is "a rooted tree in which leaves are labeled with activities and all other nodes are labeled with operators" ([Leemans 2013](https://dfahland.win.tue.nl/publications/LeemansFA_2013_blockstructured.pdf)). The operators are exclusive choice, sequence, loop and parallel. The silent leaf τ has the empty language. The loop ⟲(M1, M2, ..., Mn) runs the body M1, then any number of times one redo part and M1 again.

The mapping to the workbench grammar is direct for three constructs:

- Exclusive choice is EBNF alternation.
- Sequence is concatenation.
- The silent leaf is the empty string.

The loop differs. A process-tree loop is unbounded and has a redo part, so ⟲(x, s) means `x (s x)*`. No process-tree operator expresses a bound. A bounded repeat becomes nested choices over copies of its item, such as `x x (x x?)?` for `x{2,4}` (inference). The parallel operator has no counterpart, because the grammar has none.

Each operator "has a straightforward formal translation to a sound, block-structured workflow Petri net" ([Leemans 2013](https://dfahland.win.tue.nl/publications/LeemansFA_2013_blockstructured.pdf)). The BPMN-style flows of the workbench keep the same block structure (inference). IvM separates model and drawing in the same way. It keeps process trees inside but shows "a directly-follows based representation" with BPMN parallel gateways ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)).

### 4.2 Recursion

Maikel Leemans, van der Aalst and van den Brand add a named subtree operator and a recursion leaf to process trees ([SANER 2018](https://arxiv.org/pdf/1710.09323)). The leaf "denotes the point where we recurse on the named subtree". Their discovery uses the call nesting in software logs. They map each operator "to a block-structured concept in the target formalism", such as Statecharts, Petri nets and BPMN. The Statechart Workbench in ProM draws the result with frequency annotations. The paper does not show how a recursive call looks in that view.

This is the only process-mining precedent found for call boxes. Searches for conformance checking against recursive or context-free models found nothing in process mining (Section 9). The workbench therefore cannot borrow a checker for rules with self-embedding (inference).

### 4.3 Token replay and alignments

Token replay plays a trace on a Petri net and counts produced, consumed, missing and remaining tokens ([Rozinat 2008](https://www.vdaalst.com/publications/p436.pdf)). Silent and duplicate transitions force heuristics. Berti and van der Aalst name a "token flooding" problem, where repeated missing tokens hide later deviations ([Berti 2019](https://ceur-ws.org/Vol-2371/ATAED2019-87-103.pdf)).

An alignment pairs the trace with one complete run of the model. Each pair is a synchronous move, a log move or a model move ([Adriansyah 2014](https://research.tue.nl/en/publications/aligning-observed-and-modeled-behavior/)). IvM calls an alignment "a best guess of what run of M could have produced t" ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)). The standard cost function charges log moves and model moves equally and synchronous moves nothing ([p1299](https://www.vdaalst.com/publications/p1299.pdf)).

For a grammar, an optimal alignment is a fewest-edits parse (inference). A log move deletes a symbol from the trace, and a model move inserts one. The automata survey's Aho and Peterson repair counts a replacement as one edit. Under alignment costs a replacement costs 2, so the two cost models rank some repairs differently (inference).

The fallback to beat is the automata survey's fewest-edits Earley parse, with alignment costs as its documented unit. A departure to other costs needs evidence that users misread the repairs.

### 4.4 Several optimal alignments

The literature agrees that more than one optimal alignment can exist ([p702](https://www.vdaalst.com/publications/p702.pdf)). The search that finds one optimal alignment cannot list all of them. The IvM manual gives the smallest example ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). For the model sequence(a, b) and the trace ⟨a, c⟩, one alignment puts the log move c before the model move b. The other puts it after b, and the manual calls this order "arbitrary".

Sources disagree about how much this matters. Adriansyah et al. report that "1-alignment and representative alignment provide good approximation of all-alignments" ([ISeB 2015](https://link.springer.com/article/10.1007/s10257-014-0234-7)). Syring, Tax and van der Aalst show two optimal alignments of one trace with precision 0.8 and 0.67 ([Syring](https://arxiv.org/pdf/1909.02393)). Tax et al. report that the one-alignment method chooses arbitrarily ([Tax 2017](https://arxiv.org/pdf/1705.03303)).

The fewest-deviations rule is itself an assumption. The IvM manual states that "there is no guarantee that an optimal alignment reflects reality" ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). Alizadeh, de Leoni and Zannone call manual cost functions "error-prone" and derive costs from history ([CIDM 2015](https://www.math.unipd.it/~deleoni/documenti/cidm15.pdf)). Koorneef et al. report that the techniques "do not always return the most probable alignment" ([Koorneef 2017](https://link.springer.com/chapter/10.1007/978-3-319-74030-0_15)).

The decision to show several markers matches this evidence. The process-mining tools hide the ties. The workbench can list them with a cap, as the automata survey proposes.

### 4.5 Data-aware conformance

Mannhardt, de Leoni, Reijers and van der Aalst align control flow, data, resources and time in one search over data Petri nets ([Mannhardt 2016](https://pure.tue.nl/ws/files/4020941/377779004839577.pdf)). They argue that a search that aligns the moves first makes the other perspectives "second-class citizens". In their words, "a data attribute may provide strong evidence that the wrong activity was executed". Their evaluation used more than 140,000 road-fine cases.

The grammar has no guards now, and step attributes are references, such as an attached file. If a later grammar lets an attribute value choose a branch, this paper is the precedent. It argues for one search that costs move deviations and data deviations together.

## 5. Q4: Layout and interaction

### 5.1 One trace while stepping

The examined process-mining tools animate many cases, not one stepped trace. Disco, Apromore and bupaR draw each case as a token that moves over the map ([Apromore](https://documentation.apromore.org/conformancechecking/animatelogsprocessmap.html), [bupaR](https://bupaverse.github.io/docs/animate_maps.html)). IvM notes that a paused animation "gives a frozen view of the map" at one point in time ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)). Disco presents its animation as a replay of real events, which "should not be confused with simulation" ([Disco paper](https://ceur-ws.org/Vol-940/paper8.pdf)).

Petri-net editors offer the closer precedent. The WoPeD token game shows the current marking and lets the user fire one enabled transition at a time ([WoPeD tutorial](http://conformancechecking.win.tue.nl/wp-content/uploads/WoPeD.pdf)). The tutorial uses it to replay one trace, and an autoplay mode runs on to the final marking. CPN Tools marks enabled transitions with green auras ([CPN Tools](https://cs.au.dk/fileadmin/site_files/cs/research_areas/centers_and_projects/sttt2007.pdf)). The petrinet-io library draws enabled transitions in green and executed ones in purple ([petrinet-io](https://ceur-ws.org/Vol-4274/paper-33.pdf)).

The fallback to beat is a token game on the rule diagram. The marker sits on the current position, and the enabled next symbols carry a highlight. Tokens in a net mean concurrent threads, but several tracer markers mean several readings. The legend must say so (inference).

### 5.2 One trace after it ends

Disco opens one case in a case view with summary statistics and the history of the case ([Disco](https://fluxicon.com/book/read/casesview/)). The history appears as a graph or as a table. In the graph view, "additional data attributes are hidden", and the table view shows every imported attribute.

IvM has a trace view ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). Each trace is a row, with its name on the left and one coloured wedge for each event. The timestamp sits above the wedge, the activity inside it and the lifecycle transition below it. A second line below the wedge gives the alignment, such as "only in model" for a model move. A separate panel summarises trace and event attributes and correlates trace attributes with fitness.

The ProM conformance plug-in colours the model instead ([ProM tutorial](https://conformancechecking.win.tue.nl/wp-content/uploads/TutorialChapter12.pdf)). Darker blue marks more frequent activities. A green and purple bar on each transition shows the share of executions with and without a deviation. Yellow places mark events that the model could not explain.

The output view can merge these forms. The parse tree carries the structure, and each step node can carry its attributes. The IvM move labels are a precedent for marks on inserted and deleted steps (inference). The Celonis case view was not checked.

### 5.3 Many traces: counts, filters and variants

Frequency and performance overlays are standard. bupaR draws frequency or performance values on nodes and edges, with secondary layers ([bupaR maps](https://bupaverse.github.io/docs/advanced_maps.html)). Its trace explorer lists variants by frequency, with a default coverage of 0.2 ([processmapR](https://rdrr.io/cran/processmapR/man/trace_explorer.html)). Disco sorts variants by frequency, and a variant is one distinct activity sequence ([Disco](https://fluxicon.com/book/read/casesview/)).

IvM computes its counts from alignments on the model, not from directly-follows pairs ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)). A click on a deviation edge keeps only the traces with that deviation, and the trace view follows ([ProM forum](https://promforum.win.tue.nl/discussion/591/problem-using-the-inductive-visual-miner-with-deviations)). The IvM authors criticise tools that replace the model with a new one when the user filters.

The fallback to beat is counts on the grammar diagram, taken from the parses of many traces. A filter changes the counts but does not redraw the diagram.

### 5.4 A rejected trace

IvM adds the deviations to the drawing, not to the model ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)). A log move is a dashed self-edge, and a model move is a dashed edge that bypasses the activity. The authors write that "if all dashed/red edges would be transformed to normal edges, the model would have perfect fitness".

Celonis labels each case instead of drawing on the model. Its labels include "Conforms", "X is an undesired activity" and "A is followed by A" ([Celonis](https://help.celonis.de/pql46/en/conformance)). Users report that the checker struggles with parallel gateways and flags repeats unless the model has self-loops ([Celonis community](https://community.celonis.com/process-analysis-5/conformance-checker-analysis-problems-with-parallel-arcs-4869)).

The fallback to beat is the IvM form on the grammar view, with one alignment at a time. The output view can show the repair as tree nodes marked "inserted" and leaves marked "deleted" (inference).

### 5.5 What users misread

Van der Aalst wrote that his paper "could have been called 'Directly-Follows Graphs (DFGs) considered harmful'" ([DFG paper](https://www.vdaalst.com/publications/p1101.pdf)). He names three faults:

- Flexible ordering leads "to Spaghetti-like DFGs with loops even when activities are executed at most once".
- Frequency thresholds create "invisible gaps" in the model.
- The average time between two activities counts only the cases where one directly follows the other.

In his simulated log, Disco and Celonis report 1.5 days from 2,530 of 10,000 cases, and the true time is 3.5 days. Filtered activities disappear from the map but "still influence the statistics".

IvM reports a Disco map where an activity "occurred 13 times and has only 4 outgoing edges" ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)). When a filter hides the parallel edges, a parallel block "looks exactly like the exclusive choice". The IvM manual warns that its own overlays "might convey more information than is actually available" ([IvM manual](https://leemans.ch/publications/ivm%20ProM%206.10.pdf)). The reason is that the drawn alignment is one of several.

Timestamps also cause false deviations. In the ProM tutorial, 403 of 2,246 cases appear to skip one activity ([ProM tutorial](https://conformancechecking.win.tue.nl/wp-content/uploads/TutorialChapter12.pdf)). With equal timestamps read as a partial order, three cases remain.

The evidence on users is thin and divided. In an interview study, 5 of 37 experts advised care because DFGs "are easily misinterpreted" ([interviews](https://www.researchgate.net/publication/363343247_Process_Mining_Practices_Evidence_from_Interviews)). The same study reports analysts who call DFGs "very straightforward". A drift-detection paper calls DFGs "simple and comprehensive" ([drift paper](https://arxiv.org/pdf/2011.09130)). Both quotes come from search excerpts, not from a reading of the papers. No controlled study of misreading was found (Section 9).

A grammar diagram avoids the first two DFG faults if it never redraws from the data (inference). Counts on edges still invite a conditional reading, so a count must say what it counts. A repair overlay needs a note when other repairs tie with it.

### 5.6 Loops that repeat many times in one case

Each repeat count makes a new variant, so loops multiply variants. Celonis handles this in its query language. SHORTENED reduces "more than max_cycle_length consecutive occurrences of the same activity" to that number, with a default of 2 ([Celonis VARIANT](https://help.celonis.de/pql47/en/variant)). The built-in variant count applies it, so it can differ from a plain count ([Celonis FAQ](https://help.celonis.com/pql47/en/faq---process-query-language)). These operators handle runs of one activity only.

Bose, Verbeek and van der Aalst detect tandem arrays, which are repeats of a whole subsequence ([Bose 2011](https://ceur-ws.org/Vol-734/PaperDemo05.pdf)). The detection runs "in linear time and space with respect to the length of the traces". Their plug-in replaces each repeat with an abstract activity and keeps the replaced part in a sub-log. The model of the sub-log appears when the user zooms into the abstract activity.

The IvM trace view gives one wedge to each event, so a long loop gives a long row (inference from the manual). IvM also reduces log moves to frequencies "for readability reasons" ([IvM 2014](https://leemans.ch/publications/papers/bpi2014leemans.pdf)).

The grammar knows its loops, so the workbench does not need to detect them. The decided loop node with one child for each pass is the structural form of the Bose abstraction (inference). The fallback to beat is the decided fold of completed passes, with a pass count on the fold.

## 6. Q5: Scope

The vertical slice needs four process-mining ideas:

- A marker on the current position, with highlighted next symbols
- A highlighted path for one trace
- One alignment for a rejected trace, drawn in the IvM style
- All tied alignments as markers, with a cap.

Counts over many traces, performance overlays, discovery, object-centric logs and data-aware conformance can wait. Each one needs a log of many episodes, which the slice does not have.

pm4py 2.7.23.12 shipped on 5 November 2026 under AGPL-3.0-or-later ([PyPI](https://pypi.org/project/pm4py/)). Process Intelligence Solutions GmbH maintains it and sells a commercial licence for closed-source use. The vendor states that software that offers pm4py over a network must publish its source under the AGPL ([PIS](https://processintelligence.solutions/pm4py)). A downstream project proposes to make pm4py an optional extra for this reason ([issue](https://github.com/AleFire99/Petri_nets/issues/15)). URG uses Python, so pm4py fits as a test tool outside the shipped workbench (inference, not legal advice).

The Rust crate `process_mining` from the Rust4PM project is the candidate inside the core. Version 0.6.2 shipped on 21 August 2026, after 62 versions since December 2023 ([crates.io](https://crates.io/crates/process_mining)). It reads and writes XES, OCEL and Petri nets, and offers token replay and directly-follows discovery ([docs.rs](https://docs.rs/process_mining/)). Its modules moved between versions, so a pinned version is safer (inference from the documentation).

## 7. Q6: Test seams

Process mining offers three seams:

- An oracle for repair costs. A rule without self-embedding becomes a process tree once its bounded repeats are expanded (inference). pm4py converts a process tree to a Petri net and returns one alignment for each trace ([pm4py docs](https://pm4py.fit.fraunhofer.de/static/assets/api/2.7.6/generated/pm4py.vis.view_alignments.html)). Only the costs can match, because pm4py returns one optimal alignment of several.
- A test for ties. The IvM example of sequence(a, b) and ⟨a, c⟩ must give two repairs.
- Load tests. Public XES logs, such as the BPI Challenge 2012 log in the ProM tutorial, give real traces. The Rust crate reads them without Python.

## 8. Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance status | Walk-away result |
|---|---|---|---|---|---|---|
| pm4py | https://github.com/process-intelligence-solutions/pm4py | Python library for discovery, alignments and token replay | AGPL-3.0-or-later, commercial licence from the vendor | 2.7.23.12, 5 November 2026 | Active, maintained by Process Intelligence Solutions GmbH | Models and logs stay in open formats such as PNML, XES and OCEL (inference) |
| process_mining (Rust4PM) | https://github.com/aarkue/rust4pm | Rust library for XES, OCEL, Petri nets, discovery and token replay | MIT OR Apache-2.0 | 0.6.2, 21 August 2026 | Active, 62 versions since December 2023 | Reads and writes the open formats |
| ProM with IvM | https://promtools.org/development/prom-6-15-release/ | Research framework, IvM discovery and deviation view | Framework GPL-2.0-or-later, packages usually LGPL, IvM licence not found ([framework](https://promtools.org/prom-6-framework-license/), [packages](https://promtools.org/prom-6-package-license/)) | ProM 6.15 with IvM 6.15.11, date given without a year | ProM moved to GitHub in 6.14, IvM repository has 4 stars | Exports traces and per-trace fitness, the drawing needs ProM (inference) |
| Disco | https://fluxicon.com/blog/2025/09/disco-4-2/ | Commercial process maps, animation and case view | Proprietary | 4.2.6, after 4.2 on 9 September 2025 | Releases in 2025, none found for 2026 | Exports logs as XES, MXML or CSV, and maps as PDF, PNG, XML or CSV ([export](https://fluxicon.com/book/read/export/)) |
| Celonis | https://help.celonis.de/pql47/en/variant | Commercial platform with PQL and a conformance checker | Proprietary | Software as a service | Active | Analyses live in the platform (inference) |
| Apromore Core | https://github.com/apromore/ApromoreCore | Open edition with log replay on BPMN | LGPL-3.0 | 7.20.1, year not shown | Archived on 29 August 2025, source "now deprecated" | Models stay as BPMN XML (inference) |
| bupaR family | https://bupaverse.github.io/docs/animate_maps.html | R process maps, trace explorer and token animation | MIT | bupaR 1.0.1 (27 January 2026), processmapR 0.5.8 (20 March 2026), processanimateR 1.0.5 (20 July 2022) | bupaR active, processanimateR without a release since 2022 | Event logs stay as R data frames (inference) |
| WoPeD | https://github.com/woped/WoPeD | Petri-net editor with a token game | LGPL, version 3.0 on [Wikidata](https://www.wikidata.org/wiki/Q5859521) and 2.0 on [SourceForge](https://sourceforge.net/projects/woped/) | 3.11.1, 14 May 2026, per Wikidata | Developed at DHBW Karlsruhe since 2003, releases in 2025 and 2026 ([WoPeD](https://woped.dhbw-karlsruhe.de/)) | Not checked |
| XES | https://www.xes-standard.org/ | Event-log standard, IEEE 1849-2023 | IEEE standard | 1849-2023, published 8 September 2023 | Active standard | XML with public schemas |
| OCEL 2.0 | https://www.ocel-standard.org/ | Object-centric event-log specification | Open specification | 2.0, on arXiv in March 2024 | OCED successor in development at the IEEE task force | JSON, XML and SQLite forms |

## 9. Silences

These searches found nothing useful:

| Question | Query | Result |
|---|---|---|
| Conformance checking against recursive models | `conformance checking alignment context-free grammar OR pushdown process model recursion "alignments" procedure calls` | Only formal-methods work on recursive programs |
| A controlled study of how users misread process maps | `user study process mining visualization comprehension "directly-follows" OR "process map" misinterpretation participants empirical` | Only an interview study and conceptual warnings |
| The pm4py release that moved to AGPL, and a link to Fraunhofer FIT | `pm4py licence changed GPL-3.0 to AGPL-3.0 version 2.3 OR 2.7 Process Intelligence Solutions spin-off Fraunhofer` | No release and no link |
| A loop-folding control in the current Celonis variant explorer | `Celonis case explorer activity table rework loops "Variant Explorer" repeated activities collapse loops` | Only the PQL operators of releases 4.6 and 4.7 |
| The licence of the IvM repository | `github promworkbench InductiveVisualMiner repository license` | Licences differ across the promworkbench repositories |
| The changes in IEEE 1849-2023 against the 2016 edition | `IEEE 1849-2023 XES standard eXtensible Event Stream revision attributes extensions lifecycle concept` | Only the status and dates |
| An OCED update from 2026 | `IEEE Task Force on Process Mining OCED object-centric event data standard 2025 OR 2026 status OCEL 2.0 successor` | Nothing after December 2025 |
| A Disco release in 2026 | `Fluxicon Disco release notes 2025 OR 2026 version "Disco 4"` | Only releases 4.1 and 4.2 from 2025 |
| An open successor to Apromore Core after the archive | `Apromore Community Edition open source licence github ApromoreCore latest release 2025 2026` | Only the archived repositories |

## 10. Sources

- https://dfahland.win.tue.nl/publications/LeemansFA_2013_blockstructured.pdf
- https://leemans.ch/publications/papers/bpi2014leemans.pdf
- https://leemans.ch/publications/ivm%20ProM%206.10.pdf
- https://promforum.win.tue.nl/discussion/591/problem-using-the-inductive-visual-miner-with-deviations
- https://arxiv.org/pdf/1710.09323
- https://www.vdaalst.com/publications/p1101.pdf
- https://www.vdaalst.com/publications/p436.pdf
- https://ceur-ws.org/Vol-2371/ATAED2019-87-103.pdf
- https://research.tue.nl/en/publications/aligning-observed-and-modeled-behavior/
- https://www.vdaalst.com/publications/p702.pdf
- https://www.vdaalst.com/publications/p1299.pdf
- https://link.springer.com/article/10.1007/s10257-014-0234-7
- https://arxiv.org/pdf/1705.03303
- https://arxiv.org/pdf/1909.02393
- https://www.math.unipd.it/~deleoni/documenti/cidm15.pdf
- https://link.springer.com/chapter/10.1007/978-3-319-74030-0_15
- https://pure.tue.nl/ws/files/4020941/377779004839577.pdf
- https://conformancechecking.win.tue.nl/wp-content/uploads/TutorialChapter12.pdf
- http://conformancechecking.win.tue.nl/wp-content/uploads/WoPeD.pdf
- https://cs.au.dk/fileadmin/site_files/cs/research_areas/centers_and_projects/sttt2007.pdf
- https://ceur-ws.org/Vol-4274/paper-33.pdf
- https://ceur-ws.org/Vol-734/PaperDemo05.pdf
- https://fluxicon.com/book/read/casesview/
- https://ceur-ws.org/Vol-940/paper8.pdf
- https://fluxicon.com/blog/2025/09/disco-4-2/
- https://help.celonis.de/pql46/en/conformance
- https://help.celonis.de/pql47/en/variant
- https://help.celonis.com/pql47/en/faq---process-query-language
- https://community.celonis.com/process-analysis-5/conformance-checker-analysis-problems-with-parallel-arcs-4869
- https://documentation.apromore.org/conformancechecking/animatelogsprocessmap.html
- https://github.com/apromore/ApromoreCore
- https://bupaverse.github.io/docs/animate_maps.html
- https://bupaverse.github.io/docs/advanced_maps.html
- https://rdrr.io/cran/processmapR/man/trace_explorer.html
- https://www.researchgate.net/publication/363343247_Process_Mining_Practices_Evidence_from_Interviews
- https://arxiv.org/pdf/2011.09130
- https://standards.ieee.org/ieee/1849/10907/
- https://www.xes-standard.org/
- https://arxiv.org/pdf/2403.01975
- https://arxiv.org/pdf/2405.12709
- https://www.ocel-standard.org/
- https://tf-pm.org/newsletter/newsletter-stream-17-08-2025/recent-updates-and-next-steps-for-the-oced-standard
- https://arxiv.org/pdf/2410.14495
- https://tf-pm.org/newsletter/newsletter-stream-18-12-2025
- http://www.doc.ic.ac.uk/~alw/doc/papers/tosem0798.pdf
- https://arxiv.org/abs/1705.02288
- https://doi.org/10.1007/978-3-642-33606-5_19
- https://pypi.org/project/pm4py/
- https://github.com/process-intelligence-solutions/pm4py
- https://processintelligence.solutions/pm4py
- https://github.com/AleFire99/Petri_nets/issues/15
- https://pm4py.fit.fraunhofer.de/static/assets/api/2.7.6/generated/pm4py.vis.view_alignments.html
- https://crates.io/crates/process_mining
- https://docs.rs/process_mining/
- https://ceur-ws.org/Vol-3758/paper-16.pdf
- https://github.com/aarkue/rust4pm
- https://promtools.org/development/prom-6-15-release/
- https://promtools.org/prom-6-framework-license/
- https://promtools.org/prom-6-package-license/
- https://fluxicon.com/book/read/export/
- https://github.com/woped/WoPeD
- https://www.wikidata.org/wiki/Q5859521
- https://sourceforge.net/projects/woped/
- https://woped.dhbw-karlsruhe.de/
- https://cran.r-project.org/web/packages/processmapR/index.html
- https://cran.r-project.org/web/packages/processanimateR/index.html
- https://cran.rstudio.com/web/packages/bupaR/bupaR.pdf

## 11. Decision records

This decision record cites this survey:

- `docs/adr/symbols-decide-acceptance.md`: steps carry attributes, and only symbols decide acceptance.
