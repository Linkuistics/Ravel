//! The opaque-result transport used by Ravel's policy-routed command front.

mod completion;

pub use completion::{
    end_session, publish_result, SessionChannel, EXIT_FILE_VARIABLE, MAX_RESULT_BYTES,
};

#[cfg(test)]
mod directory_control;

mod private_directory;
