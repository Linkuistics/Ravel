use std::fmt;

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque**: the message describes the failure; errno and consumed
/// cancellation are available through typed accessors. The type implements
/// `std::error::Error` without imposing an error-handling dependency.
///
/// Its obligation is to name what is wrong, name where, and name what would
/// fix it.
pub struct LaunchError {
    message: String,
    errno: Option<i32>,
    consumed_signal: Option<i32>,
    source: Option<std::io::Error>,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            errno: None,
            consumed_signal: None,
            source: None,
        }
    }

    pub(crate) fn with_io_source(mut self, source: std::io::Error) -> Self {
        self.source = Some(source);
        self
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

    /// The first cancellation consumed during supervision, even if waiting
    /// subsequently failed. This does not establish reap or an ending.
    #[must_use]
    pub fn consumed_signal(&self) -> Option<i32> {
        self.consumed_signal
    }

    pub(crate) fn with_consumed_signal(mut self, signal: Option<i32>) -> Self {
        // Private break: a later consumption replaces the first.
        #[cfg(test)]
        if crate::run::first_latch_broken("error") {
            self.consumed_signal = signal.or(self.consumed_signal);
            return self;
        }
        self.consumed_signal = self.consumed_signal.or(signal);
        self
    }

    pub(crate) fn with_diagnostic(mut self, diagnostic: impl fmt::Display) -> Self {
        use fmt::Write as _;
        let _ = write!(self.message, "; {diagnostic}");
        self
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

impl std::error::Error for LaunchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &dyn std::error::Error)
    }
}
