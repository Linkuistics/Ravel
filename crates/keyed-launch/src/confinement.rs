//! Mandatory outer filesystem policy for standalone invocations.
use std::ffi::{CString, OsStr};
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Argv, LaunchError};

/// Open one regular file in a directory held before launching untrusted work.
/// Neither a replaced directory path nor a symlink can redirect this read.
pub fn regular_file_at(directory: &File, name: &OsStr) -> std::io::Result<File> {
    let mut components = Path::new(name).components();
    if !matches!(components.next(), Some(std::path::Component::Normal(_)))
        || components.next().is_some()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expected one artifact basename",
        ));
    }
    let name = CString::new(name.as_encoded_bytes())?;
    // SAFETY: directory is held open, name is NUL-terminated, and the returned
    // fresh descriptor is immediately owned. NONBLOCK prevents FIFO hangs.
    let fd = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "artifact is not a regular file",
        ));
    }
    Ok(file)
}

/// Multiple writable directories, literal read-only runtime files and
/// read-only directory trees. The caller decides which owner resources must
/// remain outside these grants.
pub struct FilesystemGrants<'a> {
    pub writable: &'a [PathBuf],
    pub runtime_read: &'a [PathBuf],
    /// Directories granted read-only with every entry beneath them. A tree
    /// neither lies inside a writable directory nor holds one, so no write
    /// grant reaches into it.
    pub read_trees: &'a [ReadTree],
}

/// A read-only directory tree, held by the canonical path and the identity
/// that its caller checked. The probe and the launch grant the tree only
/// while its path still resolves to that path and that directory. A path
/// that something replaced, also with a link to another directory, then
/// refuses instead of granting the new destination.
///
/// Both backends name the tree by its path. The interval between the last
/// check and the bind of the backend stays open, and Seatbelt matches the
/// path at each access after the launch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadTree {
    path: PathBuf,
    identity: (u64, u64),
}

impl ReadTree {
    /// Resolve `path` to its canonical directory, other than `/`, and record
    /// the device and inode of that directory.
    pub fn hold(path: &Path) -> Result<Self, LaunchError> {
        let canonical = canonical(path)?;
        let identity = directory_identity(&canonical)?;
        Ok(Self {
            path: canonical,
            identity,
        })
    }

    /// The canonical path of the tree.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Refuse unless the path still resolves to itself and to the directory
    /// that [`ReadTree::hold`] recorded.
    pub fn verify(&self) -> Result<(), LaunchError> {
        let resolved = canonical(&self.path)?;
        if resolved != self.path {
            return Err(LaunchError::new(format!(
                "read-only tree {} changed after its check: it now resolves to {}",
                self.path.display(),
                resolved.display()
            )));
        }
        if directory_identity(&resolved)? != self.identity {
            return Err(LaunchError::new(format!(
                "read-only tree {} changed after its check: another directory now has its path",
                self.path.display()
            )));
        }
        Ok(())
    }
}

fn directory_identity(path: &Path) -> Result<(u64, u64), LaunchError> {
    use std::os::unix::fs::MetadataExt as _;
    let metadata = std::fs::metadata(path).map_err(|error| {
        LaunchError::new(format!(
            "cannot resolve sandbox resource {}: {error}; supply an existing resource",
            path.display()
        ))
    })?;
    if !metadata.is_dir() || path.parent().is_none() {
        return Err(LaunchError::new(format!(
            "read-only tree {} must be a directory other than /",
            path.display()
        )));
    }
    Ok((metadata.dev(), metadata.ino()))
}

