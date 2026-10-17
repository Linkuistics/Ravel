# Ravel.simplify-and-componentize — brief

## Goal

Make Ravel easier to understand and maintain by reassessing its component
boundaries using the learnings from the two named Grove workstreams. Extract
useful independently reusable capabilities where the analysis justifies them,
and provide high-quality documentation of the components, complete tool,
configuration and project.

## Done when

- Usable current learnings from both upstream workstreams have been absorbed
  before the component breakdown is analysed. The adopted guidance, pinned
  sources and evidence limits are identified. Upstream completion is not a
  prerequisite, as the human directed on 2026-10-14.
- The resulting design explains each component's responsibility, interface,
  dependencies and composition, including the reasons for language and package
  boundaries. It improves the ability to reason about and change components
  separately while retaining Ravel's required behaviour.
- Useful generic components selected by the analysis can be used independently
  by other projects through documented interfaces. Their packaging is chosen
  on the evidence; no fixed crate count or repository arrangement is prescribed.
- Existing owner configuration continues to work unchanged, including
  TypeScript policies, SDK usage, settings, discovery and behaviour. CLI and
  session semantics remain compatible.
- A book (user guide) teaches Ravel and its components to a software developer
  with no background in this context. It supports operating Ravel, authoring
  configuration, understanding the architecture, following the code, using
  extracted components and developing/delivering the resulting project.
- The agreed acceptance checks establish compatibility, component contracts,
  complete-session behaviour and practical usability of the documentation.

## Decomposition

`plan-k1` established the requirements, compatibility constraints and agreed
acceptance seams. `upstream-guidance-k2` consumes pinned current upstream
guidance with its limits. `component-contracts-k3` establishes current behavioural
obligations and responsibility contracts in `docs/components.md`, with scoped
source/operation evidence and editable views. `result-job-boundary-k4` compares
the consumer's joint result/ending/cleanup protocol against materially different
arrangements, including the unchanged baseline and regrouping. It preselects no
architecture. Its scenarios, preservation frame, improvement rows and numeric
bounds live in that task.

K4 closed that campaign with the existing implementation retained, an
undetermined goal and an unmet/incomplete comparison; no target, migration or
simplification gain is selected. The funded baseline/regrouped I/C/W family was
read, but full signal/resource ownership and native entry semantics, six-month
support cost and working migration remain unresolved. Its current invariant is
recorded in `docs/specs/result-job-completion.md`; candidate views are explicitly
unselected. `result-job-transfer-k6`, before the book agreement, is a separately
bounded current-source ownership/entry observation. It carries k4's review and
spent frame without reopening full contracts or resetting comparison counts.
Later comparative selection needs prospective funding with that history and
G1–G5, not a fresh claim of completion.

K6 derived the current acquisition inventory before extending the views. A
successful `Selected` retains the live guard/source/prepared workspace and
enables limited internal channel/job association. Complete settlement on every
failure path is unsupported: source admits lost runner cancellation after a
wait/reap error, unchecked worker reap before successful selection, unreported
partial-directory cleanup and pre-exec handoff/failed-exec without reclaim.
Direct native entry supplies child state but retains host handlers/SIGCHLD
repair; the CLI's executable lifetime is distinct from a returning host.
`docs/result-job-source-evidence.md` supplies exact evidence, scope limits and
cheapest discriminating observations; the current completion spec, component
guide, runner guide and views carry the qualified contract. These are source
observations, with one fresh exception-path read, not observed runtime failures,
an extraction, selected target or recurring improvement. No promise is waived.
A human scope/funding continuation, `completion-failure-scope-k7`, precedes the
book agreement to establish the failure-contract work and native lifetime needed
for subsequent design.
It must carry k4's frame/history and G2–G5/R-W debt rather than reopen comparison
or implementation under this observation's source-only authority.

K7's human agreement establishes the first native consumer as a one-session
launcher CLI that settles its launch and exits; the persistent runner is a
separate component with its own continuing-host contract. Address all four
known acquisition/failure gaps before selecting new component boundaries,
using the agreed CLI/real-policy/fake-job, transport/real-process and runner/PTY
boundaries plus justified private worker-wait and partial-acquisition failure
controls. This is correctness work on the incumbent, not a supported target,
comparison restart, extraction or observed independent-use result.

The correctness phase starts with `completion-failure-contract-k8`, before the
book agreement. Its aggregate effort caps are D≤90 minutes (including 20 minutes
for closure), RC≤30, P≤240, E≤120 and X=0, with zero production disruption.
K8 establishes the incumbent's required local repair contract and discriminating
check plans in the current completion specification and
`docs/completion-failure-design-evidence.md`. The proposed corrections retain
consumed cancellation in the existing error outcome, gate worker admission on
confirmed reap, assign partial cleanup to allocators, and retain a checked
terminal lease through failed spawn. Product source is unchanged and behavior
is unobserved.
The terminal mechanism is conditional on no competing foreground transfer or
unrelated reuse of the recorded group identity through handback; CLI lifetime
alone establishes neither. Started is spawn-reported success, not confirmed exec.
Entry waitability is required before the worker as well as the managed child;
unchecked signal/mask and terminal operations are still conformance premises.

K8 conservatively charges D≤40 and RC≤10 minutes including closure/commit,
P=E=X=0. Remaining correctness ceilings are D≤50 (20 still reserved for phase
closure), RC≤20, P≤240, E≤120, X=0. The next leaf,
`cancellation-failure-transfer-k9`, is planning-only, allocated D≤10/RC≤5,
P=E=X=0, before `book-structure-k5`. It plans a complete C1 runner-to-front
error-cancellation slice; P≤45/E≤25 are earmarked prospectively for its subsequent
implementation/check mandate, not charged or spent by planning. Consuming that
earmark in full leaves P≤195/E≤95 for later repairs. Worker reap admission,
partial cleanup, failed-exec/terminal recovery and native entry checks remain
required before selecting component boundaries or reaching the book agreement.
Grow only the next precise useful slice; no full speculative repair tree,
package, target or migration is selected. Record charged work and pass remaining
limits forward; reaching a bound with unfinished work calls for a precise
remainder and renewed human funding, never a reset.

K9 completed that planning without product edits or behavioral execution. Its
single useful increment is `cancellation-failure-transfer-k10`: runner error
observations and the front's no-ending conversion/settlement land together,
preserving first-signal precedence and original/secondary diagnostics. Fixed
private wait-error and front-transfer cases precede its implementation; actual
checked terminal recovery remains C4 even where C1 checks recovery-error
composition with a private reported-outcome control. K9 charged conservative
D≤10/RC≤5 including closure/commit, P=E=X=0. Correctness total charged bounds
are now D≤50/RC≤15/P=E=X=0, leaving D≤40 (20 reserved for phase closure),
RC≤15/P≤240/E≤120/X=0. K10 alone may consume its earmarked P≤45/E≤25; full
use leaves P≤195/E≤95. Its closure must record charges and place the next
precise failure-work continuation before the book agreement, normally C2
planning. Worker admission, partial cleanup, checked terminal/failed-exec
recovery and native-entry checks are still required. K4/K6 history and the
G2–G5/R-W debt above remain unchanged; no target, extraction or gain follows.
K10 corrects and checks C1 on the incumbent: consumed runner cancellation crosses
observation/reap errors and reaches no-ending front settlement, with original
and secondary diagnostics. `docs/completion-failure-design-evidence.md` and its
linked measured receipts separate private reported failures from real-process
regressions. All fixed private cases and CLI/workspace checks passed; actual
checked terminal recovery remains C4. K10 conservatively charges P≤30/E≤5/RC≤5
minutes including source/tests/evidence, one reviewer, tree/retirement/commit;
no new D/X spend. Correctness cumulative charged bounds are now
D≤50/RC≤20/P≤30/E≤5/X=0, leaving D≤40 (20 reserved for phase closure), RC≤10,
P≤210/E≤115/X=0. The unused portion of k10's allocation is not fictional spend.

