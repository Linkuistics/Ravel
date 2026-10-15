//! The incumbent runner's F rows, judged by the repaired runner instrument's
//! own named claims (k46).
//!
//! Compiled only into the frozen incumbent subject (`scripts/native-incumbent.sh`)
//! as a `cfg(test)` module of its `keyed-launch` crate. A port of the repaired
//! `run/native_faults.rs`: the same role, the same controller, the same fixed
//! two-byte site record and the same facts, with the repaired internals the
//! incumbent does not have left out (there is no `start_check`, so the repaired
//! role's start-check closure has no counterpart; the Started transition is the
//! front's, which the incumbent swallows, and the role's observer stands in for
//! exactly that). Row identities come from the frozen manifest and each verdict
//! is filed under the subject `baseline` through the write-once recorder.
//!
//! Every site is a hook at an operation the incumbent already performs; none
//! adds a check, a report or a repair. A fact the instrument could not observe
//! (the controller did not settle, the site was not reached) fails the test and
//! files no record: it is an instrument fault, never the incumbent failing a
//! claim.
use super::*;
use crate::native_hooks::*;
use std::process::Stdio;
use std::sync::atomic::AtomicBool;

const ROLE: &str = "run::baseline_faults_tests::native_baseline_fault_role";
const HANDBACK_REPORT: &str = "C4 substituted terminal handback report after physical operation";

fn runner_owner(signal: libc::c_int) -> &'static str {
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    // SAFETY: a null new action only queries.
    if unsafe { libc::sigaction(signal, std::ptr::null(), &mut action) } != 0 {
        return "unknown";
    }
    let runner = on_terminate as extern "C" fn(libc::c_int) as usize;
    match action.sa_sigaction {
        libc::SIG_DFL => "default",
        libc::SIG_IGN => "ignore",
        address if address == runner => "runner",
        _ => "other",
    }
}
fn termios_bytes(terminal: &Terminal) -> Vec<u8> {
    let mut attributes = unsafe { std::mem::zeroed::<libc::termios>() };
    // SAFETY: tcgetattr fills an initialised struct from a descriptor we own.
    if unsafe { libc::tcgetattr(terminal.fd(), &mut attributes) } != 0 {
        return Vec::new();
    }
    // SAFETY: a plain-data struct viewed as bytes for comparison only.
    unsafe {
        std::slice::from_raw_parts(
            (&attributes as *const libc::termios).cast::<u8>(),
            std::mem::size_of::<libc::termios>(),
        )
    }
    .to_vec()
}

