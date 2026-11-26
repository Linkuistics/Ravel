"""Read a read-only copy of the Zotero database.

The importer never opens the live database for writing. It copies ``zotero.sqlite`` to a
temporary file, checks that the source did not change during the copy and opens the copy with
``immutable=1``. An ``immutable=1`` read ignores the write-ahead log, so the importer refuses a
database whose ``-wal`` file holds pages.
"""

from __future__ import annotations

import hashlib
import shutil
import sqlite3
import tempfile
from collections.abc import Generator
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path

from urg.errors import UrgError

# Zotero link modes, verified against the Zotero source:
# https://raw.githubusercontent.com/zotero/zotero/main/chrome/content/zotero/xpcom/attachments.js
LINK_MODES = {
    0: "imported_file",
    1: "imported_url",
    2: "linked_file",
    3: "linked_url",
    4: "embedded_image",
}
NON_RECORD_TYPES = ("attachment", "note", "annotation")


class ZoteroDbError(UrgError):
    """The Zotero database cannot be read safely."""


@dataclass(frozen=True)
class DbCreator:
    role: str
    last_name: str
    first_name: str


@dataclass(frozen=True)
class DbItem:
    item_id: int
    library_id: int
    key: str
    item_type: str
    deleted: bool
    fields: dict[str, str]
    creators: tuple[DbCreator, ...]

    @property
    def title(self) -> str:
        return self.fields.get("title", "")


@dataclass(frozen=True)
class DbAttachment:
    item_id: int
    library_id: int
    key: str
    parent_item_id: int | None
    link_mode: int | None
    content_type: str | None
    path: str | None
    deleted: bool
    title: str


@dataclass(frozen=True)
class DbLibrary:
    library_id: int
    library_type: str


@dataclass(frozen=True)
class DbSnapshot:
    """Everything the importer needs from one database copy."""

    sha256: str
    libraries: dict[int, DbLibrary]
    items: dict[int, DbItem]
    attachments: dict[int, DbAttachment]

    def regular_items(self, library_id: int) -> list[DbItem]:
        return [
            item
            for item in self.items.values()
            if item.library_id == library_id and item.item_type not in NON_RECORD_TYPES
        ]


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


@contextmanager
def database_copy(zotero_dir: Path) -> Generator[tuple[Path, str]]:
    """Yield a temporary copy of ``zotero.sqlite`` and its SHA-256."""
    source = zotero_dir / "zotero.sqlite"
    if not source.is_file():
        raise ZoteroDbError(f"no Zotero database at {source}")
    wal = zotero_dir / "zotero.sqlite-wal"
    if wal.exists() and wal.stat().st_size > 0:
        raise ZoteroDbError(
            f"{wal} holds uncommitted pages. Close Zotero so that it checkpoints the log, "
            "then run the import again."
        )
    with tempfile.TemporaryDirectory(prefix="urg-zotero-") as temporary:
        copy = Path(temporary) / "zotero.sqlite"
        before = sha256_file(source)
        shutil.copyfile(source, copy)
        after = sha256_file(copy)
        if before != after or sha256_file(source) != before:
            raise ZoteroDbError(f"{source} changed during the copy. Run the import again.")
        yield copy, after


def _connect(copy: Path) -> sqlite3.Connection:
    connection = sqlite3.connect(f"{copy.resolve().as_uri()}?immutable=1", uri=True)
    connection.row_factory = sqlite3.Row
    return connection


def read_snapshot(copy: Path, sha256: str) -> DbSnapshot:
    """Read libraries, items, fields, creators and attachments from a database copy."""
    try:
        connection = _connect(copy)
    except sqlite3.Error as e:
        raise ZoteroDbError(f"cannot open database copy: {e}") from e
    try:
        return _read(connection, sha256)
    except sqlite3.Error as e:
        raise ZoteroDbError(f"cannot read database copy: {e}") from e
    finally:
        connection.close()


