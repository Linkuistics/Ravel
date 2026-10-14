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
