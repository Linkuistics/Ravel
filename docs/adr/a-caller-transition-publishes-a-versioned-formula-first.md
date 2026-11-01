# A caller transition publishes a versioned formula first

A Ravel major that a released caller refuses publishes first through a keg-only
versioned formula. The unversioned formula keeps the previous major until that
caller publishes a compatible release. A separate promotion then publishes the
same archives as unversioned `ravel`, with no new cut. The
[release-channel specification](../specs/release-channels.md) owns the
channels, the task input, the promotion and the fate of the versioned formula.

Ravel 2.0.0 is the first case. Grove 22.8 admits only stable 1.x from 1.1.0.
Its Homebrew formula depends on unversioned `ravel`, and its formula test
requires a 1.x version. Grove's coordinated-release specification orders the
Grove release and the promotion.

The trade-off is a transition formula and a second publication step against a
broken default installation. Keg-only `ravel@2` never becomes the default
command. A user who wants 2.0 before promotion must name its prefix. The release
tooling must carry an explicit channel through every step and resume mode.

The design rejected two alternatives. Direct unversioned publication of 2.0.0
would move every fresh Grove installation and every dependent upgrade to a
major that Grove 22.8 refuses. An earlier Grove release that admits 2.x adds its
own integration and release campaign before the Ravel cut.

Reopen this decision for a later major in two cases. No released caller refuses
that major, or every released caller accepts both majors before the cut. In
either case, publish the major directly as unversioned `ravel`.
