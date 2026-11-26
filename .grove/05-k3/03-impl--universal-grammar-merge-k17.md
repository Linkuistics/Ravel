# universal-grammar-merge-k17

## Goal

Clean the abandoned Visualiser work out of jj. Then merge UniversalGrammar and
UniversalGrammar-Visualiser into the Ravel repository, on the terms that
`merge-terms` settled.

## Context

- The running log of `plan-k1`: "The Visualiser's changes are garbage". The
  owner wrote: "The Visualiser work was abandoned. jj needs to gc those
  commits."
- The running log of `merge-terms-k15` holds the publication decision and the
  layout.
- UniversalGrammar-ChatGPT is a folder of resources, not a repository. It is
  not part of the merge.

## Done when

- The 12 unmerged Visualiser changes are abandoned in jj, and the garbage is
  collected.
- The `UniversalGrammar-Visualiser.gh-issue-1` workspace is retired.
- Both repositories are in their agreed places, with the history that the
  owner agreed.
- Ravel's Taskfile runs their checks, and they pass. Ravel's own checks still
  pass.

## Notes

- The abandonment and the garbage collection cannot be undone. Before each
  step, list exactly the changes and the workspace that it removes. Compare the
  list with the 12 changes that `plan-k1` counted, and stop if they differ.
- Leave the live grove tree in the `gh-issue-1` workspace untouched until the
  workspace retires. Its content is the abandoned work.