`worker-reap-admission-k11` is the next precise planning continuation before the
book: gate selection success on confirmed policy-worker reap and retain original
failure plus teardown/cleanup/policy diagnostics. It has D≤10/RC≤5, P=E=X=0,
and prices its subsequent complete repair from the remaining P/E before growth.
C3 partial acquisition, C4 failed-exec/checked recovery and native entry remain
mandatory. There is no reopened comparison, target, extraction or root completion.

K11 completes the C2 planning as one evaluator-to-choice/front admission slice,
`worker-reap-admission-k12`, before the book agreement. Actual worker stop/wait
precedes the private reported-wait substitution; confirmed wait (including
nonzero exit) and checked post-evaluation private-directory removal gate success.
The fixed cases protect original refusal/deadline/protocol/output and cancellation
precedence, operational reports, separately bounded policy output, and no
resolution/Selected/managed start on failed admission. Narrow late-cancellation
preservation in choice is included because its current override drops the
original refusal. Partial preparation cleanup stays C3; no fault seam or runtime
behavior was constructed/executed by this planning.
K11 conservatively charges D≤10, RC=0, P=E=X=0 including closure/commit;
correctness cumulative bounds are D≤60/RC≤20/P≤30/E≤5/X=0. Remaining
D≤30 (20 reserved for phase closure), RC≤10/P≤210/E≤115/X=0.
K12 prospectively allocates P≤50/E≤25/RC≤5, D=X=0 for the complete repair,
controls, measured red/green and protected/workspace checks, evidence and closure.
Full use would leave D≤30/RC≤5/P≤160/E≤90/X=0; allocations are not spend.
K12 records actual charges and grows only the next precise supported continuation
before the book, normally C3 planning from the unreserved D≤10. C4/native-entry
work and the unchanged root/G2–G5/R-W debt remain; no parent completion is earned.

K12 now corrects and checks C2 on the incumbent: confirmed worker reap and
checked post-evaluation private-directory removal gate evaluation success before
resolution/Selected/managed start. Original conversation metadata, bounded policy
output and all operational reports survive cancellation/overflow. The linked
C2 receipts in `docs/completion-failure-design-evidence.md` distinguish physical
wait/removal from substituted reports. All 35 C2 parameters/instruments, 12 CLI
cases, protected C1 controls and all 177 workspace cases pass. One reviewer found
three actionable gaps, each fixed with executable controls; no second review or
architectural selection was needed. C3 partial preparation/identity cleanup,
C4 checked recovery/failed exec and native entry remain required.

K12 conservatively charges P≤40/E≤5/RC≤5 minutes including tests/evidence,
one reviewer and coordination/retirement/commit, D=X=0. Correctness cumulative
charged bounds are D≤60/RC≤25/P≤70/E≤10/X=0; remaining D≤30 (20 reserved
for phase closure), RC≤5/P≤170/E≤110/X=0. Its unused allocation is not spend.
`partial-acquisition-cleanup-k13`, before the book agreement, is the next precise
planning continuation, with D≤10/RC=P=E=X=0. It prices a complete C3 repair
from remaining P/E before growth and preserves the 20-minute closure reserve.
No fresh funding, parent completion, target or gain is inferred.

K13 completes C3 planning as one complete allocator-to-consumer increment,
`partial-acquisition-cleanup-k14`, before the book. Local ownership and recorded
identity span partial preparation, successful transfer and explicit cleanup in
confined, session and worker directories. Private controls cover original
acquisition plus cleanup reports, actual moved/replaced/symlinked directories,
unknown identity, destructor disarming and actual choice/front composition.
Worker failures prevent evaluation/resolution/Selected; channel allocation
occurs after Selected and prevents managed start/result instead. Source snapshot
identity checks establish no atomic concurrent-mutation guarantee. No control
or behavioral execution occurred during planning; C3 remains required work.

K13 conservatively charges D≤10/RC=P=E=X=0 including verification/retirement/
commit. Cumulative correctness charged bounds are D≤70/RC≤25/P≤70/E≤10/X=0;
remaining D≤20 (all reserved for phase closure)/RC≤5/P≤170/E≤110/X=0.
K14 prospectively allocates P≤60/E≤30/RC≤5, D=X=0 for the complete repair,
private controls, immutable measured receipts, protected/workspace checks and
closure. Full use would leave D≤20/RC=0/P≤110/E≤80/X=0; allocations are not
spend. K14 records actual charges and prices only the next precise supported
C4/native continuation from the existing k8 design and remaining P/E. No
unreserved planning D remains; any further discovery/design/review requiring
funding earns a precise human scope/funding continuation before the book,
without silently consuming the closure reserve. No parent completion or new
target/comparison/extraction/migration/gain is earned.

K14 now corrects and checks C3 on the incumbent. All three allocators retain
recorded identity through partial preparation, successful transfer and explicit
cleanup; unknown/replaced/moved ownership is reported without unsafe deletion
or destructor retry. Original classifications and typed channel I/O causes
survive cleanup context. Physical residual fixtures, real ENOENT, actual
choice/front admission and separately reported wait/removal controls are
recorded in the C3 evidence. All 97 C3 cases, protected CLI/C1/C2 checks and
283 workspace cases pass. One reviewer found three actionable gaps, each
verified and corrected through executable controls; no second review occurred.
Snapshot comparison supplies no atomic concurrent pathname-mutation guarantee.

K14 conservatively charges P≤35/E≤10/RC≤5 minutes including instruments,
repairs/evidence, measurements/regressions, the one reviewer, coordination,
retirement/commit; D=X=0. Correctness cumulative charged bounds are
D≤70/RC≤30/P≤105/E≤20/X=0. Remaining D≤20 (all reserved for phase closure),
RC=0/P≤135/E≤100/X=0. The unused k14 allocation is not spend.
`terminal-entry-funding-k15`, before the book, is the precise human funding
continuation: C4's existing k8 design can be priced at P≤65/E≤35/D=X=0, but
its needed review/coordination RC≤5 requires explicit renewal. No new cap,
allocation or authorization is inferred. Native-entry work still needs its
supported exact slice; any further discovery/design/review funding must be
agreed without spending the closure reserve. C4/native entry, root delivery
and documentation remain live, so no parent completion is earned.

K15's human agreement now renews that bounded C4 continuation: “yes” to
P≤65/E≤35 from existing balances and raising the phase RC cap from 30 to 35
minutes, sharing the added five between agreement closure and C4's one narrow
reviewer/coordination/closure. D≤90/P≤240/E≤120/X=0 remain unchanged.
K15 conservatively charges RC≤3 including recording/tree/verification/retirement/
commit, D=P=E=X=0; cumulative correctness bounds are now
D≤70/RC≤33/P≤105/E≤20/X=0. Remaining are D≤20 (all phase-closure reserve),
RC≤2/P≤135/E≤100/X=0. `failed-exec-terminal-recovery-k16`, before the book,
allocates P≤65/E≤35/RC≤2, D=X=0 for k8's existing terminal repair, fixed PTY and
recovery-report controls, protected CLI/C1/C2/C3 checks, evidence and closure.
Full use would leave D≤20/RC=0/P≤70/E≤65/X=0; allocations are not spend.
Its actual source/toolchain verification belongs to implementation; material
redesign or further review requires explicit funding. Lease/identity premises
stay conditional and Started remains spawn-reported success. Native-entry
waitability/signal/mask work still needs its precise separately priced
continuation before the book; no unreserved discovery/design is funded and the
closure reserve cannot fund it. K4/K6 history and all root obligations below
remain unchanged; no parent completion or comparative gain is inferred.

