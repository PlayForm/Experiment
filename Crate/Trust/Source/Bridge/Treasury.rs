//! Treasury reservation receipts and disclosure limits.
//!
//! Grounded in `V4_TRUST_BRIDGE.md:22` (`resource.reserve.receipt` requires a
//! transaction ID, policy version and idempotency ID) and `:39` (the treasury
//! sees quantities, claim IDs and signed receipts, not hidden reasoning).
//! No reservation is made and no receipt is signed here.

use crate::permission::Permission;

/// A typed treasury reservation receipt contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Treasury {
    /// Required transaction identifier (`:22`).
    pub transaction_id: &'static str,
    /// Required policy version (`:22`).
    pub policy_version: &'static str,
    /// Required idempotency identifier (`:22`).
    pub idempotency_id: &'static str,
    /// Whether hidden reasoning is visible to the treasury (`:39`).
    pub hidden_reasoning: Permission,
}

impl Treasury {
    /// The three required receipt fields in table order.
    pub const RECEIPT_FIELDS: [&'static str; 3] = ["transaction_id", "policy_version", "idempotency_id"];

    /// Construct the receipt contract with hidden reasoning denied.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            transaction_id: "transaction_id",
            policy_version: "policy_version",
            idempotency_id: "idempotency_id",
            hidden_reasoning: Permission::Deny,
        }
    }

    /// A duplicate idempotency ID must be refused, not double-charged
    /// (`V4_TRUST_BRIDGE.md:22`, `STORAGE.md:47`: idempotent ACKs).
    pub fn check_duplicate(&self, seen: bool) -> Result<(), crate::warning::Warning> {
        match seen {
            true => Err(crate::warning::Warning::Replay {
                message_id: self.idempotency_id.to_owned(),
            }),
            false => Ok(()),
        }
    }

    /// The treasury may see signed receipts and quantities, never hidden
    /// reasoning (`:39`).
    #[must_use]
    pub const fn may_see_hidden_reasoning() -> Permission {
        Permission::Deny
    }
}
