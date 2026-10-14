//! Private allocator instrumentation. No product environment switches.
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::{fs, io};
#[derive(Default)]
pub(crate) struct Control {
    pub(crate) role: String,
    pub(crate) step: String,
    pub(crate) mutation: String,
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) attempts: usize,
    pub(crate) route: String,
    pub(crate) all_created: Vec<(String, PathBuf)>,
    pub(crate) all_attempts: Vec<String>,
}
thread_local! { pub(crate) static CONTROL: RefCell<Control> = RefCell::new(Control::default()); }

pub(crate) fn created(role: &str, path: &Path) {
    CONTROL.with(|c| {
        let mut c = c.borrow_mut();
        c.all_created.push((role.to_owned(), path.to_owned()));
        if c.role == role {
            c.paths.push(path.to_owned());
        }
    });
    receipt(format!(
        "physical acquisition role={role} path={}",
        path.display()
    ));
}
pub(crate) fn parent(role: &str, parent: PathBuf) -> PathBuf {
    CONTROL.with(|c| {
        if c.borrow().role == role && c.borrow().step == "create" {
            parent.join("missing-parent")
        } else {
            parent
        }
    })
}
pub(crate) fn operation<T>(
    role: &str,
    step: &str,
    path: &Path,
    result: io::Result<T>,
) -> io::Result<T> {
    let value = result?;
    let fail = CONTROL.with(|c| c.borrow().role == role && c.borrow().step == step);
    if fail {
        mutate(role, path);
        if role == "session" {
            signal(false);
        }
        receipt(format!(
            "reported acquisition role={role} step={step} errno=5"
        ));
        return Err(io::Error::from_raw_os_error(libc::EIO));
    }
    Ok(value)
}
fn mutate(role: &str, path: &Path) {
    CONTROL.with(|c| {
        let c = c.borrow();
        if c.role != role {
            return;
        }
        match c.mutation.as_str() {
            "move" | "replace" | "symlink" => {
                fs::rename(path, path.with_extension("moved")).unwrap();
                if c.mutation == "replace" {
                    fs::create_dir(path).unwrap();
                    fs::write(path.join("unrelated"), "keep").unwrap();
                }
                if c.mutation == "symlink" {
                    let target = path.with_extension("target");
                    fs::create_dir(&target).unwrap();
                    fs::write(target.join("unrelated"), "keep").unwrap();
                    std::os::unix::fs::symlink(target, path).unwrap();
                }
            }
            _ => {}
        }
    });
}
pub(crate) fn cleanup(role: &str, path: &Path) -> io::Result<()> {
    CONTROL.with(|c| c.borrow_mut().all_attempts.push(role.to_owned()));
    let outer_failure = CONTROL.with(|c| {
        role == "confined"
            && (c.borrow().route.starts_with("worker-") || c.borrow().route.starts_with("cancel-"))
    });
    if outer_failure {
        return Err(io::Error::new(
            io::ErrorKind::ResourceBusy,
            "remove_dir_all: injected outer busy error (os error 16)",
        ));
    }
    let active = CONTROL.with(|c| c.borrow().role == role);
    if !active {
        return Ok(());
    }
    CONTROL.with(|c| c.borrow_mut().attempts += 1);
    let step = CONTROL.with(|c| c.borrow().step.clone());
    if step == "cleanup" {
        mutate(role, path);
    }
    if role == "session" {
        signal(true);
    }
    let mutation = CONTROL.with(|c| c.borrow().mutation.clone());
    receipt(format!(
        "owner cleanup entered role={role} path={} mutation={mutation}",
        path.display()
    ));
    if mutation == "remove" {
        receipt(format!(
            "reported remove_dir_all EACCES role={role}; physical removal not attempted"
        ));
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "remove_dir_all: injected permission denied (os error 13)",
        ));
    }
    if mutation == "unknown" {
        receipt(format!(
            "reported identity-check failure role={role}; current symlink_metadata not attempted"
        ));
        return Err(io::Error::other(
            "identity check unavailable; identity unknown",
        ));
    }
    Ok(())
}

#[allow(dead_code)] // library test target only exercises the session allocator
pub(crate) fn before_spawn(worker: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    CONTROL.with(|c| {
        let c = c.borrow();
        if c.role == "worker" {
            if c.step == "spawn_missing" {
                fs::remove_file(worker).unwrap();
            }
            if c.step == "spawn_other" {
                fs::set_permissions(worker, fs::Permissions::from_mode(0o600)).unwrap();
            }
        }
    });
}

fn signal(late: bool) {
    let route = CONTROL.with(|c| c.borrow().route.clone());
    if late != (route == "cancel-late") {
        return;
    }
    let first = match route.as_str() {
        "cancel-int" => libc::SIGINT,
        "cancel-term" | "cancel-late" => libc::SIGTERM,
        "cancel-hup" => libc::SIGHUP,
        _ => return,
    };
    assert_eq!(unsafe { libc::raise(first) }, 0);
    assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);
}

thread_local! {static RECEIPTS: RefCell<Vec<String>> = const {RefCell::new(Vec::new())};}
pub(crate) fn receipt(line: String) {
    RECEIPTS.with(|r| r.borrow_mut().push(line));
}
#[allow(dead_code)] // library target does not run subprocess fixture roles
pub(crate) fn receipts() -> Vec<String> {
    RECEIPTS.with(|r| std::mem::take(&mut *r.borrow_mut()))
}

pub(crate) fn before_channel(path: &Path) {
    if CONTROL.with(|c| c.borrow().step == "channel-real") {
        mutate("session", path);
    }
}

#[allow(dead_code)] // used by binary worker fixtures, not the library target
pub(crate) fn worker_reports(
    status: &mut io::Result<std::process::ExitStatus>,
    reports: &mut Vec<String>,
) {
    let route = CONTROL.with(|c| c.borrow().route.clone());
    if !route.starts_with("worker-") {
        return;
    }
    assert!(
        status.is_ok(),
        "actual worker reap failed before substitution"
    );
    if route.contains("nonzero") {
        assert_eq!(status.as_ref().unwrap().code(), Some(17));
    }
    receipt(format!(
        "physical worker wait={status:?}; subsequent reported wait=ECHILD and secondary poll=EIO"
    ));
    *status = Err(io::Error::from_raw_os_error(libc::ECHILD));
    reports.push("try_wait: distinct reported poll failure (os error 5)".into());
    let signal = if route.contains("cancel-int") {
        Some(libc::SIGINT)
    } else if route.contains("cancel-term") {
        Some(libc::SIGTERM)
    } else if route.contains("cancel-hup") {
        Some(libc::SIGHUP)
    } else {
        None
    };
    if let Some(signal) = signal {
        assert_eq!(unsafe { libc::raise(signal) }, 0);
        assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);
    }
}