K4's reference/frame/spend, k6's separate spend, G2–G5 and R-W runtime/lifecycle
debt remain. The k7 agreement contains the detailed scope, enabling points and
test expectations. Correctness repairs precede any later comparative target
and migration selection; the root's remaining delivery and documentation goals
still require their own supported work.

`book-structure-k5` settled the book with the human on 2026-10-16. Its contract
is `docs/book/contract.md`: the structure brief (reader, outcomes, twelve
chapters in four parts with their purposes, entry routes, emphasis, exclusions,
term risks, coverage inventories, examples and figure needs), the prose
contract, format and media (mdBook, with C4, UML and SysML figures rendered by
pinned tools), and the reader checkpoints. By the human's choice the book is
drafted against this workstream's current code before any further component
work. `ravel-book-k55` holds it, planned lazily in two halves separated by the
owner's first stand-in read; chapters 1–6 describe the compatibility-protected
surface. Chapter 10, on using an extracted component, belongs to the work that
delivers such a component, together with its interface and consumer.

`component-work-scope-k57`, after the book node, is the requirements step for
the remaining component work, informed by where the book's reads found the code
hard to follow: whether to resume a comparative redesign; which component, if
any, becomes independently usable, with its interface, consumer, packaging and
chapter 10; whether any consumer needs a persistent-runner contract; and
delivery of this workstream's corrected code (merge, release and the book's
first publication). Each implementation increment it leads to must retain
working configuration and complete sessions; an extracted capability and the
interface/consumer that makes it independently usable belong together.
Packaging, additional extractions and cleanup become separate increments only
where each stopping state is useful and verifiable, and any component change
updates the book chapters it touches. Cut their groves and detailed leaves when
those boundaries are known, rather than assigning a crate or language per leaf
now.

## Pointers

- Existing behavioural contract: `docs/specs/session-execution.md` and
  `docs/session-cli.md`.
- Required failure/native-entry correction contract:
  `docs/specs/result-job-completion.md`; exact source and discriminating plans:
  `docs/completion-failure-design-evidence.md`. Required behavior and unexecuted
  design remain distinct from the current acquisition evidence.
- Existing architectural decisions: `docs/adr/policy-evaluation-precedes-the-launch.md`,
  `docs/adr/the-launched-child-is-a-job.md` and
  `docs/adr/ravel-supervises-the-session.md`. These explain current obligations;
  their internal design choices can be reassessed within the agreed constraints.
- Configuration surface: `docs/owner-configuration.md`.
- Generic runner boundary: `docs/local-runner.md`.
- Upstream decomposition work:
  `/Users/antony/Development/grove.improve-initial-design-decomposition-skill`.
  Its first authored method is under
  `plugins/linkuistics/skills/decomposition-design/`; final trials, revisions
  and integration must be checked before relying on it.
- Upstream documentation work:
  `/Users/antony/Development/grove.gh-issue-14-documentation-skills-for-editorial-pipeline`.
  Consume its resulting guidance, evidence and declared limits, including
  requirements/structure planning, draft, copy-edit, art, proof and reader testing.
- Reusable development work uses the existing root `Taskfile.yml`.
- The book's contract: `docs/book/contract.md`.

## Agreed acceptance checks

Use the existing public CLI with real TypeScript policy evaluation and fake
launched programs to check unchanged configuration, routing, refusals and
session results. Keep real model services out of these regression checks.

Use the existing real-process and terminal-test boundaries for cancellation,
descendant cleanup, reaping and terminal recovery where the redesign touches
those responsibilities. Verify that complete Ravel sessions still satisfy their
contract after components are recomposed.

Exercise each extracted component through its consumer-facing contract, with
controlled dependencies only where an existing or justified seam is needed.
Show that a consumer can use it without depending on Ravel-specific policy or
session conventions unless those are genuinely part of its contract.

Validate documentation examples against the resulting interfaces and arrange a
short human read using practical tasks: configure and run a session; explain the
components and follow one session through the code; use an extracted component.
Determine the structure, admitted media and viewing environment using the
upstream guidance. Mechanical document checks support this read rather than
establishing comprehension by themselves. The book contract now fixes these:
examples run through the CLI/real-policy/fake-program seam, and the human chose
to hold the read themselves as a labelled stand-in, once after Parts I–II and
once after Parts III–IV, with the extracted-component task read with chapter
10. Whether a developer new to Ravel can use the book remains a named unknown.

The human confirmed these acceptance checks and shared understanding of the
requirements in `plan-k1` before the design and implementation work was cut.

## Notes

The primary problem is comprehensibility, not a shortage of features. TypeScript
removal, merging the Rust crates, additional crate splitting and extraction are
design possibilities to evaluate against the contract; none is preselected.
Do not expand the feature surface merely to justify a generic abstraction.

The human expects Rust to be capable of implementing the tool and needs the
purpose and role of the TypeScript component explained. Documentation should
expose the subject model and reasons for boundaries before asking readers to understand
language-specific internals. The reader profile and structure are now in
`docs/book/contract.md`.

On 2026-10-14, during `component-contracts-k3`, the human specifies the
documentation deliverable as a book (user guide) for a software developer with
no background in this context. Teach the problem, vocabulary, actors and session
lifecycle before configuration and implementation internals. Do not infer Rust,
TypeScript or diagram proficiency from general development experience. The
book-structure agreement settled those assumptions, chapter order,
learning/direct entry routes, media and viewing environment on 2026-10-16
(`docs/book/contract.md`).

On 2026-10-14 the human changed the upstream prerequisite: extract usable
guidance from the workstreams' current state instead of waiting for completion.
The intake below supersedes the wait requirement from `plan-k1`; the existing
configuration, session and acceptance constraints remain in force.

On 2026-10-15, answering `native-entry-campaign-funding-k41`, the human said
verbatim “I'm not actually concerned about the budget - just do what is
required.” Budget caps, allocations and reserves (D/RC/P/E) therefore no longer
gate required work: sessions do what the agreed contract requires without
cutting funding-question leaves, and still record actual charges where a
charter asks. Zero production disruption (X=0) is not a budget and still binds;
a producer still cannot narrow acceptance, and scope choices remain the human's.

On 2026-10-17, during read 1 (`book-read-one-k70`), the human said verbatim:
“I intend to add a global instruction to use ADE-STE100 for all LLM
communication (such as when you talk to me here) as well as in all
documentation. Apply that rule now.” Thus, ASD-STE100 Simplified Technical
English (Issue 9) applies to messages to the human and to each document that
a session writes or changes. The book's contract, section 2, records how the
rule applies to the book. That record includes British spelling under STE
Rule 1.14 and the local copy of the specification, which stays out of the
repository. On 2026-10-18, during `book-first-half-k123`, the human said:
“The skill I am going to adopt does not use the word list, only the rules.”
Thus, STE means its writing rules. The STE dictionary does not apply.

## Current upstream guidance

### Provenance and standing

These are pinned source snapshots, not declarations that either upstream
workstream finished. Recover a named file with `jj file show -r <commit> <path>`
in its source checkout; later design work uses these snapshots unless it
explicitly records a newer intake. No upstream survey or trial is rerun here.

| Source checkout | Consumed commit | Guidance actually used |
|---|---|---|
| `grove.improve-initial-design-decomposition-skill` (absolute path in Pointers) | `0cce939c8447d2ca45f5b97519195d89ac7ff86b` | `plugins/linkuistics/skills/decomposition-design/SKILL.md`; its `references/campaign.md`, `existing-system.md`, `views.md`, `catalogue/guide.md` and `catalogue/index.md` |
| `grove.gh-issue-14-documentation-skills-for-editorial-pipeline` (absolute path in Pointers) | `7ca78f01b4d0e8ec3d19cafaaf2f4ae51d75b246` | `docs/specs/documentation-guidance.md` §§1–2, 4–8; `plugins/grove/skills/grove/references/editorial.md`; the root requirements and the evaluation reports identified below |

