# Installed skill sources are the default

When the settings name no `sources`, Ravel reads the skill sources in the
default skill directory of its installation. The installation record names that
directory, and Ravel's Homebrew formula writes the record. Explicit `sources`
replace the default, and `"sources": []` turns it off. The
[skill discovery specification](../specs/skill-discovery.md#default-skill-directory)
owns the record, the entries, the roots and the acceptance.

The owner decided that `brew install` and `brew upgrade` are the whole skill
setup. Homebrew's sandbox denies writes into `HOME`, so no setting or link there
can satisfy that decision. A formula can still link its sources into the
Homebrew prefix. Ravel therefore reads one directory that its installation
names.

## Consequences

- A package installation changes the sources of every `HOME` that uses the
  default. A separate `HOME` isolates the settings and the cache, not the
  default sources.
- Ravel knows no package manager. The record names a path, and each packaging
  writes its own.
- An upgrade removes the links of a formula during its installation. A launch
  in that interval delivers the other sources and refuses nothing. A mandatory
  file in an absent source still refuses.
- Ravel names two roots, `skills` and `craft`, for each default source. The
  users of a package with another layout name its source in their settings.
- Ravel still fetches, installs and updates no source, as
  [skill sources are local directories](skill-sources-are-local-directories.md)
  records.

## Considered options

- **Settings that name each source.** The owner rejected it, because each user
  would have to write them after `brew install`. Reopen if the main
  installation route has no shared prefix that packages can write.
- **Links in `~/.config/ravel/skills/`.** The design rejected them. Homebrew's
  sandbox cannot write in `HOME`, and Ravel refuses a real directory inside its
  configuration directory.
- **A prefix that Ravel derives from its executable path.** The design rejected
  it. Homebrew puts the Cellar inside its repository when that directory
  exists, so a keg does not always lie below the prefix. Reopen if a supported
  packaging cannot write the record.
- **A call of `brew --prefix`, the variable `HOMEBREW_PREFIX` or a compiled list
  of prefixes.** The design rejected them. The call costs a process and depends
  on `PATH`. Many launch environments do not set the variable. A list misses
  each custom prefix.
- **Explicit sources beside the default ones.** The design rejected them. A
  repeated ID would need a refusal or an override rule, and no consumer needs
  one configured source beside the installed ones. Reopen when such a consumer
  appears, with an override of a repeated ID as the first candidate.
- **The whole source as one root.** The design rejected it, because the agent
  would list every tree of a source, such as conformance fixtures and
  examples. Reopen if a package needs other roots, with a descriptor beside its
  entry as the first candidate.
- **A list of required default sources.** The design rejected it for now. It
  would make the upgrade interval refuse, but each user would have to write
  it. Reopen if a source that an upgrade removes for a moment causes measured
  harm.
