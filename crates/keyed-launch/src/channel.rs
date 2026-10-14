//! The out-of-band completion channel: a fresh, collision-resistant path per
//! launch, naming that launch alone.

use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::LaunchError;

/// The fixed leading text of every channel file name.
///
/// A prefix rather than a bare nonce so a human reading the directory can tell
/// what the file is, and so [`Channel::discard_abandoned`] has something to
/// recognise that no neighbouring file will accidentally match.
///
/// Named for [`signal`], the verb that writes one. It is also the spelling
/// a caller already leaves in its control directory, so a launcher moving
/// onto this crate still recognises the channels its predecessor abandoned
/// rather than orphaning them permanently.
const CHANNEL_PREFIX: &str = "signal-";

/// Bytes of OS randomness in a channel name — 128 bits, rendered as 32
/// lowercase hex characters.
const NONCE_BYTES: usize = 16;

/// How many occupied random draws are tolerated before allocation refuses.
///
/// A bound rather than an unbounded retry: at 128 bits a single collision means
/// the randomness source is not random, and spinning on that forever turns a
/// diagnosable fault into a hang.
const DRAW_RETRY_LIMIT: usize = 8;

/// The out-of-band completion signal for one launch.
///
/// **Allocation picks a name; it writes nothing.** The channel file comes into
/// existence only when something calls [`signal`] on its path — which is what
/// makes *appearance* the event [`run`](crate::run) watches for. A channel that
/// is never signalled is a path that never existed.
#[derive(Debug)]
pub struct Channel {
    path: PathBuf,
    directory: File,
}

impl Channel {
    /// Draw a fresh channel path inside `dir`, retrying an occupied name
    /// without touching whatever occupies it.
    ///
    /// `dir` must already exist. That is checked here rather than left to the
    /// child's first write, because the two failures land in very different
    /// places: checked, the caller is told which directory is missing before
    /// anything is spawned; unchecked, a child runs to completion and its
    /// signal silently fails to land, which reads as a launch that hung.
    pub fn allocate(dir: &Path) -> Result<Self, LaunchError> {
        let metadata = fs::metadata(dir).map_err(|error| {
            LaunchError::new(format!(
                "cannot allocate a completion channel in {}: {error}; create the directory, or \
                 pass one that exists",
                dir.display()
            ))
            .with_io_source(error)
        })?;
        if !metadata.is_dir() {
            return Err(LaunchError::new(format!(
                "cannot allocate a completion channel in {}: it is not a directory; pass the \
                 directory the channel files should live in",
                dir.display()
            )));
        }

        for _ in 0..DRAW_RETRY_LIMIT {
            let path = dir.join(format!("{CHANNEL_PREFIX}{}", hex(draw_nonce()?)));
            match fs::symlink_metadata(&path) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    let directory = File::open(dir).map_err(|error| {
                        LaunchError::new(format!(
                            "cannot hold completion directory {}: {error}",
                            dir.display()
                        ))
                        .with_io_source(error)
                    })?;
                    return Ok(Self { path, directory });
                }
                Err(error) => {
                    return Err(LaunchError::new(format!(
                        "cannot establish whether the drawn completion-channel path {} is free: \
                         {error}; make the directory readable and retry",
                        path.display()
                    ))
                    .with_io_source(error))
                }
                Ok(_) => continue,
            }
        }
        Err(LaunchError::new(format!(
            "could not allocate a fresh completion channel after {DRAW_RETRY_LIMIT} occupied \
             random draws in {}; at 128 bits this means the OS randomness source is not random — \
             check /dev/urandom",
            dir.display()
        )))
    }

    /// The path a child creates its completion signal at. This is the value a launch
    /// publishes to the child under the caller's chosen variable name.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Whether anything now exists at the channel's name, in the directory it
    /// was allocated in.
    ///
    /// **Appearance is the whole signal**: an empty file, a link or a FIFO
    /// counts equally. Nothing is opened or followed: the name is only looked
    /// up, beside the directory held since allocation.
    #[must_use]
    pub fn appeared(&self) -> bool {
        use std::os::fd::AsRawFd as _;
        use std::os::unix::ffi::OsStrExt as _;
        let Some(name) = self
            .path
            .file_name()
            .and_then(|name| std::ffi::CString::new(name.as_bytes()).ok())
        else {
            return false;
        };
        // SAFETY: `fstatat(2)` on a directory descriptor this channel holds,
        // a NUL-terminated name, and an initialised struct it writes into.
        unsafe {
            let mut status: libc::stat = std::mem::zeroed();
            libc::fstatat(
                self.directory.as_raw_fd(),
                name.as_ptr(),
                &mut status,
                libc::AT_SYMLINK_NOFOLLOW,
            ) == 0
        }
    }

    /// Remove this launch's channel file, consuming the channel so nothing can
    /// observe a path whose file is gone.
    ///
    /// A channel that was never signalled has no file, and discarding it is
    /// still success: the post-condition is *this path holds nothing*, not
    /// *this call removed something*.
    pub fn discard(self) -> Result<(), LaunchError> {
        remove_if_present(&self.path)
    }

    /// Remove every channel file in `dir` — the ones a previous launcher
    /// allocated and did not live to discard.
    ///
    /// **The name grammar is this crate's, so recognising an abandoned channel
    /// has to be too.** The alternative is a consumer open-coding
    /// `signal-<32 hex>` in its own cleanup, which is a second spelling of a
    /// rule that only one place should hold.
    ///
    /// Allocation does not depend on this: it draws fresh names and retries
    /// occupied ones. This is hygiene, and a caller that cannot afford to fail
    /// on it may report the error and carry on.
    pub fn discard_abandoned(dir: &Path) -> Result<(), LaunchError> {
        let entries = fs::read_dir(dir).map_err(|error| {
            LaunchError::new(format!(
                "cannot list abandoned completion channels in {}: {error}; make the directory \
                 readable and retry",
                dir.display()
            ))
        })?;

        // Every entry is attempted before anything is reported. Stopping at the
        // first failure would leave channels behind that this pass could have
        // removed, and the one unremovable file would hide the rest.
        let mut failures = Vec::new();
        for entry in entries {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(error) => {
                    failures.push(format!("reading a directory entry: {error}"));
                    continue;
                }
            };
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !is_channel_name(name) {
                continue;
            }
            if let Err(error) = remove_if_present(&path) {
                failures.push(format!("{error}"));
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(LaunchError::new(format!(
                "could not remove {} abandoned completion channel(s) in {}; remove them by hand:\n- {}",
                failures.len(),
                dir.display(),
                failures.join("\n- ")
            )))
        }
    }
}