def _read(connection: sqlite3.Connection, sha256: str) -> DbSnapshot:
    libraries = {
        row["libraryID"]: DbLibrary(row["libraryID"], row["type"])
        for row in connection.execute("SELECT libraryID, type FROM libraries")
    }
    deleted = {row[0] for row in connection.execute("SELECT itemID FROM deletedItems")}

    # A field such as bookTitle or websiteTitle maps onto a base field such as publicationTitle.
    # The title of every item type maps onto the base field "title".
    base_name: dict[int, str] = {}
    for row in connection.execute(
        "SELECT m.fieldID AS fieldID, f.fieldName AS baseName "
        "FROM baseFieldMappings m JOIN fields f ON f.fieldID = m.baseFieldID"
    ):
        base_name[row["fieldID"]] = row["baseName"]

    fields: dict[int, dict[str, str]] = {}
    for row in connection.execute(
        "SELECT d.itemID AS itemID, d.fieldID AS fieldID, f.fieldName AS fieldName, "
        "v.value AS value FROM itemData d JOIN fields f ON f.fieldID = d.fieldID "
        "JOIN itemDataValues v ON v.valueID = d.valueID"
    ):
        item_fields = fields.setdefault(row["itemID"], {})
        value = str(row["value"])
        item_fields[row["fieldName"]] = value
        base = base_name.get(row["fieldID"])
        if base is not None:
            item_fields.setdefault(base, value)

    creators: dict[int, list[DbCreator]] = {}
    for row in connection.execute(
        "SELECT ic.itemID AS itemID, t.creatorType AS role, c.lastName AS lastName, "
        "c.firstName AS firstName FROM itemCreators ic "
        "JOIN creators c ON c.creatorID = ic.creatorID "
        "JOIN creatorTypes t ON t.creatorTypeID = ic.creatorTypeID "
        "ORDER BY ic.itemID, ic.orderIndex"
    ):
        creators.setdefault(row["itemID"], []).append(
            DbCreator(row["role"], row["lastName"] or "", row["firstName"] or "")
        )

    items: dict[int, DbItem] = {}
    for row in connection.execute(
        "SELECT i.itemID AS itemID, i.libraryID AS libraryID, i.key AS key, "
        "t.typeName AS typeName FROM items i JOIN itemTypes t ON t.itemTypeID = i.itemTypeID"
    ):
        item_id = row["itemID"]
        items[item_id] = DbItem(
            item_id=item_id,
            library_id=row["libraryID"],
            key=row["key"],
            item_type=row["typeName"],
            deleted=item_id in deleted,
            fields=fields.get(item_id, {}),
            creators=tuple(creators.get(item_id, [])),
        )

    attachments: dict[int, DbAttachment] = {}
    for row in connection.execute(
        "SELECT itemID, parentItemID, linkMode, contentType, path FROM itemAttachments"
    ):
        item = items.get(row["itemID"])
        if item is None:
            continue
        attachments[item.item_id] = DbAttachment(
            item_id=item.item_id,
            library_id=item.library_id,
            key=item.key,
            parent_item_id=row["parentItemID"],
            link_mode=row["linkMode"],
            content_type=row["contentType"],
            path=row["path"],
            deleted=item.deleted,
            title=item.title,
        )
    return DbSnapshot(sha256=sha256, libraries=libraries, items=items, attachments=attachments)


def resolve_attachment_file(zotero_dir: Path, attachment: DbAttachment) -> Path | None:
    """Return the file location of a stored or linked attachment, or ``None`` for a link.

    Zotero stores an imported file as ``storage:<file name>`` in the directory named after the
    attachment key, under ``<Zotero data directory>/storage``. A linked file keeps an absolute
    path. Source: Zotero.Attachments.getStorageDirectoryByLibraryAndKey in
    https://raw.githubusercontent.com/zotero/zotero/main/chrome/content/zotero/xpcom/attachments.js
    """
    path = attachment.path
    if not path:
        return None
    if path.startswith("storage:"):
        return zotero_dir / "storage" / attachment.key / path.removeprefix("storage:")
    if attachment.link_mode == 2:
        return Path(path)
    return None
