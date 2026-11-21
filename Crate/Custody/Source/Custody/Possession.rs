//! Possession as a narrow capability over encrypted bytes.
//!
//! A holder may store ciphertext without any decryption entitlement; toolchain
//! capability must also match the artifact type
//! (`docs/19:15-20,66-70`).

use super::qualification::Qualification;
use crate::permission::Permission;
use crate::right::Right;
use crate::switch::Switch;

/// The rights actually granted to one holder for one artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Possession {
    /// `holder_id` the grant is bound to.
    pub holder_id: String,
    /// Trusted role used to bound the grant.
    pub qualification: Qualification,
    /// Rights this holder actually carries.
    pub rights: Vec<Right>,
    /// Whether bytes are encrypted at rest in this holder.
    pub encryption: Switch,
    /// `lease_expires_at` canonical UTC deadline of the grant.
    pub lease_expires_at: String,
}

impl Possession {
    /// A possession grant that permits holding only.
    #[must_use]
    pub fn of(holder: &str, qualification: Qualification, expires: &str) -> Self {
        Self {
            holder_id: holder.to_owned(),
            qualification,
            rights: vec![Right::Hold],
            encryption: Switch::Off,
            lease_expires_at: expires.to_owned(),
        }
    }

    /// Bind additional rights that a qualified review has approved.
    #[must_use]
    pub fn granting(mut self, right: Right) -> Self {
        match self.rights.contains(&right) {
            true => self,
            false => {
                self.rights.push(right);
                self
            }
        }
    }

    /// Declare that the bytes are held under envelope encryption.
    #[must_use]
    pub fn encrypted(mut self) -> Self {
        self.encryption = Switch::On;
        self
    }

    /// Whether one right is both granted and admissible for this role.
    #[must_use]
    pub fn authorizes(&self, right: Right) -> Permission {
        Permission::of(self.rights.contains(&right))
            .and(self.qualification.may_hold(right))
            .and(self.encryption.permit())
            .and(Permission::filled(&self.lease_expires_at))
    }

    /// A worker may never be granted a right beyond holding ciphertext.
    #[must_use]
    pub fn worker_bound(&self) -> Permission {
        let forbidden = self
            .rights
            .iter()
            .any(|right| right.worker_forbidden() || self.qualification.may_hold(*right) == Permission::Deny);
        match (self.qualification.is_worker(), forbidden) {
            (true, true) => Permission::Deny,
            (true, false) | (false, true) | (false, false) => Permission::Allow,
        }
    }
}
