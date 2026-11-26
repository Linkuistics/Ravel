# Representation note

This note records which forms of representation the observed cases need. It follows the decision in `docs/adr/emergent-categories-and-representation.md`: a form enters only when observed cases need it. Each form below names the patterns that justify it.

Every decision in this note is provisional. The evidence is one discovery run over three chapters. A model proposed the patterns, and a review still has to check them.

## The evidence

The discovery run is `pattern-discovery` artifact `e470dcc1112a8ab7adf9b832d13cd66df0dec35f4491752b1b71c88cff95e0dc`. Its report is `pattern-report` artifact `88ca26a0241322d10a7e954c9f1a94cda56a2e80da5bd27cc2d360d82e7e6658`. The run covers three chapters:

- `6IQTDYAW`: Crafting Interpreters.
- `H3MVK7J5`: Practical Foundations for Programming Languages.
- `PZZBPXX9`: Regular Expressions Cookbook.

The run proposed 20 patterns and rejected none. The pattern IDs below have the prefix `pattern:3119a52fede4:`, so `p001` stands for `pattern:3119a52fede4:p001`.

## Hierarchy

Four patterns need nested containment:

- `p001`, fixed recipe section template: each level-2 recipe section contains level-3 part sections.
- `p002`, graded variant catalogue: a Solution section contains an ordered list of variants.
- `p012`, regex explained part by part: the explanation follows the nested parts of the regular expression.
- `p013`, aside carries semantic subtlety: a level-4 sidebar sits inside a level-3 Discussion section.

Decision: keep the section tree of the extraction as the hierarchy of the document. An episode contains its moves. The tree stays a tree. A part that two parents share goes in a cross-reference, not in a second parent.

The hierarchy in `p012` belongs to the subject matter, not to the document. The representation does not model it yet. One pattern from one book is too little.

## Cross-references

Nine patterns need links between distant parts of a chapter: `p003`, `p004`, `p005`, `p006`, `p007`, `p008`, `p017`, `p018` and `p019`. Examples:

- `p004`, new case as delta: each new case points back to an earlier case or recipe. The pattern has 9 supporting instances in three books.
- `p006`, deferral to later material: the text points to a later chapter or a later part of the book.
- `p008`, opening preview organises sections: later sections point back to the opening preview.

Decision: cross-references are typed edges between nodes of the tree and between interpretations. The `relationship` interpretations of the analysis stage are their first form. The edge keeps its evidence, as each interpretation does. Cross-reference is the form that the most patterns need.

## Attributes

Four patterns need properties on a node:

- `p001`: the fixed heading title and level identify the part of a recipe.
- `p002`: each regular expression carries options and the flavours that support it.
- `p013`: an aside has a mark such as sidenote or sidebar.
- `p015`, closing apparatus sections: apparatus sections have distinct titles or levels.

Decision: a node carries a set of named attributes. The names stay open. The vocabulary grows from the cases, and no schema fixes it yet.

## Refinement rules

Two patterns describe development, where a later part revises an earlier one:

- `p003`, failing case motivates fix: a later fix replaces an attempt that the text showed failing.
- `p005`, later revision of earlier solution: a later part changes code or a regular expression from an earlier part.

Thirteen patterns also propose a grammar fragment. An example is `Motivation -> FailingCase Principle? Fix` from `p003`. The fragments use optional parts and repeated parts.

Decision: keep two forms apart. A grammar fragment describes the order of parts in an explanation. A revision is a cross-reference of the type "revises", from the later part to the earlier one. The fragments stay text in the discovery record. No parser reads them yet.

## Open questions

- Seven patterns rest on one book: `p001`, `p002`, `p009`, `p010`, `p012`, `p017` and `p019`. The minimum counts instances, not books. A pattern of one book can describe the style of that book rather than explanation in general.
- Six patterns state no representation need: `p009`, `p010`, `p011`, `p014`, `p016` and `p020`. Seven patterns propose no grammar fragment: `p004`, `p006`, `p012`, `p013`, `p017`, `p018` and `p020`.
- The fragments of several patterns share the head `Chapter`. `p007`, `p008` and `p015` each give a different expansion. A later stage must decide whether these patterns combine into one rule.
- 226 listed observations fit no pattern. Most of them are moves (112) and episodes (45). The discovery report lists them.
