//! The front's half of the private protocol channel: length-prefixed JSON.
//!
//! A frame is a four-byte big-endian length followed by that many bytes of
//! UTF-8 JSON. Both the Ravel CLI and the internal Rhai worker use this module. The channel is
//! a socket pair the front creates; the worker's end is its descriptor 3.
//!
//! Each read names the largest frame it accepts. A context and a
//! selection result are bounded by the fixed protocol message bound, and a
//! context by the caller's context budget. The worker checks each of its
//! frames against the same bound before it sends it, and reports an overflow
//! instead, so a frame over its bound here is the worker breaking the protocol.

use std::io::{self, Read, Write};

use serde_json::Value;

use crate::authority::RHAI_ENTRY_BYTES;
use crate::inputs::{PARAMS_LIMIT, PROMPT_LIMIT, TASK_ID_LIMIT};
use crate::limits::{CONTEXT_MAX_BYTES, MESSAGE_BYTES};

/// The factor of the largest JSON escape. JSON writes a control byte as six
/// bytes, such as `\u0001`.
const ESCAPE: u64 = 6;

/// The descriptor of the resolved skill directories, which `request.skills`
/// holds (`docs/specs/skill-discovery.md`, *Bounds*).
const DESCRIPTOR_BYTES: u64 = 1024 * 1024;

/// The allowance for everything else in the evaluate frame: the member names,
/// the bounds and the values that Ravel does not bound itself.
const EVALUATE_ENVELOPE_BYTES: u64 = 1024 * 1024;

/// The largest evaluate frame within the public bounds
/// (`docs/specs/configuration-evaluation.md`, *Frame bounds*). The entry, the
/// two prompts and the task ID can each escape six-fold. A parameter adds at
/// most six bytes of quotes and separators beside the escape of its name and
/// value. The caller's context travels twice: in `request.context` and as the
/// delivered context in `selectContext`. Each encodes to at most the context
/// budget, so it does not escape again.
const EVALUATE_BYTES: u64 = ESCAPE
    * (RHAI_ENTRY_BYTES + 2 * PROMPT_LIMIT as u64 + TASK_ID_LIMIT as u64)
    + 2 * ESCAPE * PARAMS_LIMIT as u64
    + 2 * CONTEXT_MAX_BYTES
    + DESCRIPTOR_BYTES
    + EVALUATE_ENVELOPE_BYTES;

/// The largest reply to a harness run. It holds three strings of 1 MiB each,
/// the output, the error output and the result, each escaped six-fold. One
/// more message bound is room for the rest of the map.
const HARNESS_REPLY_BYTES: u64 = 3 * ESCAPE * MESSAGE_BYTES + MESSAGE_BYTES;

/// The frame bound: the largest frame the front sends and the worker reads. It
/// is the larger of the evaluate frame and the reply to a harness run. The
/// worker accepts the same bound in `rhai_worker::role`, and the front checks
/// its evaluate frame against it before it sends the frame.
pub const FRONT_FRAME_BYTES: usize = {
    (if EVALUATE_BYTES > HARNESS_REPLY_BYTES {
        EVALUATE_BYTES
    } else {
        HARNESS_REPLY_BYTES
    }) as usize
};

#[derive(Debug)]
pub enum FrameError {
    /// The peer closed the channel before a whole frame arrived.
    Closed,
    Io(io::Error),
    /// A frame's length, and the bound it exceeded.
    TooLarge(usize, usize),
    NotJson(serde_json::Error),
}

impl std::fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameError::Closed => write!(f, "the channel closed before a whole frame arrived"),
            FrameError::Io(error) => write!(f, "{error}"),
            FrameError::TooLarge(length, max) => {
                write!(f, "a frame of {length} bytes exceeds {max}")
            }
            FrameError::NotJson(error) => write!(f, "a frame is not JSON: {error}"),
        }
    }
}

pub fn write_frame(mut writer: impl Write, message: &Value) -> Result<(), FrameError> {
    let body = serde_json::to_vec(message).map_err(FrameError::NotJson)?;
    if body.len() > FRONT_FRAME_BYTES {
        return Err(FrameError::TooLarge(body.len(), FRONT_FRAME_BYTES));
    }
    let length = u32::try_from(body.len())
        .map_err(|_| FrameError::TooLarge(body.len(), FRONT_FRAME_BYTES))?;
    writer
        .write_all(&length.to_be_bytes())
        .and_then(|()| writer.write_all(&body))
        .map_err(FrameError::Io)
}

/// The next frame, if it is at most `max` bytes.
pub fn read_frame(mut reader: impl Read, max: usize) -> Result<Value, FrameError> {
    let mut header = [0; 4];
    read_exact(&mut reader, &mut header)?;
    let length = u32::from_be_bytes(header) as usize;
    if length > max {
        return Err(FrameError::TooLarge(length, max));
    }
    let mut body = vec![0; length];
    read_exact(&mut reader, &mut body)?;
    serde_json::from_slice(&body).map_err(FrameError::NotJson)
}

fn read_exact(reader: &mut impl Read, buffer: &mut [u8]) -> Result<(), FrameError> {
    reader.read_exact(buffer).map_err(|error| {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            FrameError::Closed
        } else {
            FrameError::Io(error)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_frame_bound_is_the_derived_evaluate_frame() {
        assert_eq!(EVALUATE_BYTES, 38_541_312);
        assert_eq!(HARNESS_REPLY_BYTES, 19 * 1024 * 1024);
        assert_eq!(FRONT_FRAME_BYTES, 38_541_312);
    }

    #[test]
    fn a_frame_round_trips_with_a_big_endian_length() {
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &json!({"type": "hello"})).unwrap();
        assert_eq!(&bytes[..4], &[0, 0, 0, 16]);
        assert_eq!(
            read_frame(bytes.as_slice(), 16).unwrap(),
            json!({"type": "hello"})
        );
    }

    #[test]
    fn a_truncated_frame_reads_as_closed_and_an_oversized_one_is_refused() {
        assert!(matches!(
            read_frame([0, 0, 0, 9, b'{'].as_slice(), 9),
            Err(FrameError::Closed)
        ));
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &json!({"type": "hello"})).unwrap();
        assert!(matches!(
            read_frame(bytes.as_slice(), 15),
            Err(FrameError::TooLarge(16, 15))
        ));
    }
}
