//! Exercise the operating-system boundary, not just the generated policy text.
mod support;

use std::fs;
use std::time::Duration;

use std::ffi::OsString;

use keyed_launch::{Argv, Channel, Escalation, Launch};

#[test]
fn confined_child_and_descendants_cannot_read_or_write_outside_the_invocation() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let secret = root.join("parent-project.txt");
    fs::write(&secret, "parent contents").unwrap();
    std::os::unix::fs::symlink(&secret, work.join("link")).unwrap();
    let script = work.join("probe.sh");
    fs::write(&script, format!(
        "set -eu\nif cat '{}' ; then exit 41; fi\nif cat link; then exit 42; fi\nif sh -c 'echo changed > \"$1\"' probe '{}'; then exit 43; fi\nprintf allowed > result\n: > \"$TEST_CHANNEL\"\n",
        secret.display(), secret.display()
    )).unwrap();
    let argv = Argv::new(OsString::from("/bin/sh"), vec![script.into_os_string()]);
    let channel = Channel::allocate(&work).unwrap();
    let ended = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "TEST_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: &[],
            read_trees: &[],
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(ended.status.success(), "sandbox failed: {:?}", ended.status);
    assert!(ended.signalled);
    assert_eq!(fs::read_to_string(secret).unwrap(), "parent contents");
    assert_eq!(fs::read_to_string(work.join("result")).unwrap(), "allowed");
}

#[test]
fn explicit_runtime_file_grant_allows_reads_but_denies_writes() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let runtime_file = root.join("runtime-credential.txt");
    fs::write(&runtime_file, "runtime contents").unwrap();
    let script = work.join("runtime-probe.sh");
    fs::write(
        &script,
        "set -eu\n\
         test \"$(cat \"$1\")\" = 'runtime contents'\n\
         if sh -c 'printf changed > \"$1\"' probe \"$1\"; then exit 41; fi\n\
         if sh -c 'rm \"$1\"' probe \"$1\"; then exit 42; fi\n\
         printf read-only > result\n\
         : > \"$TEST_CHANNEL\"\n",
    )
    .unwrap();
    let argv = Argv::new(
        OsString::from("/bin/sh"),
        vec![
            script.into_os_string(),
            runtime_file.clone().into_os_string(),
        ],
    );
    let channel = Channel::allocate(&work).unwrap();
    let ended = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "TEST_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: std::slice::from_ref(&runtime_file),
            read_trees: &[],
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(ended.status.success(), "sandbox failed: {:?}", ended.status);
    assert!(ended.signalled);
    assert_eq!(
        fs::read_to_string(runtime_file).unwrap(),
        "runtime contents"
    );
    assert_eq!(
        fs::read_to_string(work.join("result")).unwrap(),
        "read-only"
    );
}

/// A confined launch grants and runs the file its caller named. A name would
/// need a lookup, and the runner has none: a relative path refuses as a bare
/// name does, although `sh` is on PATH and the relative file exists.
#[test]
fn a_confined_program_that_is_not_an_absolute_path_refuses() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let marker = work.join("ran");
    fs::write(work.join("probe.sh"), "#!/bin/sh\n: > ran\n").unwrap();
    for program in ["sh", "./probe.sh", "work/probe.sh"] {
        let argv = Argv::new(OsString::from(program), vec![]);
        let channel = Channel::allocate(&work).unwrap();
        let error = keyed_launch::run_confined_observed(
            Launch {
                argv: &argv,
                channel: Some((&channel, "TEST_CHANNEL")),
                scrub: support::controls(),
                grant: &[],
                transparent: None,
                cwd: Some(&work),
                escalation: Escalation {
                    grace: Duration::ZERO,
                    kill_grace: Duration::from_millis(100),
                },
            },
            &keyed_launch::FilesystemGrants {
                writable: std::slice::from_ref(&work),
                runtime_read: &[],
                read_trees: &[],
            },
            &mut |_| {},
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("absolute program path"), "{error}");
        assert!(error.contains(program), "{error}");
        assert!(!marker.exists(), "{program} was launched");
    }
}

fn quick() -> Escalation {
    Escalation {
        grace: Duration::ZERO,
        kill_grace: Duration::from_millis(100),
    }
}

