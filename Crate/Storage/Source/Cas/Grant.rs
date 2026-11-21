//! An artifact grant: identity alone never authenticates a recipient.
//!
//! `v6_artifact_grants` pairs an artifact digest with a recipient, purpose,
//! policy digest and expiry (`sql/v6_migration.sql:74-84`); `docs/08:28-31`
//! states that the SHA digest alone must not authenticate a recipient.

use super::digest::Digest;

/// A scoped right to use one content-addressed artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grant {
    /// Artifact this grant covers.
    pub artifact: Digest,
    /// Named recipient identity.
    pub recipient: String,
    /// Declared purpose.
    pub purpose: String,
    /// Digest of the policy that authorized the grant.
    pub policy: Digest,
    /// Expiry in UTC.
    pub expires_utc: String,
    /// Event id that revoked the grant, when revoked.
    pub revoked: Option<String>,
}

impl Grant {
    /// Declare a grant.
    #[must_use]
    pub fn of(
        artifact: Digest,
        recipient: &str,
        purpose: &str,
        policy: Digest,
        expires_utc: &str,
    ) -> Self {
        Self {
            artifact,
            recipient: recipient.to_owned(),
            purpose: purpose.to_owned(),
            policy,
            expires_utc: expires_utc.to_owned(),
            revoked: None,
        }
    }

    /// Mark the grant revoked by an event.
    #[must_use]
    pub fn revoked(mut self, event_id: &str) -> Self {
        self.revoked = Some(event_id.to_owned());
        self
    }

    /// Whether this grant admits a recipient for one artifact.
    #[must_use]
    pub fn admits(&self, artifact: &Digest) -> crate::permission::Permission {
        let identity = self.artifact.matches(artifact);
        let active = match self.revoked {
            Some(_) => crate::permission::Permission::Deny,
            None => crate::permission::Permission::Allow,
        };
        let fields = match self.recipient.is_empty() || self.purpose.is_empty() {
            true => crate::permission::Permission::Deny,
            false => crate::permission::Permission::Allow,
        };
        identity.and(active).and(fields)
    }

    /// Whether the grant names a specific recipient, never a bare digest.
    #[must_use]
    pub fn scoped(&self) -> crate::permission::Permission {
        match self.recipient.is_empty() {
            true => crate::permission::Permission::Deny,
            false => crate::permission::Permission::Allow,
        }
    }
}
