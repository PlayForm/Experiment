//! The three typed relay operations.
//!
//! Grounded in `security/src/relay.rs:27-39` (`Request`: `Publish`, `Fetch`,
//! `Ack`, tagged `op` and snake-cased; `Response` with `ok`, `reason`,
//! `messages`), `relay.rs:93-98` (message IDs are 32 hex digits), and
//! `relay.rs:101,133` (fetch is capped at 16). The response's `ok` flag is
//! modelled as a typed verdict elsewhere; no HTTP verb is involved.

use serde::{Deserialize, Serialize};

use crate::crypto::envelope::Envelope;
use crate::warning::Warning;

/// One operation on the opaque relay wire.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Wire {
    /// Submit a signed ciphertext envelope (`relay.rs:30`).
    Publish { envelope: Envelope },
    /// Retrieve at most `limit` envelopes from the peer mailbox
    /// (`relay.rs:31`).
    Fetch { limit: usize },
    /// Archive a delivered message (`relay.rs:32`).
    Ack { id: String },
}

impl Wire {
    /// Server-side fetch cap (`relay.rs:101,133`: `limit.min(16)`).
    pub const MAX_FETCH: usize = 16;

    /// The operation name used on the wire.
    #[must_use]
    pub const fn operation(&self) -> &'static str {
        match self {
            Self::Publish { .. } => "publish",
            Self::Fetch { .. } => "fetch",
            Self::Ack { .. } => "ack",
        }
    }

    /// Validate a message ID as the prototype does (`relay.rs:93-98`).
    pub fn check_message_id(id: &str) -> Result<(), Warning> {
        let shape = id.len() == 32 && id.bytes().all(|byte| byte.is_ascii_hexdigit());
        match shape {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "message-id",
                detail: id.to_owned(),
            }),
        }
    }

    /// Clamp a fetch limit to the server cap without branching at the caller.
    #[must_use]
    pub const fn clamped_limit(limit: usize) -> usize {
        match limit <= Self::MAX_FETCH {
            true => limit,
            false => Self::MAX_FETCH,
        }
    }
}
