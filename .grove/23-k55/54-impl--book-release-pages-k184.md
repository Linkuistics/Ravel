# book-release-pages-k184


## Goal

Implement the book contract's release-to-GitHub-Pages path. Build the book from
the exact release source, preserve its checked version, and make deployment
and recovery part of the release procedure. Test the path without a public
release. The first public book release remains outside this book node.



## Context

- Found by draft `deliver-ravel-k182`. Read `docs/book/contract.md`, section 3,
  the book node brief and chapter 12's “Keep the book with its release”.
  This is an agreed delivery requirement, not a new hosting choice.
- At discovery, `Taskfile.yml`'s sequential release task checks chapter 3's
  tagged version line but does not build or deploy the book. Its script calls
  cover doctor, preparation, product checks, version cut, archive build/smoke,
  GitHub assets, tap and local installation. `scripts/book.sh` has setup, build
  and local serve modes only. The checkout has no `.github/` directory.
- Chapters 3 and 12 must agree with the delivered mechanism. Chapter 3 already
  directs readers to the book for their installed release on GitHub Pages.
  Chapter 12 currently states the implementation gap. Decide and test how a
  reader reaches the right release's book, with the contract retained.
- Keep the pinned mdBook and figure toolchain. The book stays out of product
  archives and worker identity. Keep ordinary `task check` independent of the
  book toolchain, as toolchain leaf k58 established.
- Use the existing Taskfile for reusable work. Check current GitHub Pages
  requirements against official sources. Preserve the order that puts every
  archive smoke route before public release changes. Define the deployment
  boundary and partial-failure recovery explicitly.
- No public version cut, project tag, push, tap edit, Pages deployment or
  permanent installation is authorised by this leaf. Use isolated fixtures and
  dry runs. Report separately any remote configuration needed at first release.
- This leaf precedes set proof `book-second-half-k163`. After implementation,
  insert a contiguous chapter-12 correction run before that proof: draft,
  copy-edit, art, proof, each with bare stem `deliver-ravel`. The draft updates
  the release order, Pages instructions, recovery and task inventory from the
  actual implementation. Reconcile chapter 3's statement through its editorial
  owner if the mechanism changes its teaching. Do not let set proof absorb
  missing drafting. Observe the editorial family's re-run escalation rule.

## Done when

- The release builds the exact tag's book and delivers it under the agreed
  Pages contract. A reader can identify and reach the book for their release.
- Executable controls reject an incorrect version/source, a failed build,
  premature deployment and an incomplete delivery. Recovery resumes the
  intended release without another bump or a concealed identity change.
- Tests run without public-service writes and preserve the current archive,
  formula, release-order and book-version controls. Record any remote checks
  that fixtures cannot establish.
- The implementation's teaching and figure changes have a complete editorial
  correction run before set proof. The book node brief carries any exact
  first-release setup or external limitation, without claiming a deployed site.

## Notes