/// A read-only tree outside the writable directory: a child and its
/// descendant read every entry, also through a relative link inside the tree.
/// Neither can write, change the mode of, remove or rename an entry, and a
/// sibling of the tree stays unreadable. `chmod` skips a mode that does not
/// change, so the mode check uses a file without a write bit.
#[test]
fn a_read_only_tree_reads_through_links_and_denies_every_write() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    let tree = root.join("tree");
    fs::create_dir_all(&work).unwrap();
    fs::create_dir_all(tree.join("skills/alpha")).unwrap();
    fs::write(tree.join("skills/alpha/SKILL.md"), "alpha body").unwrap();
    fs::write(tree.join("shared.md"), "shared body").unwrap();
    // The mode of `shared.md` admits writes, so only the sandbox stops them.
    // `locked.md` has no write bit, so that a mode change is a real change.
    fs::write(tree.join("locked.md"), "locked").unwrap();
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(tree.join("locked.md"), fs::Permissions::from_mode(0o444)).unwrap();
    }
    std::os::unix::fs::symlink("../../shared.md", tree.join("skills/alpha/shared")).unwrap();
    std::os::unix::fs::symlink("skills", tree.join("linked")).unwrap();
    fs::write(root.join("sibling.txt"), "sibling").unwrap();
    let script = work.join("probe.sh");
    fs::write(
        &script,
        "set -u\n\
         t=$1\n\
         test \"$(cat \"$t/skills/alpha/SKILL.md\")\" = 'alpha body' || exit 41\n\
         test \"$(sh -c 'cat \"$1\"' d \"$t/skills/alpha/shared\")\" = 'shared body' || exit 42\n\
         test \"$(cat \"$t/linked/alpha/SKILL.md\")\" = 'alpha body' || exit 43\n\
         ls \"$t/skills\" | grep -qx alpha || exit 44\n\
         if sh -c 'printf x > \"$1\"' d \"$t/shared.md\" 2>/dev/null; then exit 45; fi\n\
         if sh -c 'printf x > \"$1\"' d \"$t/skills/alpha/shared\" 2>/dev/null; then exit 46; fi\n\
         if chmod u+w \"$t/locked.md\" 2>/dev/null; then exit 47; fi\n\
         if rm -f \"$t/shared.md\" 2>/dev/null; then test -e \"$t/shared.md\" || exit 48; fi\n\
         if mv \"$t/skills\" \"$t/moved\" 2>/dev/null; then exit 49; fi\n\
         if sh -c ': > \"$1\"' d \"$t/new\" 2>/dev/null; then exit 50; fi\n\
         if cat \"$2\" 2>/dev/null; then exit 51; fi\n\
         printf read-only > result\n\
         : > \"$TEST_CHANNEL\"\n",
    )
    .unwrap();
    let argv = Argv::new(
        OsString::from("/bin/sh"),
        vec![
            script.into_os_string(),
            tree.clone().into_os_string(),
            root.join("sibling.txt").into_os_string(),
        ],
    );
    let channel = Channel::allocate(&work).unwrap();
    let ended = keyed_launch::run_confined_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "TEST_CHANNEL")),
            scrub: support::controls(),
            grant: &[],
            transparent: None,
            cwd: Some(&work),
            escalation: quick(),
        },
        &keyed_launch::FilesystemGrants {
            writable: std::slice::from_ref(&work),
            runtime_read: &[],
            read_trees: std::slice::from_ref(&tree),
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(ended.status.success(), "sandbox failed: {:?}", ended.status);
    assert_eq!(
        fs::read_to_string(work.join("result")).unwrap(),
        "read-only"
    );
    assert_eq!(
        fs::read_to_string(tree.join("shared.md")).unwrap(),
        "shared body"
    );
    assert!(tree.join("skills/alpha/SKILL.md").is_file());
    assert!(!tree.join("new").exists());
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = fs::metadata(tree.join("locked.md"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o444);
    }
}

/// A tree that a writable directory holds, or that holds one, refuses before
/// the launch, as do a missing tree and a file.
#[test]
fn a_read_only_tree_that_overlaps_a_writable_directory_or_is_no_directory_refuses() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir_all(work.join("inner")).unwrap();
    fs::write(root.join("file"), "f").unwrap();
    let marker = work.join("ran");
    let script = root.join("probe.sh");
    fs::write(&script, ": > ran\n").unwrap();
    for (tree, expected) in [
        (work.join("inner"), "overlaps writable directory"),
        (root.clone(), "overlaps writable directory"),
        (root.join("absent"), "cannot resolve sandbox resource"),
        (root.join("file"), "must be a directory"),
    ] {
        let argv = Argv::new(
            OsString::from("/bin/sh"),
            vec![script.clone().into_os_string()],
        );
        let channel = Channel::allocate(&work).unwrap();
        let error = keyed_launch::run_confined_observed(
            Launch {
                argv: &argv,
                channel: Some((&channel, "TEST_CHANNEL")),
                scrub: support::controls(),
                grant: &[],
                transparent: None,
                cwd: Some(&work),
                escalation: quick(),
            },
            &keyed_launch::FilesystemGrants {
                writable: std::slice::from_ref(&work),
                runtime_read: std::slice::from_ref(&script),
                read_trees: std::slice::from_ref(&tree),
            },
            &mut |_| {},
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains(expected), "{}: {error}", tree.display());
        assert!(!marker.exists());
    }
}

/// The read probe lists each tree inside the sandbox. A tree that the
/// confined lister cannot read fails the probe, and a readable one passes.
#[test]
fn the_read_probe_lists_each_tree_inside_the_sandbox() {
    use std::os::unix::fs::PermissionsExt as _;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let readable = root.join("readable");
    let closed = root.join("closed");
    fs::create_dir_all(readable.join("a")).unwrap();
    fs::create_dir_all(&closed).unwrap();
    keyed_launch::confinement_reads_available(std::slice::from_ref(&readable)).unwrap();
    // Traversal without listing: a path inside still opens, but the backend
    // cannot give the read of the directory.
    fs::set_permissions(&closed, fs::Permissions::from_mode(0o100)).unwrap();
    let error = keyed_launch::confinement_reads_available(&[readable.clone(), closed.clone()])
        .unwrap_err()
        .to_string();
    fs::set_permissions(&closed, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(error.contains("probe"), "{error}");
    let error = keyed_launch::confinement_reads_available(&[root.join("absent")])
        .unwrap_err()
        .to_string();
    assert!(error.contains("cannot resolve sandbox resource"), "{error}");
}
