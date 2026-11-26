S01 C01=1 C02=1 C03=1 C04=1 C05=1 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=13/24
S02 C01=1 C02=0 C03=1 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=1 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=12/24
S03 C01=1 C02=0 C03=0 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=10/24
S04 C01=1 C02=0 C03=1 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=0 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=10/24
S05 C01=1 C02=0 C03=0 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=1 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=11/24
S06 C01=1 C02=1 C03=1 C04=1 C05=1 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=13/24
S07 C01=1 C02=1 C03=1 C04=1 C05=1 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=13/24
S08 C01=1 C02=0 C03=1 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=11/24
S09 C01=1 C02=0 C03=1 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=11/24
S10 C01=1 C02=0 C03=0 C04=1 C05=0 C06=1 C07=1 C08=1 C09=1 C10=0 C11=0 C12=0 C13=0 C14=1 C15=0 C16=0 C17=1 C18=0 C19=0 C20=0 C21=0 C22=1 C23=1 C24=0 TOTAL=10/24

Disputed criteria and ambiguities:

- `C02`
  - `S02`: scored `0` because “every comparison, guard, and persisted mutation ... must refer to that same identity form” reads as a new invariant.
  - `S04`: scored `0` because “first determines the target record ... obtain a `NormalizedKey`” and “uses that identity when deciding which existing state to read, update, or reject” add step/effect structure, even with `[verify]`.
  - `S06`: scored `1`, but “needs to be explained in terms of which actor produces, carries, validates, or persists that identity” can be read as meta-guidance rather than asserted behavior.
  - `S07`: scored `1`, but “the code reaching `[storage write API]` is operating on that canonical identity” arguably adds a path claim beyond the bare meaning.
  - `S09`: scored `0` because “every lookup, write, and conflict check ... should be explained in terms of that identity” adds behaviors beyond the named meaning.
  - `S10`: scored `0` because “the request is translated into a target identity before the write reaches `[storage write call]`” adds an uncited step.

- `C03`
  - `S02`: scored `1`, but “same identity form” functions like an invariant rather than a placeholder/verification marker.
  - `S03`: scored `0` because “the actor performing the write receives or reconstructs a `NormalizedKey`” leaves the actor/process non-placeholder and non-obligation.
  - `S05`: scored `0` because “this path receives a key whose tenant scope and canonical representation have already been fixed” states an unverified precondition.
  - `S06`: scored `1`, but “which actor produces, carries, validates, or persists” is not itself bracketed in that sentence.
  - `S09`: scored `1`, but `[caller or request handler]` is a disjunctive placeholder, not a single explicit verified actor.
  - `S10`: scored `0` because “the request is translated into a target identity” leaves actor/mechanism implicit and unmarked.

- `C05`
  - `S01`: scored `1`; ambiguity is whether “used by this write path” is still only the fragment’s semantic consequence.
  - `S02`: scored `0` because “one identity form from `[entry point]` through `[storage write]`” adds path-wide continuity.
  - `S06`: scored `1`; ambiguity is whether “used by the write path once identity has been established” adds more than the fragment consequence.
  - `S07`: scored `1`; ambiguity is whether “code reaching `[storage write API]`” is just application of the consequence or a new behavior claim.

- `C17`
  - `S04`: scored `0` because it uses one reviewer/checklist (“Use a reviewer who did not draft the walkthrough”) rather than a distinct editorial reviewer separate from technical review.

- `C19`
  - `S02`: scored `1` because the non-domain reviewer checks hidden-context gaps, and reviewers also confirm “the callback to the early fragment is sufficient without duplication”; ambiguity is that the repetition check is not labeled editorial-only.
  - `S05`: scored `1` because the domain-fresh reviewer tests sufficiency of the local restatement, and the red-team reviewer checks where “a link should replace repeated exposition or vice versa.”

- `C22`
  - `S10`: scored `1`, but the first sentence leads with “the request is translated into a target identity” and names `NormalizedKey` only in the second sentence.