The decomposition skill calls itself a candidate method authored for trials;
its complete loop and existing-system path are new synthesis, untested. The
catalogue index reports schema revision 1 and input digest `4b2f7e4615bb8221`.
That is a navigation identity, not a quality verdict. Catalogue admission,
trial exposure and effectiveness are different properties. This intake uses
its vocabulary and selection procedure; no transformation entry has been
qualified for Ravel yet.

The live upstream `trial-existing-system-k45` and `guidance-revision-k24` still
owe execution and incorporation of trial findings. Their handoffs explicitly
say that approved instrument designs establish no qualified capture or scored
run. The revision leaf also records known obligation-accounting defects in
`state-the-compatibility-promise` revision 8. Do not copy that entry's accounting
as authoritative; use the scoped preservation definition and Ravel's promises.
This intake does not claim to have audited the full evidence register, all
catalogue entries or the underlying literature.

The editorial source has an agreed design but live `guidance-increments-k9`,
`revised-trials-k10` and `sample-reading-k11`. The installed editorial family
reference and four stage skills are byte-identical to those source files at
intake; that does not mean the design's proposed additions have been delivered.
Use the pinned design directly for the reader/contract additions. The
decomposition skill is not in this session's installed skill catalogue, so use
its pinned source rather than substituting the older local design skill.

The historical `docs/evaluations/editorial-pipeline-pilot/README.md` supports a
bounded four-stage artifact result; proof was retained as a fallback and was
not scored. The separate `docs/evaluations/writing-code-walkthroughs/README.md`
reports an unmet improvement endpoint, a regression, incomplete samples and
unestablished transfer. Neither report proves human comprehension or the
effectiveness of the new editorial guidance. The current guidance workstream's
model comparison and human sample read remain future evidence.

### Decomposition practice for Ravel

Start with required behaviour and supported consumers. Separate observation
from obligation: unchanged TypeScript policies, SDK use, settings, discovery
and CLI/session outcomes are already required here. Preserve unknown relied-on
behaviour while investigating it. Static graphs expose dependency paths; they
do not establish runtime wiring, hidden effects or requirement authority.

Record logical responsibilities before mapping them to Rust crates, the policy
worker, packages or processes. For each responsibility name the decision it
hides, its interface, state/effect ownership, guarantees, caller assumptions,
dependencies and failure composition. Explain the purpose of TypeScript and
the two Rust crates from their contracts. A rewrite in Rust must still preserve
executable TypeScript owner configuration; a generic-looking helper earns
extraction only when an independent consumer can use its actual contract.

Phrase each concern as a concrete change or check that needs knowledge of
several parts, with the reason. The catalogue's caller obligations, logic and
effects, shared decisions, split invariants, dependency tangles, implicit
composition, over-splitting and transition obstacles are classification aids,
not findings about Ravel. Discover the evidence before choosing an entry.

Compare materially different ways to remove an evidenced entanglement, keeping
the unchanged baseline available and considering regrouping as well as
splitting. Change representation, state ownership or coordination where a
boundary move alone transfers the burden to callers. Evaluate the local
reasoning burden and composition obligations together. Component count,
smaller files and shorter explanations establish no improvement. Distinguish
observed evidence, design estimates and unknowns.

Keep target and migration designs separate. A target can be supported while
its route remains unknown. Each retained intermediate state names its
composition, consumers, ownership, preserved behaviour, practical usefulness,
checks, recovery and temporary machinery with removal conditions. Preserve
the existing CLI/real-policy/fake-program and real-process/terminal seams;
protect reference expectations from the redesign being judged.

Before a comparative campaign, freeze normal, failing and independent-change
scenarios, improvement rows, preservation scope/reference/equivalence and any
authorised exceptions. Set numeric discovery and comparison bounds before
generating options. Use the source campaign, records, iteration and evidence
references when those steps are reached; catalogue reads and options share
the campaign's budget. At a bound, hand forward the precise uncertainty.
Do not substitute this intake for a complete application of the method or
claim that an unread alternative was rejected. Assess behavioural support,
improvement and comparison completeness separately. Design support names
checks still owed; it is not observed runtime preservation.

Use a textual obligation table before adding diagrams. Keep calls, shared
state, configuration and ordering distinct; say what is observed, inferred or
proposed. Trace a normal and failing session through the owners, and check
ordering, cleanup, participant knowledge and partial failure wherever they
matter. A composition picture is not evidence that its participants can meet
their local obligations.

### Documentation practice for Ravel

The documentation must let readers configure and run a session, explain the
components and follow a session through the code, use an extracted component,
and develop/deliver the resulting project. Establish their expertise in the
domain and in Rust, TypeScript and diagram notation separately. Teach the
subject's actors, actions and consequences before language-specific internals.
Keep the complete tool, configuration author, generic-component consumer and
project contributor perspectives explicit.

Once the component contracts are stable enough, cut a human structure-agreement
task before editorial drafting. The human decides the substantive section
order, audience, emphasis and exclusions. Its document contract records page
purposes and learning/direct entry routes, terminology risks, subject-derived
coverage inventories where coverage is claimed, example and figure questions,
a named prose contract, admitted formats/media and actual viewing environments.
Choose format against those tasks; recheck dated tool facts when the choice
depends on them rather than copying upstream renderer-version advice.

Then use the existing lazy draft → copy-edit → art → proof pipeline with one
owner per class. Draft owns structure, teaching and technical/normative truth;
copy-edit owns wording under the prose contract and routes missing teaching
back to draft; art owns figures and their text; proof checks the whole set and
entry routes. Correction runs stay bounded under the editorial procedure.
Validated examples and mechanical checks support these stages.

Figures answer named reader questions at explicit levels, carry meaningful
relations and accessible textual information, and teach notation only where
the reader needs it. Tool-rendered figures retain editable source, generated
SVG, pinned renderer and a verified regeneration recipe in the Taskfile;
figures wholly within the page's own bytes need no separate asset. Inspect the
actual declared viewing environment as well as source/render agreement.

Schedule the agreed short human reader checkpoint outside the four stages,
after proof, using practical find/explain/use tasks and stated entry surfaces.
Record the reader's words, difficulties, reported coverage and unknowns.
Human acceptance, observed task performance, model proxies and mechanical
results remain separate evidence. An early comprehension failure stops that
read and routes a bounded correction to its owning stage.

K16 now corrects and checks C4 on the incumbent. The runner retains an owned
terminal lease and private attempted-group pipe through the spawn outcome,
checks recovery and keeps original errno/first consumed cancellation plus each
secondary report through actual front settlement. Failed-spawn recovery never
signals the recorded identity and remains conditional on exclusive foreground
authority and no unrelated group-ID reuse through handback. Real PTY receipts
separate handoff/modes/waits, substituted reports and pre-exec signal death;
Started remains spawn-reported success. The unchanged k14 product fails the new
public CLI foreground test. Eight broken controls, 22 ordinary PTY cases and
48 error/three absent-publication consumer branches discriminate the repair.
One narrow reviewer found a fixture cleanup gap, fixed and checked through
controller/barrier/fixture timeout controls. A protected C1 stopping-order
regression was corrected without weakening its test. All 290 workspace cases,
136 focused session cases, 11 protected runner cases and 13 CLI cases pass;
each of the prior 283 workspace identities is individually confirmed. The
C4 evidence document and linked frozen receipts retain the measured domain,
failed attempts, source support and authority limits.