#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_fault_role() {
    let Some(site) = std::env::var_os("K28_FAULT_SITE") else {
        return;
    };
    let number = |name: &str| -> u8 { std::env::var(name).unwrap().parse().unwrap() };
    let _ = site;
    let (site, mode) = (number("K28_FAULT_SITE"), number("K28_FAULT_MODE"));
    let case = std::env::var("K28_FAULT_CASE").unwrap();
    let mut fds = [0 as libc::c_int; 2];
    // SAFETY: pipe fills two descriptors this role owns until it exits.
    assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
    // The private post-entry action the F02 rows name: default plus
    // no-child-wait. "watching-ignored" is the harness control that gives the
    // incumbent the one disposition it does repair, so the repair's site is
    // seen to be reachable and its absence under the named state is observed.
    if case == "watching" || case == "watching-ignored" {
        let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
        action.sa_sigaction = if case == "watching" { libc::SIG_DFL } else { libc::SIG_IGN };
        action.sa_flags = if case == "watching" { libc::SA_NOCLDWAIT } else { 0 };
        assert_eq!(unsafe { libc::sigemptyset(&mut action.sa_mask) }, 0);
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) },
            0
        );
    }
    let terminal = Terminal::open();
    let own = own_group();
    let modes_before = terminal.as_ref().map(termios_bytes);
    // Site 0 selects nothing: a role that only stands in for a front operation.
    select(site, mode, false, fds[1]);
    // The secondary failure of a paired row: the terminal handback's own report,
    // substituted after the physical operation by the incumbent's own C4 seam.
    if std::env::var("K28_FAULT_PAIR").as_deref() == Ok("handback") {
        terminal_tests::inject_handback_report();
    }
    // The front's Started parent-mask restore, which the incumbent swallows
    // (`let _ = signal_state::restore_mask_at(..)`). The role has no mask of
    // its own to restore, so the stand-in is the failed restore, ignored.
    let started_fails = case == "started";
    let argv = crate::Argv::new("/bin/sh".into(), vec!["-c".into(), "exit 7".into()]);
    let entry = EntrySignals::new([], []);
    let mut events = Vec::new();
    let result = run_observed(
        Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: &[],
            transparent: Some(&entry),
            cwd: None,
            escalation: Escalation {
                grace: Duration::from_secs(1),
                kill_grace: Duration::from_secs(1),
            },
        },
        &mut |event| {
            if started_fails && event == LaunchEvent::Started {
                let _ = Err::<(), _>(std::io::Error::from_raw_os_error(libc::EIO));
            }
            events.push(event)
        },
    );
    let ended = match &result {
        Ok(ended) => format!(
            "end={:?} code={:?} group={:?}",
            ended.end,
            ended.status.code(),
            ended.group
        ),
        Err(error) => format!(
            "error={:?} errno={:?} consumed={:?}",
            error.to_string(),
            error.raw_os_error(),
            error.consumed_signal()
        ),
    };
    // Whatever child existed was waited for by the product; nothing may remain.
    let mut status = 0;
    // SAFETY: a nonblocking wait for any child of this role.
    let stray = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
    let stray_errno = std::io::Error::last_os_error().raw_os_error();
    let mut chld = unsafe { std::mem::zeroed::<libc::sigaction>() };
    unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut chld) };
    let mut records = Vec::new();
    let mut buffer = [0u8; 64];
    // SAFETY: nonblocking drain of the role's own pipe after every writer is gone.
    unsafe {
        let flags = libc::fcntl(fds[0], libc::F_GETFL);
        libc::fcntl(fds[0], libc::F_SETFL, flags | libc::O_NONBLOCK);
        let count = libc::read(fds[0], buffer.as_mut_ptr().cast(), buffer.len());
        if count > 0 {
            records.extend_from_slice(&buffer[..count as usize]);
        }
    }
    let line = format!(
        "events={events:?} {ended} records={records:?} stray={stray} stray_errno={stray_errno:?} \
         chld_nocldwait={} chld_ignored={} int={} term={} hup={} foreground_restored={} modes_unchanged={}",
        i32::from(chld.sa_flags & libc::SA_NOCLDWAIT != 0),
        i32::from(chld.sa_sigaction == libc::SIG_IGN),
        runner_owner(libc::SIGINT),
        runner_owner(libc::SIGTERM),
        runner_owner(libc::SIGHUP),
        terminal.as_ref().map(|t| t.foreground() == own).map_or(-1, i32::from),
        match (&terminal, &modes_before) {
            (Some(t), Some(before)) => i32::from(&termios_bytes(t) == before),
            _ => -1,
        },
    );
    std::fs::write(std::env::var_os("K28_FAULT_OUT").unwrap(), line).unwrap();
}

#[derive(Debug)]
struct Row {
    line: String,
    clean: bool,
}
impl Row {
    fn word(&self, name: &str) -> Option<String> {
        self.line
            .split_whitespace()
            .find_map(|word| word.strip_prefix(&format!("{name}=")))
            .map(str::to_owned)
    }
    fn is(&self, name: &str, value: &str) -> bool {
        self.word(name).as_deref() == Some(value)
    }
    /// The launch ended in an error before any child existed. Whether the text
    /// names the failed operation is a separate claim.
    fn refused(&self) -> bool {
        self.line.contains("error=") && self.line.contains("events=[]")
    }
    /// One two-byte record: this site, performed or omitted, and whether the
    /// operation ran while the terminal's foreground group was its own.
    fn site_record(&self, site: u8, performed: bool, foreground: bool) -> bool {
        let flags = u8::from(performed) | (u8::from(foreground) << 2);
        self.line.contains(&format!("records=[{site}, {flags}]"))
    }
    /// The site was reached at all, performed or not.
    fn site_reached(&self, site: u8) -> bool {
        self.line.contains(&format!("records=[{site},"))
    }
    fn admitted(&self) -> bool {
        self.line.contains("events=[Started")
    }
}

fn fault_row(case: &str, site: u8, mode: u8, pty: bool) -> Row {
    paired_row(case, site, mode, pty, "")
}

