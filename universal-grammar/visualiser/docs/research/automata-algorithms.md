# Automata algorithms for the grammar workbench

Topic 3 research report for the requirements session, 2026-11-17.

The brief asked about Q3 semantics and Q6 test seams first. The coordinator added bounded repeats, terminal marking, test oracles, recovery and parse trees. Later additions were the cost of the stack machinery, a mixed presentation, next symbols, partial trees and loop passes. Decisions since the brief: the core is Rust, native and WebAssembly. The workbench is TypeScript with D3. Grammars are EBNF text with bounded repeats such as `x{2,4}`, every terminal is a whole symbol, and KDL serialises every artifact.

The human wants the parse tree as the main result of a trace, "for structure more than knowledge modelling". The human also wants a mixed presentation, with BPMN as the example. A set of rules that needs no stack appears as one state machine. A reference that needs a stack appears as a call box with drill-down. Process mining and conformance checking belong to another survey.

The human defined two views of a run. The grammar view shows the current position and the next options. The output view shows the output so far as a partial parse tree. In both views, the passes of a right-recursive rule should appear as passes of one loop, like a Kleene star.

Status: final. The section "Silences" lists the gaps that remain. Text marked "Inference" is my reasoning, not a source.

## 1. Findings that change decisions

- **Q3.** The per-rule badge in the request gives wrong answers in both directions. The right test is grammar-wide: compute the strongly connected components of the call graph and test each one for self-embedding ([Nederhof 2000](https://arxiv.org/pdf/cs/9910022)). The badge can claim "no self-embedding", never "not regular", because regularity is undecidable ([Greibach's theorem](https://en.wikipedia.org/wiki/Greibach%27s_theorem)).
- **Q3, Q4.** The position automaton has n + 1 states for n symbol occurrences, but up to n² edges ([Brüggemann-Klein 1993](https://www.sciencedirect.com/science/article/pii/0304397593902874)). Thompson's automaton has linear edges, and each position is one of its symbol edges ([Allauzen and Mohri 2006](https://cs.nyu.edu/~mohri/pub/glush.pdf)). Inference: the diagram can draw junctions in Thompson's style while the tracer keeps position states. The junctions also show tree ambiguity that a position drawing hides.
- **Q3.** Determinisation and minimisation merge symbol occurrences into one node. The tracer then cannot say which occurrence matched, and the parse tree loses its link to the rule text. Trace the position automaton, and offer the minimal automaton only as a read-only view.
- **Q2.** A bounded repeat costs one copy of its body per unit of the upper bound. A nested expansion keeps the edges linear, but a flat expansion makes them quadratic. Every tool examined caps or splits wide bounds: RE2 at 1000, Java at 5000 content-model nodes, Lark at 50 copies.
- **Q2.** Quoted terminals make every bare name a rule reference, so a misspelt rule name becomes an undefined-name error. Where tools define terminals implicitly, a misspelling passes as a new terminal ([ANTLR issue 269](https://github.com/antlr/antlr4/issues/269)).
- **Q3.** One path through a rule diagram does not determine one tree. The expression `(x*)*` reads `x x` along one path but gives two trees. The rule check needs two tests: path ambiguity on the automaton and star normal form on the expression ([Brüggemann-Klein 1993](https://www.sciencedirect.com/science/article/pii/0304397593902874)).
- **Q3, Q5.** An Earley parser whose items are positions in the per-rule automata serves as the whole tracer. It gives the frontier, the next symbols, the call stacks, the partial trees and recursion in one mechanism. This replaces the separate graph-structured stack of the request. The next symbols are the terminals after the dot in the newest Earley set, as in [Earley-based decoders](https://arxiv.org/pdf/2506.01151) and [Marpa](http://jeffreykegler.github.io/Ocean-of-Awareness-blog/individual/2018/05/combinator2.html). Marpa shows such items to users as its main debugging tool ([Marpa progress reports](https://github.com/jeffreykegler/Marpa--R2/blob/master/cpan/pod/Progress.pod)).
- **Q3, Q5.** A set of rules without self-embedding can appear as one automaton. Calls to non-recursive rules expand into copies, and tail calls become jumps, as in OpenFst, JSGF and the Railroad Diagram Generator. These tools drop the rule boundaries that parse trees need. Tag the entry and exit of each copy, as OpenFst's call and return labels allow ([replace.h](https://www.openfst.org/doxygen/fst/html/replace_8h_source.html)). Inference: each tail call back into a rule then starts a loop pass. A cycle through several rules can have several entries, so each pass needs a defined start.
- **Q3, Q4.** The fewest-edits parse costs cubic time in the sequence length ([Aho and Peterson 1972](https://www.semanticscholar.org/paper/A-Minimum-Distance-Error-Correcting-Parser-for-Aho-Peterson/0caae1d9d751f409067b8bcb2266b06f171e5e32)). For a few hundred symbols this fits in a worker. Show unexplained symbols as deletions in the cheapest repair, as lrpar does ([grmtools book](https://softdevteam.github.io/grmtools/master/book/errorrecovery.html)).
- **Q6.** Both oracle types failed in real use when their semantics drifted. Regex engines disagree on 15% of real regexes ([Davis 2019](https://fservant.github.io/papers/Davis_Michael_Coghlan_Servant_Lee_ESECFSE19.pdf)), and Earley's own tree method was wrong ([Scott 2008](https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044)). Naive random generation runs away on recursive rules ([Fuzzing Book](https://www.fuzzingbook.org/html/GrammarFuzzer.html)).

## 2. Findings by question

### Q2. Notation

#### Bounded repeats: copies against counters

Expansion writes the body of `x{n,m}` n times, then adds m − n optional copies. The positions multiply by the upper bound: `x{2,4}` with a three-symbol body gives 12 positions. The shape of the optional tail sets the edge count. Inference, from the follow sets of the position automaton:

- A flat tail `x? x? … x?` lets each copy jump to every later copy. For `x{0,10}` with a one-symbol body this gives 55 edges.
- A nested tail `(x (x (…)?)?)?` lets each copy go only to the next copy or stop. The same repeat then gives 10 edges.

Lark expands below a threshold, one alternative per count, with `[rule] * n for n in range(mn, mx + 1)` ([Lark source](https://github.com/lark-parser/lark/blob/master/lark/load_grammar.py)). Inference: `~0..49` then holds 1,225 symbol occurrences. At 50 copies and above, Lark splits the repeat into sub-rules of small factors, after [pull request 949](https://github.com/lark-parser/lark/pull/949). Its author measured one rule of 211 copies at 67.36 units of build time, against 0.62 for the split form. He concluded: "The slowdown is not actually caused by a large number of states, but by large rules".

Counting automata keep one copy of the body and add a bounded counter. [Turoňová 2020](https://dx.doi.org/10.1145/3428286) translates regexes to counting automata and determinises them to counting-set automata. They report "the size of the produced CsAs does not depend on the repetition bounds used in the regex (while the size of the DFA is exponential to them)". [Gelade 2009](https://link.springer.com/chapter/10.1007/978-3-642-03816-7_32) extends Glushkov's method to counting automata. They found that "Weakly deterministic expressions with counting are exponentially more succinct and strictly more expressive than strongly deterministic ones".

The counter moves the cost from the picture to the trace. Inference: a trace state becomes a pair of position and counter value, so the frontier still grows with the bound. Slow matching stays a risk with counters. [Turoňová 2022](https://www.usenix.org/system/files/sec22-turonova.pdf) found that "Regexes with bounded repetition with higher bounds are potentially vulnerable to ReDoS attacks even for automata-based matchers". [Su 2024](https://www.usenix.org/system/files/usenixsecurity24-su-weihao.pdf) reports false negatives in their generator.

Limits that tools set on wide bounds:

- [RE2](https://github.com/google/re2/wiki/Syntax) rejects counts "above 1000".
- Java caps content-model nodes with `jdk.xml.maxOccurLimit`, default 5000 since 7u45 ([JAXP security guide](https://docs.oracle.com/en/java/javase/26/security/java-api-xml-processing-jaxp-security-guide.html)). The limit applies to "maxOccurs occurrence indicators with values other than 'unbounded'". A value of zero or less disables the limit.
- [Lark](https://lark-parser.readthedocs.io/en/latest/grammar.html) documents `item ~ n..m` as "not recommended for wide ranges, due to performance issues". The code splits at `REPEAT_BREAK_THRESHOLD = 50`.
- llama.cpp fails with `number of repetitions exceeds sane defaults`. A 2026 pull request to make the cap configurable is still open, and a maintainer wrote "we established we don't really want to do this" ([PR 29004](https://github.com/ggml-org/llama.cpp/pull/29004)).
- [XGrammar](https://xgrammar.mlc.ai/docs/latest/defining_structures/lark_grammar.html) states that large bounds such as `{0,10000}` "do not expand the automaton". The page does not describe the mechanism.

Fallback to beat: nested expansion with a warning above a copy count such as Lark's 50. Evidence for departure: real move grammars that need bounds above the cap, or traces where users lose track of copies.

#### Terminal marking and misspelt rule names

Bison rejects an undefined bare name and suggests the nearest rule: `symbol 'lis' is used, but is not defined as a token and has no rules; did you mean 'list'?` ([Bison NEWS](https://github.com/akimd/bison/blob/master/NEWS)). Typos in string aliases pass unless `-Wdangling-alias` is set. ANTLR defines a token implicitly for an unknown name and warns "implicit definition of token Token in parser" ([ANTLR error types](https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html)). A 2013 issue asked for an error, because the generated parser fails only when the rule runs ([issue 269](https://github.com/antlr/antlr4/issues/269)). It is still a warning. [Lark](https://lark-parser.readthedocs.io/en/latest/grammar.html) separates the two kinds by case: "Names of rules are always in lowercase, while names of terminals are always in uppercase."

Inference: with quoted terminals, as in the request, each bare name must resolve to a rule. A declared move vocabulary then also catches misspelt terminals, like Lark's `%declare`. Edit distance against both name lists gives the "did you mean" text. Fallback to beat: quoted terminals plus an undefined-name error. Evidence for departure: users who find quotes on every move label a burden.

### Q3. Semantics

#### The regularity badge

The request labels a rule "Right-Regular (Pure FSM)" when it "Contains no self-embedding or non-tail recursion (or no non-terminal references)". It labels a rule "Context-Free (RTN Sub-Grammar)" when it "Invokes other non-terminals or contains recursive self-embedding". Four grammars show the faults:

```
C = 'x' D 'y' ;   D = 'z' ;      (calls a rule, language regular)
E = E 'x' | 'y' ;                (non-tail recursion, language y x*)
A = 'a' B 'c' ;   B = A | 'b' ;  (no self-call in A, yet self-embedding)
S = 'a' S 'a' | 'a' ;            (self-embedding, language still regular)
```

[Chomsky 1959](https://www.sciencedirect.com/science/article/pii/S0019995859903626) proved that a language is not finite-state exactly when all its grammars are self-embedding. A self-embedding grammar can still generate a regular language, as rule S shows. Regularity of a context-free language is undecidable ([Gécseg and Steinby](https://arxiv.org/pdf/1509.06233), [Greibach's theorem](https://en.wikipedia.org/wiki/Greibach%27s_theorem)).

[Nederhof 2000](https://arxiv.org/pdf/cs/9910022) gives the computation. Group the mutually recursive rules into sets. A set is left-generating when some call inside it has symbols before it, and right-generating when some call has symbols after it. A set that is both is "self". In his words: "That recursive(N_i) = self, for some i, is a sufficient and necessary condition for the grammar to be self-embedding." Tarjan's algorithm finds the sets in linear time.

Inference, for EBNF bodies: test each path through the rule automaton that reaches a call. The call has symbols before it when such a path passes a symbol that can derive a non-empty sequence. The badge then belongs to the set, not to the rule text alone. [Mohri and Nederhof 2001](https://www.dfki.de/en/web/research/projects-and-publications/publication/3307/) approximate self-embedding sets by a superset automaton, which is a later option.

Fallback to beat: the call-graph test, with badges "no self-embedding" and "self-embedding". Evidence for departure: none expected, because the test is exact for the property it names.

#### One automaton for a stack-free set

Nederhof's test above also decides which sets need no stack. A set of mutually recursive rules needs none exactly when the test does not class it as "self". Right-linear recursion is one sufficient case. [JSGF](https://www.w3.org/TR/jsgf/) allows only that case: "Recognizers supporting the JSpeech Grammar Format allow right recursion." It rejects "Other forms of recursion (left recursion, embedded recursion)". A footnote adds: "Technically speaking, the JSpeech Grammar Format defines what is formally called a regular grammar."

Tools test narrower conditions:

- OpenFst `Replace` accepts no recursion at all: "If there are cyclic dependencies among the replacement rules, the resulting FST does not have a finite expansion and an exception will be raised" ([Pynini docs](https://github.com/google-research/opengrm/blob/main/opengrm/pynini/docs/index.md)).
- The OpenFst pushdown extension finds each "left- (right-) linear dependency SCC". For "regular or, more generally, strongly regular grammars", it gives a "bounded-stack (expandable as an FST) result" ([PDT replace.h](https://www.openfst.org/doxygen/fst/html/extensions_2pdt_2replace_8h_source.html)).
- RELAX NG expands each reference to a pattern that is not an element, and states: "This must not result in a loop" ([RELAX NG §4.19](https://relaxng.org/spec-20011203.html)).
- XML Schema states: "Circular groups are disallowed" ([XML Schema 1.0 §3.8.6](https://www.w3.org/TR/xmlschema-1/)). Recursion through element declarations stays legal ([xmlschema-dev 2002](https://lists.w3.org/Archives/Public/xmlschema-dev/2002May/0030.html)).

Inference: the two schema languages are a precedent for the mixed presentation. Groups expand in place, and elements nest like call boxes.

Two rewrites build the automaton for a set:

- A call to a non-recursive rule expands into a copy. OpenFst redirects the call arc into a copy of the called automaton and adds return arcs from its final states ([Pynini docs](https://github.com/google-research/opengrm/blob/main/opengrm/pynini/docs/index.md)). [Bannister, Brown and Eppstein 2015](https://ar5iv.labs.arxiv.org/html/1509.00818) call this "nonterminal nesting". They cap it "to keep the size of each individual st-digraph to a human-readable level".
- A tail call becomes a jump. Their "loop back" step redirects the call edge to the start, and "we change its label from A to ϵ". The [Railroad Diagram Generator](https://www.bottlecaps.de/rr/ui) replaces direct recursion by repetition unless the user disables the option. OpenFst merges the start states of a left-linear set, or the final states of a right-linear set, into one state.

Copies multiply along call chains. Inference: a chain of d rules, each calling the next twice, gives `2^d` copies. [Nederhof 2000](https://arxiv.org/pdf/cs/9910022) saw "sharp increases in the sizes of the automata for small increases in the size of the grammar".

These rewrites drop the rule boundaries that a parse tree needs. Loop back turns the call into an empty edge. Bannister, Brown and Eppstein promise only "heuristics that modify the grammar but preserve the language". Their JSON number graph "is not series-parallel, and therefore could not be represented by EBNF". The ooRexx documentation tells authors to disable recursion elimination, factoring and inlining, and gives no reason ([ooRexx wiki](https://sourceforge.net/p/oorexx/wiki/how-to-change-oorexx-docs/)).

Two mechanisms keep the boundaries. OpenFst can keep labels on call and return arcs, through `call_label_type` and `return_label_type` ([replace.h](https://www.openfst.org/doxygen/fst/html/replace_8h_source.html)). [Laurikari 2000](https://research.aalto.fi/en/publications/nfas-with-tagged-transitions-their-conversion-to-deterministic-au/) adds tags to transitions, which record where in the input a run last took each tagged transition.

Inference: tag each edge that enters or leaves a copy with the rule name. The tracer opens a tree node at an entry tag and closes it at an exit tag. In a right-linear set, each nested node closes where its caller closes, so entry tags alone determine the tree. The OpenFst pushdown code makes the same choice: "the other parenthesis is omitted (appropriate for the strongly-regular case)". A position automaton has no empty edges, so its tags sit on symbol edges.

Inference, for one tracer: an Earley item names a rule, a position and an origin, but not a call site. The callers that wait at that origin give the call sites, so one item can highlight several copies.

Fallback to beat: Earley over per-rule position automata as the only tracer, with the tagged set automaton as its drawing. Evidence for departure: copy counts above a readable size, which favour call boxes, or call-site lookups too slow for live stepping.

#### States and edges of the position automaton

[Brüggemann-Klein 1993](https://www.sciencedirect.com/science/article/pii/0304397593902874) showed that "the Glushkov automaton can be constructed in a time quadratic in the size of the expression, and that this is worst-case optimal". The automaton has one state per symbol occurrence plus a start state, and no empty moves. So "n + 1 states" in the request holds when n counts occurrences, not distinct symbols.

Repetition and optionality add edges, not states. A star links each last position back to each first position. An optional part lets edges skip over it. Inference: `(m1|…|mk)*` has k² + k edges, so ten moves give 110 edges. Its minimal automaton has one state with ten loops.

Sources disagree on typical size. [Nicaud 2009](https://link.springer.com/chapter/10.1007/978-3-642-00982-2_53) found that "in average, the number of transitions is linear" in a uniform model. [Nicaud, Pivoteau and Razet 2010](http://www-igm.univ-mlv.fr/~nicaud/articles/fsttcs10.pdf) found quadratic averages under a tree-shaped distribution. [Broda 2012](https://www.worldscientific.com/doi/abs/10.1142/S0129054112400400) reports about two transitions per state, and a partial-derivative automaton about half the size.

Smaller automata exist, but each merges positions. The follow automaton and the partial-derivative automaton are quotients of the position automaton ([Ilie and Yu, via Allauzen and Mohri](https://cs.nyu.edu/~mohri/pub/glush.pdf), [Champarnaud and Ziadi 2002](https://www.semanticscholar.org/paper/Canonical-derivatives,-partial-derivatives-and-Champarnaud-Ziadi/a73a90b8bc1b8f034bf08163e8b5265ac4fc1455)). [Hromkovič 2001](https://www.sciencedirect.com/science/article/pii/S0022000001917489) builds automata with O(n log² n) transitions and proves a lower bound of Ω(n log n). [Caron and Ziadi 2000](https://www.sciencedirect.com/science/article/pii/S030439759700296X) characterise position automata by graph properties, which allows a round trip back to an expression. A later paper reports an error in that characterisation and gives a revised version ([arXiv 0907.4296](https://arxiv.org/pdf/0907.4296)).

Which construction people read best: [Blackwell](https://www.cl.cam.ac.uk/~afb21/publications/yourwish.pdf) found a "clear advantage for graphical notations in typical end-user tasks" over regular expressions. No study compares Thompson, position and minimal-automaton pictures.

#### Determinisation and minimisation

A deterministic state is a set of positions, so a highlight can no longer name one occurrence. [Borsotti 2021](https://link.springer.com/article/10.1007/s00236-020-00366-7) keys "each state of the FA ... to a set of RE positions". Inference: a tree node needs one occurrence, so a trace on a merged state must recover it from the positions.

A rule whose position automaton is already deterministic needs no subset step. That holds exactly when the expression is one-unambiguous ([Brüggemann-Klein and Wood 1998](https://people.cs.umu.se/drewes/CIAA-talks/Miklarz.pdf)). XML 1.0 makes this a rule for content models: "for compatibility, it is an error if the content model allows an element to match more than one occurrence of an element type in the content model" ([XML 1.0](https://www.w3.org/TR/xml/)). The check is quadratic ([Martens](https://homepages.inf.ed.ac.uk/libkin/dbtheory/mnt.pdf)) or linear ([Xu 2020](https://csse.szu.edu.cn/staff/xuzw/paper/2020_scp_inclusion.pdf)). Some regular languages have no one-unambiguous expression, so the check informs and must not forbid.

[Nederhof 2000](https://arxiv.org/pdf/cs/9910022) found that minimising each sub-automaton before substitution avoids much growth. He still saw "sharp increases in the sizes of the automata for small increases in the size of the grammar".

Fallback to beat: trace the position automaton and show the minimal automaton as a view whose nodes list their merged occurrences. Evidence for departure: a user study where position diagrams of real rules prove unreadable.

#### From a run to a parse tree

[Book, Even, Greibach and Ott 1971](https://www.semanticscholar.org/paper/Ambiguity-in-Graphs-and-Expressions-Book-Even/526ca04dc8ba3a02d99735174b0a7b797bd9bbae) call an expression unambiguous "if every tape in the event can be generated from the expression in one way only". Their flow graph preserves this, so their test counts paths. Inference: their flow graph corresponds to the position automaton.

Path ambiguity misses tree ambiguity. `(x*)*` has one position, so `x x` has one path. It has two trees: one outer iteration of two symbols, or two iterations of one.

[Brüggemann-Klein 1993](https://www.researchgate.net/publication/342318085_Regular_Expressions_into_Finite_Automata) proves that strong unambiguity holds exactly when weak unambiguity, star normal form and epsilon normal form hold. Her example `(a* + b*)*` is weakly but not strongly unambiguous. Star normal form is reachable in linear time. [Brabrand and Thomsen 2010](https://dl.acm.org/doi/10.1145/1836089.1836120) give "a characterization of ambiguity of regular expressions that leads to a sound and complete static analysis" on the expression itself.

Empty iterations make the count infinite. [Sulzmann and Lu](https://www.worldscientific.com/doi/10.1142/S0129054117400068) restate the rule: an expression is problematic when it contains `s*` and s accepts the empty sequence. For other expressions, Frisch and Cardelli prove that "the set of distinct parse trees which share the same underlying word is always finite". Inference: the rule check flags each repeat whose body is nullable, and the tracer forbids empty iterations.

Trees from a position run: [Okui and Suzuki 2010](https://link.springer.com/chapter/10.1007/978-3-642-18098-9_25) pick the leftmost-longest tree on position automata without backtracking. The [BSP tool](https://www.researchgate.net/publication/300645845_BSP_A_Parsing_Tool_for_Ambiguous_Regular_Expressions) offers greedy and POSIX trees. Inference: in an RTN run, call and return steps bracket the rule nodes of the tree.

#### Parsers that build forests over EBNF

[Scott and Johnstone 2018](https://www.sciencedirect.com/science/article/pii/S0167642318302375) extend GLL to EBNF with "a parse tree representation of derivations which reflects the EBNF structure of the grammar, is worst case cubic size, and captures all derivations in the case of ambiguity". They warn: "Particular care is need in the case of closures with nullable bodies". [Breveglieri, Crespi Reghizzi and Morzenti](https://arxiv.org/pdf/1309.7584) run Earley over a net of one automaton per nonterminal. [Borsotti 2022](https://www.sciencedirect.com/science/article/abs/pii/S2590118422000739) builds the finest trees for EBNF with GLR. Their nets use deterministic machines ([ICTCS 2020](https://ceur-ws.org/Vol-2756/paper_17.pdf)).

Do parser states match diagram nodes? Inference: an EBNF slot marks a dot in the rule text, and a position state marks the last symbol read. One position state groups the slots that share that last symbol. So items over position automata map one-to-one onto diagram nodes, while items over deterministic machines map onto groups.

Earley trees need care. [Scott 2008](https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044): "Earley gave an outline of a method for turning his recognisers into parsers, but it turns out that this method is incorrect." Her version builds a shared forest in cubic time.

#### Tracing stepwise, and many trees

[Marpa progress reports](https://github.com/jeffreykegler/Marpa--R2/blob/master/cpan/pod/Progress.pod) list the dotted rules at a location, tagged predicted, partial or complete, with origins. The manual calls it "Marpa's most powerful tool for debugging grammars". A line reads `R11:2 @2-4 L1c3-13 expression -> expression '+' . expression`. The [Grammar Tool Box](https://www.researchgate.net/publication/222194445_The_Grammar_Tool_Box_A_Case_Study_Comparing_GLR_Parsing_Algorithms) shipped PAT, which "shows animations of the data structures while they are being built".

Tools that return several trees:

- [Lark](https://lark-parser.readthedocs.io/en/stable/recipes.html) offers `ambiguity='explicit'` with `_ambig` nodes, not recommended "for highly ambiguous grammars, or very long inputs". Expanding all trees risks "exponential explosion". [Issue 455](https://github.com/lark-parser/lark/issues/455) reports a missing `_ambig` node.
- [nearley](https://nearley.js.org/docs/parser) returns an array: "nearley gives you all the parsings (in a deterministic order!)". [Issue 292](https://github.com/kach/nearley/issues/292) reports two identical trees.
- [Marpa](https://metacpan.org/pod/Marpa::R2::Scanless::R) offers `max_parses`, which "causes a fatal error when that number of parse results is exceeded", plus `ambiguous()` and `ambiguity_metric()`.

Inference: a pass over the shared forest counts trees without listing them, and a cycle means infinitely many.

Known boundaries cut trees. [Pereira and Schabes 1992](https://aclanthology.org/P92-1017/) use bracketed constituents and report "better time complexity than the original one when sufficient bracketing is provided". [Schabes 1993](https://aclanthology.org/E93-1040.pdf) reports linear time for full bracketing. Inference: a known episode boundary drops every chart item that crosses it.

Fallback to beat: Earley over position automata, a forest count, and a capped list of trees. Evidence for departure: grammars where forests stay small, which would allow a plain path search.

#### Next symbols across live readings

The grammar view lists the symbols that can come next. In a position automaton, these are the labels of the positions in the follow set of the current position. Homogeneity makes this direct, because each position carries one symbol. [Pingali and Bilardi 2012](https://apps.cs.utexas.edu/apps/sites/default/files/tech_reports/TR-2102.pdf) separate the static follow set of a node, a union over all paths, from the look-ahead of one path. Inference: the grammar view needs the second kind, because a static follow set at a rule end offers symbols that no live caller accepts.

In an Earley chart, the next symbols are the terminals after the dot in the items of the newest Earley set. Earley-based constrained decoders build their token masks from these terminals ([arXiv 2506.01151](https://arxiv.org/pdf/2506.01151)). The Rust parser [llguidance](https://github.com/guidance-ai/llguidance) reports about 50 μs per mask for a vocabulary of 128,000 tokens ([llg-go-brrr](https://guidance-ai.github.io/llguidance/llg-go-brrr)). Marpa's `terminals_expected()` "returns a list of the symbols acceptable at the current location" ([Kegler 2018](http://jeffreykegler.github.io/Ocean-of-Awareness-blog/individual/2018/05/combinator2.html)). The nearley error text names each expected token with its rule chain, as in `main → ● %ws AS %ws` ([example](https://ull-esit-pl-2122.github.io/practicas/egg-oop-parser.html)).

Parsers with explicit stacks take a union. In XGrammar, "the token masks for every stack is merged into a final token mask by finding the union of the accepted tokens" ([arXiv 2411.15100](https://arxiv.org/pdf/2411.15100)). llama.cpp keeps one stack per reading, and the nested repeat `("a"*)*` overflowed its stack ([issue 18988](https://github.com/ggml-org/llama.cpp/issues/18988)). [PR 18993](https://github.com/ggml-org/llama.cpp/pull/18993) adds cycle detection and a depth cap.

The base GLL algorithm has no error report of its own ([Scott and Johnstone](https://dotat.at/tmp/gll.pdf)). A [rust-lang/gll issue](https://github.com/rust-lang/gll/issues/16) proposes to "keep a buffer of 'attempted input matches' at the 'most advanced input location'". The parser would report these as an "expected one of ..." error. Inference: the slots of the GLL descriptors at the current position give the same set.

Nullable rules hide next symbols. The antlr4-c3 completion engine "does not return all possible tokens when it encounters rules that do not necessarily consume tokens" ([issue 23](https://github.com/mike-lischke/antlr4-c3/issues/23)). Inference: the collection must pass through nullable rules, and through rule ends to the callers.

Fallback to beat: the union of the terminals after the dot in the newest Earley set, each with its rule chain. Evidence for departure: so many live readings that the list stops being readable.

#### Partial trees for a prefix

[Pingali and Bilardi 2012](https://apps.cs.utexas.edu/apps/sites/default/files/tech_reports/TR-2102.pdf) state the property that a partial tree needs. Their CR-paths "are always prefixes of complete balanced paths, and if a prefix u of w has been read up to that point, all these paths generate u". [Lang 1988](https://dl.acm.org/doi/10.3115/991635.991710) parses "sentences with unknown parts of unknown length". His parser "produces in finite form all possible parses (often infinite in number) that could account for the missing parts". Inference: a partial tree is a set of complete subtrees under open rule nodes. The items of the newest Earley set supply the open nodes, and their origins give the left edges.

Each step can extend the tree in place. [Scott 2008](https://pure.royalholloway.ac.uk/en/publications/sppf-style-parsing-from-earley-recognisers/) gives two Earley parsers that return a shared forest in cubic time. A fragment of the full text says that the second builds the forest while the Earley sets grow ([ResearchGate](https://www.researchgate.net/publication/220367479_SPPF-Style_Parsing_From_Earley_Recognisers)). Marpa's events are "left-eidetic but right-blind" ([Event.pod](https://metacpan.org/dist/Marpa-R2/view/pod/Event.pod)). XGrammar keeps a persistent stack "to enable rapid state branching and rollback" ([arXiv 2411.15100](https://arxiv.org/pdf/2411.15100)). Inference: a forward step appends one Earley set and its forest nodes, and a backward step drops them, so step back needs no snapshots.

Partial trees also serve error reports. A [rust-lang/gll issue](https://github.com/rust-lang/gll/issues/16) proposes "spines" of "vertebrae", where each vertebra is an unfinished rule with its finished parts. The issue would build these spines while parsing and keep only the rightmost ones. It also suggests a typed partial node per rule, with each field marked `Complete`, `Partial` or `NotStarted`. The issue opened in August 2018 and is still open.

Fallback to beat: build the forest while the Earley sets grow. Draw the open items of the newest set as the right edge of the tree. Evidence for departure: forests too large to update within one step in the browser.

#### Loop passes, including recursion

Validators count passes with one counter per looping state. [Xerces2-J](https://github.com/apache/xerces2-j/blob/trunk/src/org/apache/xerces/impl/xs/models/XSDFACM.java) keeps counters for "looping states which use counters to check minOccurs/maxOccurs". A counter grows while the automaton loops on one state, and it resets when the automaton enters a new state. At the end, a count below the minimum means "not enough loops on the current state to be considered final".

The error for the maximum reads "'optional' can occur a maximum of '2' times in the current sequence. This limit was exceeded." ([lemminx issue 368](https://github.com/eclipse-lemminx/lemminx/issues/368)). Where readings differ, the counting-set automata of [Turoňová 2020](https://dx.doi.org/10.1145/3428286) keep a set of counter values per state.

Most regex engines keep only the last pass: "All engines return the last value captured. With the exception of the .NET engine, all intermediate values are lost" ([RexEgg](https://www.rexegg.com/regex-capture.php)). The .NET `Captures` property holds "all the substrings captured by the group" ([.NET docs](https://learn.microsoft.com/en-ca/dotnet/api/system.text.regularexpressions.group.captures?view=net-8.0)). Inference: in an Earley tracer, the pass number is the count of completed iterations under the open repeat node. The view shows "pass 2 of 2..4" from that count and the bounds in the rule text, for each reading.

Right recursion is a loop in another form. "Right-recursive grammars are analogous to tail-recursion, and are equivalent to regular expressions (and consequently don't need a stack)" ([Steffen](https://martinsteffen.github.io/compilerconstruction/leftandtailrecusion/)). JSGF says that "Any right recursive rule can be re-written using the Kleene star" or the plus operator ([JSGF](https://www.w3.org/TR/jsgf/)). The section on stack-free sets shows two tools that redraw a final self-reference as a loop.

Marpa's definition finds the tail calls. A rule is right-recursive when its rightmost non-nulling symbol derives back to the rule's own left side, directly or through other rules ([Kegler 2019](https://arxiv.org/html/1910.08129v3)). Marpa applies Leo's memoisation only to such rules, and this keeps long right recursions linear. Its progress reports must unfold each chain, "processing that can be quadratic in the length of the recursion" ([Progress.pod](https://github.com/jeffreykegler/Marpa--R2/blob/master/cpan/pod/Progress.pod)). Inference: each tail call into the loop rule starts a pass. A tracer that records one pass marker per tail call does not need to unfold chains.

A cycle through several rules is harder. Marpa's definition covers indirect chains, and its manual shows an indirect right recursion through three rules. OpenFst's `LEFT_SR` parser "Uses epsilons rather than parentheses labels for any non-terminal instances within a left- (right-) linear dependency SCC" ([pdt replace.h](https://www.openfst.org/doxygen/fst/html/extensions_2pdt_2replace_8h_source.html)). Inference: such a cycle forms one loop, but outside calls can enter it at several rules. Control-flow analysis meets the same problem in irreducible graphs, whose loops have several entries ([Ramalingam 2002](https://dl.acm.org/doi/pdf/10.1145/570886.570887)). Inference: the tracer can take the rule where a reading entered the cycle as the start of each pass.

Left recursion is a loop too. Bison advises left recursion for lists, "because it can parse a sequence of any number of elements with bounded stack space" ([Bison manual](https://www.gnu.org/software/bison/manual/html_node/Recursion.html)). Lark rewrites `c+` as the left-recursive helper `_c : _c c | c` and inlines it, so passes reach the tree as siblings ([load_grammar.py](https://github.com/lark-parser/lark/blob/master/lark/load_grammar.py), [tree construction](https://lark-parser.readthedocs.io/en/latest/tree_construction.html)). ANTLR rewrites direct left recursion into a loop, and makes "the current context the child of the incoming localctx" ([Parser API](https://www.antlr.org/api/Java/org/antlr/v4/runtime/Parser.html)). It rejects mutual left recursion with error 119, "The following sets of rules are mutually left-recursive" ([ErrorType](https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html)). Inference: in an Earley chart, a left-recursive pass ends where the rule completes and an item of the same rule and origin moves over it.

Tools mark passes in three ways. Lark flattens passes into siblings, ANTLR nests them left-deep, and most regex engines keep the last pass only. [Scott and Johnstone 2018](https://www.sciencedirect.com/science/article/pii/S0167642318302375) build forests that reflect "the EBNF structure of the grammar", with closures as iteration. Laurikari's tags record "the positions in the input string for the last uses of selected transitions" ([Laurikari 2000](https://research.aalto.fi/en/publications/nfas-with-tagged-transitions-their-conversion-to-deterministic-au/)). Inference: give each loop one node with one child per pass, whether the loop comes from a repeat, a tail call or left recursion. Each pass child can keep its rule name, so the rule view still shows the recursion as written.

Fallback to beat: one loop node per repeat, tail-call cycle or left-recursive rule, with one child per pass. The pass count comes from the tree. Evidence for departure: real grammars whose cycles have several entries, where readers cannot tell where a pass starts.

#### Recovery for a failed sequence

[Aho and Peterson 1972](https://www.semanticscholar.org/paper/A-Minimum-Distance-Error-Correcting-Parser-for-Aho-Peterson/0caae1d9d751f409067b8bcb2266b06f171e5e32): "We present an algorithm that will parse any input string to completion finding the fewest possible number of errors." Errors are insertions, deletions and replacements, in time "proportional to the cube of the length of the input". [Saha](https://arxiv.org/html/1411.7315) gives O(|G|² n³). [Seq2Parse](https://pg.ucsd.edu/publications/Seq2Parse-neurosymbolic-parse-error-repair_OOPSLA-2022.pdf) tested Earley parsers with error rules. It found them "overwhelmed by the many error-correction rules that are not relevant to the particular program".

Faster methods exist ([Rajasekaran and Nicolae 2016](https://link.springer.com/chapter/10.1007/978-3-319-30000-9_41)). For regular languages, see [Wagner 1974](https://dl.acm.org/doi/10.1145/360980.360995), "Order-n correction for regular languages".

Other repair strategies:

- CPCT+ "reports the complete set of minimum cost repair sequences for a given location" ([Diekmann and Tratt](https://arxiv.org/abs/1804.07133)). It repaired "98.37% of files" of "200,000 real-world syntactically invalid Java programs" "within a timeout of 0.5s". lrpar prints `Repair sequences found: 1: Delete + 2: Insert INT` ([grmtools book](https://softdevteam.github.io/grmtools/master/book/errorrecovery.html)).
- GLR* finds "the maximal subset of the input that can be parsed" ([Lavie and Tomita 1993](https://aclanthology.org/1993.iwpt-1.12.pdf)). Parsable sentences rose from 51.7% to 99.2% ([Lavie thesis](https://www.cs.cmu.edu/~alavie/papers/thesis.pdf)). In 23 of 58 rescued sentences the parse was bad "because essential words were skipped" ([cmp-lg/9405023](https://arxiv.org/html/cmp-lg/9405023)).
- ANTLR deletes or inserts one token and reports `extraneous input X expecting Y` or `missing X at Y` ([DefaultErrorStrategy](https://www.antlr.org/api/Java/org/antlr/v4/runtime/DefaultErrorStrategy.html)).
- Tree-sitter marks ERROR and MISSING nodes without a message ([issue 833](https://github.com/tree-sitter/tree-sitter/issues/833)).

Inference: show each deleted symbol as unexplained and each inserted symbol as a ghost node. List the equal-cost repairs as CPCT+ does. Browser cost, also inference: a regular rule needs about 300 × 10,000 = 3 million table cells for 300 symbols. The cubic method needs 27 million steps times a grammar factor, so it belongs in a worker.

Fallback to beat: fewest edits on an Earley chart, with a cap on listed repairs. Evidence for departure: measured times above a second on real grammars.

### Q4. Layout and interaction

#### Stepping and history

JFLAP shows NFA runs as a list of configurations, with `Freeze`, `Thaw` and `Remove` controls ([JFLAP simulator](https://research.cs.vt.edu/AVresearch/openalgoviz/JFLAP/trunk/DOCS/gui.sim.SimulatorPane.html)). Its book warns "After one Step, you will see six configurations!" ([JFLAP book](https://www.jflap.org/jflapbook/jflapbook2006.pdf)). The exercises freeze and delete configurations because the count "can become quite large" ([Linz and Rodger](https://turing.plymouth.edu/~zshen/Webfiles/notes/CS378/JFLAPActivities.pdf)).

Stepping back, inference: an Earley chart keeps every earlier set, so a step back shows the previous set without recomputation. A frontier over 1,000 positions fits in a 125-byte bit set, so 300 steps cost 37.5 KB. Recomputation from the start is also cheap at this size. History on a junction drawing needs the chosen tree, because several empty paths can join two positions.

#### Two drawings of one rule

[Allauzen and Mohri 2006](https://cs.nyu.edu/~mohri/pub/glush.pdf) relate the two constructions. Thompson's automaton has states and transitions linear in the length of the expression. Glushkov's automaton has n + 1 states but up to n² transitions, and it equals Thompson's automaton with the empty transitions eliminated. In Thompson's automaton each position is one symbol edge, and only these edges carry symbols. No automaton without empty edges stays linear for every expression. [Hromkovič 2001](https://www.sciencedirect.com/science/article/pii/S0022000001917489) proves a worst-case lower bound of Ω(n log n) transitions.

Inference: the Glushkov state of position i is the end of its symbol edge in Thompson's automaton. So tracer state on positions maps one-to-one onto a junction drawing. A Glushkov edge from i to j maps onto an empty path from the end of i to the start of j. In `(x*)*` two such paths lead from x back to x, one through each star. They are the two trees of `x x` from the section on parse trees. A junction drawing thus shows tree ambiguity that a position drawing hides.

Junctions also carry many edges with little ink. [Dickerson, Eppstein, Goodrich and Meng](https://arxiv.org/pdf/cs/0212046) merge edges into shared tracks, which draw any complete bipartite graph without crossings. A star over a choice of k moves links k last positions to k first positions. Inference: one junction draws those k² edges as 2k track segments. In the syntax diagrams of [Bannister, Brown and Eppstein 2015](https://ar5iv.labs.arxiv.org/html/1509.00818), automaton states "become confluent junctions in the drawing".

Process-modelling guidelines agree ([7PMG](https://www.vdaalst.com/publications/p574.pdf)). Guideline G2 reads "Minimize the routing paths per element", because "The higher the degree of elements in the process model the harder it becomes to understand the model". G4 asks that each split connector match a join of the same type. G7 reads "Decompose a model with more than 50 elements", because above that size "the error probability tends to be higher than 50%". Inference: BPMN gateways are junctions, and Thompson's construction yields matched splits and joins. In a position drawing of a star over k moves, every node has k outgoing edges, against G2.

Debuggex ties a trace to its diagram. Moving the execution line shifts the cursor in the test string "and the corresponding one in the central diagram" ([Debuggex review](https://scraping.pro/debuggex-review-get-your-regex-to-play-visual/)). A cursor in the diagram marks "all possible positions in our match where we are potentially expecting this symbol".

Fallback to beat: junction drawings in Thompson's style, with tracer state on symbol boxes. Following G7, any expansion above 50 elements becomes a call box. Evidence for departure: a reader test where position drawings of real move rules read faster.

### Q5. Scope

The discourse survey states: "This report found no move nested inside an instance of the same move" ([discourse-grammars.md](discourse-grammars.md)). Without self-embedding, each grammar describes a regular language ([Chomsky 1959](https://www.sciencedirect.com/science/article/pii/S0019995859903626)).

The call structure still matters. Inference: a tree needs rule nodes, and a flat automaton keeps them only through tags. Left and right recursion also stay legal without self-embedding. A naive top-down run loops forever on left recursion, but Earley does not. Earley's original algorithm takes quadratic time on right recursion, and Leo's method of 1991 makes it linear for LR-regular grammars ([Kegler 2010](https://blogs.perl.org/users/jeffrey_kegler/2010/06/marpa-is-now-on-for-right-recursions.html), [Marpa::Algorithm](https://metacpan.org/pod/release/JKEGL/Marpa-0.102000/lib/Marpa/Doc/Algorithm.pod)). Inference: at 300 symbols the quadratic cost is near 90,000 steps, so cost alone does not force flattening.

Defer: graph-structured stacks, GLR, approximation of self-embedding sets, counting-automaton views, and full enumeration of trees. Features that tools never delivered: tree-sitter error messages, where a maintainer wrote "we haven't yet built out functionality for error messages" ([issue 833](https://github.com/tree-sitter/tree-sitter/issues/833)). JFLAP 8 stayed in beta ([JFLAP site](https://www.jflap.org/jflaptmp/)).

Fallback to beat: Earley over per-rule position automata, with no other stack, and tagged set automata as drawings. Evidence for departure: a traced corpus with grammars that Earley parses too slowly in the browser.

### Q6. Test seams

The Rust regex crate keeps one corpus: "All regex tests are specified in TOML files" ([regex internals](https://burntsushi.net/regex-internals/)). One unit test per engine configuration runs every applicable case. The older set-up was "terrible macro soup" ([issue 656](https://github.com/rust-lang/regex/issues/656)). Inference: the Rust core can run one KDL case corpus natively and in WebAssembly.

Differential tests against a reference recogniser:

- [Davis 2019](https://fservant.github.io/papers/Davis_Michael_Coghlan_Servant_Lee_ESECFSE19.pdf) found that 15% of regexes behave differently across eight languages, and concluded "developers' belief in a regex lingua franca is understandable but unfounded".
- [Csmith](https://users.cs.utah.edu/~regehr/papers/pldi11-preprint.pdf) found "more than 325 previously unknown bugs". It avoids undefined behaviour "that would destroy its ability to automatically find wrong-code bugs".
- Reference algorithms can be wrong, as Earley's tree method was ([Scott 2008](https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044)).
- [FAdo](https://pypi.org/project/FAdo/) builds position, follow and partial-derivative automata and compares languages, so it can serve as an external oracle.
- Inference: on stack-free sets, the tagged set automaton and the Earley tracer must give the same trees. They form a differential pair inside the tool.

Random generation of sentences:

- [Purdom 1972](https://link.springer.com/article/10.1007/BF01932308) gives "a small set of short sentences" that uses each production at least once. [Malloy and Power](https://mural.maynoothuniversity.ie/id/eprint/6434/) restated it "so that it is obvious where to proceed at each step".
- A naive generator gets stuck, and a capped one produces "an infinite addition of parentheses" ([Fuzzing Book](https://www.fuzzingbook.org/html/GrammarFuzzer.html)). Their solution gives recursion an infinite cost when closing a derivation.
- In Hypothesis, "any undefined terminals from the use of %declare will result in generation errors" ([strategies](https://hypothesis.readthedocs.io/en/latest/reference/strategies.html)).

Properties to test, by inference: each rule automaton has n + 1 states, and all edges into a node carry one symbol. Position and minimal automata accept the same language, and KDL output reads back unchanged. Mutate each generated sentence by one edit, and check that recovery finds a cost of at most one. Fallback to beat: a shared case corpus plus Purdom coverage and cost-bounded generation. Evidence for departure: none.

## 3. Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance | Walk-away result |
|---|---|---|---|---|---|---|
| JFLAP | https://www.jflap.org/ | Teaching simulator for automata | Custom no-fee licence | 7.1, 27 July 2018 | Version 8 stayed in beta | Saved files remain, format not checked |
| FAdo | https://fado.dcc.fc.up.pt/ | Python automata conversions | GPL-3.0-or-later | 2.2.0, 1 April 2024 | No release for 12 months | Expressions stay plain text |
| llguidance | https://github.com/guidance-ai/llguidance | Rust Earley parser for constrained generation | MIT | v1.0.0 announced 23 June 2025, release tags not shown | 1,599 commits on main, latest date not shown | Grammars stay Lark-like text |
| Xerces2-J | https://github.com/apache/xerces2-j | XML Schema validator with counting states | Apache-2.0 | Not checked | Not checked | Schemas stay plain XML |
| Lark | https://github.com/lark-parser/lark | Python Earley and LALR parser | MIT | 1.3.1, date not checked | Documentation dated May 2026 | Grammars stay EBNF text |
| nearley | https://nearley.js.org/ | JavaScript Earley parser | MIT | 2.20.1, about 2020 | No npm release for six years | Grammar text stays, actions are JavaScript |
| Marpa::R2 | https://metacpan.org/release/JKEGL/Marpa-R2-14.000000 | Perl Earley parser | LGPL-3.0-or-later | 14.000000, 3 August 2026 | Active | BNF text stays |
| grmtools | https://softdevteam.github.io/grmtools/ | Rust LR parsers with CPCT+ repair | Apache-2.0 or MIT | 0.14.2, 30 April 2026 | Active | Yacc grammar text stays |
| RE2 | https://github.com/google/re2 | Automata regex engine | Not checked | Not checked | Not checked | Patterns stay plain text |
| XGrammar | https://xgrammar.mlc.ai/ | Grammars for constrained decoding | Not checked | Documentation 0.2.6 | Not checked | Grammars stay text |
| Chipmunk | https://dx.doi.org/10.1145/3428286 | Counting-set automata matcher | Not checked | Not checked | Not checked | Paper remains |
| PAT in GTB | https://www.researchgate.net/publication/222194445_The_Grammar_Tool_Box_A_Case_Study_Comparing_GLR_Parsing_Algorithms | GLR animation applet | Not found | Not found | No download found | Fails, only papers remain |
| Hypothesis | https://hypothesis.readthedocs.io/ | Property tests, Lark generation | Not checked | Not checked | Active documentation | Tests stay plain code |
| OpenFst | https://www.openfst.org/ | Weighted automata library with Replace and a pushdown extension | Not checked | Documentation 1.8.5 | Not checked | Not checked |
| Railroad Diagram Generator | https://www.bottlecaps.de/rr/ui | EBNF to SVG syntax diagrams | Not checked | v2.6, 28 December 2025 | Active | EBNF text and SVG stay |
| Debuggex | https://www.debuggex.com/ | Regex diagram with a stepped trace | Closed source | Not checked | Not checked | Fails, because matching runs on its server |

## 4. Silences

- No study compares Thompson, position and minimal-automaton pictures for readers. Queries: "Blackwell SWYN regular expression notation", "regular expression visualisation user study".
- No automata tool examined describes step back by snapshot or by recomputation. XGrammar's persistent stack for rollback is the closest source. Query: "JFLAP NFA simulation step configurations freeze trace remove".
- I did not read Frisch and Cardelli's definition of non-problematic values, or the span rule of Pereira and Schabes.
- Aycock and Horspool's treatment of nullable rules in Earley parsing comes from memory, and the report does not rely on it.
- Wagner's complexity, Gelade's size bounds and the Sippu and Soisalon-Soininen definition sit behind paywalls or truncation.
- The llama.cpp cap value, the XGrammar mechanism and the release dates of Lark 1.3.1 and lrpar 0.15.0 were not found.
- No source reports how wide the bounds in real move grammars are.
- No source covers tests for diagram renderers.
- No source examined builds parse trees from a flattened set automaton. The closest searches covered Laurikari's tagged transitions and the OpenFst pushdown code.
- I read 7PMG only through search excerpts, because its PDF did not parse. The details of Bannister, Brown and Eppstein come from the ar5iv HTML version for the same reason.
- BPMN call activities and sub-processes were not surveyed. Process mining and conformance checking belong to another survey, as the coordinator asked.
- No examined tool shows right-recursive parse trees as loop passes. Query: "tail call grammar right recursion displayed as loop iteration parse tree flatten list nodes".
- The claim that Scott's second parser builds the forest during recognition comes from a fragment of the full text. Ramalingam 2002 comes from search excerpts only.
- The release dates of Xerces2-J and the release tags of llguidance were not found.

## 5. Sources

- https://arxiv.org/pdf/cs/9910022
- https://en.wikipedia.org/wiki/Greibach%27s_theorem
- https://arxiv.org/pdf/1509.06233
- https://www.sciencedirect.com/science/article/pii/S0019995859903626
- https://www.dfki.de/en/web/research/projects-and-publications/publication/3307/
- https://www.sciencedirect.com/science/article/pii/0304397593902874
- https://www.researchgate.net/publication/342318085_Regular_Expressions_into_Finite_Automata
- https://link.springer.com/chapter/10.1007/978-3-642-00982-2_53
- http://www-igm.univ-mlv.fr/~nicaud/articles/fsttcs10.pdf
- https://www.worldscientific.com/doi/abs/10.1142/S0129054112400400
- https://cs.nyu.edu/~mohri/pub/glush.pdf
- https://www.semanticscholar.org/paper/Canonical-derivatives,-partial-derivatives-and-Champarnaud-Ziadi/a73a90b8bc1b8f034bf08163e8b5265ac4fc1455
- https://www.sciencedirect.com/science/article/pii/S0022000001917489
- https://www.sciencedirect.com/science/article/pii/S030439759700296X
- https://arxiv.org/pdf/0907.4296
- https://www.cl.cam.ac.uk/~afb21/publications/yourwish.pdf
- https://link.springer.com/article/10.1007/s00236-020-00366-7
- https://people.cs.umu.se/drewes/CIAA-talks/Miklarz.pdf
- https://www.w3.org/TR/xml/
- https://homepages.inf.ed.ac.uk/libkin/dbtheory/mnt.pdf
- https://csse.szu.edu.cn/staff/xuzw/paper/2020_scp_inclusion.pdf
- https://www.semanticscholar.org/paper/Ambiguity-in-Graphs-and-Expressions-Book-Even/526ca04dc8ba3a02d99735174b0a7b797bd9bbae
- https://dl.acm.org/doi/10.1145/1836089.1836120
- https://www.worldscientific.com/doi/10.1142/S0129054117400068
- https://link.springer.com/chapter/10.1007/978-3-642-18098-9_25
- https://www.researchgate.net/publication/300645845_BSP_A_Parsing_Tool_for_Ambiguous_Regular_Expressions
- https://www.sciencedirect.com/science/article/pii/S0167642318302375
- https://arxiv.org/pdf/1309.7584
- https://www.sciencedirect.com/science/article/abs/pii/S2590118422000739
- https://ceur-ws.org/Vol-2756/paper_17.pdf
- https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044
- https://github.com/jeffreykegler/Marpa--R2/blob/master/cpan/pod/Progress.pod
- https://metacpan.org/pod/Marpa::R2::Scanless::R
- https://metacpan.org/release/JKEGL/Marpa-R2-14.000000
- https://www.researchgate.net/publication/222194445_The_Grammar_Tool_Box_A_Case_Study_Comparing_GLR_Parsing_Algorithms
- https://lark-parser.readthedocs.io/en/stable/recipes.html
- https://lark-parser.readthedocs.io/en/latest/grammar.html
- https://github.com/lark-parser/lark/blob/master/lark/load_grammar.py
- https://github.com/lark-parser/lark/pull/949
- https://github.com/lark-parser/lark/issues/455
- https://nearley.js.org/docs/parser
- https://github.com/kach/nearley/issues/292
- https://www.npmjs.com/package/nearley
- https://aclanthology.org/P92-1017/
- https://aclanthology.org/E93-1040.pdf
- https://dx.doi.org/10.1145/3428286
- https://link.springer.com/chapter/10.1007/978-3-642-03816-7_32
- https://www.usenix.org/system/files/sec22-turonova.pdf
- https://www.usenix.org/system/files/usenixsecurity24-su-weihao.pdf
- https://github.com/google/re2/wiki/Syntax
- https://docs.oracle.com/en/java/javase/26/security/java-api-xml-processing-jaxp-security-guide.html
- https://github.com/ggml-org/llama.cpp/pull/29004
- https://xgrammar.mlc.ai/docs/latest/defining_structures/lark_grammar.html
- https://github.com/akimd/bison/blob/master/NEWS
- https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html
- https://github.com/antlr/antlr4/issues/269
- https://www.semanticscholar.org/paper/A-Minimum-Distance-Error-Correcting-Parser-for-Aho-Peterson/0caae1d9d751f409067b8bcb2266b06f171e5e32
- https://arxiv.org/html/1411.7315
- https://pg.ucsd.edu/publications/Seq2Parse-neurosymbolic-parse-error-repair_OOPSLA-2022.pdf
- https://link.springer.com/chapter/10.1007/978-3-319-30000-9_41
- https://dl.acm.org/doi/10.1145/360980.360995
- https://arxiv.org/abs/1804.07133
- https://softdevteam.github.io/grmtools/master/book/errorrecovery.html
- https://softdevteam.github.io/grmtools/
- https://aclanthology.org/1993.iwpt-1.12.pdf
- https://www.cs.cmu.edu/~alavie/papers/thesis.pdf
- https://arxiv.org/html/cmp-lg/9405023
- https://www.antlr.org/api/Java/org/antlr/v4/runtime/DefaultErrorStrategy.html
- https://github.com/tree-sitter/tree-sitter/issues/833
- https://research.cs.vt.edu/AVresearch/openalgoviz/JFLAP/trunk/DOCS/gui.sim.SimulatorPane.html
- https://www.jflap.org/jflapbook/jflapbook2006.pdf
- https://turing.plymouth.edu/~zshen/Webfiles/notes/CS378/JFLAPActivities.pdf
- https://www.jflap.org/jflaptmp/
- https://www.jflap.org/
- https://burntsushi.net/regex-internals/
- https://github.com/rust-lang/regex/issues/656
- https://fservant.github.io/papers/Davis_Michael_Coghlan_Servant_Lee_ESECFSE19.pdf
- https://users.cs.utah.edu/~regehr/papers/pldi11-preprint.pdf
- https://pypi.org/project/FAdo/
- https://fado.dcc.fc.up.pt/
- https://link.springer.com/article/10.1007/BF01932308
- https://mural.maynoothuniversity.ie/id/eprint/6434/
- https://www.fuzzingbook.org/html/GrammarFuzzer.html
- https://hypothesis.readthedocs.io/en/latest/reference/strategies.html
- https://github.com/lark-parser/lark
- https://github.com/google/re2
- https://xgrammar.mlc.ai/
- https://hypothesis.readthedocs.io/
- https://www.w3.org/TR/jsgf/
- https://github.com/google-research/opengrm/blob/main/opengrm/pynini/docs/index.md
- https://www.openfst.org/doxygen/fst/html/replace_8h_source.html
- https://www.openfst.org/doxygen/fst/html/extensions_2pdt_2replace_8h_source.html
- https://www.openfst.org/
- https://relaxng.org/spec-20011203.html
- https://www.w3.org/TR/xmlschema-1/
- https://lists.w3.org/Archives/Public/xmlschema-dev/2002May/0030.html
- https://research.aalto.fi/en/publications/nfas-with-tagged-transitions-their-conversion-to-deterministic-au/
- https://ar5iv.labs.arxiv.org/html/1509.00818
- https://www.bottlecaps.de/rr/ui
- https://sourceforge.net/p/oorexx/wiki/how-to-change-oorexx-docs/
- https://arxiv.org/pdf/cs/0212046
- https://www.vdaalst.com/publications/p574.pdf
- https://scraping.pro/debuggex-review-get-your-regex-to-play-visual/
- https://www.debuggex.com/
- https://blogs.perl.org/users/jeffrey_kegler/2010/06/marpa-is-now-on-for-right-recursions.html
- https://metacpan.org/pod/release/JKEGL/Marpa-0.102000/lib/Marpa/Doc/Algorithm.pod
- https://apps.cs.utexas.edu/apps/sites/default/files/tech_reports/TR-2102.pdf
- https://arxiv.org/pdf/2506.01151
- https://github.com/guidance-ai/llguidance
- https://guidance-ai.github.io/llguidance/llg-go-brrr
- http://jeffreykegler.github.io/Ocean-of-Awareness-blog/individual/2018/05/combinator2.html
- https://ull-esit-pl-2122.github.io/practicas/egg-oop-parser.html
- https://arxiv.org/pdf/2411.15100
- https://github.com/ggml-org/llama.cpp/issues/18988
- https://github.com/ggml-org/llama.cpp/pull/18993
- https://dotat.at/tmp/gll.pdf
- https://github.com/rust-lang/gll/issues/16
- https://github.com/mike-lischke/antlr4-c3/issues/23
- https://dl.acm.org/doi/10.3115/991635.991710
- https://pure.royalholloway.ac.uk/en/publications/sppf-style-parsing-from-earley-recognisers/
- https://www.researchgate.net/publication/220367479_SPPF-Style_Parsing_From_Earley_Recognisers
- https://metacpan.org/dist/Marpa-R2/view/pod/Event.pod
- https://github.com/apache/xerces2-j
- https://github.com/apache/xerces2-j/blob/trunk/src/org/apache/xerces/impl/xs/models/XSDFACM.java
- https://github.com/eclipse-lemminx/lemminx/issues/368
- https://www.rexegg.com/regex-capture.php
- https://learn.microsoft.com/en-ca/dotnet/api/system.text.regularexpressions.group.captures?view=net-8.0
- https://martinsteffen.github.io/compilerconstruction/leftandtailrecusion/
- https://arxiv.org/html/1910.08129v3
- https://dl.acm.org/doi/pdf/10.1145/570886.570887
- https://www.gnu.org/software/bison/manual/html_node/Recursion.html
- https://lark-parser.readthedocs.io/en/latest/tree_construction.html
- https://www.antlr.org/api/Java/org/antlr/v4/runtime/Parser.html

## 6. Decision records

These decision records cite this survey:

- `docs/adr/one-earley-tracer.md`: one Earley parser traces every grammar, with one set for each step.
- `docs/adr/trees-follow-rules-and-passes.md`: a repetition makes a loop node with one child for each pass.
- `docs/adr/regex-style-ebnf-over-whole-symbols.md`: a bare name without a rule is an error, and a cap limits wide bounds.
- `docs/adr/readings-from-a-shared-forest.md`: a shared forest by the method of Scott (2008) gives every reading up to the reading cap.
