# Vertical slice: the first URG milestone

This file preserves the human's request for the first URG milestone. The human gave it on 2026-11-17. It is the authority for the detail of each step. Later sessions do not receive the conversation that carried it.

The ADRs in `docs/adr/` record its decisions, and `CONTEXT.md` records its vocabulary.

<!-- tc: exempt because the text below is the human's own request, quoted verbatim -->

---

You are helping me move **Universal Refinement Grammar (URG)** into implementation.

URG is the project. **Grove is the tooling I use to execute and review work**, alongside Codex, Claude and Herdr. This task does not call for developing Grove or designing another orchestration system. Use the existing infrastructure where appropriate, and keep the implementation focused on URG.

## Objective

Discover and represent **how authors explain technical subjects**, at four connected levels:

- Whole books.
- Chapters.
- Sections.
- Explanatory episodes, which may span several paragraphs, examples, code fragments or figures and may cross section boundaries.

The immediate objective is descriptive. We want to recover explanatory structures, relationships and recurring patterns from actual books.

**Model training and evaluation of teaching effectiveness are outside the current scope.** Do not silently substitute either of those objectives.

## Decisions already made

Carry these decisions into the implementation:

1. **Use a broad corpus with selective close analysis.** Broad coverage should reveal recurring patterns and variation. Close analysis should improve the representation and investigate ambiguous, unusual or poorly captured cases. We are not restricting the project to a small, exhaustively annotated corpus.

2. **Let the corpus help reveal the categories.** Any initial taxonomy, annotation vocabulary or formalism is provisional. Do not make a comprehensive theory of explanation a prerequisite for processing books. Preserve observations that do not fit the current categories.

3. **Preserve evidence and interpretation separately.** Retain individual observations, source passages and locations, surrounding context, document structure, relationships to code and figures, bibliographic identity, uncertainty and alternative interpretations. An interpretation must remain traceable to the evidence that supports it.

4. **Derive grammars and pattern summaries from observations.** Derived representations must not replace or erase the observations. A pattern should identify its supporting instances, exceptions and unresolved cases.

5. **Support post-hoc book and section weights.** Distinguish:
   - Reweighting the prevalence or prominence of already established patterns.
   - Rerunning discovery when different weights could change which categories or patterns are discovered.

   Reweighting an existing report does not, by itself, establish what a differently weighted discovery process would have found.

6. **Keep preference weights, corpus relevance and extraction uncertainty separately identifiable.** Also preserve uncertainty about interpretation. Do not collapse these into an opaque confidence or quality score.

7. **Choose representation from observed needs.** Trees may capture document hierarchy; graphs may capture relationships; attributes may preserve bindings and context; refinement rules may express recurring development. These are possibilities to investigate, not an architecture to impose before inspecting examples.

## Inputs

The starting inputs are:

- `Zotero Library Export.rdf`
- `zotero_corpus_audit.xlsx`

The previous audit found **2,723 bibliographic records, including 1,280 books and 841 Computing books**. Verify these figures against the available inputs and explain any discrepancy.

The workbook’s **R-prefixed record IDs identify records within that particular export**. Preserve their mapping, associated with the exact export snapshot, while establishing durable source identities. Do not treat the R-prefixed IDs as universal Zotero identifiers.

PDF attachment records still need to be reconciled with accessible files. An attachment record does not establish that the PDF is present, readable or the correct edition.

Locate the actual input files. Do not assume paths or invent unavailable contents.

## First: establish the working environment

Inspect the current project directory and applicable `AGENTS.md` instructions. Locate any existing URG repository, implementation, research notes or relevant project configuration.

Earlier context places Grove at `~/Development/grove`, with Herdr providing the interactive execution environment. Verify the current arrangement **only to the extent needed to run this project correctly**:

- Applicable repository and workspace instructions.
- Existing workflow state and configuration.
- Available execution and review capabilities.
- The established Codex/Claude implementation and review workflow.
- How project tasks and their outputs are represented and resumed.

Use existing Grove workflow state, configuration and execution mechanisms. Do not introduce a second workflow engine or duplicate Grove’s orchestration state. URG still needs its own research data, provenance, artifact versions and dependency information; keep those concerns distinct from execution control.

Determine the appropriate URG repository from the actual environment. If there is no existing repository, establish a separate project using the surrounding repository conventions. Do not put URG into the Grove codebase merely because Grove runs it.

Keep this inspection bounded. Its purpose is to enable implementation.

## Implement the smallest end-to-end milestone

Build a working vertical slice covering the following six steps.

### 1. Import the catalogue and build a source manifest

Import the RDF and reconcile it with the workbook.

Preserve the original metadata and export-specific identifiers. Establish durable identities without relying solely on titles or filenames. Distinguish bibliographic records, editions and concrete document files sufficiently to avoid silently merging different sources.

Build a manifest that records:

- Bibliographic identity and available author, edition and publication details.
- Original RDF identifiers and workbook record mappings.
- Attachment references and resolved file locations.
- File identity or fingerprint.
- Availability, readability and reconciliation status.
- Ambiguous matches, duplicates and missing files.

Retain the reasoning or evidence behind uncertain matches. Do not resolve ambiguity by silently selecting the first plausible PDF.

The expected first implementation change is a catalogue importer and inspectable source manifest, unless existing code already provides them.

### 2. Recover one chapter from each of three contrasting books

