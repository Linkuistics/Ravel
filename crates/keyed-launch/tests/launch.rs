//! The launch half of the crate's public interface, exercised end to end
//! against a fake child and no consumer at all.
//!
//! Every child here is `/bin/sh` running a script the test wrote, which is what
//! makes the seam real: nothing below knows what a session is, and the argv
//! arrives the way a launcher's would — a program and arguments the caller
//! built.

mod support;

use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use keyed_launch::{run, run_observed, Argv, Channel, End, Escalation, Group, Launch, LaunchEvent};
use tempfile::TempDir;

/// The escalation on test timescales. Long enough that a poll tick lands inside
/// each phase, short enough that the whole suite stays in single-digit seconds.
const FAST: Escalation = Escalation {
    grace: Duration::from_millis(600),
    kill_grace: Duration::from_millis(900),
};

/// The two signals of the escalation, by number, so the tests can say which
/// step ran without taking a `libc` dependency of their own.
const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

struct Harness {
    dir: TempDir,
}

impl Harness {
    /// A scratch directory with a control directory for the channel.
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("control")).unwrap();
        Self { dir }
    }

    fn control(&self) -> PathBuf {
        self.dir.path().join("control")
    }

    fn script(&self, body: &str) -> PathBuf {
        let path = self.dir.path().join("child.sh");
        fs::write(&path, body).unwrap();
        path
    }

    /// `sh <script>`.
    fn argv(&self, script: &Path) -> Argv {
        Argv::new(OsString::from("sh"), vec![script.into()])
    }
}

/// A fixture report is separate from the appearance-only completion channel.
fn report_path(channel: &Channel) -> PathBuf {
    let mut path = channel.path().as_os_str().to_os_string();
    path.push(".report");
    path.into()
}

fn launch<'a>(
    argv: &'a Argv,
    channel: &'a Channel,
    scrub: &'a [&'a OsStr],
    cwd: Option<&'a Path>,
) -> Launch<'a> {
    Launch {
        argv,
        channel: Some((channel, "TEST_CHANNEL")),
        scrub,
        grant: &[],
        transparent: None,
        cwd,
        escalation: FAST,
    }
}

// ---------------------------------------------------------------------------
// The channel reaches the child, and the child's signal is reported

