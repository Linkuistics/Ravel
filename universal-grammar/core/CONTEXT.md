# Universal Refinement Grammar

URG discovers and represents how authors explain technical subjects, from whole books down to explanatory episodes. It keeps source evidence, interpretations and derived patterns separately identifiable.

## Language

### Catalogue and sources

**Catalogue record**:
One bibliographic entry in the Zotero user library, such as a book or an article.
_Avoid_: item, entry, record (alone)

**Export snapshot**:
One Zotero RDF export file, identified by its SHA-256 fingerprint.
_Avoid_: the export, the library

**Export-scoped ID**:
An identifier that is valid only inside one **Export snapshot** or one audit workbook. RDF `#item_N` values, RDF `rdf:about` URIs and workbook R-IDs are export-scoped.
_Avoid_: record ID, Zotero ID

**Source identity**:
The durable identity of a **Catalogue record**: its Zotero library and item key.
_Avoid_: title, filename

**Edition**:
One published version of a work, distinguished by details such as ISBN, edition statement, year and publisher.

**Attachment record**:
Zotero's claim that a **Catalogue record** has a file. It does not establish that the file is present, readable or the correct **Edition**.

**Document file**:
One concrete file, such as a PDF, identified by the SHA-256 of its content.

**Source manifest**:
The inspectable list of **Catalogue records** with their identities, **Attachment records**, **Document files** and **Reconciliation status**.

**Reconciliation status**:
The recorded outcome of matching a **Catalogue record** to its **Document files**, with the evidence behind any uncertain match.

### Evidence and interpretation

**Evidence**:
Recovered source material with its location, such as a text span, code block, figure or caption. Its anchors are the PDF page index, the printed page label and coordinates.

**Document structure**:
The hierarchy of a book as extraction recovers it: chapters, sections, paragraphs, code blocks, figures and captions. It belongs to the **Evidence** layer.

**Interpretation**:
A claim about **Evidence**, such as a label, a boundary, a relationship or a reading. It cites its evidence and records its author and its **Interpretation uncertainty**.
_Avoid_: annotation, finding

**Explanatory episode**:
A stretch of explanation with one purpose. It can span paragraphs, examples, code and figures, cross section boundaries and overlap other episodes.
_Avoid_: segment, unit

**Move**:
One step inside an **Explanatory episode**, such as introducing a term or giving an example. Its labels are provisional.

**Relationship**:
A link between episodes, moves, code, figures or **Evidence**, such as "example of" or "refines".

**Alternative reading**:
A competing **Interpretation** of the same **Evidence**. URG keeps alternatives side by side and does not force a choice.

**Correction**:
A recorded change to one **Interpretation** version, with a rationale. It creates a new version and never overwrites **Evidence** or earlier versions.

**Observation**:
Any recorded instance that a **Pattern** can cite: a fact of **Document structure** or an **Interpretation**. Each observation records its layer.

### Derived results

**Pattern**:
A recurring structure derived from **Observations**. It names its supporting instances, its exceptions and its unresolved cases.
_Avoid_: rule, category (alone)

**Grammar fragment**:
A **Pattern** expressed as rules, for example refinement rules that describe how an explanation develops.

**Derived result**:
Anything computed from **Observations**, such as a **Pattern** or a report. URG can regenerate it, so it never replaces the observations.

### Weighting and uncertainty

**Preference weight**:
The human's stated importance of a book or a section for aggregation.

**Relevance assessment**:
A judgement of how relevant a book or a section is to the corpus question. It is not a **Preference weight**.

**Extraction uncertainty**:
Doubt about recovered **Evidence**, such as OCR errors, reading-order problems or missing figures.

**Interpretation uncertainty**:
Doubt about an **Interpretation**, recorded together with its **Alternative readings**.

**Equal-book baseline**:
An aggregation in which each analysed book has the same total influence, whatever the length of its analysed material.

**Reweighting**:
Recomputing the prevalence or prominence of already discovered **Patterns** under different weights.

**Discovery run**:
One process that finds categories and **Patterns** from **Observations** under recorded settings, including weights. A rerun with other settings can find different patterns.

### Pipeline

**Stage**:
One pipeline step with explicit inputs and outputs, such as catalogue import, extraction, analysis, correction, derivation or report.

**Research artifact**:
A versioned URG data product, such as a **Source manifest**, an extraction, an analysis or a report.
_Avoid_: artifact (alone), output file

**Dependency record**:
The identities and versions of every input behind a **Research artifact**. Reuse compares dependency records, never the mere existence of a file.

**Close analysis**:
Detailed analysis of a selected case. It refines the representation, or it examines an ambiguous, unusual or poorly captured case.

## Relationships

- A **Catalogue record** has zero or more **Attachment records**.
- An **Attachment record** resolves to zero or one **Document file**.
- One **Document file** can back several **Catalogue records**, and the **Source manifest** flags them as duplicates.
- An **Interpretation** cites one or more pieces of **Evidence**.
- A **Correction** targets exactly one **Interpretation** version.
- A **Pattern** cites its supporting **Observations**, its exceptions and its unresolved cases.
- A **Research artifact** has exactly one **Dependency record**.

## Example dialogue

> **Dev:** "The correction moves the end of episode 3 two paragraphs later. Do I extract the chapter again?"
> **Domain expert:** "No. The **Evidence** is unchanged. The **Correction** creates a new **Interpretation** version, and only the **Derived results** that cite episode 3 regenerate."
> **Dev:** "And the weighted report?"
> **Domain expert:** "If only the **Preference weights** changed, that is **Reweighting**. It reuses the discovered **Patterns**. It does not show what a new **Discovery run** would find."

## Flagged ambiguities

- The request says "record" for **Catalogue records**. Pipeline data are **Research artifacts** or **Observations**.
- "Weight" alone is ambiguous. Name the **Preference weight**, the **Relevance assessment** or the uncertainty.
- Do not use a "confidence" or "quality" score. Name the uncertainty instead.
- The request uses "observation" for structural facts and for interpretations alike. URG accepts both and records the layer of each **Observation**.
- R-IDs are workbook IDs, scoped to the export behind that workbook. They are not Zotero identifiers.
- Grove also says "artifact" for committed work. URG data products are **Research artifacts**.
