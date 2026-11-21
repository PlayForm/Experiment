//! The short-lived OpenSSH user certificate contract.
//!
//! Grounded in `security/src/ssh_ca.rs:39-72`: `issue` builds a user certificate
//! valid from `now-10` to `now+600`, derives the serial from the first eight
//! bytes of `SHA-256(identity || now)`, sets `key_id` to
//! `run-lab:<identity>:<serial>`, sets exactly one dedicated principal
//! (`lab-maintenance`, explicitly NOT root), validates the result against the
//! CA fingerprint, and journals `SSH_CERT_ISSUED`. No certificate is issued here.

use crate::permission::Permission;

/// A typed description of the SSH certificate the prototype issues.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    /// Certificate type (`ssh_ca.rs:55`: `CertType::User`).
    pub cert_type: &'static str,
    /// The single valid principal (`ssh_ca.rs:56`).
    pub principal: &'static str,
    /// Backdated validity seconds (`ssh_ca.rs:47`).
    pub before_seconds: u64,
    /// Forward validity seconds (`ssh_ca.rs:47`: `now + 600`).
    pub after_seconds: u64,
    /// Key ID prefix (`ssh_ca.rs:54`).
    pub key_id_prefix: &'static str,
    /// Serial derivation (`ssh_ca.rs:48-52`).
    pub serial_source: &'static str,
    /// Fingerprint algorithm used for self-validation (`ssh_ca.rs:62`).
    pub fingerprint: &'static str,
}

impl Certificate {
    /// Construct the prototype's certificate definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            cert_type: "user",
            principal: "lab-maintenance",
            before_seconds: 10,
            after_seconds: 600,
            key_id_prefix: "run-lab:",
            serial_source: "sha256(identity||now)[..8]",
            fingerprint: "Sha256",
        }
    }

    /// The `key_id` string for an identity and serial (`ssh_ca.rs:54`).
    #[must_use]
    pub fn key_id(&self, identity: &str, serial: u64) -> String {
        format!("{}{identity}:{serial}", self.key_id_prefix)
    }

    /// The principal is a dedicated account, never root (`ssh_ca.rs:45-46`).
    #[must_use]
    pub const fn grants_root() -> Permission {
        Permission::Deny
    }

    /// Total validity window in seconds.
    #[must_use]
    pub const fn window_seconds(&self) -> u64 {
        self.before_seconds + self.after_seconds
    }
}
