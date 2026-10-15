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