K16 conservatively charges P≤35/E≤10/RC≤2 minutes including instruments,
source/evidence, measurements, the one reviewer and coordination/retirement/
commit; D=X=0. Cumulative correctness bounds are D≤70/RC≤35/P≤140/E≤30/X=0.
Remaining are D≤20 (all phase-closure reserve), RC=0/P≤100/E≤90/X=0. Its unused
allocation is not spend. `native-entry-funding-k17`, before the book, is the next
precise human continuation: executable-entry capture, worker/managed-child
waitability, ignored/blocked signals and checked handler/mask transitions still
need a complete priced slice and renewed planning/review authority. Its proposed
D≤10/RC≤5 renewal is not authorization; it does not consume the closure reserve
or reset any cap. Root delivery, comparison history and G2–G5/R-W debt remain;
no parent completion, extraction, target/migration or simplification gain follows.

K17's human answered “approved” to the native-entry renewal: raise the phase
D cap from 90 to 100 minutes, adding D≤10 for bounded next planning while
preserving the original D≤20 phase-closure reserve, and raise RC from 35 to 40,
adding RC≤5 shared between agreement closure and next planning/review/coordination.
P≤240/E≤120/X=0 and zero production disruption remain unchanged. K17 charges
conservative RC≤3 including recording/tree/verification/retirement/commit,
D=P=E=X=0. Cumulative correctness charged bounds are now
D≤70/RC≤38/P≤140/E≤30/X=0; remaining D≤30 (D≤10 available for this planning
slice and D≤20 reserved for phase closure), RC≤2/P≤100/E≤90/X=0.

`native-entry-correctness-k18`, before the book, is that single planning
continuation. Its D≤10 includes closure; P=E=X=0 in planning. The remaining
RC≤2 is shared across planning/review/coordination and closure, never reset per
leaf. It must price a complete independently useful entry/runner/front repair,
real-process/PTY discriminators and protected C1–C4/CLI/runner/workspace checks
from actual remaining balances before commissioning implementation. Preserve
capture before runtime changes, worker and managed-child waitability,
ignored/blocked signals, checked handler/mask transitions, auxiliary-thread
eligibility and late joining. Material new mechanism or work outside the
allowance needs a precise human funding/scope continuation before the book;
the closure reserve cannot fund it. Allocations are not spend. The one-session
CLI consumer, C4 authority premises, k4/k6 comparison history, G2–G5/R-W debt
and all root delivery/documentation obligations remain unchanged; no parent
completion or comparative gain follows from this agreement.

K18 completed the bounded current-source/toolchain planning read without product
edits or behavioral execution. Pre-runtime entry capture and output-thread
shielding already exist; their unchecked failure paths, runner-only late SIGCHLD
repair, discarded Started mask failure and separate selection-failure settlement
still require a coherent checked transition design. K18's fixed discriminator
matrix retains worker/child waits, ignored/blocked signals, direct versus
terminal INT, transition/after-reap/no-ending cancellation and protected C1–C4.
It establishes neither runtime conformance nor a complete supported producer
price. Existing shielding is a mechanism to preserve and check, not an absent
feature to reinvent. SA_NOCLDWAIT exec reachability remains an explicit question.

K18 conservatively charges D≤10/RC=P=E=X=0 including verification/retirement/
commit. Correctness cumulative charged bounds are D≤80/RC≤38/P≤140/E≤30/X=0;
remaining D≤20 (all phase-closure reserve)/RC≤2/P≤100/E≤90/X=0.
`native-entry-transition-funding-k19`, before the book, is the precise human
continuation. It proposes additional D≤20/RC≤10 for checked-transition design,
complete pricing and shared review/coordination/closure, raising caps to
D≤120/RC≤50 while leaving P≤240/E≤120/X=0 unchanged. This is not authorization
or spend; no producer is commissioned. A complete entry-to-settlement price
must cover source, finite owner-local controls, red/green, protected CLI/C1–C4/
runner/workspace checks, evidence and closure from actual balances. No reserve
is borrowed. Native entry, k4/k6/G2–G5/R-W debt and all root delivery/documentation
obligations remain live; no parent completion or comparative gain follows.

K19's human answered **“Approve the bounded renewal (Recommended)”** to adding
D≤20 for checked native-entry transition design and complete pricing, and
RC≤10 for shared review/coordination/closure. Approved phase caps are now
D≤120/RC≤50/P≤240/E≤120/X=0; the original D≤20 phase-closure reserve and zero
production disruption remain. K19 charges conservative RC≤3 for agreement,
recording/tree/verification/retirement/commit, D=P=E=X=0. Cumulative correctness
charged bounds are D≤80/RC≤41/P≤140/E≤30/X=0. Remaining are D≤40 (D≤20 for
this design/pricing slice and D≤20 exclusively phase-closure reserve), shared
RC≤9/P≤100/E≤90/X=0. The approval, allocations and charges are distinct.

`native-entry-transitions-k20`, before the book, is the exact bounded
design/pricing continuation. It prospectively allocates D≤20 (12 for checked
ownership/transitions, 5 for exact cases/private controls and 3 for complete
pricing/tree/closure) and RC≤4 for at most one focused adversarial transition
review and its coordination/closure; P=E=X=0. Full use leaves D≤20 exclusively
phase-closure reserve and shared RC≤5/P≤100/E≤90/X=0. Actual charges govern
later commissioning. The design must settle the k18 entry-to-settlement
ownership/failure joins, freeze exact discriminators through the already agreed
CLI/real-process/PTY seams, resolve executable versus private SA_NOCLDWAIT
reachability, preserve C1–C4 and price the complete useful producer against
actual balances. Existing capture/shielding are mechanisms to preserve and check.
No implementation is commissioned from unused P/E balances; unsupported work
earns a precise continuation before the book without reserve borrowing.

K4/k6 comparison history, G2–G5/R-W debt, the conditional C4 authority premises,
the separate returning-host contract and all root delivery/documentation
obligations remain unchanged. K20 and the book remain live, so no parent
completion, target/migration/extraction or comparative gain is earned.

K20 now records the checked native-entry design in the existing completion
specification and docs/native-entry-transition-design.md, with editable views.
The front scope survives selection failure, checked watching precedes both
children, reader shielding lasts through thread exit and a failed Started mask
transition enters bounded real runner cleanup. Immutable child entry values
remain separate from launcher repairs. The finite matrix distinguishes physical
operations from substituted reports, Linux post-entry SA_NOCLDWAIT from exec
inheritance and Darwin flag reachability still unclaimed. One focused reviewer
found no concrete issue in its assigned transition scope. No product/control
construction or runtime execution occurred; C1–C4 and conditional C4 premises
retain their prior scope. The ADR set changes no architectural decision.

K20 conservatively charges D≤15/RC≤3, P=E=X=0 including review/presentation/
tree/verification/retirement/commit. Cumulative correctness charged bounds are
D≤95/RC≤44/P≤140/E≤30/X=0; remaining D≤25 (D≤20 exclusively phase-closure
reserve), shared RC≤6/P≤100/E≤90/X=0. Its complete repair estimate is
P≤95/E≤80/RC≤4, including source, finite instruments, red/broken/green,
untouched protected checks, evidence and producer closure.

native-entry-repair-k21 is the exact next planning continuation before the book,
allocated D≤5/RC≤1, P=E=X=0, to freeze the additive fallible Started interface
and finite parameter expansion and commission only that complete supported
producer from actual balances. Full planning allocation leaves D≤20 reserved,
RC≤5/P≤100/E≤90/X=0; full subsequent producer allocation would leave
P≤5/E≤10/RC≤1 and reserved D≤20. Neither projection is spend. Material new
design/review or runtime infrastructure earns a precise human continuation.
Minimum-Rust/Linux qualification, G2–G5/R-W, k4/k6 history, later comparative
selection and all root delivery/documentation obligations remain live.

