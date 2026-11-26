# configuration-experience-k7

## Goal

Show the owner what the new configuration experience could look like, and
collect the owner's reaction. The owner reacts to three throwaway samples:

- A `config.lua` that routes kinds and selects skills.
- A standalone skill file, with and without metadata.
- The output of a static inspection and of an evaluated preview.

## Context

- Issue 6 asks for configuration that people readily understand, a simple
  table-oriented example and programmable selection.
- The combined survey `launcher/docs/research/harness-delivery.md` names the delivery
  modes and limitations that an inspection shows.
- The owner's `config.rhai` is the real routing to translate. Keep the
  translation outside the repository, because it is personal configuration.

## Done when

- The owner saw each sample and gave a reaction.
- The running log records each reaction in the owner's words, with what it
  means for the two designs that follow: `configuration-contract` and
  `skill-contract`.

## Notes

- The samples are mock-ups. They need not run, and polish is a defect.
- Show at least two shapes of `config.lua`: a plain table, and a function that
  decides from the context. The owner chooses between them, or asks for a
  mixture.
- Show a namespaced skill identifier and a bare Grove name in the samples. The
  design must settle both.
