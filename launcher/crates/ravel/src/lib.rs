//! The opaque-result transport used by Ravel's configuration script-routed command front.

mod completion;

pub use completion::{
    EXIT_FILE_VARIABLE, MAX_RESULT_BYTES, SessionChannel, end_session, publish_result,
};

#[cfg(test)]
mod directory_control;

mod private_directory;
