//! The fixed generic boundary envelope with encrypted semantic content.
//!
//! Mirrors every field of `schemas/opaque-envelope.schema.json`: `v` fixed to
//! `6.2`, `rt` route ID, `e` epoch, `enc` encapsulation token, `ct` ciphertext
//! and `s` signature. `docs/05:47-48` permits a small fixed envelope whose
//! meaningful fields live in authenticated ciphertext. No bytes are encrypted.

use crate::verdict::Verdict;

/// One generic opaque wire envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Opaque {
    /// Protocol version, fixed to `6.2`.
    pub version: String,
    /// Opaque routing token; not an authorization credential.
    pub route: String,
    /// Key epoch of the recipient.
    pub epoch: u64,
    /// Encapsulation material for the recipient.
    pub encapsulation: String,
    /// Authenticated ciphertext of the semantic payload.
    pub ciphertext: String,
    /// Signature over the canonical authenticated envelope.
    pub signature: String,
}

impl Opaque {
    /// Record an envelope from supplied wire fields.
    #[must_use]
    pub fn of(
        route: &str,
        epoch: u64,
        encapsulation: &str,
        ciphertext: &str,
        signature: &str,
    ) -> Self {
        Self {
            version: crate::PROTOCOL_REVISION.to_owned(),
            route: route.to_owned(),
            epoch,
            encapsulation: encapsulation.to_owned(),
            ciphertext: ciphertext.to_owned(),
            signature: signature.to_owned(),
        }
    }

    /// Whether all six envelope fields have the required shape.
    #[must_use]
    pub fn well_formed(&self) -> Verdict {
        Verdict::of(self.version == crate::PROTOCOL_REVISION)
            .and(Verdict::opaque(&self.route))
            .and(Verdict::of(self.epoch >= 1))
            .and(Verdict::filled(&self.encapsulation))
            .and(Verdict::filled(&self.ciphertext))
            .and(Verdict::filled(&self.signature))
    }

    /// The only fields observable at the boundary.
    #[must_use]
    pub const fn visible_fields() -> [&'static str; 6] {
        crate::ENVELOPE_FIELDS
    }

    /// Semantic content is not visible in a wire envelope.
    #[must_use]
    pub const fn semantic_content_visible() -> bool {
        false
    }
}
