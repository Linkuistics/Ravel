//! The role and rows for the runner-owned F rows. Test builds only; the sites
//! themselves live in `native_hooks`.
use super::*;
use crate::native_hooks::*;
use std::process::Stdio;
use std::sync::atomic::AtomicBool;

// ---- the role and its rows ---------------------------------------------------
//
// One role process per row, owned and settled by the shared controller. It
// installs the row's fixed site, runs one real launch of a real child and
// writes what physically happened. F06 runs it on a PTY of its own so the
// launch holds a terminal lease and the failed spawn exercises real recovery.

const ROLE: &str = "run::native_faults::native_fault_role";

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
#[ignore = "private runner fault role; run task test:native-entry"]
fn native_fault_role() {
    let Some(site) = std::env::var_os("K28_FAULT_SITE") else {
        return;
    };
    let number = |name: &str| -> u8 { std::env::var(name).unwrap().parse().unwrap() };
    let _ = site;
    let (site, mode) = (number("K28_FAULT_SITE"), number("K28_FAULT_MODE"));
    let ignore = std::env::var("K28_FAULT_BREAK").unwrap() == "ignore";
    let case = std::env::var("K28_FAULT_CASE").unwrap();
    let mut fds = [0 as libc::c_int; 2];
    // SAFETY: pipe fills two descriptors this role owns until it exits.
    assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
    if case == "watching" {
        // The private post-entry action: default disposition plus no-child-wait.
        let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
        action.sa_sigaction = libc::SIG_DFL;
        action.sa_flags = libc::SA_NOCLDWAIT;
        assert_eq!(unsafe { libc::sigemptyset(&mut action.sa_mask) }, 0);
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) },
            0
        );
    }
    let terminal = Terminal::open();
    let own = own_group();
    let modes_before = terminal.as_ref().map(termios_bytes);
    select(site, mode, ignore, fds[1]);
    // A paired secondary failure: the terminal handback's own report.
    match std::env::var("K28_FAULT_PAIR").as_deref() {
        Ok("handback") => terminal_tests::inject_handback_report(),
        Ok("dropped") => {
            terminal_tests::inject_handback_report();
            terminal_tests::drop_recovery_reports();
        }
        _ => {}
    }
    // The stand-in for the front's Started parent-mask restore: omitted and
    // reported failed. The role has no mask of its own to restore.
    let started_fails = case == "started";
    // A queued handled signal under the parent's known handoff block: pending
    // when the spawn fails or the check fails, joined only afterwards.
    let queued = std::env::var("K28_FAULT_QUEUE_TERM").is_ok();
    if queued {
        let mut handled = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        // SAFETY: the set is initialised before use; the role's own thread mask.
        unsafe {
            libc::sigemptyset(&mut handled);
            libc::sigaddset(&mut handled, libc::SIGTERM);
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, &handled, std::ptr::null_mut()),
                0
            );
            assert_eq!(libc::raise(libc::SIGTERM), 0);
        }
    }
    let argv = crate::Argv::new("/bin/sh".into(), vec!["-c".into(), "exit 7".into()]);
    let entry = EntrySignals::new([], []);
    let mut events = Vec::new();
    let result = run_observed_with_start_check(
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
        &mut |event| events.push(event),
        &mut || {
            if started_fails {
                Err(std::io::Error::from_raw_os_error(libc::EIO))
            } else {
                Ok(())
            }
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
    // The queued TERM is still pending behind the role's own block: nothing in
    // the failed launch consumed it. The role then joins it the way a front's
    // finalisation does, by restoring the mask under the runner's handler.
    let mut pending = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    // SAFETY: an initialised output set, a read-only query.
    let pending_term = unsafe {
        libc::sigpending(&mut pending);
        libc::sigismember(&pending, libc::SIGTERM)
    };
    let latch_before = INTERRUPTED_BY.load(Ordering::SeqCst);
    if queued {
        let mut handled = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        // SAFETY: restoring exactly the signal this role blocked above.
        unsafe {
            libc::sigemptyset(&mut handled);
            libc::sigaddset(&mut handled, libc::SIGTERM);
            libc::pthread_sigmask(libc::SIG_UNBLOCK, &handled, std::ptr::null_mut());
        }
    }
    let latch_after = INTERRUPTED_BY.load(Ordering::SeqCst);
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
    let line =
        format!(
        "events={events:?} {ended} records={records:?} stray={stray} stray_errno={stray_errno:?} \
         chld_nocldwait={} int={} term={} hup={} foreground_restored={} modes_unchanged={} \
         pending_term={pending_term} latch_before={latch_before} latch_after={latch_after}",
        i32::from(chld.sa_flags & libc::SA_NOCLDWAIT != 0),
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
    /// The launch ended in an error whose text names `text`, before any child.
    fn refused(&self, text: &str) -> bool {
        self.line.contains("error=") && self.line.contains(text) && self.line.contains("events=[]")
    }
    /// One two-byte record: this site, performed or omitted, and whether the
    /// operation ran while the terminal's foreground group was its own.
    fn site_record(&self, site: u8, performed: bool, foreground: bool) -> bool {
        let flags = u8::from(performed) | (u8::from(foreground) << 2);
        self.line.contains(&format!("records=[{site}, {flags}]"))
    }
    fn admitted(&self) -> bool {
        self.line.contains("events=[Started")
    }
}

fn fault_row(case: &str, site: u8, mode: u8, broken: &str, pty: bool) -> Row {
    paired_row(case, site, mode, broken, pty, "", false)
}

/// `handback`: add the C4 terminal-handback report as the secondary failure.
/// `queue`: queue an actual TERM under the role's own block before the launch.
fn paired_row(
    case: &str,
    site: u8,
    mode: u8,
    broken: &str,
    pty: bool,
    handback: &str,
    queue: bool,
) -> Row {
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
        .env("K28_FAULT_BREAK", broken)
        .env("K28_FAULT_CASE", case)
        .env("K28_FAULT_OUT", &out);
    if !handback.is_empty() {
        command.env("K28_FAULT_PAIR", handback);
    }
    if queue {
        command.env("K28_FAULT_QUEUE_TERM", "1");
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
    eprintln!("native runner fault case={case} site={site} mode={mode} broken={broken}: {row:?} {settlement:?}");
    row
}

fn ensure_conforms(label: &str, row: &Row, checks: &[(&str, bool)]) {
    let failed: Vec<_> = checks
        .iter()
        .filter(|(_, ok)| !ok)
        .map(|(n, _)| *n)
        .collect();
    assert!(
        row.clean && failed.is_empty(),
        "{label}: controller={} failed={failed:?} {row:?}",
        row.clean
    );
}

#[test]
fn native_f02_managed_watching_faults_refuse_before_any_child() {
    // (site, mode, text, no-child-wait left set after the site)
    let rows = [
        (WATCH_QUERY, REPORT, "cannot query child watching", true),
        (WATCH_REPAIR, OMIT, "cannot repair child watching", true),
        (WATCH_REPAIR, PERFORM, "cannot repair child watching", false),
    ];
    for (site, mode, text, left_set) in rows {
        let label = format!("F02 managed site={site} mode={mode}");
        let row = fault_row("watching", site, mode, "none", false);
        ensure_conforms(
            &label,
            &row,
            &[
                ("refused before any child", row.refused(text)),
                // The raw accessor is spawn-only: a setup failure carries its
                // errno in the text, never in `raw_os_error`.
                ("EIO in the report", row.line.contains("Input/output error")),
                ("no spawn errno", row.is("errno", "None")),
                ("site", row.site_record(site, mode != OMIT, false)),
                (
                    "actual flag state",
                    row.is("chld_nocldwait", if left_set { "1" } else { "0" }),
                ),
                ("no stray child", row.is("stray", "-1")),
            ],
        );
        // The ignored failure admits the child although watching was never
        // established.
        let broken = fault_row("watching", site, mode, "ignore", false);
        assert!(broken.clean, "{label}: controller {broken:?}");
        assert!(
            broken.admitted() && !broken.refused(text),
            "{label}: ignoring the failure still refused: {broken:?}"
        );
    }
}

#[test]
fn native_f04_runner_handler_faults_fail_admission_with_the_partial_subset_owned() {
    // (site, mode, signal named in the report, owners of INT/TERM/HUP after)
    // The runner installs in the order TERM, HUP, INT.
    let rows = [
        (HANDLER_QUERY, REPORT, 15, ["default", "default", "default"]),
        (INT_INSTALL, OMIT, 2, ["default", "runner", "runner"]),
        (INT_INSTALL, PERFORM, 2, ["runner", "runner", "runner"]),
        (TERM_INSTALL, OMIT, 15, ["default", "default", "default"]),
        (TERM_INSTALL, PERFORM, 15, ["default", "runner", "default"]),
    ];
    for (site, mode, signal, owners) in rows {
        let label = format!("F04 runner site={site} mode={mode}");
        let text = format!("cannot install runner handler for signal {signal}");
        let row = fault_row("handlers", site, mode, "none", false);
        ensure_conforms(
            &label,
            &row,
            &[
                ("refused before any child", row.refused(&text)),
                ("EIO in the report", row.line.contains("Input/output error")),
                ("no spawn errno", row.is("errno", "None")),
                ("site", row.site_record(site, mode != OMIT, false)),
                ("INT owner", row.is("int", owners[0])),
                ("TERM owner", row.is("term", owners[1])),
                ("HUP owner", row.is("hup", owners[2])),
                ("no stray child", row.is("stray", "-1")),
            ],
        );
        let broken = fault_row("handlers", site, mode, "ignore", false);
        assert!(broken.clean, "{label}: controller {broken:?}");
        assert!(
            broken.admitted() && !broken.refused(&text),
            "{label}: ignoring the failure still refused: {broken:?}"
        );
    }
}

#[test]
fn native_f06_child_restoration_faults_fail_the_spawn_and_recover_the_terminal() {
    let rows = [
        (TERM_RESTORE, OMIT),
        (TERM_RESTORE, PERFORM),
        (MASK_BUILD, OMIT),
        (MASK_BUILD, PERFORM),
        (MASK_RESTORE, OMIT),
        (MASK_RESTORE, PERFORM),
    ];
    for (site, mode) in rows {
        let label = format!("F06 site={site} mode={mode}");
        let row = fault_row("child", site, mode, "none", true);
        ensure_conforms(
            &label,
            &row,
            &[
                (
                    "spawn failed before any Started",
                    row.refused("cannot spawn"),
                ),
                ("actual spawn errno EIO", row.is("errno", "Some(5)")),
                (
                    "child's site record, terminal handed over",
                    row.site_record(site, mode != OMIT, true),
                ),
                (
                    "terminal returned to the launcher",
                    row.is("foreground_restored", "1"),
                ),
                ("modes unchanged", row.is("modes_unchanged", "1")),
                ("no stray child", row.is("stray", "-1")),
            ],
        );
        let broken = fault_row("child", site, mode, "ignore", true);
        assert!(broken.clean, "{label}: controller {broken:?}");
        assert!(
            broken.admitted() && broken.line.contains("end=Exited code=Some(7)"),
            "{label}: ignoring the failure still failed the spawn: {broken:?}"
        );
    }
}

const HANDBACK_REPORT: &str = "C4 substituted terminal handback report after physical operation";

/// Paired failures: the omitted primary operation plus a reported secondary
/// one. Both must survive in the returned error, beside the real cleanup.
#[test]
fn native_paired_started_restore_and_child_action_keep_the_recovery_report() {
    // F05: the Started parent restore (omitted, reported failed) with the
    // terminal handback's report failing during real recovery of a live child.
    let started = paired_row("started", 0, 0, "none", true, "handback", false);
    ensure_conforms(
        "paired F05 Started restore + recovery report",
        &started,
        &[
            (
                "primary retained",
                started
                    .line
                    .contains("Started parent transition failed: Input/output error"),
            ),
            ("secondary retained", started.line.contains(HANDBACK_REPORT)),
            (
                "child started and actually reaped",
                started.line.contains("events=[Started, Reaped]"),
            ),
            ("no stray child", started.is("stray", "-1")),
            // The handback physically ran; only its report was substituted.
            (
                "terminal physically returned",
                started.is("foreground_restored", "1"),
            ),
        ],
    );
    // F06: the TERM action restore omitted in the child, with the same report.
    let child = paired_row("child", TERM_RESTORE, OMIT, "none", true, "handback", false);
    ensure_conforms(
        "paired F06 action + C4 recovery report",
        &child,
        &[
            ("primary retained", child.refused("cannot spawn")),
            ("actual spawn errno", child.is("errno", "Some(5)")),
            ("secondary retained", child.line.contains(HANDBACK_REPORT)),
            (
                "child's site record, terminal handed over",
                child.site_record(TERM_RESTORE, false, true),
            ),
            ("no stray child", child.is("stray", "-1")),
            (
                "terminal physically returned",
                child.is("foreground_restored", "1"),
            ),
        ],
    );
    // Breaking the retained secondary: the same physical handback with its
    // substituted report discarded by the product. The primary stands alone.
    for (case, site, mode) in [("started", 0, 0), ("child", TERM_RESTORE, OMIT)] {
        let dropped = paired_row(case, site, mode, "none", true, "dropped", false);
        assert!(dropped.clean, "{dropped:?}");
        assert!(
            !dropped.line.contains(HANDBACK_REPORT) && dropped.line.contains("error="),
            "dropping the secondary report was not visible: {dropped:?}"
        );
    }
}

/// The queued-TERM variants at the runner: an actual TERM pending under the
/// parent's known block when the failure occurs. Nothing consumes it before
/// the parent's finalisation, and the failure's own reports are retained.
#[test]
fn native_queued_term_stays_pending_through_a_failed_started_or_spawn() {
    for (label, case, site, mode) in [
        ("F05 Started restore", "started", 0, 0),
        ("F06 action restore", "child", TERM_RESTORE, OMIT),
    ] {
        let row = paired_row(case, site, mode, "none", true, "", true);
        ensure_conforms(
            label,
            &row,
            &[
                (
                    "TERM was actually pending after the failure",
                    row.is("pending_term", "1"),
                ),
                (
                    "the failed launch consumed nothing",
                    row.is("latch_before", "0"),
                ),
                // Joined only when the parent restores its mask, by the
                // runner's handler: transfer to finalisation, not pre-spawn
                // consumption while blocked.
                (
                    "joined at the parent's restore",
                    row.is("latch_after", "15"),
                ),
                ("failure reported", row.line.contains("error=")),
                ("no stray child", row.is("stray", "-1")),
                ("terminal returned", row.is("foreground_restored", "1")),
            ],
        );
    }
}
