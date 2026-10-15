# native-entry-fault-families-k36

## Goal

Construct the unbuilt fault families on the k34 controller: F02 (6), F04 (14),
F05 (20), F06 (6), the four paired failures and the two queued-TERM variants,
each with its matching broken control.

## Context

Read the k22 brief's frozen F/paired/cancellation table verbatim and the design.
Mutation rows keep both the omitted and the performed+substituted modes; query
rows use the substituted report. Pre-exec controls stay fixed and allocation-free.
Never rewrite a report to model an irreversible physical operation.

## Done when

- All 46 F rows plus four paired and two cancellation cases run on the controller
  with actual operation versus substituted report recorded separately.
- A failed pending query never becomes empty cancellation; the Started restore
  row uses a live real child/group and PTY lease; original and secondary reports
  survive under cancellation precedence.
- Each claimed repair has a failing broken control; already-correct baselines are
  stated, not invented.

## Notes

If this exceeds one session, decompose at F04/F05/F06.

## Decisions (running log)
