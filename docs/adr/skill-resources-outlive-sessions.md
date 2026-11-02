# Skill resources outlive sessions

Ravel retains captured skill sources and directory descriptors as immutable
content objects, beyond any session. It has no automatic collection and no
prune command. The
[skill discovery specification](../specs/skill-discovery.md#publication-concurrency-and-cleanup)
owns their format, publication, integrity and manual removal.

Readers include detached delegates and harness sessions that the owner opens
independently. Ravel tracks neither. Retention commits the cache format to old
resource paths that stay readable across upgrades. Copies and abandoned objects
cost disk space. An owner stops the affected readers before a manual removal.
This content cache is not a session log or a result store.

## Considered options

- **Directories that point into mutable checkouts.** The design rejected them,
  because a source edit or move would change the resources of an existing
  reader. Reconsider copying when large packs make bounded capture impractical
  and a real snapshot facility keeps the same reader semantics.
- **Reference counts or expiry for each session.** The design rejected them,
  because neither proves that every reader has stopped.
- **A persistent reader-lease service.** It is outside this increment, because
  every independent reader would have to join its protocol. Reopen collection
  when an explicit reader-lifetime protocol can cover those readers.
