//! Nonce, message-ID binding and replay guard.
//!
//! Grounded in `security/src/crypto.rs:157-170` (a 16-byte random nonce becomes
//! both `message_id` and `nonce_hex` in hex), `crypto.rs:193` (verification
//! requires equality and a 16-byte nonce) and `crypto.rs:230-236` (open marks
//! the message ID once in a replay directory). No randomness is drawn and no
//! mark file is written here.

use crate::{NONCE_BYTES, warning::Warning};

/// A typed nonce and its message-ID binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nonce {
    /// Hex representation of the nonce, also used as the message ID.
    pub hex: String,
}

impl Nonce {
    /// Directory under which replay markers are written (`crypto.rs:231`).
    pub const REPLAY_DIRECTORY: &'static str = "replay";

    /// Validate the nonce shape and its message-ID binding
    /// (`crypto.rs:193`: `message_id == nonce_hex` and 16 decoded bytes).
    pub fn check(&self, message_id: &str) -> Result<(), Warning> {
        let shape = self.hex.len() == NONCE_BYTES * 2
            && self.hex.bytes().all(|byte| byte.is_ascii_hexdigit());
        match shape && message_id == self.hex {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "nonce",
                detail: self.hex.clone(),
            }),
        }
    }

    /// Replay marker path for this nonce (`crypto.rs:235`).
    #[must_use]
    pub fn replay_path(&self) -> String {
        format!("{}/{}", Self::REPLAY_DIRECTORY, self.hex)
    }

    /// Classify a repeated message ID as a replay warning rather than retrying
    /// the operation (`crypto.rs:230-236`).
    #[must_use]
    pub fn duplicate(&self) -> Warning {
        Warning::Replay {
            message_id: self.hex.clone(),
        }
    }
}