/// The whole loop in one case: the launcher publishes a path, the child writes
/// an empty file there, and the launcher reports its appearance. Nothing in between
/// knows what "done" means.
#[test]
fn a_child_signals_through_the_published_path() {
    let harness = Harness::new();
    let script = harness.script(": > \"$TEST_CHANNEL\"\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(ended.signalled);
    assert!(ended.status.success());
    assert_eq!(
        ended.end,
        End::Exited,
        "the child exited on its own before the grace elapsed, so nothing escalated"
    );
}

#[test]
fn a_child_that_never_signals_ends_unsignalled() {
    let harness = Harness::new();
    let script = harness.script("exit 3\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(!ended.signalled);
    assert_eq!(ended.end, End::Exited);
    assert_eq!(ended.status.code(), Some(3));
    assert!(
        !channel.path().exists(),
        "an unsignalled launch leaves no channel file"
    );
}

/// A child that signals and then takes its time still exits on its own terms,
/// so nothing escalates and `End` says so — even though a signal came back.
///
/// The grace is long here on purpose: the child's own exit has to land well
/// inside it, or the case under test is not the case being run.
#[test]
fn a_child_that_signals_and_exits_inside_the_grace_is_never_touched() {
    let harness = Harness::new();
    let script = harness.script(": > \"$TEST_CHANNEL\"\nsleep 1\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let patient = Escalation {
        grace: Duration::from_secs(30),
        kill_grace: FAST.kill_grace,
    };

    let ended = run(Launch {
        escalation: patient,
        ..launch(&argv, &channel, support::controls(), None)
    })
    .unwrap();

    assert!(ended.signalled);
    assert_eq!(
        ended.end,
        End::Exited,
        "a signal is not an escalation — nothing was sent to this child"
    );
    assert!(ended.status.success());
    assert!(
        ended.elapsed < patient.grace,
        "the run must not have waited out the grace: {:?}",
        ended.elapsed
    );
}

// ---------------------------------------------------------------------------
// The escalation

/// The case the escalation exists for: a child that has signalled and then goes
/// on waiting forever, exactly as an interactive one does when it returns to
/// its prompt.
#[test]
fn a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace() {
    let harness = Harness::new();
    // A loop of short sleeps rather than one long one: the child is killed
    // mid-wait, and a long-lived grandchild would go on holding the inherited
    // stdout pipe after its parent is reaped — which hangs the test *runner*,
    // not the test.
    let script = harness.script(": > \"$TEST_CHANNEL\"\nwhile : ; do sleep 0.05 ; done\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let mut events = Vec::new();
    let ended = run_observed(
        launch(&argv, &channel, support::controls(), None),
        &mut |e| events.push(e),
    )
    .unwrap();
    assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);

    assert_eq!(ended.end, End::Escalated);
    assert!(ended.signalled);
    assert!(
        !ended.status.success(),
        "a terminated child does not exit successfully: {:?}",
        ended.status
    );
    assert!(
        ended.elapsed >= FAST.grace,
        "the grace must elapse before anything is sent: {:?}",
        ended.elapsed
    );
    // Which signal ended it, rather than how long it took. The elapsed time
    // cannot separate the two steps: the poll interval is coarser than either
    // grace, so a child that dies promptly on SIGTERM can still be observed
    // later than `grace + kill_grace`. The signal says exactly which step ran.
    assert_eq!(
        ended.status.signal(),
        Some(SIGTERM),
        "a child that dies on SIGTERM must never reach SIGKILL"
    );
}

/// SIGTERM is a request. A child that declines it is still ended, which is what
/// makes the second step of the escalation load-bearing rather than defensive.
#[test]
fn a_child_that_ignores_sigterm_is_killed_after_the_kill_grace() {
    let harness = Harness::new();
    let script =
        harness.script("trap '' TERM\n: > \"$TEST_CHANNEL\"\nwhile : ; do sleep 0.05 ; done\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let mut events = Vec::new();
    let ended = run_observed(
        launch(&argv, &channel, support::controls(), None),
        &mut |e| events.push(e),
    )
    .unwrap();
    assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);

    assert_eq!(ended.end, End::Escalated);
    assert_eq!(
        ended.status.signal(),
        Some(SIGKILL),
        "a child that declines SIGTERM must still be ended"
    );
    assert!(
        ended.elapsed >= FAST.grace + FAST.kill_grace,
        "SIGKILL must wait out both graces: {:?}",
        ended.elapsed
    );
}

/// An unsignalled child is never touched: channel appearance, not a timeout, is what
/// authorises the escalation.
#[test]
fn an_unsignalled_child_runs_to_its_own_exit_untouched() {
    let harness = Harness::new();
    let script = harness.script("sleep 1\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(
        ended.status.success(),
        "an unsignalled child must exit on its own terms: {:?}",
        ended.status
    );
    assert_eq!(ended.end, End::Exited);
}

// ---------------------------------------------------------------------------
// The environment the child receives

/// Scrubbing is a **removal**, not an omission. The distinction is the whole
/// point: an environment is inherited, so a launcher that merely declines to
/// set a variable still passes on whatever its own held.
///
/// The two variables are `HOME` and `PATH` rather than invented ones, because
/// inventing them would mean writing the process environment — a global that
/// every parallel sibling test's own spawn reads at the same moment, which
/// hangs the runner rather than failing the test. These two are inherited by
/// construction, which is exactly the property under test.
#[test]
fn a_scrubbed_variable_is_removed_from_an_inherited_environment() {
    assert!(
        std::env::var_os("HOME").is_some() && std::env::var_os("PATH").is_some(),
        "this test needs both variables present in its own environment to have anything to say"
    );
    let harness = Harness::new();
    let script = harness
        .script("printf '%s|%s\\n' \"${PATH-<unset>}\" \"${HOME-<unset>}\" > \"$TEST_CHANNEL.report\"\n: > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let mut scrub = support::controls().to_vec();
    scrub.push(OsStr::new("HOME"));

    let ended = run(launch(&argv, &channel, &scrub, None)).unwrap();

    assert!(ended.signalled);
    let reported = fs::read_to_string(report_path(&channel)).unwrap();
    let (path, home) = reported.trim_end().split_once('|').unwrap();
    assert_eq!(
        home, "<unset>",
        "a scrubbed variable must be removed, not merely left unset"
    );
    assert_eq!(
        path,
        std::env::var("PATH").unwrap(),
        "everything else must arrive by ordinary inheritance"
    );
}

/// The grant survives a scrub list that names the channel variable itself.
///
/// That is the expected shape rather than a caller's mistake: the scrub list is
/// the launch-control variables a nested launcher must not inherit, and the
/// channel variable is the first of them. If the grant were applied before the
/// scrub, this launch would remove the path it had just published and the child
/// could never signal — which shows up as a session that hangs, not as an error.
#[test]
fn granting_the_channel_survives_a_scrub_list_that_names_it() {
    let harness = Harness::new();
    let script = harness.script(": > \"${TEST_CHANNEL?unset}\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let mut scrub = support::controls().to_vec();
    scrub.extend([OsStr::new("TEST_CHANNEL"), OsStr::new("HOME")]);

    let ended = run(launch(&argv, &channel, &scrub, None)).unwrap();

    assert!(
        ended.signalled,
        "the child could not reach its channel, so the grant was scrubbed away"
    );
}

/// A grant is set after the scrub, so it replaces a value the scrub removed,
/// and before the channel, so no grant can publish another path under the
/// channel's variable.
#[test]
fn a_grant_replaces_a_scrubbed_value_and_cannot_replace_the_channel() {
    let harness = Harness::new();
    let script =
        harness.script("printf '%s|%s\\n' \"${HOME-<unset>}\" \"$GRANTED\" > \"$TEST_CHANNEL.report\"\n: > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let mut scrub = support::controls().to_vec();
    scrub.push(OsStr::new("HOME"));
    let grant: [(&OsStr, &OsStr); 3] = [
        (OsStr::new("HOME"), OsStr::new("/granted/home")),
        (OsStr::new("GRANTED"), OsStr::new("yes")),
        (OsStr::new("TEST_CHANNEL"), OsStr::new("/nowhere")),
    ];

    let ended = run(Launch {
        grant: &grant,
        ..launch(&argv, &channel, &scrub, None)
    })
    .unwrap();

    assert!(ended.signalled, "the grant did not replace the channel");
    assert_eq!(
        fs::read_to_string(report_path(&channel)).unwrap(),
        "/granted/home|yes\n"
    );
}

/// The program is what is spawned, and `argv[0]` is what the child is told
/// it is. `ps` reads the child's own argument vector back.
#[test]
fn the_child_sees_its_arg0_while_its_program_is_what_runs() {
    let harness = Harness::new();
    let argv = Argv::new(
        OsString::from("/bin/sh"),
        vec![
            OsString::from("-c"),
            OsString::from("ps -o args= -p $$ > \"$TEST_CHANNEL.report\"\n: > \"$TEST_CHANNEL\""),
        ],
    )
    .with_arg0(OsString::from("chosen-name"));
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(ended.signalled);
    let args = fs::read_to_string(report_path(&channel)).unwrap();
    assert!(args.starts_with("chosen-name -c "), "{:?}", args);
}

/// The channel's appearance is the whole signal: an empty file is a signal
/// without content. A child that never touches the channel has not signalled.
#[test]
fn an_empty_channel_signals() {
    let harness = Harness::new();
    let channel = Channel::allocate(&harness.control()).unwrap();
    let script = harness.script(": > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(ended.signalled);
    assert_eq!(ended.end, End::Exited);

    let channel = Channel::allocate(&harness.control()).unwrap();
    let script = harness.script("exit 0\n");
    let argv = harness.argv(&script);
    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();
    assert!(!ended.signalled, "control: nothing appeared");
}

#[test]
fn the_channel_path_is_published_under_the_callers_chosen_variable_name() {
    let harness = Harness::new();
    // Written through the *caller's* name, recorded in a separate fixture output:
    // the two agree only because `channel_var` carried it.
    let script = harness.script(
        "printf '%s\\n' \"$TEST_CHANNEL\" > \"$TEST_CHANNEL.report\"\n: > \"$TEST_CHANNEL\"\n",
    );
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(ended.signalled);
    assert_eq!(
        fs::read_to_string(report_path(&channel))
            .unwrap()
            .trim_end(),
        channel.path().display().to_string()
    );
}

#[test]
fn the_child_starts_in_the_given_directory() {
    let harness = Harness::new();
    let elsewhere = harness.dir.path().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    let script = harness.script("pwd -P > \"$TEST_CHANNEL.report\"\n: > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(
        &argv,
        &channel,
        support::controls(),
        Some(&elsewhere),
    ))
    .unwrap();

    assert!(ended.signalled);
    let reported = PathBuf::from(
        fs::read_to_string(report_path(&channel))
            .unwrap()
            .trim_end(),
    );
    assert_eq!(
        reported.canonicalize().unwrap(),
        elsewhere.canonicalize().unwrap()
    );
}

// ---------------------------------------------------------------------------
// Refusals

#[test]
fn a_program_that_does_not_exist_names_itself_and_says_what_to_check() {
    let dir = TempDir::new().unwrap();
    let argv = Argv::new(
        OsString::from("no-such-program-anywhere"),
        vec![OsString::from("x")],
    );
    fs::create_dir(dir.path().join("control")).unwrap();
    let channel = Channel::allocate(&dir.path().join("control")).unwrap();

    let mut events = Vec::new();
    let error = run_observed(
        launch(&argv, &channel, support::controls(), None),
        &mut |e| events.push(e),
    )
    .unwrap_err();
    assert!(events.is_empty());

    let message = error.to_string();
    assert!(message.contains("no-such-program-anywhere"), "{message}");
    assert!(message.contains("executable"), "{message}");
}

/// Successive launches in one directory get channels that name them alone, so
/// one launch can never observe the signal another left.
#[test]
fn successive_launches_get_independent_channels() {
    let harness = Harness::new();
    let script = harness.script("printf '%s\\n' \"$TEST_CHANNEL\" > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);

    let first = Channel::allocate(&harness.control()).unwrap();
    let first_ended = run(launch(&argv, &first, support::controls(), None)).unwrap();
    let second = Channel::allocate(&harness.control()).unwrap();
    assert!(
        !second.appeared(),
        "the first channel cannot signal the second launch"
    );
    let second_ended = run(launch(&argv, &second, support::controls(), None)).unwrap();

    assert_ne!(first.path(), second.path());
    assert!(first_ended.signalled);
    assert!(second_ended.signalled);

    first.discard().unwrap();
    second.discard().unwrap();
    assert_eq!(
        fs::read_dir(harness.control()).unwrap().count(),
        0,
        "a discarded channel leaves nothing behind"
    );
}

/// A program and argument list its caller built is spawned whole and directly.
/// Text a shell would read as a variable, a quote or a word break is one
/// argument here, because nothing reads it a second time.
#[test]
fn a_caller_built_argv_is_spawned_whole_and_directly() {
    let harness = Harness::new();
    let record = harness.dir.path().join("args");
    let script = harness.script(&format!(
        "for argument in \"$@\"; do printf '%s\\0' \"$argument\"; done > {}\n\
         : > \"$TEST_CHANNEL\"\n",
        quoted(&record)
    ));
    let words = [
        "one two  three",
        "${script}",
        "it's \"quoted\"",
        "line\nbreak",
        "",
    ];
    let mut args = vec![script.into_os_string()];
    args.extend(words.map(OsString::from));
    let argv = Argv::new(OsString::from("/bin/sh"), args);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert!(ended.signalled);
    let received = fs::read_to_string(&record).unwrap();
    let received: Vec<&str> = received.split_terminator('\0').collect();
    assert_eq!(received, words);
}

// ---------------------------------------------------------------------------
// The child is a job of its own: its dispositions and its process group

/// A path interpolated into a fixture script, single-quoted.
///
/// Every path below is a `TempDir`'s, so this only has to survive spaces —
/// `/var/folders/…` on macOS. A path containing a single quote would need the
/// real dance, and no fixture here can produce one.
fn quoted(path: &Path) -> String {
    format!("'{}'", path.display())
}

/// Ignore SIGINT for as long as this value lives — a launcher's own policy,
/// which is what a driver that must survive a terminal Ctrl-C actually sets.
///
/// Restored on drop, because a disposition is process-global and every other
/// test in this binary shares it.
struct IgnoringSigint(libc::sighandler_t);

impl IgnoringSigint {
    fn new() -> Self {
        // SAFETY: `signal(2)` setting and later restoring one disposition.
        Self(unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) })
    }
}

impl Drop for IgnoringSigint {
    fn drop(&mut self) {
        // SAFETY: restoring the disposition this value replaced.
        unsafe { libc::signal(libc::SIGINT, self.0) };
    }
}

/// **An ignored disposition is the one kind that survives `execve`.** A
/// launcher that ignores SIGINT for its own reasons hands the ignore to its
/// child, to that child's children, and to every wrapper the template names —
/// and a non-interactive shell that inherits an ignored SIGINT keeps ignoring
/// it *and forces it on what it spawns*. An interactive session under
/// `sh -lc '…'` then cannot be interrupted at all, and nothing in it can say
/// why.
///
/// The child here **reports what it inherited** rather than installing a
/// handler of its own, which is the only fixture that can see the fault: a
/// child that installs a handler overwrites the inherited disposition, so it
/// behaves identically whether or not the launcher leaked one. `kill -INT $$`
/// against the inherited disposition is that report — the default action kills
/// a non-interactive shell, and an inherited ignore lets it run on to the line
/// below.
#[test]
fn an_ignored_sigint_in_the_launcher_does_not_reach_the_child() {
    const SIGINT: i32 = 2;

    let harness = Harness::new();
    let survived = harness.dir.path().join("survived-sigint");
    let script = harness.script(&format!(
        "kill -INT $$\nprintf 'inherited\\n' > {marker}\n",
        marker = quoted(&survived)
    ));

    let ignoring = IgnoringSigint::new();

    // The positive control, and it runs first on purpose. The same script under
    // a plain `Command` — no runner, no reset — *does* inherit the ignore and
    // writes the marker. Without seeing that, the assertion below would pass
    // just as well on a fixture that could never detect an inherited ignore at
    // all, which is the one thing it exists to rule out.
    let control = support::command("sh").arg(&script).status().unwrap();
    assert!(
        survived.exists(),
        "control: a plain spawn must inherit the launcher's ignored SIGINT — \
         the fixture cannot detect the fault it is testing for (status {control:?})"
    );
    fs::remove_file(&survived).unwrap();

    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    drop(ignoring);

    assert!(
        !survived.exists(),
        "the child inherited the launcher's ignored SIGINT: a session under a \
         wrapper cannot be interrupted, and neither can anything it spawns"
    );
    assert_eq!(
        ended.status.signal(),
        Some(SIGINT),
        "the child must have died of the SIGINT it sent itself, which is what \
         the default disposition means: {:?}",
        ended.status
    );
}

/// The escalation signals the child's **process group**, so a grandchild — a
/// tool subprocess, a language server, an agent's own in-flight command — is
/// reaped with its parent instead of surviving it and staying attached to the
/// terminal. A survivor is not merely untidy: it can hold a lock the launcher's
/// caller is about to wait on, and then the SIGKILL buys a stall rather than a
/// teardown.
#[test]
fn the_escalation_reaps_the_childs_descendants() {
    let harness = Harness::new();
    let grandchild_pid = harness.dir.path().join("grandchild-pid");
    // Signals, then declines to end: the launch reaches the full escalation,
    // and what it reaches for is what this test is about.
    let script = harness.script(&format!(
        "sh -c 'while : ; do sleep 0.05 ; done' &\n\
         printf '%s\\n' \"$!\" > {pid}\n\
         : > \"$TEST_CHANNEL\"\n\
         while : ; do sleep 0.05 ; done\n",
        pid = quoted(&grandchild_pid)
    ));
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    // The cross-check: the same shape of process, started at the same moment,
    // in *this* process's group rather than the child's. It must be untouched.
    // A fixture that reported "gone" for any pid — a `kill(2)` probe reading
    // the wrong errno, say — would look identical without it.
    let mut bystander = support::command("sh")
        .arg("-c")
        .arg("while : ; do sleep 0.05 ; done")
        .spawn()
        .unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();
    assert_eq!(ended.end, End::Escalated);

    let grandchild: i32 = fs::read_to_string(&grandchild_pid)
        .expect("the fixture never reported its grandchild")
        .trim()
        .parse()
        .unwrap();
    assert!(
        gone(grandchild),
        "the grandchild outlived the escalation that killed its parent"
    );
    assert!(
        bystander.try_wait().unwrap().is_none(),
        "the escalation reached a process outside the launched job's group"
    );

    bystander.kill().unwrap();
    bystander.wait().unwrap();
}

// ---------------------------------------------------------------------------
// The group ends with every launch

/// How the child under test ends, each leaving a TERM-ignoring descendant in
/// its group behind it.
#[derive(Clone, Copy, Debug)]
enum Leaves {
    /// It signals, and exits 7 on the escalation's TERM.
    OnTheTerm,
    /// It signals, and exits 5 well inside the grace.
    WithinTheGrace,
    /// It never signals, and exits 3 on its own.
    OnItsOwn,
}

/// **The group ends with the launch, whatever ended the child.** A descendant
/// that ignores the TERM outlives a child that exits on it. One still running
/// outlives a child that exits within the grace, or on its own without any
/// escalation. A caller that relaunched or published on that ending would act
/// beside it, so the runner kills what remains of the group before it reaps
/// the child. The check comes at the return boundary and allows no grace: the
/// runner has itself confirmed the group gone, so the descendant is not even a
/// zombie by then.
///
/// The child's own status stays the launch's in every case: the group's kill
/// comes after its exit and cannot rewrite it.
#[test]
fn a_term_ignoring_descendant_is_gone_before_the_launch_returns() {
    for leaves in [Leaves::OnTheTerm, Leaves::WithinTheGrace, Leaves::OnItsOwn] {
        let harness = Harness::new();
        let descendant_pid = harness.dir.path().join("descendant-pid");
        let ending = match leaves {
            Leaves::OnTheTerm => {
                "exec 2>/dev/null\ntrap 'exit 7' TERM\n: > \"$TEST_CHANNEL\"\nwhile : ; do sleep 0.05 ; done\n"
            }
            Leaves::WithinTheGrace => ": > \"$TEST_CHANNEL\"\nsleep 0.1\nexit 5\n",
            Leaves::OnItsOwn => "exit 3\n",
        };
        let script = harness.script(&format!(
            "sh -c 'trap \"\" TERM ; while : ; do sleep 0.05 ; done' &\n\
             printf '%s\\n' \"$!\" > {pid}\n\
             {ending}",
            pid = quoted(&descendant_pid)
        ));
        let argv = harness.argv(&script);
        let channel = Channel::allocate(&harness.control()).unwrap();

        let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

        let descendant: i32 = fs::read_to_string(&descendant_pid)
            .expect("the fixture never reported its descendant")
            .trim()
            .parse()
            .unwrap();
        let survived = !gone_now(descendant);
        if survived {
            // SAFETY: the fixture's own descendant, which must not outlive the
            // test that failed because of it.
            unsafe { libc::kill(descendant, libc::SIGKILL) };
        }
        assert!(
            !survived,
            "{leaves:?}: the TERM-ignoring descendant outlived the launch"
        );
        assert_eq!(ended.group, Group::Gone, "{leaves:?}");
        let (code, end) = match leaves {
            Leaves::OnTheTerm => (7, End::Escalated),
            Leaves::WithinTheGrace => (5, End::Exited),
            Leaves::OnItsOwn => (3, End::Exited),
        };
        assert_eq!(
            ended.status.code(),
            Some(code),
            "{leaves:?}: the child's own status is the launch's: {:?}",
            ended.status
        );
        assert_eq!(ended.end, end, "{leaves:?}");
    }
}

/// Whether `pid` is gone **now**: no wait for anything to reap it.
fn gone_now(pid: i32) -> bool {
    // SAFETY: `kill(2)` with signal 0 — the existence probe, which sends
    // nothing.
    let probed = unsafe { libc::kill(pid, 0) };
    probed == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}

/// **A stop is not an exit.** macOS's `waitid(WEXITED | WNOWAIT)` reports a
/// stopped child anyway, and keeps reporting it on every poll. A runner that
/// believed it would kill the group and reap a child that was only paused. So
/// the child here stops itself, stays stopped across several poll ticks, and
/// is then continued. It must finish its own script, and the launch must not
/// return until it has.
#[test]
fn a_stopped_child_is_neither_reaped_nor_killed() {
    use std::time::Instant;
    let harness = Harness::new();
    let pid_file = harness.dir.path().join("child-pid");
    let continued = harness.dir.path().join("continued");
    let script = harness.script(&format!(
        "printf '%s\\n' \"$$\" > {pid}\nkill -STOP $$\n: > {continued}\nexit 4\n",
        pid = quoted(&pid_file),
        continued = quoted(&continued)
    ));
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    std::thread::scope(|scope| {
        let runner =
            scope.spawn(|| run(launch(&argv, &channel, support::controls(), None)).unwrap());

        let deadline = Instant::now() + Duration::from_secs(5);
        let pid: i32 = loop {
            if let Some(pid) = fs::read_to_string(&pid_file)
                .ok()
                .and_then(|text| text.trim().parse().ok())
            {
                break pid;
            }
            assert!(
                Instant::now() < deadline,
                "the child never reported its pid"
            );
            std::thread::sleep(Duration::from_millis(10));
        };
        while !stopped(pid) {
            assert!(Instant::now() < deadline, "the child never stopped");
            std::thread::sleep(Duration::from_millis(10));
        }
        // Several of the runner's half-second poll ticks, each of which sees
        // the stop on macOS.
        std::thread::sleep(Duration::from_millis(1600));
        assert!(
            !runner.is_finished(),
            "the launch returned while its child was only stopped"
        );
        assert!(stopped(pid), "the stopped child was killed or reaped");

        // SAFETY: continuing the fixture's own stopped child.
        unsafe { libc::kill(pid, libc::SIGCONT) };
        let ended = runner.join().unwrap();
        assert_eq!(
            ended.status.code(),
            Some(4),
            "the continued child must finish its own script: {:?}",
            ended.status
        );
        assert!(continued.exists());
        assert_eq!(ended.end, End::Exited);
        assert_eq!(ended.group, Group::Gone);
    });
}

/// Whether `pid` is a stopped process, read from `ps`, which is the same
/// observation on macOS and Linux.
fn stopped(pid: i32) -> bool {
    support::command("ps")
        .args(["-o", "stat=", "-p", &pid.to_string()])
        .output()
        .is_ok_and(|out| {
            String::from_utf8_lossy(&out.stdout)
                .trim_start()
                .starts_with('T')
        })
}

/// Whether `pid` is gone, waiting out the moment between its parent's death and
/// `init` reaping it.
fn gone(pid: i32) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        // SAFETY: `kill(2)` with signal 0 — the existence probe, which sends
        // nothing.
        if unsafe { libc::kill(pid, 0) } == -1
            && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        {
            return true;
        }
        if std::time::Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn observed_immediate_exit_and_failed_spawn_have_exact_events() {
    use keyed_launch::{run_observed, LaunchEvent};
    let harness = Harness::new();
    let script = harness.script("exit 3\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let parent = std::process::id();
    let mut events = Vec::new();
    let ended = run_observed(
        launch(&argv, &channel, support::controls(), None),
        &mut |event| {
            assert_eq!(std::process::id(), parent);
            events.push(event);
        },
    )
    .unwrap();
    assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);
    assert_eq!(ended.status.code(), Some(3));
    assert_eq!(ended.end, End::Exited);
    assert!(!ended.signalled);

    events.clear();
    let missing = harness.dir.path().join("missing-directory");
    assert!(run_observed(
        launch(&argv, &channel, support::controls(), Some(&missing)),
        &mut |event| { events.push(event) }
    )
    .is_err());
    assert!(events.is_empty());
}

#[test]
fn a_channel_appearance_does_not_emit_reaped_before_the_child_exits() {
    use std::sync::mpsc;
    use std::time::Instant;
    let harness = Harness::new();
    let release = harness.dir.path().join("release");
    let script = harness
        .script(": > \"$TEST_CHANNEL\"\nwhile [ ! -f release ]; do sleep 0.01; done\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let (send, receive) = mpsc::channel();
    std::thread::scope(|scope| {
        let runner = scope.spawn(|| {
            run_observed(
                Launch {
                    escalation: Escalation {
                        grace: Duration::from_secs(5),
                        kill_grace: Duration::ZERO,
                    },
                    ..launch(
                        &argv,
                        &channel,
                        support::controls(),
                        Some(harness.dir.path()),
                    )
                },
                &mut |event| send.send(event).unwrap(),
            )
            .unwrap()
        });
        assert_eq!(
            receive.recv_timeout(Duration::from_secs(5)).unwrap(),
            LaunchEvent::Started
        );
        let deadline = Instant::now() + Duration::from_secs(5);
        while !channel.path().exists() {
            assert!(
                Instant::now() < deadline,
                "child never published its signal"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(receive.try_recv(), Err(mpsc::TryRecvError::Empty));
        fs::write(release, "exit").unwrap();
        assert_eq!(
            receive.recv_timeout(Duration::from_secs(5)).unwrap(),
            LaunchEvent::Reaped
        );
        let ended = runner.join().unwrap();
        assert!(ended.status.success());
        assert_eq!(ended.end, End::Exited);
        assert!(ended.signalled);
        assert_eq!(receive.try_recv(), Err(mpsc::TryRecvError::Empty));
    });
}
