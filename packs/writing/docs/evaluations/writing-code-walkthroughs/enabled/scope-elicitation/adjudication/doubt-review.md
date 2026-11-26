# Adversarial protocol review

## Findings

1. Prompt-byte drift is not in the rubric's exhaustive list of replaceable-run
   reasons. Control repetition 1 attempt 2 must remain selected, or the campaign
   must be declared protocol-breached; attempt 3 cannot silently repair it.
2. Absence of an observable `SKILL.md` read is not a discovery count. Skill
   metadata may be presented without a raw discovery event.
3. Every observable skill-file use was invalidated, while every retained enabled
   sample lacked an observable read. The retained scores cannot establish an
   effect of the skill wording.
4. Enabled repetition 5 attempt 3 unambiguously triggers the frozen shortfall
   rule.

## Reconciliation

- Finding 1: **valid and actionable**. Attempt 2 is now the selected behavioral
  sample; attempt 3 is a post-breach diagnostic. The campaign is reported as
  non-comparable, and the original adjudication is preserved as superseded.
- Finding 2: **valid and actionable**. Discovery is reported as indeterminate;
  `0/4` refers only to observable reads or uses in valid streams.
- Finding 3: **valid and actionable**. All score tables are explicitly
  descriptive enabled-template outcomes, not skill-wording attribution.
- Finding 4: **confirmed**. The shortfall remains unchanged.

