//! Held directory descriptors. Capture reads its source and the cache writes
//! its objects through open directories, so that a replaced path segment
//! cannot redirect a read or a write
//! (docs/specs/skill-discovery.md#publication-concurrency-and-cleanup).

use std::ffi::{CString, OsStr, OsString};
use std::fs::File;
use std::io::{self, ErrorKind};
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};
use std::os::unix::ffi::{OsStrExt as _, OsStringExt as _};
use std::path::{Path, PathBuf};

/// An open directory and the path that diagnostics name for it.
#[derive(Debug)]
pub struct Dir {
    fd: OwnedFd,
    path: PathBuf,
}

/// The file type of an entry, from its own status and never from a link
/// target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Directory,
    File,
    Link,
    Other,
}

/// The status of one entry: its identity, type, owner, mode, size and change
/// times. Two looks at an unchanged entry give equal values, except for the
/// access time, which this type leaves out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stat {
    pub dev: u64,
    pub ino: u64,
    pub mode: u32,
    pub uid: u32,
    pub size: i64,
    pub mtime: (i64, i64),
    pub ctime: (i64, i64),
}

impl Stat {
    // The field types differ between macOS and GNU/Linux, so a cast that is
    // needed on one platform is a no-op on the other.
    #[allow(clippy::unnecessary_cast, clippy::useless_conversion)]
    fn from_raw(raw: &libc::stat) -> Stat {
        Stat {
            dev: raw.st_dev as u64,
            ino: raw.st_ino as u64,
            mode: u32::from(raw.st_mode),
            uid: raw.st_uid,
            size: raw.st_size as i64,
            mtime: (raw.st_mtime as i64, raw.st_mtime_nsec as i64),
            ctime: (raw.st_ctime as i64, raw.st_ctime_nsec as i64),
        }
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn kind(&self) -> Type {
        match self.mode & (libc::S_IFMT as u32) {
            m if m == libc::S_IFDIR as u32 => Type::Directory,
            m if m == libc::S_IFREG as u32 => Type::File,
            m if m == libc::S_IFLNK as u32 => Type::Link,
            _ => Type::Other,
        }
    }

    pub fn identity(&self) -> (u64, u64) {
        (self.dev, self.ino)
    }

    pub fn permissions(&self) -> u32 {
        self.mode & 0o7777
    }
}

fn c_name(name: &OsStr) -> io::Result<CString> {
    CString::new(name.as_bytes())
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "a name holds NUL"))
}

