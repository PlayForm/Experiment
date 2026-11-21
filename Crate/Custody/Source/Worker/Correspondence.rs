//! The sealed correspondence record of one research segment.
//!
//! Raw prompts, outputs and local tool transcripts go to the vault; sanitized
//! scientific statements go to separately signed publication objects. A worker
//! cannot enumerate that linkage or exchange messages directly with a sibling
//! (`docs/21:55-61`, `docs/22:29-44`, `docs/27:44-47`).

use super::turn::Turn;
use crate::permission::Permission;

/// An immutable chain of archived worker turns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Correspondence {
    /// `conversation_id` assigned by the trusted broker.
    pub conversation_id: String,
    /// `segment_id` within the conversation.
    pub segment_id: String,
    /// Turn records in arrival order.
    pub turns: Vec<Turn>,
    /// Encrypted CAS digest of the bounded input envelope.
    pub input_digest: String,
    /// Encrypted CAS digest of the output envelope.
    pub output_digest: String,
    /// Effective view identifier for the whole segment.
    pub view_id: String,
    /// Supplied source hash for the segment.
    pub source_hash: String,
    /// Supervisor-sealed receipt digest.
    pub receipt_digest: String,
}

impl Correspondence {
    /// Start a segment with no turns.
    #[must_use]
    pub fn of(conversation: &str, segment: &str, view: &str, source: &str) -> Self {
        Self {
            conversation_id: conversation.to_owned(),
            segment_id: segment.to_owned(),
            turns: Vec::new(),
            input_digest: String::new(),
            output_digest: String::new(),
            view_id: view.to_owned(),
            source_hash: source.to_owned(),
            receipt_digest: String::new(),
        }
    }

    /// Return a new record with one more sealed turn, leaving this one intact.
    #[must_use]
    pub fn with_turn(&self, turn: Turn) -> Self {
        let mut next = self.clone();
        next.turns.push(turn);
        next
    }

    /// Attach the encrypted envelope references and the supervisor receipt.
    #[must_use]
    pub fn sealed(mut self, input: &str, output: &str, receipt: &str) -> Self {
        self.input_digest = input.to_owned();
        self.output_digest = output.to_owned();
        self.receipt_digest = receipt.to_owned();
        self
    }

    /// Whether the record is complete enough for archival.
    #[must_use]
    pub fn complete(&self) -> Permission {
        Permission::filled(&self.conversation_id)
            .and(Permission::filled(&self.segment_id))
            .and(Permission::digest(&self.input_digest))
            .and(Permission::digest(&self.output_digest))
            .and(Permission::filled(&self.view_id))
            .and(Permission::digest(&self.source_hash))
            .and(Permission::digest(&self.receipt_digest))
    }

    /// Whether every turn carried its own provenance.
    #[must_use]
    pub fn turns_complete(&self) -> bool {
        self.turns
            .iter()
            .all(|turn| turn.complete() == Permission::Allow)
    }
}