/// System filesystem resources exposed by the native backend. A caller that
/// protects owner resources must check these implicit grants too.
#[cfg(target_os = "macos")]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[
        "/private/etc/ssl/cert.pem",
        "/private/preboot/Cryptexes/OS",
        "/System",
        "/usr/bin",
        "/usr/sbin",
        "/usr/lib",
        "/usr/libexec",
        "/usr/share",
        "/bin",
        "/sbin",
        "/Library/Apple",
        "/opt/homebrew/Cellar",
        "/usr/local/lib",
        "/private/var/db/timezone",
        "/dev/random",
        "/dev/urandom",
        "/private/etc/passwd",
        "/private/etc/group",
        "/private/etc/hosts",
        "/private/etc/resolv.conf",
        "/private/etc/services",
        "/private/etc/protocols",
        "/private/etc/localtime",
        "/dev/null",
        "/dev/zero",
    ]
}

#[cfg(target_os = "linux")]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[
        "/usr/bin",
        "/usr/sbin",
        "/usr/lib",
        "/usr/lib64",
        "/usr/libexec",
        "/usr/share",
        "/bin",
        "/sbin",
        "/lib",
        "/lib64",
        "/etc/ld.so.cache",
        "/etc/passwd",
        "/etc/group",
        "/etc/nsswitch.conf",
        "/etc/resolv.conf",
        "/etc/hosts",
        "/etc/ssl/certs",
        "/proc",
        "/dev",
    ]
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[]
}

/// Establish an empty native sandbox around a system no-op before selection.
/// Presence alone does not prove that the backend can apply its policy.
pub fn confinement_available() -> Result<(), LaunchError> {
    probe(&[], &[OsStr::new("/usr/bin/true")])
}

/// List each tree inside a sandbox that grants only `trees` and the system
/// reads. Success shows that the backend gives the read of each tree, not
/// only that it accepted the policy. A tree that changed after its hold
/// refuses before the probe runs.
pub fn confinement_reads_available(trees: &[ReadTree]) -> Result<(), LaunchError> {
    let trees = read_trees(trees, &[])?;
    let lister = ["/bin/ls", "/usr/bin/ls"]
        .into_iter()
        .find(|path| Path::new(path).is_file())
        .ok_or_else(|| {
            LaunchError::new("cannot probe the read grants: no ls at /bin/ls or /usr/bin/ls")
        })?;
    let mut argv = vec![OsStr::new(lister), OsStr::new("-a"), OsStr::new("--")];
    argv.extend(trees.iter().map(|tree| tree.as_os_str()));
    probe(&trees, &argv)
}