/// `handback`: add the incumbent's own C4 terminal-handback report as a
/// secondary failure.
fn paired_row(case: &str, site: u8, mode: u8, pty: bool, handback: &str) -> Row {
    use crate::native_barrier::{Controller, Limits};
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::os::unix::process::CommandExt as _;
    let directory = tempfile::tempdir().unwrap();
    let channels = directory.path().join("barrier");
    std::fs::create_dir(&channels).unwrap();
    let out = directory.path().join("outcome");
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            ROLE,
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("K28_FAULT_SITE", site.to_string())
        .env("K28_FAULT_MODE", mode.to_string())
        .env("K28_FAULT_CASE", case)
        .env("K28_FAULT_OUT", &out);
    if !handback.is_empty() {
        command.env("K28_FAULT_PAIR", handback);
    }
    let mut master = None;
    if pty {
        let (mut m, mut s) = (-1, -1);
        // SAFETY: openpty fills two descriptors that are owned immediately below.
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut m,
                    &mut s,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            },
            0
        );
        // SAFETY: fresh descriptors from openpty.
        let (m, s) = unsafe { (std::fs::File::from_raw_fd(m), std::fs::File::from_raw_fd(s)) };
        for fd in [m.as_raw_fd(), s.as_raw_fd()] {
            // SAFETY: close-on-exec on a descriptor this row owns.
            assert_eq!(
                unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
                0
            );
        }
        command
            .stdin(s.try_clone().unwrap())
            .stdout(s.try_clone().unwrap())
            .stderr(s);
        // SAFETY: only async-signal-safe calls between fork and exec. setsid
        // makes the role its own session and group leader; TIOCSCTTY gives it
        // the slave as its controlling terminal.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1 || libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        master = Some(m);
    } else {
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0);
    }
    controller.configure(&mut command);
    controller.adopt(command.spawn().unwrap());
    drop(command);
    // A process exiting onto a terminal nobody reads can block in tty close
    // beyond SIGKILL, so the row drains its own master on a thread it owns,
    // stops it, and joins it on every path.
    let stop = std::sync::Arc::new(AtomicBool::new(false));
    let drain = master.as_ref().map(|master| {
        // SAFETY: O_NONBLOCK on a descriptor this row owns.
        unsafe {
            let flags = libc::fcntl(master.as_raw_fd(), libc::F_GETFL);
            assert_eq!(
                libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK),
                0
            );
        }
        let mut reader = master.try_clone().unwrap();
        let stop = std::sync::Arc::clone(&stop);
        std::thread::spawn(move || {
            use std::io::Read as _;
            let mut sink = [0u8; 4096];
            while !stop.load(Ordering::SeqCst) {
                let mut poll = libc::pollfd {
                    fd: reader.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                };
                // SAFETY: one initialised pollfd, bounded wait.
                unsafe { libc::poll(&mut poll, 1, 20) };
                while matches!(reader.read(&mut sink), Ok(count) if count > 0) {}
            }
        })
    });
    let settlement = controller.finish();
    stop.store(true, Ordering::SeqCst);
    if let Some(drain) = drain {
        drain.join().unwrap();
    }
    drop(master);
    let row = Row {
        line: std::fs::read_to_string(&out).unwrap_or_default(),
        clean: settlement.clean() && settlement.status.is_some_and(|s| s.success()),
    };
    eprintln!("native baseline runner fault case={case} site={site} mode={mode}: {row:?} {settlement:?}");
    row
}

/// Every manifest row identity of one family, in manifest order. The crate has
/// no JSON dependency, and the identity carries every parameter the row names.
fn rows(family: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let prefix = format!("{family}__");
    text.split("\"id\": \"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .filter(|id| id.starts_with(&prefix))
        .map(str::to_owned)
        .collect()
}

/// The value of `name` in a row identity such as `F04__owner=A__operation=B`.
fn parameter<'a>(id: &'a str, name: &str) -> &'a str {
    id.split("__")
        .find_map(|part| part.strip_prefix(&format!("{name}=")))
        .unwrap_or_else(|| panic!("{id} names no {name}"))
}

