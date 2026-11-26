"""A small Zotero library: an RDF export, a database extract and short PDFs.

The fixture holds these cases:

- item 10, a book with two PDFs, linked through its attachments (several PDFs)
- item 20, a book exported as ``#item_20`` whose PDF is missing from storage
- items 30 and 40, two books backed by one stored file, the second as a linked file
- item 50, a book without attachments or a local number, linked by a field match, whose
  edition disagrees between the RDF and the database
- item 60, an article exported as ``#item_60`` with another title, and no PDF
- item 70, a book in the trash, absent from the export
- item 80, a book in a group library
- item 90, a book whose PDF does not open
"""

from __future__ import annotations

import sqlite3
from dataclasses import dataclass
from pathlib import Path

import pymupdf  # pyright: ignore[reportMissingTypeStubs]

SCHEMA = """
CREATE TABLE libraries (libraryID INTEGER PRIMARY KEY, type TEXT NOT NULL);
CREATE TABLE itemTypes (itemTypeID INTEGER PRIMARY KEY, typeName TEXT);
CREATE TABLE items (itemID INTEGER PRIMARY KEY, itemTypeID INT, libraryID INT, key TEXT);
CREATE TABLE deletedItems (itemID INTEGER PRIMARY KEY);
CREATE TABLE fields (fieldID INTEGER PRIMARY KEY, fieldName TEXT);
CREATE TABLE baseFieldMappings (itemTypeID INT, baseFieldID INT, fieldID INT);
CREATE TABLE itemDataValues (valueID INTEGER PRIMARY KEY, value UNIQUE);
CREATE TABLE itemData (itemID INT, fieldID INT, valueID INT);
CREATE TABLE creators (creatorID INTEGER PRIMARY KEY, firstName TEXT, lastName TEXT);
CREATE TABLE creatorTypes (creatorTypeID INTEGER PRIMARY KEY, creatorType TEXT);
CREATE TABLE itemCreators (itemID INT, creatorID INT, creatorTypeID INT, orderIndex INT);
CREATE TABLE itemAttachments (
    itemID INTEGER PRIMARY KEY, parentItemID INT, linkMode INT, contentType TEXT, path TEXT
);
"""

ITEM_TYPES = {"book": 1, "journalArticle": 2, "attachment": 3}
FIELDS = {"title": 1, "date": 2, "edition": 3, "publisher": 4, "place": 5, "ISBN": 6}


def key(item_id: int) -> str:
    return f"KEY{item_id:05d}"


@dataclass(frozen=True)
class Fixture:
    rdf: Path
    zotero_dir: Path
    shared_pdf: Path


RDF_HEAD = """<?xml version="1.0"?>
<rdf:RDF
 xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
 xmlns:z="http://www.zotero.org/namespaces/export#"
 xmlns:dc="http://purl.org/dc/elements/1.1/"
 xmlns:dcterms="http://purl.org/dc/terms/"
 xmlns:bib="http://purl.org/net/biblio#"
 xmlns:foaf="http://xmlns.com/foaf/0.1/"
 xmlns:link="http://purl.org/rss/1.0/modules/link/"
 xmlns:prism="http://prismstandard.org/namespaces/1.2/basic/"
 xmlns:vcard="http://nwalsh.com/rdf/vCard#">
"""


def _book(about: str, title: str, attachments: list[int], extra: str = "") -> str:
    links = "".join(f'<link:link rdf:resource="#item_{n}"/>' for n in attachments)
    return (
        f'<bib:Book rdf:about="{about}"><z:itemType>book</z:itemType>'
        f"<dc:title>{title}</dc:title>{links}{extra}</bib:Book>\n"
    )


def _attachment(number: int, title: str, mime: str = "application/pdf") -> str:
    return (
        f'<z:Attachment rdf:about="#item_{number}"><z:itemType>attachment</z:itemType>'
        f"<dc:title>{title}</dc:title><link:type>{mime}</link:type></z:Attachment>\n"
    )


RDF_BODY = (
    _book(
        "urn:isbn:9780262510875",
        "Structure and Interpretation of Computer Programs",
        [11, 12],
        "<dc:publisher><foaf:Organization><vcard:adr><vcard:Address>"
        "<vcard:locality>Cambridge</vcard:locality></vcard:Address></vcard:adr>"
        "<foaf:name>MIT Press</foaf:name></foaf:Organization></dc:publisher>"
        "<bib:authors><rdf:Seq><rdf:li><foaf:Person><foaf:surname>Abelson</foaf:surname>"
        "<foaf:givenName>Harold</foaf:givenName></foaf:Person></rdf:li></rdf:Seq></bib:authors>"
        "<dc:date>1996</dc:date><prism:edition>2nd</prism:edition>"
        "<dc:identifier>ISBN 978-0-262-51087-5</dc:identifier>",
    )
    + _attachment(11, "Full Text PDF")
    + _attachment(12, "Second Scan")
    + _book("#item_20", "Missing Book", [21])
    + _attachment(21, "Missing PDF")
    + _book("http://example.org/one", "Shared One", [31])
    + _attachment(31, "Shared PDF")
    + _book("http://example.org/two", "Shared Two", [41])
    + _attachment(41, "Shared PDF, linked")
    + _book(
        "http://example.org/fieldmatch",
        "Field Match Book",
        [],
        "<prism:edition>1st</prism:edition><dc:date>2001</dc:date>",
    )
    + '<bib:Article rdf:about="#item_60"><z:itemType>journalArticle</z:itemType>'
    "<dc:title>An Article (draft)</dc:title></bib:Article>\n"
    + _book("#item_90", "Broken Book", [91])
    + _attachment(91, "Broken PDF")
    + '<z:Collection rdf:about="#collection_1"><dc:title>Computing</dc:title>'
    '<dcterms:hasPart rdf:resource="#collection_2"/>'
    '<dcterms:hasPart rdf:resource="#item_20"/></z:Collection>\n'
    '<z:Collection rdf:about="#collection_2"><dc:title>Lisp</dc:title>'
    '<dcterms:hasPart rdf:resource="urn:isbn:9780262510875"/></z:Collection>\n'
    '<z:Collection rdf:about="#collection_3"><dc:title>Other</dc:title>'
    '<dcterms:hasPart rdf:resource="http://example.org/one"/>'
    '<dcterms:hasPart rdf:resource="#item_60"/></z:Collection>\n'
)