K21 freezes the additive fallible Started check and exact finite expansion in
native-entry-repair-k22. Existing infallible observers adapt unchanged; failed
Started checks retain io source/first cancellation/reports and require bounded
real child/group/wait/terminal settlement. The spawn-only errno accessor remains
unset after Started. E/F tuples, private barriers, fault modes and paired secondary
reports are fixed in k22; no product/control construction or behavioral execution
occurred in planning. Tier 2 source/trace/coverage checks support the existing seam,
not runtime conformance. No new architectural decision or ADR change follows.

K21 charges conservative D≤5/RC≤1, P=E=X=0 including verification and closure.
Cumulative correctness charges are D≤100/RC≤45/P≤140/E≤30/X=0; remaining
D≤20 exclusively phase-closure reserve, shared RC≤5/P≤100/E≤90/X=0.
K22, before the book, commissions the one complete independently useful repair
at P≤95/E≤80/RC≤4, D=X=0, including all source/control work, frozen red/broken/
green receipts, untouched protected checks, one narrow review and closure.
Full use would leave P≤5/E≤10/RC≤1 plus reserved D≤20; projections are not spend.
New protocol/review/infrastructure needs a precise human continuation, without
reserve borrowing. K22 closure records actual spend and the next precise
continuation before the book. Minimum-Rust/Linux, G2–G5/R-W, k4/k6 history and
all root delivery/documentation obligations remain live; no phase/root close,
comparative gain or independent adoption is established.

K22's implementation source verification found a material prerequisite missing
from its reuse estimate: runner emergency wait and worker post-kill wait block,
runner killed-state polling has no deadline, stop reports are discarded, and
unknown-status terminal recovery can return without reporting unconfirmed
foreground/modes. One narrow reviewer supported these findings; its group-stop
finding was clarified to distinguish attempts from confirmed absence. The
transition design now records exact source evidence and all classifications.
No product/control was changed and no native behavior was measured. Prior C1–C4
receipts retain their scoped meaning; the complete native repair is unimplemented.

K22 decomposes only to preserve that independently useful prerequisite record
as native-entry-settlement-prerequisite-k23 and the precise human continuation
native-entry-settlement-scope-k24. It keeps the complete frozen repair as one
future producer, not module slices. K23 charges conservative P≤15/RC≤4,
D=E=X=0, including source verification, evidence, its one reviewer and closure.
Cumulative correctness bounds are D≤100/RC≤49/P≤155/E≤30/X=0; remaining
D≤20 exclusively phase-closure reserve, shared RC≤1/P≤85/E≤90/X=0.

K24's human answered verbatim **“Approve the bounded renewal (Recommended)”**
to additional D≤10/shared RC≤5 for bounded protocol revision and complete
repricing. Approved phase caps are D≤130/RC≤55/P≤240/E≤120/X=0; zero production
disruption and the D≤20 phase-closure reserve remain. K24 charges conservative
RC≤4 for agreement/context verification, recording/tree/verification/retirement/
commit, D=P=E=X=0. Cumulative correctness charged bounds are
D≤100/RC≤53/P≤155/E≤30/X=0. Remaining D≤30 comprises D≤10 for this protocol/
pricing continuation and D≤20 exclusively phase-closure reserve; shared
RC≤2/P≤85/E≤90/X=0 remain.

`native-entry-settlement-k25` inside k22 allocates D≤10 including closure and
shared RC≤2 for at most one focused review/coordination/closure, P=E=X=0.
It must specify bounded runner/worker error settlement, checked stop reports,
safe pre-reap group identity and unknown-status terminal-recovery uncertainty,
justify any finite-control amendment at the agreed seams within the existing
30/5/5-second controller bounds, and price the complete repair from actual
balances. The old complete estimate stays suspended. A supported complete price
must precede commissioning; any precise excess needs further human funding.
No implementation or control amendment is commissioned by this renewal alone.
K22 and the book remain live, with all prior support/comparison/root obligations.
No parent/phase/root close, target, extraction, comparative gain or independent
adoption is earned.

K25 selects the unimplemented bounded settlement protocol in the existing design
and required spec: owner-local nonblocking observation, absolute 2-second runner
error/post-KILL and total worker teardown bounds, separate existing 1-second
worker drain/group confirmation, checked individual stops and explicit unresolved
ownership. ECHILD/reap/release disables numeric signalling. Unknown status grants
no foreign-foreground takeover; all recovery uncertainty and original/secondary
reports survive to T7. Existing protected logical wait-event traces are reconciled
individually. Exactly 32 S01–S05 rows are proposed and priced, not commissioned;
frozen API/T0–T8/E/F/R remain. One focused reviewer returned no concrete finding.
No product/control or native behavior was constructed/executed.

K25 conservatively charges D≤10/RC≤2, P=E=X=0 including verification/retirement/
commit. Cumulative correctness bounds become D≤110/RC≤55/P≤155/E≤30/X=0;
remaining D≤20 exclusively phase closure, RC=0/P≤85/E≤90/X=0. The complete
producer price is P≤125/E≤85/RC≤5; necessary planning is D≤5/RC≤1 and
funding-agreement recording/closure RC≤4. Exact shortfall is D≤5/RC≤10/P≤40,
with no E/X increase: propose caps D≤135/RC≤65/P≤280/E≤120/X=0. Allocations
are not spend. native-entry-complete-funding-k26 inside k22 is the next precise
human continuation before the book. The repair stays one complete future producer;
none is commissioned from insufficient balances and no reserve is borrowed.
Minimum-Rust/Linux qualification, returning-host, k4/k6/G2–G5/R-W and all root
obligations remain live; no parent/phase/root close or comparative gain follows.

K26's human answered verbatim **“approved”** to the complete repair renewal:
add D≤5/RC≤10/P≤40, raising phase caps to D≤135/RC≤65/P≤280/E≤120/X=0
with zero production disruption and the original D≤20 closure reserve intact.
Approval includes exactly 32 S01–S05 cleanup-failure rows at the existing agreed
CLI/real-policy/fake-job, real-process and PTY seams, unchanged T0–T8/additive
Started API, and the 30-second outer/5-second acknowledgement/5-second cleanup
envelope. Longer legal graces require an exact priced timing remainder before
execution; no blanket matrix or target infrastructure is authorised.

K26 conservatively charges RC≤4 for agreement recording/tree/verification/
retirement/commit, D=P=E=X=0. Cumulative correctness charged bounds are
D≤110/RC≤59/P≤155/E≤30/X=0; remaining D≤25 (5 planning and 20 exclusively
phase closure), RC≤6/P≤125/E≤90/X=0. native-entry-complete-repair-k27 inside
k22, before the book, alone receives D≤5/RC≤1, P=E=X=0 for mechanical planning
and commissioning the one complete producer from actual balances. The complete
producer price is P≤125/E≤85/RC≤5, D=X=0. Full use of planning and producer
would leave D≤20 reserved/RC=0/P=0/E≤5/X=0; allocations are not spend.
This requirements session cuts no implementation leaf and closes no parent.
Minimum-Rust/Linux, returning-host, k4/k6/G2–G5/R-W and all root obligations
remain live; no book advancement, target, extraction or comparative gain follows.

K27 completes the mechanical handoff without product/control construction or
behavioral execution. native-entry-complete-repair-k28 inside k22 is the sole
next complete repair producer before the book. It retains unchanged additive
Started API/T0–T8/E/F/R, exactly 32 approved S rows, selected bounded settlement,
individual protected traces, immutable per-case receipts and the 30/5/5 envelope.

