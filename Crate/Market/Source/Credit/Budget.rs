//! Personal experiment envelope versus the campaign treasury balance.
//!
//! Grounded in `MARKET.md:81-83`: a researcher can have a personal budget
//! envelope while the campaign owns the actual treasury balance; researchers
//! express subjective priorities with bids but acquire no unrestricted spending
//! authority. `playbooks/research-market-week.yaml:41-55` supplies the native
//! caps and reserves. A budget is a bound, not a spendable balance.

use crate::{Permission, Resource, Warning};

/// A researcher's bounded envelope over a shared treasury.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Budget {
    /// Envelope owner identity (`MARKET.md:81`).
    pub envelope_owner: &'static str,
    /// Upper bound in native units (`MARKET.md:9`).
    pub ceiling: Resource,
}

impl Budget {
    /// Construct an envelope value.
    #[must_use]
    pub const fn of(envelope_owner: &'static str, ceiling: Resource) -> Self {
        Self {
            envelope_owner,
            ceiling,
        }
    }

    /// Check a bid's upper bound against the envelope.
    pub fn check_request(&self, requested: &Resource) -> Result<(), Warning> {
        match self.ceiling.fits(requested) {
            true => Ok(()),
            false => Err(Warning::Insufficient {
                resource: "experiment_envelope",
            }),
        }
    }

    /// The envelope is not the treasury balance (`MARKET.md:81-82`).
    #[must_use]
    pub const fn is_unrestricted_spending_authority() -> Permission {
        Permission::Deny
    }

    /// The envelope may never be exceeded (`MARKET.md:5-7,77`).
    #[must_use]
    pub const fn may_exceed() -> Permission {
        Permission::Deny
    }
}
