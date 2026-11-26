# Grammar workbench: the first milestone

This file keeps the words of the human for the first milestone, verbatim. The human pasted the request on 2026-11-17 and made decisions in the conversation that followed. Later sessions do not receive that conversation.

The decisions replace parts of the pasted request. For example, the core is a Rust library, not a vanilla JavaScript component. The tracer is one Earley parser, not Glushkov automata with a graph-structured stack. Where this file and a decision record in `docs/adr/` disagree, the decision record wins. `CONTEXT.md` holds the vocabulary.

<!-- tc: exempt because the text below is the human's own request and decisions, quoted verbatim -->

---

## Decisions in the conversation

The label before each quote names its topic and its date.

- Core, 2026-11-17: "I want it to run in wasm as well, so Rust is a good strategy"
- Serialised form, 2026-11-17: "available in a serialised form usable by many languages"
- KDL for serialisation, 2026-11-17: "because it has better lexical characteristics for LLM generation"
- Terminals, 2026-11-17: "Lexical sophistication can come later if required."
- Parser, 2026-11-17: incremental parsing "is unlikely to be a bottleneck"
- Grammar input, 2026-11-17: "I want the grammar input to be EBNF (with extensions such as min/max repeats)"
- No KDL schema, 2026-11-17: "I agree with all of this"
- First milestone, the vertical slice, 2026-11-17: "Option 1 is my choice"
- Test seams, 2026-11-17: "I agree with all of this"
- Trees, 2026-11-17: a grammar "for structure more than knowledge modelling"
- Diagrams, 2026-11-17: "Note that I'm not looking to a single diagram for an entire grammar. I want right-regular subgrammars to be able to use an FSM representation i.e. a mixed-representation presentation that can be used for 'current state' as well as history visualisation. Think BPMN for example."
- Views, 2026-11-17: one view shows "where in 'the loop' the process currently is, and what are the next options". The other shows the history, "which is in effect the entire generated output of the grammar to that point". Both views can show attributes, such as "a referenced resource in the output".
- Loops, 2026-11-17: completed passes appear "under the repetition operator". "This is all just about presentation."
- Scope, 2026-11-18: "Remember that this is not *only* for process visualisation. Sometimes it will be e.g. a tutorial step model or something like that."
- Tutorial grammars, 2026-11-18: "The tutorial grammar will mostly be used to guide the LLM in creating a tutorial, but visualising that is fine."
- Agreement to the requirements, 2026-11-18: "This all looks fine otherwise"

## Later messages from the human, which amend the pasted request

1. "Feel free to push back and ask questions about this project and it's goals."
2. "I would prefer this written in typescript. You can use D3 if useful. We will need to design/agree on an input grammar formalism."
3. "Do any websearch neccessary to find research/existing implementations about this problem/topic"

## The pasted request

# Task: Build an Interactive EBNF Grammar-to-FSM Visualisation Web Component

You are an expert front-end engineer and formal language theorist. Your task is to build a clean, zero-dependency (or minimal-dependency) interactive Web Component (`<grammar-fsm-visualizer>`) in vanilla HTML, CSS, and modern JavaScript (ES Modules) that compiles generative process grammars (EBNF) into interactive Finite-State Machine (FSM) and Recursive Transition Network (RTN) diagrams.

## 1. Theoretical & Algorithmic Requirements

1. **EBNF Parser & AST:**
   - Parse production rules of the form `NonTerminal = Expression ;`.
   - Support terminal literals (`'token'`), non-terminal references (`Identifier`), grouping `( ... )`, alternation `|`, concatenation, and repetition operators (`*`, `+`, `?`).
   - Perform static call-graph analysis on the AST to classify each production rule as either:
     - **Right-Regular (Pure FSM):** Contains no self-embedding or non-tail recursion (or no non-terminal references).
     - **Context-Free (RTN Sub-Grammar):** Invokes other non-terminals or contains recursive self-embedding.

