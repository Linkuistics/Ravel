use std::fmt;

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque**: the message is the interface, and the type implements
/// `std::error::Error` without imposing an error-handling dependency on its
/// consumers.
///
/// Its obligation is to name what is wrong, name where, and name what would
/// fix it.
pub struct LaunchError {
    message: String,
    errno: Option<i32>,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            errno: None,
        }
    }

    pub(crate) fn with_errno(mut self, errno: Option<i32>) -> Self {
        self.errno = errno;
        self
    }

    /// The system's error number when the child could not be spawned, and
    /// `None` for every other failure.
    ///
    /// A caller that reports a failed start by its cause, as a shell's 127 for
    /// a missing program and 126 for any other, needs the number and not the
    /// message. Nothing was started when this is `Some`.
    #[must_use]
    pub fn raw_os_error(&self) -> Option<i32> {
        self.errno
    }
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and a record dump would obscure the human report.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}
