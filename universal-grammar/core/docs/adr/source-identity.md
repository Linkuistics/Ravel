# Source identity

A catalogue record's durable identity is its Zotero library and item key. URG reads both from a read-only copy of the Zotero database. A document file's identity is the SHA-256 of its content. RDF identifiers and workbook R-IDs are export-scoped aliases, stored with the fingerprint of the snapshot that defines them.

The database copy also supplies attachment file paths, because the RDF export has none. An RDF `#item_N` equals the database item ID when both come from the same library state. Each import checks that agreement and records the result.

## Trade-off

The importer needs a second input, and the RDF and the database must come from the same library state. In return, identity rests on stable keys, not on titles or filenames.

## Rejected alternatives

- Matching titles to filenames in Zotero storage. Duplicates, editions and renamed files make this ambiguous, and the human asked for identity that does not rest solely on titles or filenames.
- RDF `rdf:about` URIs as identity. They are export-scoped, and they mix URLs, ISBNs and local numbers.

## Evidence

In the first snapshot pair, all 2,961 RDF attachment identifiers matched database attachment rows. All 2,704 linked catalogue records agreed with the database on item type and title. The remaining 19 records have no attachment, so field matching must link them, with recorded evidence.