/// Create an empty file at `path` to signal that the launched child is done.
///
/// A free function rather than a [`Channel`] method because the two ends are
/// different processes: the launcher holds the `Channel`, and the child holds
/// only the path it was handed under the launch's chosen variable name.
/// Appearance carries the signal; the runner never reads channel content.
pub fn signal(path: &Path) -> Result<(), LaunchError> {
    fs::write(path, []).map_err(|error| {
        LaunchError::new(format!(
            "cannot create the completion signal at {}: {error}; the launcher allocated this path \
             and its directory should exist — check that the channel was not removed early",
            path.display()
        ))
    })
}

/// Exactly [`CHANNEL_PREFIX`] followed by 32 lowercase hex characters.
///
/// Deliberately exact. A looser rule would let this crate's cleanup delete a
/// neighbouring file that merely starts the same way, in a directory whose
/// other contents belong to the consumer.
fn is_channel_name(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix(CHANNEL_PREFIX) else {
        return false;
    };
    suffix.len() == NONCE_BYTES * 2
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn remove_if_present(path: &Path) -> Result<(), LaunchError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(LaunchError::new(format!(
            "cannot remove the completion channel {}: {error}; remove it by hand",
            path.display()
        ))),
    }
}

fn draw_nonce() -> Result<[u8; NONCE_BYTES], LaunchError> {
    let mut source = File::open("/dev/urandom").map_err(|error| {
        LaunchError::new(format!("cannot open the OS randomness source: {error}"))
            .with_io_source(error)
    })?;
    let mut nonce = [0_u8; NONCE_BYTES];
    source.read_exact(&mut nonce).map_err(|error| {
        LaunchError::new(format!(
            "cannot read {} bytes of OS randomness for a completion-channel name: {error}",
            NONCE_BYTES
        ))
        .with_io_source(error)
    })?;
    Ok(nonce)
}