K27 charges conservative D≤5/RC≤1, P=E=X=0 including verification/retirement/
commit. Cumulative correctness charged bounds are D≤115/RC≤60/P≤155/E≤30/X=0;
approved caps remain D≤135/RC≤65/P≤280/E≤120/X=0. Remaining balances are
D≤20 exclusively phase closure, RC≤5/P≤125/E≤90/X=0. K28 allocates the full
P≤125/E≤85/RC≤5, D=X=0, including every source/control, measurement, protected
check, evidence, one narrow review and closure. Full use would leave D≤20
reserved/RC=0/P=0/E≤5/X=0; projections are not charges. K28 records actual
spend and preserves receipts, then prices/funds/cuts the exact support remainder
before the book. Minimum-Rust/Linux qualification and returning-host/comparison/
root obligations remain live; no local success closes the phase or root.
Material discovery, second substantial review, insufficient complete price or
extra infrastructure needs precise renewed funding, never reserve borrowing
or narrower acceptance. Existing ADRs change no architectural decision.

K28's first attempt preserves an unsealed cross-layer repair and partial
development controls, with the complete producer still live. Its exact remaining
qualification work and available failed/green logs are in
`docs/native-entry-development-evidence.md`; no immutable complete native campaign
or qualified-controller set exists. The human approved retaining benign stop
EPERM after confirmed reap/recovery/ESRCH absence, and `Ended::operation_reports`
in the existing JSON notice. Neither amendment waives failure/cancellation
precedence or complete qualification.

The first attempt conservatively charges P≤70/E≤15/RC≤4, D=X=0, including
preservation and verification. Cumulative correctness bounds become
D≤115/RC≤64/P≤225/E≤45/X=0; remaining D≤20 exclusively phase closure,
RC≤1/P≤55/E≤75/X=0. `native-entry-qualification-funding-k29`, before live k28
inside k22, proposes the remaining complete P≤100/E≤70/RC≤10, D=X=0. Exact
shortfall is P≤45/RC≤9; proposed caps D≤135/RC≤74/P≤325/E≤120/X=0 require
explicit approval, no D/E/X increase or reserve borrowing. No retirement or
parent closure is claimed. Support/floors, returning-host, independent adoption,
K4/k6/G2–G5/R-W and every root/book obligation remain live.

K28 is now an open node, preserving its entire original acceptance contract.
Its first child native-entry-development-k30 delivers the tested cross-layer
source/development increment together; the remaining native qualification stays
live as native-entry-qualification-k31 after k29 funding and the tree-owned
review of k30. This replaces the earlier unsealed/force-stop preservation plan.
No module is accepted separately, no complete native-conformance campaign is
claimed, and no parent/phase/root closes. Decomposition does not reset the one
in-session review allowance already spent across k28.

Additional observed verification replaces the first attempt's E≤15 bound with
E≤30 (delta E≤15 only); P≤70/RC≤4, D=X=0 remain counted once across k30/k28.
Cumulative correctness bounds are D≤115/RC≤64/P≤225/E≤60/X=0; remaining D≤20
exclusively phase closure/RC≤1/P≤55/E≤60/X=0. The aggregate remaining price is
P≤100/E≤70/RC≤10, D=X=0. Its exact shortfall is now P≤45/E≤10/RC≤9:
proposed caps D≤135/RC≤74/P≤325/E≤130/X=0, leaving only D≤20 reserved on full
use. This replaces the earlier no-E-increase proposal, not the approved caps.
K29 must obtain explicit renewal before allocating the aggregate remainder.
No new design reserve, infrastructure, comparative gain, support qualification
or narrower acceptance follows. All support/comparison/root obligations stay live.

K29's human answered verbatim **“Approve the renewal (Recommended)”** to adding
P≤45/E≤10/RC≤9 with no D/X increase. Approved phase caps are now
D≤135/RC≤74/P≤325/E≤130/X=0; zero production disruption and the original D≤20
phase-closure reserve remain. The renewed aggregate P≤100/E≤70/RC≤10 is shared
across k29, the tree-owned k32 review, any earned integration and k31
qualification, at the unchanged 153-row matrix, EPERM/Ended amendments and
30/5/5 envelope. K29 charges conservative RC≤3, D=P=E=X=0, including
verification/recording/tree/retirement/commit. Cumulative correctness charged
bounds are D≤115/RC≤67/P≤225/E≤60/X=0; remaining D≤20 exclusively phase
closure, RC≤7/P≤100/E≤70/X=0. K32 receives RC≤5, D=P=E=X=0; any integration it
earns and k31 share P≤100/E≤70/RC≤2, D=X=0, charging actual work only. Full use
leaves D≤20 reserved/RC=0/P=0/E=0/X=0; allocations are not spend. No new
protocol, infrastructure, support qualification or narrower acceptance is
authorised. Rust 1.85/Linux, returning-host, adoption, K4/k6/G2–G5/R-W and every
root/book obligation stay live; no parent/phase/root close or gain follows.

K32's bounded source review records a cancellation race between the last runner
latch read and final blocking. Native-entry-development-k33 integrates the
review before native-entry-qualification-k31; the full qualification remains
required. K32 charges RC≤5, D=P=E=X=0 including retirement/commit. Cumulative
bounds are D≤115/RC≤72/P≤225/E≤60/X=0, leaving D≤20 exclusively phase closure
and shared RC≤2/P≤100/E≤70/X=0 for k33/k31. No parent, phase or root closes;
support qualification, comparative work and every delivery/book obligation
remain live.

K33 integrated K32's runner-latch finding: finalisation consumes that latch
after the final block, covered by two failing-then-passing development controls.
It charges RC≤1/P≤10/E≤5, D=X=0. Cumulative bounds are
D≤115/RC≤73/P≤235/E≤65/X=0, leaving D≤20 exclusively phase closure and
RC≤1/P≤90/E≤65/X=0 for native-entry-qualification-k31. No parent, phase or
root closes; qualification, support, comparative and delivery/book obligations
remain live.

K31's controller and handoff/reader children, recorded in its brief, charged
RC≤1/P≤55/E≤25, leaving cumulative bounds D≤115/RC≤74/P≤290/E≤90/X=0. Their
actual rate showed the remaining price, P≤112/E≤84/RC≤13, exceeded the balance.
K39's human answered verbatim **“Renew full shortfall (Recommended)”**: add
RC≤13/P≤77/E≤44, no D/X, raising phase caps to D≤135/RC≤87/P≤402/E≤174/X=0
with zero production disruption and the original D≤20 closure reserve intact.
K39 charges RC≤3, D=P=E=X=0. Cumulative bounds are D≤115/RC≤77/P≤290/E≤90/X=0;
remaining D≤20 exclusively phase closure, RC≤10/P≤112/E≤84/X=0 for k31's
fault-family, settlement and campaign children and one optional instrument
review. No phase or root closes; support, comparative and delivery/book
obligations remain live.

K36, K37 and K40 (inside k31) built the fault-family, settlement and
campaign-tooling development instruments, charging P≤107/E≤52/RC≤2 together,
D=X=0 (records in the k22 and k31 briefs). K41 read the incumbent's source
(about 129 rows hostable, about 24 rejected premises) and received the standing
budget direction recorded in Notes; it charges RC≤4, D=P=E=X=0. Bookkeeping:
caps D≤135/RC≤88/P≤467/E≤177/X=0; cumulative D≤115/RC≤83/P≤397/E≤142/X=0;
remaining D≤20 (phase closure), RC≤5/P≤70/E≤35/X=0 for the incumbent baseline
k42 and the frozen campaign k43. No phase or root closes; support, comparative
and delivery/book obligations remain live.

K44 (inside k38) built the incumbent subject and the `baseline` class and filed
46 front rows (39 already-correct, 7 fails); charge P≤22/E≤6/RC≤1, D=X=0.
Cumulative D≤115/RC≤84/P≤419/E≤148/X=0 under caps D≤135/RC≤88/P≤467/E≤177/X=0;
remaining D≤20 (phase closure), RC≤4/P≤48/E≤29/X=0 for k45, k46 and k43. No
phase or root closes; support, comparative and delivery/book obligations remain
live.

