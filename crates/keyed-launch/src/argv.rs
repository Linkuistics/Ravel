use std::ffi::{OsStr, OsString};

/// A program and its arguments, in order, ready to spawn.
///
/// There is no shell and no second reading of any word: each is spawned as it
/// is given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
    arg0: Option<OsString>,
}

impl Argv {
    /// A program and arguments the caller built, each string one whole word.
    #[must_use]
    pub fn new(program: OsString, args: Vec<OsString>) -> Self {
        Self {
            program,
            args,
            arg0: None,
        }
    }

    /// The same launch with `arg0` as the child's `argv[0]` in place of the
    /// program. The program is still what is spawned.
    ///
    /// For a caller that resolved a name to a path itself: it spawns the path,
    /// so that nothing is looked up a second time, and the child still sees
    /// the name it was chosen by. A confined launch runs the path it is given,
    /// and its child's `argv[0]` is that path whatever this says.
    #[must_use]
    pub fn with_arg0(mut self, arg0: OsString) -> Self {
        self.arg0 = Some(arg0);
        self
    }

    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    /// The child's `argv[0]`: the program, unless [`Argv::with_arg0`] named
    /// another.
    #[must_use]
    pub fn arg0(&self) -> &OsStr {
        self.arg0.as_deref().unwrap_or(&self.program)
    }

    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }

    /// The whole launch as one word list, program first — the shape a
    /// `Command`-building consumer and a diagnostic both want.
    #[must_use]
    pub fn words(&self) -> Vec<OsString> {
        let mut words = Vec::with_capacity(self.args.len() + 1);
        words.push(self.program.clone());
        words.extend(self.args.iter().cloned());
        words
    }
}
