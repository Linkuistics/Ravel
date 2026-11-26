# Layout and interaction for the grammar visualiser

This report answers Topic 4 of the research brief. The survey date is 17 November 2026.

Status: complete for this pass. Section 6 lists the searches that found nothing.

The coordinator recorded these decisions after the brief:

- A Rust core runs natively and as WebAssembly.
- The workbench is TypeScript with D3.
- The diagrams use a mixed presentation in the style of BPMN.
- A rule group that needs no stack draws as a state machine.
- A reference that needs a stack draws as a call box with drill-down.
- The same diagrams show the current state and the history of a trace.
- A trace returns parse trees, and only rules make tree nodes.

The parent session proposes one node for each symbol occurrence. Choices and repeats become gateway nodes. References become sub-process boxes that collapse and expand.

Later messages define two views of a run. The grammar view shows the current position and the next options. The output view shows a partial parse tree that grows at its right edge. Completed passes of a loop stay readable under their repeat, and the current pass is the focus. Both views can show the attributes of a step, such as a reference to an attached file.

## 1. Findings that change decisions

- Q4: For the BPMN-style flows, a layout derived from the expression structure is the fallback to beat. It computes each box from its children, as railroad diagrams do. An edit moves siblings by translation and never reorders them (inference from the code). A layered engine solves the whole graph again. A repeat draws as an arc around its item, so the self-loop faults on the ELK tracker do not arise.
- Q4: ELK Layered can use the order of the source text as a tie-breaker, which its authors call "model order". They report user complaints about lost control and about drawings that change after small edits ([ELK paper, section 4.2](https://arxiv.org/abs/2311.00533)).
- Q4: dagre 3.1.0 added a stable-layout mode on 2 August 2026, and 3.1.3 fixed its state on 24 October 2026. No evidence from years of use exists.
- Q4: elkjs is Java that GWT transpiles to JavaScript, 8.2 MB unpacked at version 0.12.1. Its FAQ lists recurring bundler problems. Its licence changed to "EPL-2.0 OR GPL-3.0-or-later" at version 0.12.0.
- Q4: d3-dag entered "light maintanence mode" in 2023 and released again from April 2026. It ignores edge labels in its dagre-compatible layer. It does not support compound graphs, so it cannot draw sub-process boxes.
- Q4: bpmn-auto-layout treats annotations and data references as decorations that "never influence ranks, semantic bands, or flow-node placement". Step attributes, such as an attached file, can follow the same rule.
- Q4 and Q5: bpmn-js offered in-place expansion of sub-processes from 2016. In 2022 its team called that "not enough" and added drill-down planes with breadcrumbs. Fixes for collapse behaviour continued into version 18.
- Q4: A tidy tree centres each parent over its children, so a new leaf at the right edge moves ancestors. Leaf slots by token index keep earlier leaves fixed (inference). A study of degree-of-interest trees found no speed gain over a plain tree browser.
- Q4: A browser page cannot open a local file by its path. The File System Access API exists only in Chromium browsers.
- Q6: bpmn-auto-layout tests deterministic geometry against invariants over a fixture corpus. Text snapshots of coordinates can replace most pixel comparisons, which Playwright ties to one platform.

## 2. Q4: Layout and interaction

### 2.1 Two layouts for BPMN-style flows

The obvious candidate is a layered engine, such as elkjs or dagre, which the request names. This section argues that a structure-derived layout is the fallback to beat for flows that keep the expression structure.

A structure-derived layout is a fold over the expression tree. A sequence places its children left to right. A choice stacks its branches between a split gateway and a join gateway. A repeat draws its item with a return arc beneath it. Each box knows its width, height and entry point before its parent places it.

The railroad library by Tab Atkins shows the method in code ([railroad.js](https://raw.githubusercontent.com/tabatkins/railroad-diagrams/gh-pages/railroad.js)). Each constructor reads the sizes of its direct children in one pass. A sequence adds widths with `this.width += item.width + (item.needsSpace?20:0);`. A later `format()` call only places the children inside the sizes that the constructors computed.

The library has no global optimisation and no automatic line breaks. The author must choose `Stack` when "a simple Sequence would be too wide" ([README-js](https://raw.githubusercontent.com/tabatkins/railroad-diagrams/gh-pages/README-js.md)). Text widths are estimates, because SVG "can't actually respond to the sizes of content" ([README](https://github.com/tabatkins/railroad-diagrams)).

BPMN research also has structure-derived layouts. Kitzmann and co-authors run a grid layout once for each sub-process ([CEC 2009](https://etc.leif.me/papers/Kitzmann2009a.pdf)). Gschwind and co-authors use the block structure of process models for a linear-time layout during modelling ([JVLC 2014](https://hreijers.win.tue.nl/H.A.%20Reijers%20Bestanden/gschwind-layout-flat.pdf)). They tested it on about 700 insurance models against yWorks and dot.

Inference: under an edit, the structure-derived layout changes only the edited subtree and the sizes of its ancestors. Siblings move by translation, and no element changes its order. Inference: this is a stronger stability guarantee than any layered engine offers. No published study measures it, so the claim rests on the code, not on use.

Chiplunkar and Pit-Claudel give the first formal treatment of railroad layout ([ECOOP 2026](https://arxiv.org/abs/2509.15834)). Sizes come from the sub-diagrams, and only line wrapping is optimisation. They state: "This helps us avoid the instability typical of optimization-based 2-dimensional layouts". The paper does not define or measure stability. Their prototype is librrd ([repository](https://github.com/epfl-systemf/librrd)). The paper also cites drift in SQLite: "the disconnection between the source grammar and the diagram leads to a semantic bug".

Inference: this layout has four properties that a layered engine does not give by default:

- The layout is deterministic and needs one pass over the tree
- Edges do not cross, because the drawing nests like the expression
- Labels sit inside nodes, because each node is one symbol occurrence
- Text order equals drawing order, from left to right and from top to bottom.

Inference: the structure-derived layout stops working when the graph loses the expression structure. Two cases matter. A minimised automaton merges states, so its nodes no longer map to occurrences. A recursive reference that compiles to a loop adds a back edge that no repeat operator owns. A layered engine handles both cases.

A layered engine breaks cycles, assigns layers, orders nodes and assigns coordinates, each over the whole graph. A small edit can therefore move nodes far from the edit (section 2.3).

The BPMN layout tool from bpmn.io takes a middle path. The design document of bpmn-auto-layout describes "a constrained layered layout" ([LAYOUT.md](https://raw.githubusercontent.com/bpmn-io/bpmn-auto-layout/main/docs/LAYOUT.md)). It adds: "This is a BPMN-specific layered algorithm, not a generic graph layout followed by BPMN patches." Its main points are these:

- Ranks come from longest paths, and each rank becomes an x-position
- Semantic bands give the y-positions, and branches of a gateway alternate above and below the main path
- A deterministic depth-first search marks back edges, which the router draws last, around the outside
- The order of elements in the file breaks ties, so "The same semantic input therefore produces byte-identical output"
- The engine discards existing coordinates, so stability comes from determinism alone.

Inference: bpmn-auto-layout shows that a layered layout with domain rules and file order meets the needs of BPMN-style flows. The document sits on the main branch, which carries the 2.0.0 alphas of July 2026. The stable release 1.3.0 can differ.

### 2.2 Layered engines in the browser

Three layered engines run in the browser without a server: elkjs, dagre and d3-dag.

elkjs, the JavaScript build of the Eclipse Layout Kernel, is the most complete. ELK Layered has about 140 layout options ([ELK paper, section 2](https://arxiv.org/abs/2311.00533)). It supports compound nodes, ports, edge labels and self-loops. Its build runs in a Web Worker through the `workerUrl` option ([elkjs README](https://github.com/kieler/elkjs)). The ELK authors state that "elkjs is not as performant for larger graphs as the original ELK" and that ELK "typically had (only) one to three active developers" ([ELK paper, section 4](https://arxiv.org/abs/2311.00533)).

dagre is the engine that Mermaid used first. Version 3.0.0 rewrote it in TypeScript on 22 March 2026 ([changelog](https://raw.githubusercontent.com/dagrejs/dagre/master/changelog.md)). Version 3.1.0 added layout settings for each cluster and a stable-layout mode. The README warns that only the `@dagrejs` package receives updates ([dagre README](https://github.com/dagrejs/dagre)).

d3-dag targets small graphs. Its README reports 5.1 ms for a 184-node graph with the fast preset ([d3-dag README](https://github.com/erikbrinkman/d3-dag)). Its 2023 README said "I won't be trying to expand to new use cases" ([d3-dag 1.1.0 README](https://unpkg.com/d3-dag@1.1.0/README.md)). The project released nothing between September 2023 and April 2026.

Graphviz also runs in the browser. The package `@hpcc-js/wasm-graphviz` 1.30.0 has no dependencies and an Apache-2.0 licence. It is 2,082,842 bytes unpacked ([npm](https://registry.npmjs.org/@hpcc-js/wasm-graphviz/latest)).

### 2.3 Stability under edits

The ELK authors describe what users want ([ELK paper, section 4.2](https://arxiv.org/abs/2311.00533)). Users complain when they "appear to have 'little control' over actual results". They also complain when drawings "change drastically after minimal changes of the graph".

Layer constraints "did not really seem to catch on, probably for lack of awareness of these options", so the team turned to model order. Model order takes the order of nodes and edges from the textual input file. ELK uses this order as a tie-breaker in cycle breaking, layer assignment and crossing minimisation. The order "should only be violated if the graph can clearly be improved by it". The team also avoids randomness, so the same input gives the same drawing.

The Kiel group published the method and tested it on SCCharts and Lingua Franca ([IVAPP 2023](https://rtsys.informatik.uni-kiel.de/~biblio/downloads/papers/ivapp23a.pdf), [Diagrams 2024](https://arxiv.org/abs/2406.11393)). They claim that "Model order can combine the desire for control of secondary notation with automatic layout, without additional overhead". The claim holds only if the textual order matches the intention of the user.

ELK also offers two interactive options:

- `org.eclipse.elk.interactiveLayout` keeps the graph "changeable interactively and by setting constraints" ([option page](https://eclipse.dev/elk/reference/options/org-eclipse-elk-interactiveLayout.html))
- `crossingMinimization.semiInteractive` keeps the order of nodes in a layer from earlier positions ([option page](https://eclipse.dev/elk/reference/options/org-eclipse-elk-layered-crossingMinimization-semiInteractive.html)).

dagre 3.1.0 added `useDynamic` and `corePath`. The changelog says the change "Enables persistent node ordering and layout stability when modifying graph structures" ([changelog](https://raw.githubusercontent.com/dagrejs/dagre/master/changelog.md)).

Misue, Eades, Lai and Sugiyama named the problem in 1995 ([JVLC 6(2)](https://www.semanticscholar.org/paper/Layout-Adjustment-and-the-Mental-Map-Misue-Eades/8f3ea4c4374a59f2625a68e9498da03195f2efc0)). Their abstract warns about a creation method that adjusts a layout. Such a method "may totally rearrange the layout and thus destroy the user's 'mental map' of the diagram". In the same year, North described DynaDAG, an incremental layered layout ([GD 1995](https://link.springer.com/content/pdf/10.1007/BFb0021824.pdf)). He wrote that "Most graph drawing algorithms to date are not incrementally stable." He ranked node stability above edge stability.

Inference: thirty years later, the browser engines still compute each layout from nothing by default. Only the ELK options and the new dagre mode carry order from the source text or from the previous drawing.

### 2.4 Edge labels, self-loops and back edges

ELK places edge labels as dummy nodes during layering. A user learned that "edge labels are treated like nodes during layout" ([ELK issue 1091](https://github.com/eclipse-elk/elk/issues/1091)). Reports on the ELK tracker describe self-loops that cross other edges and labels that overlap ([issue 469](https://github.com/eclipse-elk/elk/issues/469), [issue 603](https://github.com/eclipse-elk/elk/issues/603)). Other reports describe exceptions in self-loop label placement and with splines and hierarchy ([issue 368](https://github.com/eclipse-elk/elk/issues/368), [issue 352](https://github.com/eclipse-elk/elk/issues/352)).

Inference: in the BPMN-style drawing, terminals are nodes, not edge labels. Edges carry no labels, except perhaps repeat bounds such as `{2,4}`. This choice removes most of the label problem from the layered engine.

Mermaid state diagrams show the same weakness. Issue 8706 reports Mermaid 12.2.0 with the default ELK layout on 6 November 2026 ([issue 8706](https://github.com/mermaid-js/mermaid/issues/8706)). Two self-transitions on one state render as "flat loops a few pixels tall beside the state". One loop line "runs through the other's label". With dagre, one of the two labels is not drawn at all ([issue 6888](https://github.com/mermaid-js/mermaid/issues/6888)). An older report says that the state diagram code does not tell self-loops from other transitions ([issue 6336](https://github.com/mermaid-js/mermaid/issues/6336)).

Issue 8580 reports that ELK labels of bidirectional edges overlap each other and nodes since Mermaid 12.1.0. The same diagram is clean in 12.0.0 and with dagre ([issue 8580](https://github.com/mermaid-js/mermaid/issues/8580)). Inference: label placement in a general engine can regress in a minor release, so a pinned engine version needs layout tests.

### 2.5 Collapse, expand and drill-down

bpmn-js offers two mechanisms for sub-processes. The older one changes a sub-process between collapsed and expanded in place, since version 0.17.0 of 2016. The newer one, from 9.0.0 of 18 February 2022, opens a collapsed sub-process on its own plane ([announcement](https://bpmn.io/blog/posts/2022-bpmn-js-900-collapsed-subprocesses.html)). The announcement gives the reason: "Expanding and collapsing sub-processes in place is not enough." The user can "inspect and change their contents without touching the parent element".

The changelog shows the cost of the in-place mechanism ([CHANGELOG](https://raw.githubusercontent.com/bpmn-io/bpmn-js/develop/CHANGELOG.md)). These are the entries:

- 9.0.4: "correct hanging sequence flow label after collapsing sub-process"
- 9.4.0: "move labels when collapsing sub-process" and "assign default size when expanding element"
- 18.13.2: "correct handling of annotations during sub-process collapse/expand, copy/paste, and remove actions".

The drill-down plane has its own diagram record in the BPMN file. In bpmn-auto-layout, the child plane has "no coordinate relationship to the collapsed parent shape" ([LAYOUT.md](https://raw.githubusercontent.com/bpmn-io/bpmn-auto-layout/main/docs/LAYOUT.md)).

Camunda Cockpit uses the same drill-down ([Cockpit drill-down](https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/drilldown/)). Breadcrumbs in the upper left "show the process hierarchy". The collapsed shape "indicates if there are running child instances or if an incident occurred in a child activity".

Inference: drill-down keeps the parent drawing unchanged, so the mental map of the parent survives. In-place expansion changes the size of a box. Then the engine must either move the neighbours or let shapes overlap.

The Cytoscape.js extension for expand and collapse moves nothing by default ([README](https://github.com/iVis-at-Bilkent/cytoscape.js-expand-collapse)). The user can name a layout "to rearrange after expand/collapse" or choose a fisheye view. The README recommends cose-bilkent "with randomize: false to preserve mental map upon expand/collapse". A collapse replaces the children with meta edges to the collapsed node. The README states that the extension "is no longer being maintained".

### 2.6 Overlays for the state and history of a trace

<!-- tc: depart spelling because the URL path of the example keeps its American spelling -->
bpmn-js offers four ways to mark elements ([colour example](https://github.com/bpmn-io/bpmn-js-examples/tree/main/colors)):

- A marker adds a CSS class to an element through `canvas.addMarker('UserTask_1', 'highlight')`
- An overlay places HTML over an element through `overlays.add`
- A colour in the diagram record persists in the BPMN file through `modeling.setColor`
- A custom renderer draws shapes and connections with computed colours.

Camunda Cockpit, built on bpmn-js, uses these mechanisms in its definition and history views. The definition view shows incidents "together with an instance counter label" on the diagram ([definition view](https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-definition-view/)). The history view adds a heatmap "showing which nodes and sequence flows have the most activity" ([history views](https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-history-views/)). It measures activity "by the number of tokens which have been processed by the node or sequence flow". The heatmap is in the enterprise edition only.

The heatmap has years of use. It arrived in Camunda 7.4.0 in November 2015, after a community plugin ([7.4.0 release](https://camunda.com/blog/2015/11/camunda-bpm-740-released/)). A forum answer says it needs only "a mapping of activityId to a numerical value" ([forum thread](https://forum.camunda.io/t/heatmap-for-durations-instead-of-frequencies/1642)). The same answer says that no API changes the measure, for example to durations. A user must copy the plugin.

A third-party plugin discarded the identifier that `overlays.add()` returns ([operaton-cockpit-plugins PR 71](https://github.com/datakurre/operaton-cockpit-plugins/pull/71)). It leaked one empty overlay container per activity for each filter change.

Inference: the trace views map onto these mechanisms:

- The current frontier maps onto markers, because a marker is a class that CSS can style
- Visit counts map onto badge overlays, as in the Cockpit counters
- A heatmap over many traces belongs to the process-mining survey
- The path of one trace maps onto marked nodes and edges in visit order.

### 2.7 A diagram linked to a tree view

Camunda Cockpit places an activity instance tree beside the diagram ([instance view](https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-instance-view/)). The tree "contains a node for each activity that is currently active in the process instance". The tree and the diagram share the selection in both directions: "the selected flow node will also be selected in the tree and the associated variables will be shown and vice versa".

The history view has no tree. It lists past activities in an audit log table with start time, end time and state ([history views](https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-history-views/)).

Inference: the Cockpit tree shows only live scopes, much like a call stack. A parse tree also holds the finished rules, so a shared selection extends the Cockpit pattern to them.

### 2.8 Parse tree layout and growth at the right edge

The obvious candidate is `d3.tree` from d3-hierarchy. It uses "the Reingold–Tilford “tidy” algorithm, improved to run in linear time by Buchheim et al." ([d3-hierarchy](https://d3js.org/d3-hierarchy/tree)). It takes one node size for all nodes and has no accessor for the size of each node.
The package d3-flextree adds variable node sizes. Its last release is 2.1.2 of November 2021, and it depends on d3-hierarchy 1.x ([npm](https://registry.npmjs.org/d3-flextree)).

The tidy algorithm follows the rule "A father should be centered over its sons" ([Reingold and Tilford 1981](https://reingold.co/tidier-drawings.pdf)). A new leaf at the right edge therefore moves its parent and every open ancestor. Moen noted in 1990 that earlier tree algorithms "cannot reuse layout information when the trees changes" ([Moen 1990](https://www.semanticscholar.org/paper/Drawing-dynamic-trees-Moen/27b64f7128983e451cbfd3c420cec7eb2b19d0b8)).

Inference: the leaves of a parse tree are the tokens in order. A slot for each token index gives each leaf a fixed x position. The layout places a rule node over the span of its leaves. A new token then adds one leaf and widens only the open rules on the right spine. That spine is the stack of open rules, so the output view also shows the call stack. Alternative parse trees of one token sequence share the same x axis, so they can stack vertically for comparison.

Graham and Kennedy survey views for pairs and sets of trees ([Information Visualization, 2010](https://napier-repository.worktribe.com/output/213124/a-survey-of-multiple-tree-visualisation)). This survey did not read their text.

### 2.9 Readable large and repetitive trees

Furnas defines a degree of interest as the a priori importance of a node minus its distance from the focus ([CHI 1986](https://cspages.ucalgary.ca/~saul/581/exer.eps/4furnas86.pdf)). The view shows the nodes whose interest exceeds a threshold. In a tree, this rule keeps the path from the root to the focus open while distant branches fold. Card and Nation add bounds on the screen space to this rule in degree-of-interest trees ([AVI 2002](https://www.semanticscholar.org/paper/Degree-of-interest-trees:-a-component-of-an-user-Card-Nation/7922679e8e9430bc1c578ce70de98cd29a8a16db)). SpaceTree rescales branches to fit the screen and shows "preview icons summarizing the topology of the branches that cannot be expanded" ([SpaceTree](https://www.cs.umd.edu/hcil/spacetree/)).

Real use gave mixed results:

- SpaceTree helped with navigation to nodes visited before and with estimates of the tree topology. Its last release was a Java applet of April 2003, and "As of 2020 the Java Applets do not run on web browsers for security reasons" ([SpaceTree](https://www.cs.umd.edu/hcil/spacetree/)).
- Budiu, Pirolli and Fleetwood compared a degree-of-interest tree with an Explorer-like browser ([AVI 2006](https://www.cs.cmu.edu/~ralucav/papers/avi06-short.pdf)). Users of the degree-of-interest tree "do not complete the task faster". They visited 80 nodes against 53. The tree was "more forgiving" after mistakes, but it had "a greater potential for distraction".
- Jakobsen and Hornbæk state that "fisheye techniques are largely absent in widespread user interfaces" ([HCIV 2009](https://dl.ifip.org/db/conf/interact/hciv2009/JakobsenH09.pdf)). In their study of a code editor, users did not always see "what semantic relation" caused lines to appear in the context area.

Inference: the output view can apply a fixed rule instead of a computed threshold. Each completed pass of a loop folds into a summary glyph with its length. The current pass and the path to it stay open. A user can pin any pass open. A fixed rule answers the complaint that users could not see why an element appeared.

### 2.10 Step attributes, instance data and attached files

BPMN draws a data object as a page with a folded corner, linked to its task by an association. A state follows the name in square brackets, such as "Contract [unsigned]" ([bpmtips](https://bpmtips.com/bpmn-miwg-demonstration-2018-nearly-everything-you-always-wanted-to-know-about-data-objects-but-were-afraid-to-ask/)). In bpmn-auto-layout, "data references are decorations: they never influence ranks, semantic bands, or flow-node placement" ([LAYOUT.md](https://raw.githubusercontent.com/bpmn-io/bpmn-auto-layout/main/docs/LAYOUT.md)). The engine places them on a free side of their owner or reports an error.

Both Camunda tools show the variables of one instance in a panel beside the diagram. Operate shows "Details, Input Mappings, and Output Mappings" for the selected element ([Operate](https://docs.camunda.io/docs/components/operate/userguide/basic-operate-navigation/)). A migration bug shows the data behind the overlays ([issue 735](https://github.com/camunda/camunda-7-to-8-migration-tooling/issues/735)). Operate showed no history until someone set the `state` and `flowNodeScopeKey` fields by hand. Then "instance count badges on the diagram are updated, along with executed flow node highlighting". Inference: each trace step needs a state and the identifier of its enclosing rule instance.

A browser page cannot open a local file from its path. Chrome and Edge forbid "navigation to file:// URLs from non-file:// URLs", and only an Edge policy for intranet shares relaxes the block ([Lawrence](https://textslashplain.com/2019/10/09/navigating-to-file-urls/)). The File System Access API runs only in Chromium browsers and needs a user gesture ([Chrome developers](https://developer.chrome.com/docs/capabilities/web-apis/file-system-access)). File handles come from a picker, from drag-and-drop or from a directory that the user grants. A stored handle needs permission again in a later session. A Mozilla reviewer wrote that the API "should be marked harmful" ([standards position](https://github.com/mozilla/standards-positions/issues/154)).

Inference: attachments can use paths relative to a project directory. In Chromium, the user grants that directory once. In other browsers, the native Rust host can serve the files over HTTP.

### 2.11 Pan, zoom, Shadow DOM and accessibility

In Safari, d3-zoom receives wheel events only over painted content, so the issue suggests a background rectangle ([issue 76](https://github.com/d3/d3-zoom/issues/76)). bpmn-js fixed the handling of "wheel events over overlays" in version 18.33.0 of November 2026 ([changelog](https://raw.githubusercontent.com/bpmn-io/bpmn-js/develop/CHANGELOG.md)). Inference: HTML badges over the SVG need the same care.

D3 had one Shadow DOM bug, a missing namespace, which version 3.5.4 fixed ([d3 issue 2161](https://github.com/d3/d3/issues/2161)). This survey found no report about d3-zoom or d3-drag in a shadow root. d3-drag uses the parent node as its default container ([d3-drag](https://d3js.org/d3-drag)). Inference from the code: for a direct child of a shadow root, `d3.pointer` falls back to page coordinates ([pointer.js](https://raw.githubusercontent.com/d3/d3-selection/main/src/pointer.js)). A ShadowRoot has no SVG matrix and no bounding box. A container set to the SVG element avoids this.

Outside the shadow root, event retargeting makes the host element the target. A drag library fixed this with `event.composedPath()` ([pragmatic-drag-and-drop issue 15](https://github.com/atlassian/pragmatic-drag-and-drop/issues/15)).

Graphics ARIA defines the roles `graphics-document`, `graphics-object` and `graphics-symbol` ([W3C, 2018](https://www.w3.org/TR/graphics-aria-1.0/)). It asks for navigation of nested objects "similar to nested lists". It warns that "presenting and navigating every element will make the graphic harder to understand and use". bpmn-js added keyboard access to its menus, its drill-down button and its breadcrumbs in versions 17 and 18.

### 2.12 SVG export

The bpmn-js licence is MIT with one added clause ([LICENSE](https://raw.githubusercontent.com/bpmn-io/bpmn-js/develop/LICENSE)). The watermark that links to bpmn.io "MUST NOT be removed or changed", and it "must stay fully visible and not visually overlapped by other elements". The licence text does not mention exports.

bpmn-js loses the CSS classes of `canvas.addMarker` on export, and the report carries the label wontfix ([diagram-js issue 168](https://github.com/bpmn-io/diagram-js/issues/168)). Other export bugs broke the ids of arrowhead markers, with alpha colours and with two diagrams on one page ([issue 981](https://github.com/bpmn-io/bpmn-js/issues/981), [issue 706](https://github.com/bpmn-io/bpmn-js/issues/706)). A forum reply of August 2026 states: "Our license doesn’t require the watermark on the exported diagrams" ([forum](https://forum.bpmn.io/t/is-kroki-baseviewer-savesvg-license-compliant/15229)). The forum does not show the role of its author.

Inference: styles in the stylesheet of a shadow root do not travel with the SVG. The export must inline computed styles or embed the stylesheet. Marker ids need a prefix for each component instance.

## 3. Q5: Scope

The fallback scope is the request as written. The evidence argues for cuts:

- Regretted: in-place expansion of sub-processes in bpmn-js, with collapse fixes into version 18 (section 2.5)
- Unused: ELK layer constraints, which "did not really seem to catch on" (section 2.3)
- Absent from practice: fisheye views (section 2.9)
- Requested and never received: Camunda heatmaps of durations, because no API changes the measure (section 2.6).

Inference: the first slice can use drill-down only, CSS-class overlays, count badges and the token-slot tree. Node dragging conflicts with derived positions, and bpmn-auto-layout "discards existing coordinates". The ELK authors report that users want control. So the slice needs a decision: dragged positions persist in KDL, or the slice has no dragging.

## 4. Q6: Test seams

The fallback is screenshot comparison in a browser. Playwright warns that "Browser rendering can vary based on the host OS, version, settings, hardware" ([Playwright](https://playwright.dev/docs/test-snapshots)). Its baselines carry a platform suffix, such as `chromium-darwin`. Vitest 4.0 made Browser Mode stable in October 2025, but its screenshot matcher is new ([Vitest 4](https://vitest.dev/blog/vitest-4)).

bpmn-auto-layout tests geometry instead. It states: "The same semantic input therefore produces byte-identical output" ([LAYOUT.md](https://raw.githubusercontent.com/bpmn-io/bpmn-auto-layout/main/docs/LAYOUT.md)). It also states: "Wrong-way docking and non-orthogonal connections must remain zero across the fixture corpus". Playwright can compare such text output with `toMatchSnapshot`.

Inference: the design gives these seams without pixels:

- The parser and the rule checks run natively in Rust.
- The layout takes measured label sizes as input, so tests can feed fixed sizes.
- Invariants check that no label overlaps a node and that siblings keep source order.
- A stability test edits one rule and checks that other rules keep their coordinates.
- A tree test adds one token and checks that earlier leaves keep their positions.

Screenshots then cover a few fixtures in one pinned browser.

## 5. Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance status | Walk-away result |
|---|---|---|---|---|---|---|
| elkjs | https://github.com/kieler/elkjs | Layered and other layouts, transpiled from Java | EPL-2.0 OR GPL-3.0-or-later | 0.12.1, 28 October 2026 | Active, releases every 4 to 8 months, 92 open issues | Positions in JSON, no drawing |
| @dagrejs/dagre | https://github.com/dagrejs/dagre | Layered layout | MIT | 3.1.3, 24 October 2026 | Active since the 2026 rewrite, 171 open issues | Positions in a graph object, no drawing |
| d3-dag | https://github.com/erikbrinkman/d3-dag | Layered layout for small graphs | MIT | 1.3.0, 1 November 2026 | Revived in 2026 after a gap of 30 months | Positions only |
| railroad-diagrams | https://github.com/tabatkins/railroad-diagrams | Railroad diagrams computed from the expression structure | MIT | No releases, published from the gh-pages branch | 13 open issues | Static SVG with no runtime |
| librrd | https://github.com/epfl-systemf/librrd | Railroad layout with line wrapping, a research prototype | Not stated in the paper | ecoop2026-artifact | Research artifact of ECOOP 2026 | Not checked |
| bpmn-js | https://github.com/bpmn-io/bpmn-js | BPMN rendering and modelling | MIT terms plus a watermark clause | 18.33.0, 16 November 2026 | Active, several releases each month | BPMN XML with diagram interchange, readable by other BPMN tools |
| bpmn-auto-layout | https://github.com/bpmn-io/bpmn-auto-layout | Layered layout for BPMN | MIT | 1.3.0, 11 March 2026, and 2.0.0-alpha.2, 24 July 2026 | Active, 24 open issues | Diagram interchange written into the BPMN XML |
| Camunda 7 Cockpit | https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-instance-view/ | Monitoring of process instances | Community and enterprise editions, heatmap enterprise only | 7.24, 14 October 2025, the last community release ([blog](https://camunda.com/blog/2025/02/migrating-solutions-camunda-7-camunda-8-strategy-update/)) | Community edition ended, enterprise support until at least 2030 | BPMN files stay, plugins and overlays go |
| Camunda 8 Operate | https://docs.camunda.io/docs/components/operate/userguide/basic-operate-navigation/ | Monitoring of process instances | Not checked | Not checked | Active, issue 735 fixed in January 2026 | Not checked |
| Mermaid | https://github.com/mermaid-js/mermaid | Diagrams from text, with dagre or ELK | MIT (not checked) | 12.2.0 in use on 6 November 2026 | Active | Source text stays, and Mermaid computes the layout again |
| @hpcc-js/wasm-graphviz | https://github.com/hpcc-systems/hpcc-js-wasm | Graphviz compiled to WebAssembly | Apache-2.0 | 1.30.0, date not found | Not checked | DOT text, a standard format |
| cytoscape.js-expand-collapse | https://github.com/iVis-at-Bilkent/cytoscape.js-expand-collapse | Expand and collapse of compound nodes | MIT | Not checked | No longer maintained, per the README | Not applicable |
| d3-hierarchy | https://github.com/d3/d3-hierarchy | Tidy tree, cluster and other hierarchy layouts | ISC | 3.1.3, 3 November 2026 | Sparse, no release from April 2022 to November 2026 | Positions only |
| d3-flextree | https://github.com/klortho/d3-flextree | Tidy tree with variable node sizes | WTFPL | 2.1.2, 8 November 2021 | No release for five years | Positions only |
| SpaceTree | https://www.cs.umd.edu/hcil/spacetree/ | Tree browser with dynamic rescaling | Noncommercial | 1.6, April 2003 | Ended, the Java applets no longer run | Nothing runs |

## 6. Silences

These searches found nothing:

- No report of d3-zoom or d3-drag failures in a shadow root. Queries: "d3-zoom shadow DOM issue github wheel event custom element" and "\"d3-drag\" OR \"d3-zoom\" \"shadow root\" OR \"shadowRoot\" OR \"shadow DOM\" issue event target retargeting".
- No study of the stability of railroad layouts under grammar edits. Query: "railroad syntax diagram layout stability after grammar edit incremental user study".
- No OMG text for the notation of BPMN data objects. Query: "BPMN 2.0.2 specification Data Object \"folded\" corner shape collection marker \"state\" square brackets name OMG formal/13-12-09".
- No primary text for the conclusions of the Graham and Kennedy survey. Query: "\"A survey of multiple tree visualisation\" Graham Kennedy 2010 side by side juxtaposition comparison trees TreeJuxtaposer".
- No release date for `@hpcc-js/wasm-graphviz` 1.30.0. The full registry document was too large to fetch.
- No mention of Shadow DOM in the bpmn-js changelog.
- No highlighted paths of executed elements in the Camunda 7 history documentation. The Operate documentation does not explain its badges or how its history shows loops.

This pass did not survey Lit against plain custom elements, Vite library mode, `@web/test-runner` or the Cytoscape.js layout extensions.

## 7. Sources

- https://arxiv.org/abs/2311.00533
- https://raw.githubusercontent.com/tabatkins/railroad-diagrams/gh-pages/railroad.js
- https://raw.githubusercontent.com/tabatkins/railroad-diagrams/gh-pages/README-js.md
- https://github.com/tabatkins/railroad-diagrams
- https://etc.leif.me/papers/Kitzmann2009a.pdf
- https://hreijers.win.tue.nl/H.A.%20Reijers%20Bestanden/gschwind-layout-flat.pdf
- https://arxiv.org/abs/2509.15834
- https://github.com/epfl-systemf/librrd
- https://raw.githubusercontent.com/bpmn-io/bpmn-auto-layout/main/docs/LAYOUT.md
- https://github.com/kieler/elkjs
- https://raw.githubusercontent.com/dagrejs/dagre/master/changelog.md
- https://github.com/dagrejs/dagre
- https://github.com/erikbrinkman/d3-dag
- https://unpkg.com/d3-dag@1.1.0/README.md
- https://registry.npmjs.org/@hpcc-js/wasm-graphviz/latest
- https://rtsys.informatik.uni-kiel.de/~biblio/downloads/papers/ivapp23a.pdf
- https://arxiv.org/abs/2406.11393
- https://eclipse.dev/elk/reference/options/org-eclipse-elk-interactiveLayout.html
- https://eclipse.dev/elk/reference/options/org-eclipse-elk-layered-crossingMinimization-semiInteractive.html
- https://www.semanticscholar.org/paper/Layout-Adjustment-and-the-Mental-Map-Misue-Eades/8f3ea4c4374a59f2625a68e9498da03195f2efc0
- https://link.springer.com/content/pdf/10.1007/BFb0021824.pdf
- https://github.com/eclipse-elk/elk/issues/1091
- https://github.com/eclipse-elk/elk/issues/469
- https://github.com/eclipse-elk/elk/issues/603
- https://github.com/eclipse-elk/elk/issues/368
- https://github.com/eclipse-elk/elk/issues/352
- https://github.com/mermaid-js/mermaid/issues/8706
- https://github.com/mermaid-js/mermaid/issues/6888
- https://github.com/mermaid-js/mermaid/issues/6336
- https://github.com/mermaid-js/mermaid/issues/8580
- https://bpmn.io/blog/posts/2022-bpmn-js-900-collapsed-subprocesses.html
- https://raw.githubusercontent.com/bpmn-io/bpmn-js/develop/CHANGELOG.md
- https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/drilldown/
- https://github.com/bpmn-io/bpmn-js/issues/1483
- https://github.com/iVis-at-Bilkent/cytoscape.js-expand-collapse
- https://github.com/bpmn-io/bpmn-js-examples/tree/main/colors
- https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-definition-view/
- https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-instance-view/
- https://docs.camunda.org/manual/latest/webapps/cockpit/bpmn/process-history-views/
- https://camunda.com/blog/2015/11/camunda-bpm-740-released/
- https://forum.camunda.io/t/heatmap-for-durations-instead-of-frequencies/1642
- https://github.com/datakurre/operaton-cockpit-plugins/pull/71
- https://d3js.org/d3-hierarchy/tree
- https://registry.npmjs.org/d3-hierarchy
- https://registry.npmjs.org/d3-flextree
- https://reingold.co/tidier-drawings.pdf
- https://www.semanticscholar.org/paper/Drawing-dynamic-trees-Moen/27b64f7128983e451cbfd3c420cec7eb2b19d0b8
- https://napier-repository.worktribe.com/output/213124/a-survey-of-multiple-tree-visualisation
- https://cspages.ucalgary.ca/~saul/581/exer.eps/4furnas86.pdf
- https://www.semanticscholar.org/paper/Degree-of-interest-trees:-a-component-of-an-user-Card-Nation/7922679e8e9430bc1c578ce70de98cd29a8a16db
- https://www.cs.umd.edu/hcil/spacetree/
- https://www.cs.cmu.edu/~ralucav/papers/avi06-short.pdf
- https://dl.ifip.org/db/conf/interact/hciv2009/JakobsenH09.pdf
- https://bpmtips.com/bpmn-miwg-demonstration-2018-nearly-everything-you-always-wanted-to-know-about-data-objects-but-were-afraid-to-ask/
- https://docs.camunda.io/docs/components/operate/userguide/basic-operate-navigation/
- https://github.com/camunda/camunda-7-to-8-migration-tooling/issues/735
- https://textslashplain.com/2019/10/09/navigating-to-file-urls/
- https://developer.chrome.com/docs/capabilities/web-apis/file-system-access
- https://github.com/mozilla/standards-positions/issues/154
- https://github.com/d3/d3-zoom/issues/76
- https://github.com/d3/d3/issues/2161
- https://d3js.org/d3-drag
- https://raw.githubusercontent.com/d3/d3-selection/main/src/pointer.js
- https://github.com/atlassian/pragmatic-drag-and-drop/issues/15
- https://www.w3.org/TR/graphics-aria-1.0/
- https://github.com/bpmn-io/diagram-js/issues/168
- https://github.com/bpmn-io/bpmn-js/issues/981
- https://github.com/bpmn-io/bpmn-js/issues/706
- https://forum.bpmn.io/t/is-kroki-baseviewer-savesvg-license-compliant/15229
- https://camunda.com/blog/2025/02/migrating-solutions-camunda-7-camunda-8-strategy-update/
- https://playwright.dev/docs/test-snapshots
- https://vitest.dev/blog/vitest-4
- https://github.com/bpmn-io/bpmn-js
- https://github.com/bpmn-io/bpmn-auto-layout
- https://github.com/mermaid-js/mermaid
- https://github.com/hpcc-systems/hpcc-js-wasm
- https://github.com/d3/d3-hierarchy
- https://github.com/klortho/d3-flextree
- https://ar5iv.labs.arxiv.org/html/2311.00533
- https://registry.npmjs.org/elkjs
- https://registry.npmjs.org/@dagrejs/dagre
- https://registry.npmjs.org/d3-dag
- https://raw.githubusercontent.com/bpmn-io/bpmn-js/develop/LICENSE

## 8. Decision records

These decision records cite this survey:

- `docs/adr/layout-in-the-core.md`: the core computes the coordinates, and D3 only draws.
- `docs/adr/rule-diagrams-in-bpmn-style.md`: a reference box opens the diagram of its rule, and nothing expands in place.