K45 (the second child of k42, inside k38) classified the 40 runner, worker and
terminal rows from real incumbent runs: 30 observed (14 already-correct, 16
fails) and 10 rejected premises (S01 x8, S02 always-running, S05 runner). With
k44 the node stands at 24 fails, 52 already-correct, 10 rejected and 67
unclassified (k46). The one in-session reviewer, spent here, corrected a first
draft that had rejected S02, S03 and the S04 substitution on too narrow a
premise; S02, S03 and S04 are hosted on the incumbent's own error paths. No
product defect. Charge P≤20/E≤8/RC≤1, D=X=0. Cumulative
D≤115/RC≤85/P≤439/E≤156/X=0 under caps D≤135/RC≤88/P≤467/E≤177/X=0; remaining
D≤20 (phase closure), RC≤3/P≤28/E≤21/X=0 for k46 and k43. K42 stays a node (k46
remains); no phase or root closes; support, comparative and delivery/book
obligations remain live.

K47's inspection-only review of the incumbent instrument records four findings
in native-entry-incumbent-instrument-k47 and cuts its tree-owned integration
before k43's freeze. The reported 65/56/32 classification remains development
evidence subject to that triage; no baseline verdict was changed or rerun by the
review. K42 stays open through integration. Charge RC<=10 minutes including
inspection, findings, bookkeeping, retirement and commit, D=P=E=X=0; cumulative
bounds are D<=115/RC<=96/P<=463/E<=162/X=0. The historical RC<=88 cap is exceeded
by eight; the standing k41 direction removes budget gates and requires no funding
leaf. K43, supported-target qualification and every parent/root obligation remain
live; no node closes.

K43 (the last child of k38, inside k31) took the frozen campaign on
aarch64-apple-darwin, rustc 1.99.0. It digested 327 subjects (source, manifest,
plan, the two documents, the built fixture and worker, every test binary and the
incumbent tree) before and after, unchanged, ran 112 tests (59 repaired and
broken, 53 incumbent), none failing, and reconciled **153 of 153 rows** against
450 write-once records (327 repaired/broken, 123 baseline), each with an
immutable receipt. The incumbent baseline stands at 67 fails, 56 already correct
and 30 rejected premises. R01's 310 identities and 20 ignored flags were listed
first, then preserved through test:cli, test:session, test:runner,
test:terminal-recovery, test and check, each exiting 0 (338 run, 46 skipped =
310 + 74 new) and joined identity by identity with the join seen to fail on its
wrong inputs; a digest after the six tasks equals the campaign's. The docs were
reconciled for Ended::operation_reports and retained EPERM before the freeze
(four silent documents edited, the spec and design already carried both), swept
with positive and cross-tree controls. No product defect; production code,
instruments and plan unchanged. Local success qualifies one host and one
compiler only. The support continuation is cut as native-entry-linux-
qualification-k49 inside k22, before the book, priced P<=45/E<=60/RC<=4 and
funded by k41's standing direction; the native x86_64 remainder is to be cut by
it as a human scope question. Charge P<=12/E<=15/RC<=3, D=X=0. Cumulative
D<=115/RC<=102/P<=510/E<=187/X=0 against the historical caps
D<=135/RC<=88/P<=467/E<=177/X=0, which no longer gate required work (k41);
remaining D<=20 exclusively phase closure.

K52 (the last child of k49, inside k22) took the immutable GNU/Linux measurement on
the corrected tree and re-ran Darwin. Its first aarch64 run failed at the
incumbent build: k51's behaviour-preserving `hex` rewrite in `crates/ravel/build.rs`
tripped the incumbent driver's worker-identity guard, which no run had exercised
since. The guard now pins that one difference by the SHA-256 of its diff and
still refuses any other (three controls); the failed run is kept as the finding
and every reported run was taken after the repair. Results, each from one tree
with subjects digested before and after, unchanged: **aarch64-apple-darwin**
(rustc 1.99.0) 153 of 153 rows, 0 of 153 receipts differ from k43, R01 310 of 310
through six tasks exiting 0; **aarch64-unknown-linux-gnu** (7.0.14-linuxkit,
glibc 2.43, rustc 1.85.0 native) 153 of 153 rows with the two platform overrides
(E03 ignore, E04 raw entry) and 151 receipts equal to k43's, build/clippy
-D warnings/fmt clean, R01 310 of 310 through six tasks exiting 0 including
`task check`; **x86_64-unknown-linux-gnu under QEMU user-mode, recorded as
emulated** 146 of 153 rows, 7 emulation-limited each with reason and control, and
74 protected identities that cannot run (a user namespace cannot be created),
shown by a no-fail-fast supplement (261 passed, 77 failed: 74 namespace, 1 argv
read from `ps`, 2 matrix instruments). Nothing about an x86-64 kernel is
established; the native-x86_64 choice stays in `native-entry-x86-64-linux-scope-k53`,
whose numbers did not move. The documents' status statements were edited once,
before any run, and claim scope without outcome. Evidence:
`docs/evidence/native-entry-linux-campaign/` and the k52 section of
`docs/native-entry-development-evidence.md`. No production source changed.
Charge P<=30/E<=30/RC=0, D=X=0; k50 and k51 recorded no charge, so the cumulative
is at least D<=115/RC<=102/P<=540/E<=217/X=0 against historical caps that no
longer gate required work (k41); remaining D<=20 exclusively phase closure.

K54 (the last live leaf of k22) carried the human's k53 deferral into the shipping
documents: the spec's conformance paragraph, the design's status header and the
evidence note's per-target limit and "Exact remainder" now state that
x86_64-unknown-linux-gnu is built, linted at Rust 1.85.0 and run only under QEMU
user-mode emulation (146 of 153 rows reconciled, 7 emulation-limited, 74
protected identities unable to run), that nothing about an x86-64 kernel is
qualified, and that native qualification is deferred, not rejected, and tracked in
[Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1), which
depends on [Linkuistics/TestAnyware#1](https://github.com/Linkuistics/TestAnyware/issues/1)
(an x86_64 guest under QEMU full-system emulation, binaries cross-built on the
host, recorded as a real x86-64 kernel on an emulated CPU). A sweep with a
positive and a mutated control finds no pointer to the retired handle outside
`.grove/`. `docs/RELEASING.md`, the frozen plan and cases and `docs/evidence/`
are unchanged; what is released is unchanged. Documents only, no measurement.

**K22 closes.** Its repair and support continuation are delivered and measured:
the complete native-entry repair with 153 of 153 rows and 310 of 310 protected
identities on aarch64-apple-darwin (k43) and, at the declared minimum Rust 1.85.0,
on aarch64-unknown-linux-gnu (k52), with x86_64 recorded as emulated and its
native qualification outside this grove. Support limits stay explicit: one
kernel and one libc per Linux target, no other kernel, libc or Rust version, and
no x86-64 kernel. This promotes what survives to the book's brief chain. The
following are *not* delivered by k22, have no live leaf, and are scope questions
for the next requirements session with the human rather than work k22 left
undone: a returning host (a different contract from the one-session launcher),
independent adoption, the comparative questions K4 (reference
0bdcb1f9ae24db5e9dd499308dd791744e908eb0, one iteration/seven attempts), k6,
G2 six-month cost, G3 delivery/use beyond entry, G4 unread alternatives, G5
target execution and R-W runtime/lifecycle debt, and the root's delivery,
extraction and documentation goals. The standing budget direction in Notes still
applies; X=0 still binds. No root goal is met by k22. `book-structure-k5` has
since given them live leaves: the documentation goal is `ravel-book-k55`, and
the rest are `component-work-scope-k57`, which follows the book.
