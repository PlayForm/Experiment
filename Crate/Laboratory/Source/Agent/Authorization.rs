//! Service-role authorization, never identity alone.
//!
//! Grounded in `docs/04-internal-api.md:1-7` (mTLS identity is mapped to an
//! allowlisted service role before parsing or acting), `docs/09-cryptographic-
//! trust.md:9-12` (an authenticated TLS identity alone does not authorize an
//! endpoint) and `docs/23-interface-inventory.md:37-41,75-88` (a valid
//! certificate and signature do not imply role scope, expected epoch or
//! idempotency). No worker role is authorized on any internal or public
//! operation (`docs/26:61-65`).

/// An authorization decision bound to role, scope and policy epoch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorization {
    /// Authenticated service role, not a raw certificate identity.
    pub role: String,
    /// Operation scope the role may invoke.
    pub scope: String,
    /// Policy epoch the decision was computed under.
    pub epoch: crate::state::Epoch,
    /// Recorded decision of the trusted policy engine.
    pub decision: crate::Permission,
}

impl Authorization {
    /// Record an authorization, refusing an empty role with a warning.
    #[must_use]
    pub fn of(
        role: String,
        scope: String,
        epoch: crate::state::Epoch,
        decision: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        match role.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "service role",
                detail: "an allowlisted service role is required".to_owned(),
            }),
            false => Ok(Self {
                role,
                scope,
                epoch,
                decision,
            }),
        }
    }

    /// The roles the laboratory never authorizes, at any scope
    /// (`docs/15:23-30`).
    #[must_use]
    pub const fn forbidden() -> [&'static str; 3] {
        ["offline-worker", "model", "researcher"]
    }

    /// Re-evaluate under the pinned epoch; a stale epoch fails closed.
    #[must_use]
    pub fn under(&self, current: &crate::state::Epoch) -> Result<crate::Permission, crate::Warning> {
        current.check(&self.epoch)
    }
}