/// File one row. The controller settling the role cleanly is the instrument's
/// integrity, asserted before anything is claimed of the incumbent.
fn file(id: &str, label: &str, row: &Row, checks: &[(&'static str, bool)]) {
    assert!(row.clean, "{label}: the role was not observed to settle: {row:?}");
    let unmet: Vec<_> = checks.iter().filter(|(_, held)| !held).map(|(n, _)| *n).collect();
    let held: Vec<_> = checks.iter().filter(|(_, held)| *held).map(|(n, _)| *n).collect();
    crate::native_barrier::row(
        id,
        "baseline",
        unmet.is_empty(),
        &format!("{label}: unmet {unmet:?}; held {held:?}; {}", row.line),
    );
}

fn site_of(name: &str) -> u8 {
    match name {
        "watch-action-query" => WATCH_QUERY,
        "action-query" => HANDLER_QUERY,
        "INT-install" => INT_INSTALL,
        "TERM-install" => TERM_INSTALL,
        "TERM-action-restore" => TERM_RESTORE,
        "mask-set-build" => MASK_BUILD,
        "mask-restore" => MASK_RESTORE,
        other => panic!("{other} is not a runner fault site"),
    }
}
fn mode_of(mode: &str) -> u8 {
    match mode {
        "report-EIO" => REPORT,
        "omit-EIO" => OMIT,
        "perform-report-EIO" => PERFORM,
        other => panic!("manifest mode {other}"),
    }
}

#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_f02_managed_watching_query() {
    let mut hosted = 0;
    for id in rows("F02") {
        if parameter(&id, "boundary") != "managed" || parameter(&id, "operation") != "watch-action-query" {
            continue;
        }
        hosted += 1;
        let (site, mode) = (site_of("watch-action-query"), mode_of(parameter(&id, "mode")));
        let row = fault_row("watching", site, mode, false);
        assert!(
            row.site_reached(site),
            "{id}: the incumbent never performed the query: {row:?}"
        );
        file(
            &id,
            "F02 managed query",
            &row,
            &[
                ("refused-before-any-child", row.refused()),
                ("names-the-failed-query", row.line.contains("cannot query child watching")),
                ("EIO-in-the-report", row.line.contains("Input/output error")),
                ("no-spawn-errno", row.is("errno", "None")),
                ("site", row.site_record(site, true, false)),
                ("actual-flag-state-left-set", row.is("chld_nocldwait", "1")),
                ("no-stray-child", row.is("stray", "-1")),
            ],
        );
    }
    assert_eq!(hosted, 1);
}

#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_f04_runner_handler_faults() {
    let mut hosted = 0;
    for id in rows("F04") {
        if parameter(&id, "owner") != "runner" || parameter(&id, "operation") == "INT-final-restore" {
            continue;
        }
        hosted += 1;
        let operation = parameter(&id, "operation");
        let (site, mode) = (site_of(operation), mode_of(parameter(&id, "mode")));
        // The runner installs in the order TERM, HUP, INT.
        let (signal, owners) = match (operation, mode) {
            ("action-query", _) => (15, ["default", "default", "default"]),
            ("INT-install", OMIT) => (2, ["default", "runner", "runner"]),
            ("INT-install", _) => (2, ["runner", "runner", "runner"]),
            ("TERM-install", OMIT) => (15, ["default", "default", "default"]),
            ("TERM-install", _) => (15, ["default", "runner", "default"]),
            other => panic!("{id}: {other:?}"),
        };
        let row = fault_row("handlers", site, mode, false);
        assert!(
            row.site_reached(site),
            "{id}: the incumbent never reached the site: {row:?}"
        );
        file(
            &id,
            &format!("F04 runner {operation}"),
            &row,
            &[
                ("refused-before-any-child", row.refused()),
                (
                    "names-the-failed-handler-operation",
                    row.line.contains(&format!("cannot install runner handler for signal {signal}")),
                ),
                ("EIO-in-the-report", row.line.contains("Input/output error")),
                ("no-spawn-errno", row.is("errno", "None")),
                ("site", row.site_record(site, mode != OMIT, false)),
                ("INT-owner", row.is("int", owners[0])),
                ("TERM-owner", row.is("term", owners[1])),
                ("HUP-owner", row.is("hup", owners[2])),
                ("no-stray-child", row.is("stray", "-1")),
            ],
        );
    }
    assert_eq!(hosted, 5);
}

#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_f06_child_restoration_faults() {
    let mut hosted = 0;
    for id in rows("F06") {
        hosted += 1;
        let operation = parameter(&id, "operation");
        let (site, mode) = (site_of(operation), mode_of(parameter(&id, "mode")));
        let row = fault_row("child", site, mode, true);
        assert!(
            row.site_reached(site),
            "{id}: the incumbent's child never reached the site: {row:?}"
        );
        file(
            &id,
            &format!("F06 {operation}"),
            &row,
            &[
                ("spawn-failed-before-any-Started", row.refused() && row.line.contains("cannot spawn")),
                ("actual-spawn-errno-EIO", row.is("errno", "Some(5)")),
                (
                    "childs-site-record-with-terminal-handed-over",
                    row.site_record(site, mode != OMIT, true),
                ),
                ("terminal-returned-to-the-launcher", row.is("foreground_restored", "1")),
                ("modes-unchanged", row.is("modes_unchanged", "1")),
                ("no-stray-child", row.is("stray", "-1")),
            ],
        );
    }
    assert_eq!(hosted, 6);
}

