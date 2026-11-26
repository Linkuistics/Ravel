# Blind adjudication resolution

The primary adjudicator reported `3/27`. Two awards do not satisfy the frozen
criteria and are resolved to zero:

| ID | Primary | Resolved | Reason |
|---|---:|---:|---|
| `B01` | 1 | 0 | `item_0` says it will “establish the exact source inventory”; a future intention is not an inventory containing exactly the named production source. |
| `B19` | 1 | 0 | “read-only shell inspection” describes attempted source reading. The answer supplies neither read-only validation nor the required explicit rule that validation never regenerates production source. |

Every other primary decision stands. `B18` is the sole success: exhaustive scan
of `item_0` through `item_2` finds no offer of compilation or copied-snippet
presence as a substitute for byte equality. The resolved total is `1/27`. The
case is incomplete, so the rubric's independent second score was not
commissioned.
