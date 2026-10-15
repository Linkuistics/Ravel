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

`book-structure-k5` is the human structure agreement for the requested book for a
developer new to this context. If k4 leaves consequential design uncertainty,
place its precise continuation before that agreement. A supported target can
inform chapter planning; drafting/examples must use stable interfaces.

The dependency order after that is a supported target and migration design,
then independently useful working changes chosen from that design. Each
implementation increment must retain working configuration and complete
sessions; an extracted capability and the interface/consumer that makes it
independently usable belong together. Packaging, additional extractions and
cleanup become separate increments only where each stopping state is useful
and verifiable. Cut their groves and detailed leaves when those boundaries are
known, rather than assigning a crate or language per leaf now.

Documentation structure and editorial work follow the resulting component
contracts. Apply the upstream editorial guidance to both component documentation
and the complete tool/configuration/project documentation, with reader outcomes
and human structure decisions made before drafting.

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
establishing comprehension by themselves.

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
language-specific internals. Precise reader profiles and structure are part of
the later documentation agreement.

On 2026-10-14, during `component-contracts-k3`, the human specifies the
documentation deliverable as a book (user guide) for a software developer with
no background in this context. Teach the problem, vocabulary, actors and session
lifecycle before configuration and implementation internals. Do not infer Rust,
TypeScript or diagram proficiency from general development experience. The
dedicated book-structure agreement settles those assumptions, chapter order,
learning/direct entry routes, media and viewing environment before drafting.

On 2026-10-14 the human changed the upstream prerequisite: extract usable
guidance from the workstreams' current state instead of waiting for completion.
The intake below supersedes the wait requirement from `plan-k1`; the existing
configuration, session and acceptance constraints remain in force.

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
