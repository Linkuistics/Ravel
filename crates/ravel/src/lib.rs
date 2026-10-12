//! Ravel's private session runtime. The command front is a later increment.

mod completion;

pub use completion::{
    end_session, publish_result, SessionChannel, EXIT_FILE_VARIABLE, MAX_RESULT_BYTES,
};