fn hex(nonce: [u8; NONCE_BYTES]) -> String {
    let mut rendered = String::with_capacity(NONCE_BYTES * 2);
    for byte in nonce {
        // Infallible: `String`'s `Write` never errors, and the format is fixed.
        let _ = write!(&mut rendered, "{byte:02x}");
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_allocated_channel_names_a_path_that_does_not_yet_exist() {
        let dir = tempfile::tempdir().unwrap();

        let channel = Channel::allocate(dir.path()).unwrap();

        assert!(
            !channel.path().exists(),
            "allocation must pick a name, not create a file — appearance is the event"
        );
        assert_eq!(channel.path().parent(), Some(dir.path()));
        assert!(is_channel_name(
            channel.path().file_name().unwrap().to_str().unwrap()
        ));
    }

    #[test]
    fn successive_allocations_in_one_directory_never_collide() {
        let dir = tempfile::tempdir().unwrap();

        let first = Channel::allocate(dir.path()).unwrap();
        let second = Channel::allocate(dir.path()).unwrap();

        assert_ne!(first.path(), second.path());
    }

    #[test]
    fn allocation_names_a_missing_directory_and_says_what_to_do() {
        let dir = tempfile::tempdir().unwrap();
        let absent = dir.path().join("never-created");

        let error = Channel::allocate(&absent).unwrap_err();

        let message = error.to_string();
        assert!(message.contains(&absent.display().to_string()), "{message}");
        assert!(message.contains("create the directory"), "{message}");
    }

    #[test]
    fn signalling_creates_an_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();

        signal(channel.path()).unwrap();

        assert!(channel.appeared());
        assert_eq!(fs::read(channel.path()).unwrap(), b"");
    }

    /// Appearance, without opening or interpreting the child-controlled entry,
    /// is the whole completion signal.
    #[test]
    fn anything_at_the_channel_name_has_appeared_and_nothing_else_has() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();
        assert!(!channel.appeared(), "control: nothing is there yet");
        fs::write(dir.path().join("neighbour"), "").unwrap();
        assert!(
            !channel.appeared(),
            "a neighbouring file is not the channel"
        );

        fs::write(channel.path(), "").unwrap();
        assert!(channel.appeared(), "an empty file has appeared");
        fs::remove_file(channel.path()).unwrap();
        std::os::unix::fs::symlink(dir.path().join("absent"), channel.path()).unwrap();
        assert!(channel.appeared(), "a dangling link has appeared");
    }

    #[test]
    fn appearance_does_not_follow_links_or_read_large_files_or_fifos() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();
        let outside = dir.path().join("outside");
        fs::write(&outside, "done").unwrap();
        std::os::unix::fs::symlink(&outside, channel.path()).unwrap();
        assert!(channel.appeared(), "a link appears without being followed");
        assert_eq!(fs::read_to_string(&outside).unwrap(), "done");
        fs::remove_file(channel.path()).unwrap();
        fs::write(channel.path(), vec![b'x'; 4097]).unwrap();
        assert!(channel.appeared(), "content size cannot affect appearance");
        fs::remove_file(channel.path()).unwrap();
        let path = std::ffi::CString::new(channel.path().as_os_str().as_encoded_bytes()).unwrap();
        // SAFETY: valid NUL-terminated path in a private test directory.
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert!(channel.appeared(), "appearance must not block on a FIFO");
    }

    #[test]
    fn completion_stays_in_its_original_directory() {
        let dir = tempfile::tempdir().unwrap();
        let control = dir.path().join("control");
        fs::create_dir(&control).unwrap();
        let channel = Channel::allocate(&control).unwrap();
        let outside = dir.path().join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join(channel.path().file_name().unwrap()), "done").unwrap();
        fs::rename(&control, dir.path().join("original")).unwrap();
        std::os::unix::fs::symlink(outside, &control).unwrap();
        assert!(
            !channel.appeared(),
            "the substituted directory is not the held one"
        );
    }

    #[test]
    fn discarding_removes_the_file_and_succeeds_when_there_was_none() {
        let dir = tempfile::tempdir().unwrap();
        let signalled = Channel::allocate(dir.path()).unwrap();
        signal(signalled.path()).unwrap();
        let signalled_path = signalled.path().to_path_buf();
        let untouched = Channel::allocate(dir.path()).unwrap();

        signalled.discard().unwrap();
        untouched.discard().unwrap();

        assert!(!signalled_path.exists());
    }

    #[test]
    fn abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone() {
        let dir = tempfile::tempdir().unwrap();
        let channel = dir.path().join("signal-0123456789abcdef0123456789abcdef");
        // Each of these fails the grammar in a different way, and each is a
        // file a consumer could legitimately keep in the same directory.
        // A distinct nonce, not the same one in another case: on a
        // case-insensitive filesystem the two names would be one file, and the
        // test would be asserting nothing.
        let uppercase = dir.path().join("signal-FEDCBA9876543210FEDCBA9876543210");
        let short = dir.path().join("signal-0123456789abcdef");
        let unprefixed = dir.path().join("0123456789abcdef0123456789abcdef");
        let neighbour = dir.path().join("driver.lease");
        for path in [&channel, &uppercase, &short, &unprefixed, &neighbour] {
            std::fs::write(path, "x").unwrap();
        }

        Channel::discard_abandoned(dir.path()).unwrap();

        assert!(!channel.exists(), "an exact channel name must be removed");
        for path in [&uppercase, &short, &unprefixed, &neighbour] {
            assert!(path.exists(), "{} must survive cleanup", path.display());
        }
    }

    #[test]
    fn abandoned_cleanup_names_the_directory_when_it_cannot_be_listed() {
        let dir = tempfile::tempdir().unwrap();
        let absent = dir.path().join("never-created");

        let error = Channel::discard_abandoned(&absent).unwrap_err();

        assert!(
            error.to_string().contains(&absent.display().to_string()),
            "{error}"
        );
    }
}
