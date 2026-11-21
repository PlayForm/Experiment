//! The `(principal, action, artifact, purpose, context)` access scope.
//!
//! A trusted broker evaluates an embargo rule against this value before it
//! compiles any context. The value carries no reason field that could flow to
//! a worker (`docs/19:29-31`, `docs/20:31-35`).

/// The non-disclosing inputs to an embargo decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
    /// Artifact or repository subject digest.
    pub subject_digest: String,
    /// Recipient group or holder identifier.
    pub recipient_id: String,
    /// Research track requesting access.
    pub track_id: String,
    /// Requested purpose, e.g. `publication`.
    pub purpose: String,
    /// Canonical evaluation instant.
    pub evaluated_at: String,
    /// Bound policy revision digest.
    pub policy_digest: String,
}

impl Scope {
    /// Build a scope from trusted request fields.
    #[must_use]
    pub fn of(
        subject: &str,
        recipient: &str,
        track: &str,
        purpose: &str,
        at: &str,
        policy: &str,
    ) -> Self {
        Self {
            subject_digest: subject.to_owned(),
            recipient_id: recipient.to_owned(),
            track_id: track.to_owned(),
            purpose: purpose.to_owned(),
            evaluated_at: at.to_owned(),
            policy_digest: policy.to_owned(),
        }
    }

    /// Whether the scope is complete enough for a decision.
    #[must_use]
    pub fn complete(&self) -> crate::permission::Permission {
        crate::permission::Permission::digest(&self.subject_digest)
            .and(crate::permission::Permission::filled(&self.recipient_id))
            .and(crate::permission::Permission::filled(&self.track_id))
            .and(crate::permission::Permission::filled(&self.purpose))
            .and(crate::permission::Permission::filled(&self.evaluated_at))
            .and(crate::permission::Permission::digest(&self.policy_digest))
    }
}
