# Ravel notices

Ravel is licensed under Apache-2.0; the repository LICENSE holds the text.
The compiled worker carries the following third-party runtime.

## Bun 1.4.2, inside `ravel-policy`

The policy worker is compiled with Bun 1.4.2 and carries Bun's runtime inside
it: the `bun` executable from the `@oven/bun-<platform>` 1.4.2 package that the
build pins by digest. Bun is MIT-licensed and statically links further
libraries, among them JavaScriptCore and WebKit under the LGPL 2.
[`bun-LICENSE.md`](bun-LICENSE.md) is Bun's own statement of those components,
their licences and where to obtain the patched WebKit source. It is copied
unchanged from `LICENSE.md` at Bun's `bun-v1.4.2` tag.