fn checked(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

const DIRECTORY_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC;

impl Dir {
    /// Open the directory at `path`, following links in the path.
    pub fn open(path: &Path) -> io::Result<Dir> {
        let name = c_name(path.as_os_str())?;
        // SAFETY: `name` is a valid NUL-terminated path for this call.
        let fd = checked(unsafe { libc::open(name.as_ptr(), DIRECTORY_FLAGS) })?;
        Ok(Dir {
            // SAFETY: `open` returned a new descriptor that nothing else owns.
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
            path: path.to_owned(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn try_clone(&self) -> io::Result<Dir> {
        Ok(Dir {
            fd: self.fd.try_clone()?,
            path: self.path.clone(),
        })
    }

    /// Open the directory `name` in this one. A link refuses as `ELOOP` or
    /// `ENOTDIR`.
    pub fn open_dir(&self, name: &OsStr) -> io::Result<Dir> {
        let c = c_name(name)?;
        // SAFETY: the descriptor is open and `c` is a valid NUL-terminated name.
        let fd = checked(unsafe {
            libc::openat(
                self.fd.as_raw_fd(),
                c.as_ptr(),
                DIRECTORY_FLAGS | libc::O_NOFOLLOW,
            )
        })?;
        Ok(Dir {
            // SAFETY: `openat` returned a new descriptor that nothing else owns.
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
            path: self.path.join(name),
        })
    }

    /// Open the regular file `name` for reading, without following a link.
    /// `O_NONBLOCK` keeps a FIFO that replaced the file from holding the
    /// resolver; it does not change reads of a regular file.
    pub fn open_file(&self, name: &OsStr) -> io::Result<File> {
        self.open_at(
            name,
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
            0,
        )
    }

    /// Create the new file `name`, which must not exist, with `mode`.
    pub fn create(&self, name: &OsStr, mode: u32) -> io::Result<File> {
        self.open_at(
            name,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            mode,
        )
    }

    #[allow(clippy::unnecessary_cast)]
    fn open_at(&self, name: &OsStr, flags: libc::c_int, mode: u32) -> io::Result<File> {
        let c = c_name(name)?;
        // SAFETY: the descriptor is open and `c` is a valid NUL-terminated name.
        let fd = checked(unsafe {
            libc::openat(self.fd.as_raw_fd(), c.as_ptr(), flags, mode as libc::c_uint)
        })?;
        // SAFETY: `openat` returned a new descriptor that nothing else owns.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    /// Flush the whole storage device of this directory: on macOS its write
    /// cache too, so that every earlier `fsync` reaches permanent storage.
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsync.2.html
    #[cfg(target_os = "macos")]
    pub fn flush_device(&self) -> io::Result<()> {
        // SAFETY: the descriptor is open; F_FULLFSYNC takes no argument.
        checked(unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_FULLFSYNC) }).map(drop)
    }

    /// On Linux, `fsync` of each file already reached the device, so this
    /// flushes the directory alone.
    // https://man7.org/linux/man-pages/man2/fsync.2.html
    #[cfg(target_os = "linux")]
    pub fn flush_device(&self) -> io::Result<()> {
        // SAFETY: the descriptor is open.
        checked(unsafe { libc::fsync(self.fd.as_raw_fd()) }).map(drop)
    }

    /// The status of this directory.
    pub fn stat(&self) -> io::Result<Stat> {
        stat_fd(self.fd.as_raw_fd())
    }

    /// The status of the entry `name`, without following a link.
    pub fn stat_at(&self, name: &OsStr) -> io::Result<Stat> {
        let c = c_name(name)?;
        let mut raw = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: the descriptor is open, `c` is a valid name and `raw` has room
        // for one `stat`.
        checked(unsafe {
            libc::fstatat(
                self.fd.as_raw_fd(),
                c.as_ptr(),
                raw.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        })?;
        // SAFETY: `fstatat` succeeded, so it filled `raw`.
        let raw = unsafe { raw.assume_init() };
        Ok(Stat::from_raw(&raw))
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn mkdir(&self, name: &OsStr, mode: u32) -> io::Result<()> {
        let c = c_name(name)?;
        // SAFETY: the descriptor is open and `c` is a valid NUL-terminated name.
        checked(unsafe { libc::mkdirat(self.fd.as_raw_fd(), c.as_ptr(), mode as libc::mode_t) })
            .map(drop)
    }

    pub fn symlink(&self, target: &OsStr, name: &OsStr) -> io::Result<()> {
        let (target, c) = (c_name(target)?, c_name(name)?);
        // SAFETY: the descriptor is open and both arguments are valid
        // NUL-terminated strings.
        checked(unsafe { libc::symlinkat(target.as_ptr(), self.fd.as_raw_fd(), c.as_ptr()) })
            .map(drop)
    }

    /// The target of the link `name`.
    pub fn read_link(&self, name: &OsStr) -> io::Result<OsString> {
        let c = c_name(name)?;
        let mut buffer = vec![0u8; 256];
        loop {
            // SAFETY: the descriptor is open, `c` is a valid name and the
            // buffer has the stated length.
            let read = unsafe {
                libc::readlinkat(
                    self.fd.as_raw_fd(),
                    c.as_ptr(),
                    buffer.as_mut_ptr().cast(),
                    buffer.len(),
                )
            };
            let read = usize::try_from(read).map_err(|_| io::Error::last_os_error())?;
            if read < buffer.len() {
                buffer.truncate(read);
                return Ok(OsString::from_vec(buffer));
            }
            buffer.resize(buffer.len() * 2, 0);
        }
    }

    /// The names in this directory, without `.` and `..`, sorted by bytes.
    pub fn list(&self) -> io::Result<Vec<OsString>> {
        let fd = std::os::fd::IntoRawFd::into_raw_fd(self.fd.try_clone()?);
        // SAFETY: `fdopendir` takes ownership of the duplicated descriptor,
        // and `closedir` below releases it.
        let stream = unsafe { libc::fdopendir(fd) };
        if stream.is_null() {
            let error = io::Error::last_os_error();
            // SAFETY: `fdopendir` failed, so the duplicate is still ours.
            unsafe { libc::close(fd) };
            return Err(error);
        }
        // The duplicate shares its offset with the held descriptor, so an
        // earlier listing can have left it at the end.
        // SAFETY: `stream` is an open directory stream.
        unsafe { libc::rewinddir(stream) };
        let mut names = Vec::new();
        let outcome = loop {
            set_errno(0);
            // SAFETY: `stream` is an open directory stream.
            let entry = unsafe { libc::readdir(stream) };
            if entry.is_null() {
                let error = io::Error::last_os_error();
                break match error.raw_os_error() {
                    Some(0) | None => Ok(()),
                    Some(_) => Err(error),
                };
            }
            // SAFETY: `readdir` returned an entry whose name is NUL-terminated.
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
            let bytes = name.to_bytes();
            if bytes != b"." && bytes != b".." {
                names.push(OsString::from_vec(bytes.to_vec()));
            }
        };
        // SAFETY: `stream` is open, and nothing uses it after this call.
        unsafe { libc::closedir(stream) };
        outcome?;
        names.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        Ok(names)
    }

    /// Change the mode of this directory.
    pub fn chmod(&self, mode: u32) -> io::Result<()> {
        chmod_fd(self.fd.as_raw_fd(), mode)
    }

    /// Remove the entry `name`: a file or link, or an empty directory.
    pub fn unlink(&self, name: &OsStr, directory: bool) -> io::Result<()> {
        let c = c_name(name)?;
        let flags = if directory { libc::AT_REMOVEDIR } else { 0 };
        // SAFETY: the descriptor is open and `c` is a valid NUL-terminated name.
        checked(unsafe { libc::unlinkat(self.fd.as_raw_fd(), c.as_ptr(), flags) }).map(drop)
    }

    /// Remove the entry `name` and everything beneath it, restoring write
    /// permission on each directory first. A link is removed, never followed.
    pub fn remove_tree(&self, name: &OsStr) -> io::Result<()> {
        #[cfg(test)]
        super::faults::removing(&self.path.join(name))?;
        if self.stat_at(name)?.kind() != Type::Directory {
            return self.unlink(name, false);
        }
        let inner = self.open_dir(name)?;
        inner.chmod(0o700)?;
        for child in inner.list()? {
            inner.remove_tree(&child)?;
        }
        self.unlink(name, true)
    }

    /// Remove everything beneath this directory, restoring write permission
    /// on it first. The directory itself stays.
    pub fn clear(&self) -> io::Result<()> {
        self.chmod(0o700)?;
        for child in self.list()? {
            self.remove_tree(&child)?;
        }
        Ok(())
    }

    /// Rename `name` in this directory to `to_name` in `to`, failing with
    /// `EEXIST` or `ENOTEMPTY` when the destination exists.
    pub fn rename_noreplace(&self, name: &OsStr, to: &Dir, to_name: &OsStr) -> io::Result<()> {
        #[cfg(test)]
        super::faults::renamed(&self.path.join(name))?;
        let (from, dest) = (c_name(name)?, c_name(to_name)?);
        rename_noreplace(self.fd.as_raw_fd(), &from, to.fd.as_raw_fd(), &dest)
    }
}

/// Change the mode of an open file.
/// Send the bytes of `file` to its device with a plain `fsync`. On macOS
/// this does not flush the write cache of the device, which `sync_all` does
/// for each file at about 5 ms. [`Dir::flush_device`] flushes it once
/// before a publication.
pub fn flush_file(file: &File) -> io::Result<()> {
    // SAFETY: the descriptor is open.
    checked(unsafe { libc::fsync(file.as_raw_fd()) }).map(drop)
}

pub fn chmod_file(file: &File, mode: u32) -> io::Result<()> {
    chmod_fd(file.as_raw_fd(), mode)
}

/// The status of an open file.
pub fn stat_file(file: &File) -> io::Result<Stat> {
    stat_fd(file.as_raw_fd())
}

fn stat_fd(fd: libc::c_int) -> io::Result<Stat> {
    let mut raw = std::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: `fd` is open and `raw` has room for one `stat`.
    checked(unsafe { libc::fstat(fd, raw.as_mut_ptr()) })?;
    // SAFETY: `fstat` succeeded, so it filled `raw`.
    let raw = unsafe { raw.assume_init() };
    Ok(Stat::from_raw(&raw))
}

#[allow(clippy::unnecessary_cast)]
fn chmod_fd(fd: libc::c_int, mode: u32) -> io::Result<()> {
    // SAFETY: `fd` is open.
    checked(unsafe { libc::fchmod(fd, mode as libc::mode_t) }).map(drop)
}

#[cfg(target_os = "macos")]
fn set_errno(value: libc::c_int) {
    // SAFETY: `__error` returns the address of this thread's errno.
    unsafe { *libc::__error() = value };
}

#[cfg(target_os = "linux")]
fn set_errno(value: libc::c_int) {
    // SAFETY: `__errno_location` returns the address of this thread's errno.
    unsafe { *libc::__errno_location() = value };
}

// https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/renamex_np.2.html
#[cfg(target_os = "macos")]
fn rename_noreplace(
    from_fd: libc::c_int,
    from: &CString,
    to_fd: libc::c_int,
    to: &CString,
) -> io::Result<()> {
    // SAFETY: both descriptors are open and both names are valid
    // NUL-terminated strings.
    checked(unsafe {
        libc::renameatx_np(
            from_fd,
            from.as_ptr(),
            to_fd,
            to.as_ptr(),
            libc::RENAME_EXCL,
        )
    })
    .map(drop)
}

// https://man7.org/linux/man-pages/man2/renameat2.2.html
// The release links against glibc 2.17, and glibc added its `renameat2`
// wrapper in 2.28, so Ravel enters the system call itself. Linux added the
// call in 3.15.
#[cfg(target_os = "linux")]
fn rename_noreplace(
    from_fd: libc::c_int,
    from: &CString,
    to_fd: libc::c_int,
    to: &CString,
) -> io::Result<()> {
    // SAFETY: both descriptors are open and both names are valid
    // NUL-terminated strings. Each argument is widened to the register width
    // that the kernel reads.
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            libc::c_long::from(from_fd),
            from.as_ptr(),
            libc::c_long::from(to_fd),
            to.as_ptr(),
            libc::c_long::from(libc::RENAME_NOREPLACE),
        )
    };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Whether `error` from [`Dir::rename_noreplace`] says that the kernel or the
/// file system has no rename without replacement. A plain rename can replace
/// an empty directory, so Ravel has no substitute for it.
#[cfg(target_os = "linux")]
pub fn noreplace_unsupported(error: &io::Error) -> bool {
    // ENOSYS: a kernel before 3.15. EINVAL: a file system without the flag.
    matches!(error.raw_os_error(), Some(libc::ENOSYS | libc::EINVAL))
}

/// Whether `error` from [`Dir::rename_noreplace`] says that the file system
/// has no rename without replacement.
#[cfg(target_os = "macos")]
pub fn noreplace_unsupported(error: &io::Error) -> bool {
    matches!(error.raw_os_error(), Some(libc::ENOTSUP))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs;

    use super::Dir;

    /// A plain `rename` replaces an empty destination directory. This rename
    /// must not, so the empty case tells the two apart.
    #[test]
    fn rename_noreplace_moves_to_a_free_name_and_never_replaces_a_directory() {
        let root = tempfile::tempdir().unwrap();
        for name in ["from", "empty", "full"] {
            fs::create_dir(root.path().join(name)).unwrap();
        }
        fs::write(root.path().join("from/kept"), "kept").unwrap();
        fs::write(root.path().join("full/held"), "held").unwrap();
        let dir = Dir::open(root.path()).unwrap();
        let from = dir.stat_at(OsStr::new("from")).unwrap().identity();

        for taken in ["empty", "full"] {
            let error = dir
                .rename_noreplace(OsStr::new("from"), &dir, OsStr::new(taken))
                .unwrap_err();
            assert!(
                matches!(error.raw_os_error(), Some(libc::EEXIST | libc::ENOTEMPTY)),
                "{taken}: {error}"
            );
            assert_eq!(dir.stat_at(OsStr::new("from")).unwrap().identity(), from);
            assert_ne!(dir.stat_at(OsStr::new(taken)).unwrap().identity(), from);
        }
        assert_eq!(
            fs::read_to_string(root.path().join("full/held")).unwrap(),
            "held"
        );

        dir.rename_noreplace(OsStr::new("from"), &dir, OsStr::new("free"))
            .unwrap();
        assert_eq!(dir.stat_at(OsStr::new("free")).unwrap().identity(), from);
        assert_eq!(
            fs::read_to_string(root.path().join("free/kept")).unwrap(),
            "kept"
        );
        assert!(dir.stat_at(OsStr::new("from")).is_err());
    }
}