def _pdf(path: Path, pages: int, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    document = pymupdf.open()  # pyright: ignore
    for page_number in range(pages):
        page = document.new_page()  # pyright: ignore
        page.insert_text((72, 72), f"{text} page {page_number + 1}")  # pyright: ignore
    document.save(path)  # pyright: ignore
    document.close()  # pyright: ignore


def build_fixture(root: Path) -> Fixture:
    root.mkdir(parents=True, exist_ok=True)
    rdf = root / "export.rdf"
    rdf.write_text(RDF_HEAD + RDF_BODY + "</rdf:RDF>\n")
    zotero_dir = root / "Zotero"
    storage = zotero_dir / "storage"
    _pdf(storage / key(11) / "sicp.pdf", 1, "first scan")
    _pdf(storage / key(12) / "sicp-2.pdf", 2, "second scan")
    shared_pdf = storage / key(31) / "shared.pdf"
    _pdf(shared_pdf, 1, "shared")
    broken = storage / key(91) / "broken.pdf"
    broken.parent.mkdir(parents=True)
    broken.write_bytes(b"this is not a PDF")

    connection = sqlite3.connect(zotero_dir / "zotero.sqlite")
    connection.executescript(SCHEMA)
    connection.executemany("INSERT INTO libraries VALUES (?, ?)", [(1, "user"), (2, "group")])
    connection.executemany(
        "INSERT INTO itemTypes VALUES (?, ?)", [(v, k) for k, v in ITEM_TYPES.items()]
    )
    connection.executemany("INSERT INTO fields VALUES (?, ?)", [(v, k) for k, v in FIELDS.items()])
    connection.executemany("INSERT INTO creatorTypes VALUES (?, ?)", [(1, "author")])

    def item(item_id: int, item_type: str, library_id: int = 1, **fields: str) -> None:
        connection.execute(
            "INSERT INTO items VALUES (?, ?, ?, ?)",
            (item_id, ITEM_TYPES[item_type], library_id, key(item_id)),
        )
        for name, value in fields.items():
            connection.execute("INSERT OR IGNORE INTO itemDataValues (value) VALUES (?)", (value,))
            (value_id,) = connection.execute(
                "SELECT valueID FROM itemDataValues WHERE value = ?", (value,)
            ).fetchone()
            connection.execute(
                "INSERT INTO itemData VALUES (?, ?, ?)", (item_id, FIELDS[name], value_id)
            )

    def attachment(item_id: int, parent: int, path: str, link_mode: int = 0, *, title: str) -> None:
        item(item_id, "attachment", title=title)
        connection.execute(
            "INSERT INTO itemAttachments VALUES (?, ?, ?, ?, ?)",
            (item_id, parent, link_mode, "application/pdf", path),
        )

    item(
        10,
        "book",
        title="Structure and Interpretation of Computer Programs",
        date="1996-00-00 1996",
        edition="2nd",
        publisher="MIT Press",
        place="Cambridge",
        ISBN="9780262510875",
    )
    connection.execute("INSERT INTO creators VALUES (1, 'Harold', 'Abelson')")
    connection.execute("INSERT INTO itemCreators VALUES (10, 1, 1, 0)")
    attachment(11, 10, "storage:sicp.pdf", title="Full Text PDF")
    attachment(12, 10, "storage:sicp-2.pdf", title="Second Scan")
    item(20, "book", title="Missing Book")
    attachment(21, 20, "storage:missing.pdf", title="Missing PDF")
    item(30, "book", title="Shared One")
    attachment(31, 30, "storage:shared.pdf", title="Shared PDF")
    item(40, "book", title="Shared Two")
    attachment(41, 40, str(shared_pdf), link_mode=2, title="Shared PDF, linked")
    item(50, "book", title="Field Match Book", edition="First", date="2001")
    item(60, "journalArticle", title="An Article")
    item(70, "book", title="Trashed Book")
    connection.execute("INSERT INTO deletedItems VALUES (70)")
    item(80, "book", library_id=2, title="Group Book")
    item(90, "book", title="Broken Book")
    attachment(91, 90, "storage:broken.pdf", title="Broken PDF")
    connection.commit()
    connection.close()
    return Fixture(rdf=rdf, zotero_dir=zotero_dir, shared_pdf=shared_pdf)
