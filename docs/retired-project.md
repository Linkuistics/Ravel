# Retired actor-daemon project

The actor-daemon design and its complete source tree are preserved in this
repository under the bookmark `retired/actor-daemon-2026-10-12` at commit
`035edfa30f3bf40eb870a525f49ac98a3b5abbb1`. The bookmark was pushed to
`origin` (`https://github.com/Linkuistics/Ravel.git`) and its exact remote
commit was verified on 2026-10-12, before active source replacement.

This revision includes the owner's local README table-formatting edit. Its
tree is identical to the starting working-copy snapshot
`93f202ba5438955f27090ae0444cc93e61fd8d15`; only the commit description was
added before sealing the snapshot. The preserved README's SHA-256 is
`9ce4d3e7d4774cad418c713bc4feec6f9ded6302ab77ca7eb0f02ecf3058600b`.
The previous main tip, `2f452e51b7297de2397f052cb3bcec308267b677`, remains
an ancestor. Replacement work advances main through ordinary descendant
commits, preserving the old history and this retired bookmark.

## Inspect or retrieve

From a jj checkout of Ravel, fetch the remote and inspect the historical README
without changing the working copy:

```sh
jj git fetch --remote origin
jj --no-pager log -r 'retired/actor-daemon-2026-10-12@origin'
jj file show -r 'retired/actor-daemon-2026-10-12@origin' README.md
```

To browse the whole historical project in a separate workspace:

```sh
jj workspace add --revision 'retired/actor-daemon-2026-10-12@origin' ../Ravel-retired
```

The preserved source, design documents and README describe the older
actor-daemon project. They are historical material, not requirements for the
extracted launcher. Keep this document in the replacement project's ordinary
documentation so the retired project remains discoverable.

## Independent version lifecycle

The tag inspection found only `structured-plan-migration`, with no released
version tag. The old README's discussion of a previous v0.1.0 is design prose,
not a release-tag baseline. The extracted development front and private worker
start together at **0.0.0**; the first minor release is **0.1.0**. Their build
identities use Ravel's actual version, independently of the source launcher's
22.1.0 version.

## Transition boundary

This preservation increment changes only this documentation after sealing the
owner's README edit. The active old source tree remains in place at this
boundary. Subsequent increments deliver the independent runnable pair and then
copy personal configuration and snapshot history into its separate store.
