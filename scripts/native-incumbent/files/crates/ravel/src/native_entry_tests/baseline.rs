//! The incumbent judged by the repaired instruments' own claims.
//!
//! Compiled only into the frozen incumbent subject (`scripts/native-incumbent.sh`).
//! Every row's identity and parameters come from the frozen manifest, so a row
//! that is not hosted here is a missing record the driver names, not a silent
//! omission. Each row runs the *incumbent product* once (never a mutant of it) and
//! is judged by the same `case` / `judge` / `judge_readers` claims that judge the
//! repaired product; the verdict is filed under the subject `baseline`.
//!
//! A fact the instrument could not observe (a barrier that was never reached, a
//! controller that did not settle) is an instrument failure and fails the test:
//! it is never recorded as the incumbent failing a claim.
use super::*;

fn manifest() -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    serde_json::from_slice(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .unwrap()
}

/// The manifest's rows of one family: identity and parameters.
fn rows(family: &str) -> Vec<(String, Value)> {
    manifest()["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["family"] == family)
        .map(|row| {
            (
                row["id"].as_str().unwrap().to_owned(),
                row["parameters"].clone(),
            )
        })
        .collect()
}

fn signal(name: &str) -> i32 {
    match name {
        "INT" => libc::SIGINT,
        "TERM" => libc::SIGTERM,
        "HUP" => libc::SIGHUP,
        "PIPE" => libc::SIGPIPE,
        other => panic!("manifest signal {other} is not one the instruments name"),
    }
}

fn baseline(id: &str, conforms: bool, note: &str) {
    record(id, "baseline", conforms, note);
}

/// The facts the last `case` observed: worker wait, managed ending, outcome.
fn facts() -> String {
    LAST_FACTS.with(|last| last.borrow().clone())
}

/// Whether the fixture was ended by the delivered signal while it waited in the
/// barrier, before it could acknowledge the controller's probe. The controller
/// itself reports that as a failed receipt (its acknowledgement never came), so
/// the repaired instrument treats it as an instrument fault. For the incumbent it
/// is an observation: the signal met a default disposition and took the process
/// down at the boundary. It counts only when everything else the controller owns
/// settled and the only barrier fault is the missing acknowledgement.
fn died_at_boundary(seen: &Boundary, signal: i32) -> bool {
    seen.ready.is_some()
        && seen.delivered.is_err()
        && seen.died_by == Some(signal)
        && seen.barrier_failure.is_none()
        && !seen.settlement.watchdog_expired
        && seen.settlement.unresolved.is_empty()
        && seen.settlement.holders.iter().all(|&(_, reaped)| reaped)
        && seen
            .settlement
            .faults
            .iter()
            .all(|fault| fault.starts_with("no ACK record: fixture exited first"))
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e01_entry_state_reaches_the_managed_child() {
    let rows = rows("E01");
    assert_eq!(rows.len(), 16);
    for (id, p) in rows {
        let case_row = json!({"family":"E01",
            "signal": signal(p["signal"].as_str().unwrap()),
            "ignored": p["disposition"] == "ignore",
            "blocked": p["mask"] == "blocked"});
        let conforms = case(case_row, "none");
        baseline(&id, conforms, &facts());
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e02_entry_blocked_pending_is_not_cancellation() {
    let rows = rows("E02");
    assert_eq!(rows.len(), 6);
    for (id, p) in rows {
        let case_row = json!({"family":"E02",
            "signal": signal(p["signal"].as_str().unwrap()),
            "blocked": true, "pending": p["pending"] == "yes"});
        let conforms = case(case_row, "none");
        baseline(&id, conforms, &facts());
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e03_both_real_waits_survive_ignored_sigchld_entry() {
    let rows = rows("E03");
    assert_eq!(rows.len(), 2);
    for (id, p) in rows {
        let chld = i64::from(p["disposition"] == "ignore");
        let conforms = case(json!({"family":"E03","chld":chld}), "none");
        baseline(
            &id,
            conforms,
            &format!("raw flags {}; {}", LAST_RAW_CHLD_FLAGS.get(), facts()),
        );
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e04_private_flag_boundaries_and_raw_entry() {
    let rows = rows("E04");
    assert_eq!(rows.len(), 3);
    for (id, p) in rows {
        match p["boundary"].as_str().unwrap() {
            "worker" => {
                let conforms = case(json!({"family":"E04-worker"}), "none");
                baseline(&id, conforms, &facts());
            }
            "raw-executable-entry" => {
                let conforms = case(json!({"family":"E04-raw","chld":2}), "none");
                baseline(
                    &id,
                    conforms,
                    &format!(
                        "raw SIGCHLD flags {}; SA_NOCLDWAIT reached the executable entry: {}; {}",
                        LAST_RAW_CHLD_FLAGS.get(),
                        LAST_RAW_CHLD_FLAGS.get() & libc::SA_NOCLDWAIT != 0,
                        facts()
                    ),
                );
            }
            // The end-to-end claim at the managed boundary: the front sets the
            // private default plus no-child-wait state just before the runner,
            // and the managed child's wait must still be confirmed. The incumbent
            // has no repair to mutate, so the repaired tree's separate runner-level
            // mutant discriminator has no counterpart here.
            "managed" => {
                let conforms = case(json!({"family":"E04-managed"}), "none");
                baseline(&id, conforms, &facts());
            }
            other => panic!("unknown E04 boundary {other}"),
        }
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e06_boundaries_join_cancellation() {
    let rows = rows("E06");
    assert_eq!(rows.len(), 15);
    for (id, p) in rows {
        let boundary = match p["boundary"].as_str().unwrap() {
            "selection-to-runner" => "selection-to-runner",
            "Started-before-mask-restore" => "started-before-mask-restore",
            "reaped-before-front" => "after-reap-before-front",
            "refusal-before-finalise" => "selection-failure-before-finalise",
            "ENOENT-before-finalise" => "no-ending-error-before-finalise",
            other => panic!("E06 boundary {other} has no instrument boundary"),
        };
        let signal = signal(p["signal"].as_str().unwrap());
        let seen = boundary_row(boundary, signal, "none");
        let axes = judge(boundary, signal, &seen);
        // The controller axis is the instrument's own integrity, not a claim:
        // unless the incumbent itself was ended at the boundary, it must hold.
        let died = died_at_boundary(&seen, signal);
        assert!(
            axes.controller || died,
            "{id}: the boundary was not observed: {axes:?} {:?}",
            seen.barrier_failure
        );
        let conforms = axes.cancelled && axes.zero_result && axes.boundary;
        let note = if died {
            format!("{axes:?}; the fixture was ended by signal {signal} at the boundary before it acknowledged; stderr: {}", seen.diagnostics.lines().rev().take(3).collect::<Vec<_>>().join(" | "))
        } else {
            format!("{axes:?}")
        };
        baseline(&id, conforms, &note);
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_e08_reader_shield_and_collection_bound() {
    let rows = rows("E08");
    assert_eq!(rows.len(), 4);
    for (id, p) in rows {
        let readers = p["readers"].as_str().unwrap();
        let result = p["result"].as_str().unwrap();
        let seen = reader_row(readers, result, "none");
        let axes = judge_readers(readers, result, &seen);
        let died = died_at_boundary(&seen, libc::SIGTERM);
        assert!(
            axes.controller || died,
            "{id}: the boundary was not observed: {axes:?} {:?}",
            seen.barrier_failure
        );
        let conforms = !died && axes.conforms();
        let note = if died {
            format!("{axes:?}; the fixture was ended by SIGTERM at the boundary before it acknowledged")
        } else {
            format!("{axes:?}")
        };
        baseline(&id, conforms, &note);
    }
}

/// The paired F05 row's secondary, selected in the fixture's own process: the
/// runner's terminal handback report (after the physical handback), so the
/// row's primary and secondary are both faulted in one real front run. The
/// site's two-byte record goes to the file the F06 rows' child record uses.
#[cfg(feature = "native-faults")]
pub(super) fn select_pair(case: &Value) {
    if case["pair"] != "handback" {
        return;
    }
    let record = File::create(root().join("child-fault-record.bin")).unwrap();
    let fd = record.as_raw_fd();
    // Kept open for the process's life, as the F06 selection does.
    std::mem::forget(record);
    keyed_launch::native_fault_select("terminal-handback-report", "report", false, fd);
}

// ---- F01, F04, F05 and the queued-TERM rows (k46) ----------------------------
//
// Every hosted row below faults one operation the incumbent already performs,
// through a seam at that call (`faults::{query, mutation_c, mutation_r}`, the
// same hooks the repaired product has), and lets the incumbent do what it does.
// The judged claims are the repaired rows' own, named one by one. Where the
// repaired text is the repaired product's wording (a label), the claim is
// restated by what it means and the restatement is stated in the row's note.
// A claim the instrument needs in order to have observed the row at all (the
// controller settled, the faulted site reached) is an integrity fact: it fails
// the test and files no record rather than reading as the incumbent failing.

/// One row's named claims. `integrity` facts are asserted by the caller.
struct Claims(Vec<(&'static str, bool)>);
impl Claims {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn claim(&mut self, name: &'static str, held: bool) {
        self.0.push((name, held));
    }
    fn conforms(&self) -> bool {
        self.0.iter().all(|(_, held)| *held)
    }
    fn note(&self, seen: &Boundary, extra: &str) -> String {
        let unmet: Vec<_> = self.0.iter().filter(|(_, held)| !held).map(|(n, _)| *n).collect();
        let held: Vec<_> = self.0.iter().filter(|(_, held)| *held).map(|(n, _)| *n).collect();
        let ended = match (seen.settlement.status.and_then(|s| s.code()), seen.died_by) {
            (Some(code), _) => format!("exit {code}"),
            (_, Some(signal)) => format!("signal {signal}"),
            _ => "no status".to_owned(),
        };
        let tail: Vec<_> = seen.diagnostics.lines().rev().filter(|l| !l.trim().is_empty()).take(3).collect();
        format!(
            "unmet {unmet:?}; held {held:?}; {ended}; result_empty={}; {extra}; fault={}; stderr: {}",
            seen.result_empty,
            seen.fault.as_ref().map_or("none".to_owned(), |f| format!(
                "{} {} performed={} actual={} errno={}",
                f["site"], f["mode"], f["operationPerformed"], f["actualResult"], f["actualErrno"]
            )),
            tail.into_iter().rev().collect::<Vec<_>>().join(" | ")
        )
    }
}

fn site_and_mode(p: &Value) -> (String, &'static str) {
    let mode = match p["mode"].as_str().unwrap() {
        "report-EIO" => "report",
        "omit-EIO" => "omit",
        "perform-report-EIO" => "perform",
        other => panic!("manifest mode {other} is not one the instruments name"),
    };
    (p["operation"].as_str().unwrap().to_owned(), mode)
}

/// What every row asserts before it may claim anything about the incumbent: the
/// controller settled the fixture, the exchange completed and the faulted site
/// was reached in the mode named.
fn integrity(id: &str, seen: &Boundary, site: &str, mode: &str) {
    assert!(
        seen.settlement.clean() && seen.delivered.is_ok() && seen.barrier_failure.is_none(),
        "{id}: the fixture was not observed: {seen:?}"
    );
    assert!(
        fault_site_matches(seen, site, mode, false),
        "{id}: the faulted site was not reached as named: {:?}",
        seen.fault
    );
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_f01_initializer_result_failures() {
    let rows = rows("F01");
    assert_eq!(rows.len(), 3);
    for (id, p) in rows {
        let operation = match p["operation"].as_str().unwrap() {
            "mask-query" => 1,
            "TERM-action-query" => 2,
            "TERM-membership" => 3,
            other => panic!("F01 operation {other} has no initializer site"),
        };
        let conforms = case(json!({"family":"F01","operation":operation}), "none");
        let facts = facts();
        assert!(
            facts.contains(&format!("native baseline F01: operation {operation} reported EIO")),
            "{id}: the report was never substituted: {facts}"
        );
        baseline(&id, conforms, &facts);
    }
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_f04_selection_handler_faults() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let mut hosted = 0;
    for (id, p) in rows("F04") {
        if p["owner"] != "selection" {
            continue;
        }
        let (site, mode) = site_and_mode(&p);
        if site == "INT-final-restore" {
            continue;
        }
        hosted += 1;
        // Owners of INT/TERM/HUP the repaired claim names: the installed subset
        // is retained until finalisation.
        let owners = match (site.as_str(), mode) {
            ("action-query", _) | ("INT-install", "omit") => ["default", "default", "default"],
            ("INT-install", _) | ("TERM-install", "omit") => ["front", "default", "default"],
            ("TERM-install", _) => ["front", "front", "default"],
            other => panic!("{id}: {other:?}"),
        };
        let row = json!({"family":"F04","owner":"selection","site":site,"mode":mode,
            "boundary":"before-finalise"});
        let seen = run_row(row, "none");
        integrity(&id, &seen, &site, mode);
        let facts = seen.ready.as_ref().expect("finalisation facts");
        let found = ["INT", "TERM", "HUP"].map(|name| facts["owners"][name].as_str().unwrap().to_owned());
        let outcome = seen.outcome.as_ref();
        let mut claims = Claims::new();
        claims.claim("failed-exit", seen.settlement.status.is_some_and(|s| !s.success()));
        claims.claim("zero-result", seen.result_empty);
        claims.claim(
            "cancellation-unavailable-code",
            outcome.is_some_and(|v| v["error"]["code"] == "cancellation_unavailable"),
        );
        claims.claim(
            "errno-reported",
            outcome.is_some_and(|v| {
                v["error"]["message"].as_str().is_some_and(|m| m.contains("Input/output error"))
            }),
        );
        claims.claim("no-worker", !seen.worker_confirmed && !seen.worker_started);
        claims.claim("no-managed", !seen.managed_started);
        claims.claim("partial-subset-owned", found == owners);
        claims.claim(
            "handled-signals-blocked-at-finalisation",
            facts["blocked"].as_u64().unwrap() & handled == handled,
        );
        baseline(
            &id,
            claims.conforms(),
            &claims.note(&seen, &format!("owners at finalisation {found:?}, expected {owners:?}")),
        );
    }
    assert_eq!(hosted, 5);
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_f04_int_final_restore_failure() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let mut hosted = 0;
    for (id, p) in rows("F04") {
        let (site, mode) = site_and_mode(&p);
        if site != "INT-final-restore" {
            continue;
        }
        hosted += 1;
        let owner = p["owner"].as_str().unwrap().to_owned();
        let result = if owner == "selection" { "policy-refusal" } else { "success" };
        let row = json!({"family":"F04","owner":owner,"site":"INT-final-restore","mode":mode,
            "result":result,"boundary":"before-finalise"});
        let seen = run_row(row, "none");
        integrity(&id, &seen, &site, mode);
        let child_ran = owner == "runner";
        let after = &seen.fault.as_ref().unwrap()["after"];
        let installed = if owner == "selection" { "front" } else { "other" };
        let expected = if mode == "omit" { installed } else { "default" };
        let mut claims = Claims::new();
        claims.claim("failed-exit", seen.settlement.status.is_some_and(|s| !s.success()));
        claims.claim("zero-result", seen.result_empty);
        claims.claim(
            "original-retained",
            owner != "selection" || seen.diagnostics.contains("owner_declined"),
        );
        // The repaired label names the release; the claim is that the failed
        // disposition release is reported at all, by a line naming it.
        claims.claim("release-failure-reported", seen.diagnostics.contains("disposition release"));
        claims.claim(
            "child-ran-and-reaped",
            if child_ran {
                seen.managed_started
                    && seen.worker_confirmed
                    && seen.managed_group.as_deref() == Some("Gone")
            } else {
                !seen.managed_started
            },
        );
        claims.claim("owner-is-the-operations-own", after["INT"] == expected);
        claims.claim(
            "handled-signals-blocked-after",
            after["blocked"].as_u64().unwrap() & handled == handled,
        );
        baseline(
            &id,
            claims.conforms(),
            &claims.note(&seen, &format!("INT owner after {} (expected {expected})", after["INT"])),
        );
    }
    assert_eq!(hosted, 4);
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_f05_final_mask_restore_and_started_restore() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let mut hosted = 0;
    for (id, p) in rows("F05") {
        let (site, mode) = site_and_mode(&p);
        let outcome = p["outcome"].as_str().unwrap().to_owned();
        match site.as_str() {
            "final-mask-restore" if outcome != "policy-refusal" => {
                hosted += 1;
                let (original, child_ran) = match outcome.as_str() {
                    "selection-success" => ("", true),
                    "spawn-ENOENT" => ("No such file or directory", false),
                    other => panic!("{id}: outcome {other}"),
                };
                let row = json!({"family":"F05","outcome":outcome,"site":site,"mode":mode});
                let seen = run_row(row, "none");
                integrity(&id, &seen, &site, mode);
                let mut claims = Claims::new();
                claims.claim("failed-exit", seen.settlement.status.is_some_and(|s| !s.success()));
                claims.claim("zero-result", seen.result_empty);
                claims.claim("original-retained", original.is_empty() || seen.diagnostics.contains(original));
                // The failure itself is retained in the diagnostics (the
                // substituted errno's text) ...
                claims.claim("secondary-retained", seen.diagnostics.contains("Input/output error"));
                // ... and the repaired claim is a *label*: the line must name
                // the failed transition, not only its errno.
                claims.claim("secondary-names-the-operation", seen.diagnostics.contains("final mask release"));
                claims.claim(
                    "child-ran-and-reaped",
                    if child_ran {
                        seen.managed_started
                            && seen.worker_confirmed
                            && seen.managed_group.as_deref() == Some("Gone")
                    } else {
                        !seen.managed_started
                    },
                );
                let after = seen.fault.as_ref().unwrap()["after"]["blocked"].as_u64().unwrap() & handled;
                baseline(
                    &id,
                    claims.conforms(),
                    &claims.note(&seen, &format!("handled signals blocked after the site: {after:#b}")),
                );
            }
            "Started-parent-restore" => {
                hosted += 1;
                let row = json!({"family":"F05","outcome":"selection-success","site":site,
                    "mode":mode,"pty":true,"boundary":"before-finalise"});
                let seen = run_row(row, "none");
                integrity(&id, &seen, &site, mode);
                let facts = seen.ready.as_ref().expect("finalisation facts");
                let at = |key: &str| facts[key].as_i64().unwrap();
                let blocked = seen.fault.as_ref().unwrap()["after"]["blocked"].as_u64().unwrap() & handled;
                let mut claims = Claims::new();
                claims.claim("failed-exit", seen.settlement.status.is_some_and(|s| !s.success()));
                claims.claim("zero-result", seen.result_empty);
                claims.claim(
                    "supervision-failed-code",
                    seen.outcome.as_ref().is_some_and(|v| v["error"]["code"] == "supervision_failed"),
                );
                claims.claim("child-started-once-and-reaped", seen.managed_started && seen.worker_confirmed && facts["managedReaped"] == true);
                claims.claim("no-ending-produced", seen.managed_end.is_none());
                claims.claim("terminal-returned", at("foreground") == at("pgrp"));
                claims.claim("no-stray-child", at("strayChild") == 0);
                claims.claim(
                    "mask-is-the-operations-own",
                    blocked == if mode == "omit" { handled } else { 0 },
                );
                baseline(
                    &id,
                    claims.conforms(),
                    &claims.note(
                        &seen,
                        &format!("ended {:?}; the launch {}", seen.managed_end, if seen.result_empty { "wrote nothing" } else { "completed and wrote its result" }),
                    ),
                );
            }
            _ => {}
        }
    }
    // final-mask-restore: 2 outcomes x 2 modes; Started restore: 2 modes.
    assert_eq!(hosted, 6);
}

/// The queued-TERM rows at the front: an actual TERM pending under the parent's
/// block when the Started restore fails (F05) or the child's TERM action
/// restore fails (F06).
#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_queued_term_rows() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let mut hosted = 0;
    for (id, p) in rows("cancellation") {
        let case_name = p["case"].as_str().unwrap().to_owned();
        let (row, label) = match case_name.as_str() {
            "F05-Started-restore" => (
                json!({"family":"F05","outcome":"selection-success","site":"Started-parent-restore",
                    "mode":"omit","pty":true,"boundary":"started-before-mask-restore"}),
                "Started parent transition failed",
            ),
            "F06-TERM-action" => (
                json!({"family":"F06","site":"TERM-action-restore","mode":"omit",
                    "boundary":"handoff-block-held"}),
                "cannot spawn",
            ),
            other => panic!("{id}: cancellation case {other}"),
        };
        hosted += 1;
        let (site, f06) = if case_name == "F06-TERM-action" {
            ("TERM-action-restore", true)
        } else {
            ("Started-parent-restore", false)
        };
        let seen = drive_row(row, libc::SIGTERM, "none");
        assert!(
            seen.settlement.clean() && seen.delivered.is_ok() && seen.barrier_failure.is_none(),
            "{id}: the fixture was not observed: {seen:?}"
        );
        let ready = seen.ready.as_ref().expect("ready facts");
        let ack = seen.ack.as_ref().expect("acknowledged facts");
        // The queued signal was an actual pending TERM under the block: the
        // instrument's own precondition for the row, never the incumbent's claim.
        assert_eq!(ready["blocked"].as_u64().unwrap() & handled, handled, "{id}: {ready}");
        assert_ne!(ack["pending"].as_u64().unwrap() & bit(libc::SIGTERM), 0, "{id}: {ack}");
        assert!(ack["frontLatch"].is_null(), "{id}: {ack}");
        // The site must have been reached. The F06 site is in the forked child,
        // whose fixed two-byte record the fixture wrote before it ended.
        if f06 {
            let record = seen.child_record.clone().unwrap_or_default();
            assert_eq!(record.len(), 2, "{id}: the child never reached the site: {record:?}");
            assert_eq!(record[1] & 1, 0, "{id}: the omitted restore was performed");
        } else {
            assert!(
                fault_site_matches(&seen, site, "omit", false),
                "{id}: site not reached: {:?}",
                seen.fault
            );
        }
        let mut claims = Claims::new();
        claims.claim("joined-at-finalisation-death-by-TERM", seen.died_by == Some(libc::SIGTERM));
        claims.claim("zero-result", seen.result_empty);
        claims.claim(
            "outcome-names-TERM",
            seen.outcome.as_ref().is_some_and(|v| v["error"]["signal"] == "SIGTERM"),
        );
        claims.claim("original-failure-retained", seen.diagnostics.contains(label));
        if f06 {
            claims.claim(
                "spawn-failed-before-any-managed-start",
                seen.diagnostics.contains("Input/output error") && !seen.managed_started && seen.managed_end.is_none(),
            );
            claims.claim("worker-teardown-confirmed", seen.worker_confirmed);
        } else {
            claims.claim("live-child-cleaned-up-with-no-ending", seen.managed_started && seen.managed_end.is_none());
        }
        baseline(
            &id,
            claims.conforms(),
            &claims.note(
                &seen,
                &format!(
                    "managed_started={} ended={:?} child_record={:?}",
                    seen.managed_started, seen.managed_end, seen.child_record
                ),
            ),
        );
    }
    assert_eq!(hosted, 2);
}

/// The paired F05 Started row, hosted end to end at the front. The primary is
/// the front's own Started parent restore (`restore_mask_at`, which the
/// incumbent swallows with `let _`), omitted and reported failed at that call;
/// the secondary is the runner's terminal handback report, substituted after the
/// physical handback in the same run. No fixture callback stands in for either
/// operation: the incumbent's own front performs the first and its own runner the
/// second, and what each does with its failure is what is judged.
#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_paired_started_restore_and_recovery_report() {
    const HANDBACK_REPORT: &str =
        "C4 substituted terminal handback report after physical operation";
    let mut hosted = 0;
    for (id, p) in rows("paired") {
        if p["case"] != "F05-Started-restore-recovery-report" {
            continue;
        }
        hosted += 1;
        let row = json!({"family":"F05","outcome":"selection-success",
            "site":"Started-parent-restore","mode":"omit","pty":true,
            "boundary":"before-finalise","pair":"handback"});
        let seen = run_row(row, "none");
        integrity(&id, &seen, "Started-parent-restore", "omit");
        // The secondary's site was reached in this run, on the incumbent's own
        // runner: its record is the fixed two bytes, bit 0 the physical
        // handback having run. Otherwise the row observed nothing of it.
        let record = seen.child_record.clone().unwrap_or_default();
        assert_eq!(record.len(), 2, "{id}: the handback report site was not reached: {record:?}");
        assert_eq!(record[0], 9, "{id}: {record:?}");
        assert_eq!(record[1] & 1, 1, "{id}: the handback was not performed: {record:?}");
        let facts = seen.ready.as_ref().expect("finalisation facts");
        let at = |key: &str| facts[key].as_i64().unwrap();
        let retained = |needle: &str| {
            seen.diagnostics.contains(needle)
                || seen.outcome.as_ref().is_some_and(|v| {
                    v["error"]["message"].as_str().is_some_and(|m| m.contains(needle))
                })
        };
        let mut claims = Claims::new();
        claims.claim("primary-retained", retained("Started parent transition failed"));
        claims.claim("secondary-retained", retained(HANDBACK_REPORT));
        claims.claim(
            "child-started-and-actually-reaped",
            seen.managed_started && seen.worker_confirmed && facts["managedReaped"] == true,
        );
        claims.claim("no-stray-child", at("strayChild") == 0);
        claims.claim("terminal-physically-returned", at("foreground") == at("pgrp"));
        baseline(
            &id,
            claims.conforms(),
            &claims.note(
                &seen,
                &format!(
                    "ended {:?}; the launch {}; handback site record {record:?}",
                    seen.managed_end,
                    if seen.result_empty { "wrote nothing" } else { "completed and wrote its result" }
                ),
            ),
        );
    }
    assert_eq!(hosted, 1);
}

/// The F02 worker rows and the F05 finalisation-time rows are rejected
/// premises, and each names an operation the incumbent never performs. Seen
/// here as a fact, with the same fixture and the same site selection the
/// repaired rows use: the selected site is never reached (no fault receipt), the
/// worker is admitted and the front runs to its end. The positive counterpart
/// is the hosted rows above, whose receipts exist on the same fixture.
#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_control_rejected_premises_name_operations_the_incumbent_never_performs() {
    // F02's worker rows: the worker-side watching query and repair.
    for (site, mode) in [
        ("watch-action-query", "report"),
        ("default-action-repair", "omit"),
        ("default-action-repair", "perform"),
    ] {
        let seen = run_row(json!({"family":"F02","owner":"worker","site":site,"mode":mode}), "none");
        assert!(seen.settlement.clean(), "{site}/{mode}: {seen:?}");
        assert!(
            seen.fault.is_none(),
            "{site}/{mode}: the incumbent's worker performed the operation: {:?}",
            seen.fault
        );
        assert!(
            seen.worker_started,
            "{site}/{mode}: the worker was not admitted, so the premise was not exercised"
        );
    }
    // F05's finalisation-time operations: the block, the pending query and the
    // TERM membership. The incumbent's one block is the pre-launch
    // `block_and_check`, before the managed child.
    for (site, mode) in [
        ("finalisation-block", "omit"),
        ("finalisation-block", "perform"),
        ("pending-query", "report"),
        ("TERM-membership", "report"),
    ] {
        for outcome in ["selection-success", "policy-refusal", "spawn-ENOENT"] {
            let seen = run_row(
                json!({"family":"F05","outcome":outcome,"site":site,"mode":mode}),
                "none",
            );
            assert!(seen.settlement.clean(), "{outcome}/{site}/{mode}: {seen:?}");
            assert!(
                seen.fault.is_none(),
                "{outcome}/{site}/{mode}: the incumbent performed the operation: {:?}",
                seen.fault
            );
        }
    }
    // F05's final mask restore on the policy-refusal path: the refusal settles
    // without restoring the mask (the signals were never blocked). The same
    // site *is* reached on the spawn-ENOENT path, on this same fixture.
    let refusal = run_row(
        json!({"family":"F05","outcome":"policy-refusal","site":"final-mask-restore","mode":"omit"}),
        "none",
    );
    assert!(refusal.settlement.clean(), "{refusal:?}");
    assert!(refusal.fault.is_none(), "{:?}", refusal.fault);
    assert!(refusal.settlement.status.is_some_and(|s| s.code() == Some(3)), "{refusal:?}");
    let reached = run_row(
        json!({"family":"F05","outcome":"spawn-ENOENT","site":"final-mask-restore","mode":"omit"}),
        "none",
    );
    assert!(reached.settlement.clean(), "{reached:?}");
    assert!(
        fault_site_matches(&reached, "final-mask-restore", "omit", false),
        "{:?}",
        reached.fault
    );
}

// ---- harness controls -------------------------------------------------------
//
// A baseline that reads "already correct" is trustworthy only if the same
// instrument is seen to fail when the incumbent is made wrong in the way the row
// names. These run the harness against the incumbent with one defect put in by a
// private seam; they file no row record.

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_control_a_wrong_entry_state_is_detected() {
    let row = json!({"family":"E01","signal":libc::SIGTERM,"ignored":false,"blocked":false});
    assert!(case(row.clone(), "none"), "the incumbent conforms on this row");
    assert!(
        !case(row, "capture"),
        "the same instrument must fail a managed child handed the wrong entry state"
    );
}

#[test]
#[ignore = "private real-main fixture; run native-incumbent.sh test"]
fn native_baseline_control_unshielded_readers_are_detected() {
    let good = reader_row("both-complete", "selection-success", "none");
    assert!(judge_readers("both-complete", "selection-success", &good).shielded);
    let broken = reader_row("both-complete", "selection-success", "shielding");
    let axes = judge_readers("both-complete", "selection-success", &broken);
    assert!(axes.controller, "{axes:?}");
    assert!(!axes.shielded, "unshielded readers passed the eligibility receipt");
}
