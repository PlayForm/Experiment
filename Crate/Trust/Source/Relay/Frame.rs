//! Byte-length-prefixed application frames.
//!
//! Grounded in `security/src/relay.rs:74-91`: `read_frame` reads a four-byte
//! big-endian `u32` size, rejects zero and sizes above `MAX_FRAME` (9 MiB),
//! then reads exactly that many bytes; `write_frame` applies the same cap and
//! flushes. The framing is JSON over TLS 1.3, not HTTP (`relay.rs:1-4`).
//! This type checks sizes; it reads and writes no bytes.

use crate::{MAX_FRAME_BYTES, warning::Warning};

/// The framed-wire length prefix contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Frame {
    /// Width of the big-endian length prefix (`relay.rs:75,87`).
    pub prefix_bytes: usize,
    /// Largest accepted frame (`relay.rs:26`).
    pub max_bytes: usize,
}

impl Frame {
    /// Construct the prototype's frame contract.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            prefix_bytes: 4,
            max_bytes: MAX_FRAME_BYTES,
        }
    }

    /// Validate a declared frame size without allocating a buffer
    /// (`relay.rs:76-78`).
    pub fn check_size(&self, size: usize) -> Result<(), Warning> {
        match size > 0 && size <= self.max_bytes {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "frame-size",
                detail: size.to_string(),
            }),
        }
    }

    /// Encode a validated length as four big-endian bytes (`relay.rs:87`).
    #[must_use]
    pub const fn length_bytes(size: u32) -> [u8; 4] {
        size.to_be_bytes()
    }

    /// The framed payload carried by the relay (`relay.rs:1-2`).
    #[must_use]
    pub const fn payload_format() -> &'static str {
        "json"
    }
}