Choose three accessible books whose explanatory approaches appear meaningfully different. Base the selection on the actual catalogue and document contents, and record a short rationale.

Recover one complete chapter from each, with verified boundaries. Preserve enough surrounding structure to place each chapter within its book.

Retain, where available:

- PDF page indices and printed page labels, without conflating them.
- Headings, sections, paragraph order and source spans.
- Code blocks, captions, figures and relationships among them.
- Coordinates or equivalent anchors needed to inspect the original evidence.
- Extraction method, version and known limitations.

Check representative pages against the rendered document. Record reading-order problems, OCR uncertainty, missing figures and other extraction defects explicitly.

Do not present a few recovered chapters as analysis of the complete books.

### 3. Produce inspectable candidate explanatory structures

Analyse the recovered chapters and produce candidate structures supported by explicit evidence.

Begin with descriptions grounded in the passages. Allow provisional labels and competing interpretations. Do not force every passage into a predefined catalogue of rhetorical moves.

Represent the distinction between:

- Observed document structure.
- Candidate explanatory episodes, moves and relationships.
- Patterns or grammar fragments inferred across observations.

Preserve relationships between levels where the evidence supports them. Episodes should not be required to form a strict, non-overlapping tree.

Provide inspectable machine-readable records and a readable view that lets me move from a proposed structure or pattern to its supporting passages and source locations. A simple generated report is sufficient; a bespoke application is not required.

Document which observed cases justify hierarchy, cross-references, attributes or refinement rules, and which decisions remain provisional.

### 4. Correct an analysis and regenerate derived results

Provide a practical way to correct an observation or interpretation, including episode boundaries, labels, relationships and alternative readings.

Preserve the original analysis and correction history. Corrections must identify their target version and rationale, and must not silently overwrite source evidence.

Demonstrate one substantive correction and regenerate the affected derived results. Show what changed and what remained reusable.

A correction to interpretation should not require extracting unchanged PDFs again.

### 5. Compare equal-book and supplied-weight reports

Generate a pattern report using an explicit **equal-book baseline**, so a longer analysed chapter does not automatically give its book more influence.

Support a separate, inspectable configuration for book and section weights. Document the aggregation formula, normalization, denominators and handling of missing or unanalysed material.

Compare the baseline with a supplied weight configuration. If I have not supplied actual preferences, use a clearly labelled illustrative configuration to demonstrate the mechanism; do not describe those values as my preferences.

Keep preference weights, relevance assessments and uncertainty visible as distinct inputs.

For this milestone, comparison may reweight a fixed set of discovered patterns. Label that limitation. Record enough discovery provenance to support a later, separate discovery run under different weights.

### 6. Verify resumability and reuse

Demonstrate that the work can resume after interruption and reuse valid completed outputs.

Reuse should depend on the relevant inputs and versions, not just the existence of an output file. Capture dependencies such as:

- Source document identity.
- Extraction code and configuration.
- Representation/schema version.
- Analysis instructions and recorded model configuration.
- Analysis revisions and corrections.
- Discovery settings.
- Report weight configuration.

Show that:

- Restarting does not repeat valid completed work.
- Incomplete or invalid outputs are not accepted as completed.
- A correction invalidates the appropriate derived results.
- Changing report weights reuses extraction and analysis when discovery is unchanged.

Use Grove’s existing execution mechanisms for running and resuming tasks. Implement only the URG-specific artifact and dependency behaviour needed to support them.

## Implementation approach

Choose the language, tools and layout from the actual project context. Prefer a small, inspectable implementation over speculative infrastructure.

Make stages batchable across the corpus through explicit inputs and outputs, stable identifiers, per-source results and recorded failures. One unavailable or malformed book should not prevent other books from being processed.

Keep representations versioned. Record analysis instructions, execution provenance and relevant configuration so that a result can be investigated or rerun. Do not imply that recording these guarantees identical future model output.

Use the existing Codex/Claude review workflow. Review should examine both software behaviour and the faithfulness of the representation to the sampled passages. Keep proposed interpretations distinguishable from independently reviewed or corrected ones.

Add meaningful verification for identity mapping, evidence traceability, correction propagation, weighting and resumability. Avoid building a large test framework before the vertical slice works.

Do not spend this milestone designing a universal DSL, a complete taxonomy, a graph platform or a custom orchestration layer.

## Working instructions and completion report

Begin by reporting:

1. The repository and relevant infrastructure actually found.
2. Any material access limitations.
3. The first concrete implementation change.

Then proceed with the work. Make routine, reversible decisions without repeatedly asking for approval. Ask only when missing information materially blocks correct progress or would require an expensive commitment.

If access prevents part of the milestone, complete the useful work that is supported, identify the exact missing input or capability, and distinguish implemented behaviour from unverified integration. Do not fabricate chapter analyses, file availability or successful runs.

At completion, provide:

- What was implemented and where.
- Commands or existing workflow entry points to reproduce the milestone.
- The three selected books and chapters.
- Inspectable examples linking candidate structures to source evidence.
- The demonstrated correction and its downstream effects.
- The equal-book versus supplied-weight comparison.
- Evidence of interruption recovery and reuse.
- Remaining extraction, representation and integration limitations.
- The next smallest step justified by what the observed cases revealed.

The goal is a working, evidence-preserving URG research pipeline that can grow across the corpus while allowing its explanatory categories and formal representation to evolve.