2. **Glushkov’s Construction (EBNF AST to ε-Free NFA):**
   - Compile each production body into an ε-free Non-Deterministic Finite Automaton (NFA) using **Glushkov’s construction** (calculating `Nullable`, `First`, `Last`, and `Follow` sets over indexed symbol positions).
   - Ensure the resulting graph has $n + 1$ states (where $n$ is the number of symbol occurrences in the rule) and zero $\varepsilon$-transitions so diagrams remain uncluttered.
   - Optionally apply subset construction and **Hopcroft’s DFA minimisation** to pure regular rules to reduce visual node count.

3. **Recursive Transition Networks (RTN) & Ambiguity-Aware Path Tracing:**
   - Represent non-terminal transitions as distinct styled edges (e.g., dashed borders with `[NonTerminal]` labels) that act as subroutine calls into child state machines.
   - Implement an interactive step-by-step execution tracer that accepts a space-separated token sequence.
   - Support **parallel frontier highlighting** (NFA simulation across all valid active states simultaneously) and **Graph-Structured Stack (GSS) / Earley-style call stacks** to track multiple active execution paths across recursive RTN sub-grammar boundaries without losing track of ambiguity.
   - Visually distinguish **active states/edges**, **traversed history**, and **pruned/dead-end branches** (low-opacity dashed red) when a branch fails to match the next token.

## 2. Architecture & UI/UX Specifications

1. **Encapsulated Web Component (`<grammar-fsm-visualizer>`):**
   - Use Shadow DOM with clean CSS custom properties (`--bg-color`, `--accent-color`, `--active-color`, `--cfg-color`, `--dead-color`) for easy theming.
   - Expose attributes/properties (`grammar`, `trace`, `readonly`) and dispatch custom DOM events (`state-change`, `trace-complete`, `rule-selected`).

2. **Graph Layout & Interactive SVG Canvas:**
   - Use a layered directed-graph layout algorithm (either implement a clean Sugiyama/Coffman-Graham layered layout with barycentre crossing minimisation, or integrate `elkjs` / `dagre` via ES module import) to route forward edges, self-loops, and backward cycle arcs cleanly without overlapping labels.
   - Support pan, zoom, and node dragging on the SVG canvas.
   - **RTN Drill-Down Navigation:** Clicking any non-terminal edge pushes that sub-grammar onto an interactive breadcrumb navigation bar, smoothly transitioning the viewport to inspect that sub-machine's internal states.

3. **Control Panel & Developer Ergonomics:**
   - Provide a split-pane UI (collapsible editor pane + main SVG viewport).
   - Include a live EBNF editor with syntax error reporting (line/column feedback).
   - Include trace playback controls: `Step Back`, `Step Forward`, `Play/Pause`, `Reset`, a live visual call-stack inspector, and a badge indicating whether the currently viewed rule is **Regular (FSM)** or **Context-Free (RTN)**.
   - Include an **Export SVG / JSON AST** button.

## 3. Deliverables & Code Quality Standards

- Provide the complete project file structure:
  1. `index.html` (demo page with a realistic multi-rule process grammar example, such as an order approval workflow with recursive escalation).
  2. `src/compiler/parser.js` (EBNF lexer, recursive-descent parser, and regularity classifier).
  3. `src/compiler/glushkov.js` (Glushkov NFA builder and optional DFA minimiser).
  4. `src/engine/tracer.js` (RTN / GSS multi-path token stepper).
  5. `src/ui/layout.js` (Layered graph coordinate and Bezier arc router).
  6. `src/components/grammar-fsm-visualizer.js` (The `<grammar-fsm-visualizer>` Web Component).
- Write clean, modular, well-documented ES2022+ JavaScript with JSDoc type annotations.
- Do not leave placeholder comments or `// TODO` stubs; implement all algorithms completely so the component works immediately upon opening `index.html` via a local static server.
