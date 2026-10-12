//! A program and its arguments, spawned as a **job** and supervised until it
//! ends.
//!
//! The caller builds an [`Argv`] and this crate launches it. Nothing here
//! chooses a program, reads a configuration or understands what the child is
//! for: every word of the command, every variable name and every path is the
//! caller's. What the crate owns is how the child is spawned, how its end is
//! learned, and how it is stopped.
//!
//! # From an argv to a running child
//!
//! [`Argv::new`] takes a program and its arguments, each string one whole
//! word; [`run`] spawns them directly, with no shell, and supervises the
//! child until it ends.
//!
//! **The child is a job.** It is spawned into a process group of its own and
//! handed the launcher's controlling terminal, so a terminal signal reaches the
//! child rather than the launcher, the escalation can reap a grandchild the
//! child spawned, and a launcher's own ignored dispositions are not inherited
//! across the `exec` by every wrapper the command names. Whatever ends the
//! child, its whole group ends with the launch, and the terminal comes back
//! with the modes it was handed over in. A group that survives is reported as
//! [`Group::Present`] beside the child's status. See [`run`].
//!
//! **A launch ends out of band.** An interactive child returns to its prompt
//! when it finishes rather than exiting, so its own exit is not the event
//! anyone is waiting for. [`Channel`] is: a fresh path per launch that the
//! child creates an empty file at (through [`signal`]) when it is done, and whose
//! *appearance* starts the kill [`Escalation`] the child cannot perform on
//! itself. See [`Escalation`] for why that is the launcher's job.
//!
//! [`run_observed`] adds synchronous parent-side [`LaunchEvent`] notifications
//! at successful spawn and confirmed reap, including reap during wait-error
//! recovery. Notifications precede terminal recovery; failed
//! spawn emits none. [`run`] keeps the same interface without an observer.

mod argv;
mod channel;
mod confinement;
mod error;
mod run;

pub use argv::Argv;
pub use channel::{signal, Channel};
pub use confinement::{
    confinement_available, confinement_system_reads, regular_file_at, FilesystemGrants,
};
pub use error::LaunchError;
pub use run::{
    reraise, run, run_confined_observed, run_noninteractive, run_observed, take_interrupt, End,
    Ended, EntrySignals, Escalation, Group, Launch, LaunchEvent, NoninteractiveLaunch,
};