#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_paired_started_restore_and_child_action() {
    let mut hosted = 0;
    for id in rows("paired") {
        match parameter(&id, "case") {
            // F05: the Started parent restore (omitted, reported failed) with
            // the terminal handback's report failing during recovery.
            "F05-Started-restore-recovery-report" => {
                hosted += 1;
                let row = paired_row("started", 0, 0, true, "handback");
                file(
                    &id,
                    "paired F05 Started restore + recovery report",
                    &row,
                    &[
                        (
                            "primary-retained",
                            row.line.contains("Started parent transition failed: Input/output error"),
                        ),
                        ("secondary-retained", row.line.contains(HANDBACK_REPORT)),
                        (
                            "child-started-and-actually-reaped",
                            row.line.contains("events=[Started, Reaped]"),
                        ),
                        ("no-stray-child", row.is("stray", "-1")),
                        ("terminal-physically-returned", row.is("foreground_restored", "1")),
                    ],
                );
            }
            // F06: the TERM action restore omitted in the child, same report.
            "F06-action-C4-recovery-report" => {
                hosted += 1;
                let row = paired_row("child", TERM_RESTORE, OMIT, true, "handback");
                assert!(row.site_reached(TERM_RESTORE), "{id}: site not reached: {row:?}");
                file(
                    &id,
                    "paired F06 action + C4 recovery report",
                    &row,
                    &[
                        ("primary-retained", row.refused() && row.line.contains("cannot spawn")),
                        ("actual-spawn-errno", row.is("errno", "Some(5)")),
                        ("secondary-retained", row.line.contains(HANDBACK_REPORT)),
                        (
                            "childs-site-record-with-terminal-handed-over",
                            row.site_record(TERM_RESTORE, false, true),
                        ),
                        ("no-stray-child", row.is("stray", "-1")),
                        ("terminal-physically-returned", row.is("foreground_restored", "1")),
                    ],
                );
            }
            _ => {}
        }
    }
    assert_eq!(hosted, 2);
}

// ---- harness controls --------------------------------------------------------
//
// A control proves an observation is the incumbent's doing and not an
// instrument that reads the same whatever it is given. None files a row.

/// The F02 repair rows are rejected premises: under the named state (default
/// plus no-child-wait) the incumbent never performs the repair. Seen here as a
/// fact: the repair site is reached when SIGCHLD is *ignored* (the one state the
/// incumbent repairs) and is not reached under default plus no-child-wait,
/// while the query site is reached in both.
#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_control_f02_the_incumbent_repairs_only_an_ignored_disposition() {
    let ignored = fault_row("watching-ignored", WATCH_REPAIR, PERFORM, false);
    assert!(ignored.clean, "{ignored:?}");
    assert!(
        ignored.site_reached(WATCH_REPAIR),
        "the repair site was not reached for an ignored SIGCHLD: {ignored:?}"
    );
    let named = fault_row("watching", WATCH_REPAIR, PERFORM, false);
    assert!(named.clean, "{named:?}");
    assert!(
        !named.site_reached(WATCH_REPAIR),
        "the incumbent repaired default plus no-child-wait: {named:?}"
    );
    // The query is the incumbent's, in both states.
    let query = fault_row("watching", WATCH_QUERY, REPORT, false);
    assert!(query.site_reached(WATCH_QUERY), "{query:?}");
}

/// The same instrument sees a fault the incumbent does check: the mask restore
/// fails the spawn with EIO, so the F06 "spawn failed" reading is not a role
/// that cannot fail one.
#[test]
#[ignore = "private runner fault role; run native-incumbent.sh test"]
fn native_baseline_control_f06_a_checked_child_operation_fails_the_spawn() {
    let row = fault_row("child", MASK_RESTORE, OMIT, true);
    assert!(row.clean, "{row:?}");
    assert!(row.refused() && row.line.contains("cannot spawn"), "{row:?}");
    let unfaulted = fault_row("child", 0, 0, true);
    assert!(unfaulted.clean, "{unfaulted:?}");
    assert!(
        unfaulted.line.contains("end=Exited code=Some(7)"),
        "the unfaulted child must run and exit 7: {unfaulted:?}"
    );
}