fn probe(trees: &[PathBuf], argv: &[&OsStr]) -> Result<(), LaunchError> {
    let mut command = platform_command(&[], &[], trees)?;
    command
        .args(argv)
        .env_clear()
        .current_dir("/")
        .stdin(std::process::Stdio::null());
    // An inherited ignored SIGCHLD would auto-reap the probe. Repair it only
    // for this wait, then restore the caller's disposition, including on error.
    let ignored = crate::run::restore_child_watching()?;
    let output = command.output();
    if ignored {
        // SAFETY: restoring the disposition inspected before the probe.
        unsafe {
            libc::signal(libc::SIGCHLD, libc::SIG_IGN);
        }
    }
    let output =
        output.map_err(|error| LaunchError::new(format!("cannot probe confinement: {error}")))?;
    if !output.status.success() {
        return Err(LaunchError::new(format!(
            "confinement backend could not establish the probe sandbox ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

pub(crate) fn command_with_grants(
    argv: &Argv,
    policy: &FilesystemGrants<'_>,
) -> Result<Command, LaunchError> {
    let roots: Vec<PathBuf> = policy
        .writable
        .iter()
        .map(|path| {
            let path = canonical(path)?;
            if !path.is_dir() || path.parent().is_none() {
                return Err(LaunchError::new(
                    "confinement requires writable directories other than /",
                ));
            }
            Ok(path)
        })
        .collect::<Result<_, _>>()?;
    let program = executable(argv.program())?;
    let mut reads = vec![program.clone()];
    for path in policy.runtime_read {
        let path = canonical(path)?;
        if !path.is_file() {
            return Err(LaunchError::new(format!(
                "runtime grant {} must name a regular file; stage task inputs instead of granting directories",
                path.display()
            )));
        }
        reads.push(path);
    }
    let trees = read_trees(policy.read_trees, &roots)?;
    let mut command = platform_command(&roots, &reads, &trees)?;
    command.arg(program).args(argv.args());
    Ok(command)
}

/// The canonical directory of each held tree, unchanged since its hold,
/// which no writable root overlaps.
fn read_trees(trees: &[ReadTree], roots: &[PathBuf]) -> Result<Vec<PathBuf>, LaunchError> {
    trees
        .iter()
        .map(|held| {
            held.verify()?;
            let tree = held.path.clone();
            if let Some(root) = roots
                .iter()
                .find(|root| tree.starts_with(root) || root.starts_with(&tree))
            {
                return Err(LaunchError::new(format!(
                    "read-only tree {} overlaps writable directory {}",
                    tree.display(),
                    root.display()
                )));
            }
            Ok(tree)
        })
        .collect()
}

fn canonical(path: &Path) -> Result<PathBuf, LaunchError> {
    path.canonicalize().map_err(|error| {
        LaunchError::new(format!(
            "cannot resolve sandbox resource {}: {error}; supply an existing resource",
            path.display()
        ))
    })
}

/// The file a confined launch grants and runs: the caller's absolute path. A
/// name is refused, never looked up, so the file the caller chose is the file
/// that runs.
fn executable(program: &OsStr) -> Result<PathBuf, LaunchError> {
    let path = Path::new(program);
    if !path.is_absolute() {
        return Err(LaunchError::new(format!(
            "a confined launch takes an absolute program path, and {program:?} is not one; \
             resolve the program to a file before launching it"
        )));
    }
    canonical(path)
}

#[cfg(target_os = "macos")]
fn platform_command(
    roots: &[PathBuf],
    reads: &[PathBuf],
    trees: &[PathBuf],
) -> Result<Command, LaunchError> {
    // Seatbelt is inherited by descendants. Parameters keep path text out of
    // the policy language, including quotes and newlines in native filenames.
    // Reference: OpenAI's upstream seatbelt_base_policy.sbpl (Apache-2.0),
    // https://github.com/openai/codex/tree/main/codex-rs/sandboxing/src .
    // Hidden resources look absent, as in Linux's empty namespace. Harnesses
    // can probe optional configuration without reading it or failing startup.
    let mut profile = String::from(
        r#"(version 1)
(deny default (with errno ENOENT))
(allow process-exec process-fork)
(allow signal (target same-sandbox))
(allow process-info* (target same-sandbox))
(allow system-mac-syscall (mac-policy-name "vnguard"))
(allow system-mac-syscall (require-all (mac-policy-name "Sandbox") (mac-syscall-number 67)))
(allow sysctl-read (sysctl-name-prefix "hw.") (sysctl-name "kern.osrelease") (sysctl-name "kern.ostype") (sysctl-name "kern.osversion") (sysctl-name "kern.osproductversion") (sysctl-name "kern.argmax"))
(allow mach-lookup (global-name "com.apple.system.opendirectoryd.libinfo") (global-name "com.apple.trustd") (global-name "com.apple.trustd.agent") (global-name "com.apple.system.logger"))
(allow network-outbound (remote ip "*:*"))
(allow network-outbound (literal "/private/var/run/mDNSResponder"))
(allow mach-lookup (global-name "com.apple.SystemConfiguration.DNSConfiguration") (global-name "com.apple.SystemConfiguration.configd") (global-name "com.apple.networkd"))
(allow system-socket (require-all (socket-domain AF_SYSTEM) (socket-protocol 2)))
(allow sysctl-read (sysctl-name "kern.hostname") (sysctl-name "kern.version") (sysctl-name-prefix "net.routetable."))
(allow file-read-metadata)
(allow file-read* (literal "/"))
(allow file-read* file-write-data (literal "/dev/null") (literal "/dev/zero"))
"#,
    );
    let backend = "/usr/bin/sandbox-exec";
    if !Path::new(backend).is_file() {
        return Err(LaunchError::new(
            "filesystem confinement requires /usr/bin/sandbox-exec; restore the system Seatbelt launcher",
        ));
    }
    let mut command = Command::new(backend);
    for path in confinement_system_reads() {
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (subpath \"{path}\"))\n"
        ));
    }
    for (index, root) in roots.iter().enumerate() {
        let name = format!("WRITABLE_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-write* file-map-executable (subpath (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, root));
    }
    // Explicit denies preserve read-only files beneath broader writable roots.
    // Ancestor unlink denies prevent moving a directory to bypass the literal.
    // https://github.com/openai/codex/blob/main/codex-rs/sandboxing/src/seatbelt.rs
    for (index, path) in reads.iter().enumerate() {
        let name = format!("RUNTIME_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (literal (param \"{name}\")))\n"
        ));
        profile.push_str(&format!(
            "(deny file-write* (literal (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, path));
        for (ancestor_index, ancestor) in path.ancestors().skip(1).enumerate() {
            let anchor = format!("{name}_ANCESTOR_{ancestor_index}");
            profile.push_str(&format!(
                "(deny file-write-unlink (literal (param \"{anchor}\")))\n"
            ));
            command.arg("-D").arg(parameter(&anchor, ancestor));
        }
    }
    // A subpath covers the tree and each entry beneath it. Seatbelt checks
    // the resolved path, so a link inside the tree reads its target there.
    // The default deny keeps the tree read-only, because no writable root
    // overlaps it.
    for (index, tree) in trees.iter().enumerate() {
        let name = format!("TREE_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (subpath (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, tree));
    }
    command.args(["-p", &profile, "--"]);
    Ok(command)
}

#[cfg(target_os = "macos")]
fn parameter(name: &str, path: &Path) -> std::ffi::OsString {
    let mut value = std::ffi::OsString::from(format!("{name}="));
    value.push(path);
    value
}

#[cfg(target_os = "linux")]
fn platform_command(
    roots: &[PathBuf],
    reads: &[PathBuf],
    trees: &[PathBuf],
) -> Result<Command, LaunchError> {
    // Start with an empty mount namespace, never a read-only bind of `/`.
    // https://github.com/containers/bubblewrap/blob/main/README.md
    let backend = ["/usr/bin/bwrap", "/bin/bwrap"].into_iter().find(|path| Path::new(path).is_file())
        .ok_or_else(|| LaunchError::new("filesystem confinement requires bubblewrap at /usr/bin/bwrap; install the system bubblewrap package"))?;
    let mut command = Command::new(backend);
    // The confined runner already creates the session. Keeping the payload in the
    // monitor's group lets its cleanup wait observe every ordinary descendant.
    command.args([
        "--die-with-parent",
        "--unshare-user",
        "--unshare-pid",
        "--unshare-ipc",
        "--unshare-uts",
    ]);
    for &path in confinement_system_reads() {
        if !matches!(path, "/proc" | "/dev") && Path::new(path).exists() {
            command.args(["--ro-bind", path, path]);
        }
    }
    command.args(["--proc", "/proc", "--dev", "/dev"]);
    for root in roots {
        command.arg("--bind").arg(root).arg(root);
    }
    for path in reads {
        command.arg("--ro-bind").arg(path).arg(path);
    }
    // A read-only bind of a tree covers each entry beneath it, and a
    // relative link inside the tree resolves inside the same mount.
    for tree in trees {
        command.arg("--ro-bind").arg(tree).arg(tree);
    }
    // Auto-created parent directories belong to the namespace's tmpfs root.
    // Freeze that root too, leaving only the invocation bind writable.
    command.args([
        "--remount-ro",
        "/",
        "--remount-ro",
        "/dev",
        "--remount-ro",
        "/proc",
    ]);
    command.arg("--");
    Ok(command)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn platform_command(_: &[PathBuf], _: &[PathBuf], _: &[PathBuf]) -> Result<Command, LaunchError> {
    Err(LaunchError::new(
        "standalone confinement is unavailable on this platform; the task was not launched",
    ))
}
